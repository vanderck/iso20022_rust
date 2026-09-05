use crate::context::CrudClient;
use crate::crudgrpc::{CreateRequest, DbPath, Fld, ReadRequest, UpdateRequest};

use super::journey_event::{JourneyEvent, JourneyState};
use super::state_machine::StateMachine;

pub struct StateEffects;

impl StateEffects {
    fn db_path(table: &str) -> DbPath {
        DbPath {
            server: "CBS".into(),
            schema: "iso20022".into(),
            table: table.into(),
        }
    }

    fn first_row_first_col(resp: crate::crudgrpc::DataResponse) -> Option<String> {
        let row = resp.data.into_iter().next()?;
        row.list.into_iter().next()
    }

    pub async fn read_current_state(
        crud: &mut crate::context::CrudClient,
        journey_id: &str,
    ) -> Result<Option<JourneyState>, tonic::Status> {
        let req = ReadRequest {
            path: Some(Self::db_path("payment_journeys")),
            column: vec!["current_status".into()],
            where_clause: vec![Fld {
                name: "journey_id".into(),
                relation: None,
                value: journey_id.to_string(),
            }],
            page: 0,
            page_size: Some(1),
        };
        let resp = crud.read(req).await?.into_inner();
        Ok(Self::first_row_first_col(resp).and_then(|s| JourneyState::from_db_value(&s)))
    }

    pub async fn status_history_event_exists(
        crud: &mut CrudClient,
        status_code: &str,
        related_msg_id: &str,
    ) -> Result<bool, tonic::Status> {
        let req = ReadRequest {
            path: Some(Self::db_path("status_history")),
            column: vec!["status_id".into()],
            where_clause: vec![
                Fld { name: "status_code".into(), relation: None, value: status_code.to_string() },
                Fld { name: "related_msg_id".into(), relation: None, value: related_msg_id.to_string() },
            ],
            page: 0,
            page_size: Some(1),
        };
        let resp = crud.read(req).await?.into_inner();
        Ok(Self::first_row_first_col(resp).is_some())
    }

    /// Append a row to `status_history` – idempotent on (status_code, related_msg_id).
    ///
    /// Uses an optimistic insert: the row is written unconditionally and a
    /// `gRPC AlreadyExists` response (unique-constraint violation surfaced by
    /// the CRUD service) is treated as success.  This eliminates the
    /// check-then-act TOCTOU window that existed when a read was issued first —
    /// two concurrent retries of the same event no longer risk both passing the
    /// existence check and producing duplicate audit rows.
    pub async fn append_status_history(
        crud: &mut CrudClient,
        journey_id: Option<&str>,
        event: &JourneyEvent,
        event_id: &str,
    ) -> Result<(), tonic::Status> {
        let mut data = vec![
            Fld { name: "status_code".into(), relation: None, value: event.status_history_code().to_string() },
            Fld { name: "event_description".into(), relation: None, value: event.description.clone() },
            Fld { name: "related_msg_id".into(), relation: None, value: event_id.to_string() },
        ];

        if let Some(jid) = journey_id.filter(|x| !x.trim().is_empty()) {
            data.push(Fld { name: "journey_id".into(), relation: None, value: jid.to_string() });
        }
        if let Some(rc) = event.reason_code.as_deref().filter(|x| !x.trim().is_empty()) {
            data.push(Fld { name: "reason_code".into(), relation: None, value: rc.to_string() });
        }

        let req = CreateRequest {
            who: "state_effects".into(),
            review: Some(false),
            path: Some(Self::db_path("status_history")),
            data,
        };
        match crud.create(req).await {
            Ok(_) => Ok(()),
            // Duplicate row — idempotent; treat as success.
            Err(e) if e.code() == tonic::Code::AlreadyExists => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn apply_event_to_journey(
        crud: &mut CrudClient,
        journey_id: &str,
        event: &JourneyEvent,
    ) -> Result<Option<JourneyState>, tonic::Status> {
        let current = Self::read_current_state(crud, journey_id).await?;
        let next = match StateMachine::transition(current, event) {
            Ok(n) => n,
            Err(reason) => {
                tracing::warn!(
                    journey_id = %journey_id,
                    event = ?event.kind,
                    current_state = ?current,
                    %reason,
                    "State transition rejected — journey state unchanged"
                );
                return Ok(None);
            }
        };

        let req = UpdateRequest {
            who: "state_effects".into(),
            review: Some(false),
            path: Some(Self::db_path("payment_journeys")),
            where_clause: vec![Fld {
                name: "journey_id".into(),
                relation: None,
                value: journey_id.to_string(),
            }],
            data: vec![Fld {
                name: "current_status".into(),
                relation: None,
                value: next.as_db_value().to_string(),
            }],
        };
        crud.update(req).await?;
        Ok(Some(next))
    }
}

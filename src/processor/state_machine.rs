use super::journey_event::{JourneyEvent, JourneyEventKind, JourneyState};
use super::transition_policy::TransitionPolicy;

pub struct StateMachine;

impl StateMachine {
    pub fn transition(
        current: Option<JourneyState>,
        event: &JourneyEvent,
    ) -> Result<JourneyState, String> {
        let next = match event.kind {
            JourneyEventKind::CustomerPaymentInitiated => JourneyState::Received,
            JourneyEventKind::CustomerStatusAccepted => JourneyState::CustomerAccepted,
            JourneyEventKind::CustomerStatusRejected => JourneyState::CustomerRejected,
            JourneyEventKind::CustomerStatusPending => JourneyState::Pending,
            JourneyEventKind::FiPaymentSent => JourneyState::SentToNetwork,
            JourneyEventKind::FiStatusReceived => Self::map_fi_status(event.iso_status.as_deref()),
            JourneyEventKind::FiReturnReceived => JourneyState::Returned,
            JourneyEventKind::FiReversalRequested => JourneyState::FiReversalRequested,
            JourneyEventKind::CustomerReversalRequested => JourneyState::CustomerReversalRequested,
            JourneyEventKind::RecallRequested => JourneyState::RecallRequested,
            JourneyEventKind::UnableToApply => JourneyState::UnableToApply,
            JourneyEventKind::ClaimNonReceipt => JourneyState::InvestigationOpen,
            JourneyEventKind::InvestigationResolutionAccepted => JourneyState::InvestigationResolved,
            JourneyEventKind::InvestigationResolutionRejected => JourneyState::RecallRejected,
            JourneyEventKind::InvestigationResolutionPending => JourneyState::InvestigationPendingResponse,
            JourneyEventKind::StatusRequestReceived => return Err("status requests are audit events and should not mutate journey state".into()),
            JourneyEventKind::ReportingUpdate => return Err("reporting updates should not mutate journey state".into()),
            JourneyEventKind::SanctionsHold => JourneyState::SanctionsHold,
        };

        if TransitionPolicy::is_allowed(current, next) || TransitionPolicy::should_accept_by_rank(current, next) {
            return Ok(next);
        }

        Err(format!(
            "transition blocked: current={:?} event={:?} next={:?}",
            current, event.kind, next
        ))
    }

    fn map_fi_status(iso_status: Option<&str>) -> JourneyState {
        match iso_status.unwrap_or("") {
            "ACTC" => JourneyState::TechnicalAccepted,
            "ACSP" => JourneyState::SettlementInProcess,
            "ACSC" => JourneyState::Settled,
            "RJCT" => JourneyState::Rejected,
            "PDNG" => JourneyState::Pending,
            "RCVD" => JourneyState::Received,
            "ACWC" => JourneyState::Pending,
            _ => JourneyState::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(kind: JourneyEventKind, iso_status: Option<&str>) -> JourneyEvent {
        JourneyEvent {
            source_message: super::super::journey_event::SupportedMessage::Pacs008,
            kind,
            refs: Default::default(),
            iso_status: iso_status.map(str::to_string),
            reason_code: None,
            description: "test".into(),
        }
    }

    #[test]
    fn none_current_plus_customer_payment_initiated_yields_received() {
        let ev = event(JourneyEventKind::CustomerPaymentInitiated, None);
        assert_eq!(StateMachine::transition(None, &ev), Ok(JourneyState::Received));
    }

    #[test]
    fn map_fi_status_covers_every_documented_code() {
        assert_eq!(StateMachine::map_fi_status(Some("ACTC")), JourneyState::TechnicalAccepted);
        assert_eq!(StateMachine::map_fi_status(Some("ACSP")), JourneyState::SettlementInProcess);
        assert_eq!(StateMachine::map_fi_status(Some("ACSC")), JourneyState::Settled);
        assert_eq!(StateMachine::map_fi_status(Some("RJCT")), JourneyState::Rejected);
        assert_eq!(StateMachine::map_fi_status(Some("PDNG")), JourneyState::Pending);
        assert_eq!(StateMachine::map_fi_status(Some("RCVD")), JourneyState::Received);
        assert_eq!(StateMachine::map_fi_status(Some("ACWC")), JourneyState::Pending);
    }

    #[test]
    fn map_fi_status_falls_back_to_pending_for_unknown_or_missing() {
        assert_eq!(StateMachine::map_fi_status(Some("BOGUS")), JourneyState::Pending);
        assert_eq!(StateMachine::map_fi_status(Some("")), JourneyState::Pending);
        assert_eq!(StateMachine::map_fi_status(None), JourneyState::Pending);
    }

    #[test]
    fn fi_status_received_uses_map_fi_status_and_respects_transition_policy() {
        let ev = event(JourneyEventKind::FiStatusReceived, Some("ACTC"));
        // Received -> TechnicalAccepted is an allowed transition.
        assert_eq!(StateMachine::transition(Some(JourneyState::Received), &ev), Ok(JourneyState::TechnicalAccepted));
    }

    #[test]
    fn status_request_received_never_mutates_state() {
        let ev = event(JourneyEventKind::StatusRequestReceived, None);
        let err = StateMachine::transition(Some(JourneyState::Received), &ev).unwrap_err();
        assert!(err.contains("should not mutate journey state"));
    }

    #[test]
    fn reporting_update_never_mutates_state() {
        let ev = event(JourneyEventKind::ReportingUpdate, None);
        let err = StateMachine::transition(Some(JourneyState::Settled), &ev).unwrap_err();
        assert!(err.contains("should not mutate journey state"));
    }

    #[test]
    fn sanctions_hold_event_moves_to_sanctions_hold_from_received() {
        let ev = event(JourneyEventKind::SanctionsHold, None);
        assert_eq!(StateMachine::transition(Some(JourneyState::Received), &ev), Ok(JourneyState::SanctionsHold));
    }

    #[test]
    fn disallowed_transition_is_rejected_with_a_descriptive_error() {
        // Completed is terminal - no event should be able to move it anywhere.
        let ev = event(JourneyEventKind::CustomerPaymentInitiated, None);
        let err = StateMachine::transition(Some(JourneyState::Completed), &ev).unwrap_err();
        assert!(err.contains("transition blocked"));
        assert!(err.contains("Completed"));
    }

    #[test]
    fn claim_non_receipt_opens_an_investigation_from_settled() {
        let ev = event(JourneyEventKind::ClaimNonReceipt, None);
        assert_eq!(StateMachine::transition(Some(JourneyState::Settled), &ev), Ok(JourneyState::InvestigationOpen));
    }

    #[test]
    fn investigation_resolution_rejected_maps_to_recall_rejected() {
        let ev = event(JourneyEventKind::InvestigationResolutionRejected, None);
        assert_eq!(
            StateMachine::transition(Some(JourneyState::InvestigationOpen), &ev),
            Ok(JourneyState::RecallRejected)
        );
    }
}

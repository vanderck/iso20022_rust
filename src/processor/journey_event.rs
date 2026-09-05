#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedMessage {
    Pain001,
    Pain002,
    Pain007,
    Pacs002,
    Pacs004,
    Pacs007,
    Pacs008,
    Pacs028,
    Camt026,
    Camt027,
    Camt029,
    Camt052,
    Camt053,
    Camt054,
    Camt056,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JourneyState {
    Received,
    CustomerAccepted,
    CustomerRejected,
    TechnicalAccepted,
    Pending,
    ReadyForExecution,
    SentToNetwork,
    SettlementInProcess,
    Settled,
    Rejected,
    UnableToApply,
    ReturnRequested,
    Returned,
    CustomerReversalRequested,
    FiReversalRequested,
    RecallRequested,
    RecallRejected,
    Reversed,
    InvestigationOpen,
    InvestigationPendingResponse,
    InvestigationResolved,
    /// Confirmed AML/sanctions match. Funds are held, not returned to the
    /// sender/counterparty — see `FlowCoordinator`'s handling of
    /// `ScreeningResult::Blocked`. Resolution (release or formal rejection
    /// after authorities have been notified) happens outside this app today;
    /// there is deliberately no automatic outbound message tied to entering
    /// this state.
    SanctionsHold,
    Completed,
    Cancelled,
}

impl JourneyState {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            JourneyState::Received => "RECEIVED",
            JourneyState::CustomerAccepted => "CUSTOMER_ACCEPTED",
            JourneyState::CustomerRejected => "CUSTOMER_REJECTED",
            JourneyState::TechnicalAccepted => "TECHNICAL_ACCEPTED",
            JourneyState::Pending => "PENDING",
            JourneyState::ReadyForExecution => "READY_FOR_EXECUTION",
            JourneyState::SentToNetwork => "SENT_TO_NETWORK",
            JourneyState::SettlementInProcess => "SETTLEMENT_IN_PROCESS",
            JourneyState::Settled => "SETTLED",
            JourneyState::Rejected => "REJECTED",
            JourneyState::UnableToApply => "UNABLE_TO_APPLY",
            JourneyState::ReturnRequested => "RETURN_REQUESTED",
            JourneyState::Returned => "RETURNED",
            JourneyState::CustomerReversalRequested => "CUSTOMER_REVERSAL_REQUESTED",
            JourneyState::FiReversalRequested => "FI_REVERSAL_REQUESTED",
            JourneyState::RecallRequested => "RECALL_REQUESTED",
            JourneyState::RecallRejected => "RECALL_REJECTED",
            JourneyState::Reversed => "REVERSED",
            JourneyState::InvestigationOpen => "INVESTIGATION_OPEN",
            JourneyState::InvestigationPendingResponse => "INVESTIGATION_PENDING_RESPONSE",
            JourneyState::InvestigationResolved => "INVESTIGATION_RESOLVED",
            JourneyState::SanctionsHold => "SANCTIONS_HOLD",
            JourneyState::Completed => "COMPLETED",
            JourneyState::Cancelled => "CANCELLED",
        }
    }

    pub fn from_db_value(value: &str) -> Option<Self> {
        match value.trim() {
            "RECEIVED" => Some(Self::Received),
            "CUSTOMER_ACCEPTED" => Some(Self::CustomerAccepted),
            "CUSTOMER_REJECTED" => Some(Self::CustomerRejected),
            "TECHNICAL_ACCEPTED" => Some(Self::TechnicalAccepted),
            "PENDING" => Some(Self::Pending),
            "READY_FOR_EXECUTION" => Some(Self::ReadyForExecution),
            "SENT_TO_NETWORK" => Some(Self::SentToNetwork),
            "SETTLEMENT_IN_PROCESS" => Some(Self::SettlementInProcess),
            "SETTLED" => Some(Self::Settled),
            "REJECTED" => Some(Self::Rejected),
            "UNABLE_TO_APPLY" => Some(Self::UnableToApply),
            "RETURN_REQUESTED" => Some(Self::ReturnRequested),
            "RETURNED" => Some(Self::Returned),
            "CUSTOMER_REVERSAL_REQUESTED" => Some(Self::CustomerReversalRequested),
            "FI_REVERSAL_REQUESTED" => Some(Self::FiReversalRequested),
            "RECALL_REQUESTED" => Some(Self::RecallRequested),
            "RECALL_REJECTED" => Some(Self::RecallRejected),
            "REVERSED" => Some(Self::Reversed),
            "INVESTIGATION_OPEN" => Some(Self::InvestigationOpen),
            "INVESTIGATION_PENDING_RESPONSE" => Some(Self::InvestigationPendingResponse),
            "INVESTIGATION_RESOLVED" => Some(Self::InvestigationResolved),
            "SANCTIONS_HOLD" => Some(Self::SanctionsHold),
            "COMPLETED" => Some(Self::Completed),
            "CANCELLED" => Some(Self::Cancelled),
            _ => None,
        }
    }
}

/// Debtor/creditor party identification captured from the originating
/// customer message (pain.001) or an inbound interbank credit transfer
/// (pacs.008), so that any outbound `pacs.008` generated later for this
/// journey can carry real party data instead of an empty `<Dbtr/>`/`<Cdtr/>`.
#[derive(Debug, Clone, Default)]
pub struct PartyData {
    pub debtor_name: Option<String>,
    pub debtor_account: Option<String>,
    pub creditor_name: Option<String>,
    pub creditor_account: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct JourneyRefs {
    pub original_msg_id: Option<String>,
    pub end_to_end_id: Option<String>,
    pub tx_id: Option<String>,
    pub uetr: Option<String>,
    pub clr_sys_ref: Option<String>,
    pub case_id: Option<String>,
    pub assignment_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JourneyEventKind {
    CustomerPaymentInitiated,
    CustomerStatusAccepted,
    CustomerStatusRejected,
    CustomerStatusPending,
    FiPaymentSent,
    FiStatusReceived,
    FiReturnReceived,
    FiReversalRequested,
    CustomerReversalRequested,
    RecallRequested,
    UnableToApply,
    ClaimNonReceipt,
    InvestigationResolutionAccepted,
    InvestigationResolutionRejected,
    InvestigationResolutionPending,
    StatusRequestReceived,
    ReportingUpdate,
    SanctionsHold,
}

#[derive(Debug, Clone)]
pub struct JourneyEvent {
    pub source_message: SupportedMessage,
    pub kind: JourneyEventKind,
    pub refs: JourneyRefs,
    pub iso_status: Option<String>,
    pub reason_code: Option<String>,
    pub description: String,
}

#[cfg(test)]
const ALL_JOURNEY_STATES: &[JourneyState] = &[
    JourneyState::Received,
    JourneyState::CustomerAccepted,
    JourneyState::CustomerRejected,
    JourneyState::TechnicalAccepted,
    JourneyState::Pending,
    JourneyState::ReadyForExecution,
    JourneyState::SentToNetwork,
    JourneyState::SettlementInProcess,
    JourneyState::Settled,
    JourneyState::Rejected,
    JourneyState::UnableToApply,
    JourneyState::ReturnRequested,
    JourneyState::Returned,
    JourneyState::CustomerReversalRequested,
    JourneyState::FiReversalRequested,
    JourneyState::RecallRequested,
    JourneyState::RecallRejected,
    JourneyState::Reversed,
    JourneyState::InvestigationOpen,
    JourneyState::InvestigationPendingResponse,
    JourneyState::InvestigationResolved,
    JourneyState::SanctionsHold,
    JourneyState::Completed,
    JourneyState::Cancelled,
];

impl JourneyEvent {
    pub fn status_history_code(&self) -> &'static str {
        match self.kind {
            JourneyEventKind::CustomerPaymentInitiated => "CUSTOMER_PAYMENT_INITIATED",
            JourneyEventKind::CustomerStatusAccepted => "CUSTOMER_STATUS_ACCEPTED",
            JourneyEventKind::CustomerStatusRejected => "CUSTOMER_STATUS_REJECTED",
            JourneyEventKind::CustomerStatusPending => "CUSTOMER_STATUS_PENDING",
            JourneyEventKind::FiPaymentSent => "FI_PAYMENT_SENT",
            JourneyEventKind::FiStatusReceived => "FI_STATUS_UPDATE",
            JourneyEventKind::FiReturnReceived => "FI_RETURN",
            JourneyEventKind::FiReversalRequested => "FI_REVERSAL_REQUEST",
            JourneyEventKind::CustomerReversalRequested => "CUSTOMER_REVERSAL_REQUEST",
            JourneyEventKind::RecallRequested => "RECALL_REQUEST",
            JourneyEventKind::UnableToApply => "UNABLE_TO_APPLY",
            JourneyEventKind::ClaimNonReceipt => "CLAIM_NON_RECEIPT",
            JourneyEventKind::InvestigationResolutionAccepted => "INVESTIGATION_RESOLUTION_ACCEPTED",
            JourneyEventKind::InvestigationResolutionRejected => "INVESTIGATION_RESOLUTION_REJECTED",
            JourneyEventKind::InvestigationResolutionPending => "INVESTIGATION_RESOLUTION_PENDING",
            JourneyEventKind::StatusRequestReceived => "STATUS_REQUEST_RECEIVED",
            JourneyEventKind::ReportingUpdate => "REPORTING_UPDATE",
            JourneyEventKind::SanctionsHold => "SANCTIONS_HOLD",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_journey_state_round_trips_through_db_value() {
        for &state in ALL_JOURNEY_STATES {
            let db_value = state.as_db_value();
            assert_eq!(
                JourneyState::from_db_value(db_value),
                Some(state),
                "state {state:?} did not round-trip via db_value {db_value:?}"
            );
        }
    }

    #[test]
    fn from_db_value_rejects_unknown_string() {
        assert_eq!(JourneyState::from_db_value("NOT_A_REAL_STATE"), None);
        assert_eq!(JourneyState::from_db_value(""), None);
        assert_eq!(JourneyState::from_db_value("received"), None); // case-sensitive
    }

    #[test]
    fn from_db_value_trims_whitespace() {
        assert_eq!(JourneyState::from_db_value("  RECEIVED  "), Some(JourneyState::Received));
        assert_eq!(JourneyState::from_db_value("\tCOMPLETED\n"), Some(JourneyState::Completed));
    }

    #[test]
    fn all_db_values_are_unique() {
        let mut values: Vec<&'static str> = ALL_JOURNEY_STATES.iter().map(|s| s.as_db_value()).collect();
        let before = values.len();
        values.sort_unstable();
        values.dedup();
        assert_eq!(values.len(), before, "two JourneyState variants share the same db_value");
    }

    #[test]
    fn every_event_kind_has_a_status_history_code() {
        let kinds = [
            JourneyEventKind::CustomerPaymentInitiated,
            JourneyEventKind::CustomerStatusAccepted,
            JourneyEventKind::CustomerStatusRejected,
            JourneyEventKind::CustomerStatusPending,
            JourneyEventKind::FiPaymentSent,
            JourneyEventKind::FiStatusReceived,
            JourneyEventKind::FiReturnReceived,
            JourneyEventKind::FiReversalRequested,
            JourneyEventKind::CustomerReversalRequested,
            JourneyEventKind::RecallRequested,
            JourneyEventKind::UnableToApply,
            JourneyEventKind::ClaimNonReceipt,
            JourneyEventKind::InvestigationResolutionAccepted,
            JourneyEventKind::InvestigationResolutionRejected,
            JourneyEventKind::InvestigationResolutionPending,
            JourneyEventKind::StatusRequestReceived,
            JourneyEventKind::ReportingUpdate,
            JourneyEventKind::SanctionsHold,
        ];
        let mut codes: Vec<&'static str> = kinds.iter().map(|k| {
            let ev = JourneyEvent {
                source_message: SupportedMessage::Pacs008,
                kind: *k,
                refs: JourneyRefs::default(),
                iso_status: None,
                reason_code: None,
                description: String::new(),
            };
            ev.status_history_code()
        }).collect();
        let before = codes.len();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), before, "two JourneyEventKind variants share the same status_history_code");
    }
}

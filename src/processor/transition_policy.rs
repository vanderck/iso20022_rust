use super::journey_event::JourneyState;

pub struct TransitionPolicy;

impl TransitionPolicy {
    pub fn rank(state: JourneyState) -> i32 {
        match state {
            JourneyState::Completed => 100,
            JourneyState::Returned => 95,
            JourneyState::Reversed => 94,
            JourneyState::Rejected => 93,
            JourneyState::CustomerRejected => 92,
            JourneyState::SanctionsHold => 91,
            JourneyState::UnableToApply => 90,
            JourneyState::InvestigationResolved => 85,
            JourneyState::RecallRejected => 84,
            JourneyState::InvestigationPendingResponse => 82,
            JourneyState::InvestigationOpen => 80,
            JourneyState::RecallRequested => 78,
            JourneyState::FiReversalRequested => 77,
            JourneyState::CustomerReversalRequested => 76,
            JourneyState::Settled => 70,
            JourneyState::SettlementInProcess => 60,
            JourneyState::SentToNetwork => 55,
            JourneyState::ReadyForExecution => 50,
            JourneyState::TechnicalAccepted => 40,
            JourneyState::CustomerAccepted => 35,
            JourneyState::Pending => 30,
            JourneyState::Received => 20,
            JourneyState::Cancelled => 15,
            JourneyState::ReturnRequested => 14,
        }
    }

    pub fn is_terminal(state: JourneyState) -> bool {
        matches!(
            state,
            JourneyState::Completed
                | JourneyState::Returned
                | JourneyState::Reversed
                | JourneyState::Rejected
                | JourneyState::CustomerRejected
                | JourneyState::RecallRejected
                | JourneyState::Cancelled
        )
    }

    pub fn is_allowed(current: Option<JourneyState>, next: JourneyState) -> bool {
        match current {
            None => matches!(next, JourneyState::Received),
            Some(JourneyState::Received) => matches!(
                next,
                JourneyState::CustomerAccepted
                    | JourneyState::CustomerRejected
                    | JourneyState::TechnicalAccepted
                    | JourneyState::Pending
                    | JourneyState::Cancelled
                    | JourneyState::SanctionsHold
            ),
            Some(JourneyState::CustomerAccepted) => matches!(
                next,
                JourneyState::ReadyForExecution
                    | JourneyState::Pending
                    | JourneyState::CustomerRejected
            ),
            Some(JourneyState::ReadyForExecution) => matches!(
                next,
                JourneyState::SentToNetwork | JourneyState::Cancelled | JourneyState::Rejected
            ),
            Some(JourneyState::SentToNetwork) => matches!(
                next,
                JourneyState::TechnicalAccepted
                    | JourneyState::SettlementInProcess
                    | JourneyState::Rejected
                    | JourneyState::CustomerReversalRequested
                    | JourneyState::FiReversalRequested
            ),
            Some(JourneyState::TechnicalAccepted) => matches!(
                next,
                JourneyState::SettlementInProcess | JourneyState::Rejected | JourneyState::Pending
            ),
            Some(JourneyState::SettlementInProcess) => matches!(
                next,
                JourneyState::Settled
                    | JourneyState::Rejected
                    | JourneyState::Pending
                    | JourneyState::RecallRequested
                    | JourneyState::FiReversalRequested
            ),
            Some(JourneyState::Settled) => matches!(
                next,
                JourneyState::Completed
                    | JourneyState::UnableToApply
                    | JourneyState::Returned
                    | JourneyState::RecallRequested
                    | JourneyState::FiReversalRequested
                    | JourneyState::CustomerReversalRequested
                    | JourneyState::InvestigationOpen
            ),
            Some(JourneyState::Pending) => matches!(
                next,
                JourneyState::TechnicalAccepted
                    | JourneyState::SettlementInProcess
                    | JourneyState::Rejected
                    | JourneyState::InvestigationOpen
                    | JourneyState::Cancelled
            ),
            Some(JourneyState::UnableToApply) => matches!(
                next,
                JourneyState::InvestigationOpen
                    | JourneyState::Returned
                    | JourneyState::InvestigationResolved
                    | JourneyState::Completed
            ),
            Some(JourneyState::RecallRequested) => matches!(
                next,
                JourneyState::Returned
                    | JourneyState::RecallRejected
                    | JourneyState::InvestigationPendingResponse
                    | JourneyState::InvestigationResolved
            ),
            Some(JourneyState::FiReversalRequested) => matches!(
                next,
                JourneyState::Reversed
                    | JourneyState::RecallRejected
                    | JourneyState::InvestigationPendingResponse
            ),
            Some(JourneyState::CustomerReversalRequested) => matches!(
                next,
                JourneyState::FiReversalRequested
                    | JourneyState::Reversed
                    | JourneyState::RecallRejected
                    | JourneyState::InvestigationPendingResponse
            ),
            Some(JourneyState::InvestigationOpen) => matches!(
                next,
                JourneyState::InvestigationPendingResponse
                    | JourneyState::InvestigationResolved
                    | JourneyState::Returned
                    | JourneyState::RecallRejected
                    | JourneyState::Reversed
            ),
            Some(JourneyState::InvestigationPendingResponse) => matches!(
                next,
                JourneyState::InvestigationResolved
                    | JourneyState::Returned
                    | JourneyState::RecallRejected
                    | JourneyState::Reversed
            ),
            Some(JourneyState::InvestigationResolved) => matches!(next, JourneyState::Completed),
            // Resolution happens outside this app (compliance/ops confirming
            // a false positive, or formally closing the case after the
            // Trésorerie/CTIF-CFI notification has been made) — this only
            // models the two possible outcomes once that happens.
            Some(JourneyState::SanctionsHold) => matches!(next, JourneyState::Completed | JourneyState::Rejected),
            Some(JourneyState::Returned) => matches!(next, JourneyState::Completed),
            Some(JourneyState::Reversed) => matches!(next, JourneyState::Completed),
            Some(JourneyState::Rejected) => matches!(next, JourneyState::Completed),
            Some(JourneyState::CustomerRejected) => matches!(next, JourneyState::Completed),
            Some(JourneyState::Cancelled) => matches!(next, JourneyState::Completed),
            Some(JourneyState::RecallRejected) => matches!(next, JourneyState::Completed),
            Some(JourneyState::ReturnRequested) => matches!(next, JourneyState::Returned),
            Some(JourneyState::Completed) => false,
        }
    }

    #[cfg(test)]
    fn all_states() -> Vec<JourneyState> {
        vec![
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
        ]
    }

    pub fn should_accept_by_rank(current: Option<JourneyState>, next: JourneyState) -> bool {
        match current {
            None => matches!(next, JourneyState::Received),
            Some(cur) => {
                if Self::is_terminal(cur) {
                    return false;
                }
                Self::rank(next) > Self::rank(cur)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_state_has_a_distinct_rank() {
        let mut ranks: Vec<i32> = TransitionPolicy::all_states().into_iter().map(TransitionPolicy::rank).collect();
        let before = ranks.len();
        ranks.sort_unstable();
        ranks.dedup();
        assert_eq!(ranks.len(), before, "two JourneyState variants share the same rank");
    }

    #[test]
    fn completed_has_the_highest_rank() {
        let max_other = TransitionPolicy::all_states()
            .into_iter()
            .filter(|&s| s != JourneyState::Completed)
            .map(TransitionPolicy::rank)
            .max()
            .unwrap();
        assert!(TransitionPolicy::rank(JourneyState::Completed) > max_other);
    }

    #[test]
    fn is_terminal_matches_the_documented_terminal_set() {
        let terminal = [
            JourneyState::Completed,
            JourneyState::Returned,
            JourneyState::Reversed,
            JourneyState::Rejected,
            JourneyState::CustomerRejected,
            JourneyState::RecallRejected,
            JourneyState::Cancelled,
        ];
        for state in TransitionPolicy::all_states() {
            assert_eq!(
                TransitionPolicy::is_terminal(state),
                terminal.contains(&state),
                "is_terminal({state:?}) mismatch"
            );
        }
    }

    #[test]
    fn no_state_is_allowed_after_completed() {
        for next in TransitionPolicy::all_states() {
            assert!(!TransitionPolicy::is_allowed(Some(JourneyState::Completed), next));
        }
    }

    #[test]
    fn none_current_only_allows_received() {
        assert!(TransitionPolicy::is_allowed(None, JourneyState::Received));
        for next in TransitionPolicy::all_states().into_iter().filter(|&s| s != JourneyState::Received) {
            assert!(!TransitionPolicy::is_allowed(None, next), "None -> {next:?} should not be allowed");
        }
    }

    #[test]
    fn documented_happy_path_transitions_are_allowed() {
        assert!(TransitionPolicy::is_allowed(Some(JourneyState::Received), JourneyState::TechnicalAccepted));
        assert!(TransitionPolicy::is_allowed(Some(JourneyState::TechnicalAccepted), JourneyState::SettlementInProcess));
        assert!(TransitionPolicy::is_allowed(Some(JourneyState::SettlementInProcess), JourneyState::Settled));
        assert!(TransitionPolicy::is_allowed(Some(JourneyState::Settled), JourneyState::Completed));
    }

    #[test]
    fn nonsensical_transitions_are_not_allowed() {
        // Received should never jump straight to Settled.
        assert!(!TransitionPolicy::is_allowed(Some(JourneyState::Received), JourneyState::Settled));
        // A rejected (terminal) journey shouldn't go anywhere except Completed.
        assert!(!TransitionPolicy::is_allowed(Some(JourneyState::Rejected), JourneyState::Received));
        assert!(TransitionPolicy::is_allowed(Some(JourneyState::Rejected), JourneyState::Completed));
    }

    #[test]
    fn should_accept_by_rank_rejects_from_any_terminal_state_even_if_ranked_higher() {
        // Cancelled (rank 15) -> Completed (rank 100) is allowed via is_allowed
        // directly, but should_accept_by_rank alone must still refuse once a
        // state is terminal, regardless of the target's rank.
        for terminal in [
            JourneyState::Completed,
            JourneyState::Returned,
            JourneyState::Reversed,
            JourneyState::Rejected,
            JourneyState::CustomerRejected,
            JourneyState::RecallRejected,
            JourneyState::Cancelled,
        ] {
            for next in TransitionPolicy::all_states() {
                assert!(
                    !TransitionPolicy::should_accept_by_rank(Some(terminal), next),
                    "should_accept_by_rank(Some({terminal:?}), {next:?}) should be false"
                );
            }
        }
    }

    #[test]
    fn should_accept_by_rank_requires_strictly_higher_rank() {
        // Equal rank never "wins" (guards against a no-op / self transition
        // being accepted as a rank-based override).
        assert!(!TransitionPolicy::should_accept_by_rank(Some(JourneyState::Pending), JourneyState::Pending));
        // Lower rank is rejected.
        assert!(!TransitionPolicy::should_accept_by_rank(Some(JourneyState::Settled), JourneyState::Received));
        // Strictly higher rank from a non-terminal state is accepted.
        assert!(TransitionPolicy::should_accept_by_rank(Some(JourneyState::Received), JourneyState::Settled));
    }

    #[test]
    fn should_accept_by_rank_none_current_only_allows_received() {
        assert!(TransitionPolicy::should_accept_by_rank(None, JourneyState::Received));
        assert!(!TransitionPolicy::should_accept_by_rank(None, JourneyState::Completed));
    }
}

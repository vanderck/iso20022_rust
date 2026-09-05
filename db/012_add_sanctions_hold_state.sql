-- =============================================================================
-- Migration 012: journey_state.SANCTIONS_HOLD – confirmed sanctions match
-- =============================================================================
--
-- A confirmed AML/sanctions screening match (ScreeningResult::Blocked) must
-- freeze the funds rather than reject-and-return them to the sender: EU
-- sanctions regulations prohibit making funds/economic resources available
-- to a designated person, which an automatic return can itself violate.
-- This state lets FlowCoordinator record that freeze on the journey instead
-- of routing a Blocked hit through the same reject-and-notify path as an
-- ordinary decline.
--
-- Resolution (confirming a false positive, or formally closing the case
-- after the Belgian Treasury/CTIF-CFI notification has been made) happens
-- outside this app; see TransitionPolicy::is_allowed for the two outcomes
-- this app models (SanctionsHold -> Completed | Rejected).


BEGIN;

ALTER TYPE iso20022.journey_state ADD VALUE IF NOT EXISTS 'SANCTIONS_HOLD';

COMMIT;

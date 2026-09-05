use std::time::Instant;

/// Structured result from every handler's `consume()` method.
///
/// Replaces the freeform `Ok(String)` so that callers (workers in main.rs)
/// can emit structured tracing spans, feed dashboards, and make decisions
/// without parsing log strings.
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub msg_type: String,
    pub msg_id: String,
    pub journey_id: Option<String>,
    pub next_state: Option<String>,
    pub outbound_count: usize,
    pub duration_ms: u64,
    pub warnings: Vec<String>,
}

impl ProcessingResult {
    pub fn builder(msg_type: &str, msg_id: &str) -> ProcessingResultBuilder {
        ProcessingResultBuilder {
            msg_type: msg_type.to_string(),
            msg_id: msg_id.to_string(),
            journey_id: None,
            next_state: None,
            outbound_count: 0,
            start: Instant::now(),
            warnings: Vec::new(),
        }
    }
}

pub struct ProcessingResultBuilder {
    msg_type: String,
    msg_id: String,
    journey_id: Option<String>,
    next_state: Option<String>,
    outbound_count: usize,
    start: Instant,
    warnings: Vec<String>,
}

impl ProcessingResultBuilder {
    pub fn journey(mut self, id: &str) -> Self {
        self.journey_id = Some(id.to_string());
        self
    }

    pub fn journey_opt(mut self, id: Option<String>) -> Self {
        self.journey_id = id;
        self
    }

    pub fn next_state(mut self, state: Option<String>) -> Self {
        self.next_state = state;
        self
    }

    pub fn outbound(mut self, count: usize) -> Self {
        self.outbound_count = count;
        self
    }

    pub fn warn(mut self, msg: impl Into<String>) -> Self {
        self.warnings.push(msg.into());
        self
    }

    pub fn finish(self) -> ProcessingResult {
        let duration_ms = self.start.elapsed().as_millis() as u64;
        ProcessingResult {
            msg_type: self.msg_type,
            msg_id: self.msg_id,
            journey_id: self.journey_id,
            next_state: self.next_state,
            outbound_count: self.outbound_count,
            duration_ms,
            warnings: self.warnings,
        }
    }
}

impl std::fmt::Display for ProcessingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "processed {} msg_id={} journey={} state={} outbound={} duration_ms={}",
            self.msg_type,
            self.msg_id,
            self.journey_id.as_deref().unwrap_or("-"),
            self.next_state.as_deref().unwrap_or("-"),
            self.outbound_count,
            self.duration_ms,
        )?;
        for w in &self.warnings {
            write!(f, " WARN={}", w)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults_to_none_and_zero() {
        let r = ProcessingResult::builder("pacs.008", "MSG1").finish();
        assert_eq!(r.msg_type, "pacs.008");
        assert_eq!(r.msg_id, "MSG1");
        assert_eq!(r.journey_id, None);
        assert_eq!(r.next_state, None);
        assert_eq!(r.outbound_count, 0);
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn builder_chain_sets_all_fields() {
        let r = ProcessingResult::builder("pacs.008", "MSG1")
            .journey("JRN-1")
            .next_state(Some("SETTLED".into()))
            .outbound(2)
            .warn("careful")
            .warn("careful again")
            .finish();
        assert_eq!(r.journey_id.as_deref(), Some("JRN-1"));
        assert_eq!(r.next_state.as_deref(), Some("SETTLED"));
        assert_eq!(r.outbound_count, 2);
        assert_eq!(r.warnings, vec!["careful".to_string(), "careful again".to_string()]);
    }

    #[test]
    fn journey_opt_accepts_none() {
        let r = ProcessingResult::builder("pacs.008", "MSG1").journey_opt(None).finish();
        assert_eq!(r.journey_id, None);
    }

    #[test]
    fn journey_opt_accepts_some() {
        let r = ProcessingResult::builder("pacs.008", "MSG1").journey_opt(Some("JRN-2".into())).finish();
        assert_eq!(r.journey_id.as_deref(), Some("JRN-2"));
    }

    #[test]
    fn display_uses_dash_placeholders_when_journey_and_state_are_absent() {
        let r = ProcessingResult::builder("pacs.008", "MSG1").finish();
        let s = format!("{r}");
        assert!(s.contains("journey=-"));
        assert!(s.contains("state=-"));
        assert!(!s.contains("WARN="));
    }

    #[test]
    fn display_includes_journey_state_and_every_warning() {
        let r = ProcessingResult::builder("pacs.008", "MSG1")
            .journey("JRN-1")
            .next_state(Some("SETTLED".into()))
            .warn("w1")
            .warn("w2")
            .finish();
        let s = format!("{r}");
        assert!(s.contains("journey=JRN-1"));
        assert!(s.contains("state=SETTLED"));
        assert!(s.contains("WARN=w1"));
        assert!(s.contains("WARN=w2"));
    }
}

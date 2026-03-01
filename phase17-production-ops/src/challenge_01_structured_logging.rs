/// Challenge 01 - Structured Logging with Spans
///
/// Build a structured logger that supports:
/// - Log levels with filtering (only emit logs >= min_level)
/// - Structured key-value fields on each log entry
/// - Hierarchical spans (enter/exit) that provide context to child logs
/// - Querying entries by target and formatting them

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub fields: HashMap<String, String>,
    pub span_id: Option<u64>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub id: u64,
    pub name: String,
    pub fields: HashMap<String, String>,
    pub parent_id: Option<u64>,
}

pub struct Logger {
    min_level: LogLevel,
    entries: Vec<LogEntry>,
    spans: HashMap<u64, Span>,
    active_span: Option<u64>,  // currently active span ID
    next_span_id: u64,
    clock: u64,  // simple incrementing timestamp
}

impl Logger {
    pub fn new(min_level: LogLevel) -> Self {
        Logger {
            min_level,
            entries: Vec::new(),
            spans: HashMap::new(),
            active_span: None,
            next_span_id: 1,
            clock: 0,
        }
    }

    /// Log a message at the given level. Attaches current span context.
    /// Only records the entry if level >= min_level.
    pub fn log(
        &mut self,
        level: LogLevel,
        target: &str,
        message: &str,
        fields: HashMap<String, String>,
    ) {
        // TODO: Implement log
        // 1. Check level >= self.min_level
        // 2. Create LogEntry with current span_id and timestamp
        // 3. Increment clock
        // 4. Push to entries
        todo!("Implement log")
    }

    /// Enter a new span. Returns the span ID.
    /// The new span's parent is the currently active span (if any).
    pub fn enter_span(&mut self, name: &str, fields: HashMap<String, String>) -> u64 {
        // TODO: Implement enter_span
        // 1. Create a Span with next_span_id, name, fields, parent_id = active_span
        // 2. Store it in self.spans
        // 3. Set active_span to the new span's id
        // 4. Increment next_span_id
        // 5. Return the span id
        todo!("Implement enter_span")
    }

    /// Exit the current span. Restores the parent span as active.
    pub fn exit_span(&mut self) {
        // TODO: Implement exit_span
        // 1. If active_span is Some, look up the span
        // 2. Set active_span to the span's parent_id
        todo!("Implement exit_span")
    }

    /// Return all recorded log entries.
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    /// Return entries filtered by target string.
    pub fn entries_for_target(&self, target: &str) -> Vec<&LogEntry> {
        // TODO: Filter entries where entry.target == target
        todo!("Implement entries_for_target")
    }

    /// Format a log entry as a human-readable string.
    /// Format: "[LEVEL] target: message {key=value, ...} (span: span_name)"
    pub fn format_entry(&self, entry: &LogEntry) -> String {
        // TODO: Implement formatting
        // 1. Level name in brackets
        // 2. Target and message
        // 3. Fields as {k=v, ...}
        // 4. If span_id is set, append (span: name)
        todo!("Implement format_entry")
    }

    /// Get a span by ID.
    pub fn get_span(&self, id: u64) -> Option<&Span> {
        self.spans.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_filtering() {
        let mut logger = Logger::new(LogLevel::Warn);
        logger.log(LogLevel::Debug, "test", "debug msg", HashMap::new());
        logger.log(LogLevel::Info, "test", "info msg", HashMap::new());
        logger.log(LogLevel::Warn, "test", "warn msg", HashMap::new());
        logger.log(LogLevel::Error, "test", "error msg", HashMap::new());

        assert_eq!(logger.entries().len(), 2);
        assert_eq!(logger.entries()[0].level, LogLevel::Warn);
        assert_eq!(logger.entries()[1].level, LogLevel::Error);
    }

    #[test]
    fn test_span_context() {
        let mut logger = Logger::new(LogLevel::Trace);
        let span_id = logger.enter_span("request", HashMap::new());
        logger.log(LogLevel::Info, "handler", "processing", HashMap::new());
        logger.exit_span();
        logger.log(LogLevel::Info, "handler", "after span", HashMap::new());

        assert_eq!(logger.entries()[0].span_id, Some(span_id));
        assert_eq!(logger.entries()[1].span_id, None);
    }

    #[test]
    fn test_nested_spans() {
        let mut logger = Logger::new(LogLevel::Trace);
        let outer = logger.enter_span("outer", HashMap::new());
        logger.log(LogLevel::Info, "app", "in outer", HashMap::new());

        let inner = logger.enter_span("inner", HashMap::new());
        logger.log(LogLevel::Info, "app", "in inner", HashMap::new());

        // Inner span should have outer as parent
        let inner_span = logger.get_span(inner).unwrap();
        assert_eq!(inner_span.parent_id, Some(outer));

        logger.exit_span(); // exit inner
        logger.log(LogLevel::Info, "app", "back in outer", HashMap::new());

        logger.exit_span(); // exit outer
        logger.log(LogLevel::Info, "app", "no span", HashMap::new());

        assert_eq!(logger.entries()[0].span_id, Some(outer));
        assert_eq!(logger.entries()[1].span_id, Some(inner));
        assert_eq!(logger.entries()[2].span_id, Some(outer));
        assert_eq!(logger.entries()[3].span_id, None);
    }

    #[test]
    fn test_entries_for_target() {
        let mut logger = Logger::new(LogLevel::Trace);
        logger.log(LogLevel::Info, "rpc", "request received", HashMap::new());
        logger.log(LogLevel::Info, "db", "query executed", HashMap::new());
        logger.log(LogLevel::Warn, "rpc", "slow request", HashMap::new());

        let rpc_entries = logger.entries_for_target("rpc");
        assert_eq!(rpc_entries.len(), 2);
        assert_eq!(rpc_entries[0].message, "request received");
        assert_eq!(rpc_entries[1].message, "slow request");
    }

    #[test]
    fn test_format_entry() {
        let mut logger = Logger::new(LogLevel::Trace);
        let span_id = logger.enter_span("req", HashMap::new());

        let mut fields = HashMap::new();
        fields.insert("method".to_string(), "GET".to_string());
        logger.log(LogLevel::Info, "http", "request", fields);

        let formatted = logger.format_entry(&logger.entries()[0]);
        assert!(formatted.contains("[Info]") || formatted.contains("[INFO]"));
        assert!(formatted.contains("http"));
        assert!(formatted.contains("request"));
        assert!(formatted.contains("method=GET") || formatted.contains("method = GET"));
        assert!(formatted.contains("req")); // span name
    }

    #[test]
    fn test_structured_fields() {
        let mut logger = Logger::new(LogLevel::Trace);
        let mut fields = HashMap::new();
        fields.insert("user_id".to_string(), "42".to_string());
        fields.insert("action".to_string(), "login".to_string());
        logger.log(LogLevel::Info, "auth", "user authenticated", fields);

        let entry = &logger.entries()[0];
        assert_eq!(entry.fields.get("user_id").unwrap(), "42");
        assert_eq!(entry.fields.get("action").unwrap(), "login");
    }
}

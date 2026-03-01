//! # Challenge 10.4: Cow<'a, T> Optimization
//!
//! ## Problem
//! Build a `LogParser` that parses Solana-style program log lines using `Cow<'a, str>`
//! fields — borrowing directly from input when no transformation is needed, cloning
//! only when escaping or normalizing.
//!
//! ## Why This Matters
//! Cow is used throughout Agave for efficient string/byte handling. When processing
//! millions of log lines, avoiding unnecessary allocations significantly reduces
//! memory pressure. Interviewers test Cow usage to assess zero-copy parsing skills.
//!
//! ## Requirements
//! - Parse "Program <addr> invoke [<depth>]" → Invoke
//! - Parse "Program log: <message>" → Log (borrow if clean, own if has escapes)
//! - Parse "Program <addr> consumed <n> of <m> compute units" → ComputeUnits
//! - Unknown lines → Unknown (borrow the whole line)
//! - Track allocation count (each Cow::Owned increments)
//!
//! ## Constraints
//! - Fields from input without modification MUST use Cow::Borrowed
//! - Only allocate when transformation is needed (e.g., unescaping \\n, \\t)
//!
//! ## Hints
//! - `str::contains` to check if escaping is needed before allocating
//! - `Cow::Borrowed(slice)` for direct substrings
//! - Track allocations with a counter incremented on each Cow::Owned

use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq)]
pub enum LogKind {
    Invoke,
    Log,
    ComputeUnits,
    Unknown,
}

#[derive(Debug)]
pub struct ParsedLog<'a> {
    pub kind: LogKind,
    pub program: Cow<'a, str>,
    pub message: Cow<'a, str>,
    pub depth: Option<u32>,
    pub compute_used: Option<u64>,
    pub compute_budget: Option<u64>,
}

pub struct LogParser {
    // TODO: implement fields
    // - allocation_count: usize
    _placeholder: (),
}

impl LogParser {
    pub fn new() -> Self {
        todo!("Initialize parser with allocation counter at 0")
    }

    /// Parse a single log line. Borrow from input wherever possible.
    pub fn parse_line<'a>(&mut self, _input: &'a str) -> ParsedLog<'a> {
        todo!(
            "Parse the input line. Use Cow::Borrowed for direct substrings, \
             Cow::Owned only when transformation is needed (e.g., unescaping). \
             Increment allocation_count for each Cow::Owned."
        )
    }

    pub fn parse_batch<'a>(&mut self, _lines: &'a [&'a str]) -> Vec<ParsedLog<'a>> {
        todo!("Parse each line and collect results")
    }

    pub fn allocation_count(&self) -> usize {
        todo!("Return the allocation counter")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_invoke() {
        let mut parser = LogParser::new();
        let line = "Program 11111111111111111111111111111111 invoke [1]";
        let parsed = parser.parse_line(line);
        assert_eq!(parsed.kind, LogKind::Invoke);
        assert_eq!(parsed.program.as_ref(), "11111111111111111111111111111111");
        assert_eq!(parsed.depth, Some(1));
    }

    #[test]
    fn test_invoke_borrows() {
        let mut parser = LogParser::new();
        let line = "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]";
        let parsed = parser.parse_line(line);
        assert!(matches!(parsed.program, Cow::Borrowed(_)));
    }

    #[test]
    fn test_parse_clean_log() {
        let mut parser = LogParser::new();
        let line = "Program log: Transfer successful";
        let parsed = parser.parse_line(line);
        assert_eq!(parsed.kind, LogKind::Log);
        assert_eq!(parsed.message.as_ref(), "Transfer successful");
        assert!(matches!(parsed.message, Cow::Borrowed(_)));
    }

    #[test]
    fn test_parse_log_with_escapes() {
        let mut parser = LogParser::new();
        let line = r"Program log: Error:\nInsufficient funds";
        let parsed = parser.parse_line(line);
        assert_eq!(parsed.kind, LogKind::Log);
        assert!(matches!(parsed.message, Cow::Owned(_)));
        assert!(parsed.message.contains('\n'));
    }

    #[test]
    fn test_parse_compute_units() {
        let mut parser = LogParser::new();
        let line = "Program Vote111 consumed 2904 of 200000 compute units";
        let parsed = parser.parse_line(line);
        assert_eq!(parsed.kind, LogKind::ComputeUnits);
        assert_eq!(parsed.compute_used, Some(2904));
        assert_eq!(parsed.compute_budget, Some(200000));
    }

    #[test]
    fn test_parse_unknown() {
        let mut parser = LogParser::new();
        let line = "some random log output";
        let parsed = parser.parse_line(line);
        assert_eq!(parsed.kind, LogKind::Unknown);
        assert!(matches!(parsed.message, Cow::Borrowed(_)));
    }

    #[test]
    fn test_allocation_count_zero_for_clean() {
        let mut parser = LogParser::new();
        parser.parse_line("Program log: clean");
        parser.parse_line("Program 111 invoke [1]");
        parser.parse_line("unknown line");
        assert_eq!(parser.allocation_count(), 0);
    }

    #[test]
    fn test_allocation_count_with_escapes() {
        let mut parser = LogParser::new();
        parser.parse_line("Program log: clean");
        parser.parse_line(r"Program log: has\nnewline");
        parser.parse_line(r"Program log: has\ttab");
        assert_eq!(parser.allocation_count(), 2);
    }

    #[test]
    fn test_batch_parsing() {
        let mut parser = LogParser::new();
        let lines = vec![
            "Program 111 invoke [1]",
            "Program log: hello",
            "Program 111 consumed 100 of 200 compute units",
        ];
        let results = parser.parse_batch(&lines);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].kind, LogKind::Invoke);
        assert_eq!(results[1].kind, LogKind::Log);
        assert_eq!(results[2].kind, LogKind::ComputeUnits);
    }
}

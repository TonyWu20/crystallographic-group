//! Crate-owned parse error type and the ariadne rendering helper.
//!
//! The public parse entry points return [`HallParseError`]. The spanned
//! `chumsky` errors stay internal to that type. Call [`HallParseError::print`]
//! to render them as ariadne diagnostics.

use std::fmt;

use ariadne::{Color, Config, IndexType, Label, Report, ReportKind, Source};
use chumsky::error::Rich;

/// Error returned by the public Hall-symbol parse entry points.
///
/// Holds the spanned errors from `chumsky` together with the input they
/// refer to. The `Rich` error type is not part of the public API.
#[derive(Debug, Clone)]
pub struct HallParseError<'a> {
    errors: Vec<Rich<'a, char>>,
    input: &'a str,
}

impl<'a> HallParseError<'a> {
    pub(crate) fn new(input: &'a str, errors: Vec<Rich<'a, char>>) -> Self {
        Self { errors, input }
    }

    /// Number of errors in this failure.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Whether this failure holds no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Render each error with ariadne and print it to stdout.
    ///
    /// The spans use byte indices into the input.
    pub fn print(&self) {
        for error in &self.errors {
            render_rich_error(error, "hall-symbol", self.input);
        }
    }
}

impl fmt::Display for HallParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reasons: Vec<String> = self
            .errors
            .iter()
            .map(|error| error.reason().to_string())
            .collect();
        write!(f, "failed to parse Hall symbol: {}", reasons.join("; "))
    }
}

impl std::error::Error for HallParseError<'_> {}

/// Render a single `Rich` error as an ariadne report with byte-index spans.
fn render_rich_error(error: &Rich<char>, source_name: &str, source: &str) {
    Report::build(ReportKind::Error, (source_name, error.span().into_range()))
        .with_config(Config::new().with_index_type(IndexType::Byte))
        .with_message(error.to_string())
        .with_label(
            Label::new((source_name, error.span().into_range()))
                .with_message(error.reason().to_string())
                .with_color(Color::Red),
        )
        .finish()
        .print((source_name, Source::from(source)))
        .expect("printing the ariadne report to stdout should not fail");
}

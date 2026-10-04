//! Crate-owned parse error type and the ariadne rendering helper.
//!
//! The public parse entry points return [`HallParseError`]. The spanned
//! `chumsky` errors stay internal to the parse functions. The error owns the
//! input string and the byte-index span data of each failure, so it has no
//! lifetime parameter and can outlive the input borrow. Call
//! [`HallParseError::render`] for the ariadne report text or
//! [`HallParseError::print`] to write it to stderr.

use std::{fmt, io::Write, ops::Range};

use ariadne::{Color, Config, IndexType, Label, Report, ReportKind, Source};
use chumsky::error::Rich;

/// Error returned by the public Hall-symbol parse entry points.
///
/// Owns the input string and the owned span data (byte offsets and label
/// text) of each `chumsky` error. The error has no lifetime parameter, so it
/// can outlive the input borrow. The `Rich` error type is not part of the
/// public API.
#[derive(Debug, Clone)]
pub struct HallParseError {
    errors: Vec<OwnedParseError>,
    input: String,
}

/// Owned span data for one `chumsky` parse error: the byte offsets into the
/// input and the label text.
#[derive(Debug, Clone)]
struct OwnedParseError {
    span: Range<usize>,
    reason: String,
    description: String,
}

impl HallParseError {
    /// Wrap the raw spanned errors of a failed parse.
    ///
    /// Copies the input and extracts the owned span data from each `Rich`
    /// error. This keeps the `Rich` type internal to the parse functions.
    pub(crate) fn new(input: &str, errors: Vec<Rich<'_, char>>) -> Self {
        let errors = errors
            .into_iter()
            .map(|error| OwnedParseError {
                span: error.span().start..error.span().end,
                reason: error.reason().to_string(),
                description: error.to_string(),
            })
            .collect();
        Self {
            errors,
            input: input.to_string(),
        }
    }

    /// Number of errors in this failure.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Whether this failure holds no errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Render each error as ariadne report text and return the combined
    /// result.
    ///
    /// The spans use byte indices into the owned input. The render path is
    /// infallible: a failed write to the in-memory buffer yields the report
    /// text written so far, never a panic.
    pub fn render(&self) -> String {
        let mut buffer = Vec::new();
        for error in &self.errors {
            let report = build_report(error, "hall-symbol");
            let _ = report.write(("hall-symbol", Source::from(self.input.as_str())), &mut buffer);
        }
        String::from_utf8_lossy(&buffer).into_owned()
    }

    /// Write the ariadne report text to stderr.
    ///
    /// A convenience wrapper around [`HallParseError::render`]. The write is
    /// best-effort: a failed write is ignored so the error path never
    /// panics.
    pub fn print(&self) {
        let _ = write!(std::io::stderr(), "{}", self.render());
    }
}

impl fmt::Display for HallParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reasons: Vec<String> = self
            .errors
            .iter()
            .map(|error| error.reason.clone())
            .collect();
        write!(f, "failed to parse Hall symbol: {}", reasons.join("; "))
    }
}

impl std::error::Error for HallParseError {}

/// Build one ariadne report from the owned span data, with byte-index spans.
fn build_report<'s>(error: &OwnedParseError, source_name: &'s str) -> Report<'s, (&'s str, Range<usize>)> {
    Report::build(ReportKind::Error, (source_name, error.span.clone()))
        .with_config(Config::new().with_index_type(IndexType::Byte))
        .with_message(error.description.clone())
        .with_label(
            Label::new((source_name, error.span.clone()))
                .with_message(error.reason.clone())
                .with_color(Color::Red),
        )
        .finish()
}

#[cfg(test)]
mod test {
    use super::HallParseError;

    /// Compile-time check: `HallParseError` has no lifetime parameter, so it
    /// is `'static` and can outlive any input borrow.
    const fn assert_static<T: 'static>() {}

    const _: () = {
        assert_static::<HallParseError>();
    };
}

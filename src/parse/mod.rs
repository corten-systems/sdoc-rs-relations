mod relation;
pub mod tree;

use anyhow::{bail, Result};
use serde::Serialize;
use std::cmp::Ordering;

use crate::parse::relation::{is_opening, next, relation};
use std::collections::BTreeMap;
use std::num::NonZeroUsize;

/// Line and column numbers are 1-based and 0-based, respectively,
/// consistent with the definition in [`proc_macro2::LineColumn`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.LineColumn.html).
/// However, we specify `line` as a `NonZeroUsize` to make this more explicit.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct LineColumn {
    /// The 1-indexed line in the source file on which the span starts or ends (inclusive).
    pub line: NonZeroUsize,
    /// The 0-indexed column (in UTF-8 characters) in the source file on which the span starts or ends (inclusive).
    pub column: usize,
}

/// Copied from [`proc_macro2::LineColumn`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.LineColumn.html).
impl PartialOrd for LineColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Copied from [`proc_macro2::LineColumn`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.LineColumn.html).
impl Ord for LineColumn {
    fn cmp(&self, other: &Self) -> Ordering {
        self.line
            .cmp(&other.line)
            .then(self.column.cmp(&other.column))
    }
}

impl From<proc_macro2::LineColumn> for LineColumn {
    fn from(lc: proc_macro2::LineColumn) -> Self {
        LineColumn {
            line: NonZeroUsize::new(lc.line)
                .expect("proc_macro2::LineColumn line numbers should be non-zero"),
            column: lc.column,
        }
    }
}

/// Copied from [`proc_macro2::Span.html`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Span {
    pub start: LineColumn,
    pub end: LineColumn,
}

/// Easy conversion from [`proc_macro2::Span.html`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html).
impl From<proc_macro2::Span> for Span {
    fn from(span: proc_macro2::Span) -> Self {
        Span {
            start: span.start().into(),
            end: span.end().into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Relation {
    pub identifier: String,
    pub attributes: BTreeMap<String, String>,
}

pub fn relations_from_doc(mut input: &str) -> Result<Vec<Relation>> {
    let mut relations = Vec::new();

    // Find the next candidate tag, but it might only be a partial match
    while let Ok((next, _)) = next(input) {
        input = next;

        // Make sure it is a complete, proper opening tag
        if is_opening(input) {
            // It is an opening tag, so let's try to parse it
            match relation(input) {
                Ok((remaining, relation)) => {
                    relations.push(relation);
                    input = remaining;
                }
                Err(_) => {
                    const LENGTH: usize = 32;
                    let truncated = if input.len() > LENGTH {
                        format!("{}...", &input[..LENGTH])
                    } else {
                        input.to_string()
                    };
                    bail!("malformed: {truncated}"); // note a likely error
                }
            }
        }
    }

    Ok(relations)
}

#[test]
fn test_relations_from_doc() -> Result<()> {
    let relations = relations_from_doc(
        "prefix @relation(ident1, attr1=val1) middle @relation(ident2, attr2=val2) suffix",
    )?;
    assert_eq!(relations.len(), 2);
    assert_eq!(relations[0].identifier, "ident1");
    assert_eq!(relations[1].identifier, "ident2");
    assert_eq!(relations[0].attributes.len(), 1);
    assert_eq!(relations[1].attributes.len(), 1);
    assert_eq!(relations[0].attributes["attr1"], "val1");
    assert_eq!(relations[1].attributes["attr2"], "val2");
    Ok(())
}

#[test]
fn test_relations_from_doc_failure() {
    let relations = relations_from_doc("prefix @relation(ident, attr=va{}ue) suffix");
    assert!(relations.is_err());
    if let Err(err) = relations {
        let message = err.to_string();
        assert!(message.starts_with("malformed: @relation(ident, attr=va{}ue)"));
    }
}

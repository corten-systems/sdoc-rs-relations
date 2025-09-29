mod relation;

use anyhow::{bail, Result};
use serde::Serialize;

use std::collections::BTreeMap;

use crate::parse::relation::{is_opening, next, relation};

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
    let relations = relations_from_doc(
        "prefix @relation(ident, attr=val&ue) suffix",
    );
    assert!(relations.is_err());
    if let Err(err) = relations {
        let message = err.to_string();
        assert!(message.starts_with("malformed: @relation(ident, attr=val&ue)"));
    }
}

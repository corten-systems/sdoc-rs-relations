mod relation;

use serde::Serialize;
use std::collections::BTreeMap;

use crate::parse::relation::{next, relation};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Relation {
    pub identifier: String,
    pub attributes: BTreeMap<String, String>,
}

pub fn relations_from_doc(mut input: &str) -> Vec<Relation> {
    let mut relations = Vec::new();

    while let Ok((next, _)) = next(input) {
        input = next;
        while let Ok((remaining, relation)) = relation(input) {
            relations.push(relation);
            input = remaining;
        }
    }

    relations
}

#[test]
fn test_relations_from_doc() {
    relations_from_doc(
        "prefix @relation(ident1, attr1=val1) middle @relation(ident2, attr2=val2) suffix",
    );
}

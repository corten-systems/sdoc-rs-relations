use anyhow::{anyhow, Context, Result};

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::parse;
use crate::parse::tree::Scope;
use crate::parse::Span;

/// A type-tagged hexadecimal hash.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Hash {
    Sha256(String),
}

impl From<&Vec<u8>> for Hash {
    fn from(bytes: &Vec<u8>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hash = format!("{:x}", hasher.finalize());
        Self::Sha256(hash)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Relations {
    pub file: PathBuf,
    pub hash: Hash,
    pub relations: Vec<Relation>,
}

/// This is the information we require to [link source code to requirements](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_01_user_guide.html#10.2-Linking-source-code-to-requirements).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Relation {
    #[serde(rename = "relation")]
    pub ident: String,
    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        rename = "attributes"
    )]
    pub attrs: BTreeMap<String, String>,
    pub scope: Scope,
    pub span: Span,
}

/// Analyze the provided Rust source file and find relations between items, storing file paths relative to the crate root.
pub fn find_relations<P: AsRef<Path>, R: AsRef<Path>>(path: &P, prefix: &R) -> Result<Relations> {
    let path = path.as_ref();
    let prefix = prefix.as_ref();

    // Read the file into a byte array
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read source file: {}", path.display()))?;

    // Calculate SHA256 hash as hexadecimal string
    let hash = Hash::from(&bytes);

    // Convert byte buffer to string
    let src = String::from_utf8(bytes).with_context(|| {
        format!(
            "failed to convert bytes to UTF-8 string: {}",
            path.display()
        )
    })?;

    let file_ast = syn::parse_file(&src).map_err(|err| {
        let span = err.span();
        let start = span.start();
        anyhow!(
            "failed parsing\n - file: {}\n - error: {}\n - line: {}, column: {}",
            path.display(),
            err,
            start.line,
            start.column
        )
    })?;

    // Determine the path to store in `Relation.file` relative to the crate root
    let relative_path = path.strip_prefix(prefix).unwrap_or(path);

    let mut relations = Relations {
        file: relative_path.to_path_buf(),
        hash: hash.clone(),
        relations: vec![],
    };

    // Parse the file and collect all the places
    let places = parse::tree::Visitor::visit(&file_ast);
    for place in &places {
        for doc in &place.docs {
            for relation in parse::relations_from_doc(doc)? {
                let relation = Relation {
                    ident: relation.identifier,
                    attrs: relation.attributes,
                    scope: place.scope,
                    span: place.span,
                };
                relations.relations.push(relation);
            }
        }
    }

    Ok(relations)
}

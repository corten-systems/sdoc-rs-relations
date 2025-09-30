use anyhow::{anyhow, Context, Result};

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::parse;
use crate::parse::tree::Scope;
use crate::parse::Span;

/// Copied from [`syn::Item`](https://docs.rs/syn/latest/syn/enum.Item.html).
/// This is exhaustive, but when we convert from `syn::Item` to `Item,` we make it
/// an error to match the wildcard pattern since `syn:Item` is `non-exhaustive`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(tag = "type")]
pub enum Item {
    Const,
    Enum,
    ExternCrate,
    Fn,
    ForeignMod,
    Impl,
    Macro,
    Mod,
    Static,
    Struct,
    Trait,
    TraitAlias,
    Type,
    Union,
    Use,
}

impl TryFrom<&syn::Item> for Item {
    type Error = anyhow::Error;
    fn try_from(item: &syn::Item) -> Result<Self> {
        match item {
            syn::Item::Const(_) => Ok(Item::Const),
            syn::Item::Enum(_) => Ok(Item::Enum),
            syn::Item::ExternCrate(_) => Ok(Item::ExternCrate),
            syn::Item::Fn(_) => Ok(Item::Fn),
            syn::Item::ForeignMod(_) => Ok(Item::ForeignMod),
            syn::Item::Impl(_) => Ok(Item::Impl),
            syn::Item::Macro(_) => Ok(Item::Macro),
            syn::Item::Mod(_) => Ok(Item::Mod),
            syn::Item::Static(_) => Ok(Item::Static),
            syn::Item::Struct(_) => Ok(Item::Struct),
            syn::Item::Trait(_) => Ok(Item::Trait),
            syn::Item::TraitAlias(_) => Ok(Item::TraitAlias),
            syn::Item::Type(_) => Ok(Item::Type),
            syn::Item::Union(_) => Ok(Item::Union),
            syn::Item::Use(_) => Ok(Item::Use),
            syn::Item::Verbatim(_) => Err(anyhow!("unsupported syn::Item variant found")),
            _ => Err(anyhow!("non-exhaustive syn::Item variant found")),
        }
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Relations {
    pub file: PathBuf,
    pub hash: Hash,
    pub relations: Vec<Relation>,
}

/// This is the information we require to [link source code to requirements](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_01_user_guide.html#10.2-Linking-source-code-to-requirements).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
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
pub fn find_relations<P: AsRef<Path>, R: AsRef<Path>>(
    file: &P,
    crate_root: &R,
) -> Result<Relations> {
    let path = file.as_ref();
    let crate_root = crate_root.as_ref();

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
    let relative_path = path.strip_prefix(crate_root).unwrap_or(path);

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

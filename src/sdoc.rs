use anyhow::{anyhow, Context, Result};

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};
use syn::spanned::Spanned;

use crate::parse;

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

/// Copied from [`proc_macro2::Span.html`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Span {
    pub start: LineColumn,
    pub end: LineColumn,
}

/// This is the information we require to [link source code to requirements](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_01_user_guide.html#10.2-Linking-source-code-to-requirements).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Relation {
    pub file: PathBuf,
    pub hash: String,
    pub ident: String,
    pub attrs: BTreeMap<String, String>,
    pub item: Item,
    pub span: Span,
}

/// Analyze the provided Rust source file and find relations between items, storing file paths relative to the crate root.
pub fn find_relations<P: AsRef<Path>, R: AsRef<Path>>(
    file: &P,
    crate_root: &R,
) -> Result<Vec<Relation>> {
    let path = file.as_ref();
    let crate_root = crate_root.as_ref();

    // Read the file into a byte array
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read source file: {}", path.display()))?;

    // Calculate SHA256 hash as hexadecimal string
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = format!("sha256-{:x}", hasher.finalize());

    // Convert byte buffer to string
    let src = String::from_utf8(bytes).with_context(|| {
        format!(
            "failed to convert bytes to UTF-8 string: {}",
            path.display()
        )
    })?;

    let syntax = syn::parse_file(&src)
        .with_context(|| format!("failed to parse Rust file: {}", path.display()))?;

    // Determine the path to store in `Relation.file` relative to the crate root
    let relative_path = path.strip_prefix(crate_root).unwrap_or(path);

    let mut out: Vec<Relation> = Vec::new();

    // Process file-level inner doc comments (//! ...)
    collect_file_level_relations(relative_path, &hash, &syntax, &mut out)?;

    // Walk all items recursively
    for item in &syntax.items {
        collect_item_relations(relative_path, &hash, item, &mut out)?;
    }

    Ok(out)
}
fn collect_file_level_relations(
    path: &Path,
    hash: &str,
    file: &syn::File,
    out: &mut Vec<Relation>,
) -> Result<()> {
    let docs = doc_strings_from_attrs(&file.attrs);

    if docs.is_empty() {
        return Ok(());
    }

    // Compute a span that roughly covers the file's items
    let (start, end) = file_span_from_items(&file.items);
    for doc in docs {
        for relation in parse::relations_from_doc(&doc)? {
            out.push(Relation {
                file: path.to_path_buf(),
                hash: hash.to_string(),
                ident: relation.identifier,
                attrs: relation.attributes,
                item: Item::Mod, // module, crate root, or submodule file
                span: Span { start, end },
            });
        }
    }

    Ok(())
}

fn item_attrs(item: &syn::Item) -> Result<&[syn::Attribute]> {
    match item {
        syn::Item::Const(i) => Ok(&i.attrs),
        syn::Item::Enum(i) => Ok(&i.attrs),
        syn::Item::ExternCrate(i) => Ok(&i.attrs),
        syn::Item::Fn(i) => Ok(&i.attrs),
        syn::Item::ForeignMod(i) => Ok(&i.attrs),
        syn::Item::Impl(i) => Ok(&i.attrs),
        syn::Item::Macro(i) => Ok(&i.attrs),
        syn::Item::Mod(i) => Ok(&i.attrs),
        syn::Item::Static(i) => Ok(&i.attrs),
        syn::Item::Struct(i) => Ok(&i.attrs),
        syn::Item::Trait(i) => Ok(&i.attrs),
        syn::Item::TraitAlias(i) => Ok(&i.attrs),
        syn::Item::Type(i) => Ok(&i.attrs),
        syn::Item::Union(i) => Ok(&i.attrs),
        syn::Item::Use(i) => Ok(&i.attrs),
        syn::Item::Verbatim(_) => Err(anyhow!("unsupported syn::Item variant")),
        _ => Err(anyhow!("non-exhaustive syn::Item variant")),
    }
}

fn collect_item_relations(
    path: &Path,
    hash: &str,
    item: &syn::Item,
    out: &mut Vec<Relation>,
) -> Result<()> {
    // Extract doc strings from the item's attributes (outer and inner)
    let docs = doc_strings_from_attrs(item_attrs(item)?);

    let span = item.span();
    let start = span.start();
    let end = span.end();

    for doc in docs {
        for relation in parse::relations_from_doc(&doc)? {
            out.push(Relation {
                file: path.to_path_buf(),
                hash: hash.to_string(),
                ident: relation.identifier,
                attrs: relation.attributes,
                item: Item::try_from(item)?,
                span: Span {
                    start: to_line_col(start),
                    end: to_line_col(end),
                },
            });
        }
    }

    // Recurse into module contents if any
    if let syn::Item::Mod(m) = item
        && let Some((_brace, items)) = &m.content
    {
        for it in items {
            collect_item_relations(path, hash, it, out)?;
        }
    }

    Ok(())
}

fn to_line_col(lc: proc_macro2::LineColumn) -> LineColumn {
    // proc_macro2::LineColumn uses 1-based lines, so this should never be zero
    LineColumn {
        line: NonZeroUsize::new(lc.line)
            .expect("proc_macro2::LineColumn line numbers are 1-based and non-zero"),
        column: lc.column,
    }
}

fn file_span_from_items(items: &[syn::Item]) -> (LineColumn, LineColumn) {
    // Fold over items once, tracking the minimal start and maximal end in our own
    // Ord-enabled LineColumn representation.
    let acc: Option<(LineColumn, LineColumn)> = items.iter().fold(None, |acc, it| {
        let s = it.span();
        let start = to_line_col(s.start());
        let end = to_line_col(s.end());
        match acc {
            None => Some((start, end)),
            Some((min_start, max_end)) => Some((min_start.min(start), max_end.max(end))),
        }
    });

    if let Some((s, e)) = acc {
        (s, e)
    } else {
        // Fallback for empty files: both at (1,0)
        let lc = proc_macro2::LineColumn { line: 1, column: 0 };
        let lc = to_line_col(lc);
        (lc, lc)
    }
}

fn doc_strings_from_attrs(attrs: &[syn::Attribute]) -> Vec<String> {
    use syn::{Meta, MetaNameValue};
    let mut out = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }
        if let Meta::NameValue(MetaNameValue {
            value:
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(ls),
                    ..
                }),
            ..
        }) = &attr.meta
        {
            out.push(ls.value());
        }
    }
    out
}

use anyhow::{Context, Result};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;

/// Line and column numbers are 1-based and 0-based, respectively,
/// consistent with the definition in [`proc_macro2::LineColumn`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.LineColumn.html).
/// However, we specify `line` as a `NonZeroUsize` to make this more explicit.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
/// This is exhaustive, but when we convert from `syn::Item` to `CodeType` we
/// make it an error to match the wildcard pattern since `syn:Item` is `non-exhaustive`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    Verbatim,
}

impl From<&syn::Item> for Item {
    fn from(item: &syn::Item) -> Self {
        match item {
            // FIXME
            syn::Item::Mod(_) => Item::Mod,
            syn::Item::Struct(_) => Item::Struct,
            syn::Item::Enum(_) => Item::Enum,
            syn::Item::Fn(_) => Item::Fn,
            syn::Item::Trait(_) => Item::Trait,
            syn::Item::Impl(_) => Item::Impl,
            syn::Item::Const(_) => Item::Const,
            syn::Item::Static(_) => Item::Static,
            syn::Item::Type(_) => Item::Type,
            syn::Item::Union(_) => Item::Union,
            syn::Item::Macro(_) => Item::Macro,
            syn::Item::Use(_) => Item::Use,
            syn::Item::ForeignMod(_) => Item::ForeignMod,
            syn::Item::ExternCrate(_) => Item::ExternCrate,
            syn::Item::TraitAlias(_) => Item::Trait,
            syn::Item::Verbatim(_) => Item::Other,
            _ => Item::Other,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Relation {
    pub path: PathBuf,
    pub relation: String,
    pub attrs: BTreeMap<String, String>,
    pub item: Item,
    pub from: LineColumn,
    pub to: LineColumn,
}

/// Analyze the provided Rust source file and find relations between items.
pub fn find_relations<P: AsRef<Path>>(file: &P) -> Result<Vec<Relation>> {
    let path = file.as_ref();
    let src = fs::read_to_string(path)
        .with_context(|| format!("failed to read source file: {}", path.display()))?;

    let syntax = syn::parse_file(&src)
        .with_context(|| format!("failed to parse Rust file: {}", path.display()))?;

    let mut out: Vec<Relation> = Vec::new();

    // Process file-level inner doc comments (//! ...)
    collect_file_level_relations(path, &syntax, &mut out);

    // Walk all items recursively
    for item in &syntax.items {
        collect_item_relations(path, item, &mut out);
    }

    Ok(out)
}

fn collect_file_level_relations(path: &Path, file: &syn::File, out: &mut Vec<Relation>) {
    let docs = doc_strings_from_attrs(&file.attrs);

    if docs.is_empty() {
        return;
    }

    // Compute a span that roughly covers the file's items
    let (start, end) = file_span_from_items(&file.items);
    for doc in docs {
        for (rel_id, kvs) in parse_relations_from_doc(&doc) {
            out.push(Relation {
                path: path.to_path_buf(),
                relation: rel_id,
                attrs: kvs,
                item: Item::Mod, // file as a module (crate root / submodule file)
                from: start,
                to: end,
            });
        }
    }
}

fn item_attrs(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        // FIXME
        syn::Item::Const(i) => &i.attrs,
        syn::Item::Enum(i) => &i.attrs,
        syn::Item::ExternCrate(i) => &i.attrs,
        syn::Item::Fn(i) => &i.attrs,
        syn::Item::ForeignMod(i) => &i.attrs,
        syn::Item::Impl(i) => &i.attrs,
        syn::Item::Macro(i) => &i.attrs,
        syn::Item::Mod(i) => &i.attrs,
        syn::Item::Static(i) => &i.attrs,
        syn::Item::Struct(i) => &i.attrs,
        syn::Item::Trait(i) => &i.attrs,
        syn::Item::TraitAlias(i) => &i.attrs,
        syn::Item::Type(i) => &i.attrs,
        syn::Item::Union(i) => &i.attrs,
        syn::Item::Use(i) => &i.attrs,
        syn::Item::Verbatim(_) => &[], // FIXME
        _ => &[],           // FIXME
    }
}

fn collect_item_relations(path: &Path, item: &syn::Item, out: &mut Vec<Relation>) {
    // Extract doc strings from the item's attributes (outer and inner)
    let docs = doc_strings_from_attrs(item_attrs(item));

    let span = item.span();
    let start = span.start();
    let end = span.end();

    for doc in docs {
        for (rel_id, kvs) in parse_relations_from_doc(&doc) {
            out.push(Relation {
                path: path.to_path_buf(),
                relation: rel_id,
                attrs: kvs,
                item: Item::from(item),
                from: to_line_col(start),
                to: to_line_col(end),
            });
        }
    }

    // Recurse into module contents if any
    if let syn::Item::Mod(m) = item {
        if let Some((_brace, items)) = &m.content {
            for it in items {
                collect_item_relations(path, it, out);
            }
        }
    }
}

fn to_line_col(lc: proc_macro2::LineColumn) -> LineColumn {
    LineColumn {
        line: NonZeroUsize::new(lc.line).unwrap_or(NonZeroUsize::new(1).unwrap()),
        column: lc.column,
    }
}

fn file_span_from_items(items: &[syn::Item]) -> (LineColumn, LineColumn) {
    let mut start: Option<proc_macro2::LineColumn> = None;
    let mut end: Option<proc_macro2::LineColumn> = None;

    for it in items {
        let s = it.span();
        let s_start = s.start();
        let s_end = s.end();
        if start.map_or(true, |cur| {
            s_start.line < cur.line || (s_start.line == cur.line && s_start.column < cur.column)
        }) {
            start = Some(s_start);
        }
        if end.map_or(true, |cur| {
            s_end.line > cur.line || (s_end.line == cur.line && s_end.column > cur.column)
        }) {
            end = Some(s_end);
        }
    }

    match (start, end) {
        (Some(s), Some(e)) => (to_line_col(s), to_line_col(e)),
        _ => {
            // Fallback for empty files: both at (1,0)
            let lc = proc_macro2::LineColumn { line: 1, column: 0 };
            let lc = to_line_col(lc);
            (lc, lc)
        }
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

fn parse_relations_from_doc(doc: &str) -> Vec<(String, BTreeMap<String, String>)> {
    let mut results = Vec::new();
    let mut rest = doc;
    while let Some(idx) = rest.find("@relation(") {
        let after = &rest[idx + "@relation(".len()..];
        if let Some(end_idx) = find_matching_paren(after) {
            let inside = &after[..end_idx];
            if let Some((rel, attrs)) = parse_inside_relation(inside) {
                results.push((rel, attrs));
            }
            rest = &after[end_idx + 1..];
        } else {
            break; // no closing ')'
        }
    }
    results
}

fn find_matching_paren(s: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (i, ch) in s.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    return Some(i);
                }
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_inside_relation(s: &str) -> Option<(String, BTreeMap<String, String>)> {
    // Split at first comma to get the relation id
    let (id_part, rest) = match s.split_once(',') {
        Some((a, b)) => (a.trim(), Some(b)),
        None => (s.trim(), None),
    };
    if id_part.is_empty() {
        return None;
    }

    let relation = id_part.trim_matches(|c: char| c.is_whitespace());
    let mut map: BTreeMap<String, String> = BTreeMap::new();

    if let Some(rest) = rest {
        for piece in rest.split(',') {
            let piece = piece.trim();
            if piece.is_empty() {
                continue;
            }
            if let Some((k, v)) = piece.split_once('=') {
                let k = k.trim().to_string();
                let v = v.trim().trim_matches('"').to_string();
                if !k.is_empty() {
                    map.insert(k, v);
                }
            }
        }
    }

    Some((relation.to_string(), map))
}

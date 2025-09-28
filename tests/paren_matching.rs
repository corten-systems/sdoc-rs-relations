use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn relation_value_with_parentheses_in_string() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        "{}",
        r#"
//! Doc with relation that has parentheses inside quoted value
/// @relation(REQ-777, note="calls func(a) and then func(b) before returning")
fn foo() {}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "REQ-777");
    assert_eq!(
        relation.attrs.get("note"),
        Some(&"calls func(a) and then func(b) before returning".to_string())
    );
}

#[test]
fn relation_does_not_nest_parentheses() {
    // Even if there is an extra '(' before the closing ')', we should stop at the first ')'.
    // This leads to an invalid TAG (contains '(') and thus no relation should be parsed.
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        "{}",
        r#"
//! Bad tag with extra paren before close
/// @relation(REQ-888(, status="draft") and more text
fn bar() {}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    // Because '(' is not allowed in TAG, parser should reject it -> 0 relations
    assert_eq!(relations.len(), 0);
}

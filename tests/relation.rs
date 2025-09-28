use std::io::Write;

use tempfile::NamedTempFile;

#[test]
fn test_relation_in_file_level_doc_comment_zero_attributes() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
//! This is a file-level doc comment with @relation(REQ-001)

fn main() {{
    println!("Hello, world!");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "REQ-001");
    assert!(relation.attrs.is_empty());
}

#[test]
fn test_relation_in_file_level_doc_comment_one_attribute() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
//! This is a file-level doc comment with @relation(REQ-002, status="draft")

fn main() {{
    println!("Hello, world!");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "REQ-002");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("status"), Some(&"draft".to_string()));
}

#[test]
fn test_relation_in_file_level_doc_comment_two_attributes() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
//! This is a file-level doc comment with @relation(REQ-003, status="approved", priority="high")

fn main() {{
    println!("Hello, world!");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "REQ-003");
    assert_eq!(relation.attrs.len(), 2);
    assert_eq!(relation.attrs.get("status"), Some(&"approved".to_string()));
    assert_eq!(relation.attrs.get("priority"), Some(&"high".to_string()));
}

#[test]
fn test_relation_in_function_doc_comment_zero_attributes() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// This function has @relation(FUNC-001)
fn test_function() {{
    println!("Test");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "FUNC-001");
    assert!(relation.attrs.is_empty());
}

#[test]
fn test_relation_in_function_doc_comment_one_attribute() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// This function has @relation(FUNC-002, complexity="low")
fn test_function() {{
    println!("Test");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "FUNC-002");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("complexity"), Some(&"low".to_string()));
}

#[test]
fn test_relation_in_function_doc_comment_two_attributes() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// This function has @relation(FUNC-003, complexity="medium", tested="true")
fn test_function() {{
    println!("Test");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "FUNC-003");
    assert_eq!(relation.attrs.len(), 2);
    assert_eq!(
        relation.attrs.get("complexity"),
        Some(&"medium".to_string())
    );
    assert_eq!(relation.attrs.get("tested"), Some(&"true".to_string()));
}

#[test]
fn test_relation_in_struct_doc_comment() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// This struct represents @relation(STRUCT-001, type="data")
struct TestStruct {{
    field: String,
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "STRUCT-001");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("type"), Some(&"data".to_string()));
}

#[test]
fn test_relation_in_impl_doc_comment() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
struct TestStruct;

/// Implementation with @relation(IMPL-001, purpose="utility")
impl TestStruct {{
    pub fn new() -> Self {{
        TestStruct
    }}
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "IMPL-001");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("purpose"), Some(&"utility".to_string()));
}

#[test]
fn test_multiple_relations_in_single_file() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
//! File level @relation(FILE-001, scope="global")

/// Function with @relation(FUNC-100, priority="high")
fn first_function() {{}}

/// Another function with @relation(FUNC-200, priority="low")  
fn second_function() {{}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 3);

    // Sort relations by relation name to ensure consistent testing
    let mut sorted_relations = relations.clone();
    sorted_relations.sort_by(|a, b| a.relation.cmp(&b.relation));

    assert_eq!(sorted_relations[0].relation, "FILE-001");
    assert_eq!(
        sorted_relations[0].attrs.get("scope"),
        Some(&"global".to_string())
    );

    assert_eq!(sorted_relations[1].relation, "FUNC-100");
    assert_eq!(
        sorted_relations[1].attrs.get("priority"),
        Some(&"high".to_string())
    );

    assert_eq!(sorted_relations[2].relation, "FUNC-200");
    assert_eq!(
        sorted_relations[2].attrs.get("priority"),
        Some(&"low".to_string())
    );
}

#[test]
fn test_relation_with_nested_quotes() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// Function with @relation(QUOTE-001, description="handles \"quoted\" strings")
fn quote_function() {{
    println!("Handle quotes");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "QUOTE-001");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(
        relation.attrs.get("description"),
        Some(&r#"handles \"quoted\" strings"#.to_string())
    );
}

#[test]
fn test_relation_in_multiline_doc_comment() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// This is a multiline
/// doc comment that has
/// @relation(MULTI-001, type="multiline")
/// across multiple lines
fn multiline_function() {{
    println!("Multi-line");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "MULTI-001");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("type"), Some(&"multiline".to_string()));
}

#[test]
fn test_relation_in_module_doc_comment() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// This module handles @relation(MOD-001, purpose="utilities")
mod test_module {{
    pub fn helper() {{}}
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "MOD-001");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(
        relation.attrs.get("purpose"),
        Some(&"utilities".to_string())
    );
}

#[test]
fn test_relation_with_special_characters_in_attributes() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// Function with @relation(SPEC-001, description="handles input/output operations", version="1.0")
fn io_function() {{
    println!("I/O operations");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "SPEC-001");
    assert_eq!(relation.attrs.len(), 2);
    assert_eq!(
        relation.attrs.get("description"),
        Some(&"handles input/output operations".to_string())
    );
    assert_eq!(relation.attrs.get("version"), Some(&"1.0".to_string()));
}

#[test]
fn test_no_relations_in_regular_comments() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
// This is a regular comment with @relation(SHOULD-NOT-APPEAR)
/* This is also a regular comment with @relation(ALSO-SHOULD-NOT-APPEAR) */
fn test_function() {{
    // Another regular comment @relation(IGNORE-ME)
    println!("Test");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations =
        strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");

    // Should find no relations since they're in regular comments, not doc comments
    assert_eq!(relations.len(), 0);
}

#[test]
fn test_relation_whitespace_insensitivity() {
    // Test that whitespace around the main relation identifier is not significant
    // '@relation(REQ-002, status="draft")' and '@relation( REQ-002   , status="draft")'
    // should produce identical results

    // First case: no extra whitespace
    let mut temp_file1 = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file1,
        r#"
//! This is a file-level doc comment with @relation(REQ-002, status="draft")

fn main() {{
    println!("Hello, world!");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations1 =
        strictdoc_rs::sdoc::find_relations(&temp_file1).expect("Failed to find relations");

    // Second case: extra whitespace around relation identifier
    let mut temp_file2 = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file2,
        r#"
//! This is a file-level doc comment with @relation( REQ-002   , status="draft")

fn main() {{
    println!("Hello, world!");
}}
"#
    )
    .expect("Failed to write to temporary file");

    let relations2 =
        strictdoc_rs::sdoc::find_relations(&temp_file2).expect("Failed to find relations");

    // Both should produce identical results
    assert_eq!(relations1.len(), 1);
    assert_eq!(relations2.len(), 1);
    
    let relation1 = &relations1[0];
    let relation2 = &relations2[0];
    
    // Relation identifiers should be identical
    assert_eq!(relation1.relation, "REQ-002");
    assert_eq!(relation2.relation, "REQ-002");
    assert_eq!(relation1.relation, relation2.relation);
    
    // Attributes should be identical
    assert_eq!(relation1.attrs.len(), 1);
    assert_eq!(relation2.attrs.len(), 1);
    assert_eq!(relation1.attrs.get("status"), Some(&"draft".to_string()));
    assert_eq!(relation2.attrs.get("status"), Some(&"draft".to_string()));
    assert_eq!(relation1.attrs, relation2.attrs);
}

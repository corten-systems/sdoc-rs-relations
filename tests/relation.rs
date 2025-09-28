use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_relation_in_file_level_doc_comment_zero_attributes() {
    let test_file = create_test_file(
        "file_level_zero_attrs.rs",
        r#"
//! This is a file-level doc comment with @relation(REQ-001)

fn main() {
    println!("Hello, world!");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "REQ-001");
    assert!(relation.attrs.is_empty());
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_file_level_doc_comment_one_attribute() {
    let test_file = create_test_file(
        "file_level_one_attr.rs",
        r#"
//! This is a file-level doc comment with @relation(REQ-002, status="draft")

fn main() {
    println!("Hello, world!");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "REQ-002");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("status"), Some(&"draft".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_file_level_doc_comment_two_attributes() {
    let test_file = create_test_file(
        "file_level_two_attrs.rs",
        r#"
//! This is a file-level doc comment with @relation(REQ-003, status="approved", priority="high")

fn main() {
    println!("Hello, world!");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "REQ-003");
    assert_eq!(relation.attrs.len(), 2);
    assert_eq!(relation.attrs.get("status"), Some(&"approved".to_string()));
    assert_eq!(relation.attrs.get("priority"), Some(&"high".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_function_doc_comment_zero_attributes() {
    let test_file = create_test_file(
        "function_zero_attrs.rs",
        r#"
/// This function has @relation(FUNC-001)
fn test_function() {
    println!("Test");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "FUNC-001");
    assert!(relation.attrs.is_empty());
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_function_doc_comment_one_attribute() {
    let test_file = create_test_file(
        "function_one_attr.rs",
        r#"
/// This function has @relation(FUNC-002, complexity="low")
fn test_function() {
    println!("Test");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "FUNC-002");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("complexity"), Some(&"low".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_function_doc_comment_two_attributes() {
    let test_file = create_test_file(
        "function_two_attrs.rs",
        r#"
/// This function has @relation(FUNC-003, complexity="medium", tested="yes")
fn test_function() {
    println!("Test");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "FUNC-003");
    assert_eq!(relation.attrs.len(), 2);
    assert_eq!(relation.attrs.get("complexity"), Some(&"medium".to_string()));
    assert_eq!(relation.attrs.get("tested"), Some(&"yes".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_struct_doc_comment_zero_attributes() {
    let test_file = create_test_file(
        "struct_zero_attrs.rs",
        r#"
/// This struct represents @relation(STRUCT-001)
struct TestStruct {
    field: i32,
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "STRUCT-001");
    assert!(relation.attrs.is_empty());
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_struct_doc_comment_one_attribute() {
    let test_file = create_test_file(
        "struct_one_attr.rs",
        r#"
/// This struct represents @relation(STRUCT-002, visibility="public")
pub struct TestStruct {
    field: i32,
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "STRUCT-002");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("visibility"), Some(&"public".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_struct_doc_comment_two_attributes() {
    let test_file = create_test_file(
        "struct_two_attrs.rs",
        r#"
/// This struct represents @relation(STRUCT-003, visibility="public", serializable="true")
#[derive(Clone)]
pub struct TestStruct {
    field: i32,
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "STRUCT-003");
    assert_eq!(relation.attrs.len(), 2);
    assert_eq!(relation.attrs.get("visibility"), Some(&"public".to_string()));
    assert_eq!(relation.attrs.get("serializable"), Some(&"true".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_multiline_doc_comment() {
    let test_file = create_test_file(
        "multiline_doc.rs",
        r#"
/**
 * This is a multiline doc comment
 * that contains @relation(MULTI-001, type="requirement")
 * in the middle
 */
fn test_function() {
    println!("Test");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "MULTI-001");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("type"), Some(&"requirement".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_multiple_relations_in_same_doc_comment() {
    let test_file = create_test_file(
        "multiple_relations.rs",
        r#"
/// This comment has @relation(REL-001) and @relation(REL-002, status="active")
fn test_function() {
    println!("Test");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 2);
    
    // Find each relation by ID since order might vary
    let rel1 = relations.iter().find(|r| r.relation == "REL-001").expect("REL-001 not found");
    let rel2 = relations.iter().find(|r| r.relation == "REL-002").expect("REL-002 not found");
    
    assert!(rel1.attrs.is_empty());
    assert_eq!(rel2.attrs.len(), 1);
    assert_eq!(rel2.attrs.get("status"), Some(&"active".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_in_module_doc_comment() {
    let test_file = create_test_file(
        "module_doc.rs",
        r#"
/// This module handles @relation(MOD-001, purpose="utilities")
mod test_module {
    pub fn helper() {}
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "MOD-001");
    assert_eq!(relation.attrs.len(), 1);
    assert_eq!(relation.attrs.get("purpose"), Some(&"utilities".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_relation_with_special_characters_in_attributes() {
    let test_file = create_test_file(
        "special_chars.rs",
        r#"
/// Function with @relation(SPEC-001, description="handles input/output operations", version="1.0")
fn io_function() {
    println!("I/O operations");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    assert_eq!(relations.len(), 1);
    let relation = &relations[0];
    assert_eq!(relation.relation, "SPEC-001");
    assert_eq!(relation.attrs.len(), 2);
    assert_eq!(relation.attrs.get("description"), Some(&"handles input/output operations".to_string()));
    assert_eq!(relation.attrs.get("version"), Some(&"1.0".to_string()));
    
    cleanup_test_file(&test_file);
}

#[test]
fn test_no_relations_in_regular_comments() {
    let test_file = create_test_file(
        "regular_comments.rs",
        r#"
// This is a regular comment with @relation(SHOULD-NOT-APPEAR)
/* This is also a regular comment with @relation(ALSO-SHOULD-NOT-APPEAR) */
fn test_function() {
    // Another regular comment @relation(IGNORE-ME)
    println!("Test");
}
"#,
    );

    let relations = strictdoc_rs::sdoc::find_relations(&test_file).expect("Failed to find relations");
    
    // Should find no relations since they're in regular comments, not doc comments
    assert_eq!(relations.len(), 0);
    
    cleanup_test_file(&test_file);
}

// Helper function to create temporary test files
fn create_test_file(filename: &str, content: &str) -> PathBuf {
    let test_dir = std::env::temp_dir().join("strictdoc_test");
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    
    let file_path = test_dir.join(filename);
    fs::write(&file_path, content).expect("Failed to write test file");
    
    file_path
}

// Helper function to clean up test files
fn cleanup_test_file(file_path: &PathBuf) {
    let _ = fs::remove_file(file_path);
}
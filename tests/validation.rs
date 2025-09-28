use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_tag_validation_rejects_invalid_characters() {
    // Test TAGs with spaces - should be rejected
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// Function with @relation(REQ 001, status="test")
fn test_function() {{
    println!("Test");
}}
"#
    ).expect("Failed to write to temporary file");

    let relations = strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");
    assert_eq!(relations.len(), 0, "TAG with spaces should be rejected");

    // Test TAGs with single quotes - should be rejected
    let mut temp_file2 = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file2,
        r#"
/// Function with @relation(REQ'001, status="test")
fn test_function() {{
    println!("Test");
}}
"#
    ).expect("Failed to write to temporary file");

    let relations2 = strictdoc_rs::sdoc::find_relations(&temp_file2).expect("Failed to find relations");
    assert_eq!(relations2.len(), 0, "TAG with single quotes should be rejected");

    // Test TAGs with double quotes - should be rejected
    let mut temp_file3 = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file3,
        r#"
/// Function with @relation(REQ"001, status="test")
fn test_function() {{
    println!("Test");
}}
"#
    ).expect("Failed to write to temporary file");

    let relations3 = strictdoc_rs::sdoc::find_relations(&temp_file3).expect("Failed to find relations");
    assert_eq!(relations3.len(), 0, "TAG with double quotes should be rejected");

    // Test TAGs with commas - should be rejected
    let mut temp_file4 = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file4,
        r#"
/// Function with @relation(REQ,001, status="test")
fn test_function() {{
    println!("Test");
}}
"#
    ).expect("Failed to write to temporary file");

    let relations4 = strictdoc_rs::sdoc::find_relations(&temp_file4).expect("Failed to find relations");
    assert_eq!(relations4.len(), 0, "TAG with commas should be rejected");
}

#[test]
fn test_tag_validation_accepts_valid_characters() {
    // Test TAGs with allowed special characters - should be accepted
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// Function with @relation(REQ-001@#$%^&*_+|/., status="test")
fn test_function() {{
    println!("Test");
}}
"#
    ).expect("Failed to write to temporary file");

    let relations = strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");
    assert_eq!(relations.len(), 1, "TAG with allowed special characters should be accepted");
    assert_eq!(relations[0].relation, "REQ-001@#$%^&*_+|/.");

    // Test alphanumeric TAGs - should be accepted
    let mut temp_file2 = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file2,
        r#"
/// Function with @relation(REQ001ABC, status="test")
fn test_function() {{
    println!("Test");
}}
"#
    ).expect("Failed to write to temporary file");

    let relations2 = strictdoc_rs::sdoc::find_relations(&temp_file2).expect("Failed to find relations");
    assert_eq!(relations2.len(), 1, "Alphanumeric TAG should be accepted");
    assert_eq!(relations2[0].relation, "REQ001ABC");
}

#[test] 
fn test_tag_validation_whitespace_trimming() {
    // Test that leading/trailing whitespace is trimmed for valid TAGs
    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        temp_file,
        r#"
/// Function with @relation(  REQ-001  , status="test")
fn test_function() {{
    println!("Test");
}}
"#
    ).expect("Failed to write to temporary file");

    let relations = strictdoc_rs::sdoc::find_relations(&temp_file).expect("Failed to find relations");
    assert_eq!(relations.len(), 1, "TAG with leading/trailing whitespace should be accepted");
    assert_eq!(relations[0].relation, "REQ-001", "Whitespace should be trimmed");
}
//! This is a file-level doc comment with @relation(REQ-001)

pub fn test003() {
    println!("Hello, world!");
}


// This is a file-level doc comment with @relation(REQ-002, status=draft)
fn test011() {
    println!("Hello, world!");
}


// This is a file-level doc comment with @relation(REQ-003, status=approved, priority=high)
fn test019() {
    println!("Hello, world!");
}


/// This function has @relation(FUNC-001)
fn test_function1() {
    println!("Test");
}


/// This function has @relation(FUNC-002, complexity=low)
fn test_function2() {
    println!("Test");
}


/// This function has @relation(FUNC-003, complexity=medium, tested=true)
fn test_function3() {
    //! This is an inner doc comment with @relation(FUNC-004-INNER)
    println!("Test");
    /// This is an inner doc comment with @relation(STRUCT-005-INNER)
    struct Foo {
        /// And this field has @relation(FIELD-005-INNER) to test the relation parser
        bar: String,
    }
}

/// This struct represents @relation(STRUCT-001, type=data)
struct TestStruct {
    /// And this field has @relation(FIELD-001-INNER) to test the relation parser
    field: String,
    /// This is an inner doc comment with @relation(FIELD-002-OUTER)
    field2: String,
}

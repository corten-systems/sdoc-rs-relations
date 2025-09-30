//! This is a file-level doc comment with @relation(REQ-001)

fn test003() {
    println!("Hello, world!");
}


//! This is a file-level doc comment with @relation(REQ-002, status=draft)

fn test011() {
    println!("Hello, world!");
}


//! This is a file-level doc comment with @relation(REQ-003, status=approved, priority=high)

fn test019() {
    println!("Hello, world!");
}


/// This function has @relation(FUNC-001)
fn test_function() {
    println!("Test");
}


/// This function has @relation(FUNC-002, complexity=low)
fn test_function() {
    println!("Test");
}


/// This function has @relation(FUNC-003, complexity=medium, tested=true)
fn test_function() {
    println!("Test");
}


/// This struct represents @relation(STRUCT-001, type=data)
struct TestStruct {
    field: String,
}


struct TestStruct;

/// Implementation with @relation(IMPL-001, purpose=utility)
impl TestStruct {
    pub fn new() -> Self {
        TestStruct
    }
}


//! File level @relation(FILE-001, scope=global)

/// Function with @relation(FUNC-100, priority=high)
fn first_function() {}

/// Another function with @relation(FUNC-200, priority=low)  
fn second_function() {}


/// Function with @relation(QUOTE-001, description=handles \quoted\" strings")
fn quote_function() {
    println!("Handle quotes");
}


/// This is a multiline
/// doc comment that has
/// @relation(MULTI-001, type=multiline)
/// across multiple lines
fn multiline_function() {
    println!("Multi-line");
}


/// This module handles @relation(MOD-001, purpose=utilities)
mod test_module {
    pub fn helper() {}
}


/// Function with @relation(SPEC-001, description=handles input/output operations, version=1.0)
fn io_function() {
    println!("I/O operations");
}


// This is a regular comment with @relation(SHOULD-NOT-APPEAR)
/* This is also a regular comment with @relation(ALSO-SHOULD-NOT-APPEAR) */
fn test_function() {
    // Another regular comment @relation(IGNORE-ME)
    println!("Test");
}


//! This is a file-level doc comment with @relation(REQ-002, status=draft)

fn test105() {
    println!("Hello, world!");
}


//! This is a file-level doc comment with @relation( REQ-002   , status=draft)

fn test113() {
    println!("Hello, world!");
}


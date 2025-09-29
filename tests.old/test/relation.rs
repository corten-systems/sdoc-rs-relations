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

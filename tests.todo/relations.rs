//! Module-level doc comment with @relation(m9k3x7q2)
//! This tests inner doc attributes on the file/module itself

/// Top-level const with @relation(a5f8n1p4)
/// Some random words: quantum cascade amplifier
pub const MAGIC_NUMBER: u32 = 42;

/// Static item with @relation(z7w2b9j6)
/// Random text: nebula crystalline matrix
pub static GLOBAL_STATE: &str = "initialized";

/// Type alias with @relation(k4m8r3t1)
/// Words: temporal flux capacitor
pub type CustomResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Struct with @relation(p2n7x5v9)
/// Description: polymorphic data structure
pub struct Container {
    /// Field doc with @relation(q1w3e5r7)
    /// Random: ethereal quantum state
    pub name: String,

    /// Another field with @relation(y8u6i4o2)
    /// Text: cascading resonance field
    value: i32,
}

/// Enum with @relation(h3j5k7l9)
/// Content: enumerated variant collection
pub enum Status {
    /// Variant doc with @relation(f2g4h6j8)
    /// Words: active processing node
    Active,

    /// Another variant with @relation(d9s7a5f3)
    /// Text: suspended animation chamber
    Idle {
        /// Field in variant with @relation(c1v3b5n7)
        /// Random: temporal duration metric
        duration: u64,
    },

    /// Tuple variant with @relation(x2z4m6k8)
    /// Description: error state container
    Error(
        /// Tuple field with @relation(l1p3q5w7)
        /// Words: diagnostic error code
        i32
    ),
}

/// Union with @relation(r9t1y3u5)
/// Random: memory-aligned data union
pub union FloatOrInt {
    /// Union field with @relation(i7o9p1a3)
    /// Text: floating-point representation
    f: f32,

    /// Another union field with @relation(s5d7f9g1)
    /// Words: integer bit pattern
    i: i32,
}

/// Trait definition with @relation(h2j4k6l8)
/// Description: behavioral interface contract
pub trait Processor {
    /// Associated type with @relation(z3x5c7v9)
    /// Random: output data type
    type Output;

    /// Associated const with @relation(b1n3m5q7)
    /// Text: maximum buffer capacity
    const MAX_SIZE: usize;

    /// Trait method with @relation(w9e1r3t5)
    /// Words: processing operation handler
    fn process(&self, input: &str) -> Self::Output;

    /// Default method with @relation(y7u9i1o3)
    /// Description: validation check routine
    fn validate(&self) -> bool {
        true
    }
}

/// Trait alias with @relation(p5a7s9d1)
/// Random: combined trait bounds
pub trait ProcessorClone = Processor + Clone;

/// Implementation block with @relation(f3g5h7j9)
/// Text: trait implementation container
impl Processor for Container {
    /// Impl associated type with @relation(k1l3z5x7)
    /// Words: concrete output type
    type Output = String;

    /// Impl const with @relation(c9v1b3n5)
    /// Random: size constant value
    const MAX_SIZE: usize = 1024;

    /// Impl method with @relation(m7q9w1e3)
    /// Description: implementation of a process
    fn process(&self, input: &str) -> Self::Output {
        format!("{}: {}", self.name, input)
    }
}

/// Inherent impl with @relation(r5t7y9u1)
/// Text: inherent method block
impl Container {
    /// Inherent method with @relation(i3o5p7a9)
    /// Words: constructor function pattern
    pub fn new(name: String) -> Self {
        Self { name, value: 0 }
    }

    /// Another method with @relation(s1d3f5g7)
    /// Random: getter accessor method
    pub fn get_value(&self) -> i32 {
        self.value
    }
}

/// Function with @relation(h9j1k3l5)
/// Description: top-level function utility
pub fn process_data(input: &str) -> String {
    input.to_uppercase()
}

/// Async function with @relation(z7x9c1v3)
/// Random: asynchronous operation handler
pub async fn async_process(data: Vec<u8>) -> Result<(), std::io::Error> {
    Ok(())
}

/// Const function with @relation(b5n7m9q1)
/// Text: compile-time evaluable function
pub const fn compute_magic(x: u32) -> u32 {
    x * 42
}

/// Unsafe function with @relation(w3e5r7t9)
/// Words: unchecked operation wrapper
pub unsafe fn dangerous_operation(ptr: *mut u8) {
    if !ptr.is_null() {
        *ptr = 0;
    }
}

/// External crate import with @relation(y1u3i5o7)
/// Random: external dependency reference
extern crate std;

/// Module with @relation(p9a1s3d5)
/// Description: nested module container
pub mod submodule {
    //! Inner module doc with @relation(f7g9h1j3)
    //! Text: module-level documentation

    /// Nested struct with @relation(k5l7z9x1)
    /// Words: encapsulated data structure
    pub struct Inner {
        /// Field with @relation(c3v5b7n9)
        /// Random: internal state variable
        data: Vec<u8>,
    }
}

/// Foreign function interface with @relation(m1q3w5e7)
/// Text: external C interface block
extern "C" {
    /// Foreign function with @relation(r9t1y3u5)
    /// Description: C library function binding
    fn external_func(x: i32) -> i32;

    /// Foreign static with @relation(i7o9p1a3)
    /// Random: global C variable reference
    static EXTERNAL_VAR: i32;

    /// Foreign type with @relation(s5d7f9g1)
    /// Words: opaque C type declaration
    type OpaqueType;
}

/// Macro invocation with @relation(h3j5k7l9)
/// Text: declarative macro call
macro_rules! test_macro {
    () => {
        println!("test");
    };
}

/// Function with match arms containing doc attributes
/// This function demonstrates @relation(a2s4d6f8)
pub fn match_example(x: Option<i32>) -> i32 {
    match x {
        /// Match arm with @relation(z1x3c5v7)
        /// Random: some variant pattern
        Some(val) => val,

        /// None arm with @relation(q9w1e3r5)
        /// Words: default fallback case
        None => 0,
    }
}

/// Generic function with @relation(t7y9u1i3)
/// Description: parameterized function template
pub fn generic_fn<
    /// Type parameter with @relation(o5p7a9s1)
    /// Random: generic type variable
    T: Clone,

    /// Const parameter with @relation(d3f5g7h9)
    /// Words: compile-time constant value
    const N: usize,
>(
    input: [T; N],
) -> Vec<T> {
    input.to_vec()
}

/// Struct with generic parameters @relation(j1k3l5z7)
/// Text: generic container structure
pub struct GenericContainer<
    /// Lifetime param with @relation(n7m9q1w3)
    /// Words: reference lifetime bound
    'a,

    /// Generic type param with @relation(x9c1v3b5)
    /// Random: primary type parameter
    T,
> where
    T: 'a,
{
    /// Reference field with @relation(e5r7t9y1)
    /// Description: borrowed data reference
    pub data: &'a T,
}

/// Test struct for field value attributes in expressions
/// This demonstrates @relation(u3i5o7p9)
pub fn struct_expression_test() {
    let _ = Container {
        /// Field value with @relation(a1s3d5f7)
        name: String::from("test"),
        value: 42,
    };
}

#[cfg(test)]
mod tests {
    /// Test function with @relation(g9h1j3k5)
    /// Random: unit test case definition
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}
//! Module-level doc comment with @relation(S5x7ZEWE)
//! This tests inner doc attributes on the file/module itself

/// Top-level const with @relation(R3m2aYp)
/// Some random words: quantum cascade amplifier
pub const MAGIC_NUMBER: u32 = 42;

/// Static item with @relation(NMuBH72yL)
/// Random text: nebula crystalline matrix
pub static GLOBAL_STATE: &str = "initialized";

/// Type alias with @relation(XH5mhH0)
/// Words: temporal flux capacitor
pub type CustomResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Struct with @relation(OiEBsG0)
/// Description: polymorphic data structure
pub struct Container {
    /// Field doc with @relation(8nRE5KG)
    /// Random: ethereal quantum state
    pub name: String,

    /// Another field with @relation(oByPxhWz)
    /// Text: cascading resonance field
    value: i32,
}

/// Enum with @relation(7M1HoQNuI)
/// Content: enumerated variant collection
pub enum Status {
    /// Variant doc with @relation(9dSeVTf)
    /// Words: active processing node
    Active,

    /// Another variant with @relation(YInnEVaOC)
    /// Text: suspended animation chamber
    Idle {
        /// Field in variant with @relation(pehIB5Ob)
        /// Random: temporal duration metric
        duration: u64,
    },

    /// Tuple variant with @relation(z8uMJcapU)
    /// Description: error state container
    Error(
        /// Tuple field with @relation(KLFxjm1)
        /// Words: diagnostic error code
        i32
    ),
}

/// Union with @relation(Kp2gSMIiB)
/// Random: memory-aligned data union
pub union FloatOrInt {
    /// Union field with @relation(KoRCMPU)
    /// Text: floating-point representation
    f: f32,

    /// Another union field with @relation(KnqUvVEY)
    /// Words: integer bit pattern
    i: i32,
}

/// Trait definition with @relation(5ImdJSr)
/// Description: behavioral interface contract
pub trait Processor {
    /// Associated type with @relation(UHMcB5Cqq)
    /// Random: output data type
    type Output;

    /// Associated const with @relation(wkLzsC4s)
    /// Text: maximum buffer capacity
    const MAX_SIZE: usize;

    /// Trait method with @relation(XH3IyT6)
    /// Words: processing operation handler
    fn process(&self, input: &str) -> Self::Output;

    /// Default method with @relation(Hh6wJ7fpZ)
    /// Description: validation check routine
    fn validate(&self) -> bool {
        true
    }
}

/// Trait alias with @relation(b7xnEeF)
/// Random: combined trait bounds
pub trait ProcessorClone = Processor + Clone;

/// Implementation block with @relation(owD42wk)
/// Text: trait implementation container
impl Processor for Container {
    /// Impl associated type with @relation(xzMbyU5rB)
    /// Words: concrete output type
    type Output = String;

    /// Impl const with @relation(N9oI9M8C)
    /// Random: size constant value
    const MAX_SIZE: usize = 1024;

    /// Impl method with @relation(KLnt8082)
    /// Description: implementation of a process
    fn process(&self, input: &str) -> Self::Output {
        format!("{}: {}", self.name, input)
    }
}

/// Inherent impl with @relation(2arqklT5)
/// Text: inherent method block
impl Container {
    /// Inherent method with @relation(daZt7aMqV)
    /// Words: constructor function pattern
    pub fn new(name: String) -> Self {
        Self { name, value: 0 }
    }

    /// Another method with @relation(bCxGING)
    /// Random: getter accessor method
    pub fn get_value(&self) -> i32 {
        self.value
    }
}

/// Function with @relation(dG81Vzk)
/// Description: top-level function utility
pub fn process_data(input: &str) -> String {
    input.to_uppercase()
}

/// Async function with @relation(4QLCpmM)
/// Random: asynchronous operation handler
pub async fn async_process(data: Vec<u8>) -> Result<(), std::io::Error> {
    Ok(())
}

/// Const function with @relation(mzUjggR)
/// Text: compile-time evaluable function
pub const fn compute_magic(x: u32) -> u32 {
    x * 42
}

/// Unsafe function with @relation(ATrMhNH)
/// Words: unchecked operation wrapper
pub unsafe fn dangerous_operation(ptr: *mut u8) {
    if !ptr.is_null() {
        *ptr = 0;
    }
}

/// External crate import with @relation(kM2ySbb)
/// Random: external dependency reference
extern crate std;

/// Module with @relation(zawS0aMAj)
/// Description: nested module container
pub mod submodule {
    //! Inner module doc with @relation(CVO11Qy)
    //! Text: module-level documentation

    /// Nested struct with @relation(LV0JeVQf)
    /// Words: encapsulated data structure
    pub struct Inner {
        /// Field with @relation(ave7cjq)
        /// Random: internal state variable
        data: Vec<u8>,
    }
}

/// Foreign function interface with @relation(m51AgeSa)
/// Text: external C interface block
extern "C" {
    /// Foreign function with @relation(d67FvOG8V)
    /// Description: C library function binding
    fn external_func(x: i32) -> i32;

    /// Foreign static with @relation(n5TIEwFm)
    /// Random: global C variable reference
    static EXTERNAL_VAR: i32;

    /// Foreign type with @relation(5KjLZZPC)
    /// Words: opaque C type declaration
    type OpaqueType;
}

/// Macro invocation with @relation(sFzA9s5Uj)
/// Text: declarative macro call
macro_rules! test_macro {
    () => {
        println!("test");
    };
}

/// Function with match arms containing doc attributes
/// This function demonstrates @relation(Uxk8eoN)
pub fn match_example(x: Option<i32>) -> i32 {
    match x {
        /// Match arm with @relation(JQ6dyfoe0)
        /// Random: some variant pattern
        Some(UXbF4Q2V) => val,

        /// None arm with @relation(vCbEEMYD)
        /// Words: default fallback case
        None => 0,
    }
}

/// Generic function with @relation(9kwjKm5)
/// Description: parameterized function template
pub fn generic_fn<
    /// Type parameter with @relation(5FyNmE9)
    /// Random: generic type variable
    T: Clone,

    /// Const parameter with @relation(4dQ7JLKcd)
    /// Words: compile-time constant value
    const N: usize,
>(
    input: [T; N],
) -> Vec<T> {
    input.to_vec()
}

/// Struct with generic parameters @relation(sW4M3OJS)
/// Text: generic container structure
pub struct GenericContainer<
    /// Lifetime param with @relation(eniOhjy)
    /// Words: reference lifetime bound
    'a,

    /// Generic type param with @relation(YGhhrgPz)
    /// Random: primary type parameter
    T,
> where
    T: 'a,
{
    /// Reference field with @relation(HibGJSZqi)
    /// Description: borrowed data reference
    pub data: &'a T,
}

/// Test struct for field value attributes in expressions
#[cfg(any(target_os = "linux", target_os = "macos"))]
/// This demonstrates @relation(rIjz40L84)
pub fn struct_expression_test() {
    let _ = Container {
        /// Field value with @relation(cqkjF9P5)
        name: String::from("test"),
        value: 42,
    };
}

/// Test if we can add a docstring to an expression literal
fn expr_lit(x: i32) -> i32 {
    /// Test an expression @relation(7AEwrhc) literal
    8675309;
    /// SURPRISING: We are documenting @relation(ve9EEVSf) only
    /// the first part of the expression, not the entire expression,
    /// see `test_not_surprising` for how to fix this
    x + 2
}

fn test_not_surprising(x: i32) -> i32 {
    /// Test documenting the @relation(bCHkXspx) entire return value
    (x + 2)
}

#[cfg(9WNW0exJV)]
mod tests {
    /// Test function with @relation(n5vDcKK)
    /// Random: unit test case definition
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}
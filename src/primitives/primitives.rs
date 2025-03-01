// Rust provides a way to access a wide variety of primitives. A sample includes:

// Scalar Types
// signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// unsigned integers: u8, u16, u32, u64, u128, usize (pointer size)
// floating point: f32, f64
// char: unicode containing scalar values like 'a', 'α', etc (4 bytes each)
// bool: either `true` or `false`
// and the unit type `()`, whose only purpose value is an empty tuple: ()
// Despite the unit type being a tuple, its not considered a compount type
// because it does not contain multiple values.


// Compound Types
// arrays like [1,2,3]
// tuples like (1, true)


// Variables can always be annotated via suffix or by default. Integers
// default to i32 and floats default to f64. Note that rust can also
// infer type from context.

#![allow(unused)]
fn main() {
    // Variable types can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let another_float = 1.0f64; // Suffix annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used
    let default_float = 3.0; // f64
    let default_integer = 5; // i32

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 321362178461278i64;

    // A mutable variable's value can be changed
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
}



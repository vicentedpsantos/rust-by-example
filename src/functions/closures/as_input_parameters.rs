// While rust chooses how to capture variables on the fly without type annotation,
// this ambiguity is not allowed when writing functions. When taking a closure as
// an input parameter, the closure's complete type must be annotated using one of
// a few traits. In order of decreasing restriction, they are:
//
// - `Fn`: the closure captures by reference (`&T`)
// - `FnMut`: the closure captures by mutable reference (`&mut T`)
// - `FnOnce`: the closure captures by value (`T`)

// On a variable-by-variable basis, the compiler will capture variables in the least
// restrictive manner possible
// For instance, consider a parameter annotated as FnOnce. This specifies that the closure
// may capture by &T, &mut T, or T, but the compiler will ultimately choose based on how the
// captured variables are used in the closure.
//
// This is because if a move is possible, then any type of borrow is possible. Note that
// the reverse is not true. If the parameter is annotated as Fn, then capturing variables
// by &mut T or T are not allowed.

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returs nothing
    F: FnOnce() {
        f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
        f(3)
}

fn apply_to_4<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(4)
}

fn main() {
    use std::mem;

    let greetings = "Hello";

    let mut farewell = "goodbye".to_owned();

    // Captures two variables: `greetings` by reference
    // and `farewell` by value.
    let diary = || {
        // `greetings` is by reference, requires `Fn`
        println!("I said {}.", greetings);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`
        farewell.push_str("!!");
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`
        mem::drop(farewell);
    };

    // Call the function which applies the closure
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    let triple = |x| 3 * x;

    println!("4 tripled: {}", apply_to_4(triple));
}

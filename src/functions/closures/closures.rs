// Closures in Rust are like Ruby blocks.
// The syntax and capabilities of closures make them convenient
// for on the fly usage. Calling a closure is exactly like calling
// a function. However, both input and return types can be inferred
// and input variable names must be specified.

fn main() {
    fn function (i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them
    // to references. Annotation is identical to function annotation
    // but is optional as are the `{}` wrapping the body. These nameless
    // functions are assigned to appropriately named variables.

    let closure_annotated = |i: i32| -> i32 { i + 1};
    let closure_inferred = |i| i + 1;

    let i = 1;

    // Call the function and closures;

    println!("function: {}", function(i));
    println!("Closure annotated: {}", closure_annotated(i));
    println!("Closure inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`
    // The return type is inferred

    let one = || 1;
    println!("Closure returning one: {}", one());
}

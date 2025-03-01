// Rust provides safety via static typing. Variable bindings
// can be annotated when declared. However, in most cases,
// the compiler will be able to infer the type of the variable
// from the context, heavily reducing annotation burden.

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings
    // can be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;
}

// Variable bindings have a scope, and are constrained to live in
// a block. A block is a collection of statements enclosed by
// braces `{}`

fn main() {
    let long_lived_binding = 1;

    // This is a block and has a smaller scope than the main
    // function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }

    // End of the block
    // `short_lived_binding` does not exist in this scope

    println!("outer long: {}", long_lived_binding);
}

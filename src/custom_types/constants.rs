// Rust has two different types of constants which can be declared in any scope
// including global. Both require explicit type annotation:
// - `const`: An unchangeable value (the common case)
// - `static`: A possibly `mut`able variable with 'static lifetime. The static
//             lifetime is inferred and does not have to be specified. Accessing
//             or modifying a mutable static variable is `unsafe`

// Globals are declared outside all other scopes.

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

// This fails because a String is a growable buffer, its purpose
// is to be modified. &str is the constnat representation of a string
const TARGET_PORT_KEY: String = String::from("Vicente");

fn is_big(n: i32) -> bool { 
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" } );

    println!("Target port key is: {}", TARGET_PORT_KEY);
}

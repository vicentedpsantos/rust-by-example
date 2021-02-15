// $ rustc --crate-type=lib rary.rs
// $ ls lib*
// library.rlib
//
// Libraries get prefixed with "lib", and by default they get named their crate file,
// but this default can be overridden by passing the --crate-name option to rustc
// or by using the crate_name attribute
pub fn public_function() {
    println!("Called rary's `public_function()`");
}

fn private_function() {
    println!("Called rary's `private_function()`");
}

pub fn indirect_access() {
    println!("Called rary's `indirect_access()`, that \n> ");

    private_function();
}

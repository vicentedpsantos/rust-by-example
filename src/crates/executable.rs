// extern crate rary;
//
// To link a crate to this new library you may use `rustc`'s --extern flag. All of its
// items will then be imported under a module named the same as the library. This module
// generally behaves the same ways as any other module

fn main() {
    rary::public_function();

    rary::indirect_access();
}

// Where library.rlib is the path to the compiled library, assumed that it's
// in the same directory here:
// $ rustc executable.rs --extern rary=library.rlib --edition=2018
// $ ./executable

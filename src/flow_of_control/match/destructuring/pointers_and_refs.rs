// For pointers, a distinction needs to be made between destructuring and dereferencing as they are
// different concepts which are used differently from a language like C.
//
// Dereferencing uses *
// Destructuring uses &, ref, and ref mut

fn main() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via destructuring: {:?}", val),
    }

    // explicitly create reference from left side of expression
    let ref _is_reference = 3;

    // Accordingly, by defining 2 values without references, references can be
    // retrieved via `ref` and `ref mut`
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference
    match value {
        ref r => println!("Created a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it
            *m += 10;

            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

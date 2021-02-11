// Since closures may be used as arguments, you might wonder if the same can be said about
// functions. And indeed they can! If you declare a function that takes a closure as parameter,
// then any function that satisfies the trait bround of that closure can be passed as parameter.

// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it

fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I am a function");
}

fn main() {
    // Define closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure");
    call_me(closure);
    call_me(function);
}

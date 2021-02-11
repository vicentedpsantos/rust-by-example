// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`

fn apply<F>(f:F) where F: Fn() {
    f();
}

fn main() {
    let x = 7;

    let print = || println!("{}", x);

    apply(print);
}

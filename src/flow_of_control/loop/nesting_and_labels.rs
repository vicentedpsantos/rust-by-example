// It's possible to break or continue outer loops when dealing with nested loops. In these cases,
// the loops must be annotated with some 'label, and the label must be passed to the break/continue
// statement.

#![allow(unreachable_code)]

fn main() {
    'de_fora: loop {
        println!("Entered the outer loop");

        'inner: loop {
            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'de_fora;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

// The derive attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("{:?} months in a year.", 12);
    println!("{1:?}, {0:?} is the same {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)))
}


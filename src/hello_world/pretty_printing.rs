// fmt::Debug sacrifices elegance but makes things printable. Rust also
// provides pretty printing with {:#?}
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Vicente";
    let age = 30;
    let vicente = Person { name, age };

    // Pretty print
    println!("{:#?}", vicente)
}

// One can manually implement fmt::Display to control the display

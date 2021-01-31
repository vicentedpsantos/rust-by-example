use std::fmt;

fn main() {
    // In general {} will be replaced automatically with any arguments.
    // These will be stringified.
    println!("{} days", 31);

    // In rust if types are not specified, the language will guess its type.
    // e.g. 31 will become an i32 if no type is given.
    // The number 31i64 has a type of i64

    // You can also use positional arguments for the println! macro
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // These arguments can also be named
    println!("{sujeito} {verbo} {adjetivo}",
             sujeito="Guilherme",
             verbo="eh",
             adjetivo="putao");

    // Special formatting can be specified after a `:`
    // :b specifies base for  the integer
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    //You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1"
    println!("{number:>width$}", number=1, width=6);

    struct Circle {
        radius: i32
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };

    println!("{}", circle.to_string());

    struct Structure {
        height: i32
    }

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.height)
        }
    }

    println!("This struct `{}` won't print...", Structure { height: 42 });

    let pi = 3.141592;

    println!("{:.3}", pi)
}

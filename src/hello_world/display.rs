// Import std::fmt module to make display available
use std::fmt;

// Define a structure for which fmt::Display will be implemented
struct Structure(i32, i32);

impl fmt::Display for Structure {
    // This trait requires fmt with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.1)
    }
}

fn main() {
    let s = Structure(42, 100);

    println!("{}", s.to_string());
}

// fmt::Display is cleaner than fmt::Debug, but this presents a problem for the std
// library. How should ambiguous types be displayed? for example, if the std lib
// implemented a single style for vec<T>, what style should that be? Would it be either
// of these?
// Vec<path>: /:/etc:/home/username:/bin (split on :)
// Vec<number>: 1,2,3 (split on ,)
// No, since there is no ideal style for all types and the std lib doesn't presume
// to dictate one. fmt::Debug is to be used for generic cases
// For any container type which is not generic, fmt::Display can be implemented.

use std::fmt;

// A structure holding two numbers. Debug will be derived for comparison purposes
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure with named fields for comparison
struct Point2D {
    x: f64,
    y: f64
}

// Similarly implement `Display` for `Point2D`
impl fmt::Display for Point2D {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
   }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

fn main() {
    let vicentes_min_max = MinMax(80, 100);
    println!("Vicente's result");
    println!("{}", vicentes_min_max);

    let tamires_min_max = Point2D { x: 90.0, y: 100.0 };
    println!("Tamires' result");
    println!("{}", tamires_min_max);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small range is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.33, y: 7.22 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", Complex { real: point.x, imag: point.y });

}

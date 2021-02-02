// There are three types of structures("structs") that can be
// created using the `struct` keyword
// - Tuple structs, which are basically named tuples
// - The classic `C structs`
// - Unit structs, which are field-less, are used for generics

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

fn square(point: Point, f: f32) -> Rectangle {
    Rectangle {
	top_left: Point {
	    x: point.x,
	    y: point.y * 2.0
	},
	bottom_right: Point {
	    x: f,
	    y: f
	}
    }
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn height(&self) -> f32 {
    	self.top_left.y - self.bottom_right.y
    }

    fn width(&self) -> f32 {
    	self.bottom_right.x - self.top_left.x
    }

    fn area(&self) -> f32 {
    	self.height() * self.width()
    }
}

fn rect_area(rect: Rectangle) -> f32 {
    rect.area()
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Pete");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a Point
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make new point by using a struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    println!("Point returns square: {:?}", square(point, 3.2));

    let rectangle = Rectangle {
    	// struct instantiation is an expression too
	top_left: Point { x: left_edge, y: top_edge },
	bottom_right: bottom_right
    };

    println!("Rectangle area: {}", rect_area(rectangle));

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

}

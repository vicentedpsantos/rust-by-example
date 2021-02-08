// Functions are declared using the fn keyword. Its arguments are
// type annotated, just like variables, and, if the function returns
// a value, the return type must be specified after an arrow ->.

// The final expression in the function will be used as return value.
// Alternatively, the return statement can be used to return a value
// earlier from within the function, even from inside loops or if
// statements.

// Unlike C/C++, theres no restriction on the order of function
// definitions

fn main() {
    fizzbuzz_to(1000);
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
	return false;
    }

    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit
// type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
    	println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
	println!("fizz");
    } else if is_divisible_by(n, 5) {
	println!("buzz");
    } else {
	println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
    	fizzbuzz(n);
    }
}

// If you use a type lias, you can refer to each enum variant
// via its alias. This might be useful if the enum's name is too long
// or too generic and you want to rename it.

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias,
    // not its long and inconvenient name
    let x = Operations::Add;
}

// The most common place for these are in impl blocks
// using the Self alias
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y
        }
    }
}

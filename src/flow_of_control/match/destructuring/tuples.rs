// Tuples can be destructured ni a match as follows:

fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?} and `z` is {:?}", y, z),
        (1, ..)   => println!("First is 1 and the rest doesn't matter"),
        _         => println!("It doesn't matter what they are"),
    }
}

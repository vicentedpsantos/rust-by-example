// Rust provides HOF. These are functions that take one or more functions
// and/or produce a more useful function. HOFs and lazy iterators give Rust its
// functional flavor

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared off numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number;
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("Imperative style: {}", acc);

    // functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
             .take_while(|&n_squared| n_squared < upper)
             .filter(|&n_squared| is_odd(n_squared))
             .fold(0, |acc, n_squared| acc + n_squared);

    println!("Functional style: {}", sum_of_squared_odd_numbers);
}

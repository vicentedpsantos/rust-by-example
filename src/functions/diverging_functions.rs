// Diverging functions never return. They are marked using `!`, which is an empty type

fn foo() -> ! {
    panic!("This call never returns");
}

// As opposed to all the other types, this one cannot be instantiated, because the set of all
// possible values this type can have is empty. Note that, it is different from the `()` type,
// which has exactly one possible value.
//
// For example, this function returns as usual, although there is no information in the return
// value
fn some_fn() {
    ()
}

fn main() {
    let a: () = some_fn();

    println!("This function returns and you can see this line");

    // As opposed to this function, which never returns the control back to the caller
    let x: ! = panic!("This call never returns!");
    println!("You will never see this line!");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;

        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue
            };

            acc += addition;
        }

        acc
    }

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}

// Closures are inherently flexible and will do what the functionality
// requires to make the closure work without annotation. This allows
// capturing to flexibly adapt to the use case, sometimes moving and sometimes
// borrowing. Closures can capture variables:
//
// - By reference: &T
// - By mutable reference: &mut T
// - By value: T

// They preferentially capture variables by reference and only go lower when
// required

fn main() {
    use std::mem;

    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    // Call the closure to see the borrow
    print();

    // `color` can be borrowed again because print only holds an
    // immutable reference to `color`
    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could either take a `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately borrows `count`
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus, calling
    // the closure mutates the closure which requres a `mut`

    let mut inc = || {
        count += 1;
        println!("`count`:{}", count);
    };

    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    inc();

    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // This consumes the variable, so it can only be called once.
    consume();

    // Using `move` before vertical pipes forces closure to take
    // ownership of captured variables
    
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
}

pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
pub fn public_function() {
    println!("Called `my::inaccessible::public_function()`");
}

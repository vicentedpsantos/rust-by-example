fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // This errors! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
    }

    // Ok! `_mutable_integer` is not frozen in the outer scope
    _mutable_integer = 3;
}

// The same set of rules can be applied to functions: a type `T` becomes generic
// when preceded by `<T>`.
//
// Using generic functions sometimes requires explicitly specifying type paremeters.
// This may be the case if the function is called where the return type is generic,
// or if the compiler doesn't have enough information to infer the necessary
// type parameters.
//
// A function call with explicitly specified type parameters looks like fun::<A, B, ...>().
struct A;
struct S(A);
struct SGEN<T>(T);

// The following functions all take ownership of the variable passed into them and
// immediately go out of scope, freeing the variable

// Define a function `reg_fn` that takes an argument `_s` of type `S`.
// This has no `<T>` so this is not a generic function
fn reg_fn(_s: S) {}

// Define function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
// It has been explicitly given the type parameter `A`, but because `A` has not
// been specified as a generic type parameter for `gen_spec_t`, it is not generic.
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A)); // Concrete type
    gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`

    generic::<char>(SGen('a'));

    generic(SGen('c'));
}

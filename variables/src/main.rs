/*
fn main() {
    let x = 5;
    println!("The value of x is {x}");
    x = 123123123;
    println!("The new value of x is {x}");
}

    -------------------------------------------------------------------------
    CONSOLE OUTPUT:

    error[E0384]: cannot assign twice to immutable variable `x`
    --> src\main.rs:4:5
    |
    2 |     let x = 5;
    |         -
    |         |
    |         first assignment to `x`
    |         help: consider making this binding mutable: `mut x`
    3 |     println!("The value of x is {x}");
    4 |     x = 123123123;
    |     ^^^^^^^^^^^^^ cannot assign twice to immutable variable

    ------------------------------------------------------------------------

    By default variables are immutable in rust. It allows more safety and easy concurrency.

*/


fn main() {
    let mut x = 5; // marked the variable mutable
    println!("The value of x is {x}");
    x = 123123123;
    println!("The new value of x is {x}");
}
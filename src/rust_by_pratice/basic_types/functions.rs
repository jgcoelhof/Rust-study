//Functions
// 🌟🌟🌟

use std::process;
#[allow(dead_code)]

pub fn functions() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}
#[allow(dead_code)]

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
#[allow(dead_code)]

//🌟
pub fn functions2() {
    print();
}
#[allow(dead_code)]

// Replace i32 with another type
fn print() -> String {
    String::from("Success")
}
#[allow(dead_code)]

//🌟🌟🌟
// Solve it in two ways
// DON'T let `println!` work
pub fn functions3() {
    never_return();

}
#[allow(dead_code)]

fn never_return() -> ! {
    eprintln!("This function never returns!");
    process::exit(1);
}

//Diverging functions
// Diverging functions never return to the caller, so they may be used in places where a value of any type is expected.
#[allow(dead_code)]

pub fn diverging_functions() {
    println!("Success!");
}
#[allow(dead_code)]

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    eprintln!("This function never returns!");
    process::exit(1);
   // panic!("This function never returns!");
   // loop{}
}

//🌟🌟
#[allow(dead_code)]

pub fn diverging_functions2() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
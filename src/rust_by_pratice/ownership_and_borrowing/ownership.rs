//Ownership
// 🌟🌟


pub fn ownership() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello, world!");
    let y = &x[7..];
    let z = &x[..5];
    println!("{}, {}", z, y);


    // Using chars method
    let y_chars: String = x.chars().skip(7).collect();
    let z_chars: String = x.chars().take(5).collect();
    println!("{}, {}", z_chars, y_chars);

    let y = String::from(&x[7..]);
    let z = String::from(&x[..5]);
    println!("{}, {}", z, y);
}

//🌟🌟

// Don't modify code in main!
pub fn ownership2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
// 🌟🌟


pub fn ownership3() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.clone();
    s
}

//🌟🌟

pub fn ownership4() {
    // Fix the error without removing any code
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}", s)
}
//🌟🌟

pub fn ownership5() {
    // Don't use clone ,use copy instead
    let x = (1, 2, (), "hello");
    let y = x;

    println!("{:?}, {:?}", x, y);
}
//Mutability
// Mutability can be changed when ownership is transferred.
//
// 🌟

pub fn mutability() {

    // make the necessary variable mutable
    let s = String::from("Hello");

    let mut s1 = s;

    s1.push_str(", World!");

    println!("{s1}");

    println!("Success!");
    }

//🌟🌟🌟


pub fn mutability2() {
    let x = Box::new(5);

    let mut y = Box::new(5);   // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

// Partial move
// Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at the same time. Doing this will result in a partial move of the variable, which means that parts of the variable will be moved while other parts stay. In such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used

//🌟

use std::mem;

pub fn partial_move() {
    let mut t = (String::from("hello"), String::from("world"));

    let _s = mem::replace(&mut t.0, String::new());

    // Modify this line only, don't use `_s`
    println!("{:?}", (&t.0, &t.1));
}
//🌟🌟

pub fn partial_move2() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}


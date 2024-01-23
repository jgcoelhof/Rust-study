//Binding and mutability

#[allow(dead_code)]

// Fix the error below with least amount of modification to the code
//🌟 A variable can be used only if it has been initialized.
pub fn assert() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
   // let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
//🌟 Use mut to mark a variable as mutable.

#[allow(dead_code)]

// Fill the blanks in the code to make it compile
pub fn mutt() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
//Scope

//A scope is the range within the program for which the item is valid.
//🌟
#[allow(dead_code)]


// Fix the error below with least amount of modification
pub fn scope() {
    let x: i32 = 10;

        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);

    println!("The value of x is {} and value of y is {}", x, y);
}
//🌟🌟
#[allow(dead_code)]

// Fix the error with the use of define_x
pub fn scope2() {
    let x = define_x();
    println!("{}, world", x);
}
#[allow(dead_code)]

fn define_x() -> &'static str {
    "hello"
}

//Shadowing
//You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one.

//🌟🌟

#[allow(dead_code)]

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
pub fn shadowing() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}
// 🌟🌟

#[allow(dead_code)]
// Remove a line in the code to make it compile
pub fn shadowing2() {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    let mut _x: i32 = _x;
    _x += 3;


    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}
//Unused variables
#[allow(dead_code)]

pub fn unused() {
    let x = 1;
    println!("{x}")
}

// Warning: unused variable: `x`

//Destructuring
// 🌟🌟 We can use a pattern with let to destructure a tuple to separate variables.
#[allow(dead_code)]


// Fix the error below with least amount of modification
pub fn destructuring() {
    let  (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

//Destructuring assignments
// Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.
//
// 🌟🌟
#[allow(dead_code)]

pub fn destructuring_assignments() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
}
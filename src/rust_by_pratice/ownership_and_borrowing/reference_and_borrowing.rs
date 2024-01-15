// /Reference and Borrowing
// Reference
// 🌟
pub fn reference() {
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}


//🌟

pub fn reference2() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}

//🌟


// Fix error
pub fn reference3() {
    let s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

//🌟

// Fix error
pub fn reference4() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

//🌟🌟


pub fn reference5() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let mut p = s;

    p.push_str("world");

    println!("Success!");
}

//Ref
// ref can be used to take references to a value, similar to &.
//
// 🌟🌟🌟

pub fn refref() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

//Borrowing rules
// 🌟


// Remove something to make it work
// Don't remove a whole line !
pub fn borrowing() {
    let mut s = String::from("hello");

    let r1 = s.clone();
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

//Mutability
// 🌟 Error: Borrow an immutable object as mutable


pub fn mutability() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

//🌟🌟 Ok: Borrow a mutable object as immutable

// This code has no errors!
pub fn mutability2() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");
    println!("{s}");
    println!("Success!");
}
//NLL
// 🌟🌟


// Comment one line to make it work
pub fn nll() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    // let r2 = &mut s;
    r1.push_str("!");

    println!("{}",r1);
}

pub fn nll2(){
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;
    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    let r3 = &s;
}

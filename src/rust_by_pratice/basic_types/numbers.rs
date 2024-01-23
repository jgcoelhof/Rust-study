//Integer
// 🌟

#[allow(dead_code)]

// Remove something to make it work
pub fn integer() {
    let x: u8 = 5;
    let mut _y: u8 = 5;

    _y = x;

    let _z: u8 = 10; // Type of z ?

    println!("Success!");
}

//🌟
#[allow(dead_code)]


// Fill the blank
pub fn integer2() {
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}

//🌟🌟🌟

#[allow(dead_code)]

// Modify `assert_eq!` to make it work
pub fn integer3() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}
#[allow(dead_code)]

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

//🌟🌟
#[allow(dead_code)]

// Fill the blanks to make it work
pub fn integer4() {
    assert_eq!(i8::MAX, std::i8::MAX); // Preencher com std::i8::MAX
    assert_eq!(u8::MAX, std::u8::MAX); // Preencher com std::u8::MAX

    println!("Success!");
}

//🌟🌟
#[allow(dead_code)]

pub fn integer5() {
    let v1 = 251_u16 + 8;
    let v2 = u16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}
#[allow(dead_code)]

//🌟🌟
pub fn integer6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert_eq!(v, 1597);

    println!("Success!");
}

//Floating-Point
//🌟

#[allow(dead_code)]

// Fill the blank to make it work
pub fn floating() {
    let x = 1_000.000_1; // ?
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}
#[allow(dead_code)]

//🌟🌟 Make it work in two distinct ways
pub fn floating2() {
    assert_eq!(format!("{:.10}", 0.1 + 0.2), format!("{:.10}", 0.3));

    println!("Success!");
}
#[allow(dead_code)]

//Range
// 🌟🌟 Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122
pub fn range() {
    let mut sum = 0;
    for i in -3..=2 {
        sum += i;
    }

    assert_eq!(sum, -3);
    for i in 97..=122 {

        println!("{i}")
    }
    for c in 'a'..='z' {
        println!("{}", c);
    }
}

//🌟🌟


// Fill the blanks
use std::ops::{Range, RangeInclusive};
#[allow(dead_code)]

pub fn range2() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

//Computations
//🌟
#[allow(dead_code)]

// Fill the blhttps://practice.rs/basic-types/numbers.html#computationsanks and fix the errors
pub fn computations() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);
    assert_eq!(f64::round(9.6 / 3.2) as i32, 3);

    assert!(25 % 5 == 0);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
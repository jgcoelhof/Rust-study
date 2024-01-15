//Char
//🌟


// Make it work
use std::mem::size_of_val;
pub fn char() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}

//🌟

// Make it work
pub fn char2() {
    let c1 = String::from("中");
    print_char(c1);
}

fn print_char(c : String) {
    println!("{}", c);
}


//Bool
//🌟

// Make println! work
pub fn bool() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}


//🌟
// Make it work
pub fn bool2() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

//Unit type
// 🌟🌟


// Make it work, don't modify `implicitly_ret_unit` !
pub fn unit_type() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit_wrapper());

    println!("Success!");
}

fn implicitly_ret_unit_wrapper() -> (i32, i32) {
    implicitly_ret_unit();
    (2, 3)
}

fn implicitly_ret_unit() {
}


//🌟🌟 What's the size of the unit type?


// Modify `4` in assert to make it work
pub fn unit_type2() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

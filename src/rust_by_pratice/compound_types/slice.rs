//Slice
//Slices are similar to arrays,
//but their length is not known at compile time,
//so you can't use slice directly.
#[allow(dead_code)]

//🌟🌟 Here, both [i32] and str are slice types,
//but directly using it will cause errors.
//You have to use the reference of the slice instead: &[i32], &str.
pub fn slice() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];

    let _s2: &str = "hello, world";

    println!("Success!");
}

//A slice reference is a two-word object, for simplicity reasons,
//from now on we will use slice instead of slice reference.
//The first word is a pointer to the data,
//and the second word is the length of the slice.
// The word size is the same as usize, determined by the processor architecture, e.g.
//64 bits on an x86-64.
//Slices can be used to borrow a section of an array,
//and have the type signature &[T].
#[allow(dead_code)]

//🌟🌟🌟
pub fn slice1() {
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}
#[allow(dead_code)]

//🌟🌟
pub fn slice2() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(&slice, &[2, 3, 4]);
    println!("Success!")
}

//String slices
//🌟
#[allow(dead_code)]

pub fn string_slices() {
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);

    println!("Success!");
}
#[allow(dead_code)]

//🌟
pub fn string_slices1() {
    let s: &str = "你好，世界";
    let slice: &str = &s[0..3];
    assert!(slice == "你");

    println!("Success!");
}
#[allow(dead_code)]

//🌟🌟 &String can be implicitly converted into &str.
pub fn string_slices2() {
    let mut s = String::from("Hello, world!");
    let letter = first_letter(&s);
    println!("the first letter is: {}", letter);

    s.clear();
}
#[allow(dead_code)]

fn first_letter(s: &str) -> &str {
    &s[..1]
}

//Array
//The type of array is [T; Length],
//as you can see, array's length is part of their type signature.
//So their length must be known at compile time.

//For example, you cant initialize an array like below:
// pub fn array1(n:i32){
//     let arr=[1;n];
// }
// This will cause an error,
//because the compiler has no idea of the exact size of the array at compile time.

// 🌟
#[allow(dead_code)]

pub fn array() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    assert!(arr.len() == 5);
    print!("success")
}
#[allow(dead_code)]

//🌟🌟
pub fn array1() {
    let _arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}
//🌟 All elements in an array can be initialized to the same value at once.
#[allow(dead_code)]

pub fn array2() {
    let list: [i32; 100] = [1; 100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);
    println!("success")
}
//🌟 All elements in an array must be of the same type
#[allow(dead_code)]

pub fn array3() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("Success!");
}
#[allow(dead_code)]

//🌟 Indexing starts at 0.
pub fn array4() {
    let arr = ['a', 'b', 'c'];
    let ele = arr[0];
    assert!(ele == 'a');
    println!("Success!")
}
// /🌟 Out of bounds indexing causes panic.
#[allow(dead_code)]

// Fix the error
pub fn array5() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let _name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1];

    println!("Success!");
}


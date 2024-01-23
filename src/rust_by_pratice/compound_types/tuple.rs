//Tuple
//🌟 Elements in a tuple can have different types. 
//Tuple's type signature is (T1, T2, ...),
// where T1, T2 are the types of tuple's members.   
#[allow(dead_code)]
pub fn tuple(){
    let _t0: (u8, i16) = (0, -1);

    let _t1: (u8,(i16, u32)) = (0, (-1, 1));

    let _t: (u8, u16, i64, &str, String) = (1u8,2u16,3i64,"hello",String::from(", world"));
    println!("Success!")
}
#[allow(dead_code)]
pub fn tuple1(){
    //🌟 Members can be extracted from the tuple using indexing.
    let t = ("i","am","sunface");
    assert_eq!(t.2, "sunface");
    println!("Success!")
}
#[allow(dead_code)]
pub fn tuple2(){
    //🌟 Long tuples cannot be printed
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple)
}

#[allow(dead_code)]

pub fn tuple3(){
    let tup = (1, 6.4, "hello");
    let (x, z, y)= tup;
    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!"); 
}
#[allow(dead_code)]

//🌟🌟 Destructure assignments.
pub fn tuple4(){
    let (x, y, z);
    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}
#[allow(dead_code)]

//🌟🌟 Tuples can be used as function arguments and return values
pub fn tuple5(){
    let (x, y) = sum_multiply((5, 6));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
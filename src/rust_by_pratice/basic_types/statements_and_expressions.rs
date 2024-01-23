//🌟🌟
// Make it work with two ways
#[allow(dead_code)]

pub fn statements_and_expressions() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

//🌟



#[allow(dead_code)]

pub fn statements_and_expressions2() {
    let v = 3;
    let x = 3;

    assert_eq!(v, x);

    println!("Success!");
}

//🌟
#[allow(dead_code)]


pub fn statements_and_expressions3() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}
#[allow(dead_code)]

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
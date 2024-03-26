//Enum
// 🌟🌟 Enums can be created with explicit discriminator.

// Fix the errors
enum Number {
    Zero=0 ,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


pub fn enumm() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    println!("Success!");
}

//🌟 Each enum variant can hold its own data.


// Fill in the blank
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn enum2() {
    let msg1 = Message::Move{x :1 , y : 2 }; // Instantiating with x = 1, y = 2
    let msg2 = Message::Write(String::from("hello, world!")); // Instantiating with "hello, world!"

    println!("Success!");
}
//🌟🌟 We can get the data which an enum variant is holding by pattern match.


// Fill in the blank and fix the error
enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn enum3() {
    let msg = Message1::Move{x: 1, y: 2};

    if let Message1::Move{x,y} = msg {
        assert_eq!(x, 1);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}

//🌟🌟

// Fill in the blank and fix the errors
#[derive(Debug)]
enum Message4 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
pub fn enum4() {
    let msgs:[Message4;3]= [
        Message4::Quit,
        Message4::Move{x:1, y:3},
        Message4::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message4) {
    println!("{:?}", msg);
}

//🌟🌟 Since there is no null in Rust,
// we have to use enum Option<T> to deal with the cases when the value is absent.

// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.




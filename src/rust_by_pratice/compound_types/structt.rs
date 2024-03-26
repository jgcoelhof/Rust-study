//Struct
// The types of structs

// 🌟 We must specify concrete values for each of the fields in struct./
// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
pub fn structt() {
    let age = 30;
    let hobby=String::from("code");
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby
    };

    println!("Success!");
}

//🌟 Unit struct don't have any fields.
// It can be useful when you need to implement a trait on some type
// but don’t have any data that you want to store in the type itself.


struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
pub fn structt2() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(u: Unit) {   }

//🌟🌟🌟 Tuple struct looks similar to tuples, it has added meaning the struct name provides but has no named fields.
// It's useful when you want to give the whole tuple a name, but don't care about the fields's names.


// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
pub fn structt3() {
    let x = 0;
    let y = 127;
    let z = 255;
    let v = Point(x, y, z);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Point) {
    let Point(x, y, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

//Operating on structs
// 🌟 You can make a whole struct mutable when instantiating it,
// but Rust doesn't allow us to mark only certain fields as mutable.


// Fill the blank and fix the error without adding/removing new line
struct Person1 {
    name: String,
    age: u8,
}
pub fn structt4() {
    let age = 18;
    let mut p = Person1 {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

//🌟 Using field init shorthand syntax to reduce repetitions.

// Fill the blank
struct Person2 {
    name: String,
    age: u8,
}
fn main() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person2 {
    Person2 {
        age,
        name
    }
}

//🌟 You can create instance from other instance with struct update syntax


// Fill the blank to make the code work
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
pub fn struct5() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

//Print the structs
// 🌟🌟 We can use #[derive(Debug)] to make a struct printable.


// Fill the blanks to make the code work
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn print_struct() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:#?}", rect1); // Print debug info to stdout
}


//Partial move
// Within the destructuring of a single variable,
// both by-move and by-reference pattern bindings can be used at the same time.
// Doing this will result in a partial move of the variable,
// which means that parts of the variable will be moved while other parts stay.
// In such a case,
// the parent variable cannot be used afterwards as a whole,
// however the parts that are only referenced (and not moved) can still be used.
//
// Example
/*
fn main() {
    #[derive(Debug)]
    struct Person {
        n  name: Strig,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}*/

//Exercises
// 🌟🌟


// Fix errors to make it work

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

pub fn struct6() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name;

    // Modify this line to use a reference
    println!("{}, {}", _name, f.data);
}

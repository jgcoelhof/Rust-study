
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    #[allow(dead_code)]

    fn area(&self) -> u32 {
        self.width * self.height
    }
}
#[allow(dead_code)]

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    )
}

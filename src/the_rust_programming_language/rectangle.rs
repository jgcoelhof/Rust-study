pub fn rectangles() {
    let height: f32 = 30f32;
    let width: f32 = 50f32;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(height: f32, width: f32) -> f32 {
    width * height
}

pub fn rectangles1() {
    let rect = (40, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect)
    );
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangles2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
fn rectangles3() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
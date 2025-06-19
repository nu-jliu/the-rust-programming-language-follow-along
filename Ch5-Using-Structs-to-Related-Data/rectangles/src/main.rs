#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1 = 50;
    // let height1 = 30;

    // let area = area(width1, height1);
    // println!("The area of the rectangle is {area} square pixels");

    // let rect1 = (30, 50);

    // let area = area(rect1);
    // println!("The area of the rectangle is {area} square pixels");

    let rect1 = &Rectangle {
        width: 30,
        height: 50,
    };

    let area = area(rect1);
    println!("The area of the rectangle is {area} square pixels");
    println!("rect1 is: {rect1:#?}");
    dbg!(rect1);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

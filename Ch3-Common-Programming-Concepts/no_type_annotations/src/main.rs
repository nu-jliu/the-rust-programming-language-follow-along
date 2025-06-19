use std::io;

fn main() {
    // Data Types
    // let guess: u32 = "42".parse().expect("Not a number");

    // Floating-point types
    // let x = 2.0;
    // let y: f32 = 3.0;

    // Numberic Operations
    // // addition
    // let sum = 5 + 10;

    // // substraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;

    // // remainder
    // let remainder = 43 % 5;

    // The Boolean Type
    // let t = true;

    // let f: bool = false;

    // The Character Type
    // let c = 'z';
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';

    // The Tuple Type
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;

    // println!("The value of y is: {y}");

    // let tup2: (i32, f64, u8) = (500, 6.4, 1);

    // let first = tup2.0;
    // let second = tup2.1;
    // let third = tup2.2;

    // The Array Type
    // let a = [1, 2, 3, 4, 5];
    // let months = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // accesing array elements
    // let a = [1, 2, 3, 4, 5];

    // let first = a[0];
    // let second = a[1];

    // invalid array elements access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index!");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    let element = a[index];
    println!("The value at index {index} is {element}");
}

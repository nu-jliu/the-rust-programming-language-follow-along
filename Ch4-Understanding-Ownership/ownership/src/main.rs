fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");

    println!("{s}");

    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{s2}, world!"); // s1 wont work

    let mut s3 = String::from("Hello");
    s3 = String::from("ahoy");

    println!("{s3}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{s}");

    let x = 5;
    make_copy(x);
    println!("{x}");

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // println!("s1: {s1}, s2: {s2}, s3: {s3}");

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn make_copy(some_int: i32) {
    println!("{some_int}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours!!!");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("{some_string}");
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

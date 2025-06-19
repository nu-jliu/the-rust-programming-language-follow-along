fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("Hello");
    change(&mut s);

    println!("{s}");

    let mut s = String::from("hello");

    // let r1 = &mut s;
    let r2 = &mut s;

    println!("r2: {r2}");

    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("r1: {r1}, r2: {r2}");

    let r3 = &mut s;
    println!("r3: {r3}");

    // let nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

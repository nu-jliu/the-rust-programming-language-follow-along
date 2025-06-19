use std::thread;
use std::vec;

fn main() {
    println!("Hello, world!");

    let deg_f = 90.0;
    let deg_c = fahrenheit_to_celsius(deg_f);

    println!("The Celsius for {deg_f}F is {deg_c}C");

    let n = 100;
    let fib_n = fibonacci(n);

    println!("The {n}th fibonacci sequence is {fib_n}");
}

fn fahrenheit_to_celsius(deg_f: f32) -> f32 {
    (deg_f - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) -> u128 {
    if n <= 1 {
        return 1;
    }

    let mut fib_vec = vec![1; n as usize];

    for i in 2..n {
        let fib_i1 = fib_vec[(i - 1) as usize];
        let fib_i2 = fib_vec[(i - 2) as usize];

        let fib_i = fib_i1 + fib_i2;

        fib_vec[i as usize] = fib_i;
    }

    return fib_vec[(n - 1) as usize];
}

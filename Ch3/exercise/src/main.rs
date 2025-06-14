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

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    if n == 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

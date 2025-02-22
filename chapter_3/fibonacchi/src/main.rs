use std::io;

fn main() {
    println!("Enter the Fibonacci number you'd like to get: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading the line");

    let number = input
        .trim()
        .parse::<u32>()
        .expect("Error parsing user input");

    println!("5th Fibonacci number: {}", get_nth_fibonacci(number));
}

fn get_nth_fibonacci(n: u32) -> u32 {
    let mut prev: u32 = 0;
    let mut cur: u32 = 1;

    if n == 1 {
        return prev;
    } else if n == 2 {
        return cur;
    }

    for _ in 3..=n {
        let _t = cur;
        cur += prev;
        prev = _t;
    }

    cur
}

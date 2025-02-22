use std::io;

fn main() {
    let a = [1; 10];

    println!("Enter an index you would like to access:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read the value");

    let idx = input
        .trim()
        .parse::<usize>()
        .expect("Unable to parse the value");

    let value = a[idx];
    println!("The value is {value}");
}

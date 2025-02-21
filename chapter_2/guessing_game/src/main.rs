use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your number: ");

        let mut guess = String::new();

        let bytes_read = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let guess = match guess.trim().parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Your input is not a number. Try again.");
                continue;
            }
        };

        println!("You guessed: {}. Total bytes read: {bytes_read}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Equal. Win.");
                break;
            }
        }
    }
}

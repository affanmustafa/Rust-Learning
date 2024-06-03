use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Welcome to your Rust Guessing Game!");
    println!("Choose a number from 1 to 100");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Not a valid number.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Go higher!"),
            Ordering::Greater => println!("Go lower!"),
            Ordering::Equal => {
                println!("You guessed it! Congratulations");
                break;
            }
        }
    }

    println!("Secret number {secret_number}");
}

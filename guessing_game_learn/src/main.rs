use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count: u8 = 0;
    println!("Guess the number!");

    println!("Please enter your guess");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // &mut is a mutable reference. read_line appends the input to the string that's why we use a mutable reference
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                count += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                count += 1;
            }
            Ordering::Equal => {
                println!("You win!");
                count += 1;
                println!("You took {} guesses", count);
                break;
            }
        }
    }
}

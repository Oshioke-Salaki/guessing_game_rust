use rand::Rng;
use std::cmp::Ordering;
use std::io; //This imports the input/output library from the standard library

fn main() {
    println!("Welcome to the guess the number game by Oshioke Salaki!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // Used to generate a random number between 1 to 100.
    let mut total_attempts: u32 = 0;

    loop {
        if total_attempts == 0 {
            println!("Please enter your guess.");
        } else {
            println!("Try again, enter your guess.");
        }
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please enter a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Oops, that's too big!"),
            Ordering::Less => println!("Oops, that's too small!"),
            Ordering::Equal => {
                break println!("Woohoo! You win!");
            }
        }
        total_attempts += 1;
    }
}

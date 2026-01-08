use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100); // generates random number between 1 to 100

    println!("Secret Number: {}", secret_number);

    loop {
        println!("Enter your input here");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small!".red()),
            Ordering::Equal => {println!("{}","You Win!".green());break;},
            Ordering::Greater => println!("{}","Too Big!".red()),
        };
    }
}

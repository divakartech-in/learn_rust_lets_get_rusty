use std::io;
use rand::Rng;
use colored::*;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number");

    loop {
        let secret_number = rand::rng().random_range(1..101);

        println!("The secret number is: {}", secret_number);

        println!("Please input your Guess:");

        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read user input");

        let guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a valid number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You guessed it correct!".green());
                break;
            },
        };
    }
}

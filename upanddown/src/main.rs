use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

fn main() {
    println!("Guess the number between 1 and 100!");
    println!("If your guess is greater than the number, it will say 'Too big'");
    println!("If your guess is less than the number, it will say 'Too small'");
    println!("If your guess is correct, it will say 'You win!'");
    println!("You can exit the game by typing 'exit'");

    let secret_number = 42;

    loop {
        println!("Please input your guess.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "exit" {
                    println!("Goodbye!");
                    break;
                } else {
                    println!("Please type a number!");
                    continue;
                }
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

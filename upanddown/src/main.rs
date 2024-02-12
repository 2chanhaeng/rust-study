use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

enum Next {
    /// either a success or a failure
    Continue(&'static str),
    Break(&'static str),
}

fn set_secret(secret: &'static i8) -> Box<dyn Fn(i8) -> Next> {
    Box::new(|guess: i8| match guess.cmp(secret) {
        Less => Next::Continue("Too small"),
        Greater => Next::Continue("Too big"),
        _ => Next::Break("You win!"),
    })
}

fn unparsed(input: String) -> Next {
    match input.trim() {
        "exit" => Next::Break("Exiting the game"),
        "help" => Next::Continue(&help()),
        _ => Next::Continue("Please enter a valid number"),
    }
}

fn get_message(result: &Next) -> &'static str {
    match result {
        Next::Continue(msg) => msg,
        Next::Break(msg) => msg,
    }
}

fn help() -> &'static str {
    "
    Guess the number between 1 and 100!
    - [ your guess ] > [ the number ]: 'Too big'
    - [ your guess ] < [ the number ]: 'Too small'
    - [ your guess ] = [ the number ]: 'You win!'
    You can exit the game by typing 'exit'
    You can get help by typing 'help'
    "
}

fn main() {
    loop {
        println!("Please input your guess.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
            }
        };

        println!("You guessed: {}", guess);

    }
    println!("{}", help());
}

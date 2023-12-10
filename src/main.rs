mod program_guess;
mod user_guess;
use std::io::{self, Write};

fn main() {
    match prompt_user() {
        GuessVariant::Program => program_guess::run(),
        GuessVariant::User => user_guess::run(),
    }
}

enum GuessVariant {
    Program,
    User,
}

fn prompt_user() -> GuessVariant {
    print!("Who should guess [p-program, u-user]? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().to_lowercase().as_str() {
        "p" | "program" => GuessVariant::Program,
        "u" | "user" => GuessVariant::User,
        _ => {
            println!("Invalid input.");
            prompt_user()
        }
    }
}

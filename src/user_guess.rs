use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

pub fn run() {
    let range = 1..=100;
    println!("Guess the number! ({}..{})", range.start(), range.end());
    let secret_number = rand::thread_rng().gen_range(range);
    let mut question_count = 0;

    loop {
        question_count += 1;
        print!("{}) Your guess: ", question_count);
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

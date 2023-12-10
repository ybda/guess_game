use std::cmp::Ordering;
use std::io::{self, Write};

// Guess: 20
// 50? - (0 + 100) / 2
// 25? - (0 + 50) / 2
// 12? + (0 + 25) / 2
// 18? + (12 + 25) / 2
// 21? - (18 + 25) / 2
// 19? + (18 + 21) / 2
// 20? = (19 + 21) / 2

pub fn run() {
    let range = 0..=100;
    println!(
        "Pick a random number! ({}..{})\nIf your number is less, enter '-', if correct '='",
        range.start(),
        range.end()
    );

    let mut min: i32 = *range.start();
    let mut max: i32 = *range.end() + 1;
    let mut guess: i32 = (min + max) / 2;
    let mut question_count: u32 = 0;

    loop {
        question_count += 1;
        print!("{}) ", question_count);
        match ask_if_number_is_bigger(guess) {
            Ordering::Greater => {
                min = guess;
            }
            Ordering::Less => {
                max = guess;
            }
            Ordering::Equal => {
                break;
            }
        }
        guess = (min + max) / 2;
        if max - min < 3 {
            break;
        }
    }

    println!("Guessed number: {}", guess);
}

fn ask_if_number_is_bigger(guess: i32) -> Ordering {
    print!("{guess}: ");
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    answer = answer.trim().to_string();

    match answer.as_str() {
        "=" => Ordering::Equal,
        "-" => Ordering::Less,
        _ => Ordering::Greater,
    }
}

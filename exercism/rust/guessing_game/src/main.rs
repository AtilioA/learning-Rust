use rand::Rng;
use std::cmp;
use std::io;

fn read_input(min: u8, max: u8) -> String {
    println!(
        "Please input your guess ({min}-{max}):",
        min = min,
        max = max
    );

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input.");

    guess = guess.trim().to_string();

    return guess;
}

fn print_invalid_input(min: u8, max: u8) {
    println!(
        "Invalid input! A valid input is a number between {min} and {max}.\n",
        min = min,
        max = max
    );
}

fn guess(secret_number: u8, min: u8, max: u8) {
    loop {
        let guess: u8 = match read_input(min, max).parse() {
            // Guess must be a number.
            Ok(num) => num,
            Err(_) => {
                print_invalid_input(min, max);
                continue;
            }
        };

        if guess < min || guess > max {
            // Guess must be between min and max.
            print_invalid_input(min, max);
            continue;
        }

        println!("Your guess is: {}.", guess);

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Too small!"),
            cmp::Ordering::Greater => println!("Too big!"),
            cmp::Ordering::Equal => {
                println!("You guessed right!");
                break; // Exit guess loop.
            }
        }
    }
}

fn guessing_game(min: u8, max: u8) {
    // Generate random number between range within the current thread of execution
    let secret_number: u8 = rand::thread_rng().gen_range(min..=max);

    guess(secret_number, min, max);
}

fn main() {
    // Limits for the guessing game.
    const MIN: u8 = 1;
    const MAX: u8 = 100;

    println!("Guess the number!\n");

    guessing_game(MIN, MAX);
}

use rand::Rng;
use std::io;
use std::cmp;

fn main() {
    println!("Guess the number!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is: {}.", secret_number);
    println!("Please input your guess (0-100):");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input.");

    let guess: u8 = guess.trim().parse().expect("Please type a number (0-100)!");

    println!("Your guess is: {}.", guess);

    match guess.cmp(&secret_number) {
        cmp::Ordering::Less => println!("Too small!"),
        cmp::Ordering::Greater => println!("Too big!"),
        cmp::Ordering::Equal => println!("You guessed right!"),
    }
}

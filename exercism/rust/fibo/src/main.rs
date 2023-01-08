use std::io;

fn calculate_fibonnaci_iter(limit: usize) -> usize {
    let mut fibonnaci_numbers: Vec<usize> = vec![0, 1];
    fibonnaci_numbers[0] = 0;
    fibonnaci_numbers[1] = 1;

    let mut current_fibonacci_number: usize = 0;
    for _ in 0..limit {
        current_fibonacci_number = fibonnaci_numbers[0] + fibonnaci_numbers[1];
        fibonnaci_numbers[0] = fibonnaci_numbers[1];
        fibonnaci_numbers[1] = current_fibonacci_number;
    }

    return current_fibonacci_number;
}

fn read_input() -> String {
    println!("Enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input = input.trim().to_string();

    return input;
}

fn main() {
    println!("Fibonacci calculator");
    println!("===================\n");

    let number: usize = loop {
        match read_input().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };
    };

    println!("Input: {number}.");
    println!("Calculating fibonacci sequence until term #{number}...");

    let fib_number: usize = calculate_fibonnaci_iter(number);
    println!("The #{number} fibonacci term is {fib_number}.");
}

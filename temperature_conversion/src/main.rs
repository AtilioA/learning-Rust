use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    return input;
}

fn main() {
    println!("Temperature converter");
    println!("=====================");
    loop {
        println!("\nEnter a temperature:");

        let temperature: f32 = loop {
            match read_input().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input. Try again:");
                    continue;
                }
            };
        };

        println!("\nSelect a conversion:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Quit\n");

        let choice: u8 = loop {
            match read_input().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Invalid input. Try again:");
                    continue;
                }
            };
        };

        match choice {
            1 => {
                println!("Celsius to Fahrenheit");
                println!("=====================");
                let temperature_f: f32 = temperature * 1.8 + 32.0;
                println!("{temperature}ºC = {temperature_f}ºF");
            }
            2 => {
                println!("Fahrenheit to Celsius");
                println!("=====================");
                let temperature_c: f32 = (temperature - 32.0) / 1.8;
                println!("{temperature}ºF = {temperature_c}ºC");
            }
            3 => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Invalid choice. Try again.");
            }
        }
    }
}

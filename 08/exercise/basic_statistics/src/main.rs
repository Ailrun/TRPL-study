mod calculation;
mod user_input;

use user_input::UserInput;

fn main() {
    let mut numbers = Vec::new();
    let number_of_numbers: usize = get_number_of_numbers();

    loop {
        if numbers.len() >= number_of_numbers {
            break;
        }

        println!("Input {}th number (a to abort)", numbers.len());

        match UserInput::from_stdin() {
            UserInput::NumberInput(number) => {
                numbers.push(number);
            },
            UserInput::AbortCommand => {
                println!("Aborted by input!");
                return;
            },
            _ => {
                println!("Please input a valid number");
                continue;
            },
        };
    }

    use calculation::*;

    let mean = calculate_mean(&numbers);
    let median = calculate_median(&numbers);
    let mode = calculate_mode(&numbers);
    println!("Mean is {}", mean);
    println!("Median is {}", median);
    println!("Mode is {}", mode);
}

fn get_number_of_numbers() -> usize {
    loop {
        println!("How many numbers you want to input?");

        match UserInput::from_stdin() {
            UserInput::NumberInput(number) => {
                return number;
            },
            _ => {
                println!("Please input a valid number");
                continue;
            },
        }
    }
}

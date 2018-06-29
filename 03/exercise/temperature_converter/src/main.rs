use std::io::stdin;

fn main() {
    loop {
        println!("In which way do you want to convert a temperature?");
        println!("1) Celsius to Fahrenheit");
        println!("2) Fahrenheit to Celsius");
        println!("q) Quit");

        let mut option = String::new();
        option.clear();
        stdin().read_line(&mut option)
            .expect("Failed to read line");

        if option.trim() == "q" {
            println!("Bye!");
            return;
        }

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input one of the valid options");
                continue;
            },
        };

        let (input_unit, result_unit) = if option == 1 {
            ("celsius", "fahrenheit")
        } else if option == 2 {
            ("fahrenheit", "celsius")
        } else {
            println!("Please input one of the valid options");
            continue;
        };
        
        let input_temperature: f64;
        let result_temperature: f64;

        loop {
            println!("Input temperature in {} (q to quit): ", input_unit);

            let mut user_input = String::new();
            stdin().read_line(&mut user_input)
                .expect("Failed to read line");

            if user_input.trim() == "q" {
                println!("Bye!");
                return;
            }

            input_temperature = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a valid number");
                    continue;
                }
            };

            break;
        }

        if option == 1 {
            result_temperature = celsius_to_fahrenheit(input_temperature);
        } else {
            result_temperature = fahrenheit_to_celsius(input_temperature);
        }

        println!("Result temperature in {}: {}", result_unit, result_temperature);
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

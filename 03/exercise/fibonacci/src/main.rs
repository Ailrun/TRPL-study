use std::io::stdin;

fn main() {
    loop {
        println!("How many fibonacci numbers are there before the one you want? (q to quit)");

        let mut n = String::new();
        stdin().read_line(&mut n)
            .expect("Failed to read line");

        if n.trim() == "q" {
            println!("Bye!");
            return;
        }

        let n: u8 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid positive number");
                continue;
            }
        };

        if n > 92 {
            println!("93th fibonacci number and after are impossible to calculate in 64bit integer");
            continue;
        }

        println!("{}th fibonacci number is: {}", n, fibonacci(n));
    }
}

fn fibonacci(n: u8) -> u64 {
    let mut last_two = (0, 1);

    for _ in 0..n {
        last_two = (last_two.1, last_two.0 + last_two.1);
    }

    return last_two.0;
}

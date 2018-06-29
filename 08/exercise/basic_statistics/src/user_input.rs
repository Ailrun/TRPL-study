pub enum UserInput {
    NumberInput(usize),
    AbortCommand,
    InvalidInput,
}

impl UserInput {
    pub fn from_stdin() -> UserInput {
        use std::io::stdin;

        let mut number = String::new();
        stdin().read_line(&mut number)
            .expect("Failed to read line");
        let number = number.trim();

        if number == "a" {
            UserInput::AbortCommand
        } else {
            match number.parse() {
                Ok(num) => UserInput::NumberInput(num),
                Err(_) => UserInput::InvalidInput,
            }
        }
    }
}

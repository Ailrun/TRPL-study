mod user_input;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut company = HashMap::new();

    use user_input::{ AddCommandInput, ListCommandInput, UserInput };
    loop {
        println!("Input a command");
        print!("> ");
        io::stdout().flush()
            .expect("Failed to flush stdout");

        match UserInput::from_stdin() {
            UserInput::AddCommand(AddCommandInput{ name, department }) => {
                let members = company.entry(department).or_insert(Vec::new());
                members.push(name);
            },
            UserInput::ListCommand(ListCommandInput{ department }) => {
                match company.get(&department) {
                    None => {
                        println!("There is no such department. Try again.");
                        continue;
                    },
                    Some(members) => {
                        for member in members {
                            println!("{}", member);
                        }
                    }
                }
            },
            UserInput::QuitCommand => {
                println!("Bye!");
                return;
            },
            UserInput::InvalidCommand => {
                println!("Invalid command. Command should be one of the followings");
                println!("- Add command");
                println!("  FORM: Add <name> to <department>");
                println!("- List command");
                println!("  FORM: List members of <department>");
                println!("- Quit command");
                println!("  FORM: Quit");
                continue;
            },
        };
    }
}

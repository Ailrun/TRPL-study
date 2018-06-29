pub struct AddCommandInput {
    pub name: String,
    pub department: String,
}

pub struct ListCommandInput {
    pub department: String,
}

pub enum UserInput {
    AddCommand(AddCommandInput),
    ListCommand(ListCommandInput),
    QuitCommand,
    InvalidCommand,
}

impl UserInput {
    pub fn from_stdin() -> UserInput {
        use std::io::stdin;
        use self::UserInput::*;

        let mut user_input = String::new();
        stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        let words: Vec<_> = user_input.split_whitespace().collect();

        match match words[0] {
            "Add" => make_add_command_input(&words[1..]).map(AddCommand),
            "List" => make_list_command_input(&words[1..]).map(ListCommand),
            "Quit" => Some(QuitCommand),
            _ => None,
        } {
            Some(command) => command,
            None => InvalidCommand,
        }
    }
}

fn make_add_command_input(words: &[&str]) -> Option<AddCommandInput> {
    let mut name = String::new();
    let mut department = String::new();
    let mut met_to = false;

    for word in words {
        if met_to {
            department += word;
        } else if *word == "to" {
            met_to = true;
        } else {
            if name != "" {
                name += " ";
            }
            name += word;
        }
    }

    let name = name.trim().to_string();
    let department = department.trim().to_string();

    if met_to && name != "" && department != "" {
        Some(AddCommandInput {
            name,
            department,
        })
    } else {
        None
    }
}

fn make_list_command_input(words: &[&str]) -> Option<ListCommandInput> {
    match words[0..2] {
        ["members", "of"] => {
            let department = words[2..].join(" ");
            Some(ListCommandInput {
                department,
            })
        },
        _ => None,
    }
}

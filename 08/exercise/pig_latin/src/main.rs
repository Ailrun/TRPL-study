fn main() {
    println!("Input your word converted to pig latin");

    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input = user_input.trim();

    println!("Histay ishay igpay atinlay: {}", convert_to_pig_latin(&user_input));
}

fn convert_to_pig_latin(s: &str) -> String {
    let mut chars = s.chars();

    match chars.next() {
        Some(head) => {
            if is_vowel(&head) {
                return format!("{}{}-hay", head, chars.as_str());
            }

            return format!("{}-{}ay", chars.as_str(), head);
        },
        None => String::new(),
    }
}

fn is_vowel(c: &char) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    for vowel in &VOWELS {
        if c.to_lowercase().to_string() == vowel.to_string() {
            return true;
        }
    }

    return false;
}

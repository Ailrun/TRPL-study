fn main() {
    let ordinals = [
        "first", "second", "third",
        "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // From https://en.wikipedia.org/wiki/The_Twelve_Days_of_Christmas_(song)
    let items = [
        "a partridge in a pear-tree",
        "Two turtle doves",
        "Three french hens",
        "Four colly birds",
        "Five golden rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve lords a leaping",
    ];

    for nth_part in 0..ordinals.len() {
        println!("On the {} day of Christmas my true love sent to me", ordinals[nth_part]);
        for nth_line in 0..(nth_part + 1) {
            if nth_line != nth_part {
                println!("{},", items[nth_part - nth_line]);
            } else {
                if nth_part > 0 {
                    print!("and ");
                }
                println!("{}.", items[0])
            }
        }
    }
}

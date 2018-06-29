pub fn calculate_mean(numbers: &Vec<usize>) -> usize {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    return sum / numbers.len();
}

pub fn calculate_median(numbers: &Vec<usize>) -> usize {
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    sorted_numbers[numbers.len() / 2]
}

pub fn calculate_mode(numbers: &Vec<usize>) -> usize {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for number in numbers {
        let count = map.entry(*number).or_insert(0);
        *count += 1;
    }

    let mut max_entry = (0, 0);

    for (number, count) in &map {
        if max_entry.1 < *count {
            max_entry = (*number, *count);
        }
    }

    max_entry.0
}

const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn main() {
    let input = include_str!("../input.txt");

    let total_calibration_value = compute_calibration(input);

    println!("{}", total_calibration_value);
}

pub fn compute_calibration(input: &str) -> u32 {
    input.lines().map(|line| {
        let first_digit = find_digit(0..line.len(), line);
        let last_digit = find_digit((0..line.len()).rev(), line);
        10 * first_digit + last_digit
    }).sum()
}

fn find_digit<I>(mut iterator: I, line: &str) -> u32
where
    I: Iterator<Item = usize>,
{
    iterator.find_map(|index| {
        NUMBER_WORDS.iter()
            .enumerate()
            .find(|(_, &word)| line[index..].starts_with(word))
            .map(|(i, _)| i as u32 + 1)
            .or_else(|| line[index..].chars().next().and_then(|c| c.to_digit(10)))
    }).unwrap()
}


pub fn main() {
    let sum: u32 = include_str!("../input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .filter_map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
                Some(10 * first + last)
            } else {
                None
            }
        })
        .sum();

    println!("{}", sum);
}


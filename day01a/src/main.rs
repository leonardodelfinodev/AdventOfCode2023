pub fn main() {
    println!("{}",
        include_str!("../input.txt")
        .lines()
        .filter_map(|line| {
            let first_digit = line.chars().find(|c| c.is_digit(10))?;
            let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;
            format!("{}{}", first_digit, last_digit).parse::<u32>().ok()
        })
        .sum::<u32>()
    );
}

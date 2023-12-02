pub fn main() {
    let input = include_str!("../input.txt");

    let total_power: u64 = input.lines().map(compute_power).sum();
    println!("{}", total_power);
}

fn compute_power(game_data: &str) -> u64 {
    let (_, rounds) = game_data.split_once(": ").unwrap();
    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

    for round in rounds.split(';') {
        for cube_info in round.split(',') {
            let mut parts = cube_info.trim().split_whitespace();
            let count_str = parts.next().unwrap_or_default();
            let color = parts.next().unwrap_or_default().chars().next().unwrap_or('\0');

            let count: u64 = match fast_parse(count_str) {
                Some(num) => num,
                None => return 0,
            };

            match color {
                'r' => max_red = max_red.max(count),
                'g' => max_green = max_green.max(count),
                'b' => max_blue = max_blue.max(count),
                _ => (),
            }
        }
    }

    max_red * max_green * max_blue
}

fn fast_parse(num_str: &str) -> Option<u64> {
    num_str.chars().try_fold(0u64, |acc, c| c.to_digit(10).map(|d| acc * 10 + d as u64))
}


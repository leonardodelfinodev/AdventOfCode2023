pub fn main() {
    let input = include_str!("../input.txt");

    let games: Vec<&str> = input.lines().collect();
    let total_power: u64 = games.iter().map(|&game| compute_power(game)).sum();

    println!("{}", total_power);
}

fn compute_power(game_data: &str) -> u64 {
    let rounds = game_data.split(": ").nth(1).unwrap().split("; ");
    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

    for round in rounds {
        let cubes = round.split(", ");
        for cube_info in cubes {
            let parts: Vec<&str> = cube_info.split_whitespace().collect();
            let count: u64 = parts[0].parse().unwrap();
            match parts[1] {
                "red" => max_red = max_red.max(count),
                "green" => max_green = max_green.max(count),
                "blue" => max_blue = max_blue.max(count),
                _ => (),
            }
        }
    }

    max_red * max_green * max_blue
}

// Regex solution
// More elegant, but less efficient
/*
use std::cmp;
use regex::Regex;

pub fn main() {
    let input = include_str!("../input.txt");

    let games: Vec<&str> = input.lines().collect();

    let regex = Regex::new(r"(\d+) red|(\d+) green|(\d+) blue").unwrap();
    let total_power: u64 = games.iter().map(|&game| process_game(game, &regex)).sum();
    
    println!("{}", total_power);
}

fn process_game(game_data: &str, regex: &Regex) -> u64 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for cap in regex.captures_iter(game_data) {
        if let Some(red) = cap.get(1) {
            max_red = cmp::max(max_red, red.as_str().parse::<u64>().unwrap());
        }
        if let Some(green) = cap.get(2) {
            max_green = cmp::max(max_green, green.as_str().parse::<u64>().unwrap());
        }
        if let Some(blue) = cap.get(3) {
            max_blue = cmp::max(max_blue, blue.as_str().parse::<u64>().unwrap());
        }
    }

    max_red * max_green * max_blue
}
*/


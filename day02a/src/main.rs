pub fn main() {
    let input = include_str!("../input.txt");

    let games: Vec<&str> = input.lines().collect();

    // red, green, blue
    let max_cubes = (12, 13, 14);
    let mut sum_ids = 0;

    for game in games.iter() {
        let (id, is_possible) = process_game(game, max_cubes);
        if is_possible {
            sum_ids += id;
        }
    }

    println!("{}", sum_ids);
}

fn process_game(game_data: &str, max_cubes: (usize, usize, usize)) -> (usize, bool) {
    let parts: Vec<&str> = game_data.split(": ").collect();
    let id: usize = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();
    let rounds = parts[1].split("; ");

    for round in rounds {
        let (red, green, blue) = count_cubes(round);
        if red > max_cubes.0 || green > max_cubes.1 || blue > max_cubes.2 {
            return (id, false);
        }
    }

    (id, true)
}

fn count_cubes(round: &str) -> (usize, usize, usize) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for cube_info in round.split(", ") {
        let parts: Vec<&str> = cube_info.split_whitespace().collect();
        let count: usize = parts[0].parse().unwrap();
        match parts[1] {
            "red" => red += count,
            "green" => green += count,
            "blue" => blue += count,
            _ => (),
        }
    }

    (red, green, blue)
}

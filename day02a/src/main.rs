pub fn main() {
    let input = include_str!("../input.txt");

    // red, green, blue
    let max_cubes = (12, 13, 14);
    let mut sum_ids = 0;

    for game in input.lines() {
        let (id, is_possible) = process_game(game, max_cubes);
        if is_possible {
            sum_ids += id;
        }
    }

    println!("{}", sum_ids);
}

fn process_game(game_data: &str, max_cubes: (usize, usize, usize)) -> (usize, bool) {
    let mut parts = game_data.split(": ");
    let id: usize = parts.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
    let rounds = parts.next().unwrap().split("; ");

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
        let mut parts = cube_info.split_whitespace();
        let count: usize = parts.next().unwrap().parse().unwrap();
        match parts.next().unwrap() {
            "red" => red += count,
            "green" => green += count,
            "blue" => blue += count,
            _ => (),
        }
    }

    (red, green, blue)
}


use std::{cmp::max, collections::HashMap};

use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let lines: Vec<_> = input.lines().collect();

    let max_values = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut sum_possible = 0;
    let mut sum_power = 0;

    input.lines().for_each(|game| {
        // Find game ID
        let re = Regex::new(r"Game (\d+)").unwrap();
        let (_full, [id]) = re.captures(game).unwrap().extract();

        let mut possible = true;
        let mut max_in_game = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        game.split(";").for_each(|draw| {
            let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

            re.captures_iter(draw).for_each(|cap| {
                let (_full, [number, color]) = cap.extract();
                let tmp_max = max_in_game[color];
                max_in_game.insert(color, max(tmp_max, number.parse::<u32>().unwrap()));

                if number.parse::<u32>().unwrap() > max_values[color] {
                    possible = false;
                }
            });
        });

        sum_power += max_in_game["red"] * max_in_game["green"] * max_in_game["blue"];

        if possible {
            sum_possible += id.parse::<u32>().unwrap();
        }
    });
    println!("Sum of possible games' IDs: {}", sum_possible);
    println!("Sum of powers of min cubes present {}", sum_power);
}

//

use std::collections::HashMap;

fn get_card_id(line: &str) -> usize {
    line.split(":")
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn get_winning_numbers(line: &str) -> Vec<i32> {
    let mut nums = vec![];

    line.split("|")
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .for_each(|elem| match elem.parse::<i32>() {
            Ok(val) => nums.push(val),
            Err(_) => (),
        });

    nums
}

fn get_my_numbers(line: &str) -> Vec<i32> {
    let mut nums = vec![];

    line.split("|")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .for_each(|elem| match elem.parse::<i32>() {
            Ok(val) => nums.push(val),
            Err(_) => (),
        });

    nums
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut total_points = 0;

    for line in input.lines() {
        let winrar = get_winning_numbers(line);
        let mine = get_my_numbers(line);

        // dbg!(&winrar);
        // dbg!(&mine);

        // Determine how many points were gained
        // 0 correct numbers -> 0
        // n correct numbers -> 2^(n-1)
        let mut n_correct = 0;
        for val_mine in mine {
            if winrar.contains(&val_mine) {
                n_correct += 1;
            }
        }
        let points = if n_correct == 0 {
            0
        } else {
            2_i32.pow(n_correct - 1)
        };
        // dbg!(points);
        total_points += points;
    }
    println!("Total points: {}", total_points);

    // Part 2:

    // Determine how many of my numbers match the winning numbers
    fn get_number_of_matches(winning_numbers: &[i32], my_numbers: &[i32]) -> usize {
        let mut total = 0;
        for val in my_numbers {
            if winning_numbers.contains(val) {
                total += 1;
            }
        }
        total
    }

    // Compute how many scratchcards result from this one
    fn get_number_of_cards(
        lines: &[&str],
        idx: usize,
        mut memo: HashMap<usize, usize>,
    ) -> (usize, HashMap<usize, usize>) {
        let line = lines[idx];
        let card_id = get_card_id(line);
        let winrar = get_winning_numbers(line);
        let mine = get_my_numbers(line);

        if memo.contains_key(&idx) {
            return (memo.get(&idx).unwrap().clone(), memo);
        }

        // We get one copy of the next n_matches cards
        let n_matches = get_number_of_matches(&winrar, &mine);
        // print!("card {} -> ", card_id);
        // Include this card -> start at 1
        let mut total = 1;
        for id in card_id + 1..=card_id + n_matches {
            // print!("{} ", id);
            let (add, memo_new) = get_number_of_cards(lines, id - 1, memo);
            memo = memo_new;
            total += add
        }
        // println!();
        memo.insert(idx, total);

        (total, memo)
    }
    let lines = input.lines().collect::<Vec<_>>();
    let mut n_cards = 0;
    let mut memo = HashMap::new();
    for idx in 0..lines.len() {
        // Total number of scratchcards earned with the current card (including this one)
        let (add, memo_new) = get_number_of_cards(&lines, idx, memo);
        memo = memo_new;
        n_cards += add
    }
    println!("Total scratchcards: {}", n_cards);
}

use regex::Regex;

fn is_symbol(c: &char) -> bool {
    !c.is_digit(10) && c != &'.'
}
#[derive(Debug, Clone)]
struct DigitGroup {
    digits: Vec<u32>,
    positions: Vec<(usize, usize)>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Represent as a 2D vector of chars
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    // First, find all groups of digits using a regex and mark down their location, value, etc
    let re_digits = Regex::new(r"\d+").unwrap();

    let mut digit_groups: Vec<DigitGroup> = vec![];
    input.lines().enumerate().for_each(|(y_idx, line)| {
        re_digits.find_iter(line).for_each(|m| {
            dbg!(&m);
            let mut p = DigitGroup {
                digits: vec![],
                positions: vec![],
            };
            m.as_str().chars().enumerate().for_each(|(idx, c)| {
                p.digits.push(c.to_digit(10).unwrap());
                p.positions.push((m.start() + idx, y_idx));
            });
            digit_groups.push(p);
        });
    });

    // dbg!(&digit_groups);

    // Then, filter them based on whether a symbol is adjacent to any of their digits
    let mut part_numbers: Vec<DigitGroup> = vec![];

    digit_groups.iter().for_each(|group| {
        // Turn true if any char adjacent to the group is a symbol
        let mut is_part = false;
        group.positions.clone().into_iter().for_each(|(x, y)| {
            let x_start = if x == 0 { x } else { x - 1 };
            let x_end = if x == width - 1 { x } else { x + 1 };
            let y_start = if y == 0 { y } else { y - 1 };
            let y_end = if y == height - 1 { y } else { y + 1 };

            for i in x_start..=x_end {
                for j in y_start..=y_end {
                    if is_symbol(&grid[j][i]) {
                        is_part = true;
                    }
                }
            }
        });
        if is_part {
            part_numbers.push(group.clone());
        }
    });
    // dbg!(&part_numbers);
    let sum_of_part_numbers = part_numbers.iter().fold(0_u32, |acc_total, x| {
        let num = x
            .digits
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc_group, (idx, digit)| {
                10_u32.pow(idx as u32) * digit + acc_group
            });
        num + acc_total
    });
    println!("Sum of part numbers: {}", sum_of_part_numbers);

    // Part 2:
    // Find all star symbols
    let star_symbols: Vec<(usize, usize)> =
        grid.iter()
            .enumerate()
            .fold(vec![], |mut acc, (y_idx, line)| {
                line.iter().enumerate().for_each(|(x_idx, c)| {
                    if c == &'*' {
                        acc.push((x_idx, y_idx));
                    }
                });
                acc
            });
    // dbg!(&star_symbols);

    // Find star symbols that have two adjacent part numbers !
    let gear_ratios: Vec<u32> = star_symbols.iter().fold(vec![], |mut acc, ss| {
        // Turn true if exactly two part numbers are adjacent to this star symbol
        let mut gear_ratio = 1;
        let mut n_adjacent = 0; // adjacent part numbers
        for dg in &part_numbers {
            // Turn true if this part number is adjacent to the star symbol
            let mut is_adjacent = false;
            for (digit_x, digit_y) in &dg.positions {
                if digit_x.abs_diff(ss.0) <= 1 && digit_y.abs_diff(ss.1) <= 1 {
                    is_adjacent = true;
                }
            }
            if is_adjacent {
                n_adjacent += 1;
                gear_ratio *= dg
                    .digits
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc_group, (idx, digit)| {
                        10_u32.pow(idx as u32) * digit + acc_group
                    });
            }
        }
        // println!("{} {} -> {} adjacent", ss.0, ss.1, &n_adjacent);
        if n_adjacent == 2 {
            acc.push(gear_ratio);
        }
        acc
    });
    // dbg!(&gear_ratios);
    println!(
        "Total gear ratio: {}",
        gear_ratios.iter().fold(0, |acc, r| acc + r)
    );
}

use std::collections::{HashMap, HashSet};

use colored::{Colorize, CustomColor};

type Position = (i32, i32);

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut path: Vec<Position> = vec![];
    path.push((0, 0));

    for instruction in input.lines() {
        let mut groups = instruction.split_whitespace();
        // dbg!(&groups);
        let direction = groups.next().unwrap();
        let distance = groups.next().unwrap().parse::<usize>().unwrap();

        match direction {
            "R" => {
                // dbg!(&path);
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0 + 1, last.1));
                }
            }
            "L" => {
                // dbg!(&path);
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0 - 1, last.1));
                }
            }
            "U" => {
                // dbg!(&path);
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0, last.1 - 1));
                }
            }
            "D" => {
                // dbg!(&path);
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0, last.1 + 1));
                }
            }
            _ => panic!(),
        }
    }
    // dbg!(&path);

    // show path
    let max_x = path.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = path.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_x = path.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = path.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    dbg!(width, height);

    // for j in min_y..=max_y {
    //     for i in min_x..=max_x {
    //         if path.contains(&(i, j)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    // println!("\n");
    // fill path
    let filled_count = get_filled_count(&path, min_x, max_x, min_y, max_y);
    println!("Filled tiles: {}", filled_count);

    // Part 2
    // Reinterpret input:
    // TODO instead of considering each step in the path, consider edges of the pool
    // Define edge intersection method so we can more quickly determine if a point is inside the polygon
    for instruction in input.lines() {
        let mut groups = instruction.split_whitespace();
        // dbg!(&groups);
        let direction = groups.next().unwrap();
        let distance = groups.next().unwrap().parse::<usize>().unwrap();
        let color = groups
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(')');
        let dist_hex = &color[..5];
        let dir_hex = color.chars().nth(5).unwrap();
        // dbg!(&dist_hex, &dir_hex);
        let distance = i32::from_str_radix(dist_hex, 16).unwrap();
        println!("{} {}", dir_hex, distance);
        match dir_hex {
            '0' => {
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0 + 1, last.1));
                }
            },
            '1' => {
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0, last.1 + 1));
                }
            },
            '2' => {
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0 - 1, last.1));
                }
            },
            '3' => {
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0, last.1 - 1));
                }
            },
            _ => panic!(),
        }
    }
    let max_x = path.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = path.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_x = path.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = path.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    dbg!(width, height);
    println!("Filled {}", get_filled_count(&path, min_x, max_x, min_y, max_y));
}

fn get_filled_count(path: &Vec<Position>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> i32 {
    let mut filled_count = 0;
    let mut previous_line_inside = HashMap::new();
    for j in min_y..=max_y {
        println!("line {}", j);
        let mut last = false;
        let mut inside = 0;
        let mut on_edge = false;
        let mut this_line_inside = HashMap::new();

        for i in min_x..=max_x {
            let cur = path.contains(&(i, j));
            // dbg!(i);
            if cur != last {
                inside += 1;
                if on_edge {
                    if j == min_y {
                        // We are outside
                        inside = 0;
                    } else {
                        // Use previous line to determine if we are inside after the edge
                        if let Some(v) = previous_line_inside.get(&i) {
                            inside = *v;
                        } else {
                            inside = 0;
                        }
                    }
                    on_edge = false;
                }
            } else if cur && last {
                // We are on an edge
                on_edge = true;
                // println!("edge {} {}", i, j);
            }
            last = cur;
            // println!("{} {} -> {}", i, j, inside);

            if inside % 4 != 0 || cur {
                this_line_inside.insert(i, inside);
                filled_count += 1;
                if cur {
                    // print!("{}", "#".custom_color(CustomColor { r: 10, g: 255, b: 150 }));
                } else {
                    // print!("#");
                }
            } else {
                // print!(".");
            }
        }
        // println!();
        previous_line_inside = this_line_inside;
    }
    filled_count
}

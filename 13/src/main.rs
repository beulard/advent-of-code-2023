fn find_horizontal_reflection(pattern: &Vec<String>) -> usize {
    // Find a line index such that going down from it or up gives the same line, up to the lower or upper
    // border of the input.
    let height = pattern.len();
    let mut mirror_idx = 0;
    for i in 1..height {
        mirror_idx = i;
        for j in i..height {
            // if i == 8  {
            // println!("j {}, i {}", j, i);
            // println!("{}", i - (j-i) - 1);
            // println!("{}\n{}", pattern[j], pattern[i - (j-i) - 1]);
            // }
            if pattern[j] != pattern[i - (j - i) - 1] {
                // println!("no mirror {}\n", i);
                mirror_idx = 0;
            } else {
                // println!("line ok\n");
            }
            if i - (j - i) - 1 == 0 {
                break;
            }
        }
        // println!("{}", mirror_idx);
        if mirror_idx == i {
            break;
        }
    }
    mirror_idx
}

fn rotate_clockwise(pattern: &Vec<String>) -> Vec<String> {
    let width = pattern[0].len();
    let height = pattern.len();

    let mut pattern_rot = vec![String::new(); width];
    for i in 0..width {
        for j in 0..height {
            pattern_rot[i].push(pattern[height - j - 1].chars().nth(i).unwrap());
        }
    }
    pattern_rot
}

fn part1(input_vec: &Vec<Vec<String>>) {
    let mut total = 0;
    for pattern in input_vec {
        let mirror_idx = find_horizontal_reflection(pattern);
        total += 100 * mirror_idx;

        // Repeat with a rotation of the input
        let pattern_rot = rotate_clockwise(pattern);
        let mirror_idx = find_horizontal_reflection(&pattern_rot);

        total += mirror_idx;
    }
    println!("Total: {}", total);
}

fn find_smudged_horizontal_reflection(pattern: &Vec<String>) -> usize {
    // Find a line index such that going down from it or up gives the same line,
    // allowing exactly one error in the reflection pattern.
    let height = pattern.len();
    let mut mirror_idx = 0;
    for i in 1..height {
        mirror_idx = i;
        let mut errors = 0;
        for j in i..height {
            // println!("j {}, i {}", j, i);
            // println!("{}", i - (j-i) - 1);
            // println!("{}\n{}", pattern[j], pattern[i - (j-i) - 1]);
            for (idx, c) in pattern[j].chars().enumerate() {
                if c != pattern[i - (j - i) - 1].chars().nth(idx).unwrap() {
                    errors += 1;
                    if errors > 1 {
                        mirror_idx = 0;
                    }
                }
            }
            if i - (j - i) - 1 == 0 {
                break;
            }
        }
        // Need one error
        if errors == 0 {
            mirror_idx = 0;
        }
        // println!("{}", mirror_idx);
        if mirror_idx == i {
            break;
        }
    }
    mirror_idx
}

fn part2(input_vec: &Vec<Vec<String>>) {
    let mut total = 0;
    for pattern in input_vec {
        let mirror_idx = find_smudged_horizontal_reflection(pattern);
        total += 100 * mirror_idx;

        let pattern_rot = rotate_clockwise(&pattern);
        let mirror_idx = find_smudged_horizontal_reflection(&pattern_rot);
        total += mirror_idx;
    }
    println!("Total: {}", total);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_vec: Vec<Vec<String>> = input
        .split("\n\n")
        .map(|c| c.lines().map(|l| l.to_string()).collect())
        .collect();
    // dbg!(&input_vec);

    part1(&input_vec);
    part2(&input_vec);
}

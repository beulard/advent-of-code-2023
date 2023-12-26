use std::collections::HashMap;

fn find_good_combinations(counts: &Vec<usize>, states: &String) -> Option<Vec<String>> {
    // Determine if `states` could be a fit for `counts`.
    let mut cur_counts = vec![];
    let mut contiguous = 0;
    let mut cur_group = 0;

    // println!("states {}", states);

    // First, determine whether these `states` are consistent with counts, starting
    // from the left of the row. In any subgroup, if the number of contiguous broken
    // machines is different from the corresponding group in the input (`counts`), then
    // this configuration is impossible and we can return now.
    for c in states.chars() {
        if c == '?' {
            // If we encounter an unknown state, we cannot tell whether this is a good combination
            // -> Continue to algorithm
            break;
        }
        if c == '#' {
            contiguous += 1;
            if cur_group < counts.len() && contiguous > counts[cur_group] {
                // We are in the case where the current group of broken
                // machines is larger than the corresponding count in `counts`.
                return None;
            }
        } else if c == '.' {
            if contiguous > 0 {
                if cur_group < counts.len() && contiguous != counts[cur_group] {
                    // This subgroup is broken by a '.' but doesn't match the input `counts`
                    // -> Abort early
                    return None;
                }

                cur_counts.push(contiguous);
                contiguous = 0;
                cur_group += 1;
            }
        }
    }

    // If we have no unknowns, we can go ahead and calculate the counts, and see
    // if they match the input.
    if !states.contains('?') {
        let mut cur_counts = vec![];
        let mut contiguous: usize = 0;
        for c in states.chars() {
            if c == '#' {
                contiguous += 1;
            } else if c == '.' {
                if contiguous > 0 {
                    cur_counts.push(contiguous);
                    contiguous = 0;
                }
            }
        }
        // dbg!(contiguous, &cur_counts);
        if contiguous > 0 {
            cur_counts.push(contiguous);
        }
        if cur_counts == *counts {
            // dbg!(states);
            return Some(vec![states.clone()]);
        } else {
            return None;
        }
    }

    // The actual algorithm:
    // Find the first '?' in the states, and compute the resulting configuration
    // assuming this machine is broken, or working.
    let mut ret = vec![];

    // Take the first '?' in the states and find combinations in either case ('.' or '#')
    let idx = states.chars().position(|c| c == '?').unwrap(); // safe to unwrap since we would've returned earlier if there was no '?'

    // Try with this char swapped with a #
    let mut states_broken = states[..idx].to_string();
    states_broken.push('#');
    states_broken.push_str(&states[idx + 1..]);
    let good_broken = find_good_combinations(counts, &states_broken);

    // Try with this char swapped with a .
    let mut states_working = states[..idx].to_string();
    states_working.push('.');
    states_working.push_str(&states[idx + 1..]);
    let good_working = find_good_combinations(counts, &states_working);

    if good_broken.is_none() && good_working.is_none() {
        return None;
    } else {
        // dbg!(&good_broken, &good_working);
        if let Some(v) = good_broken {
            // println!("len: {}", v.len());
            ret.extend(v);
        }
        if let Some(v) = good_working {
            // println!("len: {}", v.len());
            ret.extend(v);
        }
        Some(ret)
    }
}

fn find_good_combinations_count<'a>(
    counts: &'a [usize],
    states: &str,
    group_len: usize,
    memo: &mut HashMap<(&'a [usize], String, usize), usize>,
) -> usize {

    // Memoization
    if let Some(v) = memo.get(&(counts, states.to_string(), group_len)) {
        return *v;
    }

    // Default cases:
    // Counts is empty
    if counts.is_empty() {
        // This means all groups so far have matched -> there is one solution here where all remaining '?'s are '.'s
        if !states.contains('#') {
            return 1;
        } else {
            // We cannot fit the remaining '#'s, so this is not a solution
            return 0;
        }
    }
    if states.is_empty() {
        // We have no more chars in the states. Check that there is only one remaining group and that its length matches our current group_len
        // Close the last group
        if counts.len() == 1 {
            // println!("one more group: {}, group len: {}", counts[0], group_len);
            if group_len == counts[0] {
                // println!("OK");
                return 1;
            }
        }
        return 0;
    }

    // Handle next char
    match states.chars().nth(0).unwrap() {
        '#' => {
            return find_good_combinations_count(
                &counts,
                &states[1..],
                group_len + 1,
                memo,
            )
        }
        '.' => {
            if group_len > 0 {
                // End of group -> check that the group_len matches the next `counts`.
                if counts[0] != group_len {
                    // Pattern cannot fit counts
                    // println!("{:?} {}", counts, group_len);
                    return 0;
                } else {
                    let ret = find_good_combinations_count(
                        &counts[1..],
                        &states[1..],
                        0,
                        memo,
                    );
                    // println!("{} {:?} -> {:?}", states, &counts, &ret);
                    return ret;
                }
            } else {
                // Not end of group, just move over by 1
                return find_good_combinations_count(
                    &counts,
                    &states[1..],
                    0,
                    memo,
                );
            }
        }
        '?' => {
            // println!("??");
            // Return result of '.' case and '#' case
            let mut dotcase = String::with_capacity(states.len());
            dotcase.push('.');
            dotcase.push_str(&states[1..]);
            let d = find_good_combinations_count(counts, &dotcase, group_len, memo);

            let mut hashcase = String::with_capacity(states.len());
            hashcase.push('#');
            hashcase.push_str(&states[1..]);
            let h = find_good_combinations_count(counts, &hashcase, group_len, memo);

            // println!("{} ({:?}) -> {}", dotcase, counts, d);
            // println!("{} ({:?}) -> {}", hashcase, counts, h);
            // println!("{} {}", dot_case, hash_case);
            memo.insert((counts, states.to_string(), group_len), d+h);
            return d + h;
        }
        _ => unreachable!(),
    }
    unreachable!();
}

fn part1(input: &String) {
    let mut total_combinations = 0;

    for line in input.lines().skip(0) {
        let states: String = line.split_whitespace().next().unwrap().to_string();
        // dbg!(&states);

        let counts: Vec<usize> = line
            .split_whitespace()
            .last()
            .unwrap()
            .split(",")
            .map(|numstr| numstr.parse::<usize>().unwrap())
            .collect();

        let mut good = find_good_combinations(&counts, &states).unwrap();

        good.sort();
        good.dedup();

        total_combinations += good.iter().count();
    }
    println!("Total: {}", total_combinations);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("PART 1");
    part1(&input);

    println!("\nPART 2");
    part2(&input);
}

fn part2(input: &String) {
    let mut total_combinations = 0;
    for line in input.lines().skip(0) {
        let states: String = line.split_whitespace().next().unwrap().to_string();
        // dbg!(&states);
        // println!("Input line #{}", line_idx);

        let counts: Vec<usize> = line
            .split_whitespace()
            .last()
            .unwrap()
            .split(",")
            .map(|numstr| numstr.parse::<usize>().unwrap())
            .collect();

        /// Unfold the input `n_unfoldings` times
        fn unfold(
            n_unfoldings: usize,
            counts: &Vec<usize>,
            states: &String,
        ) -> (Vec<usize>, String) {
            let mut unfolded_counts = vec![];
            let mut unfolded_states = String::new();
            for i in 0..n_unfoldings {
                for j in counts {
                    unfolded_counts.push(*j);
                }

                unfolded_states.push_str(states.as_str());
                if i != n_unfoldings - 1 {
                    unfolded_states.push('?');
                }
            }
            // dbg!(&unfolded_counts);
            // dbg!(&unfolded_states);
            (unfolded_counts, unfolded_states)
        }
        // Unfolding
        let (unfold_counts, unfold_states) = unfold(5, &counts, &states);
        let mut memo = HashMap::new();
        let n_sols = find_good_combinations_count(&unfold_counts, &unfold_states, 0, &mut memo);
        // println!("{} -> {}", states, n_sols);

        total_combinations += n_sols;
    }
    println!("Total: {}", total_combinations);
}

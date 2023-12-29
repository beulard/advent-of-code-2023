use std::collections::{HashMap, LinkedList, VecDeque};

fn hash(s: &str) -> usize {
    s.chars()
        .filter(|c| *c != '\n')
        .fold(0, |acc, c| (acc + (c as usize)) * 17 % 256)
}

fn part1(input: &String) {
    let mut total = 0;
    for instruction in input.split(',') {
        total += hash(instruction);
    }
    println!("Total: {}", total);
}


type Lens = (String, usize);

fn part2(input: &String) {
    let steps: Vec<_> = input.split(',').collect();

    let mut boxes: HashMap<usize, VecDeque<Lens>> = HashMap::new();

    for step in steps {
        if step.contains('-') {
            // Remove step
            let label = step.split('-').next().unwrap();
            let box_id = hash(label);
            
            if let Some(lenses) = boxes.get_mut(&box_id) {
                if let Some(lens_idx) = lenses.iter().position(|x| x.0 == label) {
                    lenses.remove(lens_idx);
                }
            }

        } else if step.contains('=') {
            // Assign step
            let mut op = step.split('=');
            let label = op.next().unwrap();
            let focal = op.next().unwrap().parse::<usize>().unwrap();

            let box_id = hash(label);
            if !boxes.contains_key(&box_id) {
                boxes.insert(box_id, VecDeque::new());
            }
            let lenses = boxes.get_mut(&box_id).unwrap();
            
            if let Some(old_lens) = lenses.iter_mut().find(|x| x.0 == label) {
                old_lens.1 = focal;
            } else {
                lenses.push_back((label.to_string(), focal));
            }
        }
    }
    // dbg!(&boxes);
    println!("Focusing power: {}", get_focusing_power(&boxes));
}

fn get_focusing_power(boxes: &HashMap<usize, VecDeque<Lens>>) -> usize {
    let mut total = 0;
    for (box_id, lenses) in boxes {
        for (lens_idx, (_, focal)) in lenses.iter().enumerate() {
            total += (1 + *box_id) * (lens_idx + 1) * focal;
        }
    }
    total
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    assert_eq!(hash("HASH"), 52);
    dbg!(hash("rn"));

    part1(&input);

    part2(&input);
}

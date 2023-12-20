use std::collections::HashMap;

// Double every row where there are no galaxies
// Double every column where there are no galaxies
fn expand(input: String) -> String {
    let mut expanded_rows = String::new();
    // Rows:
    for row in input.split_inclusive("\n").rev() {
        expanded_rows.push_str(row);
        if !row.contains("#") {
            expanded_rows.push_str(row);
        }
    }

    // Columns:
    // Rotate the map and repeat
    let mut expanded_cols = String::new();
    let width = input.lines().nth(0).unwrap().len();
    // let mut rotated = vec![];
    for i in 0..width {
        let col: String = expanded_rows.chars().skip(i).step_by(width + 1).collect();
        expanded_cols.push_str((col.clone() + "\n").as_str());

        if !col.contains('#') {
            expanded_cols.push_str((col + "\n").as_str());
        }
    }
    // Rotate back
    let mut expanded = String::new();
    let width = expanded_cols.lines().nth(0).unwrap().len();
    for i in 0..width {
        let row: String = expanded_cols
            .chars()
            .rev()
            .skip(1 + i)
            .step_by(width + 1)
            .collect();
        // Mirror
        let row: String = row.chars().rev().collect();
        expanded.push_str((row + "\n").as_str());
    }

    expanded
}

type GalaxyId = usize;
type Position = (usize, usize);
// Find the galaxies in universe and store them in a hashmap of <Identifier, Position>
fn assign_numbers(universe: &String) -> HashMap<GalaxyId, Position> {
    let mut map = HashMap::new();
    universe.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                map.insert(map.len() + 1, (x, y));
            }
        })
    });
    map
}

fn get_pairs(galaxies: &HashMap<GalaxyId, Position>) -> Vec<(GalaxyId, GalaxyId)> {
    let mut pairs = vec![];
    for i in galaxies.keys() {
        for j in galaxies.keys().filter(|x| *x > i) {
            pairs.push((*i, *j));
        }
    }
    pairs
}

fn manhattan_distance(a: &Position, b: &Position) -> usize {
    let dist_x = if a.0 >= b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let dist_y = if a.1 >= b.1 { a.1 - b.1 } else { b.1 - a.1 };

    dist_x + dist_y
}

fn expanded_manhattan_distance(
    a: &Position,
    b: &Position,
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
    expansion_factor: usize,
) -> usize {
    let x_low = a.0.min(b.0);
    let x_high = a.0.max(b.0);
    let y_low = a.1.min(b.1);
    let y_high = a.1.max(b.1);
    let mut dist = 0;
    for j in y_low..=y_high {
        if empty_rows.contains(&j) {
            dist += expansion_factor;
        } else {
            dist += 1;
        }
    }

    for i in x_low..=x_high {
        if empty_columns.contains(&i) {
            dist += expansion_factor;
        } else {
            dist += 1;
        }
    }
    dist - 2 // Subtract the first step (vertical and horizontal)
}

fn find_empty_rows(input: &String) -> Vec<usize> {
    input
        .lines()
        .enumerate()
        .filter(|(_, row)| !row.contains('#'))
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>()
}

fn find_empty_columns(input: &String) -> Vec<usize> {
    // Rotate the input so we can handle columns as lines
    let mut rotated = String::new();
    let width = input.lines().nth(0).unwrap().len();
    for i in 0..width {
        let col: String = input.chars().skip(i).step_by(width + 1).collect();
        rotated.push_str((col.clone() + "\n").as_str());
    }
    find_empty_rows(&rotated)
}


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let expanded = expand(input);
    let galaxies = assign_numbers(&expanded);
    // dbg!(&galaxies);

    let pairs = get_pairs(&galaxies);
    // dbg!(&pairs, pairs.len());

    let distances = pairs.iter().fold(HashMap::new(), |mut map, pair| {
        map.insert(
            pair.clone(),
            manhattan_distance(
                galaxies.get(&pair.0).unwrap(),
                galaxies.get(&pair.1).unwrap(),
            ),
        );
        map
    });
    // dbg!(&distances);
    // dbg!(&distances.get(&(1, 7)), &distances.get(&(3, 6)), &distances.get(&(5, 9)));

    let total = distances.iter().fold(0, |dist, x| dist + x.1);
    dbg!(total);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let galaxies = assign_numbers(&input);
    let empty_rows = find_empty_rows(&input);
    // dbg!(&empty_rows);
    let empty_columns = find_empty_columns(&input);
    // dbg!(&empty_columns);

    let pairs = get_pairs(&galaxies);
    // dbg!(&pairs);

    let mut total_dist = 0;
    for (a, b) in pairs {
        let dist = expanded_manhattan_distance(
            &galaxies[&a],
            &galaxies[&b],
            &empty_rows,
            &empty_columns,
            1000000,
        );
        // println!("({} {}) -> {}", a, b, dist);
        total_dist += dist;
    }
    println!("Total dist: {}", total_dist);
}

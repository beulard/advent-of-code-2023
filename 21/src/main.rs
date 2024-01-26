use std::collections::{HashMap, HashSet};

type Position = (isize, isize);

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
}

fn parse_input(input: &str) -> (Position, HashMap<Position, Tile>) {
    let mut map = HashMap::new();
    let mut start = (0, 0);
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            map.insert((i as isize, j as isize), match c {
                '.' | 'S' => Tile::Garden,
                '#' => Tile::Rock,
                _ => panic!(),
            });
            if c == 'S' {
                start = (i as isize, j as isize);
            }
        }
    }
    (start, map)
}

fn get_reachable_tiles(n_steps: usize, current_pos: HashSet<Position>, map: &HashMap<Position, Tile>) -> HashSet<Position> {
    if n_steps == 0 {
        return current_pos;
    }

    let mut reachable = HashSet::new();


    for pos in current_pos {
        // Try up
        for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = (pos.0 + delta.0, pos.1 + delta.1);
            if let Some(tile) = map.get(&next) {
                if *tile == Tile::Garden {
                    reachable.insert(next);
                }
            }
        }
    }

    get_reachable_tiles(n_steps-1, reachable, map)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (start, tiles) = parse_input(&input);
    // dbg!(&start, &tiles);

    let r = get_reachable_tiles(64, HashSet::from([start]), &tiles);
    // dbg!(&r);

    dbg!(r.len());
}

use std::collections::{HashMap, HashSet};

type Position = (isize, isize);

#[derive(Debug)]
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
        if map.contains_key(&(pos.0, pos.1 - 1)).then(f) {
            
        }
    }

    reachable
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (start, tiles) = parse_input(&input);
    dbg!(&start, &tiles);
}

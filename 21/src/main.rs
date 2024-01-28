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

fn part1() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (start, tiles) = parse_input(&input);
    // dbg!(&start, &tiles);

    let r = get_reachable_tiles(50, HashSet::from([start]), &tiles);
    // dbg!(&r);

    dbg!(r.len());
}

fn get_reachable_tiles_p2(n_steps: usize, current_pos: HashSet<Position>, map: &HashMap<Position, Tile>, width: usize, height: usize, memo: &mut HashMap<(usize, Position), HashSet<Position>>) -> HashSet<Position> {
    if n_steps == 0 {
        return current_pos;
    }

    let mut reachable = HashSet::new();


    for pos in current_pos {
        let rem = ((pos.0).rem_euclid(width as isize), (pos.1).rem_euclid(height as isize));
        let offset = (pos.0 / (width as isize), pos.1 / (height as isize));
        println!("({} {}) -> ({} {}) ({} {})", pos.0, pos.1, rem.0, rem.1, offset.0, offset.1);

        if let Some(soln) = memo.get(&(n_steps, rem)) {
            for x in soln {
                reachable.insert((x.0 + offset.0 * (width as isize), x.1 + offset.1 * (height as isize)));
            }
            continue;
            panic!();
        }

        for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = ((pos.0 + delta.0).rem_euclid(width as isize), (pos.1 + delta.1).rem_euclid(height as isize));
            // dbg!((pos.0 + delta.0, pos.1 + delta.1));
            // dbg!(next);
            if let Some(tile) = map.get(&next) {
                if *tile == Tile::Garden {
                    if reachable.contains(&(pos.0 + delta.0, pos.1 + delta.1)) {
                        // println!("QWE");
                    }
                    reachable.insert((pos.0 + delta.0, pos.1 + delta.1));
                    memo.entry((n_steps, pos)).or_insert(HashSet::new()).insert((next.0, next.1));
                }
            }
        }
    }

    get_reachable_tiles_p2(n_steps-1, reachable, map, width, height, memo)
}

fn part2() {
    
    let input = std::fs::read_to_string("input.txt").unwrap();
    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().len();

    let (start, tiles) = parse_input(&input);
    // dbg!(&start, &tiles);

    let mut memo = HashMap::new();
    let r = get_reachable_tiles_p2(50, HashSet::from([start]), &tiles, width, height, &mut memo);
    // dbg!(&memo);
    // dbg!(&r);

    dbg!(r.len());
}

fn main() {
    part1();
    part2();
}
use std::{
    sync::mpsc,
    thread,
};

#[derive(Clone)]
struct Map {
    data: Vec<char>,
    width: usize,
    height: usize,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

type Position = (usize, usize);
impl Map {
    fn new(input: &str) -> Map {
        let mut map = Map {
            data: vec![],
            width: 0,
            height: 0,
        };
        for line in input.lines() {
            for c in line.chars() {
                map.data.push(c);
            }
        }
        map.width = input.lines().nth(0).unwrap().len();
        map.height = input.lines().count();
        map
    }
    fn get(&self, pos: Position) -> char {
        self.data[pos.0 + pos.1 * self.width]
    }

    // Returns the list of tiles energized by this beam and any splits
    fn trace_beam(
        &self,
        start_pos: Position,
        start_direction: Direction,
        visited: &mut Vec<(Position, Direction)>,
    ) -> Vec<Position> {
        let mut energized = vec![];
        let mut position = start_pos;
        let mut direction = start_direction;
        use Direction::*;
        loop {
            visited.push((position, direction));

            // println!("{:?} {:?}", position, direction);
            // Add this tile to the energized ones
            energized.push(position);
            // Determine where to go based on the current tile and direction
            match (self.get(position), direction) {
                ('.', _) => {
                    // Do nothing
                }
                // | splitter
                ('|', Up) | ('|', Down) => {
                    // Do nothing
                }
                ('|', Left) | ('|', Right) => {
                    // Shoot beams up and down from here
                    energized.extend(self.trace_beam(position, Up, visited));
                    energized.extend(self.trace_beam(position, Down, visited));
                    // End this beam
                    break;
                }
                // - splitter
                ('-', Up) | ('-', Down) => {
                    // Shoot beams left and right from here
                    energized.extend(self.trace_beam(position, Left, visited));
                    energized.extend(self.trace_beam(position, Right, visited));
                    // End this beam
                    break;
                }
                ('-', Left) | ('-', Right) => {
                    // Do nothing
                }
                // Direction change
                ('\\', Right) => direction = Down,
                ('\\', Left) => direction = Up,
                ('\\', Down) => direction = Right,
                ('\\', Up) => direction = Left,
                ('/', Right) => direction = Up,
                ('/', Left) => direction = Down,
                ('/', Down) => direction = Left,
                ('/', Up) => direction = Right,
                _ => panic!(),
            }

            // Check if we have reached an edge
            match (position, direction) {
                ((0, _), Left) => {
                    break;
                }
                ((_, 0), Up) => {
                    break;
                }
                ((_, j), Down) if j == self.height - 1 => {
                    break;
                }
                ((i, _), Right) if i == self.width - 1 => {
                    break;
                }
                _ => (),
            }

            // Keep going
            match direction {
                Left => position.0 -= 1,
                Right => position.0 += 1,
                Up => position.1 -= 1,
                Down => position.1 += 1,
            }

            if visited.contains(&(position, direction)) {
                break;
            }
        }
        energized
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::new(&input);
    // Pretty print map
    for j in 0..map.height {
        println!(
            "{}",
            map.data[j * map.width..(j + 1) * map.width]
                .iter()
                .collect::<String>()
        );
    }
    // Compute the result of tracing the beam
    let mut energized = map.trace_beam((0, 0), Direction::Right, &mut vec![]);

    // dbg!(&energized);
    for j in 0..map.height {
        for i in 0..map.width {
            if energized.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    energized.sort();
    energized.dedup();
    println!("Number of energized tiles: {}", energized.len());

    // PART 2:
    let mut max_energized = 0;
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    for j in 0..map.height {
        // Try every starting point from the left
        let tx1 = tx.clone();
        let map1 = map.clone();
        handles.push(thread::spawn(move || {
            let mut energized = map1.trace_beam((0, j), Direction::Right, &mut vec![]);
            energized.sort();
            energized.dedup();
            tx1.send(energized.len()).unwrap();
        }));

        // Try every starting point from the right
        let tx1 = tx.clone();
        let map1 = map.clone();
        handles.push(thread::spawn(move || {
            let mut energized = map1.trace_beam((map.width - 1, j), Direction::Left, &mut vec![]);
            energized.sort();
            energized.dedup();
            tx1.send(energized.len()).unwrap();
        }));
    }

    for i in 0..map.width {
        // Try every starting point from the top
        let tx1 = tx.clone();
        let map1 = map.clone();
        handles.push(thread::spawn(move || {
            let mut energized = map1.trace_beam((i, 0), Direction::Down, &mut vec![]);
            energized.sort();
            energized.dedup();
            tx1.send(energized.len()).unwrap();
        }));

        // Try every starting point from the bottom
        let tx1 = tx.clone();
        let map1 = map.clone();
        handles.push(thread::spawn(move || {
            let mut energized = map1.trace_beam((i, map.height - 1), Direction::Up, &mut vec![]);
            energized.sort();
            energized.dedup();
            tx1.send(energized.len()).unwrap();
        }));
    }
    // Unused transmitter
    drop(tx);
    for received in rx {
        if received > max_energized {
            max_energized = received;
        }
    }

    println!("Max: {}", max_energized);
}

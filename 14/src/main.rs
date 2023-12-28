use std::{collections::HashMap, fmt};

// Move rocks from this line to the line above if possible
// Returns the upper and lower line
fn roll_up(upper: &String, lower: &String) -> (String, String) {
    let mut out_upper = String::new();
    let mut out_lower = String::new();
    for (i, (up, down)) in upper.chars().zip(lower.chars()).enumerate() {
        // println!("{} -> {} {}", i, up, down);
        match (up, down) {
            ('.', 'O') => {
                // Rock in lower row moves up
                out_upper.push('O');
                out_lower.push('.'); // Free up lower row
            }
            (u, l) => {
                // Anything else -> lower row cannot move up
                out_upper.push(u);
                out_lower.push(l);
            }
            _ => {
                panic!();
            }
        }
    }
    (out_upper, out_lower)
}

fn roll_north(map: &mut [String]) -> &[String] {
    for i in (1..map.len()).rev() {
        for j in 0..i {
            // println!("{}, {}", i, j);
            // println!("{:?} {:?}", map[j], map[j+1]);
            let (upper, lower) = roll_up(&map[j], &map[j + 1]);
            map[j] = upper;
            map[j + 1] = lower;
            // println!("{:?} {:?}", out[j], out[j+1]);
        }
    }
    map
}

fn get_load(rolled: &[String]) -> usize {
    let mut load = 0;
    for (i, line) in rolled.iter().enumerate() {
        load += (rolled.len() - i) * line.chars().filter(|c| *c == 'O').count();
    }
    load
}

fn rotate_clockwise(map: &[String]) -> Vec<String> {
    let mut out: Vec<String> = vec![String::new(); map[0].len()];
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            // First line becomes last column
            // Last line becomes first column
            out[i].push(map[map.len() - 1 - j].chars().nth(i).unwrap());
        }
    }
    out
}

fn part1() {
    let mut input: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split_whitespace()
        .map(|l| l.to_string())
        .collect();

    let rolled = roll_north(&mut input);

    println!("Load: {}", get_load(&rolled));
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<char>,
    width: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for j in 0..self.data.len() / self.width {
            for i in 0..self.width {
                write!(f, "{}", self.get(i, j))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Map {
    fn new(input: &String) -> Map {
        let mut map = Map {
            data: vec![],
            width: input.split_whitespace().nth(0).unwrap().len(),
        };
        input.split_whitespace().for_each(|line| {
            line.chars().for_each(|c| {
                map.data.push(c);
            })
        });
        map
    }
    fn get(&self, i: usize, j: usize) -> char {
        self.data[i + j * self.width]
    }
    fn set(&mut self, i: usize, j: usize, val: char) {
        self.data[i + j * self.width] = val;
    }

    fn roll_north(&mut self) {
        for j in 1..self.data.len() / self.width {
            for i in 0..self.width {
                if self.get(i, j) == 'O' {
                    // Roll rock up to the highest possible (# or O or edge blocks)
                    // Find the closest O or # up from this rock
                    let mut k = j;

                    while k != 0 && self.get(i, k - 1) == '.' {
                        k -= 1;
                    }
                    self.set(i, j, '.');
                    self.set(i, k, 'O');
                }
            }
        }
    }

    fn roll_west(&mut self) {
        for j in 0..self.data.len() / self.width {
            for i in 1..self.width {
                if self.get(i, j) == 'O' {
                    // Roll rock to the leftmost possible (# or O or edge blocks)
                    // Find the closest O or # up from this rock
                    let mut k = i;

                    while k != 0 && self.get(k - 1, j) == '.' {
                        k -= 1;
                    }
                    self.set(i, j, '.');
                    self.set(k, j, 'O');
                }
            }
        }
    }

    fn roll_south(&mut self) {
        let height = self.data.len() / self.width;
        for j in (0..self.data.len() / self.width - 1).rev() {
            for i in 0..self.width {
                if self.get(i, j) == 'O' {
                    // Roll rock down to the southmost possible (# or O or edge blocks)
                    // Find the closest O or # up from this rock
                    let mut k = j;

                    while k != height - 1 && self.get(i, k + 1) == '.' {
                        k += 1;
                    }
                    self.set(i, j, '.');
                    self.set(i, k, 'O');
                }
            }
        }
    }
    fn roll_east(&mut self) {
        for j in 0..self.data.len() / self.width {
            for i in (0..self.width - 1).rev() {
                if self.get(i, j) == 'O' {
                    // Roll rock right to the eastmost possible (# or O or edge blocks)
                    // Find the closest O or # up from this rock
                    // println!("rock at {} {}", i, j);
                    let mut k = i;
                    while k != self.width - 1 && self.get(k + 1, j) == '.' {
                        // println!("k {} val {}", k, self.get(k+1, j));
                        k += 1;
                    }
                    // println!("move to {} {}", k, j);
                    // println!();
                    self.set(i, j, '.');
                    self.set(k, j, 'O');
                }
            }
        }
    }
    fn cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn get_load(&self) -> usize {
        let mut load = 0;
        let height = self.data.len() / self.width;
        for j in 0..height {
            for i in 0..self.width {
                if self.get(i, j) == 'O' {
                    load += height - j;
                }
            }
        }
        load
    }
}

fn part2() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut map = Map::new(&input);
    println!("{}", map);

    // Keep track of previous maps so we have a cache of previous cycles
    // If we find a map in the list of previous maps, then we can use the list
    // to shortcut the cycle calculation.
    let n_cycles = 1000000000;
    let mut prev_maps: Vec<Vec<char>> = vec![];
    for i in 0..n_cycles {
        map.cycle();
        if let Some(v) = prev_maps.iter().position(|data| *data == map.data) {
            // We have a circular pattern starting at v which repeats after i-v cycles
            // The map at n_cycles - x is also equal to the map at v
            // We just have to compute x
            // println!("i={}, same as prev map {}", i, v);
            // println!("{}", (n_cycles - v) % (i - v));
            let final_cycle_idx = v + (n_cycles - v) % (i - v) - 1;
            // println!("final: {}", final_cycle_idx);
            map.data = prev_maps[final_cycle_idx].clone();
            break;
        } else {
            prev_maps.push(map.data.clone());
        }
    }
    println!("{}", map);
    println!("Load: {}", map.get_load());

}

fn main() {
    part1();

    part2();
}

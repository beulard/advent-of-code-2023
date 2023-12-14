use std::ops::Index;

#[derive(Debug)]
struct Tiles {
    data: Vec<char>,
    width: usize,
}

impl Tiles {
    fn get(&self, x: usize, y: usize) -> char {
        self.data[x + y * self.width]
    }

    fn get_safe(&self, x: i32, y: i32) -> Option<char> {
        if x >= self.width as i32 {
            None
        } else if y >= (self.data.len() / self.width) as i32 {
            None
        } else if y < 0 {
            None
        } else if x < 0 {
            None
        } else {
            Some(self.get(x as usize, y as usize))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
    Here,
}

impl Direction {}

fn get_next_step(x: i32, y: i32, provenance: Direction, map: &Tiles) -> Option<(i32, i32, Direction)> {
    use Direction::*;
    // dbg!(x, y);
    let cur = map.get_safe(x, y).unwrap();
    let next = match (cur, provenance) {
        ('|', N) => S,
        ('|', S) => N,
        ('J', N) => W,
        ('J', W) => N,
        ('L', N) => E,
        ('L', E) => N,
        ('F', E) => S,
        ('F', S) => E,
        ('-', W) => E,
        ('-', E) => W,
        ('7', W) => S,
        ('7', S) => W,
        _ => return None,
    };

    let do_not_check = match next {
        E => W,
        N => S,
        W => E,
        S => N,
        _ => return None,
    };

    let (x, y) = match next {
        N => (x, y-1),
        S => (x, y+1),
        W => (x-1, y),
        E => (x+1, y),
        _ => return None,
    };
    // dbg!(next);

    return Some((x, y, do_not_check));
}

fn get_first_step(x: i32, y: i32, map: &Tiles) -> (i32, i32, Direction) {
    if let Some(v) = get_next_step(x, y+1, Direction::N, map) {
        v
    } else if let Some(v) = get_next_step(x, y-1, Direction::S, map) {
        v
    } else if let Some(v) = get_next_step(x-1, y, Direction::E, map) {
        v
    } else if let Some(v) = get_next_step(x+1, y, Direction::W, map) {
        v
    } else {
        panic!();
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Tiles {
        data: input.lines().map(|line| line.chars()).flatten().collect(),
        width: input.lines().next().unwrap().len(),
    };

    // Start from s
    let start_pos = map.data.iter().position(|c| *c == 'S').unwrap();
    let start_x = (start_pos % map.width) as i32;
    let start_y = (start_pos / map.width) as i32;
    dbg!(start_x, start_y);
    // Find the first connected tile around S
    // Up

    let (mut x, mut y, mut do_not_check) = get_first_step(start_x, start_y, &map);
    dbg!(x, y, do_not_check);

    let mut step = 1;
    loop {
        (x, y, do_not_check) = get_next_step(x, y, do_not_check, &map).unwrap();
        
        step += 1;
        
        if step % 50000 == 0 {
            println!("Step {}", step);
        }
        if map.get_safe(x, y).unwrap() == 'S' {
            break;
        }

        // break;
    }

    dbg!(x, y, do_not_check);
    dbg!((step + 1) / 2);
}

use std::collections::HashMap;

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
}

/// Returns:
/// + x: Next x position (after step)
/// + y: Next y position (after step)
/// + direction: Direction of the next step
fn get_next_step(
    x: i32,
    y: i32,
    direction: Direction,
    map: &Tiles,
) -> Option<(i32, i32, Direction)> {
    use Direction::*;
    // dbg!(x, y);

    let (x_next, y_next) = match direction {
        N => (x, y - 1),
        S => (x, y + 1),
        W => (x - 1, y),
        E => (x + 1, y),
    };

    let next_tile = map.get_safe(x_next, y_next).unwrap();
    let next_direction = match (next_tile, direction) {
        ('|', N) => N,
        ('|', S) => S,
        ('J', S) => W,
        ('J', E) => N,
        ('L', W) => N,
        ('L', S) => E,
        ('F', N) => E,
        ('F', W) => S,
        ('-', W) => W,
        ('-', E) => E,
        ('7', E) => S,
        ('7', N) => W,
        _ => return None,
    };

    // dbg!(next);

    return Some((x_next, y_next, next_direction));
}

// Try all directions and pick the first valid one
fn get_first_step(x: i32, y: i32, map: &Tiles) -> (Direction, (i32, i32, Direction)) {
    use Direction::*;
    for first_direction in [N, S, E, W] {
        if let Some(v) = get_next_step(x, y, first_direction, map) {
            return (first_direction, v);
        }
    }
    panic!();
}

fn get_starting_pos_type(x: i32, y: i32, map: &Tiles) -> char {
    use Direction::*;
    let mut possible_directions = vec![];
    for first_direction in [N, S, E, W] {
        if let Some(_) = get_next_step(x, y, first_direction, map) {
            possible_directions.push(first_direction);
        }
    }
    dbg!(&possible_directions);
    match &possible_directions[0..=1] {
        [N, E] => 'L',
        [N, W] => 'J',
        [S, E] => 'F',
        [S, W] => '7',
        _ => panic!(),
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Tiles {
        data: input.lines().map(|line| line.chars()).flatten().collect(),
        width: input.lines().next().unwrap().len(),
    };

    println!("PART 1");
    // Start from s
    let start_pos = map.data.iter().position(|c| *c == 'S').unwrap();
    let start_x = (start_pos % map.width) as i32;
    let start_y = (start_pos / map.width) as i32;
    // dbg!(start_x, start_y);

    let (first_direction, (mut x, mut y, mut direction)) = get_first_step(start_x, start_y, &map);
    // dbg!(x, y, do_not_check);

    // Store the steps in a vector for part 2
    let mut steps: Vec<Direction> = vec![first_direction];
    let mut path_tiles: HashMap<(i32, i32), (char, Direction)> = HashMap::new();
    path_tiles.insert(
        (start_x, start_y),
        (map.get_safe(start_x, start_y).unwrap(), first_direction),
    );
    loop {
        // dbg!(x, y, direction);
        steps.push(direction);
        path_tiles.insert((x, y), (map.get_safe(x, y).unwrap(), direction));
        match get_next_step(x, y, direction, &map) {
            Some(next) => {
                (x, y, direction) = next;
            }
            None => break,
        }
    }
    // dbg!(steps);
    let start_tile = dbg!(get_starting_pos_type(start_x, start_y, &map));

    // dbg!(x, y, do_not_check);
    println!("Maximum distance from the start: {}", (steps.len() + 1) / 2);

    println!("Path length {}", steps.len());

    println!("\nPART 2");
    // dbg!(&path_tiles);

    let mut n_enclosed_tiles = 0;

    let mut tiles_in: Vec<(i32, i32)> = vec![];

    // Replace the S by its actual value to simplify the enclosed area calculation logic
    path_tiles.insert((start_x, start_y), (start_tile, first_direction));

    for j in 0..(map.data.len() / map.width) as i32 {
        for i in 0..map.width as i32 {
            // Ignore if part of the path
            if path_tiles.contains_key(&(i, j)) {
                continue;
            }
            // print!("{} {}:", i, j);

            // The logic is the following:
            // If we have an odd number of path crossings in each cardinal direction,
            // then this tile is enclosed in the loop.
            // To quantify crossings, consider casting a ray from the tile to the right edge
            // of the map.
            // + If we encounter a '|', we count this as 1.
            // + If we encounter a 'F' or 'J', we count the tile as 0.5 (a F followed by a J counts as a |).
            // + If we encounter a '7' or 'L', we count the tile as -0.5 (a 7 followed by a L counts as a |,
            //   but a L followed by a J counts as nothing: the path does not cross but turns around).
            // + If we encounter a '-', we count the tile as 0 (the path did not cross or turn around).
            // And similarly for all 4 directions.
            // (In the code we multiply these values by 2 and then divide the sum by 2 at the end to avoid floats)

            // Cast horizontal ray from the left
            // Count straight wall sections of the path
            let mut num_walls_left = 0;
            for x in 0..i {
                match path_tiles.get(&(x, j)) {
                    Some(tile) => match tile.0 {
                        '|' => num_walls_left += 2,
                        'J' | 'F' => num_walls_left += 1,
                        '7' | 'L' => num_walls_left -= 1,
                        '-' => (),
                        _ => panic!(),
                    },
                    None => (),
                }
            }
            num_walls_left /= 2;
            // print!(" L({})", num_walls_left);

            // Repeat on right side
            let mut num_walls_right = 0;
            for x in i + 1..map.width as i32 {
                match path_tiles.get(&(x, j)) {
                    Some(tile) => match tile.0 {
                        '|' => num_walls_right += 2,
                        'J' | 'F' => num_walls_right += 1,
                        '7' | 'L' => num_walls_right -= 1,
                        '-' => (),
                        _ => panic!(),
                    },
                    None => (),
                }
            }
            num_walls_right /= 2;
            // print!(" R({})", num_walls_right);

            // Cast vertical ray from the top
            // Count horizontal/corner sections of the path

            let mut num_walls_top = 0;
            for y in 0..j {
                match path_tiles.get(&(i, y)) {
                    Some(tile) => match tile.0 {
                        '-' => num_walls_top += 2,
                        'J' | 'F' => num_walls_top += 1,
                        '7' | 'L' => num_walls_top -= 1,
                        '|' => (),
                        _ => panic!(),
                    },
                    None => (),
                }
            }
            num_walls_top /= 2;
            // print!(" T({})", num_walls_top);

            // Repeat on bottom
            let mut num_walls_bottom = 0;
            for y in j + 1..(map.data.len() / map.width) as i32 {
                match path_tiles.get(&(i, y)) {
                    Some(tile) => match tile.0 {
                        '-' => num_walls_bottom += 2,
                        'J' | 'F' => num_walls_bottom -= 1,
                        '7' | 'L' => num_walls_bottom += 1,
                        '|' => (),
                        _ => panic!(),
                    },
                    None => (),
                }
            }
            num_walls_bottom /= 2;
            // print!(" B({})", num_walls_bottom);
            // println!();

            if num_walls_bottom % 2 != 0
                && num_walls_right % 2 != 0
                && num_walls_left % 2 != 0
                && num_walls_top % 2 != 0
            {
                tiles_in.push((i, j));
                n_enclosed_tiles += 1;
            }
        }
    }

    // Draw a diagram like in the examples
    for j in 0..(map.data.len() / map.width) as i32 {
        for i in 0..map.width as i32 {
            if path_tiles.contains_key(&(i, j)) {
                print!("{}", path_tiles[&(i, j)].0);
            } else if tiles_in.contains(&(i, j)) {
                print!("I");
            } else {
                print!("O");
            }
        }
        println!();
    }

    println!("\nEnclosed tiles: {}", n_enclosed_tiles);
}

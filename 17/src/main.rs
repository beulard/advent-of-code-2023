use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

#[derive(Debug)]
struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

type Position = (usize, usize);

impl Map {
    fn new(input: &str) -> Self {
        let mut map = Self {
            data: vec![],
            width: 0,
            height: 0,
        };
        input.lines().for_each(|line| {
            line.chars().for_each(|c| {
                map.data.push(c.to_string().parse::<u8>().unwrap());
            })
        });
        map.width = input.lines().nth(0).unwrap().len();
        map.height = input.lines().count();
        map
    }
    fn get(&self, position: Position) -> u8 {
        self.data[position.0 + position.1 * self.width]
    }
}

fn draw_path(map: &Map, path: &Vec<State>) {
    // Draw path
    use Direction::*;
    for j in 0..map.height {
        for i in 0..map.width {
            if (i, j) == (0, 0) {
                print!("X");
            } else if let Some(x) = path.iter().find(|x| x.pos == (i, j)) {
                print!(
                    "{}",
                    match x.direction {
                        Left => "<",
                        Right => ">",
                        Down => "v",
                        Up => "^",
                    }
                )
            } else {
                print!("{}", map.get((i, j)));
            }
        }
        println!();
    }
}

fn is_opposite(a: Direction, b: Direction) -> bool {
    use Direction::*;
    return match (a, b) {
        (Left, Right) => true,
        (Right, Left) => true,
        (Up, Down) => true,
        (Down, Up) => true,
        _ => false,
    };
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct State {
    pos: Position,
    direction: Direction,
    consecutive: usize,
}

fn get_neighbors(pos: Position, map: &Map, forbidden_directions: Vec<Direction>) -> Vec<Position> {
    let mut neighbors = vec![];
    use Direction::*;
    if pos.0 > 0 && !forbidden_directions.contains(&Left) {
        neighbors.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 && !forbidden_directions.contains(&Up) {
        neighbors.push((pos.0, pos.1 - 1));
    }
    if pos.0 < map.width - 1 && !forbidden_directions.contains(&Right) {
        neighbors.push((pos.0 + 1, pos.1));
    }
    if pos.1 < map.height - 1 && !forbidden_directions.contains(&Down) {
        neighbors.push((pos.0, pos.1 + 1));
    }
    neighbors
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
fn get_direction(src: &Position, tgt: &Position) -> Direction {
    use Direction::*;
    if src.0 > tgt.0 {
        return Left;
    } else if src.0 < tgt.0 {
        return Right;
    } else if src.1 > tgt.1 {
        return Up;
    } else if src.1 < tgt.1 {
        return Down;
    } else {
        unreachable!();
    }
}

/// MinScoredState: implementation of priority queue with a BinaryHeap of States
/// Because BinaryHeap orders elements in ascending order, we specify Ord for MinScoredState.
#[derive(Debug)]
struct MinScoredState(u32, State);

impl PartialEq for MinScoredState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for MinScoredState {}

impl PartialOrd for MinScoredState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.0).partial_cmp(&Reverse(other.0))
    }
}

impl Ord for MinScoredState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.0).cmp(&Reverse(other.0))
    }
}

/// Dijkstra shortest path for crucibles (at most 3 steps in the same direction, cannot go backward).
/// Uses "stateful" steps in the 2D grid, with each having a direction and remembering
/// the number of consecutive steps in that direction.
fn dijkstra(map: &Map, start: Position, end: Position) -> (u32, Vec<State>) {
    let mut score: HashMap<State, u32> = HashMap::new();
    let mut prev: HashMap<State, State> = HashMap::new();
    let mut pq: BinaryHeap<MinScoredState> = BinaryHeap::new();

    score.insert(
        State {
            pos: start,
            direction: Direction::Up,
            consecutive: 1,
        },
        0,
    );

    pq.push(MinScoredState(
        0,
        State {
            pos: start,
            direction: Direction::Up,
            consecutive: 0,
        },
    ));

    while let Some(MinScoredState(dist, node)) = pq.pop() {
        if node.pos == end {
            let mut cur = node;
            let mut path = vec![cur];
            while let Some(p) = prev.get(&cur) {
                path.push(*p);
                cur = *p;
            }
            path.reverse();

            return (dist, path);
        }
        let neighbors = get_neighbors(node.pos, map, vec![]);

        for tgt in neighbors {
            let dir = get_direction(&node.pos, &tgt);
            let mut consec = 1;
            // Do not apply to first step
            if node.pos != (0, 0) {
                // Cannot go backward
                if is_opposite(dir, node.direction) {
                    continue;
                }
                consec = if dir == node.direction {
                    node.consecutive + 1
                } else {
                    1
                };

                // Cannot go more than 3 steps in the same direction
                if consec > 3 {
                    continue;
                }
            }

            let state = State {
                pos: tgt,
                consecutive: consec,
                direction: dir,
            };
            let alt = dist + map.get(tgt) as u32;

            // Compare new distance with previously stored one
            // If the previous value was larger than the new one, or if no value was stored,
            // update the queue and path.
            if alt < *score.get(&state).unwrap_or(&u32::MAX) {
                score.insert(state, alt);
                prev.insert(state, node);
                pq.push(MinScoredState(alt, state));
            }
        }
    }
    unreachable!();
}

/// Dijkstra shortest path for ultra crucibles
fn ultra_dijkstra(map: &Map, start: Position, end: Position) -> (u32, Vec<State>) {
    let mut score: HashMap<State, u32> = HashMap::new();
    let mut prev: HashMap<State, State> = HashMap::new();
    let mut pq: BinaryHeap<MinScoredState> = BinaryHeap::new();

    score.insert(
        State {
            pos: start,
            direction: Direction::Up,
            consecutive: 1,
        },
        0,
    );

    pq.push(MinScoredState(
        0,
        State {
            pos: start,
            direction: Direction::Up,
            consecutive: 0,
        },
    ));

    while let Some(MinScoredState(dist, node)) = pq.pop() {
        // End condition must take into account minimum
        // consecutive steps for ultra crucible
        if node.pos == end && node.consecutive > 3 {
            let mut cur = node;
            let mut path = vec![cur];
            while let Some(p) = prev.get(&cur) {
                path.push(*p);
                cur = *p;
            }
            path.reverse();

            return (dist, path);
        }
        let neighbors = get_neighbors(node.pos, map, vec![]);

        for tgt in neighbors {
            let dir = get_direction(&node.pos, &tgt);
            let mut consec = 1;
            // Do not apply to first step
            if node.pos != (0, 0) {
                // Cannot go backward
                if is_opposite(dir, node.direction) {
                    continue;
                }
                // Have to keep going straight if consec < 4
                if node.consecutive < 4 && dir != node.direction {
                    continue;
                }
                consec = if dir == node.direction {
                    node.consecutive + 1
                } else {
                    1
                };

                // Cannot go in this direction if > 10 steps in the same direction
                if consec > 10 {
                    continue;
                }
            }

            let state = State {
                pos: tgt,
                consecutive: consec,
                direction: dir,
            };
            let alt = dist + map.get(tgt) as u32;

            // Compare new distance with previously stored one
            // If the previous value was larger than the new one, or if no value was stored,
            // update the queue and path.
            if alt < *score.get(&state).unwrap_or(&u32::MAX) {
                score.insert(state, alt);
                prev.insert(state, node);
                pq.push(MinScoredState(alt, state));
            }
        }
    }
    unreachable!();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::new(&input);

    let (cost, path) = dijkstra(
        &map,
        (0, 0),
        (map.width - 1, map.height - 1),
        // (4, 0),
    );
    // dbg!(&path, cost);

    draw_path(&map, &path);
    println!("Minimized cost: {}", cost);

    // Part 2

    let (cost, path) = ultra_dijkstra(&map, (0, 0), (map.width - 1, map.height - 1));

    draw_path(&map, &path);
    println!("Minimized cost for ultra crucible: {}", cost);
}

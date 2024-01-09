use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

type Position = (i32, i32);

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut path: Vec<Position> = vec![];
    path.push((0, 0));

    for instruction in input.lines() {
        let mut groups = instruction.split_whitespace();
        // dbg!(&groups);
        let direction = groups.next().unwrap();
        let distance = groups.next().unwrap().parse::<usize>().unwrap();

        match direction {
            "R" => {
                // dbg!(&path);
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0 + 1, last.1));
                }
            }
            "L" => {
                // dbg!(&path);
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0 - 1, last.1));
                }
            }
            "U" => {
                // dbg!(&path);
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0, last.1 - 1));
                }
            }
            "D" => {
                // dbg!(&path);
                for _ in 0..distance {
                    let last = path.last().unwrap();
                    path.push((last.0, last.1 + 1));
                }
            }
            _ => panic!(),
        }
    }
    // dbg!(&path);

    // show path
    let max_x = path.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = path.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_x = path.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = path.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    // for j in min_y..=max_y {
    //     for i in min_x..=max_x {
    //         if path.contains(&(i, j)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    // println!("\n");
    // fill path
    let filled_count = get_filled_count(&path, min_x, max_x, min_y, max_y);
    println!("Filled tiles: {}", filled_count);

    // Part 2
    // Reinterpret input: color codes are hex direction + distance.
    // Efficiency:
    // Instead of considering each step in the path, consider edges of the pool
    // Define edge intersection method so we can more quickly determine if a point is inside the polygon
    let mut edges: Vec<Edge> = vec![];
    let mut cur = (0, 0);
    for instruction in input.lines() {
        let mut groups = instruction.split_whitespace();
        let _ = groups.next().unwrap();
        let _ = groups.next().unwrap().parse::<i32>().unwrap();
        let color = groups
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(')');
        let dist_hex = &color[..5];
        let dir_hex = color.chars().nth(5).unwrap();

        let distance = i32::from_str_radix(dist_hex, 16).unwrap();

        match dir_hex {
            '0' => {
                // 0 = R
                edges.push(Edge::new(cur.0, cur.0 + distance, cur.1, cur.1));
                cur.0 = cur.0 + distance;
            }
            '1' => {
                // 1 = D
                edges.push(Edge::new(cur.0, cur.0, cur.1, cur.1 + distance));
                cur.1 = cur.1 + distance;
            }
            '2' => {
                // 2 = L
                edges.push(Edge::new(cur.0, cur.0 - distance, cur.1, cur.1));
                cur.0 = cur.0 - distance;
            }
            '3' => {
                // 3 = U
                edges.push(Edge::new(cur.0, cur.0, cur.1, cur.1 - distance));
                cur.1 = cur.1 - distance;
            }
            _ => panic!(),
        }
    }

    let min_y = edges
        .iter()
        .map(|x| x.y0.min(x.y1))
        .min_by_key(|x| *x)
        .unwrap();
    let max_y = edges
        .iter()
        .map(|x| x.y0.max(x.y1))
        .max_by_key(|x| *x)
        .unwrap();

    // Sort from left to right to simplify algorithm
    edges.sort_by(|a, b| a.x0.cmp(&b.x0));

    let mut area_inside: u64 = 0;
    let mut previous_line_inside: HashSet<RangeInclusive<i32>> = HashSet::new();
    for j in min_y..=max_y {
        let mut this_line_inside = HashSet::new();
        // Cast a ray from the left and find all intersections with vertical edges
        let ray = ((1, 0), j);
        let mut inside = false;
        let mut start = 0;
        for edge in &edges {
            match edge.intersects(ray) {
                Intersection::Some(x) => {
                    if !inside {
                        start = x;
                        inside = true;
                    } else {
                        this_line_inside.insert(start..=x);
                        inside = false;
                    }
                }
                Intersection::Collinear(x0, x1) => {
                    if inside {
                        // We are inside and we reach a collinear
                        // Determine if we are inside or outside after the segment
                        if let Some(_) = previous_line_inside.iter().find(|x| x.contains(&(x1 + 1)))
                        {
                            // We are still INSIDE after the collinear
                            // DO NOTHING
                        } else {
                            // We are OUTSIDE after the collinear
                            inside = false;
                            this_line_inside.insert(start..=x1);
                        }
                    } else {
                        // We are outside and we reach a collinear
                        // Determine if the tile after the segment is still inside or outside
                        if j == min_y {
                            // Outside
                            this_line_inside.insert(x0..=x1);
                        } else if let Some(_) =
                            previous_line_inside.iter().find(|x| x.contains(&(x1 + 1)))
                        {
                            // We are still INSIDE after the collinear
                            inside = true;
                            start = x0;
                        } else {
                            // We are OUTSIDE after the collinear
                            this_line_inside.insert(x0..=x1);
                        }
                    }
                }
                Intersection::None => {}
            }
        }

        for inside in &this_line_inside {
            area_inside += (inside.end() - inside.start() + 1) as u64;
        }

        previous_line_inside = this_line_inside;

        // Draw result
        // for i in min_x..=max_x {
        //     if let Some(_) = this_line_inside.iter().find(|x| x.contains(&i)) {
        //         print!("#");
        //     } else {
        //         print!(".");
        //     }
        // }
        // println!();
    }
    println!("Area inside: {}", area_inside);
}

#[derive(Debug)]
struct Edge {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
}

enum Intersection {
    Some(i32),
    None,
    Collinear(i32, i32),
}

impl Edge {
    fn new(x0: i32, x1: i32, y0: i32, y1: i32) -> Self {
        Self { x0, x1, y0, y1 }
    }

    /// ray is ((direction_x, directon_y), offset_x/y)
    /// returns the position of intersection, or None
    fn intersects(&self, ray: ((i32, i32), i32)) -> Intersection {
        match ray.0 {
            (1, 0) | (-1, 0) => {
                // dbg!(self.y0..=self.y1, ray.1);
                let min_y = self.y0.min(self.y1);
                let max_y = self.y0.max(self.y1);
                // dbg!((min_y..=max_y).contains(&ray.1));
                if self.y0 == self.y1 && self.y0 == ray.1 {
                    Intersection::Collinear(self.x0.min(self.x1), self.x0.max(self.x1))
                } else if (min_y + 1..max_y).contains(&ray.1) {
                    // Do not consider collinear as intersection
                    Intersection::Some(self.x0)
                } else {
                    Intersection::None
                }
            }
            (0, -1) | (0, 1) => {
                let min_x = self.x0.min(self.x1);
                let max_x = self.x0.max(self.x1);
                if (min_x..=max_x).contains(&ray.1) && self.x0 != self.x1 {
                    Intersection::Some(self.y0)
                } else {
                    Intersection::None
                }
            }
            _ => panic!(),
        }
    }
}

// mod test {
//     use crate::Edge;

//     #[test]
//     fn test_intersect() {
//         let edge = Edge::new(0, 20, 5, 5);
//         assert_eq!(edge.intersects(((1, 0), 5)), None);
//         assert_eq!(edge.intersects(((0, 1), 5)), Some(5));
//         assert_eq!(edge.intersects(((0, 1), 20)), Some(5));
//         assert_eq!(edge.intersects(((0, 1), 21)), None);
//         assert_eq!(edge.intersects(((0, -1), -1)), None);
//         let edge = Edge::new(10, 10, -25, 75);
//         assert_eq!(edge.intersects(((1, 0), -26)), None);
//         assert_eq!(edge.intersects(((1, 0), -25)), Some(10));
//         assert_eq!(edge.intersects(((1, 0), 5)), Some(10));
//         assert_eq!(edge.intersects(((1, 0), 10)), Some(10));
//         assert_eq!(edge.intersects(((-1, 0), 10)), Some(10));
//         assert_eq!(edge.intersects(((0, 1), 21)), None);
//         assert_eq!(edge.intersects(((0, -1), -5)), None);
//     }
// }

fn get_filled_count(path: &Vec<Position>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> i32 {
    let mut filled_count = 0;
    let mut previous_line_inside = HashMap::new();
    for j in min_y..=max_y {
        let mut last = false;
        let mut inside = 0;
        let mut on_edge = false;
        let mut this_line_inside = HashMap::new();

        for i in min_x..=max_x {
            let cur = path.contains(&(i, j));
            // dbg!(i);
            if cur != last {
                inside += 1;
                if on_edge {
                    if j == min_y {
                        // We are outside
                        inside = 0;
                    } else {
                        // Use previous line to determine if we are inside after the edge
                        if let Some(v) = previous_line_inside.get(&i) {
                            inside = *v;
                        } else {
                            inside = 0;
                        }
                    }
                    on_edge = false;
                }
            } else if cur && last {
                // We are on an edge
                on_edge = true;
                // println!("edge {} {}", i, j);
            }
            last = cur;
            // println!("{} {} -> {}", i, j, inside);

            if inside % 4 != 0 || cur {
                this_line_inside.insert(i, inside);
                filled_count += 1;
                if cur {
                    // print!("{}", "#".custom_color(CustomColor { r: 10, g: 255, b: 150 }));
                } else {
                    // print!("#");
                }
            } else {
                // print!(".");
            }
        }
        // println!();
        previous_line_inside = this_line_inside;
    }
    filled_count
}

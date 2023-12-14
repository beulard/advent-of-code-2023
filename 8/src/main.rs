use std::{cell::RefCell, collections::HashMap, ops::ControlFlow, rc::Rc};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let instructions: Vec<Instruction> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!(),
        })
        .collect();

    let nodes: HashMap<String, Node> =
        input.lines().skip(2).fold(HashMap::new(), |mut map, line| {
            let key = line.split_whitespace().next().unwrap().to_string();
            let destinations = line.split(" = ").last().unwrap().to_string();
            let mut destinations = destinations
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(", ")
                .map(|s| s.to_string());

            map.insert(
                key,
                Node {
                    left: destinations.next().unwrap(),
                    right: destinations.next().unwrap(),
                },
            );

            map
        });
    println!("PART 1");

    let mut cur_node = nodes.get("AAA").unwrap();

    instructions
        .iter()
        .cycle()
        .enumerate()
        .try_for_each(|(step_idx, instruction)| {
            let next_node_name = match instruction {
                Instruction::Left => &cur_node.left,
                Instruction::Right => &cur_node.right,
            };
            let next_node = nodes.get(next_node_name);
            // dbg!(next_node_name);
            // dbg!(next_node);
            cur_node = next_node.unwrap();
            if next_node_name == "ZZZ" {
                println!("Number of steps taken: {}", step_idx + 1);
                return ControlFlow::Break(());
            }

            ControlFlow::Continue(())
        });

    println!("\nPART 2");

    // IDEA: instead of doing a hashmap indexing every time, use references to nodes in the node array
    // This way we can jump directly to left/right without a map access.

    #[derive(Debug)]
    struct NodeRefs {
        name: String,
        left: Option<Rc<RefCell<NodeRefs>>>,
        right: Option<Rc<RefCell<NodeRefs>>>,
    }

    let mut node_refs = vec![];
    for node in &nodes {
        node_refs.push(Rc::new(RefCell::new(NodeRefs {
            name: node.0.clone(),
            left: None,
            right: None,
        })));
    }
    for node in &node_refs {
        let orig = nodes.get(&node.borrow().name).unwrap();
        let left = node_refs
            .iter()
            .find(|n| n.borrow().name == orig.left)
            .unwrap();
        let right = node_refs
            .iter()
            .find(|n| n.borrow().name == orig.right)
            .unwrap();
        node.borrow_mut().left = Some(left.clone());
        node.borrow_mut().right = Some(right.clone());
    }

    // dbg!(node_refs);
    let starting_nodes: Vec<_> = node_refs
        .iter()
        .filter(|x| x.borrow().name.ends_with('A'))
        .collect();

    let mut cur_nodes = vec![];
    for i in 0..starting_nodes.len() {
        cur_nodes.push(starting_nodes[i].clone());
    }

    let mut cycle_steps = vec![];

    // For each starting node, find the first node ending with Z
    for node in starting_nodes {
        let mut cur = node.clone();
        let mut first_z = 0;
        for instruction in instructions.iter().cycle().enumerate() {
            // Iterate until we get to a node ending with Z

            let left = cur.clone().borrow().left.clone().unwrap();
            let right = cur.clone().borrow().right.clone().unwrap();

            if cur.borrow().name.ends_with('Z') {
                if first_z == 0 {
                    first_z = instruction.0;
                    // first_name = cur.borrow().name.clone();
                    // dbg!(&node.borrow().name, &cur.borrow().name, first_z, &left.borrow().name, &right.borrow().name, instruction.1);
                } else {
                    let next_z = instruction.0;
                    // dbg!(&node.borrow().name, &cur.borrow().name, next_z, &left.borrow().name, &right.borrow().name, instruction.1);

                    cycle_steps.push(next_z - first_z);
                    break;
                }
            }

            match instruction.1 {
                Instruction::Left => cur = left,
                Instruction::Right => cur = right,
            }
        }
    }

    fn lcm2(a: usize, b: usize) -> usize {
        let mut left = a;
        let mut right = b;
        loop {
            if left == right {
                return left;
            } else {
                if left.min(right) == left {
                    left += a;
                } else {
                    right += b;
                }
            }
        }
    }

    fn lcm(numbers: &[usize]) -> usize {
        if numbers.len() == 1 {
            return numbers[0];
        } else if numbers.len() == 2 {
            lcm2(numbers[0], numbers[1])
        } else {
            let lcm01 = lcm(&numbers[0..=1]);
            lcm(&[&[lcm01], &numbers[2..]].concat())
        }
    }
    // dbg!(&cycle_steps);
    println!("All nodes end in 'Z' at step: {}", lcm(&cycle_steps));
}

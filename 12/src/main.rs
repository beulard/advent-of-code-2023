fn find_combinations(states: &Vec<MachineState>, counts: &Vec<usize>) {
    dbg!(states, counts);
}

#[derive(Debug)]
enum MachineState {
    Working,
    Broken,
    Unknown,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    use MachineState::*;
    for line in input.lines() {
        let states: Vec<MachineState> = line
            .split_whitespace()
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => Working,
                '#' => Broken,
                '?' => Unknown,
                _ => panic!(),
            })
            .collect();

        let counts: Vec<usize> = line
            .split_whitespace()
            .last()
            .unwrap()
            .split(",")
            .map(|numstr| numstr.parse::<usize>().unwrap())
            .collect();

        find_combinations(&states, &counts);
    }
    // dbg!(&machine_states);

    for line in input.lines() {}
}

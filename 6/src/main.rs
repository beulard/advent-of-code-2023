fn part1(input: &String) -> i32 {
    let mut lines = input.lines();
    let times: Vec<_> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let distances: Vec<_> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut winner_product = 1;
    for (time, record_distance) in times.iter().zip(distances.iter()) {
        // We can press the button between 1 and time-1 seconds

        // Brute force
        let mut winrars = 0;
        for press_time in 1..=time - 1 {
            // Speed is equal to press_time / 1 ms
            // Distance travelled equals speed * time remaining in race
            //                        =  speed * (time - press_time)
            //                        =  press_time * (time - press_time)
            // = press_time * time - press_time^2
            let travelled_distance = press_time * (time - press_time);
            if &travelled_distance > record_distance {
                winrars += 1;
            }
        }
        // dbg!(winrars);
        winner_product *= winrars;
    }
    return dbg!(winner_product);
}

fn part2(input: &String) -> u64 {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    let record_distance: u64 = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    // dbg!(record_distance, time);

    // Brute force
    let mut winrars = 0;
    for press_time in 1..=time - 1 {
        // Speed is equal to press_time / 1 ms
        // Distance travelled equals speed * time remaining in race
        //                        =  speed * (time - press_time)
        //                        =  press_time * (time - press_time)
        // = press_time * time - press_time^2
        let travelled_distance = press_time * (time - press_time);
        if travelled_distance > record_distance {
            winrars += 1;
        }
    }

    dbg!(winrars)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("PART 1");
    part1(&input);
    println!("PART 2");
    part2(&input);
}

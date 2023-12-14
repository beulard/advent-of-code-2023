use std::{cell::RefCell, rc::Rc};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let readings: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // dbg!(&readings);

    fn get_diff(reading: &Vec<i32>) -> Vec<i32> {
        let d = reading.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        // dbg!(&reading, &d);
        d
    }

    fn extrapolate(reading: &Vec<i32>) -> i32 {
        if reading.iter().all(|v| *v == 0) {
            0
        } else {
            reading.last().unwrap() + extrapolate(&get_diff(reading))
        }
    }

    let next_vals = readings.iter().map(|r| extrapolate(&r));
    // dbg!(&next_vals);

    let sum: i32 = next_vals.sum();
    println!("Sum of extrapolated histories: {}", sum);

    let backward_readings: Vec<Vec<i32>> = readings
        .iter()
        .map(|r| {
            let mut revd = r.clone();
            revd.reverse();
            revd
        }).collect();
    let next_vals = backward_readings.iter().map(|r| extrapolate(&r));
    // dbg!(&next_vals);

    let sum: i32 = next_vals.sum();
    println!("Sum of backward-extrapolated histories: {}", sum);
}

fn str_to_numeric(s: &str) -> u32 {
    match s {
        "1" | "one"   => 1,
        "2" | "two"   => 2,
        "3" | "three" => 3,
        "4" | "four"  => 4,
        "5" | "five"  => 5,
        "6" | "six"   => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine"  => 9,
        _ => 0
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    let valid_matches = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        // Find first digit
        let mut matches = vec![];
        for m in valid_matches {
            line.match_indices(m).for_each(|element| { matches.push(element); });
        }
        dbg!(line);
        matches.sort_by(|a, b| a.0.cmp(&b.0));
        // dbg!(&matches);
        let first = matches.first().unwrap();
        let last = matches.last().unwrap();
        // dbg!(str_to_numeric(first.1));
        // dbg!(str_to_numeric(last.1));
        let result = str_to_numeric(first.1) * 10 + str_to_numeric(last.1);
        println!("{} {} {} -> {}", first.1, line, last.1, result);
        sum += result;
    }
    println!("{}", sum);
}

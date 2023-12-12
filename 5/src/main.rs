use std::collections::BTreeMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
    part2_optim(&input);
}

// PART 1

#[derive(Debug)]
struct RangeMapping {
    dest: u64,
    src: u64,
    len: u64,
}

impl RangeMapping {
    fn contains(&self, x: u64) -> bool {
        x >= self.src && x < self.src + self.len
    }
    fn map(&self, x: u64) -> u64 {
        if self.contains(x) {
            x - self.src + self.dest
        } else {
            x
        }
    }
}

fn fill_mapping(line: &str) -> Option<RangeMapping> {
    if let [dest, src, len] = line.split_ascii_whitespace().collect::<Vec<_>>()[..] {
        let dest = dest.parse::<u64>().unwrap();
        let src = src.parse::<u64>().unwrap();
        let len = len.parse::<u64>().unwrap();

        // dbg!([dest, src, len]);

        Some(RangeMapping {
            dest: dest,
            src: src,
            len: len,
        })
        // dbg!(&map);
    } else {
        None
    }
}
fn part1(input: &String) {
    println!("PART 1");
    let mut seeds = vec![];
    let mut maps = vec![];

    input.split("\n\n").for_each(|block| {
        if block.starts_with("seeds:") {
            // initial seeds block
            block.split_ascii_whitespace().skip(1).for_each(|seed| {
                // dbg!(&seed);
                seeds.push(seed.parse::<u64>().unwrap());
            })
        } else {
            // map block
            println!("{}", &block.split("\n").nth(0).unwrap());
            maps.push(vec![]);

            block
                .split("\n")
                .skip(1)
                .for_each(|line| match fill_mapping(line) {
                    Some(range) => maps.last_mut().unwrap().push(range),
                    None => (),
                });
        }
    });
    // dbg!(&seeds);
    // dbg!(&maps);
    let mut dests = vec![];
    for seed in seeds {
        print!("Seed {} -> location ", seed);
        let mut dest = seed;
        for map in &maps {
            for range in map {
                if range.contains(dest) {
                    dest = range.map(dest);
                    break;
                }
            }
        }
        dests.push(dest);
        println!("{}", &dest);
    }
    println!("Min: {}", dests.iter().min().unwrap());
}

fn part2(input: &String) {
    println!("PART 2");

    let mut blocks = input.split("\n\n").into_iter();
    let seed_ranges = blocks
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|a| a.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    #[derive(Debug, Clone)]
    struct SeedRange {
        low: u64,
        high: u64,
    }
    let seed_ranges: Vec<_> = seed_ranges
        .chunks(2)
        .map(|a| SeedRange {
            low: a[0],
            high: a[0] + a[1],
        })
        .collect();
    // dbg!(&seed_ranges);
    let maps = blocks
        .map(|block| {
            block
                .split("\n")
                .filter(|c| !c.is_empty())
                .skip(1)
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|number| number.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                        .chunks(3)
                        .map(|a| RangeMapping {
                            dest: a[0],
                            src: a[1],
                            len: a[2],
                        })
                        .next()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut mymaps = vec![];
    for mapidx in 0..maps.len() {
        mymaps.push(BTreeMap::new());
        for r in &maps[mapidx] {
            mymaps[mapidx].insert(r.src, (r.dest, r.len));
        }
    }
    // dbg!(&mymaps);

    // Idea: reorder so we do it in this way:
    // For each seed range;
    // For each layer;
    //  Find the right map for the first seed
    //  Figure out how many subsequent seeds will be transformed identically using the map's length and src
    //
    //  At the end, process the result into ranges again, like for the initial input
    // Repeat for the next layer and then the next seed range

    // Output ranges for each map layer
    let mut mapped_ranges: Vec<Vec<SeedRange>> = vec![];
    // Input ranges. At the end of this iteration, we assign to it the output of this layer
    // so we can perform the same calculations with the next layer.
    let mut in_ranges: Vec<SeedRange> = seed_ranges.clone();
    for mymap in &mymaps {
        // println!("new map");
        mapped_ranges.push(vec![]);
        let mut out_ranges: Vec<SeedRange> = vec![];
        for range in &in_ranges {
            // println!("new seed range {} -> {}", range.low, range.high);
            let mut cur_seed: u64 = range.low;
            while cur_seed < range.high {
                // Find the nearest map with src <= cur_seed
                let map_below = mymap
                    .iter()
                    .rev()
                    .skip_while(|(src, (dest, len))| src > &&cur_seed)
                    .next();

                // If found:
                if let Some((src, (dest, len))) = map_below {
                    if src + len > cur_seed {
                        // println!("case 1");
                        // dbg!(cur_seed, src, len, dest);
                        let range_end = (src + len).min(range.high);
                        out_ranges.push(SeedRange {
                            low: cur_seed + dest - src,
                            high: range_end + dest - src,
                        });
                        // dbg!(&mapped_ranges);
                        // Jump to the end of the map's range directly
                        cur_seed = range_end;
                    } else {
                        // The range below is too short ! map unity until the next range
                        // println!("case 4");
                        let it = mymap.iter().skip_while(|(src, _)| src <= &&cur_seed).next();
                        if let Some((src, (dest, len))) = it {
                            // println!("case 4.2");

                            // Found a map with src > cur_seed -> use that to define length of unity mapping
                            let range_end = range.high.min(*src);
                            out_ranges.push(SeedRange {
                                low: cur_seed,
                                high: range_end,
                            });
                            cur_seed = range_end;
                        } else {
                            // No map with src > cur_seed => map unity until the end of the seed range

                            // println!("case 4.3");
                            // dbg!(cur_seed, src, len, dest);
                            out_ranges.push(SeedRange {
                                low: cur_seed,
                                high: range.high,
                            });
                            cur_seed = range.high;
                        }
                    }
                } else {
                    // No range below this seed
                    // Find the nearest map with src > cur_seed
                    let it = mymap.iter().skip_while(|(src, _)| src <= &&cur_seed).next();
                    if let Some((src, _)) = it {
                        // println!("case 2");

                        // Found a map with src > cur_seed -> use that to define length of unity mapping
                        let range_end = range.high.min(*src);
                        out_ranges.push(SeedRange {
                            low: cur_seed,
                            high: range_end,
                        });
                        cur_seed = range_end;
                    } else {
                        // println!("case 3");

                        // No map with src > cur_seed => map unity until the end of the seed range
                        out_ranges.push(SeedRange {
                            low: cur_seed,
                            high: range.high,
                        });
                        cur_seed = range.high;
                    }
                }
            }
        }
        // dbg!(&out_ranges);
        in_ranges = out_ranges;
    }
    // dbg!(&mapped_ranges);
    // dbg!(&in_ranges);

    // dbg!(last_ranges
    //     .into_iter()
    //     .fold(u64::MAX, |acc, x| x.0.min(acc)));
    let min = in_ranges.iter().fold(u64::MAX, |acc, x| acc.min(x.low));
    println!("{}", min);
}

#[derive(Debug, Clone)]
struct SeedRange {
    low: i64,
    high: i64,
}

fn get_input_ranges(input: &String) -> Vec<SeedRange> {
    let mut blocks = input.split("\n\n").into_iter();
    let input_ranges = blocks
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|a| a.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    return input_ranges
        .chunks(2)
        .map(|a| SeedRange {
            low: a[0],
            high: a[0] + a[1],
        })
        .collect();
}

type Start = i64;
type Delta = i64;
type Length = i64;
type RangeMap = BTreeMap<Start, (Delta, Length)>;

fn get_map_layers(input: &String) -> Vec<RangeMap> {
    let blocks = input.split("\n\n").into_iter();
    let mut layers: Vec<RangeMap> = vec![];
    blocks.for_each(|block| {
        layers.push(BTreeMap::new());
        let layer = layers.last_mut().unwrap();
        let vec: Vec<_> = block.split("\n").filter(|c| !c.is_empty()).collect();
        vec.into_iter().skip(1).for_each(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .chunks(3)
                .for_each(|a| {
                    layer.insert(a[1], (a[0] - a[1], a[2]));
                })
        })
    });
    layers.into_iter().filter(|q| !q.is_empty()).collect()
}

fn part2_optim(input: &String) {    
    println!("PART 2 OPTIM");

    let input_ranges = get_input_ranges(input);
    // dbg!(&input_ranges);

    let mut layers = get_map_layers(input);

    // Fill gaps in layers (add a map with unity transformation between all maps)
    for layer in &mut layers {
        // Fill in before the first range
        if !layer.contains_key(&0) {
            let end = *layer.first_entry().unwrap().key();
            layer.insert(0, (0, end));
            // dbg!(layer.first_entry().unwrap());
            // dbg!(&layer[&0]);
            // dbg!(&layer);
        }
        let iter = layer.iter().skip(1).collect::<Vec<_>>();

        let wins = iter.windows(2);
        let mut to_insert : Vec<(i64, (i64, i64))> = vec![];
        for win in wins {
            if let [cur, next] = win {
                let (start, (delta, len)) = cur;
                let (nstart, (ndelta, nlen)) = next;
                if *start + len < **nstart {
                    // println!("EYOO {}", start);
                    // Fill in
                    let new_start = *start + len;
                    // dbg!(&(new_start, (0, *nstart - new_start)));
                    to_insert.push((new_start, (0, *nstart - new_start)));
                } else {
                    // println!("do nothing");
                }
            }
        }
        for toi in to_insert {
            layer.insert(toi.0, toi.1);
        }

        // Fill in after the last range
        // Assume there is no map going to i64::MAX
        let (start, (delta, len)) = layer.last_key_value().unwrap();
        // dbg!(&start, &delta, &len);
        layer.insert(start + len, (0, i64::MAX - (start + len)));
        // dbg!(&layer);
    }
    
    // Now that we have a vector of maps where all possible values in the i64 range lie
    // in one of the covered ranges, perhaps the algorithm will be simpler ?
    let mut in_ranges: Vec<SeedRange> = input_ranges.clone();
    for layer in &layers {
        // println!("new map");
        // mapped_ranges.push(vec![]);
        let mut out_ranges: Vec<SeedRange> = vec![];
        for range in &in_ranges {
            // println!("new seed range {} -> {}", range.low, range.high);
            let mut cur_seed: i64 = range.low;
            while cur_seed < range.high {
                // Find the nearest map with src <= cur_seed
                let map_below = layer
                    .iter()
                    .rev()
                    .skip_while(|(src, (delta, len))| src > &&cur_seed)
                    .next();

                // If found:
                if let Some((src, (delta, len))) = map_below {
                    if src + len > cur_seed {
                        // println!("case 1");
                        // dbg!(cur_seed, src, len, dest);
                        let range_end = (src + len).min(range.high);
                        out_ranges.push(SeedRange {
                            low: cur_seed + delta,
                            high: range_end + delta,
                        });
                        // dbg!(&mapped_ranges);
                        // Jump to the end of the map's range directly
                        cur_seed = range_end;
                    }else {
                        // println!("case 3");
    
                    }
                } else {
                    // println!("case 2");
                    dbg!(cur_seed, range);
                    dbg!(map_below);
                    assert!(false);
                }
            }
        }
        // dbg!(&out_ranges);
        in_ranges = out_ranges;
    }
    // dbg!(&in_ranges);
    let min = in_ranges.iter().fold(i64::MAX, |acc, x| acc.min(x.low));
    println!("{}", min);
}

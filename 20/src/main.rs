use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Module {
    Broadcaster {
        outputs: Vec<String>,
    },
    Conjunction {
        outputs: Vec<String>,
        inputs: Vec<String>,
        name: String,
        /// Map from source module name to pulse type
        state: HashMap<String, PulseType>,
    },
    FlipFlop {
        outputs: Vec<String>,
        name: String,
        state: PulseType,
    },
}

impl Module {
    fn execute(&mut self, input: &PulseType, from: &str) -> Option<Pulse> {
        match self {
            Self::Broadcaster { outputs } => {
                let pulse = Pulse {
                    signal: input.clone(),
                    targets: outputs.clone(),
                    from: "broadcaster".into(),
                };
                Some(pulse)
            }

            Self::Conjunction {
                outputs,
                inputs,
                name,
                state,
            } => {
                state.insert(from.into(), input.clone());
                let mut all_high = true;
                for name in inputs {
                    if *state.get(name).unwrap_or(&PulseType::Low) != PulseType::High {
                        all_high = false;
                    }
                }
                if all_high {
                    return Some(Pulse {
                        signal: PulseType::Low,
                        targets: outputs.clone(),
                        from: name.clone(),
                    });
                } else {
                    return Some(Pulse {
                        signal: PulseType::High,
                        targets: outputs.clone(),
                        from: name.clone(),
                    });
                }
            }

            Self::FlipFlop {
                outputs,
                name,
                state,
            } => {
                use PulseType::*;
                match input {
                    High => None,
                    Low => match state {
                        Low => {
                            *state = High;
                            Some(Pulse {
                                signal: High,
                                from: name.clone(),
                                targets: outputs.clone(),
                            })
                        }
                        High => {
                            *state = Low;
                            Some(Pulse {
                                signal: Low,
                                from: name.clone(),
                                targets: outputs.clone(),
                            })
                        }
                    },
                }
            }
        }
    }
}

type ModuleConfig = HashMap<String, Module>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Pulse {
    from: String,
    signal: PulseType,
    targets: Vec<String>,
}

/// Return the name of a module as a String
fn get_name(module_def: &str) -> String {
    module_def.split(" -> ").nth(0).unwrap().trim().into()
}

/// Return the outputs for a given module in vec form
fn get_outputs(module_def: &str) -> Vec<String> {
    module_def
        .split(" -> ")
        .nth(1)
        .unwrap()
        .split(", ")
        .fold(vec![], |mut acc, x| {
            acc.push(x.into());
            acc
        })
}

fn find_inputs(name: &str, input: &str) -> Vec<String> {
    let mut inputs = vec![];
    for line in input.lines() {
        let src = get_name(line);
        let outputs = get_outputs(line);
        for o in outputs {
            if o == name {
                inputs.push(src.trim_start_matches(['%', '&']).into());
            }
        }
    }
    inputs
}

fn parse_input(input: &str) -> ModuleConfig {
    let mut modules: ModuleConfig = HashMap::new();

    // Each line is a module
    for line in input.lines() {
        let name = get_name(line);
        let outputs = get_outputs(line);
        if name.starts_with("broadcaster") {
            // broadcaster
            modules.insert("broadcaster".into(), Module::Broadcaster { outputs });
        } else if name.starts_with("%") {
            // flip flop
            modules.insert(
                name[1..].into(),
                Module::FlipFlop {
                    outputs,
                    name: name[1..].into(),
                    state: PulseType::Low,
                },
            );
        } else if name.starts_with("&") {
            // conjunction
            let inputs = find_inputs(&name[1..], input);
            modules.insert(
                name[1..].into(),
                Module::Conjunction {
                    name: name[1..].into(),
                    inputs,
                    outputs,
                    state: HashMap::new(),
                },
            );
        }
    }

    modules
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut modules = parse_input(&input);
    dbg!(&modules);

    // We have to consider pulses in the order that they are sent
    // Use a FIFO VecDeque -> push_back to add a pulse, pop_front to get the next pulse
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();

    println!();
    let mut high_pulse_count = 0;
    let mut low_pulse_count = 0;

    for _ in 0..1000 {
        pulse_queue.push_back(Pulse {
            from: "button".into(),
            signal: PulseType::Low,
            targets: vec!["broadcaster".into()],
        });
        while let Some(pulse) = pulse_queue.pop_front() {
            // let mut targets = vec![];
            pulse.targets.iter().for_each(|name| {
                match pulse.signal {
                    PulseType::High => high_pulse_count += 1,
                    PulseType::Low => low_pulse_count += 1,
                }

                // println!("{} -{:?}-> {}", pulse.from, pulse.signal, name);
                if let Some(target) = modules.get_mut(name) {
                    if let Some(output) = target.execute(&pulse.signal, &pulse.from) {
                        pulse_queue.push_back(output);
                    }
                }
            });
        }
        // println!();
    }

    println!("High: {}\nLow: {}", high_pulse_count, low_pulse_count);
    println!("Solution: {}", high_pulse_count * low_pulse_count);

    // PART 2

    // Brute force impossible -> be smart
    // Create dependency graph from rx to the input pulse ?
    // e.g. - for rx to be low, cn must send a low pulse
    //      - for cn to send a low pulse, all its inputs must be high
    //      - for th to be high,

    // Find periodicity of inputs to rx and then compute lowest number of presses as the
    // lowest common factor.
    // Recurse over inputs to cn.

    let mut low_pulses_to_rx = 0;
    let mut button_presses = 0;

    // For each module, keep track of its state at the end of the button cycle
    // As soon as possible, determine if the module state follows a pattern over time
    // When the pattern is fixed and can no longer increase in period, set the period
    let mut pattern: HashMap<String, Vec<PulseType>> = HashMap::new();
    let mut memo: HashMap<Vec<PulseType>, Vec<PulseType>> = HashMap::new();

    use PulseType::*;

    while
    // low_pulses_to_rx != 1
    button_presses < 10000 
    {
        button_presses += 1;
        if button_presses % 10000 == 0 {
            println!("{}", button_presses);
        }
        // dbg!(button_presses);
        // let input = get_states(&modules);
        // // dbg!(&input);
        // if let Some(output) = memo.get(&input) {
        //     // dbg!(output);
        //     for (idx, (name, module)) in modules
        //         .iter_mut()
        //         .filter(|(n, x)| match x {
        //             Module::FlipFlop {
        //                 outputs,
        //                 name,
        //                 state,
        //             } => true,
        //             _ => false,
        //         })
        //         .enumerate()
        //     {
        //         match module {
        //             Module::FlipFlop {
        //                 outputs,
        //                 name,
        //                 state,
        //             } => {
        //                 // println!("{}", name);
        //                 *state = output[idx].clone();
        //             }
        //             _ => {}
        //         }
        //     }
        //     continue;
        // }

        pulse_queue.push_back(Pulse {
            from: "button".into(),
            signal: Low,
            targets: vec!["broadcaster".into()],
        });
        while let Some(pulse) = pulse_queue.pop_front() {
            // let mut targets = vec![];
            // if pulse.from == "cn" && pulse.signal == PulseType::Low {
            //     dbg!(pulse);
            //     panic!();
            // }
            pulse.targets.iter().for_each(|name| {
                match pulse.signal {
                    High => high_pulse_count += 1,
                    Low => low_pulse_count += 1,
                }
                // println!("{}", name.as_str());
                match (name.as_str(), &pulse.signal) {
                    ("rx", Low) => {
                        println!("{}", button_presses);
                        low_pulses_to_rx += 1;
                        panic!();
                    }
                    _ => (),
                }

                // println!("{} -{:?}-> {}", pulse.from, pulse.signal, name);
                if let Some(target) = modules.get_mut(name) {
                    if let Some(output) = target.execute(&pulse.signal, &pulse.from) {
                        pulse_queue.push_back(output);
                    }
                }
                if name.as_str() == "cn" && pulse.signal == High {
                    // println!("{} {:?}", name.as_str(), &pulse.signal);
                    // dbg!(&pulse.from);
                    // panic!();
                }
            });
            // dbg!(&modules["cn"]);
        }
        let output = get_states(&modules);
        for (n, m) in &modules {
            match m {
                Module::FlipFlop { outputs, name, state } => {
                    pattern.entry(n.clone()).or_insert(vec![]).push(state.clone());

                }, _ => {}
            }
        }

        // memo.insert(input, output);
        // println!();
    }

    for (module, train) in pattern {
        print!("{}: ", module);
        for p in train {
            print!(
                "{}",
                match p {
                    High => "-",
                    Low => "_",
                }
            );
        }
        println!();
    }

    // Try to find a pattern in the flip flop signals
    // Assume we have enough samples that there is no hidden information
    // -> if we find the largest period, we can extrapolate the state at any iteration number
    for (name, pulses) in pattern {

    }

    println!("{}", memo.len());

    println!("Button presses for rx=1: {}", button_presses);
}

fn find_largest_period

fn get_states(modules: &HashMap<String, Module>) -> Vec<PulseType> {
    let mut ret = vec![];
    for (name, module) in modules {
        // println!("{}", name);
        match module {
            // Module::Conjunction {
            //     outputs,
            //     inputs,
            //     name,
            //     state,
            // } => {
            //     // println!("{}: {:?}", name, state);
            //     let all_high = state.iter().all(|(_, x)| *x == High);
            //     let val = match all_high {
            //         true => Low,
            //         false => High,
            //     };

            //     pattern.insert(name.clone(), val.clone());
            //     ret.push(val.clone());
            // }
            Module::FlipFlop {
                outputs,
                name,
                state,
            } => {
                // println!("{}: {:?}", name, state);
                // pattern.insert(name.clone(), state.clone());
                ret.push(state.clone());
            }
            _ => {}
        }
    }
    ret
}

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
enum Module {
    Broadcaster {
        outputs: Vec<String>,
    },
    Conjunction {
        outputs: Vec<String>,
        inputs: Vec<String>,
        name: String,
        // Map from source module name to pulse type
        // state: HashMap<String, PulseType>,
    },
    FlipFlop {
        outputs: Vec<String>,
        name: String,
        state: PulseType,
    },
}

struct ModuleConfig(HashMap<String, RefCell<Module>>);

impl ModuleConfig {
    fn get_state(&self, name: &str) -> PulseType {
        if let Some(module) = self.0.get(name) {
            let m = module.borrow();
            match &*m {
                Module::Conjunction {
                    outputs,
                    inputs,
                    name,
                } => {
                    let mut all_high = true;
                    for name in inputs {
                        if self.get_state(name) != PulseType::High {
                            all_high = false;
                        }
                    }
                    match all_high {
                        true => PulseType::Low,
                        false => PulseType::High,
                    }
                }
                Module::Broadcaster { outputs } => PulseType::Low,
                Module::FlipFlop {
                    outputs,
                    name,
                    state,
                } => state.clone(),
            }
        } else {
            panic!();
        }
    }

    fn execute(&mut self, pulse: Pulse, pulse_queue: &mut VecDeque<Pulse>) {
        pulse.targets.iter().for_each(|name| {
            // println!("{} -{:?}-> {}", pulse.from, pulse.signal, name);
            if let Some(target) = self.0.get(name).clone() {
                // if let Some(output) = target.execute(&pulse.signal, &pulse.from) {
                //     pulse_queue.push_back(output);
                // }
                let current = self.get_state(&name);

                match (target.borrow_mut().deref_mut()) {
                    Module::Broadcaster { outputs } => {
                        let pulse = Pulse {
                            signal: pulse.signal.clone(),
                            targets: outputs.clone(),
                        };
                        pulse_queue.push_back(pulse);
                    }

                    Module::Conjunction {
                        outputs,
                        inputs,
                        name,
                    } => {
                        // let current = self.0[name].borrow();
                        pulse_queue.push_back(Pulse {
                            signal: current,
                            targets: outputs.clone(),
                        });
                    }

                    Module::FlipFlop {
                        outputs,
                        name,
                        state,
                    } => {
                        use PulseType::*;
                        match pulse.signal {
                            High => {}
                            Low => match state {
                                Low => {
                                    *state = High;
                                    pulse_queue.push_back(Pulse {
                                        signal: High,
                                        targets: outputs.clone(),
                                    });
                                }
                                High => {
                                    *state = Low;
                                    pulse_queue.push_back(Pulse {
                                        signal: Low,
                                        targets: outputs.clone(),
                                    });
                                }
                            },
                        }
                    }
                }
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Pulse {
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
    let mut modules: ModuleConfig = ModuleConfig(HashMap::new());

    // Each line is a module
    for line in input.lines() {
        let name = get_name(line);
        let outputs = get_outputs(line);
        if name.starts_with("broadcaster") {
            // broadcaster
            modules.0.insert(
                "broadcaster".into(),
                RefCell::new(Module::Broadcaster { outputs }),
            );
        } else if name.starts_with("%") {
            // flip flop
            modules.0.insert(
                name[1..].into(),
                RefCell::new(Module::FlipFlop {
                    outputs,
                    name: name[1..].into(),
                    state: PulseType::Low,
                }),
            );
        } else if name.starts_with("&") {
            // conjunction
            let inputs = find_inputs(&name[1..], input);
            modules.0.insert(
                name[1..].into(),
                RefCell::new(Module::Conjunction {
                    name: name[1..].into(),
                    inputs,
                    outputs,
                }),
            );
        }
    }

    modules
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut modules = parse_input(&input);

    // We have to consider pulses in the order that they are sent
    // Use a FIFO VecDeque -> push_back to add a pulse, pop_front to get the next pulse
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();

    println!();
    let mut high_pulse_count = 0;
    let mut low_pulse_count = 0;

    for _ in 0..1000 {
        pulse_queue.push_back(Pulse {
            signal: PulseType::Low,
            targets: vec!["broadcaster".into()],
        });
        while let Some(pulse) = pulse_queue.pop_front() {
            // let mut targets = vec![];
            // if pulse.from == "cn" && pulse.signal == PulseType::Low {
            //     dbg!(pulse);
            //     panic!();
            // }
            pulse.targets.iter().for_each(|name| {
                if name == "output" {
                    println!("output: {:?}", pulse);
                }
                match pulse.signal {
                    PulseType::High => high_pulse_count += 1,
                    PulseType::Low => low_pulse_count += 1,
                }
            });
            modules.execute(pulse, &mut pulse_queue);
            // let mut targets = vec![];
        }
        // dbg!(&modules["con"]);
        // println!();
    }

    println!("High: {}\nLow: {}", high_pulse_count, low_pulse_count);
    println!("Solution: {}", high_pulse_count * low_pulse_count);

    // PART 2
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut modules = parse_input(&input);
    // dbg!(&modules);
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();

    // List of states for each module
    let mut sequence: HashMap<String, Vec<PulseType>> = HashMap::new();

    for _ in 0..13000 {
        pulse_queue.push_back(Pulse {
            signal: PulseType::Low,
            targets: vec!["broadcaster".into()],
        });

        while let Some(pulse) = pulse_queue.pop_front() {
            // let mut targets = vec![];
            // if pulse.from == "cn" && pulse.signal == PulseType::Low {
            //     dbg!(pulse);
            //     panic!();
            // }
            pulse.targets.iter().for_each(|name| {
                if name == "output" {
                    println!("output: {:?}", pulse);
                }
                match pulse.signal {
                    PulseType::High => high_pulse_count += 1,
                    PulseType::Low => low_pulse_count += 1,
                }
            });
            modules.execute(pulse, &mut pulse_queue);
            // let mut targets = vec![];
        }

        for (name, module) in &modules.0 {
            let state = modules.get_state(name);

            sequence
                .entry(name.clone())
                .or_insert(vec![])
                .push(state.clone());
        }
    }

    // Draw the input pattern over a few iterations
    for m in ["kl", "ml", "xs", "jn"] {
        print!("{}", m);
        for (i, s) in sequence[m].iter().enumerate() {
            print!(
                "{}",
                match s {
                    PulseType::High => "^",
                    PulseType::Low => "_",
                }
            );
        }
        println!();
    }

    // Take the inputs and determine the step where they keep the same value instead of alternating
    let mut special_step_idx = vec![];

    'input: for m in ["kl", "ml", "xs", "jn"] {
        let mut opp = PulseType::High;
        // print!("{}", m);
        for (i, s) in sequence[m].iter().enumerate() {
            if *s != opp {
                println!("{}", i + 1);
                special_step_idx.push(i + 1);
                continue 'input;
            }
            opp = match s {
                PulseType::High => PulseType::Low,
                PulseType::Low => PulseType::High,
            };
        }
    }
        
        println!("LCM of irregular steps in inputs: {}", lcm(&special_step_idx));
}

// LCM functions from day 8
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

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::{DerefMut};

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

    // Brute force impossible -> be smart
    // Create dependency graph from rx to the input pulse ?
    // e.g. - for rx to be low, cn must send a low pulse
    //      - for cn to send a low pulse, all its inputs must be high
    //      - for th to be high,

    // Find periodicity of inputs to rx and then compute lowest number of presses as the
    // lowest common factor.
    // Recurse over inputs to cn.
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut modules = parse_input(&input);
    // dbg!(&modules);
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();

    let mut low_pulses_to_rx = 0;
    let mut button_presses = 0;

    // For each module, keep track of its state at the end of the button cycle
    // As soon as possible, determine if the module state follows a pattern over time
    // When the pattern is fixed and can no longer increase in period, set the period
    let mut pattern: HashMap<String, Vec<PulseType>> = HashMap::new();

    use PulseType::*;

    // let mut cn_state = HashMap::new();

    // Try setting flip flops 

    while 
        low_pulses_to_rx != 1 {
        // button_presses < 1000 {
        button_presses += 1;
        if button_presses % 10000 == 0 {
            println!("{}", button_presses);
        }
        pulse_queue.push_back(Pulse {
            signal: Low,
            targets: vec!["broadcaster".into()],
        });
        while let Some(pulse) = pulse_queue.pop_front() {
            modules.execute(pulse, &mut pulse_queue);
        }
    }

    // Try to find a pattern in the flip flop signals
    // Assume we have enough samples that there is no hidden information
    // -> if we find the smallest period, we can extrapolate the state at any iteration number
    // let mut periods = vec![];
    // for (name, pulses) in &pattern {
    //     let period = find_smallest_period(pulses);
    //     println!("{}: {}", name, period);
    //     // The state of this flip flop at iteration N is the same as the state at iteration N % period:
    //     assert!(7000 > period);
    //     assert_eq!(pulses[7000], pulses[7000 % period]);
    //     periods.push(period);
    // }

    // Now find the required state for rx to receive one low pulse
    // For rx to receive one low pulse, one of its inputs must send exactly one low pulse
    // Since its output is a conjunction, the conjunction must have all high inputs ONCE in the run

    // modules.iter().filter(|x| match x.1 {

    // });

    println!("Button presses for rx=1: {}", button_presses);
}

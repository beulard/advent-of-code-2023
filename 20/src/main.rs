use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

enum ModuleType {
    Broadcaster,
    Conjunction,
    FlipFlop,
}

trait Module: Debug {
    fn execute(&mut self, input: &PulseType, from: &str) -> Option<Pulse>;
    fn module_type(&self) -> ModuleType;
}

#[derive(Debug)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl Module for Broadcaster {
    fn execute(&mut self, input: &PulseType, _: &str) -> Option<Pulse> {
        let pulse = Pulse {
            signal: input.clone(),
            targets: self.outputs.clone(),
            from: "broadcaster".into(),
        };
        Some(pulse)
    }
    fn module_type(&self) -> ModuleType {
        ModuleType::Broadcaster
    }
}

#[derive(Debug)]
struct Conjunction {
    outputs: Vec<String>,
    inputs: Vec<String>,
    name: String,
    /// Map from source module name to pulse type
    state: HashMap<String, PulseType>,
}

impl Module for Conjunction {
    fn execute(&mut self, input: &PulseType, from: &str) -> Option<Pulse> {
        self.state.insert(from.into(), input.clone());
        let mut all_high = true;
        for name in &self.inputs {
            if *self.state.get(name).unwrap_or(&PulseType::Low) != PulseType::High {
                all_high = false;
            }
        }
        if all_high {
            return Some(Pulse {
                signal: PulseType::Low,
                targets: self.outputs.clone(),
                from: self.name.clone(),
            });
        } else {
            return Some(Pulse {
                signal: PulseType::High,
                targets: self.outputs.clone(),
                from: self.name.clone(),
            });
        }
    }
    fn module_type(&self) -> ModuleType {
        ModuleType::Conjunction
    }
}

#[derive(Debug)]
struct FlipFlop {
    outputs: Vec<String>,
    name: String,
    state: PulseType,
}

impl Module for FlipFlop {
    fn execute(&mut self, input: &PulseType, _: &str) -> Option<Pulse> {
        use PulseType::*;
        match input {
            High => None,
            Low => match self.state {
                Low => {
                    self.state = High;
                    Some(Pulse {
                        signal: High,
                        from: self.name.clone(),
                        targets: self.outputs.clone(),
                    })
                }
                High => {
                    self.state = Low;
                    Some(Pulse {
                        signal: Low,
                        from: self.name.clone(),
                        targets: self.outputs.clone(),
                    })
                }
            },
        }
    }
    fn module_type(&self) -> ModuleType {
        ModuleType::FlipFlop
    }
}

type ModuleConfig = HashMap<String, Box<dyn Module>>;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Pulse {
    fn new() -> Self {
        Self {
            from: "".into(),
            signal: PulseType::Low,
            targets: vec![],
        }
    }
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
            modules.insert("broadcaster".into(), Box::new(Broadcaster { outputs }));
        } else if name.starts_with("%") {
            // flip flop
            modules.insert(
                name[1..].into(),
                Box::new(FlipFlop {
                    outputs,
                    name: name[1..].into(),
                    state: PulseType::Low,
                }),
            );
        } else if name.starts_with("&") {
            // conjunction
            let inputs = find_inputs(&name[1..], input);
            modules.insert(
                name[1..].into(),
                Box::new(Conjunction {
                    name: name[1..].into(),
                    inputs,
                    outputs,
                    state: HashMap::new(),
                }),
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

    let mut low_pulses_to_rx = 0;
    let mut button_presses = 0;
    while low_pulses_to_rx != 1 {
        button_presses += 1;
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

                // if name.as_str() == "rx" {
                // println!("{} {:?}", name.as_str(), &pulse.signal);
                // }
                match (name.as_str(), &pulse.signal) {
                    ("rx", PulseType::Low) => {
                        println!("!");
                        low_pulses_to_rx += 1;
                    }
                    _ => ()
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

    println!("Button presses for rx=1: {}", button_presses);

}

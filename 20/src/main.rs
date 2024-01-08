use std::{collections::HashMap, fmt::Debug};

trait Module: Debug {
    fn execute(&self, inputs: Vec<Pulse>) -> Vec<Pulse>;
}

#[derive(Debug)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl Module for Broadcaster {
    fn execute(&self, inputs: Vec<Pulse>) -> Vec<Pulse> {
        vec![]
    }
}

impl Broadcaster {
    fn new() -> Self {
        Self { outputs: vec![] }
    }
}

#[derive(Debug)]
struct Conjunction {
    outputs: Vec<String>,
}

impl Conjunction {
    fn new() -> Self {
        Self { outputs: vec![] }
    }
}

impl Module for Conjunction {
    fn execute(&self, inputs: Vec<Pulse>) -> Vec<Pulse> {
        vec![]
    }
}

#[derive(Debug)]
struct FlipFlop {
    outputs: Vec<String>,
}

impl FlipFlop {
    fn new() -> Self {
        Self { outputs: vec![] }
    }
}

impl Module for FlipFlop {
    fn execute(&self, inputs: Vec<Pulse>) -> Vec<Pulse> {
        vec![]
    }
}

type ModuleConfig = HashMap<String, Box<dyn Module>>;

enum PulseType {
    Low,
    High,
}

struct Pulse {
    signal: PulseType,
    targets: Vec<Box<dyn Module>>,
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
            modules.insert(name[1..].into(), Box::new(FlipFlop { outputs }));
        } else if name.starts_with("&") {
            // conjunction
            modules.insert(name[1..].into(), Box::new(Conjunction { outputs }));
        }
    }

    modules
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let modules = parse_input(&input);
    dbg!(modules);
}

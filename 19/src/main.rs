use std::collections::HashMap;

#[derive(Debug)]
enum Target {
    GoTo(String),
    Accept,
    Reject,
}

#[derive(Debug)]
enum Operator {
    Gt,
    Lt,
}

#[derive(Debug)]
enum Operand {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Condition {
    operand: Operand,
    operator: Operator,
    value: u32,
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    target: Target,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn new() -> Self {
        Self { rules: vec![] }
    }
}

fn parse_condition(cond: &str) -> Condition {
    let mut ret = Condition {
        operand: Operand::X,
        operator: Operator::Gt,
        value: 0,
    };
    let operator = if cond.contains('>') {
        '>'
    } else if cond.contains('<') {
        '<'
    } else {
        panic!()
    };
    ret.operator = match operator {
        '>' => Operator::Gt,
        '<' => Operator::Lt,
        _ => panic!(),
    };
    ret.operand = match cond.split(operator).nth(0).unwrap() {
        "x" => Operand::X,
        "m" => Operand::M,
        "a" => Operand::A,
        "s" => Operand::S,
        _ => panic!(),
    };
    ret.value = cond.split(operator).nth(1).unwrap().parse::<u32>().unwrap();
    ret
}

fn parse_workflows(list: &str) -> HashMap<String, Workflow> {
    let mut workflows = HashMap::new();

    for line in list.lines() {
        let mut wf = Workflow::new();

        line.split('{')
            .nth(1)
            .unwrap()
            .trim_end_matches('}')
            .split(',')
            .for_each(|r| {
                let mut rule = Rule {
                    condition: None,
                    target: Target::Reject,
                };
                let tgt: &str;
                if r.contains(':') {
                    // This rule has one or more conditions
                    let cond = r.split(':').nth(0).unwrap();
                    // Parse conditions
                    dbg!(cond);
                    rule.condition = Some(parse_condition(cond));
                    dbg!(&rule.condition);
                    // Set target to the right hand side of the ':'
                    tgt = r.split(':').nth(1).unwrap();
                } else {
                    // This rule is only a target
                    tgt = r;
                }

                // Handle target type
                match tgt {
                    "A" => rule.target = Target::Accept,
                    "R" => rule.target = Target::Reject,
                    "" => panic!(),
                    val => rule.target = Target::GoTo(val.to_string()),
                }
                dbg!(&rule.target);
                wf.rules.push(rule);
            });
        workflows.insert(line.split('{').nth(0).unwrap().to_string(), wf);
    }
    workflows
}

#[derive(Default, Debug)]
struct PartRating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_parts(list: &str) -> Vec<PartRating> {
    let mut parts = vec![];
    for line in list.lines() {
        let mut part = PartRating::default();
        line.trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .for_each(|category_rating| {
                // dbg!(category_rating);
                let category = category_rating.split('=').nth(0).unwrap();
                let value = category_rating
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                match category {
                    "x" => part.x = value,
                    "m" => part.m = value,
                    "a" => part.a = value,
                    "s" => part.s = value,
                    _ => panic!(),
                };
            });
        parts.push(part);
    }

    parts
}

fn apply_condition(part: &PartRating, condition: &Condition) -> bool {
    use Operand::*;
    let compare = match condition.operator {
        Operator::Gt => |a, b| a > b,
        Operator::Lt => |a, b| a < b,
    };
    match condition.operand {
        X => return compare(part.x, condition.value),
        M => return compare(part.m, condition.value),
        A => return compare(part.a, condition.value),
        S => return compare(part.s, condition.value),
    }
}

fn apply_workflow(
    part: &PartRating,
    workflow_name: &str,
    workflows: &HashMap<String, Workflow>,
) -> bool {
    let workflow = &workflows[workflow_name];
    // dbg!(workflows.keys());
    // dbg!(&workflow.rules);
    for rule in &workflow.rules {
        // dbg!(&rule.condition);
        // If we pass the rule condition, we return the result of the target workflow
        // If we fail the rule condition, we go to the next rule
        // If there is no condition, we follow the target
        if let None = rule.condition {
            match &rule.target {
                Target::Accept => return true,
                Target::Reject => return false,
                Target::GoTo(next) => return apply_workflow(part, next.as_str(), workflows),
            }
        } else if let Some(cond) = &rule.condition {
            // Check if part passes condition
            let passed = apply_condition(part, cond);
            if passed {
                // Follow target
                match &rule.target {
                    Target::Accept => return true,
                    Target::Reject => return false,
                    Target::GoTo(next) => return apply_workflow(part, next.as_str(), workflows),
                }
            } else {
                // Go to next rule
            }
        }
    }
    unreachable!();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let workflows = parse_workflows(input.split("\n\n").nth(0).unwrap());

    let parts = parse_parts(input.split("\n\n").nth(1).unwrap());

    let score = parts
        .iter()
        .filter(|part| apply_workflow(&part, "in", &workflows))
        .fold(0, |x, part| x + part.x + part.m + part.a + part.s);

    println!("Score: {}", score);
}

use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

#[derive(Debug, PartialEq, Eq)]
enum Target {
    GoTo(String),
    Accept,
    Reject,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operator {
    Gt,
    Lt,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operand {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Condition {
    operand: Operand,
    operator: Operator,
    value: u32,
}

impl Condition {
    // Determine the complement of a condition, i.e. the condition which has an opposite effect
    fn get_complement(&self) -> Condition {
        Condition {
            operator: match self.operator {
                Operator::Gt => Operator::Lt,
                Operator::Lt => Operator::Gt,
            },
            value: match self.operator {
                Operator::Gt => self.value + 1,
                Operator::Lt => self.value - 1,
            },
            operand: self.operand.clone(),
        }
    }
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
                    // dbg!(cond);
                    rule.condition = Some(parse_condition(cond));
                    // dbg!(&rule.condition);
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
                // dbg!(&rule.target);
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
    for rule in &workflow.rules {
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

// Determine how many combinations of part ratings pass all the conditions in the stack
fn compute_conditions(condition_stack: &Vec<Condition>) -> usize {
    // Combine conditions that act on the same feature (x,m,a,s) to form ranges that pass the criteria
    // In each feature, count how many options will pass all conditions
    // then multiply everything together
    use Operand::*;
    let mut total = 1;

    for feature in [X, M, A, S] {
        let mut ok_ranges: HashSet<RangeInclusive<usize>> = HashSet::new();
        ok_ranges.insert(1..=4000);
        for condition in condition_stack.iter().filter(|c| c.operand == feature) {
            // dbg!(condition);
            // Always exists since we put in 1..=4000
            let range = ok_ranges
                .iter()
                .find(|range| range.contains(&(condition.value as usize)))
                .unwrap()
                .clone();
            if condition.operator == Operator::Gt {
                ok_ranges.insert(RangeInclusive::new(
                    condition.value as usize + 1,
                    *range.end(),
                ));
            } else {
                ok_ranges.insert(RangeInclusive::new(
                    *range.start(),
                    condition.value as usize - 1,
                ));
            }
            ok_ranges.remove(&range);
        }
        for range in &ok_ranges {
            total *= range.end() - range.start() + 1;
        }
    }
    total
}

/// Determine how many combinations of part ratings are accepted by the given workflow.
/// Uses recursion when the workflow sends some parts through a different workflow in a conditional branch.
fn count_combinations_accepted_by_workflow(
    condition_stack: &Vec<Condition>,
    workflow_name: &str,
    workflows: &HashMap<String, Workflow>,
) -> usize {
    // condition_stack is a vector of conditions which apply to the initial input space
    // to reduce it into the current subspace.

    let workflow = &workflows[workflow_name];

    // Keeps track of new conditions acquired from previous rules
    let mut stack = condition_stack.clone();
    // Keeps track of the number of accepted parts from previous loop iterations
    let mut hold = 0;

    for rule in &workflow.rules {
        if let Some(cond) = &rule.condition {
            // Add to the hold the number of combinations accepted by this rule
            hold += count_accepted_by_target(
                &rule.target,
                &[stack.clone(), vec![cond.clone()]].concat(),
                workflows,
            );

            // Push the complementary condition on the stack and move to the next rule
            stack.push(cond.get_complement());
        } else {
            // No condition -> all combinations are treated the same -> return the total count in this branch
            return hold + count_accepted_by_target(&rule.target, &stack, workflows);
        }
    }

    unreachable!()
}

// Determine how many combinations are accepted by this target, depending on the current condition_stack
fn count_accepted_by_target(
    target: &Target,
    condition_stack: &Vec<Condition>,
    workflows: &HashMap<String, Workflow>,
) -> usize {
    match target {
        // All parts are acceptable -> tally up the possibilities according to the condition_stack
        Target::Accept => {
            return compute_conditions(condition_stack);
        }
        Target::Reject => {
            return 0;
        }
        Target::GoTo(next) => {
            // Recursively find the number of parts accepted by the target,
            // taking into account the current condition_stack.
            return count_combinations_accepted_by_workflow(condition_stack, next, workflows);
        }
    }
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

    // Part 2

    println!(
        "Accepted combinations: {}",
        count_combinations_accepted_by_workflow(&vec![], "in", &workflows)
    );
}
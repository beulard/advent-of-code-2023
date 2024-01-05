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

/*fn get_acceptable_parts_count(
    workflow_name: &str,
    remaining: usize,
    workflows: &HashMap<String, Workflow>,
) -> usize {
    let workflow = &workflows[workflow_name];
    // Combinations that remain after some of them have been accepted/rejected by the current rule
    let mut remaining = remaining;
    for rule in &workflow.rules {
        // Count how many parts are able to pass this rule.
        // If no condition, all parts pass -> the passable count is that of the target
        if let None = rule.condition {
            match &rule.target {
                // All parts are acceptable -> total of 4000^4 possible combinations
                Target::Accept => return remaining,
                Target::Reject => return 0,
                Target::GoTo(next) => {
                    return get_acceptable_parts_count(next.as_str(), remaining, workflows)
                }
            }
        } else if let Some(cond) = &rule.condition {
            // The count of acceptable combinations equals:
            // The number of combinations that pass the condition * the number of acceptable combinations in the target, PLUS
            // the number of combinations that fail the condition * the number of acceptable combinations in the target
            let n_pass = match cond.operator {
                Operator::Gt => (4000 - cond.value as usize) * 4000_usize.pow(3),
                Operator::Lt => (cond.value as usize - 1) * 4000_usize.pow(3),
            };

            // Combinations that pass go to the target
            // Combinations that fail go to the next rule in the workflow

            // All combinations that pass are valid
            match &rule.target {
                Target::Accept => return n_pass + (4000_usize.pow(4) - n_pass),
                Target::Reject => { // all passes continue to next rule
                }
                Target::GoTo(next) => {
                    // all passes continue to next workflow
                }
            }
            if rule.target == Target::Accept {
                return n_pass;
            } else {
            }
        }
    }
    unreachable!();
}*/

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

    // Determine how many combinations of part ratings pass all the conditions
    fn get_combinations(condition_stack: &Vec<Condition>) -> usize {
        // Combine conditions that act on the same feature (x,m,a,s) to form ranges that pass the criteria
        // In each feature, count how many options will pass all conditions
        // then multiply everything together
        use Operand::*;
        let mut total = 1;
        for feature in [X, M, A, S] {
            let mut x_ok_ranges: HashSet<RangeInclusive<usize>> = HashSet::new();
            x_ok_ranges.insert(1..=4000);
            for condition in condition_stack {
                dbg!(condition);
                if condition.operand == feature {
                    // Always exists since we put in 1..=4000
                    let range = x_ok_ranges
                        .iter()
                        .find(|range| range.contains(&(condition.value as usize)))
                        .unwrap()
                        .clone();
                    if condition.operator == Operator::Gt {
                        x_ok_ranges.insert(RangeInclusive::new(
                            condition.value as usize + 1,
                            *range.end(),
                        ));
                    } else {
                        x_ok_ranges.insert(RangeInclusive::new(
                            *range.start(),
                            condition.value as usize - 1,
                        ));
                    }
                    println!("del");
                    x_ok_ranges.remove(&range);
                }
            }
            dbg!(total);
            for range in &x_ok_ranges {
                println!(" x {}", range.end() - range.start() + 1);
                total *= range.end() - range.start() + 1;
            }
            dbg!(total);
        }
        println!("QWEQWE {}", total);
        total
    }
    // Each condition splits the input into a subspace which passes the condition and the complement, which fails it.
    // Start simple: if we have one workflow and we need to count how many of the input combinations can be accepted
    fn get_acceptable_parts_count(
        condition_stack: &Vec<Condition>,
        workflow_name: &str,
        workflows: &HashMap<String, Workflow>,
    ) -> usize {
        // condition_stack is a vector of conditions which apply to the initial input space
        // to reduce it into the current subspace.

        let workflow = &workflows[workflow_name];

        // Keeps track of new conditions acquired from previous rules
        let mut stack = condition_stack.clone();
        let mut hold = 0;

        for rule in &workflow.rules {
            // If no condition, all parts in the subspace pass -> the passable count is the number of combinations
            // which pass all the conditions in the condition_stack.
            if let None = rule.condition {
                match &rule.target {
                    // All parts are acceptable -> tally up the possibilities according to the condition_stack
                    Target::Accept => return hold + get_combinations(&stack), //remaining,
                    // All parts are rejected -> 0
                    Target::Reject => return hold + 0,
                    Target::GoTo(next) => {println!("UIUIUI"); 
                    let n_pass = get_acceptable_parts_count(&stack, next, workflows);
                    return hold + n_pass}, //get_acceptable_parts_count(next.as_str(), remaining, workflows),
                }
            } else if let Some(cond) = &rule.condition {
                // Combinations that pass go to the target
                // Combinations that fail go to the next rule in the workflow

                let mut complement = cond.clone();
                complement.operator = match cond.operator {
                    Operator::Gt => Operator::Lt,
                    Operator::Lt => Operator::Gt,
                };
                // Adjust value
                complement.value = match cond.operator {
                    Operator::Gt => cond.value + 1,
                    Operator::Lt => cond.value - 1,
                };
                let mut composite_conditions = condition_stack.clone();
                composite_conditions.push(cond.clone());
                let mut complement_conditions = condition_stack.clone();
                complement_conditions.push(complement.clone());

                // All combinations that pass all conditions are accepted, the rest goes to the next rule
                match &rule.target {
                    Target::Accept => {
                        let n_pass = get_combinations(
                            &composite_conditions,
                        );
                        hold += n_pass;
                        println!("EEEEEEEE");
                        stack.push(complement);
                    }
                    Target::Reject => {
                        // All combinations that pass this set of conditions are rejected
                        // -> Add the complement to the stack and go to the next rule
                        stack.push(complement);
                    }
                    Target::GoTo(next) => {
                        // all passes continue to next workflow
                        let n_pass = get_acceptable_parts_count(&composite_conditions, next, workflows);
                        let n_rej = get_acceptable_parts_count(&complement_conditions, next, workflows);
                        return n_pass + n_rej;
                    }
                }
                // if rule.target == Target::Accept {
                //     return n_pass;
                // } else
            }
        }

        unreachable!()
    }
    let acceptable = get_acceptable_parts_count(&mut vec![], "in", &workflows);

    println!("Total acceptable rating combinations: {}", acceptable);
}

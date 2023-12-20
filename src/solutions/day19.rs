use std::collections::HashMap;

use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 19;

async fn input(example: bool) -> (Vec<String>, Vec<String>) {
    let raw = input_raw(DAY, example).await;
    let groups: Vec<&str> = raw.split("\n\n").collect();
    let rules = groups[0]
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();
    let parts = groups[1]
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    (rules, parts)
}

struct DefaultRule(String);

#[derive(Clone)]
enum PartVar {
    X,
    M,
    A,
    S,
}
impl PartVar {
    fn from_str(input: &str) -> Self {
        match input {
            "x" => PartVar::X,
            "m" => PartVar::M,
            "a" => PartVar::A,
            "s" => PartVar::S,
            invalid => panic!("Invalid PartVar: {}", invalid),
        }
    }
}
enum Comparison {
    Gt,
    Lt,
}
impl Comparison {
    fn from_str(input: &str) -> Self {
        match input {
            ">" => Comparison::Gt,
            "<" => Comparison::Lt,
            invalid => panic!("Invalid Comparison: {}", invalid),
        }
    }
}

struct TestRule {
    var_tested: PartVar,
    number_range_inclusive: CustomRange,
    destination: String,
}
impl TestRule {
    fn from_str(input: &str) -> Self {
        //a<2006:qkq
        let (var_tested_str, rule_remaining) = input.split_at(1);
        let var_tested = PartVar::from_str(var_tested_str);
        let (comparison_str, rule_remaining) = rule_remaining.split_at(1);
        let comparison = Comparison::from_str(comparison_str);
        let remaining_split: Vec<&str> = rule_remaining.split(':').collect();
        let number = str::parse::<u64>(remaining_split[0]).unwrap();
        let destination = remaining_split[1].to_owned();

        let number_range_inclusive = match comparison {
            Comparison::Gt => CustomRange::new(number + 1, CustomRange::MAX_VALUE),
            Comparison::Lt => CustomRange::new(CustomRange::MIN_VALUE, number - 1),
        };

        Self {
            var_tested,
            number_range_inclusive,
            destination,
        }
    }

    fn test(&self, part: &Part) -> Option<String> {
        let test_value = match self.var_tested {
            PartVar::X => part.x,
            PartVar::M => part.m,
            PartVar::A => part.a,
            PartVar::S => part.s,
        };
        let test_result = self.number_range_inclusive.contains_value(test_value);

        match test_result {
            true => Some(self.destination.clone()),
            false => None,
        }
    }

    fn apply_to_super_part(&self, part: &SuperPosPart) -> Vec<SuperPosPart> {
        // let test_value_range = match self.var_tested {
        //     PartVar::X => part.x.clone(),
        //     PartVar::M => part.m.clone(),
        //     PartVar::A => part.a.clone(),
        //     PartVar::S => part.s.clone(),
        // };

        let mut pre_output: Vec<Option<SuperPosPart>> = vec![];

        // let mut new_part = part.clone();
        // new_part.location = self.destination.clone();
        let (left, remainder_result) = if self.number_range_inclusive.left > 1 {
            part.split_before(
                self.var_tested.clone(),
                self.number_range_inclusive.left - 1,
            )
        } else {
            (None, Some(part.clone()))
        };
        pre_output.push(left);
        match remainder_result {
            Some(remainder) => {
                let (mut middle, right) = remainder
                    .split_before(self.var_tested.clone(), self.number_range_inclusive.right);
                match middle.as_mut() {
                    Some(found_middle) => found_middle.location = self.destination.clone(),
                    None => {}
                }
                pre_output.push(middle);
                pre_output.push(right);
            }
            None => {}
        }

        pre_output
            .iter()
            .filter(|item| item.is_some())
            .map(|item| item.clone().unwrap())
            .collect()
    }
}

enum Rule {
    Default(DefaultRule),
    Test(TestRule),
}
impl Rule {
    fn from_str(input: &str) -> Self {
        if input.contains(':') {
            Rule::Test(TestRule::from_str(input))
        } else {
            Rule::Default(DefaultRule(input.to_owned()))
        }
    }
    fn apply_to_part(&self, part: &Part) -> Option<String> {
        match self {
            Rule::Default(default_rule) => Some(default_rule.0.clone()),
            Rule::Test(test_rule) => test_rule.test(part),
        }
    }

    fn apply_to_super_part(&self, part: &SuperPosPart) -> Vec<SuperPosPart> {
        match self {
            Rule::Default(default_rule) => {
                let mut new_part = part.clone();
                new_part.location = default_rule.0.clone();
                vec![new_part]
            }
            Rule::Test(test_rule) => test_rule.apply_to_super_part(part),
        }
    }
}

struct Workflow {
    label: String,
    rules: Vec<Rule>,
}
impl Workflow {
    fn from_str(input: &str) -> Self {
        let pos_of_left_curly = input.find('{').unwrap();
        let (label, raw_rules) = input.split_at(pos_of_left_curly);

        let (_, raw_rules) = raw_rules.split_at(1);
        let (raw_rules, _) = raw_rules.split_at(raw_rules.len() - 1);
        let raw_rules_list = raw_rules.split(',');

        let mut rules = vec![];

        for raw_rule in raw_rules_list {
            rules.push(Rule::from_str(raw_rule));
        }

        Self {
            label: label.to_owned(),
            rules,
        }
    }

    fn process_part(&self, part: &Part) -> String {
        for rule in self.rules.iter() {
            let destination_result = rule.apply_to_part(part);
            match destination_result {
                Some(destination) => return destination,
                None => {}
            }
        }

        panic!("Failed to process part");
    }

    fn process_super_part(&self, part: &SuperPosPart) -> Vec<SuperPosPart> {
        let mut parts_generated = vec![];

        for rule in self.rules.iter() {
            // TODO: this needs to split the part after rule applications,
            // creating the inverse parts where the rule could not be applied.
            // Only those inverse parts continue down the rules list.
            let mut rule_results = rule.apply_to_super_part(part);
            parts_generated.append(&mut rule_results);
        }
        parts_generated
    }
}

struct Part {
    location: String,
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}
impl Part {
    fn from_str(input: &str) -> Self {
        let regex = Regex::new(r#"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}"#).unwrap();
        let captures = regex.captures(input).unwrap();
        let x = str::parse::<u64>(captures.get(1).unwrap().as_str()).unwrap();
        let m = str::parse::<u64>(captures.get(2).unwrap().as_str()).unwrap();
        let a = str::parse::<u64>(captures.get(3).unwrap().as_str()).unwrap();
        let s = str::parse::<u64>(captures.get(4).unwrap().as_str()).unwrap();

        Self {
            location: "in".to_owned(),
            x,
            m,
            a,
            s,
        }
    }

    fn score(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug)]
struct CustomRange {
    left: u64,
    right: u64,
}
impl CustomRange {
    const MAX_VALUE: u64 = 4000;
    const MIN_VALUE: u64 = 1;
    fn new(left: u64, right: u64) -> Self {
        Self { left, right }
    }
    fn contains_value(&self, value: u64) -> bool {
        value >= self.left && value <= self.right
    }
    // fn intersection(&self, other: &Self) -> Option<Self> {
    //     if self.left == other.left {
    //         let smaller_right = if self.right < other.right {
    //             self.right
    //         } else {
    //             other.right
    //         };
    //         Some(Self::new(self.left, smaller_right))
    //     } else if self.left > other.left {
    //         other.intersection(self)
    //     } else {
    //         if other.left > self.right {
    //             None
    //         } else {
    //             let smaller_right = if self.right < other.right {
    //                 self.right
    //             } else {
    //                 other.right
    //             };
    //             Some(Self::new(other.left, smaller_right))
    //         }
    //     }
    // }
}
impl Default for CustomRange {
    fn default() -> Self {
        Self {
            left: Self::MIN_VALUE,
            right: Self::MAX_VALUE,
        }
    }
}

#[derive(Clone, Debug)]
struct SuperPosPart {
    location: String,
    x: CustomRange,
    m: CustomRange,
    a: CustomRange,
    s: CustomRange,
}
impl SuperPosPart {
    // super_part.split_before(PartVar::x, 2000) => ({x = 0..=1999} {x = 2000..=4000})
    fn split_before(
        &self,
        var_considered: PartVar,
        split_before_number: u64,
    ) -> (Option<Self>, Option<Self>) {
        assert!(split_before_number > 0);
        let mut left = self.clone();
        let mut right = self.clone();

        match var_considered {
            PartVar::X => {
                if self.x.contains_value(split_before_number) {
                    return (None, None);
                } else if self.x.left == split_before_number {
                    return (None, Some(self.clone()));
                } else if self.x.right + 1 == split_before_number {
                    return (Some(self.clone()), None);
                }
                left.x.left = split_before_number - 1;
                right.x.right = split_before_number;
            }
            PartVar::M => {
                if self.m.contains_value(split_before_number) {
                    return (None, None);
                } else if self.m.left == split_before_number {
                    return (None, Some(self.clone()));
                } else if self.m.right + 1 == split_before_number {
                    return (Some(self.clone()), None);
                }
                left.m.left = split_before_number - 1;
                right.m.right = split_before_number;
            }
            PartVar::A => {
                if self.a.contains_value(split_before_number) {
                    return (None, None);
                } else if self.a.left == split_before_number {
                    return (None, Some(self.clone()));
                } else if self.a.right + 1 == split_before_number {
                    return (Some(self.clone()), None);
                }
                left.a.left = split_before_number - 1;
                right.a.right = split_before_number;
            }
            PartVar::S => {
                if self.s.contains_value(split_before_number) {
                    return (None, None);
                } else if self.s.left == split_before_number {
                    return (None, Some(self.clone()));
                } else if self.s.right + 1 == split_before_number {
                    return (Some(self.clone()), None);
                }
                left.s.left = split_before_number - 1;
                right.s.right = split_before_number;
            }
        }

        (Some(left), Some(right))
    }
    fn paths_to_acceptance(&self, workflows: &HashMap<String, Workflow>) -> u64 {
        println!("Calculating acceptance:\n{:?}\n", self);
        match self.location.as_str() {
            "A" => return self.positions(),
            "R" => return 0,
            _ => {}
        }

        let relevant_workflow = workflows.get(&self.location).unwrap();

        let spawned_parts = relevant_workflow.process_super_part(self);

        let mut accum = 0;
        for part in spawned_parts {
            accum += part.paths_to_acceptance(workflows);
        }
        accum
    }
    fn positions(&self) -> u64 {
        ((self.x.left + 1) - self.x.right)
            + ((self.m.left + 1) - self.m.right)
            + ((self.a.left + 1) - self.a.right)
            + ((self.s.left + 1) - self.s.right)
    }
}
impl Default for SuperPosPart {
    fn default() -> Self {
        Self {
            location: "in".to_owned(),
            x: Default::default(),
            m: Default::default(),
            a: Default::default(),
            s: Default::default(),
        }
    }
}

// =============================================================================
// ENTRY POINTS
// =============================================================================

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";
const END_STATES: [&str; 2] = [ACCEPTED, REJECTED];

pub async fn d19s1(submit: bool, example: bool) {
    let (workflow_raw_lines, part_raw_lines) = input(example).await;

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    for workflow_raw in workflow_raw_lines {
        let workflow = Workflow::from_str(&workflow_raw);
        workflows.insert(workflow.label.clone(), workflow);
    }

    for part_raw in part_raw_lines {
        let part = Part::from_str(&part_raw);
        parts.push(part);
    }

    let mut total_score = 0;

    for mut part in parts {
        while !END_STATES.contains(&part.location.as_str()) {
            let workflow = workflows.get(&part.location).unwrap();
            let destination = workflow.process_part(&part);
            part.location = destination;
        }
        if part.location == ACCEPTED {
            total_score += part.score();
        }
    }

    final_answer(total_score, submit, DAY, 1).await;
}

pub async fn d19s2(submit: bool, example: bool) {
    let (workflow_raw_lines, _) = input(example).await;
    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    for workflow_raw in workflow_raw_lines {
        let workflow = Workflow::from_str(&workflow_raw);
        workflows.insert(workflow.label.clone(), workflow);
    }

    let super_part = SuperPosPart::default();

    let answer = super_part.paths_to_acceptance(&workflows);

    final_answer(answer, submit, DAY, 2).await;
}

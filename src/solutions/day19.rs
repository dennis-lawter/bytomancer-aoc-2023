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
    comparison: Comparison,
    number: u64,
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

        Self {
            var_tested,
            comparison,
            number,
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
        let test_result = match self.comparison {
            Comparison::Gt => test_value > self.number,
            Comparison::Lt => test_value < self.number,
        };

        match test_result {
            true => Some(self.destination.clone()),
            false => None,
        }
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
    let (_, _) = input(example).await;
    final_answer("NaN", submit, DAY, 2).await;
}

use std::collections::HashMap;

use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 8;

async fn input(example: bool) -> (Vec<char>, HashMap<String, Node>) {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    let directions: Vec<char> = lines[0].chars().collect();
    let nodes: Vec<Node> = lines
        .iter()
        .skip(1)
        .map(|item| Node::from_str(item.as_str()))
        .collect();
    println!("{:?}", nodes);
    let mut nodes_map: HashMap<String, Node> = HashMap::with_capacity(nodes.len());
    for node in nodes {
        nodes_map.insert(
            node.location.clone(),
            Node::new(node.location, node.left, node.right),
        );
    }

    (directions, nodes_map)
}

#[derive(Clone, Debug)]
struct Node {
    location: String,
    left: String,
    right: String,
}
impl Node {
    fn from_str(input: &str) -> Self {
        let regex = Regex::new(r#"(.+) = \((.+), (.+)\)"#).unwrap();
        let captures = regex.captures(input).unwrap();
        let location = captures.get(1).unwrap().as_str().to_owned();
        let left = captures.get(2).unwrap().as_str().to_owned();
        let right = captures.get(3).unwrap().as_str().to_owned();
        Self::new(location, left, right)
    }
    fn new(location: String, left: String, right: String) -> Self {
        Self {
            location,
            left,
            right,
        }
    }
}

fn solve_steps(
    start_node_loc: &str,
    goal_node_locs: &[String],
    nodes_map: &HashMap<String, Node>,
    directions: &Vec<char>,
) -> usize {
    let mut steps = 0;
    let mut curr_node_loc = start_node_loc.to_owned();
    while !goal_node_locs.contains(&curr_node_loc) {
        let curr_node = nodes_map[&curr_node_loc].clone();
        match directions[steps % directions.len()] {
            'L' => curr_node_loc = curr_node.left,
            'R' => curr_node_loc = curr_node.right,
            _ => {}
        }
        steps += 1;
    }

    steps
}

pub async fn d08s1(submit: bool, example: bool) {
    let (directions, nodes_map) = input(example).await;

    let curr_node_loc = "AAA".to_owned();
    let goal_node_locs = vec!["ZZZ".to_owned()];
    let steps = solve_steps(&curr_node_loc, &goal_node_locs, &nodes_map, &directions);

    final_answer(steps, submit, DAY, 1).await;
}

pub async fn d08s2(submit: bool, example: bool) {
    let (directions, nodes_map) = input(example).await;

    let mut curr_node_locs: Vec<String> = vec![];
    for key in nodes_map.keys() {
        if key.ends_with("A") {
            curr_node_locs.push(key.clone())
        }
    }

    let mut goal_node_locs: Vec<String> = vec![];
    for key in nodes_map.keys() {
        if key.ends_with("Z") {
            goal_node_locs.push(key.clone())
        }
    }

    let mut step_counts: Vec<usize> = vec![];
    for node_loc in curr_node_locs {
        let steps = solve_steps(&node_loc, &goal_node_locs, &nodes_map, &directions);
        step_counts.push(steps);
    }

    let mut steps_lcm = step_counts[0];
    for step_count in step_counts {
        steps_lcm = num::integer::lcm(steps_lcm, step_count);
    }

    final_answer(steps_lcm, submit, DAY, 2).await;
}

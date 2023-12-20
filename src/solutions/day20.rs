use std::collections::HashMap;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 20;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
struct BroadcasterModule {
    label: String,
    outputs: Vec<String>,
}
impl BroadcasterModule {
    fn from_str(input: &str) -> Self {
        let strip_len = "broadcaster -> ".len();
        let (_, connections_raw) = input.split_at(strip_len);
        let connections = connections_raw
            .split(", ")
            .map(|item| item.to_owned())
            .collect();

        Self {
            label: "broadcaster".to_owned(),
            outputs: connections,
        }
    }
}

#[derive(Debug)]
struct FlipFlopModule {
    label: String,
    on: bool,
    outputs: Vec<String>,
}
impl FlipFlopModule {
    fn from_str(input: &str) -> Self {
        // strip first char
        let (_, input_raw) = input.split_at(1);
        let (label_raw, connections_raw) =
            input_raw.split_at(input_raw.find(" -> ").unwrap() + " -> ".len());

        let connections = connections_raw
            .split(", ")
            .map(|item| item.to_owned())
            .collect();

        Self {
            label: label_raw.to_owned(),
            on: false,
            outputs: connections,
        }
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    label: String,
    memory: HashMap<String, Pulse>,
    outputs: Vec<String>,
}
impl ConjunctionModule {
    fn from_str(input: &str) -> Self {
        // strip first char
        let (_, input_raw) = input.split_at(1);
        let (label_raw, connections_raw) =
            input_raw.split_at(input_raw.find(" -> ").unwrap() + " -> ".len());

        let connections: Vec<String> = connections_raw
            .split(", ")
            .map(|item| item.to_owned())
            .collect();

        let mut memory = HashMap::with_capacity(connections.len());

        for conn in &connections {
            memory.insert(conn.clone(), Pulse::Low);
        }

        Self {
            label: label_raw.to_owned(),
            memory,
            outputs: connections,
        }
    }
}

// #[derive(Debug)]
// struct NoOpModule {
//     label: String,
// }

const FLIP_FLOP_SYMBOL: char = '%';
const CONJUNCTION_SYMBOL: char = '&';

#[derive(Debug)]
enum Module {
    Broadcaster(BroadcasterModule),
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    // NoOp(NoOpModule),
}
impl Module {
    fn from_str(input: &str) -> Self {
        if input.starts_with("broadcaster") {
            Module::Broadcaster(BroadcasterModule::from_str(input))
        } else {
            match input.chars().nth(0).unwrap() {
                FLIP_FLOP_SYMBOL => Module::FlipFlop(FlipFlopModule::from_str(input)),
                CONJUNCTION_SYMBOL => Module::Conjunction(ConjunctionModule::from_str(input)),
                unrecognized => panic!("Unrecognized module identifier: {}", unrecognized),
            }
        }
    }
    fn get_label(&self) -> String {
        match self {
            Module::Broadcaster(m) => m.label.clone(),
            Module::FlipFlop(m) => m.label.clone(),
            Module::Conjunction(m) => m.label.clone(),
        }
    }
}

fn lines_to_modules(lines: &Vec<String>) -> HashMap<String, Module> {
    let mut modules = HashMap::with_capacity(lines.len());

    for line in lines {
        let module = Module::from_str(line.as_str());
        modules.insert(module.get_label(), module);
    }

    modules
}

// =============================================================================
// ENTRY POINTS
// =============================================================================

pub async fn d20s1(submit: bool, example: bool) {
    let input = input(example).await;
    let modules = lines_to_modules(&input);
    println!("MODULES:\n");
    for (_, module) in &modules {
        println!("{:?}", module);
    }
    final_answer(input[0].to_owned(), submit, DAY, 1).await;
}

pub async fn d20s2(submit: bool, example: bool) {
    let input = input(example).await;
    final_answer(input[0].to_owned(), submit, DAY, 2).await;
}

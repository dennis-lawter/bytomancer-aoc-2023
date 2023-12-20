use std::collections::HashMap;
use std::collections::VecDeque;

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

#[derive(Debug, Clone)]
struct Packet {
    from: String,
    dest: String,
    pulse: Pulse,
}
impl Packet {
    fn new(from: String, dest: String, pulse: Pulse) -> Packet {
        Self { from, dest, pulse }
    }
}

#[derive(Debug, Clone)]
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

    fn has_output(&self, to: &String) -> bool {
        self.outputs.contains(to)
    }

    fn send_packet(&mut self, _packet: &Packet) -> VecDeque<Packet> {
        let mut packets = VecDeque::new();
        for output in &self.outputs {
            packets.push_back(Packet::new(self.label.clone(), output.clone(), Pulse::Low));
        }

        packets
    }
}

#[derive(Debug, Clone)]
struct FlipFlopModule {
    label: String,
    on: bool,
    // inputs: Vec<String>,
    outputs: Vec<String>,
}
impl FlipFlopModule {
    const DEFAULT_STATE: bool = false;
    fn from_str(input: &str) -> Self {
        // strip first char
        let (_, input_raw) = input.split_at(1);
        // let (label_raw, connections_raw) =
        //     input_raw.split_at(input_raw.find(" -> ").unwrap() + " -> ".len());
        let input_raw_split: Vec<&str> = input_raw.split(" -> ").collect();
        let label_raw = input_raw_split[0];
        let connections_raw = input_raw_split[1];

        let connections = connections_raw
            .split(", ")
            .map(|item| item.to_owned())
            .collect();

        Self {
            label: label_raw.to_owned(),
            on: Self::DEFAULT_STATE,
            // inputs: vec![],
            outputs: connections,
        }
    }

    fn has_output(&self, to: &String) -> bool {
        self.outputs.contains(to)
    }

    fn send_packet(&mut self, packet: &Packet) -> VecDeque<Packet> {
        let mut packets = VecDeque::new();

        match packet.pulse {
            Pulse::Low => {
                self.on = !self.on;
                match self.on {
                    true => {
                        for output in &self.outputs {
                            packets.push_back(Packet::new(
                                self.label.clone(),
                                output.clone(),
                                Pulse::High,
                            ));
                        }
                    }
                    false => {
                        for output in &self.outputs {
                            packets.push_back(Packet::new(
                                self.label.clone(),
                                output.clone(),
                                Pulse::Low,
                            ));
                        }
                    }
                }
            }
            Pulse::High => {}
        }

        packets
    }

    // fn add_inputs(&mut self, inputs: &Vec<String>) {
    //     self.inputs = inputs.clone()
    // }
}

#[derive(Debug, Clone)]
struct ConjunctionModule {
    label: String,
    memory: HashMap<String, Pulse>,
    // inputs: Vec<String>,
    outputs: Vec<String>,
}
impl ConjunctionModule {
    const DEFAULT_STATE: Pulse = Pulse::Low;
    fn from_str(input: &str) -> Self {
        // strip first char
        let (_, input_raw) = input.split_at(1);
        // let (label_raw, connections_raw) =
        //     input_raw.split_at(input_raw.find(" -> ").unwrap() + " -> ".len());
        let input_raw_split: Vec<&str> = input_raw.split(" -> ").collect();
        let label_raw = input_raw_split[0];
        let connections_raw = input_raw_split[1];

        let connections: Vec<String> = connections_raw
            .split(", ")
            .map(|item| item.to_owned())
            .collect();

        let memory = HashMap::new();

        Self {
            label: label_raw.to_owned(),
            memory,
            // inputs: vec![],
            outputs: connections,
        }
    }

    fn has_output(&self, to: &String) -> bool {
        self.outputs.contains(to)
    }

    fn add_input(&mut self, input: &String) {
        self.memory.insert(input.clone(), Self::DEFAULT_STATE);
    }

    fn send_packet(&mut self, packet: &Packet) -> VecDeque<Packet> {
        let mut packets = VecDeque::new();

        self.memory.insert(packet.from.clone(), packet.pulse);

        // println!("\t{:?}", self.memory);

        let mut remembers_only_high = true;
        for (_, mem_pulse) in &self.memory {
            match mem_pulse {
                Pulse::Low => {
                    remembers_only_high = false;
                }
                Pulse::High => {}
            }
        }

        match remembers_only_high {
            true => {
                for output in &self.outputs {
                    packets.push_back(Packet::new(self.label.clone(), output.clone(), Pulse::Low));
                }
            }
            false => {
                for output in &self.outputs {
                    packets.push_back(Packet::new(self.label.clone(), output.clone(), Pulse::High));
                }
            }
        }

        packets
    }
}

#[derive(Debug, Clone)]
struct RxModule {
    label: String,
    machine_activated: bool,
}
impl RxModule {
    fn new() -> Self {
        Self {
            label: "rx".to_owned(),
            machine_activated: false,
        }
    }

    fn has_output(&self, _to: &String) -> bool {
        false
    }

    fn send_packet(&mut self, packet: &Packet) -> VecDeque<Packet> {
        match packet.pulse {
            Pulse::Low => {
                self.machine_activated = true;
            }
            Pulse::High => {}
        }
        VecDeque::new()
    }
}

const FLIP_FLOP_SYMBOL: char = '%';
const CONJUNCTION_SYMBOL: char = '&';

#[derive(Debug, Clone)]
enum Module {
    Broadcaster(BroadcasterModule),
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Rx(RxModule),
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
            Module::Rx(m) => m.label.clone(),
        }
    }
    fn has_output(&self, to: &String) -> bool {
        match self {
            Module::Broadcaster(m) => m.has_output(to),
            Module::FlipFlop(m) => m.has_output(to),
            Module::Conjunction(m) => m.has_output(to),
            Module::Rx(m) => m.has_output(to),
        }
    }
    fn send_packet(&mut self, packet: &Packet) -> VecDeque<Packet> {
        match self {
            Module::Broadcaster(m) => m.send_packet(packet),
            Module::FlipFlop(m) => m.send_packet(packet),
            Module::Conjunction(m) => m.send_packet(packet),
            Module::Rx(m) => m.send_packet(packet),
        }
    }
}

fn lines_to_modules(lines: &Vec<String>) -> HashMap<String, Module> {
    let mut modules = HashMap::with_capacity(lines.len());

    for line in lines {
        let module = Module::from_str(line.as_str());
        modules.insert(module.get_label(), module);
    }

    let modules_clone = modules.clone();

    for (_, module) in &mut modules {
        match module {
            Module::Broadcaster(_) | Module::FlipFlop(_) | Module::Rx(_) => {}
            Module::Conjunction(con_mod) => {
                for (other_label, other_module) in &modules_clone {
                    if other_module.has_output(&con_mod.label) {
                        con_mod.add_input(other_label);
                    }
                }
            }
        }
    }

    modules
}

fn press_the_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    // println!("==================================================");
    // println!("Button pressed");
    // println!("==================================================");
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let button_press_packet =
        Packet::new("button".to_owned(), "broadcaster".to_owned(), Pulse::Low);

    let mut packets = VecDeque::new();
    packets.push_back(button_press_packet);

    // let mut packets = {
    //     let broadcaster = modules.get_mut("broadcaster").unwrap();
    //     broadcaster.send_packet(&button_press_packet)
    // };
    while !packets.is_empty() {
        let packet = packets.pop_front().unwrap();

        // println!("--------------------------------------------------");
        // println!("{:?}", packet);
        // println!("--------------------------------------------------");

        // count the pulse
        match packet.pulse {
            Pulse::Low => low_pulses += 1,
            Pulse::High => high_pulses += 1,
        }
        // propogate the pulse
        let mut new_packets = {
            let receiver_result = modules.get_mut(&packet.dest);
            match receiver_result {
                Some(receiver) => receiver.send_packet(&packet),
                None => VecDeque::new(),
            }
        };
        // append new pulses at the end of queue
        packets.append(&mut new_packets);
    }
    // println!("==================================================\n");

    (low_pulses, high_pulses)
}

fn rx_machine_activated(modules: &HashMap<String, Module>) -> bool {
    let rx_module_wrap = modules.get("rx").unwrap();
    match rx_module_wrap {
        Module::Rx(rx_module) => rx_module.machine_activated,
        _ => panic!("No rx module found."),
    }
}

// =============================================================================
// ENTRY POINTS
// =============================================================================

pub async fn d20s1(submit: bool, example: bool) {
    let input = input(example).await;
    let mut modules = lines_to_modules(&input);

    println!("==================================================");
    println!("MODULES:");
    println!("==================================================");
    for (_, module) in &modules {
        println!("{:?}", module);
    }
    println!("==================================================\n");

    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (new_low, new_high) = press_the_button(&mut modules);
        low += new_low;
        high += new_high;
    }

    println!("==================================================");
    println!("PULSES:");
    println!("==================================================");
    println!("LOW:     {}", low);
    println!("HIGH:    {}", high);
    println!("==================================================\n");

    final_answer(low * high, submit, DAY, 1).await;
}

pub async fn d20s2(submit: bool, example: bool) {
    let input = input(example).await;
    let mut modules = lines_to_modules(&input);
    modules.insert("rx".to_owned(), Module::Rx(RxModule::new()));

    println!("==================================================");
    println!("MODULES:");
    println!("==================================================");
    for (_, module) in &modules {
        println!("{:?}", module);
    }
    println!("==================================================\n");

    let mut press_counts: usize = 0;
    while !rx_machine_activated(&modules) {
        press_the_button(&mut modules);
        press_counts += 1;
        if press_counts % 1_000_000 == 0 {
            println!("Million presses:\t{}", press_counts / 1_000_000);
        }
    }

    final_answer(press_counts, submit, DAY, 2).await;
}

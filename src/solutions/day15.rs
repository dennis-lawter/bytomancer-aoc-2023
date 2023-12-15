use super::final_answer;
use super::input_raw;

const DAY: u8 = 15;

async fn input_as_u8s(example: bool) -> Vec<Vec<u8>> {
    let raw = input_raw(DAY, example).await;
    let groups: Vec<String> = raw
        .replace("\n", "")
        .split(',')
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    let mut output = vec![];
    for group in groups {
        output.push(group.chars().map(|item| item as u8).collect());
    }

    output
}

enum Instruction {
    Remove(RemoveInstruction),
    Insert(InsertInstruction),
}

struct RemoveInstruction {
    label: String,
}
impl RemoveInstruction {
    fn to_string(&self) -> String {
        format!("{}-", self.label)
    }
}

#[derive(Clone)]
struct InsertInstruction {
    label: String,
    focal_strength: u8,
}
impl InsertInstruction {
    fn to_string(&self) -> String {
        format!("{}={}", self.label, self.focal_strength)
    }
}

async fn input_as_instructions(example: bool) -> Vec<Instruction> {
    let raw = input_raw(DAY, example).await;
    let groups: Vec<String> = raw
        .replace("\n", "")
        .split(',')
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    let mut output = vec![];

    for group in groups {
        match group.find('-') {
            Some(matched_usize) => {
                let (label_str, _) = group.split_at(matched_usize);
                output.push(Instruction::Remove(RemoveInstruction {
                    label: label_str.to_owned(),
                }));
            }
            None => match group.find('=') {
                Some(_) => {
                    let splits: Vec<&str> = group.split("=").collect();
                    let label = splits[0].to_owned();
                    let focal_strength = str::parse::<u8>(splits[1]).unwrap();
                    output.push(Instruction::Insert(InsertInstruction {
                        label,
                        focal_strength,
                    }));
                }
                None => todo!(),
            },
        }
    }

    output
}

const MULT_BY: u64 = 17;
const MOD_BY: u64 = 256;

fn hash(current_value: u64, val: &u8) -> u64 {
    let mut current_value = current_value;
    current_value += *val as u64;
    current_value *= MULT_BY;
    current_value %= MOD_BY;

    current_value
}
fn hash_string(current_value: u64, values: &String) -> u64 {
    let mut current_value = current_value;

    for c in values.as_bytes() {
        current_value = hash(current_value, &c);
    }

    current_value
}

pub async fn d15s1(submit: bool, example: bool) {
    let input = input_as_u8s(example).await;
    let mut accum = 0;
    let mut current_value = 0;
    for group in input {
        for c in group {
            current_value = hash(current_value, &c);
        }
        accum += current_value;
        current_value = 0;
    }
    final_answer(accum, submit, DAY, 1).await;
}

struct BoxList {
    boxes: Vec<Vec<(String, u8)>>,
}
impl BoxList {
    fn new() -> Self {
        let mut boxes: Vec<Vec<(String, u8)>> = Vec::with_capacity(256);
        for _ in 0..256 {
            boxes.push(vec![]);
        }
        Self { boxes }
    }
    fn remove_by_label(&mut self, box_id: usize, label: &String) {
        for j in 0..self.boxes[box_id].len() {
            if self.boxes[box_id][j].0 == *label {
                self.boxes[box_id].remove(j);
                return;
            }
        }
    }
    fn insert(&mut self, box_id: usize, ins: InsertInstruction) {
        let label = ins.label.clone();
        for j in 0..self.boxes[box_id].len() {
            if self.boxes[box_id][j].0 == *label {
                self.boxes[box_id][j] = (ins.label, ins.focal_strength);
                return;
            }
        }
        self.boxes[box_id].push((ins.label, ins.focal_strength));
    }
    fn score(&self) -> u64 {
        let mut accum = 0;
        for i in 0..self.boxes.len() {
            for j in 0..self.boxes[i].len() {
                let lens = &self.boxes[i][j];
                accum += (i as u64 + 1) * (j as u64 + 1) * lens.1 as u64;
            }
        }

        accum
    }
    fn debug(&self) {
        for i in 0..self.boxes.len() {
            if self.boxes[i].len() > 0 {
                print!("BOX {}: ", i);
                for j in 0..self.boxes[i].len() {
                    print!("[{} {}] ", self.boxes[i][j].0, self.boxes[i][j].1);
                }
                println!();
            }
        }
    }
}

pub async fn d15s2(submit: bool, example: bool) {
    let input = input_as_instructions(example).await;
    let mut box_list = BoxList::new();
    for instruction in input {
        let mut current_value = 0;
        match instruction {
            Instruction::Remove(rm) => {
                let label = rm.label.clone();
                current_value = hash_string(current_value, &label);
                box_list.remove_by_label(current_value as usize, &label);

                println!("\nAfter \"{}\":", rm.to_string());
                box_list.debug();
            }
            Instruction::Insert(ins) => {
                let label = ins.label.clone();
                current_value = hash_string(current_value, &label);
                box_list.insert(current_value as usize, ins.clone());

                println!("\nAfter \"{}\":", ins.to_string());
                box_list.debug();
            }
        }
    }
    let score = box_list.score();
    final_answer(score, submit, DAY, 2).await;
}

use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 6;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}
impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }
}

async fn p1_input(example: bool) -> Vec<Race> {
    let raw = input_raw(DAY, example).await;
    let regex = Regex::new(r#"  +"#).unwrap();
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .map(|item| -> String { regex.replace_all(&item, " ").to_string() })
        .filter(|item| item.len() > 0)
        .collect();
    let times: Vec<u64> = lines[0]
        .split(" ")
        .skip(1)
        .map(|item| str::parse::<u64>(item).unwrap())
        .collect();
    let distances: Vec<u64> = lines[1]
        .split(" ")
        .skip(1)
        .map(|item| str::parse::<u64>(item).unwrap())
        .collect();
    let mut races = vec![];
    for i in 0..times.len() {
        races.push(Race::new(times[i], distances[i]));
    }

    races
}

pub async fn d06s1(submit: bool, example: bool) {
    let races = p1_input(example).await;
    println!("Races: {:?}", races);
    let mut race_results: Vec<u64> = Vec::with_capacity(races.len());
    for race in races {
        let num_winner = cheat_race(race);
        race_results.push(num_winner);
    }
    let answer: u64 = race_results.iter().product();
    final_answer(answer, submit, DAY, 1).await;
}

fn cheat_race(race: Race) -> u64 {
    let mut num_winner = 0;
    for speed in 1..race.time {
        let time_traveling = race.time - speed;
        if time_traveling * speed > race.distance {
            num_winner += 1;
        }
    }
    num_winner
}

async fn p2_input(example: bool) -> Race {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .replace(" ", "")
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();
    let time =
        str::parse::<u64>(lines[0].split(":").collect::<Vec<&str>>().get(1).unwrap()).unwrap();
    let distance =
        str::parse::<u64>(lines[1].split(":").collect::<Vec<&str>>().get(1).unwrap()).unwrap();
    Race::new(time, distance)
}

pub async fn d06s2(submit: bool, example: bool) {
    let race = p2_input(example).await;
    final_answer(cheat_race(race), submit, DAY, 2).await;
}

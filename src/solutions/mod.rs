use colored::Colorize;
use regex::Regex;
use reqwest::Url;

use crate::input::get_example_as_string;
use crate::input::get_input_as_string;

// solutions
pub mod day00;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07s1;
pub mod day07s2;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

// revisions
pub mod day05rev;

// visualizations

pub async fn input_raw(day: u8, example: bool) -> String {
    if example {
        get_example_as_string(day).await
    } else {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            crate::prelude::YEAR,
            day
        )
        .to_string();
        get_input_as_string(&url).await
    }
}

pub async fn final_answer<T: std::fmt::Display>(answer: T, submit: bool, day: u8, level: u8) {
    println!(
        "\n{}",
        format!(
            "   Solution {}",
            format!(" {} ", answer).black().on_yellow().bold()
        )
        .bold()
        .on_blue()
    );

    if submit {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/answer",
            crate::prelude::YEAR,
            day
        );
        let request = format!("level={}&answer={}", level, answer);
        let response = perform_submit(&url, request).await;

        if response.contains("day-success") {
            println!("{}", "Accepted!".bold().on_blue());
        } else if response.contains("Did you already complete it?") {
            println!("{}", "Solution already accepted...".bold().on_white());
        } else if response.contains("left to wait.") {
            // You have 13s left to wait.
            let time_capture_regex = Regex::new(r"You have (.+) left to wait.").unwrap();
            let captures_result = time_capture_regex.captures(&response);
            println!("{}", "    SLOW DOWN    ".bold().on_red());
            match captures_result {
                Some(captures) => {
                    println!(
                        "Please wait {}.",
                        format!("{}", captures.get(1).unwrap().as_str())
                            .bold()
                            .on_red()
                    );
                }
                None => {
                    println!("Could not determine time before next submission...");
                }
            }
        } else {
            println!("{}", "Innaccurate!".bold().on_bright_red());
        }
    }
    println!();
}

async fn perform_submit(submit_url: &String, body: String) -> String {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    const KEY: &str = "SESSION";
    let session = dotenv::var(KEY).unwrap();
    let cookie = format!("session={}", session);
    let url = submit_url.parse::<Url>().unwrap();

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let response = client
        .post(url)
        .header("cookie", cookie)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();

    body
}

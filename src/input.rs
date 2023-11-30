use std::fs::File;
use std::fs::{self};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

use dotenv;
use reqwest::Url;

const MATCH_PATTERN_FOR_EXAMPLE_IN_HTML: &str =
    r#"(?m)(?s)[Ff]or example[:,].*<pre><code>(.+)<\/code><\/pre>"#;

pub async fn get_example_from_the_web(day: u8) -> String {
    let url = format!(
        "https://adventofcode.com/{}/day/{}",
        crate::prelude::YEAR,
        day
    );
    let session = dotenv::var(crate::prelude::ENV_KEY).expect(
        format!(
            "Please provide a .env with the {} variable",
            crate::prelude::ENV_KEY
        )
        .as_str(),
    );
    let cookie = format!("session={}", session);

    let client = reqwest::Client::builder()
        .user_agent(crate::prelude::APP_USER_AGENT)
        .build()
        .unwrap();

    let response = client
        .get(url)
        .header("cookie", cookie)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();

    let regex = regex::Regex::new(MATCH_PATTERN_FOR_EXAMPLE_IN_HTML).unwrap();
    let captures_result = regex.captures(&body);
    match captures_result {
        Some(captures) => String::from(captures.get(1).unwrap().as_str()),
        None => panic!("Could not find a valid example block"),
    }
}

pub async fn get_input_as_string(input_url: &str) -> String {
    let url = input_url.to_string();
    match get_file_path_from_cache(&url) {
        Some(path) => match get_input_as_string_from_cache(&path) {
            Ok(result) => result.trim_end().to_string(),
            Err(_) => get_input_as_string_from_site(&url)
                .await
                .trim_end()
                .to_string(),
        },
        None => get_input_as_string_from_site(&url)
            .await
            .trim_end()
            .to_string(),
    }
}

const URL_PREFIX: &str = "https://adventofcode.com/";

fn get_input_as_string_from_cache(path: &String) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub async fn get_example_as_string(day: u8) -> String {
    match get_example_path_from_cache(day) {
        Some(path) => get_input_as_string_from_cache(&path)
            .expect("An issue occurred reading the example.txt"),
        None => {
            let example_body = get_example_from_the_web(day).await;
            let example_path = example_path(day);
            write_locally(&example_path, &example_body)
                .expect("Error writing the example to the cache.");
            example_body
        }
    }
}

fn example_path(day: u8) -> String {
    format!(
        "{}{}{}{}{}",
        "_cache/",
        crate::prelude::YEAR,
        "/day/",
        day,
        "/example.txt"
    )
}

fn get_example_path_from_cache(day: u8) -> Option<String> {
    let path = example_path(day);
    match File::open(path.clone()) {
        Ok(_) => Some(path),
        Err(_) => None,
    }
}

fn get_path_from_input_url(url: &String) -> String {
    let url_postfix = url
        .clone()
        .strip_prefix(URL_PREFIX)
        .expect("Invalid domain.")
        .to_string();
    format!("{}{}{}", "_cache/", url_postfix, ".txt")
}

fn write_new_input_locally(url: &String, input: &String) -> Result<(), std::io::Error> {
    let path = get_path_from_input_url(url);

    write_locally(&path, input)
}

fn write_locally(path: &String, input: &String) -> Result<(), std::io::Error> {
    let path_obj = Path::new(&path);
    println!("CACHE PATH: {}", path);
    let parent =
        Path::parent(path_obj).ok_or(std::io::Error::new(ErrorKind::Other, "Invalid path."))?;
    fs::create_dir_all(parent)?;
    let mut file = File::create(path.clone())?;
    file.write_all(input.as_bytes())?;

    Ok(())
}

fn get_file_path_from_cache(input_url: &String) -> Option<String> {
    let path = get_path_from_input_url(input_url);
    match File::open(path.clone()) {
        Ok(_) => Some(path),
        Err(_) => None,
    }
}

async fn get_input_as_string_from_site(input_url: &String) -> String {
    let session = dotenv::var(crate::prelude::ENV_KEY).expect(
        format!(
            "Please provide a .env with the {} variable",
            crate::prelude::ENV_KEY
        )
        .as_str(),
    );
    let cookie = format!("session={}", session);
    let url = input_url.parse::<Url>().unwrap();

    let client = reqwest::Client::builder()
        .user_agent(crate::prelude::APP_USER_AGENT)
        .build()
        .unwrap();

    let response = client
        .get(url)
        .header("cookie", cookie)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();
    write_new_input_locally(input_url, &body).expect("An error occurred while writing the cache.");

    body
}

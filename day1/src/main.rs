use reqwest::header;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let session = format!("session={}", env::var("AOC_SESSION")?);
    let mut headers = header::HeaderMap::new();
    headers.insert(header::COOKIE, header::HeaderValue::from_str(&session)?);
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let input = client
        .get("https://adventofcode.com/2022/day/1/input")
        .send()?
        .text()?;

    let answer: i32 = input
        .split("\n\n")
        .map(|s| s.split('\n').map(|s| s.parse::<i32>().unwrap_or(0)).sum())
        .max()
        .unwrap();
    println!("{}", answer);
    Ok(())
}

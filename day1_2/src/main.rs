use reqwest::header;
use std::collections::BinaryHeap;
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

    let result: u64 = input
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .sum()
        })
        .collect::<BinaryHeap<u64>>()
        .iter()
        .take(3)
        .sum();
    println!("{:?}", result);
    Ok(())
}

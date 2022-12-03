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
        .get("https://adventofcode.com/2022/day/2/input")
        .send()?
        .text()?;
    let result: i32 = input
        .split('\n')
        .map(|s| match s {
            "A X" => 3,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            "" => 0,
            _ => unreachable!(),
        })
        .sum();
    println!("{:?}", result);
    Ok(())
}

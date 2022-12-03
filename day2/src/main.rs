use itertools::Itertools;
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
    println!("{}", input);

    let result = input.split_whitespace();
    let result: i32 = result
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            chunk.collect_tuple().map_or(0, |(a, b)| match (a, b) {
                ("A", "X") => 1 + 3,
                ("A", "Y") => 2 + 6,
                ("A", "Z") => 3,
                ("B", "X") => 1,
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,
                ("C", "X") => 1 + 6,
                ("C", "Y") => 2,
                ("C", "Z") => 3 + 3,
                _ => unreachable!(),
            })
        })
        .sum();
    println!("{:?}", result);
    Ok(())
}

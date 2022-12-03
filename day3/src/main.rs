use reqwest::header;
use std::{collections::HashSet, env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let session = format!("session={}", env::var("AOC_SESSION")?);
    let mut headers = header::HeaderMap::new();
    headers.insert(header::COOKIE, header::HeaderValue::from_str(&session)?);

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let input = client
        .get("https://adventofcode.com/2022/day/3/input")
        .send()?
        .text()?;
    let result: usize = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let a = a.chars().collect::<HashSet<char>>();
            let b = b.chars().collect::<HashSet<char>>();
            let chr = *a.intersection(&b).next().unwrap();
            " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .find(chr)
                .unwrap()
        })
        .sum();
    println!("{:?}", result);
    Ok(())
}

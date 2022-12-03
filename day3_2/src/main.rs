use std::{collections::HashSet, env, error::Error, ops::Add};

use itertools::Itertools;
use reqwest::header;

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
        .map(|line| line.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let chr = *chunk
                .reduce(|acc, i| acc.intersection(&i).copied().collect::<HashSet<char>>())
                .unwrap()
                .iter()
                .next()
                .unwrap();
            ('a'..='z')
                .chain('A'..='Z')
                .collect::<String>()
                .find(chr)
                .unwrap()
                .add(1)
        })
        .sum();
    println!("{:?}", result);

    Ok(())
}

use reqwest::header;
use std::{env, error::Error};

pub fn fetch_input(day_n: u8) -> Result<String, Box<dyn Error>> {
    let session = env::var("AOC_SESSION")?;
    let mut headers = header::HeaderMap::new();
    headers.insert(header::COOKIE, header::HeaderValue::from_str(&session)?);

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let url = format!("https://adventofcode.com/2022/day/{}/input", day_n);
    let input = client.get(url).send()?.text()?;
    Ok(input)
}

use std::error::Error;
use util::fetch_input;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fetch_input(4)?;
    let mut result = 0;
    input.lines().for_each(|line| {
        if let [a, b, c, d] = line
            .split(['-', ','])
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()[..4]
        {
            if a >= c && b <= d || a <= c && b >= d {
                result += 1;
            }
        }
    });
    println!("{}", result);
    Ok(())
}

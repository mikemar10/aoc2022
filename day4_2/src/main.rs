use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = 0;
    let input = util::fetch_input(4)?;
    for line in input.lines() {
        let nums = line.split(['-', ',']);
        let nums = nums.map(|n| n.parse::<i32>().unwrap());
        if let [a, b, c, d] = nums.collect::<Vec<i32>>()[..4] {
            let range_a = a..=b;
            let range_b = c..=d;
            if range_a.contains(&c)
                || range_a.contains(&d)
                || range_b.contains(&a)
                || range_b.contains(&b)
            {
                result += 1;
            }
        }
    }
    println!("{}", result);
    Ok(())
}

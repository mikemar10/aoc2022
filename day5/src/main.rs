use std::{collections::HashMap, error::Error, str::Lines};

fn build_stacks(stacks: &mut HashMap<usize, Vec<char>>, input: &mut Lines) {
    for line in input.by_ref() {
        if line.is_empty() {
            break;
        }

        for (i, j) in (1..line.len()).step_by(4).enumerate() {
            if let Some(c) = line.chars().nth(j) {
                if c.is_alphabetic() {
                    stacks.entry(i + 1).or_default().insert(0, c);
                }
            }
        }
    }
}

fn move_crates(stacks: &mut HashMap<usize, Vec<char>>, input: &mut Lines) {
    for line in input {
        let line = line
            .split_whitespace()
            .enumerate()
            .filter(|(i, _)| i % 2 != 0)
            .map(|(_, val)| val.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if let [moves, src, dest] = line[..] {
            for _ in 0..moves {
                if let Some(i) = stacks.get_mut(&src).unwrap().pop() {
                    stacks.get_mut(&dest).unwrap().push(i);
                }
            }
        }
    }
}

fn move_crates2(stacks: &mut HashMap<usize, Vec<char>>, input: &mut Lines) {
    for line in input {
        let line = line // "move 3 from 1 to 2"
            .split_whitespace() // ["move", "3", "from", "1", "to", "2"]
            .enumerate() // [(0, "move"), (1, "3"), (2, "from"), (3, "1"), (4, "to"), (5, "2")]
            .filter(|(i, _)| i % 2 != 0) // [(1, "3"), (3, "1"), (5, "2")]
            .map(|(_, val)| val.parse::<usize>().unwrap()) // [3, 1, 2]
            .collect::<Vec<usize>>(); // vec![3, 1, 2]
        if let [moves, src, dest] = line[..] {
            let mut temp = Vec::new();
            let src = stacks.get_mut(&src).unwrap();
            for _ in 0..moves {
                if let Some(i) = src.pop() {
                    temp.push(i);
                }
            }
            temp.reverse();
            let dest = stacks.get_mut(&dest).unwrap();
            dest.append(&mut temp);
        }
    }
}

fn build_result(stacks: &mut HashMap<usize, Vec<char>>) -> String {
    let mut result = String::new();
    for i in 1..=stacks.len() {
        result.push(stacks.get_mut(&i).unwrap().pop().unwrap());
    }
    result
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    let input = util::fetch_input(5)?;
    let mut input = input.lines();
    build_stacks(&mut stacks, &mut input);
    //move_crates(&mut stacks, &mut input);
    move_crates2(&mut stacks, &mut input);
    let result = build_result(&mut stacks);

    println!("{:?}", result);
    Ok(())
}

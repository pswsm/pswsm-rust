use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = match std::fs::File::open("input.txt") {
        Ok(f) => f,
        Err(_e) => return Ok(()),
    };

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let lines = buf.lines();
    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();

    for line in lines {
        let split_line = line
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<u32>>();
        col1.push(split_line[0]);
        col2.push(split_line[1]);
    }

    col1.sort();
    col2.sort();
    // part1(col1, col2);
    part2(col1, col2);

    Ok(())
}

fn part1(left: Vec<u32>, right: Vec<u32>) -> std::io::Result<()> {
    let mut diff: Vec<_> = Vec::new();
    for (c1, c2) in std::iter::zip(left, right) {
        if c1 >= c2 {
            diff.push(c1 - c2);
        } else {
            diff.push(c2 - c1);
        }
    }

    let sum: u32 = diff.iter().fold(0, |acc, x| acc + x);
    println!("{sum}");

    Ok(())
}

fn part2(left: Vec<u32>, right: Vec<u32>) {
    let mut times: Vec<u32> = Vec::new();
    for item in left {
        let matches_count = right
            .iter()
            .filter(|i| *i == &item)
            .collect::<Vec<&u32>>()
            .len();

        times.push(item * matches_count as u32);
    }

    println!("Part 2: {}", times.iter().fold(0, |acc, i| acc + i));
}

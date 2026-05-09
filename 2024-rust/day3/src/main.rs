use std::io;
use std::fs;
use regex::Regex;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let data = fs::read_to_string(arg)?;

        part1(&data);
        part2(&data);
    }

    Ok(())
}

fn mul(op: &str) -> i64 {
    let parts: Vec<i64> = (&op[4..op.len()-1])
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    return parts[0] * parts[1];
}

fn process(data: &String) -> i64 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let result: i64 = re
        .find_iter(&data)
        .map(|m| mul(m.as_str()))
        .sum();
    return result;
}

fn part1(data: &String) {
    let result: i64 = process(data);

    println!("{}", result);
}

fn part2(data: &String) {
    let mut result: i64 = 0;
    let n = data.len();

    let mut start: usize = 0;
    let mut end: usize = 0;

    while end < n {
        match data[start..].find("don't()") {
            Some(x) => end = start + x,
            None => end = n,
        }

        result += process(&((&data[start..end]).to_string()));

        match data[end..].find("do()") {
            Some(x) => start = end + x,
            None => break,
        }
    }

    println!("{}", result);
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let rows = file_to_vec(arg)?;

        let mut towels: Vec<&str> = Vec::new();
        let mut patterns: Vec<&str> = Vec::new();
    
        let mut done_with_towels = false;
        for row in &rows {
            if done_with_towels {
                patterns.push(row);
            } else if row == "" {
                done_with_towels = true;
            } else {
                let mut substrings = row.split(',').map(|x| x.trim()).collect();
                towels.append(&mut substrings);
            }
        }

        part1(&towels, &patterns);
        part2(&towels, &patterns);
    }

    Ok(())
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn is_possible<'a>(towels: &Vec<&str>, pattern: &'a str, visited: &mut HashMap<&'a str, bool>) -> bool {
    if pattern.len() == 0 {
        return true;
    }
    if visited.contains_key(pattern) {
        return visited[pattern];
    }
    let prefixes = towels.iter()
        .filter(|x| pattern.starts_with(*x))
        .map(|x| x.len())
        .collect::<HashSet<_>>();
    let result = prefixes.iter()
        .any(|x| is_possible(towels, &pattern[*x..], visited));
    visited.insert(pattern, result);
    return result;
}

fn part1(towels: &Vec<&str>, patterns: &Vec<&str>) {
    let mut visited: HashMap<&str, bool> = HashMap::new();

    let result = patterns.iter()
        .filter(|x| is_possible(towels, x, &mut visited))
        .count();

    println!("{}", result);
}

fn count_ways<'a>(towels: &Vec<&str>, pattern: &'a str, visited: &mut HashMap<&'a str, i64>) -> i64 {
    if pattern.len() == 0 {
        return 1;
    }
    if visited.contains_key(pattern) {
        return visited[pattern];
    }
    let prefixes = towels.iter()
        .filter(|x| pattern.starts_with(*x))
        .collect::<HashSet<_>>();
    let result = prefixes.iter()
        .map(|x| count_ways(towels, &pattern[x.len()..], visited))
        .sum();
    visited.insert(pattern, result);
    return result;
}

fn part2(towels: &Vec<&str>, patterns: &Vec<&str>) {
    let mut visited: HashMap<&str, i64> = HashMap::new();

    let result: i64 = patterns.iter()
        .map(|x| count_ways(towels, x, &mut visited))
        .sum();

    println!("{}", result);
}

use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use std::collections::HashSet;
use std::cmp::Ordering;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let rows = file_to_vec(arg)?;

        let mut rules: HashSet<&str> = HashSet::new();
        let mut updates: Vec<Vec<&str>> = Vec::new();
    
        let mut done_with_rules = false;
        for row in &rows {
            if done_with_rules {
                let words = row.split(',');
                updates.push(words.collect());
            } else if row == "" {
                done_with_rules = true;
            } else {
                rules.insert(row.as_str());
            }
        }
                
        part1(&updates, &rules);
        part2(&updates, &rules);
    }

    Ok(())
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn compare_by_rules(a: &&str, b: &&str, rules: &HashSet<&str>) -> Ordering {
    if rules.contains(format!("{a}|{b}").as_str()) {
        return Ordering::Less;
    }
    if rules.contains(format!("{b}|{a}").as_str()) {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

fn is_valid_update(update: &Vec<&str>, rules: &HashSet<&str>) -> bool {
    let mut sorted = update.clone();
    sorted.sort_by(|a, b| compare_by_rules(a, b, rules));
    return sorted.iter().zip(update.iter()).all(|(a, b)| a == b);
}

fn fix_update<'a>(update: &Vec<&'a str>, rules: &HashSet<&str>) -> Vec<&'a str> {
    let mut sorted = update.clone();
    sorted.sort_by(|a, b| compare_by_rules(a, b, rules));
    return sorted;
}

fn get_middle_number(update: &Vec<&str>) -> Result<i64, std::num::ParseIntError> {
    let idx: usize = update.len() / 2;
    return update[idx].parse::<i64>();
}

fn part1(updates: &Vec<Vec<&str>>, rules: &HashSet<&str>) {
    let result: i64 = updates
        .iter()
        .filter(|x| is_valid_update(x, &rules))
        .filter_map(|x| get_middle_number(x).ok())
        .sum();

    println!("{}", result);
}

fn part2(updates: &Vec<Vec<&str>>, rules: &HashSet<&str>) {
    let result: i64 = updates
        .iter()
        .filter(|x| !is_valid_update(x, &rules))
        .map(|x| fix_update(x, &rules))
        .filter_map(|x| get_middle_number(&x).ok())
        .sum();

    println!("{}", result);
}

use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point(i64, i64);

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let lines = /* vec!["123".to_string()]; */ file_to_vec(arg.as_str())?;

        part1(lines.as_slice());
    }

    Ok(())
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn rnd(s: &String, count: i32, cache: &mut HashMap<(i64, i64, i64, i64), i64>) -> i64 {
    let mut result = s.parse().unwrap();

    let mut sequence: Vec<i64> = Vec::new();
    let mut prev_price = result % 10;
    let mut seen_keys: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    
    for _i in 0..count {
        result = (result << 6) ^ result;
        result = result % 16777216;

        result = (result >> 5) ^ result;
        result = result % 16777216;

        result = (result << 11) ^ result;
        result = result % 16777216;

        let new_price = result % 10;
        sequence.push(new_price - prev_price);
        prev_price = new_price;

        if sequence.len() >= 4 {
            let n = sequence.len() - 1;
            let k = (sequence[n - 3], sequence[n - 2], sequence[n - 1], sequence[n]);
            if !seen_keys.contains(&k) {
                if cache.contains_key(&k) {
                    cache.insert(k, cache[&k] + new_price);
                } else {
                    cache.insert(k, new_price);
                }
                seen_keys.insert(k);
            }
        }
    }

    return result;
}

fn part1(rows: &[String]) {
    let mut cache: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    let result: i64 = rows.iter()
        .map(|x| rnd(x, 2000, &mut cache))
        .sum();

    println!("{}", result);

    let result2: i64 = cache.iter()
        .map(|(_, v)| *v)
        .max()
        .unwrap();

    println!("{}", result2);
}

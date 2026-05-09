// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

#[derive(PartialEq)]
enum Schematics {
    None,
    Lock,
    Key
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let lines = file_to_vec(arg.as_str())?;

        part1(lines.as_slice());
    }

    Ok(())
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn part1(rows: &[String]) {
    let mut keys: Vec<Vec<i64>> = Vec::new();
    let mut locks: Vec<Vec<i64>> = Vec::new();

    let mut cur: Vec<i64> = Vec::new();
    let mut schematics = Schematics::None;
    for row in rows {
        if row == "" {
            match schematics {
                Schematics::Key => { keys.push(cur); }
                Schematics::Lock => { locks.push(cur); }
                Schematics::None => {}
            }
            cur = Vec::new();
            schematics = Schematics::None;
            continue;
        }
        if row == "#####" {
            if schematics == Schematics::None {
                schematics = Schematics::Lock;
            }
        }
        if row == "....." {
            if schematics == Schematics::None {
                schematics = Schematics::Key;
            }
        }
        let mut idx: usize = 0;
        for c in row.chars() {
            if cur.len() == idx {
                cur.push(-1);
            }
            if c == '#' {
                let n = cur[idx];
                cur[idx] = n + 1;
            }
            idx += 1;
        }
    }
    match schematics {
        Schematics::Key => { keys.push(cur); }
        Schematics::Lock => { locks.push(cur); }
        Schematics::None => {}
    }

    let mut result = 0;

    for key in &keys {
        for lock in &locks {
            if lock.iter().zip(key.iter()).all(|(a, b)| a + b <= 5) {
                result += 1;
            }
        }
    }

    println!("{}", result);
}

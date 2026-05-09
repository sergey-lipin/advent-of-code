use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let lines = file_to_vec(arg)?;

        let mut count: usize = 0;
        let mut left: Vec<i64> = Vec::new();
        let mut right: Vec<i64> = Vec::new();
        for line in lines {
            let words = line.split_whitespace();
            let numbers: Vec<i64> = words.filter_map(|x| x.parse::<i64>().ok()).collect();
            left.push(numbers[0]);
            right.push(numbers[1]);
            count += 1;
        }
        left.sort();
        right.sort();

        part1(left.as_slice(), right.as_slice(), count);
        part2(left.as_slice(), right.as_slice(), count);
    }

    Ok(())
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn part1(left: &[i64], right: &[i64], count: usize) {
    let mut result: i64 = 0;
    let mut i: usize = 0;

    while i < count {
        let d: i64 = left[i] - right[i];
        if d > 0 {
            result += d;
        } else {
            result -= d;
        }
        i = i + 1;
    }
    
    println!("{}", result);
}

fn part2(left: &[i64], right: &[i64], count: usize) {
    let mut result: i64 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut repeats: i64 = 0;

    while i < count {
        if i < (count - 1) && left[i] == left[i + 1] {
            i = i + 1;
            repeats += 1;
            continue;
        }
        while j < count {
            if right[j] > left[i] {
                break;
            }
            if right[j] == left[i] {
                result += left[i] * (repeats + 1);
            }
            j += 1;
        }
        if j >= count {
            break;
        }
        i = i + 1;
        repeats = 0;
    }
    
    println!("{}", result);
}

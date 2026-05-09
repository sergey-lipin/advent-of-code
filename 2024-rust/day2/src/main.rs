use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use std::borrow::Borrow;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let reports = load_reports(arg)?;
        
        part1(reports.as_slice());
        part2(reports.as_slice());
    }

    Ok(())
}

fn load_reports(filename: String) -> io::Result<Vec<Vec<i64>>> {
    let lines = file_to_vec(filename)?;

    let mut reports: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let words = line.split_whitespace();
        let numbers: Vec<i64> = words.filter_map(|x| x.parse::<i64>().ok()).collect();
        reports.push(numbers);
    }

    Ok(reports)
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn get_distance<T: Borrow<i64>>(sign: &mut i64, pair: &[T]) -> i64 {
    let d = pair[1].borrow() - pair[0].borrow();
    if *sign == 0 {
        if d > 0i64 {
            *sign = 1;
        } else if d < 0i64 {
            *sign = -1;
        }
    }
    return (*sign) * d;
}

fn is_valid_distance(d: i64) -> bool {
    if d < 1 || d > 3 {
        return false;
    }
    return true;
}

fn is_report_safe(report: &[i64]) -> bool {
    let mut sign: i64 = 0;
    return report
        .windows(2)
        .all(|pair| is_valid_distance(get_distance(&mut sign, pair)));
}

fn is_report_safe_ex(report: &[i64], exclude: usize) -> bool {
    let mut sign: i64 = 0;
    return report
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != exclude )
        .map(|(_, e)| e)
        .collect::<Vec<&i64>>()
        .windows(2)
        .all(|pair| is_valid_distance(get_distance(&mut sign, pair)));
}

fn is_report_safe_1(report: &[i64]) -> bool {
    return is_report_safe(report);
}

fn is_report_safe_2(report: &[i64]) -> bool {
    if is_report_safe(report) {
        return true;
    }
    let n = report.len();
    let mut i: usize = 0;
    while i < n {
        if is_report_safe_ex(report, i) {
            return true;
        }
        i += 1;
    }
    return false;
}

fn part1(reports: &[Vec<i64>]) {
    let result = reports.iter().filter(|x| is_report_safe_1(x.as_slice())).count();

    println!("{}", result);
}

fn part2(reports: &[Vec<i64>]) {
    let result = reports.iter().filter(|x| is_report_safe_2(x.as_slice())).count();

    println!("{}", result);
}

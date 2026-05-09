use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let equations = load_equations(arg)?;
        
        part1(equations.as_slice());
        part2(equations.as_slice());
    }

    Ok(())
}

fn load_equations(filename: String) -> io::Result<Vec<Vec<i64>>> {
    let lines = file_to_vec(filename)?;

    let mut equations: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        let words = line.split_whitespace();
        let numbers: Vec<i64> = words.filter_map(|x| x.trim_end_matches(':').parse::<i64>().ok()).collect();
        equations.push(numbers);
    }

    Ok(equations)
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn concatenate(left: i64, right: i64) -> i64 {
    // return format!("{left}{right}").parse::<i64>().unwrap();
    return left * 10i64.pow(right.ilog10() + 1) + right;
}

fn is_solvable(equation: &Vec<i64>, start: usize, value: i64, concat: bool) -> bool {
    if start == equation.len() {
        return value == equation[0];
    }
    return is_solvable(equation, start + 1, value + equation[start], concat)
        || is_solvable(equation, start + 1, value * equation[start], concat)
        || (concat && is_solvable(equation, start + 1, concatenate(value, equation[start]), true));
}

fn part1(equations: &[Vec<i64>]) {
    let result: i64 = equations.iter()
        .filter(|x| is_solvable(x, 2, x[1], false))
        .map(|x| x[0])
        .sum();

    println!("{}", result);
}

fn part2(equations: &[Vec<i64>]) {
    let result: i64 = equations.iter()
        .filter(|x| is_solvable(x, 2, x[1], true))
        .map(|x| x[0])
        .sum();

    println!("{}", result);
}

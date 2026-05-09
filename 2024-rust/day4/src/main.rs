use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let rows = file_to_vec(arg)?;
        
        part1(rows.as_slice());
        part2(rows.as_slice());
    }

    Ok(())
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn is_char_at_point(rows: &[String], col: &mut i32, row: &mut i32, width: i32, height: i32, dir: (i32, i32), c: char) -> bool {
    *col += dir.0;
    *row += dir.1;

    if *col < 0 || *col >= width || *row < 0 || *row >= height {
        return false;
    }

    return rows[*row as usize].as_bytes()[*col as usize] == c as u8;
}

fn count_words_from_point(rows: &[String], x: usize, y: usize) -> i64 {
    let height = rows.len() as i32;
    let width = rows[y].len() as i32;
    let word = "MAS";
    let mut result: i64 = 0;

    let dirs = vec![(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];
    for dir in dirs {
        let mut col = x as i32;
        let mut row = y as i32;
        let found: bool = word
            .chars()
            .all(|c| is_char_at_point(rows, &mut col, &mut row, width, height, dir, c));
        if found {
            result += 1;
        }
    }

    return result;
}

fn part1(rows: &[String]) {
    let result: i64 = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v == 'X')
        .map(|(x, y, _)| count_words_from_point(rows, x, y))
        .sum();

    println!("{}", result);
}

fn peek_char_at_point(rows: &[String], col: i32, row: i32, width: i32, height: i32) -> Option<char> {
    if col < 0 || col >= width || row < 0 || row >= height {
        return None;
    }
    let c = (rows[row as usize].as_bytes()[col as usize]) as char;
    if c != 'M' && c != 'S' {
        return None;
    }
    return Some(c);
}

fn is_cross_at_point(rows: &[String], x: &usize, y: &usize) -> bool {
    let height = rows.len() as i32;
    let width = rows[*y].len() as i32;
    let col = *x as i32;
    let row = *y as i32;

    let dirs = vec![(-1, -1), (1, 1), (1, -1), (-1, 1)];
    let vals: Vec<char> = dirs.iter()
        .filter_map(|dir| peek_char_at_point(rows, col + dir.0, row + dir.1, width, height))
        .collect();

    if vals.len() != 4 {
        return false;
    }
    if vals[0] == vals[1] || vals[2] == vals[3] {
        return false;
    }

    return true;
}

fn part2(rows: &[String]) {
    let result: usize = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(x, y, v)| *v == 'A' && is_cross_at_point(rows, x, y))
        .count();

    println!("{}", result);
}

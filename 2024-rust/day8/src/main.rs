use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use collecting_hashmap::CollectingHashMap;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let mut rows = file_to_vec(arg.as_str())?;
        part1(rows.as_mut_slice());
        rows = file_to_vec(arg.as_str())?;
        part2(rows.as_mut_slice());
    }

    Ok(())
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn is_within_map(rows: &[String], point: (i32, i32)) -> bool {
    let height = rows.len() as i32;
    let width = rows[0].len() as i32;

    if point.0 < 0 || point.0 >= width || point.1 < 0 || point.1 >= height {
        return false;
    }

    return true;
}

enum SetCharResult {
    Success,
    AlreadySet,
    OutOfMap
}

fn set_char(rows: &mut [String], pos: (i32, i32), c: char) -> SetCharResult {
    if !is_within_map(rows, pos) {
        return SetCharResult::OutOfMap;
    }
    let s = rows[pos.1 as usize].as_mut_str();
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    if s_bytes[pos.0 as usize] == c as u8 {
        return SetCharResult::AlreadySet;
    }
    s_bytes[pos.0 as usize] = c as u8;
    return SetCharResult::Success;
}

fn place_antinodes(rows: &mut [String], points: (&(i32, i32), &(i32, i32)), grid: bool) -> i64 {
    let mut result: i64 = 0;

    let dx = points.1.0 - points.0.0;
    let dy = points.1.1 - points.0.1;

    if grid {
        let mut pos1 = points.0.clone();
        loop {
            match set_char(rows, pos1, '#') {
                SetCharResult::Success => { result += 1; }
                SetCharResult::OutOfMap => { break; }
                _ => {}
            }
            pos1.0 -= dx;
            pos1.1 -= dy;
        }
        let mut pos2 = points.1.clone();
        loop {
            match set_char(rows, pos2, '#') {
                SetCharResult::Success => { result += 1; }
                SetCharResult::OutOfMap => { break; }
                _ => {}
            }
            pos2.0 += dx;
            pos2.1 += dy;
        }
    } else {
        let pos1 = (points.0.0 - dx, points.0.1 - dy);
        match set_char(rows, pos1, '#') {
            SetCharResult::Success => { result += 1; }
            _ => {}
        }
        let pos2 = (points.1.0 + dx, points.1.1 + dy);
        match set_char(rows, pos2, '#') {
            SetCharResult::Success => { result += 1; }
            _ => {}
        }
    }

    return result;
}

fn count_antinodes(rows: &mut [String], v: &Vec<(i32, i32)>, grid: bool) -> i64 {
    return v.iter()
        .enumerate()
        .flat_map(|(i1, p1)| v.iter().enumerate().filter(move |(i2, _)| *i2 > i1).map(move |(_, p2)| (p1, p2)))
        .map(|p| place_antinodes(rows, p, grid))
        .sum();
}

fn process_antennas(rows: &mut [String], grid: bool) {
    let antennas = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v != '.')
        .map(|(x, y, v)| (v, (x as i32, y as i32)))
        .collect::<CollectingHashMap<_, _>>();

    let result: i64 = antennas.iter()
        .map(|(_, v)| count_antinodes(rows, v, grid))
        .sum();

    println!("{}", result);
}

fn part1(rows: &mut [String]) {
    process_antennas(rows, false)
}

fn part2(rows: &mut [String]) {
    process_antennas(rows, true)
}

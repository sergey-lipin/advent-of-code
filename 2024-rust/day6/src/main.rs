use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

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

fn set_char(rows: &mut [String], pos: (i32, i32), c: char) {
    let s = rows[pos.1 as usize].as_mut_str();
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    s_bytes[pos.0 as usize] = c as u8;
}

fn turn_right(dir: &mut (i32, i32)) {
    match *dir {
        (0, -1) => {
            *dir = (1, 0);
        }
        (1, 0) => {
            *dir = (0, 1);
        }
        (0, 1) => {
            *dir = (-1, 0);
        }
        (-1, 0) => {
            *dir = (0, -1);
        }
        _ => {
            return;
        }
    }
}

fn find_start_point(rows: &[String]) -> (i32, i32) {
    let mut start_point: (usize, usize) = (0, 0);
    start_point.1 = rows
        .iter()
        .position(|x| {
            match x.find('^') {
                Some(i) => {
                    start_point.0 = i;
                    return true;
                }
                None => {
                    return false;
                }
            }
        })
        .unwrap();
    return (start_point.0 as i32, start_point.1 as i32);
}

fn part1(rows: &mut [String]) {
    let height: i32 = rows.len() as i32;
    let width: i32 = rows[0].len() as i32;

    let mut pos: (i32, i32) = find_start_point(rows);
    let mut dir: (i32, i32) = (0, -1);

    while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
        let c = (rows[pos.1 as usize].as_bytes()[pos.0 as usize]) as char;
        if c == '#' {
            pos.0 -= dir.0;
            pos.1 -= dir.1;

            turn_right(&mut dir);
        } else {
            set_char(rows, pos, 'X');
        }

        pos.0 += dir.0;
        pos.1 += dir.1;
    }

    let result: usize = rows
        .iter()
        .flat_map(|s| s.chars())
        .filter(|v| *v == 'X')
        .count();

    println!("{}", result);
}

fn is_stuck(rows: &[String], start_pos: (i32, i32), start_dir: (i32, i32), width: i32, height: i32) -> bool {
    let mut pos: (i32, i32) = start_pos;
    let mut dir: (i32, i32) = start_dir;

    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
        let c = (rows[pos.1 as usize].as_bytes()[pos.0 as usize]) as char;
        if c == '#' {
            pos.0 -= dir.0;
            pos.1 -= dir.1;

            if visited.contains(&(pos, dir)) {
                return true;
            }
            visited.insert((pos, dir));

            turn_right(&mut dir);
        }

        pos.0 += dir.0;
        pos.1 += dir.1;
    }

    return false;
}

fn part2(rows: &mut [String]) {
    let height: i32 = rows.len() as i32;
    let width: i32 = rows[0].len() as i32;

    let mut pos: (i32, i32) = find_start_point(rows);
    let mut dir: (i32, i32) = (0, -1);

    let mut result: usize = 0;

    while pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height {
        let c = (rows[pos.1 as usize].as_bytes()[pos.0 as usize]) as char;
        if c == '#' {
            pos.0 -= dir.0;
            pos.1 -= dir.1;

            turn_right(&mut dir);
        } else {
            if c != 'X' && c != '^' {
                set_char(rows, pos, '#');
                if is_stuck(rows, pos, dir, width, height) {
                    result += 1;
                }
            }

            set_char(rows, pos, 'X');
        }

        pos.0 += dir.0;
        pos.1 += dir.1;
    }

    println!("{}", result);
}

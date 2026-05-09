use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i64, i64);

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let mut lines = file_to_vec(arg.as_str())?;

        let mut rows: Vec<&mut str> = Vec::new();
        let mut moves = String::from("");
        let mut done_with_rows = false;

        for line in lines.iter_mut() {
            if done_with_rows {
                moves += line.as_str();
            } else if line == "" {
                done_with_rows = true;
            } else {
                rows.push(line.as_mut_str());
            }
        }
                
        part1(&mut rows, moves.as_str());


        lines = file_to_vec(arg.as_str())?;
        rows = Vec::new();

        for line in lines.iter_mut() {
            if line == "" {
                break;
            } else {
                *line = line.replace("#", "##");
                *line = line.replace("O", "[]");
                *line = line.replace(".", "..");
                *line = line.replace("@", "@.");
                rows.push(line.as_mut_str());
            }
        }
                
        part2(&mut rows, moves.as_str());
    }

    Ok(())
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn peek_char_at_point(rows: &Vec<&mut str>, pos: &Point) -> Option<char> {
    let height = rows.len() as i64;
    let width = rows[0].len() as i64;

    if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
        return None;
    }

    let c = (rows[pos.1 as usize].as_bytes()[pos.0 as usize]) as char;
    return Some(c);
}

fn set_char(rows: &mut Vec<&mut str>, pos: &Point, c: char) {
    let s = &mut rows[pos.1 as usize];
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    s_bytes[pos.0 as usize] = c as u8;
}

fn move_robot_1(rows: &mut Vec<&mut str>, pos: &Point, dir: char) -> Point {
    let mut new_pos = *pos;
    let mut moved = false;
    match dir {
        '^' => { new_pos.1 -= 1; }
        'v' => { new_pos.1 += 1; }
        '<' => { new_pos.0 -= 1; }
        '>' => { new_pos.0 += 1; }
        _ => { return *pos; }
    }
    let mut cur_pos = new_pos;
    while !moved {
        match peek_char_at_point(rows, &cur_pos) {
            Some('.') => {
                set_char(rows, pos, '.');
                set_char(rows, &new_pos, '@');
                if cur_pos != new_pos {
                    set_char(rows, &cur_pos, 'O');
                }
                moved = true;
            }
            Some('O') => {}
            Some(_) => { break; }
            None => { break; }
        }
        cur_pos.0 += new_pos.0 - pos.0;
        cur_pos.1 += new_pos.1 - pos.1;
    }
    if moved {
        return new_pos;
    }
    return *pos;
}

fn plan_move(rows: &mut Vec<&mut str>, pos: &Point, delta: &Point, visited: &mut HashSet<Point>) -> Option<Vec<Point>> {
    let mut result: Vec<Point> = Vec::new();
    if visited.contains(&pos) {
        return Some(result);
    }
    let new_pos = Point(pos.0 + delta.0, pos.1 + delta.1);
    match peek_char_at_point(rows, &new_pos) {
        Some('.') => {
            result.push(*pos);
        }
        Some('[') => {
            match plan_move(rows, &new_pos, delta, visited) {
                Some(mut v) => { result.append(&mut v); }
                None => { return None; }
            }
            if delta.1 != 0 {
                let adj_pos = Point(new_pos.0 + 1, new_pos.1);
                match plan_move(rows, &adj_pos, delta, visited) {
                    Some(mut v) => { result.append(&mut v); }
                    None => { return None; }
                }
            }
            result.push(*pos);
        }
        Some(']') => {
            match plan_move(rows, &new_pos, delta, visited) {
                Some(mut v) => { result.append(&mut v); }
                None => { return None; }
            }
            if delta.1 != 0 {
                let adj_pos = Point(new_pos.0 - 1, new_pos.1);
                match plan_move(rows, &adj_pos, delta, visited) {
                    Some(mut v) => { result.append(&mut v); }
                    None => { return None; }
                }
            }
            result.push(*pos);
        }
        Some(_) => { return None; }
        None => { return None; }
    }
    visited.insert(*pos);
    return Some(result);
}

fn do_move(rows: &mut Vec<&mut str>, pos: &Point, delta: &Point) {
    let new_pos = Point(pos.0 + delta.0, pos.1 + delta.1);
    match peek_char_at_point(rows, pos) {
        Some(c) => {
            set_char(rows, &new_pos, c);
            set_char(rows, pos, '.');
        }
        None => {}
    }
}

fn move_robot_2(rows: &mut Vec<&mut str>, pos: &Point, dir: char) -> Point {
    let mut delta = Point(0, 0);
    match dir {
        '^' => { delta.1 -= 1; }
        'v' => { delta.1 += 1; }
        '<' => { delta.0 -= 1; }
        '>' => { delta.0 += 1; }
        _ => { return *pos; }
    }
    let mut visited: HashSet<Point> = HashSet::new(); 
    match plan_move(rows, pos, &delta, &mut visited) {
        Some(x) => {
            for p in x {
                do_move(rows, &p, &delta);
            }
            return Point(pos.0 + delta.0, pos.1 + delta.1);
        }
        None => { return *pos; }
    }
}

fn part1(rows: &mut Vec<&mut str>, moves: &str) {
    let mut pos: Point = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v == '@')
        .map(|(x, y, _)| Point(x as i64, y as i64))
        .next()
        .unwrap();

    for m in moves.chars() {
        pos = move_robot_1(rows, &pos, m);
    }

    let result: usize = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v == 'O')
        .map(|(x, y, _)| 100 * y + x)
        .sum();

    println!("{}", result);
}

fn part2(rows: &mut Vec<&mut str>, moves: &str) {
    let mut pos: Point = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v == '@')
        .map(|(x, y, _)| Point(x as i64, y as i64))
        .next()
        .unwrap();

    // print_rows(rows);
    for m in moves.chars() {
        // println!("{}", m);
        pos = move_robot_2(rows, &pos, m);
        // print_rows(rows);
    }
    print_rows(rows);

    let result: usize = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v == '[')
        .map(|(x, y, _)| 100 * y + x)
        .sum();

    println!("{}", result);
}

fn print_rows(rows: &mut Vec<&mut str>) {
    for row in rows {
        println!("{}", row);
    }
    println!("");
}

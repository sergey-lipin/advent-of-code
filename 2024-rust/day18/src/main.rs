use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const NUM_BYTES: usize = 1024;

/*
const WIDTH: usize = 7;
const HEIGHT: usize = 7;
const NUM_BYTES: usize = 12;
*/

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point(i64, i64);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Compass {
    pos: Point,
    dir: char,
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let lines = file_to_vec(arg.as_str())?;

        part1(&lines);
        part2(&lines);
    }

    Ok(())
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn string_to_point(s: &String) -> Point {
    let nums: Vec<i64> = s
        .split(',')
        .map(|m| m.parse::<i64>().unwrap())
        .collect();
    return Point(nums[0], nums[1]);
}

fn generate_map(width: usize, height: usize, lines: &[String], num_bytes: usize) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();

    for _i in 0..height {
        rows.push(".".repeat(width));
    }

    set_char(&mut rows, Point(0, 0), 'S');
    set_char(&mut rows, Point(width as i64 - 1, height as i64 - 1), 'E');

    for i in 0..num_bytes {
        set_char(&mut rows, string_to_point(&(lines[i])), '#');
    }

    return rows;
}

fn peek_char_at_point(rows: &[String], pos: &Point) -> Option<char> {
    let height = rows.len() as i64;
    let width = rows[0].len() as i64;

    if pos.0 < 0 || pos.0 >= width || pos.1 < 0 || pos.1 >= height {
        return None;
    }

    let c = (rows[pos.1 as usize].as_bytes()[pos.0 as usize]) as char;
    return Some(c);
}

fn set_char(rows: &mut [String], pos: Point, c: char) -> bool {
    let s = rows[pos.1 as usize].as_mut_str();
    let s_num_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    if s_num_bytes[pos.0 as usize] == c as u8 {
        return false;
    }
    s_num_bytes[pos.0 as usize] = c as u8;
    return true;
}

fn get_new_pos(pos: Point, dir: char) -> Compass {
    let mut result =  Compass{pos, dir};
    match dir {
        '^' => { result.pos.1 -= 1; }
        'v' => { result.pos.1 += 1; }
        '<' => { result.pos.0 -= 1; }
        '>' => { result.pos.0 += 1; }
        _ => {}
    }
    return result;
}

fn find_path(rows: &[String], start: Compass) -> Option<i64> {
    let mut results: Vec<i64> = Vec:: new();
    let mut visited: HashMap<Compass, i64> = HashMap::new(); 
    let mut q: VecDeque<(Compass, i64)> = VecDeque::new();
    q.push_back((start, 0));

    while q.len() > 0 {
        let item = q.pop_front().unwrap();

        let k = item.0;
        if visited.contains_key(&k) && visited[&k] <= item.1 {
            continue;
        }
        visited.insert(k, item.1);

        match peek_char_at_point(rows, &(item.0.pos)) {
            Some('E') => {
                results.push(item.1);
            }
            Some('.') | Some('S') => {
                q.push_back((get_new_pos(item.0.pos, item.0.dir), item.1 + 1));

                match item.0.dir {
                    '^' | 'v' => {
                        q.push_back((get_new_pos(item.0.pos, '<'), item.1 + 1));
                        q.push_back((get_new_pos(item.0.pos, '>'), item.1 + 1));
                    }
                    '<' | '>' => {
                        q.push_back((get_new_pos(item.0.pos, '^'), item.1 + 1));
                        q.push_back((get_new_pos(item.0.pos, 'v'), item.1 + 1));
                    }
                    _ => {}
                }
            }
            Some(_) | None => {}
        }
    }

    if results.len() == 0 {
        return None;
    }
    return Some(*(results.iter().min().unwrap()));
}

fn part1(lines: &[String]) {
    let rows = generate_map(WIDTH, HEIGHT, lines, NUM_BYTES);

    let start = Compass{pos: Point(0, 0), dir: '>'};
    let result = find_path(&rows, start).unwrap();

    println!("{}", result);
}

fn part2(lines: &[String]) {
    let mut low: usize = NUM_BYTES;
    let mut high: usize = lines.len();
    let mut cut_off: usize = 0;

    loop {
        let num_bytes = (high + low) / 2;
        if num_bytes == low {
            break;
        }
        let rows = generate_map(WIDTH, HEIGHT, lines, num_bytes);

        let start = Compass{pos: Point(0, 0), dir: '>'};
        let result = find_path(&rows, start);
        match result {
            Some(_) => { low = num_bytes; }
            None => { high = num_bytes; cut_off = num_bytes - 1; }
        }
    }

    println!("{}", lines[cut_off]);
}

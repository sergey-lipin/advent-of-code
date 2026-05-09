use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

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
        let mut lines = file_to_vec(arg.as_str())?;
                
        part1(lines.as_slice());
        part2(lines.as_mut_slice());
    }

    Ok(())
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
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
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    if s_bytes[pos.0 as usize] == c as u8 {
        return false;
    }
    s_bytes[pos.0 as usize] = c as u8;
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

fn move_reindeer_1(rows: &[String], start: Compass) -> Option<i64> {
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
            Some('E') => { results.push(item.1); }
            Some('.') | Some('S') => {
                q.push_back((get_new_pos(item.0.pos, item.0.dir), item.1 + 1));

                match item.0.dir {
                    '^' | 'v' => {
                        q.push_back((get_new_pos(item.0.pos, '<'), item.1 + 1001));
                        q.push_back((get_new_pos(item.0.pos, '>'), item.1 + 1001));
                    }
                    '<' | '>' => {
                        q.push_back((get_new_pos(item.0.pos, '^'), item.1 + 1001));
                        q.push_back((get_new_pos(item.0.pos, 'v'), item.1 + 1001));
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

fn move_reindeer_2(rows: &mut [String], start: Compass) -> Option<i64> {
    let mut end: HashSet<Compass> = HashSet::new();
    let mut visited: HashMap<Compass, i64> = HashMap::new();
    let mut parents: HashMap<Compass, HashSet<Compass>> = HashMap::new();
    let mut q: VecDeque<(Compass, i64, Option<Compass>)> = VecDeque::new();
    q.push_back((start, 0, None));

    while q.len() > 0 {
        let item = q.pop_front().unwrap();

        let k = item.0;
        if visited.contains_key(&k) && visited[&k] < item.1 {
            continue;
        }
        if visited.contains_key(&k) && visited[&k] == item.1 {
            if let Some(x) = item.2 {
                if let Some(hs) = parents.get_mut(&k) {
                    hs.insert(x);
                }
            }
            continue;
        }
        if let Some(x) = item.2 {
            let mut hs: HashSet<Compass> = HashSet::new();
            hs.insert(x);
            parents.insert(k, hs);
        }
        visited.insert(k, item.1);

        match peek_char_at_point(rows, &(item.0.pos)) {
            Some('E') => { end.insert(k); }
            Some('.') | Some('S') => {
                q.push_back((get_new_pos(item.0.pos, item.0.dir), item.1 + 1, Some(k)));

                match item.0.dir {
                    '^' | 'v' => {
                        q.push_back((get_new_pos(item.0.pos, '<'), item.1 + 1001, Some(k)));
                        q.push_back((get_new_pos(item.0.pos, '>'), item.1 + 1001, Some(k)));
                    }
                    '<' | '>' => {
                        q.push_back((get_new_pos(item.0.pos, '^'), item.1 + 1001, Some(k)));
                        q.push_back((get_new_pos(item.0.pos, 'v'), item.1 + 1001, Some(k)));
                    }
                    _ => {}
                }
            }
            Some(_) | None => {}
        }
    }

    if end.len() == 0 {
        return None;
    }

    let mut res_q: VecDeque<Compass> = VecDeque::new();
    let m = end.iter().map(|x| visited[x]).min().unwrap();
    for e in end {
        if visited[&e] == m {
            res_q.push_back(e);
        }
    }

    let mut n: i64 = 0;

    while res_q.len() > 0 {
        let item = res_q.pop_front().unwrap();
        if set_char(rows, item.pos, 'O') {
            n += 1;
        }

        if parents.contains_key(&item) {
            for p in &parents[&item] {
                res_q.push_back(*p);
            }
        }
    }

    /*
    for r in rows {
        println!("{}", *r);
    }
    */

    return Some(n);
}

fn part1(rows: &[String]) {
    let pos: Point = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v == 'S')
        .map(|(x, y, _)| Point(x as i64, y as i64))
        .next()
        .unwrap();

        let start = Compass{pos, dir: '>'};
        let result = move_reindeer_1(rows, start).unwrap();

    println!("{}", result);
}

fn part2(rows: &mut [String]) {
    let pos: Point = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v == 'S')
        .map(|(x, y, _)| Point(x as i64, y as i64))
        .next()
        .unwrap();

    let start = Compass{pos, dir: '>'};
    let result = move_reindeer_2(rows, start).unwrap();

    println!("{}", result);
}

use std::collections::HashMap;
// use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point(i64, i64);

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let lines = file_to_vec(arg.as_str())?;
                
        part1(lines.as_slice());
        part2(lines.as_slice());
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

fn get_new_pos(pos: Point, dir: char) -> Point {
    let mut result =  pos;
    match dir {
        '^' => { result.1 -= 1; }
        'v' => { result.1 += 1; }
        '<' => { result.0 -= 1; }
        '>' => { result.0 += 1; }
        _ => {}
    }
    return result;
}

fn move_program(rows: &[String], start: char, end: char, dir: char) -> Option<(i64, HashMap<Point, i64>)> {
    let pos: Point = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v == start)
        .map(|(x, y, _)| Point(x as i64, y as i64))
        .next()
        .unwrap();

    let mut results: Vec<i64> = Vec:: new();
    let mut visited: HashMap<Point, i64> = HashMap::new(); 
    let mut q: VecDeque<(Point, char, i64)> = VecDeque::new();
    q.push_back((pos, dir, 0));

    while q.len() > 0 {
        let item = q.pop_front().unwrap();

        let k = item.0;
        if visited.contains_key(&k) && visited[&k] <= item.2 {
            continue;
        }
        visited.insert(k, item.2);

        match peek_char_at_point(rows, &(item.0)) {
            Some(c) if c == end => { results.push(item.2); }
            Some(c) if c == start || c == '.' => {
                if item.1 == '*' {
                    q.push_back((get_new_pos(item.0, '<'), '<', item.2 + 1));
                    q.push_back((get_new_pos(item.0, '>'), '>', item.2 + 1));
                    q.push_back((get_new_pos(item.0, '^'), '^', item.2 + 1));
                    q.push_back((get_new_pos(item.0, 'v'), 'v', item.2 + 1));
                } else {
                    q.push_back((get_new_pos(item.0, item.1), item.1, item.2 + 1));

                    match item.1 {
                        '^' | 'v' => {
                            q.push_back((get_new_pos(item.0, '<'), '<', item.2 + 1));
                            q.push_back((get_new_pos(item.0, '>'), '>', item.2 + 1));
                        }
                        '<' | '>' => {
                            q.push_back((get_new_pos(item.0, '^'), '^', item.2 + 1));
                            q.push_back((get_new_pos(item.0, 'v'), 'v', item.2 + 1));
                        }
                        _ => {}
                    }
                }
            }
            Some(_) | None => {}
        }
    }

    if results.len() == 0 {
        return None;
    }
    return Some((*(results.iter().min().unwrap()), visited));
}

fn dist(a: &Point, b: &Point) -> i64 {
    return (b.0 - a.0).abs() + (b.1 - a.1).abs();
}

fn count_valid_cheats(rows: &[String], x: usize, y: usize, min_gain: i64, max_dist: i64, forward: &HashMap<Point, i64>, backward: &HashMap<Point, i64>) -> usize {
    let pos = Point(x as i64, y as i64);
    if !forward.contains_key(&pos) || !backward.contains_key(&pos) {
        return 0;
    }

    let mut cheats: Vec<Point> = Vec::new();
    for i in -max_dist..(max_dist + 1) {
        for j in -max_dist..(max_dist + 1) {
            let pt = Point(x as i64 + i, y as i64 + j);
            let d = dist(&pt, &pos);
            if d >= 2 && d <= max_dist {
                cheats.push(pt);
            }
        }
    }
    let mut result: usize = 0;

    for k in cheats {
        match peek_char_at_point(rows, &k) {
            Some('#') | None => {}
            Some(_) => {
                let d = dist(&k, &pos);
                if forward.contains_key(&k) && backward.contains_key(&k)
                && (forward[&k] - forward[&pos] - d) >= min_gain && (backward[&pos] - backward[&k] - d) >= min_gain {
                    result += 1;
                    // println!("{:?} {} {} - {:?} {} {}", pos, forward[&pos], backward[&pos], k, forward[&k], backward[&k]);
                }
            }
        }
    }

    return result;
}

fn part1(rows: &[String]) {
    let forward = move_program(rows, 'S', 'E', '*').unwrap();
    let backward = move_program(rows, 'E', 'S', '*').unwrap();

    let result: usize = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v != '#' )
        .map(|(x, y, _)| count_valid_cheats(rows, x, y, 100, 2, &forward.1, &backward.1))
        .sum();

    println!("{}", result);
}

fn part2(rows: &[String]) {
    let forward = move_program(rows, 'S', 'E', '*').unwrap();
    let backward = move_program(rows, 'E', 'S', '*').unwrap();

    let result: usize = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v != '#' )
        .map(|(x, y, _)| count_valid_cheats(rows, x, y, 100, 20, &forward.1, &backward.1))
        .sum();

    println!("{}", result);
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use collecting_hashmap::CollectingHashMap;
use itertools::Itertools;

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

fn is_char_at_point(rows: &[String], pt: (usize, usize), dir: (i32, i32), c: char) -> bool {
    let col = pt.0 as i32 + dir.0;
    let row = pt.1 as i32 + dir.1;

    let height = rows.len() as i32;
    let width = rows[0].len() as i32;

    if col < 0 || col >= width || row < 0 || row >= height {
        return false;
    }

    return rows[row as usize].as_bytes()[col as usize] == c as u8;
}

fn calc_price_1(rows: &[String], x: usize, y: usize, c: char, visited: &mut HashSet<(usize, usize)>) -> i64 {
    let k = (x, y);
    if visited.contains(&k) {
        return 0;
    }

    let dirs = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    let mut area: i64 = 0;
    let mut fence: i64 = 0;
    q.push_back((x, y));

    while q.len() > 0 {
        let pt = q.pop_front().unwrap();

        if visited.contains(&pt) {
            continue;
        }
        visited.insert(pt);

        area += 1;
        fence += 4;
    
        for dir in dirs.as_slice() {
            if is_char_at_point(rows, pt, *dir, c) {
                q.push_back(((pt.0 as i32 + dir.0) as usize, (pt.1 as i32 + dir.1) as usize));
                fence -= 1;
            }
        }
    }

    return area * fence;
}

fn calc_price_2(rows: &[String], x: usize, y: usize, c: char, visited: &mut HashSet<(usize, usize)>) -> i64 {
    let k = (x, y);
    if visited.contains(&k) {
        return 0;
    }

    let dirs = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    let mut area: i64 = 0;
    let mut sides: HashMap<(i32, i32), CollectingHashMap<usize, usize>> = HashMap::new();
    q.push_back((x, y));

    while q.len() > 0 {
        let pt = q.pop_front().unwrap();

        if visited.contains(&pt) {
            continue;
        }
        visited.insert(pt);

        area += 1;
    
        for dir in dirs.as_slice() {
            if is_char_at_point(rows, pt, *dir, c) {
                q.push_back(((pt.0 as i32 + dir.0) as usize, (pt.1 as i32 + dir.1) as usize));
            } else {
                let mut key = 0;
                let mut value = 0;
                match dir {
                    (0, -1) => { key = pt.1; value = pt.0; }
                    (1, 0) => { key = pt.0; value = pt.1; }
                    (0, 1) => { key = pt.1; value = pt.0; }
                    (-1, 0) => { key = pt.0; value = pt.1; }
                    _ => {}
                }
                sides.entry(*dir).or_insert(CollectingHashMap::new()).insert(key, value);
            }
        }
    }

    let num_sides: usize = sides.iter()
        .map(|(_, v)| {
            v.iter().map(|(_, x)| {
                let mut s = x.clone();
                s.sort();
                return (0..s.len()).chunk_by(|&a| s[a] - a).into_iter().count()
            })
            .sum::<usize>()
        })
        .sum();
    return area * num_sides as i64;
}

fn part1(rows: &[String]) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let result: i64 = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .map(|(x, y, c)| calc_price_1(rows, x, y, c, &mut visited))
        .sum();

    println!("{}", result);
}

fn part2(rows: &[String]) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let result: i64 = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .map(|(x, y, c)| calc_price_2(rows, x, y, c, &mut visited))
        .sum();

    println!("{}", result);
}

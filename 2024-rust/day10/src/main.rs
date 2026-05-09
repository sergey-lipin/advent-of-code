use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let rows = file_to_vec(arg)?;
        
        let height = rows.len() as i32;
        let width = rows[0].len() as i32;
    
        let word = String::from("0123456789");
    
        part1(rows.as_slice(), width, height, &word);
        part2(rows.as_slice(), width, height, &word);
    }

    Ok(())
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn is_char_at_point(rows: &[String], x: i32, y: i32, width: i32, height: i32, dir: (i32, i32), c: u8) -> bool {
    let col = x + dir.0;
    let row = y + dir.1;

    if col < 0 || col >= width || row < 0 || row >= height {
        return false;
    }

    return rows[row as usize].as_bytes()[col as usize] == c;
}

fn count_trails_from_point(rows: &[String], x: i32, y: i32, width: i32, height: i32, word: &String, next_idx: usize, visited: &mut Option<HashSet<(i32, i32)>>) -> i64 {
    if next_idx == word.len() {
        if let Some(ref mut hash_set) = visited { 
            let peak: (i32, i32) = (x, y);
            if hash_set.contains(&peak) {
                return 0;
            }
            hash_set.insert(peak);
        }
        return 1;
    }
    let c = word.as_bytes()[next_idx];

    let dirs = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    return dirs.iter()
        .filter(|dir| is_char_at_point(rows, x, y, width, height, **dir, c))
        .map(|(dx, dy)| count_trails_from_point(rows, x + *dx, y + *dy, width, height, word, next_idx + 1, visited))
        .sum();
}

fn part1(rows: &[String], width: i32, height: i32, word: &String) {
    let result: i64 = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v as u8 == word.as_bytes()[0])
        .map(|(x, y, _)| count_trails_from_point(rows, x as i32, y as i32, width, height, word, 1, &mut Some(HashSet::new())))
        .sum();

    println!("{}", result);
}

fn part2(rows: &[String], width: i32, height: i32, word: &String) {
    let result: i64 = rows
        .iter()
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, v)| (x, y, v)))
        .filter(|(_, _, v)| *v as u8 == word.as_bytes()[0])
        .map(|(x, y, _)| count_trails_from_point(rows, x as i32, y as i32, width, height, word, 1, &mut None::<HashSet<(i32, i32)>>))
        .sum();

    println!("{}", result);
}

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
    }

    Ok(())
}

fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn new_numpad() -> HashMap<char, Point> {
    let mut result: HashMap<char, Point> = HashMap::new();
    result.insert('7', Point(0, 0));
    result.insert('8', Point(1, 0));
    result.insert('9', Point(2, 0));
    result.insert('4', Point(0, 1));
    result.insert('5', Point(1, 1));
    result.insert('6', Point(2, 1));
    result.insert('1', Point(0, 2));
    result.insert('2', Point(1, 2));
    result.insert('3', Point(2, 2));
    result.insert('#', Point(0, 3));
    result.insert('0', Point(1, 3));
    result.insert('A', Point(2, 3));
    return result;
}

fn new_dirpad() -> HashMap<char, Point> {
    let mut result: HashMap<char, Point> = HashMap::new();
    result.insert('#', Point(0, 0));
    result.insert('^', Point(1, 0));
    result.insert('A', Point(2, 0));
    result.insert('<', Point(0, 1));
    result.insert('v', Point(1, 1));
    result.insert('>', Point(2, 1));
    return result;
}

fn translate_step(step: (u8, u8), keypad: &HashMap<char, Point>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let old_pos = keypad[&(step.0 as char)];
    let new_pos = keypad[&(step.1 as char)];
    let gap = keypad[&'#'];

    let mut q: VecDeque<(Point, Vec<char>)> = VecDeque::new();
    q.push_back((old_pos, Vec::new()));

    while q.len() > 0 {
        let item = q.pop_front().unwrap();
        let pos = &item.0;

        if item.0 == new_pos {
            let mut moves = item.1.clone();
            moves.push('A');
            result.push(String::from_iter(moves));
            continue;
        }

        if item.1.len() > 0 {
            match item.1[item.1.len() - 1] {
                '^' => {
                    if pos.1 > new_pos.1 && gap != Point(pos.0, pos.1 - 1) {
                        let mut moves = item.1.clone();
                        moves.push('^');
                        q.push_back((Point(pos.0, pos.1 - 1), moves));
                        continue;
                    }
                }
                '>' => {
                    if pos.0 < new_pos.0 && gap != Point(pos.0 + 1, pos.1) {
                        let mut moves = item.1.clone();
                        moves.push('>');
                        q.push_back((Point(pos.0 + 1, pos.1), moves));
                        continue;
                    }
                }
                'v' => {
                    if pos.1 < new_pos.1 && gap != Point(pos.0, pos.1 + 1) {
                        let mut moves = item.1.clone();
                        moves.push('v');
                        q.push_back((Point(pos.0, pos.1 + 1), moves));
                        continue;
                    }
                }
                '<' => {
                    if pos.0 > new_pos.0 && gap != Point(pos.0 - 1, pos.1) {
                        let mut moves = item.1.clone();
                        moves.push('<');
                        q.push_back((Point(pos.0 - 1, pos.1), moves));
                        continue;
                    }
                }
                _ => {}
            }
        }

        if pos.1 > new_pos.1 && gap != Point(pos.0, pos.1 - 1) {
            let mut moves = item.1.clone();
            moves.push('^');
            q.push_back((Point(pos.0, pos.1 - 1), moves));
        }
        if pos.0 < new_pos.0 && gap != Point(pos.0 + 1, pos.1) {
            let mut moves = item.1.clone();
            moves.push('>');
            q.push_back((Point(pos.0 + 1, pos.1), moves));
        }
        if pos.1 < new_pos.1 && gap != Point(pos.0, pos.1 + 1) {
            let mut moves = item.1.clone();
            moves.push('v');
            q.push_back((Point(pos.0, pos.1 + 1), moves));
        }
        if pos.0 > new_pos.0 && gap != Point(pos.0 - 1, pos.1) {
            let mut moves = item.1.clone();
            moves.push('<');
            q.push_back((Point(pos.0 - 1, pos.1), moves));
        }
    }

    return result;
}

fn translate_segment(segment: &str, keypad: &HashMap<char, Point>) -> Vec<String> {
    let sequence = "A".to_string() + segment;
    let steps = sequence.as_bytes().windows(2);
    let mut paths: Vec<String> = Vec::new();

    for step in steps {
        let subpaths = translate_step((step[0], step[1]), keypad);

        if paths.len() == 0 {
            paths = subpaths;
            continue;
        }

        let mut tmp: Vec<String> = Vec::new();
        for i in &paths {
            for j in &subpaths {
                tmp.push(i.to_owned() + j.as_str());
            }
        }

        paths = tmp;
    }

    let min_len = paths.iter().map(|x| x.len()).min().unwrap();
    let result = paths.into_iter().filter(|x| x.len() == min_len).collect();
    return result;
}

fn get_min_path_len(row: &String, keypads: &Vec<HashMap<char, Point>>,
    cache: &mut HashMap<(usize, String), Vec<HashMap<String, usize>>>,
    best: &mut Vec<String>) -> usize {
    let mut translations = vec![row.to_owned()];

    for keypad in keypads {
        let mut new_translations: Vec<String> = Vec::new();
        for translation in &translations {
            let mut new_translation: Vec<String> = Vec::new();
            let segments = translation
                .trim_end_matches('A')
                .split('A')
                .map(|x| x.to_owned() + "A");
            for segment in segments {
                let r = translate_segment(segment.as_str(), keypad);

                let k = (keypad.len(), segment);
                if !cache.contains_key(&k) {
                    let mut v: Vec<HashMap<String, usize>> = Vec::new();
                    for j in &r {
                        let mut hm: HashMap<String, usize> = HashMap::new();

                        let pts = j
                            .trim_end_matches('A')
                            .split('A')
                            .map(|x| x.to_owned() + "A");
                        for pt in pts {
                            let mut count = 1;
                            if hm.contains_key(&pt) {
                                count += hm[&pt];
                            }
                            hm.insert(pt, count);
                        }

                        v.push(hm);
                    }
                    cache.insert(k, v);
                }

                if new_translation.len() == 0 {
                    new_translation = r;
                    continue;
                }

                let mut tmp: Vec<String> = Vec::new();
                for i in &new_translation {
                    for j in &r {
                        tmp.push(i.to_owned() + j.as_str());
                    }
                }
        
                new_translation = tmp;
            }

            new_translations.append(&mut new_translation);
        }
        translations = new_translations;
    }

    let min_len = translations.iter().map(|x| x.len()).min().unwrap();
    best.append(&mut (translations.into_iter().filter(|x| x.len() == min_len).collect()));
    return min_len;
}

fn get_string_key(comp: &HashMap<String, usize>) -> String {
    let mut keys: Vec<&String> = comp.keys().collect();
    keys.sort();
    let mut result = "".to_string();
    for key in keys {
        if result.len() > 0 {
            result += ",";
        }
        result += format!("{}:{}", key, comp[key]).as_str();
    }
    return result;
}

fn count_len(comp: &HashMap<String, usize>,
    cache: &HashMap<(usize, String), Vec<HashMap<String, usize>>>,
    cache_id: usize, depth: usize,
    visited: &mut HashMap<(String, usize), Option<usize>>) -> Option<usize> {

    if depth == 0 {
        return Some(comp.iter().map(|(k, v)| k.len() * v).sum());
    }

    let visited_key = (get_string_key(comp), depth);
    if visited.contains_key(&visited_key) {
        return visited[&visited_key];
    }

    let mut min_len: Option<usize> = None;

    for (k, v) in comp {
        let cache_key = (cache_id, k.clone());
        let cur_len = cache[&cache_key].iter()
            .filter_map(|x| count_len(x, cache, cache_id, depth - 1, visited))
            .map(|x| x * v)
            .min();
        if let Some(l) = cur_len {
            if let Some(m) = min_len {
                min_len = Some(m + l);
            } else {
                min_len = cur_len;
            }
        }
    }

    visited.insert(visited_key, min_len);
    return min_len;
}

fn part1(rows: &[String]) {
    let keypads = vec![
        new_numpad(),
        new_dirpad(),
        new_dirpad()
    ];
    let mut result = 0;

    let mut cache: HashMap<(usize, String), Vec<HashMap<String, usize>>> = HashMap::new();
    let mut all_best: HashMap<String, Vec<String>> = HashMap::new();

    for row in rows {
        let mut best: Vec<String> = Vec::new();
        let l: usize = get_min_path_len(row, &keypads, &mut cache, &mut best);
        let c: usize = row[..(row.len() - 1)].parse().unwrap();
        println!("{} {}x{}", row, l, c);
        result += l * c;
        all_best.insert(row.to_owned(), best);
    }

    println!("{}", result);
    result = 0;
    let mut visited: HashMap<(String, usize), Option<usize>> = HashMap::new();

    for row in rows {
        let mut min_len: Option<usize> = None;
        for best in &all_best[row] {
            let mut hm: HashMap<String, usize> = HashMap::new();

            let pts = best
                .trim_end_matches('A')
                .split('A')
                .map(|x| x.to_owned() + "A");
            for pt in pts {
                let mut count = 1;
                if hm.contains_key(&pt) {
                    count += hm[&pt];
                }
                hm.insert(pt, count);
            }

            let cur_len = count_len(&hm, &cache, keypads[keypads.len()-1].len(), 23, &mut visited);
            if let Some(l) = cur_len {
                if let Some(m) = min_len {
                    if l < m {
                        min_len = cur_len;
                    }
                } else {
                    min_len = cur_len;
                }
            }
        }
        if let Some(l) = min_len {
            let c: usize = row[..(row.len() - 1)].parse().unwrap();
            println!("{} {}x{}", row, l, c);
            result += l * c;
        }
    }    
    println!("{}", result);
}

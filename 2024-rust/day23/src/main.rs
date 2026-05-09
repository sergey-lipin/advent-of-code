use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::VecDeque;
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

fn dfs<'l>(node: &'l str, adj: &HashMap<&str, HashSet<&'l str>>, network: &mut Vec<&'l str>) {
    network.push(node);
    if is_clique(network, adj) {
        for n in &adj[node] {
            dfs(n, adj, network);
        }
    } else {
        network.pop();
    }
}

fn is_clique(network: &Vec<&str>, adj: &HashMap<&str, HashSet<&str>>) -> bool {
    let n = network.len();

    for i in 0..n {
        for j in (i + 1)..n {
            if !adj[network[i]].contains(&network[j]) {
                return false;
            }
        }
    }

    return true;
}

fn part2(rows: &[String]) {
    let mut adj: HashMap<&str, HashSet<&str>> = HashMap::new();

    for row in rows {
        let comps: Vec<&str> = row.split('-').collect();
        if !adj.contains_key(&comps[0]) {
            adj.insert(comps[0], HashSet::new());
        }
        adj.get_mut(&comps[0]).unwrap().insert(comps[1]);
        if !adj.contains_key(&comps[1]) {
            adj.insert(comps[1], HashSet::new());
        }
        adj.get_mut(&comps[1]).unwrap().insert(comps[0]);
    }

    let mut max_network: Vec<&str> = Vec::new();
    for node in adj.keys() {
        let mut cur_network: Vec<&str> = Vec::new();
        dfs(node, &adj, &mut cur_network);
        if cur_network.len() > max_network.len() {
            max_network = cur_network;
        }
    }

    max_network.sort();
    println!("{}", max_network.join(","));
}

fn part1(rows: &[String]) {
    let mut adj: HashMap<&str, HashSet<&str>> = HashMap::new();

    for row in rows {
        let comps: Vec<&str> = row.split('-').collect();
        if !adj.contains_key(&comps[0]) {
            adj.insert(comps[0], HashSet::new());
        }
        adj.get_mut(&comps[0]).unwrap().insert(comps[1]);
        if !adj.contains_key(&comps[1]) {
            adj.insert(comps[1], HashSet::new());
        }
        adj.get_mut(&comps[1]).unwrap().insert(comps[0]);
    }

    let sets: HashSet<Vec<&str>> = adj.iter()
        .filter(|(k, v)| {
            (k.as_bytes()[0] as char) == 't' && v.len() >= 2
        })
        .filter_map(|(k, v)| {
            let mut x: Vec<Vec<&str>> = Vec::new();
            let s: Vec<&&str> = v.iter().collect();

            for i in 0..s.len() {
                for j in (i + 1)..s.len() {
                    if adj[*s[i]].contains(*s[j]) {
                        let mut r = vec![*k, *s[i], *s[j]];
                        r.sort();
                        x.push(r);
                    }
                }
            }

            if x.len() == 0 {
                return None;
            }
            return Some(x);
        })
        .flat_map(|x| x.into_iter())
        .collect();

    // println!("{:?}", sets);

    let result = sets.len();

    println!("{}", result);
}

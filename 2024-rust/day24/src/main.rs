use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Gate {
    op: String,
    arg1: String,
    arg2: String,
    dest: String
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let rows = file_to_vec(arg)?;

        part1(&rows);
        part2(&rows);
    }

    Ok(())
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn part1(rows: &[String]) {
    let mut wires: HashMap<String, i64> = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();

    let mut done_with_wires = false;
    for row in rows {
        if done_with_wires {
            let g: Vec<&str> = row.split(" ").collect();
            let gate = Gate { op: g[1].to_string(), arg1: g[0].to_string(), arg2: g[2].to_string(), dest: g[4].to_string() };
            gates.push(gate);
        } else if row == "" {
            done_with_wires = true;
        } else {
            let wire: Vec<&str> = row.split(": ").collect();
            wires.insert(wire[0].to_string(), wire[1].parse::<i64>().unwrap());
        }
    }

    let mut q: VecDeque<usize> = VecDeque::new();

    for i in 0..gates.len() {
        q.push_back(i);
    }

    while q.len() > 0 {
        let idx = q.pop_front().unwrap();

        if !wires.contains_key(&gates[idx].arg1) || !wires.contains_key(&gates[idx].arg2) {
            q.push_back(idx);
            continue;
        }

        let arg1 = wires[&gates[idx].arg1];
        let arg2 = wires[&gates[idx].arg2];
        match gates[idx].op.as_str() {
            "AND" => { wires.insert(gates[idx].dest.clone(), arg1 & arg2); }
            "OR" => { wires.insert(gates[idx].dest.clone(), arg1 | arg2); }
            "XOR" => { wires.insert(gates[idx].dest.clone(), arg1 ^ arg2); }
            _ => { panic!(); }
        }
    }

    let mut result: i64 = 0;

    let keys: Vec<&String> = wires.keys().filter(|x| x.starts_with("z")).collect();

    for key in keys {
        result += wires[key] << key[1..].parse::<i32>().unwrap();
    }

    println!("{}", result);
}

fn add(a: i64, b: i64, gates: &Vec<Gate>) -> i64 {
    let mut wires: HashMap<String, i64> = HashMap::new();

    for i in 0..45 {
        if a & (1 << i) != 0 {
            wires.insert(format!("x{:02}", i), 1);
        } else {
            wires.insert(format!("x{:02}", i), 0);
        }
        if b & (1 << i) != 0 {
            wires.insert(format!("y{:02}", i), 1);
        } else {
            wires.insert(format!("y{:02}", i), 0);
        }
    }

    let mut q: VecDeque<usize> = VecDeque::new();

    for i in 0..gates.len() {
        q.push_back(i);
    }

    while q.len() > 0 {
        let idx = q.pop_front().unwrap();

        if !wires.contains_key(&gates[idx].arg1) || !wires.contains_key(&gates[idx].arg2) {
            q.push_back(idx);
            continue;
        }

        let arg1 = wires[&gates[idx].arg1];
        let arg2 = wires[&gates[idx].arg2];
        match gates[idx].op.as_str() {
            "AND" => { wires.insert(gates[idx].dest.clone(), arg1 & arg2); }
            "OR" => { wires.insert(gates[idx].dest.clone(), arg1 | arg2); }
            "XOR" => { wires.insert(gates[idx].dest.clone(), arg1 ^ arg2); }
            _ => { panic!(); }
        }
    }

    let mut result: i64 = 0;

    let keys: Vec<&String> = wires.keys().filter(|x| x.starts_with("z")).collect();

    for key in keys {
        result += wires[key] << key[1..].parse::<i32>().unwrap();
    }

    return result;
}

fn is_parent(a: &Gate, b: &Gate, adj: &HashMap<String, Vec<Gate>>) -> bool {
    if a.dest == b.arg1 || a.dest == b.arg2 {
        return true;
    }
    if adj.contains_key(&a.dest) {
        for i in &adj[&a.dest] {
            if is_parent(i, b, adj) {
                return true;
            }
        }
    }
    return false;
}

fn ultimate_wire(gate: &Gate, adj: &HashMap<String, Vec<Gate>>) -> String {
    let mut q: VecDeque<&Gate> = VecDeque::new();
    q.push_back(gate);
    loop {
        let i = q.pop_front().unwrap();
        if !adj.contains_key(&i.dest) {
            return i.dest.clone();
        }
        for g in &adj[&i.dest] {
            q.push_back(g);
        }
    }
}

fn compare_by_adj(a: &Gate, b: &Gate, adj: &HashMap<String, Vec<Gate>>) -> Ordering {
    let r = ultimate_wire(a, adj).cmp(&ultimate_wire(b, adj));
    if r != Ordering::Equal {
        return r;
    }
    if is_parent(a, b, adj) {
        return Ordering::Less;
    }
    if is_parent(b, a, adj) {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

fn part2(rows: &[String]) {
    let mut gates: Vec<Gate> = Vec::new();
    let mut adj: HashMap<String, Vec<Gate>> = HashMap::new();

    let mut done_with_wires = false;
    for row in rows {
        if done_with_wires {
            let g: Vec<&str> = row.split(" ").collect();
            let gate = Gate { op: g[1].to_string(), arg1: g[0].to_string(), arg2: g[2].to_string(), dest: g[4].to_string() };
            gates.push(gate.clone());
            let k1 = g[0].to_string();
            if !adj.contains_key(&k1) {
                adj.insert(k1.clone(), Vec::new());
            }
            adj.get_mut(&k1).unwrap().push(gate.clone());
            let k2 = g[2].to_string();
            if !adj.contains_key(&k2) {
                adj.insert(k2.clone(), Vec::new());
            }
            adj.get_mut(&k2).unwrap().push(gate.clone());
        } else if row == "" {
            done_with_wires = true;
        } else {
            continue;
        }
    }

    gates.sort_by(|a, b| {
        return compare_by_adj(a, b, &adj);
    });

    for gate in &gates {
        println!("{} {} {} -> {}", gate.arg1, gate.op, gate.arg2, gate.dest);
    }

    for i in 0..45 {
        let t = 1 << i;

        let r1 = add(t, t, &gates);
        if r1 != t + t {
            println!("1: {}", i);
            println!("{:046b}", r1);
            println!("{:046b}", t + t);
        }
        let r2 = add(t, 0, &gates);
        if r2 != t {
            println!("2: {}", i);
            println!("{:046b}", r2);
            println!("{:046b}", t);
        }
        let r3 = add(0, t, &gates);
        if r3 != t {
            println!("3: {}", i);
            println!("{:046b}", r3);
            println!("{:046b}", t);
        }
    }

    // z07 - gmt, cbj - qjj, z18 - dmn, z35 - cfk
    // cbj,cfk,dmn,gmt,qjj,z07,z18,z35
}

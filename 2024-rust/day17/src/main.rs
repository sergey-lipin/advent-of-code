use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ip: usize
}

fn combo(comp: &Computer, op: i64) -> i64 {
    match op {
        0 | 1 | 2 | 3 => { return op; }
        4 => { return comp.a; }
        5 => { return comp.b; }
        6 => { return comp.c; }
        _ => { panic!(); }
    }
}

fn adv(comp: &mut Computer, op: i64) {
    comp.a = comp.a >> combo(comp, op) as u32;
    comp.ip += 2;
}

fn bdv(comp: &mut Computer, op: i64) {
    comp.b = comp.a >> combo(comp, op) as u32;
    comp.ip += 2;
}

fn cdv(comp: &mut Computer, op: i64) {
    comp.c = comp.a >> combo(comp, op) as u32;
    comp.ip += 2;
}

fn bxl(comp: &mut Computer, op: i64) {
    comp.b = comp.b ^ op;
    comp.ip += 2;
}

fn bst(comp: &mut Computer, op: i64) {
    comp.b = combo(comp, op) % 8;
    comp.ip += 2;
}

fn bxc(comp: &mut Computer, _op: i64) {
    comp.b = comp.b ^ comp.c;
    comp.ip += 2;
}

fn out(comp: &mut Computer, op: i64) -> i64 {
    let result = combo(comp, op) % 8;
    comp.ip += 2;
    return result;
}

fn jnz(comp: &mut Computer, op: i64) {
    if comp.a == 0 {
        comp.ip += 2;
    } else {
        comp.ip = op as usize;
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let rows = file_to_vec(arg)?;

        let mut registers: Vec<i64> = Vec::new();
        let mut program: Vec<i64> = Vec::new();
    
        let mut done_with_registers = false;
        for row in &rows {
            if done_with_registers {
                let substrings = row[9..].split(',');
                let mut ops: Vec<i64> = substrings.map(|x| x.parse::<i64>().unwrap()).collect();
                program.append(&mut ops);
            } else if row == "" {
                done_with_registers = true;
            } else {
                registers.push(row[12..].parse::<i64>().unwrap());
            }
        }

        println!("{:?}", program);
                
        part1(&registers, &program);
        part2(&registers, &program);
    }

    Ok(())
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn run(registers: &Vec<i64>, program: &Vec<i64>, match_program: bool, visited: &mut HashSet<Computer>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();

    let mut comp = Computer{
        a: registers[0],
        b: registers[1],
        c: registers[2],
        ip: 0
    };

    while comp.ip < (program.len() - 1) {
        if visited.contains(&comp) {
            break;
        }
        visited.insert(comp);
        let op = program[comp.ip + 1];
        match program[comp.ip] {
            0 => { adv(&mut comp, op); }
            1 => { bxl(&mut comp, op); }
            2 => { bst(&mut comp, op); }
            3 => { jnz(&mut comp, op); }
            4 => { bxc(&mut comp, op); }
            5 => {
                result.push(out(&mut comp, op));
                if match_program {
                    let idx = result.len() - 1;
                    if result[idx] != program[idx] {
                        break;
                    }
                }
            }
            6 => { bdv(&mut comp, op); }
            7 => { cdv(&mut comp, op); }
            _ => { panic!(); }
        }
    }

    return result;
}

fn part1(registers: &Vec<i64>, program: &Vec<i64>) {
    let mut visited: HashSet<Computer> = HashSet::new();
    let result = run(registers, program, false, &mut visited);

    println!("{:?}", result);
}

fn do_vecs_match<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

fn get_samples(registers: &Vec<i64>, program: &Vec<i64>, min_len: usize, constant: i64, const_bits: i32, visited: &mut HashSet<Computer>) -> Option<Vec<i64>> {
    let program_len = program.len();
    let start_idx: i64 = 8i64.pow(program.len() as u32 - 1) >> const_bits;
    let end_idx: i64 = 8i64.pow(program.len() as u32) >> const_bits;
    let mut samples: Vec<i64> = Vec::new();

    let mut i: i64 = start_idx;
    while i < end_idx {
        let new_vec = vec![(i << const_bits) + constant, registers[1], registers[2]];
        let result = run(&new_vec, program, true, visited);

        if result.len() >= min_len {
            samples.push((i << const_bits) + constant);
            if samples.len() == 16 {
                break;
            }
        }

        if result.len() == program_len && do_vecs_match(&result, &program) {
            println!("{:?} {}", result, (i << const_bits) + constant);
            return None;
        }

        i += 1;
    }

    return Some(samples);
}

fn part2(registers: &Vec<i64>, program: &Vec<i64>) {
    let mut visited: HashSet<Computer> = HashSet::new();
    let mut min_len: usize = 6;
    let mut constant: i64 = 0;
    let mut const_bits: i32 = 0;
    loop {
        let samples = get_samples(registers, program, min_len, constant, const_bits, &mut visited);
        if let Some(x) = samples {
            for i in 0..64 {
                if x.windows(2).all(|w| (w[0] << i) == (w[1] << i)) {
                    min_len += 1;
                    const_bits = 64 - i;
                    constant = x[0] & ((1 << const_bits) - 1);
                    break;
                }
            }
        } else {
            break;
        }
    }
}

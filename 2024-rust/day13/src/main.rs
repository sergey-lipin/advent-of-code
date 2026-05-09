use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use itertools::Itertools;
use regex::Regex;

struct Point(i64, i64);

struct Machine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let rows = file_to_vec(arg)?;

        let mut machines: Vec<Machine> = Vec::new(); 
        for mut chunk in &rows.into_iter().chunks(4) {
            let machine = Machine{
                button_a: string_to_point(chunk.next().unwrap()),
                button_b: string_to_point(chunk.next().unwrap()),
                prize: string_to_point(chunk.next().unwrap())
            };
            machines.push(machine);
        }
                
        part1(&machines);
        part2(&machines);
    }

    Ok(())
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn string_to_point(s: String) -> Point {
    let re = Regex::new(r"([X,Y][+=][0-9]+)").unwrap();
    let nums: Vec<i64> = re
        .find_iter(&s)
        .map(|m| (m.as_str()[2..]).parse::<i64>().unwrap())
        .collect();
    return Point(nums[0], nums[1]);
}

fn get_price_1(machine: &Machine) -> i64 {
    let mut cur_point = Point(0, 0);
    for i in 1..101 {
        cur_point.0 += machine.button_a.0;
        cur_point.1 += machine.button_a.1;
        if cur_point.0 > machine.prize.0 || cur_point.1 > machine.prize.1 {
            break;
        }
        if (machine.prize.0 - cur_point.0) % machine.button_b.0 == 0 && (machine.prize.1 - cur_point.1) % machine.button_b.1 == 0 {
            let d = (machine.prize.0 - cur_point.0) / machine.button_b.0;
            if d <= 100 && d == (machine.prize.1 - cur_point.1) / machine.button_b.1 {
                return i * 3 + d;
            }
        }
    }
    return 0;
}

fn get_price_2(machine: &Machine) -> i64 {
    let p = Point(machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000);

    let c_a = (p.0 as f64 - p.1 as f64 * machine.button_b.0 as f64 / machine.button_b.1 as f64)
        / (machine.button_a.0 as f64 - machine.button_a.1 as f64 * machine.button_b.0 as f64 / machine.button_b.1 as f64);
    let c_b = (p.1 as f64 - c_a * machine.button_a.1 as f64) / machine.button_b.1 as f64;

    let count_a = ((c_a * 1000f64).round() / 1000f64) as i64;
    let count_b = ((c_b * 1000f64).round() / 1000f64) as i64;

    if machine.button_a.0 * count_a + machine.button_b.0 * count_b != p.0 {
        return 0;
    }
    if machine.button_a.1 * count_a + machine.button_b.1 * count_b != p.1 {
        return 0;
    }

    return count_a * 3 + count_b;
}

fn part1(machines: &Vec<Machine>) {
    let result: i64 = machines.iter()
        .map(|x| get_price_1(x))
        .sum();

    println!("{}", result);
}

fn part2(machines: &Vec<Machine>) {
    let result: i64 = machines.iter()
        .map(|x| get_price_2(x))
        .sum();

    println!("{}", result);
}

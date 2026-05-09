use std::collections::HashMap;
use std::io;
use std::fs;
use std::iter;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let data = fs::read_to_string(arg)?;

        part1(&data);
        part2(&data);
    }

    Ok(())
}

fn blink(x: i64) -> Box<dyn Iterator<Item = i64>> {
    if x == 0 {
        Box::new(iter::once(1))
    } else {
        let n_digits = x.ilog10() + 1;
        if n_digits % 2 == 0 {
            let d = 10i64.pow(n_digits / 2);
            Box::new(iter::once(x / d).chain(iter::once(x % d)))
        } else {
            Box::new(iter::once(x * 2024))
        }
    }
}

fn calc(it: Box<dyn Iterator<Item = i64>>, hm: &mut HashMap<(i64, i32), usize>, depth: i32, max_depth: i32) -> usize {
    if depth == max_depth {
        return it.count();
    }
    /*
    let mut result = 0;
    for x in it {
        let k = (x, depth);
        if hm.contains_key(&k) {
            result += hm[&k];
            continue;
        }
        let v = calc(blink(x), hm, depth + 1, max_depth);
        hm.insert(k, v);
        result += v;
    }
    return result;
    */
    return it
        .map(|x| {
            let k = (x, depth);
            if hm.contains_key(&k) {
                return hm[&k];
            }
            let v = calc(blink(x), hm, depth + 1, max_depth);
            hm.insert(k, v);
            return v;
        })
        .sum();
}

fn process(data: &String, n: i32) -> usize {
    let start: Vec<i64> = data
        .split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    let it: Box<dyn Iterator<Item = i64>> = Box::new(start.into_iter());
    let mut hm: HashMap<(i64, i32), usize> = HashMap::new();
    return calc(it, &mut hm, 0, n);
}

/*
fn process(data: &String, n: i32) -> usize {
    let mut it: Box<dyn Iterator<Item = i64>> = Box::new(data
        .split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok()));

    for _ in 0..n {
        it = Box::new(it.flat_map(|x| blink(x)));
    }

    return it.count()
}
*/

fn part1(data: &String) {
    let result= process(data, 25);

    println!("{}", result);
}

fn part2(data: &String) {
    let result= process(data, 75);

    println!("{}", result);
}

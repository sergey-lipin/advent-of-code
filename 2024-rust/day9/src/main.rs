use std::io;
use std::fs;
use std::iter;

const FREE: i32 = -1;

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

fn make_fat(data: &String) -> Vec<i32> {
    return data
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let id = (i / 2) as i32;
            let n = ((c as u8) - 48) as usize;
            if (i % 2) == 0 {
                return iter::repeat(id).take(n);
            }
            return iter::repeat(FREE).take(n);
        })
        .collect();
}

fn process1(data: &String) -> i64 {
    let mut fat: Vec<i32> = make_fat(data);

    let mut i_free: usize = 0;
    let mut i_file: usize = fat.len() - 1;

    loop {
        while i_free < (fat.len() - 1) && fat[i_free] != FREE {
            i_free += 1;
        }
        while i_file > 0 && fat[i_file] == FREE {
            i_file -= 1;
        }
        if i_free >= i_file {
            break;
        }
        fat[i_free] = fat[i_file];
        fat[i_file] = FREE;
    }

    return fat.iter()
        .enumerate()
        .filter(|(_, x)| **x != FREE)
        .map(|(i, x)| i as i64 * (*x) as i64)
        .sum();
}

fn part1(data: &String) {
    let result: i64 = process1(data);

    println!("{}", result);
}

fn process2(data: &String) -> i64 {
    let mut fat: Vec<i32> = make_fat(data);

    let mut free_space: Vec<(usize, usize)> = data
        .chars()
        .enumerate()
        .scan(0usize, |state, (i, c)| {
            let n = ((c as u8) - 48) as usize;
            let res = (i, (state.clone(), n));
            *state = *state + n;
            return Some(res);
        })
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, v)| v)
        .collect();

    let mut i_file = fat.len() - 1;

    loop {
        while i_file > 0 && fat[i_file] == FREE {
            i_file -= 1;
        }

        let file_len = (data.as_bytes()[(fat[i_file] * 2) as usize] - 48) as usize;

        if i_file < file_len {
            break;
        }

        let file_start = i_file - file_len + 1;

        let pos = free_space.iter().position(|(_, n)| *n >= file_len);

        match pos {
            Some(idx) => {
                let free_slot: &mut (usize, usize) = &mut free_space[idx];
                if free_slot.0 < file_start {
                    for i_free in 0..file_len {
                        fat[free_slot.0 + i_free] = fat[file_start + i_free];
                        fat[file_start + i_free] = FREE;
                    }
                    free_slot.0 += file_len;
                    free_slot.1 -= file_len;
                }
                i_file -= file_len;
            }
            None => {
                i_file -= file_len;
            }
        }
    }

    return fat.iter()
        .enumerate()
        .filter(|(_, x)| **x != FREE)
        .map(|(i, x)| i as i64 * (*x) as i64)
        .sum();
}

fn part2(data: &String) {
    let result: i64 = process2(data);

    println!("{}", result);
}

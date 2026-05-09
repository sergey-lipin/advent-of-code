use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use regex::Regex;
use itertools::Itertools;
use show_image::{ImageView, ImageInfo, create_window};
use std::{thread, time};

struct Point(i64, i64);

struct Robot {
    position: Point,
    velocity: Point,
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
enum Quadrant {
    None,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft
}

#[show_image::main]
fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();
    for arg in args {
        let rows = file_to_vec(arg)?;

        let robots: Vec<Robot> = rows.iter()
            .map(|x| string_to_robot(x))
            .collect();
                
        part1(&robots);
        part2(&robots);
    }

    Ok(())
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn string_to_robot(s: &String) -> Robot {
    let re = Regex::new(r"(-*[0-9]+)").unwrap();
    let nums: Vec<i64> = re
        .find_iter(&s)
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect();
    return Robot{
        position: Point(nums[0], nums[1]),
        velocity: Point(nums[2], nums[3]),
    };
}

fn get_robot_pos(robot: &Robot, times: i64, width: i64, height: i64) -> Point {
    let mut pos = Point(
        (robot.position.0 + (robot.velocity.0 * times)) % width,
        (robot.position.1 + (robot.velocity.1 * times)) % height);
    if pos.0 < 0 {
        pos.0 += width;
    } 
    if pos.1 < 0 {
        pos.1 += height;
    } 
    return pos;
}

fn move_robot(robot: &Robot, times: i64, width: i64, height: i64) -> Quadrant {
    let pos = get_robot_pos(robot, times, width, height);
    let middle = Point(width / 2, height / 2);

    if pos.0 < middle.0 && pos.1 < middle.1 {
        return Quadrant::TopLeft;
    } else if pos.0 > middle.0 && pos.1 < middle.1 {
        return Quadrant::TopRight;
    } else if pos.0 > middle.0 && pos.1 > middle.1 {
        return Quadrant::BottomRight;
    } else if pos.0 < middle.0 && pos.1 > middle.1 {
        return Quadrant::BottomLeft;
    }
    return Quadrant::None;
}

fn part1(robots: &Vec<Robot>) {
    let result: usize = robots.iter()
        .map(|x| move_robot(x, 100, 101, 103))
        .counts()
        .iter()
        .filter(|(q, _)| **q != Quadrant::None)
        .map(|(_, n)| n)
        .product();

    println!("{}", result);
}

fn part2(robots: &Vec<Robot>) {
    let window = create_window("image", Default::default()).unwrap();

    let mut i = 0;

    loop {
        let positions = robots.iter()
            .map(|x| get_robot_pos(x, i, 101, 103));

        let mut image_bytes = vec![255u8; 101 * 103];
        for position in positions {
            image_bytes[(101 * position.1 + position.0) as usize] = 0u8;
        }

        let robot_positions: Vec<usize> = image_bytes.iter()
            .enumerate()
            .filter(|(_, x)| **x == 0u8)
            .map(|(x, _)| x)
            .collect();
        let max_count = (0..robot_positions.len())
            .chunk_by(|x| robot_positions[*x] - x)
            .into_iter()
            .map(|(_, x)| x.count())
            .max()
            .unwrap();

        if max_count > 20 {
            let image = ImageView::new(ImageInfo::mono8(101, 103), &image_bytes);
            window.set_image("image-001", image).ok();
            let ten_millis = time::Duration::from_secs(10);
            thread::sleep(ten_millis);
            break;
        }
        
        i += 1;
    }

    println!("{}", i);
}

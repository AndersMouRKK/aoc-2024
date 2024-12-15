use std::fs;
use regex::Regex;

fn read_robots(file_path: &str) -> Vec<((i32, i32), (i32, i32))> {
    let input = fs::read_to_string(file_path).expect("File not found!");
    let mut robots = Vec::new();
    let re = Regex::new(r"p=(\d{1,3}),(\d{1,3}) v=(-?\d{1,3}),(-?\d{1,3})").unwrap();
    for line in input.lines() {
        for cap in re.captures_iter(&line) {
            robots.push(((cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            (cap[3].parse().unwrap(), cap[4].parse().unwrap())));
        }
    }
    robots
}

fn part1(file_path: &str, w: i32, h: i32) -> i64 {
    let mut robots = read_robots(file_path);
    // println!("{:?}", robots);
    let sec = 100;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for i in 0..sec {
        for robot in robots.iter_mut() {
            robot.0.0 += robot.1.0;
            robot.0.1 += robot.1.1;
            if robot.0.0 < 0 {
                robot.0.0 += w;
            }
            else if robot.0.0 >= w {
                robot.0.0 -= w;
            }
            if robot.0.1 < 0 {
                robot.0.1 += h;
            }
            else if robot.0.1 >= h {
                robot.0.1 -= h;
            }
            if i < sec - 1 || robot.0.0 == w / 2 || robot.0.1 == h / 2 {
                continue;
            }
            if robot.0.0 < w / 2 && robot.0.1 < h / 2 {
                q1 += 1;
            } else if robot.0.0 >= w / 2 && robot.0.1 < h / 2 {
                q2 += 1;
            } else if robot.0.0 < w / 2 && robot.0.1 >= h / 2 {
                q3 += 1;
            } else {
                q4 += 1;
            }
        }
    }
    println!("q1: {}, q2: {}, q3: {}, q4: {}", q1, q2, q3, q4);
    q1 * q2 * q3 * q4
}

fn part2(_file_path: &str, _w: i32, _h: i32) -> i64 {
    0
}

pub fn run(part: u8, test: bool) -> i64 {
    let file_path = format!("input/{}14.txt", if test { "test/" } else { "" });
    let (w, h) = match test {
        true => (11, 7),
        false => (101, 103),
    };
    let result = match part {
        1 => part1(&file_path, w, h),
        2 => part2(&file_path, w, h),
        _ => 0,
    };
    result
}
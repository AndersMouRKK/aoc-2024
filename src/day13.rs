use std::fs;
use regex::Regex;
use gcd::Gcd;

fn get_equations(file_path: &str) -> Vec<[i32; 6]> {
    let input = fs::read_to_string(file_path).expect("File not found!");
    let re = Regex::new(r"A: X\+(\d{1,3}), Y\+(\d{1,3})[\S\s]*?B: X\+(\d{1,3}), Y\+(\d{1,3})[\S\s]*?X=(\d{1,9}), Y=(\d{1,9})").unwrap();
    let mut equations = Vec::new();
    for cap in re.captures_iter(&input) {
        equations.push([
            cap[1].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[5].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[4].parse().unwrap(),
            cap[6].parse().unwrap(),
        ]);
    }
    equations
}

fn part1(file_path: &str) -> usize {
    let equations = get_equations(file_path);
    println!("Equations: {}", equations.len());
    
    let mut equal_as = 0;
    let mut a_floats = 0;
    let mut b_floats = 0;
    let mut sub_zero = 0;
    let mut over_100 = 0;
    let mut sum = 0;

    for eq in equations {
        
        /* Solving for y first as an expirement -- gave exact same result though
        
        let mult = eq[0] * eq[3] / (eq[0] as usize).gcd(eq[3] as usize) as i32;
        let a1 = mult * eq[1] / eq[0];
        let b1 = mult * eq[2] / eq[0];
        let a2 = mult * eq[4] / eq[3];
        let b2 = mult * eq[5] / eq[3];

        let y = (b1 - b2) / (a1 - a2);
        let x = (eq[2] - eq[1] * y) / eq[0];
        if (eq[2] - eq[1] * y) % eq[0] > 0 {
            b_floats += 1;
            continue;
        }
        */

        let mult = eq[1] * eq[4] / (eq[1] as usize).gcd(eq[4] as usize) as i32;
        let a1 = mult * eq[0] / eq[1];
        let b1 = mult * eq[2] / eq[1];
        let a2 = mult * eq[3] / eq[4];
        let b2 = mult * eq[5] / eq[4];

        if a1 == a2 {
            equal_as += 1;
        }
        if (b1 - b2) % (a1 - a2) > 0 {
            a_floats += 1;
            continue;
        }

        let x = (b1 - b2) / (a1 - a2);
        if (eq[2] - eq[0] * x) % eq[1] > 0 {
            b_floats += 1;
            continue;
        }
        let y = (eq[2] - eq[0] * x) / eq[1];
        if x > 100 || y > 100 {
            over_100 += 1;
            continue;
        }
        if x < 0 || y < 0 {
            sub_zero += 1;
            continue;
        }

        // println!("x: {}, y: {}, mult: {}, a1: {}, b1: {}, a2: {}, b2: {}", x, y, mult, a1, b1, a2, b2);
        sum += 3 * x + y;
    }

    println!("equal a_s: {}, a_floats: {}, b_floats: {}, over_100: {}, sub_zero: {}", equal_as, a_floats, b_floats, over_100, sub_zero);

    // sum as usize
    sum as usize
}

fn part2(_file_path: &str) -> usize {
    0
}

pub fn run(part: u8, test: bool) -> usize {
    let file_path = format!("input/{}13.txt", if test { "test/" } else { "" });
    let result = match part {
        1 => part1(&file_path),
        2 => part2(&file_path),
        _ => 0,
    };
    result
}
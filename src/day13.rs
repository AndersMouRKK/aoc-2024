use std::{collections::HashMap, fs};
use regex::Regex;
use gcd::Gcd;

fn get_equations(file_path: &str) -> Vec<[i64; 6]> {
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

#[derive(Debug, PartialEq, Eq, Hash)]
enum EqErr { EqualAs, AFloats, BFloats, LimitExceeded, SubZero }

fn solve_eq(eq: [i64; 6], for_x: bool, prize_offset: i64, max_presses: Option<i64>) -> Result<(i64, i64), EqErr> {
    let mult;
    let a1;
    let b1;
    let a2;
    let b2;
    let x;
    let y;
    let prize_x = eq[2] + prize_offset;
    let prize_y = eq[5] + prize_offset;

    if for_x {
        mult = eq[1] * eq[4] / (eq[1] as usize).gcd(eq[4] as usize) as i64;
        a1 = mult * eq[0] / eq[1];
        b1 = mult * prize_x / eq[1];
        a2 = mult * eq[3] / eq[4];
        b2 = mult * prize_y / eq[4];
    } else {
        mult = eq[0] * eq[3] / (eq[0] as usize).gcd(eq[3] as usize) as i64;
        a1 = mult * eq[1] / eq[0];
        b1 = mult * prize_x / eq[0];
        a2 = mult * eq[4] / eq[3];
        b2 = mult * prize_y / eq[3];
    }

    if a1 == a2 {
        return Err(EqErr::EqualAs);
    }

    let b_diff = b1 - b2;
    let a_diff = a1 - a2;
    // HINT: Remember to disregard negative mod results
    // My mistake of doing > 0 earlier took a while to uncover! xD
    if b_diff % a_diff != 0 {
        return Err(EqErr::AFloats);
    }

    if for_x {
        x = b_diff / a_diff;
        if (prize_x - eq[0] * x) % eq[1] > 0 {
            return Err(EqErr::BFloats);
        }
        y = (prize_x - eq[0] * x) / eq[1];
    } else {
        y = b_diff / a_diff;
        if (prize_x - eq[1] * y) % eq[0] != 0 {
            return Err(EqErr::BFloats);
        }
        x = (prize_x - eq[1] * y) / eq[0];
    }

    match max_presses {
        Some(max) => {
            if x > max || y > max {
                return Err(EqErr::LimitExceeded);
            }
        },
        None => {}
    }
    if x < 0 || y < 0 {
        return Err(EqErr::SubZero);
    }

    Ok((x, y))
}

fn part1(file_path: &str) -> i64 {
    let equations = get_equations(file_path);
    println!("Equation count: {}", equations.len());

    let mut errors = HashMap::new();
    let mut valid = 0;
    let mut sum = 0;

    for eq in equations {
        
        let (x, y) = match solve_eq(eq, true, 0, Some(100)) {
            Ok((x, y)) => (x, y),
            Err(e) => {
                *errors.entry(e).or_insert(0) += 1;
                continue;
            },
        };
        valid += 1;
        sum += 3 * x + y;
    }

    println!("Valid: {}, Errors: {:?}", valid, errors);

    // sum as usize
    sum as i64
}

fn part2(file_path: &str) -> i64 {
    let equations = get_equations(file_path);
    println!("Equation count: {}", equations.len());

    let prize_offset = 10000000000000;
    let mut errors = HashMap::new();
    let mut for_x_valid = 0;
    let mut for_y_valid = 0;
    let mut both_valid = 0;
    let mut diff_results = 0;
    let mut sum = 0;

    for eq in equations {

        // This can be simplified to a single function call like in part1
        // Did both for debugging purposes (see comment about negative mod results in solve_eq)
        let for_x_result = solve_eq(eq, true, prize_offset, None);
        let for_y_result = solve_eq(eq, false, prize_offset, None);
        
        if for_x_result.is_err() && for_y_result.is_err() {
            *errors.entry(for_x_result.err().unwrap()).or_insert(0) += 1;
            continue;
        }

        if !for_x_result.is_err() && for_y_result.is_err() {
            let (x, y) = for_x_result.unwrap();
            for_x_valid += 1;
            sum += 3 * x + y;
            continue;
        }
        if for_x_result.is_err() && !for_y_result.is_err() {
            let (x, y) = for_y_result.unwrap();
            for_y_valid += 1;
            sum += 3 * x + y;
            continue;
        }

        let (x1, y1) = for_x_result.unwrap();
        let (x2, y2) = for_y_result.unwrap();
        if x1 != x2 || y1 != y2 {
            
            println!("Different results: ({}, {}) vs ({}, {})", x1, y1, x2, y2);
            diff_results += 1;
            continue;
        }

        both_valid += 1;
        sum += 3 * x1 + y1;
    }

    println!("For X valid: {}, For Y valid: {}, Both valid: {}, Errors: {:?}", for_x_valid, for_y_valid, both_valid, errors);
    println!("Different results: {}", diff_results);

    // sum as usize
    sum as i64
}

pub fn run(part: u8, test: bool) -> i64 {
    let file_path = format!("input/{}13.txt", if test { "test/" } else { "" });
    let result = match part {
        1 => part1(&file_path),
        2 => part2(&file_path),
        _ => 0,
    };
    result
}
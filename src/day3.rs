use std::fs;
use regex::Regex;

fn get_mul_digits(input: &str, check_dos: bool) -> Vec<(i32, i32)> {
    let mut invalid_ranges: Vec<(usize, usize)> = Vec::new();
    if check_dos {
        // Sub-optimal regex, as it needs the messy step below
        let re = Regex::new(r"don't\(\)([\S\s]*?)do\(\)").unwrap();
        for cap in re.captures_iter(&input) {
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end();
            invalid_ranges.push((start, end));
        }

        // Messy last-minute solution to find first "don't()" after the last "do()" :/
        if !invalid_ranges.is_empty() {

            let idx_last_do = invalid_ranges.last().unwrap().1;
            let substr = &input[idx_last_do..];
            let idx_dont = substr.find("don't()").unwrap_or(0);

            if idx_dont > 0 {
                invalid_ranges.push((idx_last_do + idx_dont, input.len()));
            }
        }
    }

    // Match and capture groups with digits using regex on input string with the pattern mul(x,y) where x and y are 1-3 digits
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut digits: Vec<(i32, i32)> = Vec::new();

    for cap in re.captures_iter(&input) {
        if check_dos {
            let mut invalid: bool = false;
            let start = cap.get(0).unwrap().start();
            for (s, e) in &invalid_ranges {
                if start >= *s && start <= *e {
                    invalid = true;
                    break;
                }
            }
            if invalid {
                continue;
            }
        }

        // Parse the two integers from the capture groups
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();

        // Append the tuple to the vector
        digits.push((x, y));        
    }

    digits
}

pub fn part1() {
    println!("Day 3, part 1!");

    let input = fs::read_to_string("input/3.txt").expect("Failed to read input file");

    let digits = get_mul_digits(&input, false);
    
    // Sum the products of the two integers in each tuple
    let mut sum: i32 = 0;
    for (x, y) in digits {
        sum += x * y;
    }

    println!("Sum: {}", sum);
}

pub fn part2() {
    println!("Day 3, part 2!");

    let input = fs::read_to_string("input/3.txt").expect("Failed to read input file");

    let digits = get_mul_digits(&input, true);
    
    // Sum the products of the two integers in each tuple
    let mut sum: i32 = 0;
    for (x, y) in digits {
        sum += x * y;
    }

    println!("Sum: {}", sum);
}
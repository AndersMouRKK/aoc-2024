use std::fs;
use regex::Regex;

pub fn part1() {
    println!("Day 3, part 1!");

    let input = fs::read_to_string("input/3.txt").expect("Failed to read input file");

    // Match and capture groups with digits using regex on input string with the pattern mul(x,y) where x and y are 1-3 digits
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut digits: Vec<(i32, i32)> = Vec::new();

    for cap in re.captures_iter(&input) {
        // Parse the two integers from the capture groups
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();

        // Append the tuple to the vector
        digits.push((x, y));
    }
    
    // Sum the products of the two integers in each tuple
    let mut sum: i32 = 0;
    for (x, y) in digits {
        sum += x * y;
    }

    println!("Sum: {}", sum);
}
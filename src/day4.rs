use std::fs;

fn count_matches(haystack: &str, needles: [&str; 2]) -> usize {
    needles.iter().fold(0, |acc, &s| acc + haystack.matches(s).count())
}

pub fn part1() {
    println!("Day 4, part 1!");
    let input = fs::read_to_string("input/4.txt").expect("File not found!");
    
    let search_str = "XMAS";
    let reverse_str = search_str.chars().rev().collect::<String>();
    let search_strings = [search_str, reverse_str.as_str()];

    // Read input as 2D array of characters and find all horizontal matches in the process
    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut hor_hits = 0;

    let mut last_line_length = 0;

    for line in input.lines() {
        hor_hits += count_matches(line, search_strings);
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            chars.push(c);
        }
        lines.push(chars);
        if (last_line_length != 0) && (last_line_length != line.len()) {
            panic!("Line length mismatch!");
        }
        last_line_length = line.len();
    }

    // println!("Last line length: {}", last_line_length);
    // println!("Number of lines: {}", lines.len());
    
    let mut ver_hits = 0;
    let mut se_hits = 0;
    let mut sw_hits = 0;

    // In hindsight, I could have searched the longer diagonals rather than short, fixed ones
    for x in 0..last_line_length {
        let check_se = x <= last_line_length - search_str.len();
        let check_sw = x >= search_str.len() - 1;
        
        let mut column: String = String::new();
        for y in 0..lines.len() {
            column.push(lines[y][x]);

            if y > lines.len() - search_str.len() {
                continue;
            }
            
            if check_se {
                let mut se: String = String::new();
                for i in 0..search_str.len() {
                    se.push(lines[y + i][x + i]);
                }
                if search_strings.iter().any(|&s| s == se) {
                    se_hits += 1;
                }
            }
            
            if check_sw {
                let mut sw: String = String::new();
                for i in 0..search_str.len() {
                    sw.push(lines[y + i][x - i]);
                }
                if search_strings.iter().any(|&s| s == sw) {
                    sw_hits += 1;
                }
            }
        }
        ver_hits += count_matches(column.as_str(), search_strings);
    }
    
    println!("Horizontal hits: {}", hor_hits);
    println!("Vertical hits: {}", ver_hits);
    println!("SE hits: {}", se_hits);
    println!("SW hits: {}", sw_hits);

    println!("Total hits: {}", hor_hits + ver_hits + se_hits + sw_hits);
}
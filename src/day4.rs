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

    let mut width = 0;

    for line in input.lines() {
        hor_hits += count_matches(line, search_strings);
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            chars.push(c);
        }
        lines.push(chars);
        if (width != 0) && (width != line.len()) {
            panic!("Line length mismatch!");
        }
        width = line.len();
    }

    let height = lines.len();

    println!("Width: {}", width);
    println!("Height: {}", height);
    
    let mut ver_hits = 0;
    let mut se_hits = 0;
    let mut sw_hits = 0;

    // In hindsight, I could have searched the longer diagonals rather than short, fixed ones
    for x in 0..width {
        let check_se = x <= width - search_str.len();
        let check_sw = x >= search_str.len() - 1;
        
        let mut column: String = String::new();
        for y in 0..height {
            column.push(lines[y][x]);

            if y > height - search_str.len() {
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

pub fn part2() {
    println!("Day 4, part 2!");
    let input = fs::read_to_string("input/4.txt").expect("File not found!");

    // Read input into a 2D array of characters
    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut width = 0;
    for line in input.lines() {
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            chars.push(c);
        }
        lines.push(chars);
        if (width != 0) && (width != line.len()) {
            panic!("Line length mismatch!");
        }
        width = line.len();
    }
    let height = lines.len();

    println!("Width: {}", width);
    println!("Height: {}", height);

    let search_str = "MAS";
    let reverse_str = search_str.chars().rev().collect::<String>();
    let search_strings = [search_str, reverse_str.as_str()];
    // "Radius"
    let r = search_str.len() / 2;
    let middle_char = search_str.chars().nth(r).unwrap();

    let mut hits = 0;
    for y in r..height - r {
        for x in r..width - r {
            if lines[y][x] != middle_char {
                continue;
            }
            // Needs a bit more work to support other radii than 1
            let fwd_slash = [lines[y + r][x - r], lines[y][x], lines[y - r][x + r]];
            
            // Continue if fwd_slash is not one of the search_strings
            if !search_strings.iter().any(|&s| s == fwd_slash.iter().collect::<String>()) {
                continue;
            }

            let bwd_slash = [lines[y - r][x - r], lines[y][x], lines[y + r][x + r]];
            if search_strings.iter().any(|&s| s == bwd_slash.iter().collect::<String>()) {
                hits += 1;
            }
        }
    }

    println!("Hits: {}", hits);
}
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use gcd::Gcd;

fn read_dim_and_freqs(file_path: &str) -> ((usize, usize), HashMap<char, Vec<(i32, i32)>>) {
    let input = fs::read_to_string(file_path).expect("File not found!");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut dim = (0, 0);
    let mut freqs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
 
    for line in input.lines() {
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if c != '.' {
                let entry = freqs.entry(c).or_insert(Vec::new());
                entry.push((chars.len() as i32, map.len() as i32));
            }
            chars.push(c);
        }
        map.push(chars);
        if (dim.0 != 0) && (dim.0 != line.len()) {
            panic!("Line length mismatch!");
        }
        dim.0 = line.len();
    }
    dim.1 = map.len();
    (dim, freqs)
}

fn within_bounds(dim: (usize, usize), pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < (dim.0 as i32) && pos.1 >= 0 && pos.1 < (dim.1 as i32)
}

fn part1(file_path: &str) -> usize {
    let (dim, freqs) = read_dim_and_freqs(file_path);

    let mut all_anti_pos = HashSet::new();
    for freq in freqs {
        let positions = freq.1;
        for i in 0..positions.len()-1 {
            let pos_1 = positions[i];
            for j in i+1..positions.len() {
                let pos_2 = positions[j];
                let diff = (pos_2.0 - pos_1.0, pos_2.1 - pos_1.1);
                let anti_positions = [
                    (pos_1.0 - diff.0, pos_1.1 - diff.1),
                    (pos_2.0 + diff.0, pos_2.1 + diff.1)
                ];
                for anti_pos in anti_positions.iter() {
                    if !all_anti_pos.contains(anti_pos) && within_bounds(dim, *anti_pos) {
                        all_anti_pos.insert(*anti_pos);
                    }
                }
            }
        }
    }
    all_anti_pos.len()
}

fn part2(file_path: &str) -> usize {
    let (dim, freqs) = read_dim_and_freqs(file_path);

    let mut all_anti_pos = HashSet::new();
    for freq in freqs {
        let positions = freq.1;
        for i in 0..positions.len()-1 {
            let pos_1 = positions[i];
            all_anti_pos.insert(pos_1);
            for j in i+1..positions.len() {
                let pos_2 = positions[j];
                let mut diff = (pos_2.0 - pos_1.0, pos_2.1 - pos_1.1);

                // Reduce diff vector to smallest possible values by dividing by gcd
                let gcd = (diff.0.abs() as usize).gcd(diff.1.abs() as usize) as i32;
                diff = (diff.0 / gcd, diff.1 / gcd);
                
                // Run to bounds in either direction from pos_1
                let mut anti_pos = (pos_1.0 - diff.0, pos_1.1 - diff.1);
                while within_bounds(dim, anti_pos) {
                    all_anti_pos.insert(anti_pos);
                    anti_pos = (anti_pos.0 - diff.0, anti_pos.1 - diff.1);
                }
                anti_pos = (pos_1.0 + diff.0, pos_1.1 + diff.1);
                while within_bounds(dim, anti_pos) {
                    all_anti_pos.insert(anti_pos);
                    anti_pos = (anti_pos.0 + diff.0, anti_pos.1 + diff.1);
                }
            }
        }
    }
    all_anti_pos.len()
}

pub fn run(part: u8, test: bool) -> usize {
    let file_path = format!("input/{}8.txt", if test { "test/" } else { "" });
    let result = match part {
        1 => part1(&file_path),
        2 => part2(&file_path),
        _ => 0,
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(run(1, true), 14);
    }

    #[test]
    fn part2() {
        assert_eq!(run(2, true), 34);
    }
}
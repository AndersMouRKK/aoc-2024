use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn read_map(path: &str) -> (Vec<Vec<char>>, (usize, usize), HashMap<char, Vec<(i32, i32)>>) {
    let input = fs::read_to_string(path).expect("File not found!");

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
    (map, dim, freqs)
}

fn within_bounds(dim: (usize, usize), pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < (dim.0 as i32) && pos.1 >= 0 && pos.1 < (dim.1 as i32)
}

pub fn part1() {
    println!("Day 8, part 1");

    let (_map, dim, freqs) = read_map("input/8.txt");

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
    println!("Result: {}", all_anti_pos.len());
}
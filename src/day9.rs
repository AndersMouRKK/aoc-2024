use std::fs;
use std::collections::HashMap;

fn read_blocks(file_path: &str) -> Vec<i32> {
    let input = fs::read_to_string(file_path).expect("File not found!");
    let mut blocks = Vec::new();
    let mut is_file = true;
    let mut file_idx = 0;
    for c in input.chars() {
        for _i in 0..c.to_digit(10).unwrap() {
            blocks.push(match is_file {
                true => file_idx,
                false => -1,
            });
        }
        if is_file {
            file_idx += 1;
        }
        is_file = !is_file;
    }
    blocks
}

fn get_keys_by_index0(spaces: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut keys = Vec::new();
    for key in spaces.keys() {
        keys.push(*key);
    }
    keys.sort_by(|a, b| {
        spaces.get(a).unwrap()[0].cmp(&spaces.get(b).unwrap()[0])
    });
    keys
}

fn part1(file_path: &str) -> i64 {
    // Basic (naiive) solution
    let mut blocks = read_blocks(file_path);
    let mut i = 0;
    let mut file_idx = blocks.len() - 1;
    while i < file_idx {
        let val_i = blocks[i];
        if val_i == -1 {
            for j in (i+1..file_idx+1).rev() {
                let val_j = blocks[j];
                if val_j == -1 { 
                    continue;
                }
                blocks.swap(i, j);
                file_idx = j - 1;
                break;
            }
        }
        i += 1;
    }

    // Calc checksum
    let mut sum: i64 = 0;
    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            sum += (i as i64) * blocks[i] as i64;
        }
    }
    sum
}

fn part2(file_path: &str) -> i64 {
    let mut blocks = read_blocks(file_path);

    // Store spaces with size and start indices for fast lookup
    let mut spaces = HashMap::new();
    let mut size = 0;
    for i in 0..blocks.len() {
        if blocks[i] == -1 {
            size += 1;
        } else if size > 0 {
            let entry = spaces.entry(size).or_insert(Vec::new());
            entry.push(i - size);
            size = 0;
        }
    }
    // println!("Spaces: {:?}", spaces);
    let mut ordered_spaces = get_keys_by_index0(&spaces);
    // println!("Ordered spaces: {:?}", ordered_spaces);

    size = 0;
    let mut last_val = -2;
    for i in (0..blocks.len()).rev() {
        if blocks[i] != -1 && (i == blocks.len()-1 || blocks[i] == last_val || last_val == -1) {
            size += 1;
        } else if size > 0 {
            // println!("i: {}, size: {}, last_val: {}, blocks[i]: {}", i, size, last_val, blocks[i]);
            let mut did_swap = false;
            for space_size in ordered_spaces.iter() {
                if space_size < &size {
                    continue;
                }
                let space_indices = spaces.get_mut(space_size).unwrap();
                if space_indices[0] > i {
                    continue;
                }
                // println!("Size: {}, Space size: {}, space_indices: {:?}", size, space_size, space_indices);
                let space_idx = space_indices.remove(0);
                // println!("Moving to Space idx: {}", space_idx);
                did_swap = true;
                for j in 0..size {
                    blocks.swap(i + 1 + j, space_idx + j);
                }
                if space_indices.is_empty() {
                    spaces.remove(space_size);
                }
                if space_size > &size {
                    spaces.entry(space_size - size).or_insert(Vec::new()).push(space_idx + size);
                    spaces.get_mut(&(space_size - size)).unwrap().sort();
                }
                break;
            } 
            if did_swap {
                // println!("Blocks: {:?}", blocks);
                ordered_spaces = get_keys_by_index0(&spaces);
                if ordered_spaces.len() == 0 || spaces.get(&ordered_spaces[0]).unwrap()[0] > i {
                    break;
                }
            }
            if blocks[i] != -1 {
                size = 1;
            } else {
                size = 0;
            }
        }
        last_val = blocks[i];
    }
    // println!("{:?}", blocks);

    // Calc checksum
    let mut sum: i64 = 0;
    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            sum += (i as i64) * blocks[i] as i64;
        }
    }

    sum
}

pub fn run(part: u8, test: bool) -> i64 {
    let file_path = format!("input/{}9.txt", if test { "test/" } else { "" });
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
    use simple_stopwatch::Stopwatch;

    #[test]
    fn read_blocks_test() {
        let blocks = read_blocks("input/test/9.txt");
        let mut vec = Vec::new();
        for c in "00...111...2...333.44.5555.6666.777.888899".chars() {
            vec.push(match c == '.' {
                true => -1,
                false => c.to_digit(10).unwrap() as i32,
            });
        }
        assert_eq!(blocks, vec);
    }

    #[test]
    fn read_blocks_profile() {
        let sw = Stopwatch::start_new();
        read_blocks("input/9.txt");
        assert_eq!(sw.ms() < 2.0, true);
    }
    
    #[test]
    fn part1() {
        assert_eq!(run(1, true), 1928);
    }
}   
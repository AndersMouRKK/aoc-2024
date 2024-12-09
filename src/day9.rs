use std::fs;

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

fn part2(_file_path: &str) -> i64 {
    0
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
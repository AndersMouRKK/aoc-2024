use std::fs;

fn read_numbers(file_path: &str) -> Vec<i64> {
    fs::read_to_string(file_path)
        .expect("File not found!")
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part1(file_path: &str) -> i64 {
    let mut stones = read_numbers(file_path);
    for _i in 0..25 {
        let mut new_stones = Vec::new();
        for n in stones {
            if n == 0 {
                new_stones.push(1);
                continue;
            } 
            let str = n.to_string();
            let str_len = str.len();
            if str_len % 2 == 0 {
                new_stones.push(str[..str_len/2].parse().unwrap());
                new_stones.push(str[str_len/2..].parse().unwrap());
            } else {
                new_stones.push(n * 2024);
            }
        }
        stones = new_stones;
    }

    stones.len() as i64
}

fn part2(_file_path: &str) -> i64 {
    // TODO
    0
}

pub fn run(part: u8, test: bool) -> i64 {
    let file_path = format!("input/{}11.txt", if test { "test/" } else { "" });
    let result = match part {
        1 => part1(&file_path),
        2 => part2(&file_path),
        _ => 0,
    };
    result
}
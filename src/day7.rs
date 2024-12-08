use std::fs;

fn is_correct(nums: &Vec<&str>, i: usize, delta: usize, res: usize, str_concat: bool) -> bool {
    if delta > res {
        return false;
    }
    if i == nums.len() - 1 {
        return delta == res;
    }
    is_correct(nums, i + 1, delta + nums[i + 1].parse::<usize>().unwrap(), res, str_concat) || 
    is_correct(nums, i + 1, delta * nums[i + 1].parse::<usize>().unwrap(), res, str_concat) || 
    (str_concat && is_correct(nums, i + 1, (delta.to_string() + nums[i + 1]).parse::<usize>().unwrap(), res, str_concat))
}

pub fn part1() {
    println!("Day 7, part 1");
    let input = fs::read_to_string("input/7.txt").expect("File not found!");

    let mut total_sum = 0;
    for line in input.lines() {
        let col_split = line.split(":").collect::<Vec<&str>>();
        let res: usize = col_split[0].parse().unwrap();
        let nums = col_split[1].split_whitespace().collect::<Vec<&str>>();
        if is_correct(&nums, 0, nums[0].parse::<usize>().unwrap(), res as usize, false) {
            total_sum += res;
        }
    }
    println!("Result: {}", total_sum);
}

pub fn part2() {
    println!("Day 7, part 2");
    let input = fs::read_to_string("input/7.txt").expect("File not found!");

    let mut total_sum = 0;
    for line in input.lines() {
        let col_split = line.split(":").collect::<Vec<&str>>();
        let res: usize = col_split[0].parse().unwrap();
        let nums = col_split[1].split_whitespace().collect::<Vec<&str>>();
        if is_correct(&nums, 0, nums[0].parse::<usize>().unwrap(), res as usize, true) {
            total_sum += res;
        }
    }
    println!("Result: {}", total_sum);
}
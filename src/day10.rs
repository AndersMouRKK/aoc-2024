use std::fs;
use std::collections::HashSet;

fn read_map(file_path: &str) -> (Vec<Vec<usize>>, (usize, usize)) {
    let input = fs::read_to_string(file_path).expect("File not found!");
    let map: Vec<Vec<usize>> = input.lines().map(|l| l.chars().map(|n| n.to_digit(10).unwrap() as usize).collect()).collect();
    let dim = (map[0].len(), map.len());
    (map, dim)
}

fn within_bounds(dim: (usize, usize), pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < (dim.0 as i32) && pos.1 >= 0 && pos.1 < (dim.1 as i32)
}

fn trailhead_score(map: &Vec<Vec<usize>>, dim: (usize, usize), pos: (i32, i32), mut path: HashSet<(i32, i32)>, ends: &mut HashSet<(i32, i32)>) -> usize {
    let val = map[pos.1 as usize][pos.0 as usize];
    if val == 9 {
        match ends.insert(pos) {
            true => return 1,
            false => return 0,
        }
    }

    let neighbours = vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1), (pos.0, pos.1 - 1), (pos.0, pos.1 + 1)];
    let mut valid = Vec::new();
    for n in neighbours {
        if !path.contains(&n) && within_bounds(dim, n) && map[n.1 as usize][n.0 as usize] == val + 1 {
            valid.push(n);
        }
    }
    path.insert(pos);
    valid.iter().fold(0, |acc, n| acc + trailhead_score(map, dim, *n, path.clone(), ends))
}

fn part1(file_path: &str) -> usize {
    let (map, dim) = read_map(file_path);
    let mut sum = 0;
    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if map[y][x] == 0 {
                sum += trailhead_score(&map, dim, (x as i32, y as i32), HashSet::new(), &mut HashSet::new());
            }
        }
    }
    sum
}

fn part2(_file_path: &str) -> usize {
    // TODO
    0
}

pub fn run(part: u8, test: bool) -> usize {
    let file_path = format!("input/{}10.txt", if test { "test/" } else { "" });
    let result = match part {
        1 => part1(&file_path),
        2 => part2(&file_path),
        _ => 0,
    };
    result
}
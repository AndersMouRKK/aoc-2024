use std::fs;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

fn read_map(file_path: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let input = fs::read_to_string(file_path).expect("File not found!");
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let dim = (map[0].len(), map.len());
    (map, dim)
}

fn within_bounds(dim: (usize, usize), pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < (dim.0 as i32) && pos.1 >= 0 && pos.1 < (dim.1 as i32)
}

fn find_area(map: &Vec<Vec<char>>, dim: (usize, usize), pos: (i32, i32), fence: &mut usize, left: &mut HashSet<(i32, i32)>) {
    if !left.remove(&pos) {
        return;
    }
    let val = map[pos.1 as usize][pos.0 as usize];
    let neighbours = vec![(pos.0, pos.1 - 1), (pos.0 + 1, pos.1), (pos.0, pos.1 + 1), (pos.0 - 1, pos.1)];
    for n in neighbours {
        if within_bounds(dim, n) && map[n.1 as usize][n.0 as usize] == val {
            find_area(map, dim, n, fence, left);
            continue;
        }
        *fence += 1;
    }
}

fn find_area_pt2(map: &Vec<Vec<char>>, dim: (usize, usize), pos: (i32, i32), fences: &mut HashMap<(bool, bool, i32), Vec<i32>>, left: &mut HashSet<(i32, i32)>) {
    if !left.remove(&pos) {
        return;
    }
    let val = map[pos.1 as usize][pos.0 as usize];
    let neighbours = vec![(pos.0, pos.1 - 1), (pos.0 + 1, pos.1), (pos.0, pos.1 + 1), (pos.0 - 1, pos.1)];
    for n in neighbours {
        if within_bounds(dim, n) && map[n.1 as usize][n.0 as usize] == val {
            find_area_pt2(map, dim, n, fences, left);
            continue;
        }
        let horizontal = n.0 == pos.0;
        let x = cmp::min(n.0, pos.0);
        let y = cmp::min(n.1, pos.1);
        let (key_axis, above, search_axis) = match horizontal {
            true => (y, pos.1 > n.1, x),
            false => (x, pos.0 > n.0, y),  
        };
        fences.entry((horizontal, above, key_axis)).or_insert(Vec::new()).push(search_axis);
    }
}

fn part1(file_path: &str) -> usize {
    let (map, dim) = read_map(file_path);
    let mut left = HashSet::new();
    for y in 0..dim.1 {
        for x in 0..dim.0 {
            left.insert((x as i32, y as i32));
        }
    }

    let mut sum = 0;
    while left.len() > 0 {
        let pos = left.iter().next().unwrap().clone();
        let mut fence = 0;
        let start_area = left.len();
        find_area(&map, dim, pos, &mut fence, &mut left);
        sum += (start_area - left.len()) * fence;
    }

    sum
}

fn part2(file_path: &str) -> usize {
    let (map, dim) = read_map(file_path);
    let mut left = HashSet::new();
    for y in 0..dim.1 {
        for x in 0..dim.0 {
            left.insert((x as i32, y as i32));
        }
    }

    let mut sum = 0;
    while left.len() > 0 {
        let pos = left.iter().next().unwrap().clone();
        let start_area = left.len();
        let mut fences = HashMap::new();
        find_area_pt2(&map, dim, pos, &mut fences, &mut left);

        let mut sides = 0;
        for values in fences.values() {
            let mut sorted = values.clone();
            sorted.sort();
            let mut prev = -2;
            for v in sorted {
                if v > prev + 1 {
                    sides += 1;
                }
                prev = v;
            }
        }
        sum += (start_area - left.len()) * sides;
    }

    sum
}

pub fn run(part: u8, test: bool) -> usize {
    let file_path = format!("input/{}12.txt", if test { "test/" } else { "" });
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
    fn part1_test() {
        assert_eq!(run(1, true), 1930);
    }

    #[test]
    fn part2_test() {
        assert_eq!(run(2, true), 1206);
    }

    #[test]
    fn part2_test2() {
        assert_eq!(part2("input/test/12-2.txt"), 236);
    }

    #[test]
    fn part2_test3() {
        assert_eq!(part2("input/test/12-3.txt"), 368);
    }
}
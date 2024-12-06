use std::fs;
use std::collections::HashSet;
use simple_stopwatch::Stopwatch;

fn get_pos(bounds: &(usize, usize), pos: &(usize, usize), vel: &(i32, i32)) -> Option<(usize, usize)> {
    let next = (pos.0 as i32 + vel.0, pos.1 as i32 + vel.1);
    if (next.0 < 0) || (next.0 >= bounds.0 as i32) || (next.1 < 0) || (next.1 >= bounds.1 as i32) {
        return None
    }
    Some((next.0 as usize, next.1 as usize))
}

fn read_map(path: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let input = fs::read_to_string(path).expect("File not found!");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut dim = (0, 0);
    let mut pos = (0, 0);
 
    for line in input.lines() {
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '^' {
                pos = (chars.len(), map.len());
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
    (map, dim, pos)
}

fn run_map(map: &Vec<Vec<char>>, dim: (usize, usize), start_pos: (usize, usize), start_vel: (i32, i32)) -> Option<(usize, Vec<((usize, usize), (i32, i32))>)> {
    let mut pos = start_pos;
    let mut vel = start_vel;
    let mut visited = HashSet::new();
    let mut path = HashSet::new();
    let mut ordered_path = Vec::new();
    'outer: loop {
        let mut next = match get_pos(&dim, &pos, &vel) {
            Some(p) => p,
            None => break,
        };
        let mut blocked = map[next.1][next.0] == '#';
        while blocked {
            vel = match vel {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!("Invalid velocity!"),
            };
            next = match get_pos(&dim, &pos, &vel) {
                Some(p) => p,
                None => break 'outer,
            };
            blocked = map[next.1][next.0] == '#';
        }
        pos = next;
        if path.contains(&(pos, vel)) {
            return None;
        }
        visited.insert(pos);
        path.insert((pos, vel));
        ordered_path.push((pos, vel));
    }
    Some((visited.len(), ordered_path))
}

pub fn part1() {
    println!("Day 6, part 1");

    let (map, dim, pos) = read_map("input/6.txt");
    println!("Dim: {:?}", dim);

    let visited = match run_map(&map, dim, pos, (0, -1)) {
        Some((v, _p)) => v,
        None => panic!("Failed to run map!"),
    };
    println!("Visited: {}", visited);
}

pub fn part2() {
    println!("Day 6, part 2");

    let (mut map, dim, start_pos) = read_map("input/6.txt");
    let path = match run_map(&map, dim, start_pos, (0, -1)) {
        Some((_v, p)) => p,
        None => panic!("Failed to run map!"),
    };
    
    // Very slow solution
    let sw = Stopwatch::start_new();

    let mut player_pos = start_pos;
    let mut player_vel = (0, -1);
    let mut blocks_placed = HashSet::new();
    let mut loops = 0;
    for i in 0..path.len() {
        let pair = path[i];
        let pos = pair.0;

        if blocks_placed.contains(&pos) {
            player_pos = pos;
            player_vel = pair.1;
            continue;
        }
        map[pos.1][pos.0] = '#';
        blocks_placed.insert(pos);

        match run_map(&map, dim, player_pos, player_vel) {
            Some(_) => {},
            None => loops += 1,
        }

        map[pos.1][pos.0] = '.';
        
        player_pos = pos;
        player_vel = pair.1;
    }

    println!("Loops: {}, time spent: {} ms", loops, sw.ms());
}
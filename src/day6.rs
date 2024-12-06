use std::fs;

fn get_pos(bounds: &(usize, usize), pos: &(usize, usize), vel: &(i32, i32)) -> Option<(usize, usize)> {
    let next = (pos.0 as i32 + vel.0, pos.1 as i32 + vel.1);
    if (next.0 < 0) || (next.0 >= bounds.0 as i32) || (next.1 < 0) || (next.1 >= bounds.1 as i32) {
        return None
    }
    Some((next.0 as usize, next.1 as usize))
}

pub fn part1() {
    println!("Day 6, part 1!");
    let input = fs::read_to_string("input/6.txt").expect("File not found!");

    // Read input as 2D array of characters and find all horizontal matches in the process
    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut dim = (0, 0);
    let mut pos = (0, 0);

    for line in input.lines() {
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '^' {
                pos = (chars.len(), lines.len());
            }
            chars.push(c);
        }
        lines.push(chars);
        if (dim.0 != 0) && (dim.0 != line.len()) {
            panic!("Line length mismatch!");
        }
        dim.0 = line.len();
    }

    dim.1 = lines.len();
    println!("Dim: {:?}", dim);

    let mut vel: (i32, i32) = (0, -1);
    let mut visited = 0;
    'outer: loop {
        let mut next = match get_pos(&dim, &pos, &vel) {
            Some(p) => p,
            None => break,
        };
        let mut blocked = lines[next.1][next.0] == '#';
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
            blocked = lines[next.1][next.0] == '#';
        }
        pos = next;
        if lines[pos.1][pos.0] != 'X' {
            lines[pos.1][pos.0] = 'X';
            visited += 1;
        }
    }
    println!("Visited: {}", visited);
}
use std::{collections::HashSet, fs};
use colored::Colorize;

struct State {
    map: Vec<Vec<char>>,
    dim: (usize, usize),
    moves: Vec<char>,
    player_pos: (i32, i32),
}

impl State {
    pub fn print_map(&self, dir: Option<(i32, i32)>, dir_char: char, dir_color: (u8, u8, u8)) {
        println!("-- Map -- Player: {:?}", self.player_pos);
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                if y == self.player_pos.1 as usize && x == self.player_pos.0 as usize {
                    print!("{}", "I".bold());
                    continue;
                }
                if dir.is_some() && y as i32 == self.player_pos.1 - dir.unwrap().1 && x as i32 == self.player_pos.0 - dir.unwrap().0 {
                    print!("{}", dir_char.to_string().bold().truecolor(dir_color.0, dir_color.1, dir_color.2));
                    continue;
                }
                print!("{}", self.map[y][x].to_string().truecolor(150, 150, 150));
            }
            println!();
        }
    }

    pub fn get_box_sum(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                if self.map[y][x] == 'O' || self.map[y][x] == '[' {
                    sum += 100 * y + x;
                }
            }
        }
        sum
    }
}

fn read_state(file_path: &str, v2: bool) -> State {
    let input = fs::read_to_string(file_path).expect("File not found!");
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves = Vec::new();
    let mut player_pos = (0, 0);
    
    let mut map_read = false;
    for line in input.lines() {
        if line.is_empty() {
            map_read = true;
            continue;
        }
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if map_read {
                moves.push(c);
            } else {
                if c == '@' {
                    player_pos = (chars.len() as i32, map.len() as i32);
                }
                if v2 {
                    match c {
                        '@' => { chars.push('@'); chars.push('.'); continue; },
                        'O' => { chars.push('['); chars.push(']'); continue; },
                        _ => chars.push(c),
                    }
                }
                chars.push(c);
            }
        }
        if !map_read {
            map.push(chars);
        }
    }

    let dim = (map[0].len(), map.len());
    State { map, dim, moves, player_pos }
}

fn part1(file_path: &str) -> usize {
    let mut state = read_state(file_path, false);
    for m in &state.moves {
        let (x, y) = state.player_pos;
        let dir = match m {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0),
        };
        let next = (x + dir.0, y + dir.1);
        let (mut move_player, do_nothing) = match state.map[next.1 as usize][next.0 as usize] {
            '.' => (true, false),
            'O' => (false, false),
            '#' | _ => (false, true),
        };
        if do_nothing {
            continue;
        }
        if !move_player {
            let mut length = 2;
            loop {
                match state.map[(y + dir.1 * length) as usize][(x + dir.0 * length) as usize] {
                    '.' => {
                        state.map[(y + dir.1 * length) as usize][(x + dir.0 * length) as usize] = 'O';
                        move_player = true;
                        break;
                    },
                    '#' => break,
                    'O' | _ => (),
                }
                length += 1;
            }
        }
        if move_player {
            state.map[next.1 as usize][next.0 as usize] = '@';
            state.map[y as usize][x as usize] = '.';
            state.player_pos = (next.0, next.1);
        }
    }
    state.print_map(None, '-', (0, 0, 0));
    state.get_box_sum()
}

fn move_recur(map: &Vec<Vec<char>>, dir: (i32, i32), pos: (i32, i32), visited: &mut HashSet<(i32, i32)>) -> bool {
    visited.insert(pos);
    if dir.0 < 0 {
        visited.insert((pos.0 - 1, pos.1));
        if map[pos.1 as usize][pos.0 as usize] != ']' {
            panic!("Not a box: {:?}", pos);
        }
    } else {
        visited.insert((pos.0 + 1, pos.1));
        if map[pos.1 as usize][pos.0 as usize] != '[' {
            panic!("Not a box: {:?}", pos);
        }
    }
    let next = (pos.0 + dir.0, pos.1 + dir.1);
    let horizontal = dir.1 == 0;
    match map[next.1 as usize][next.0 as usize] {
        '#' => false,
        '.' => {
            if horizontal {
                true
            } else {
                match map[next.1 as usize][(next.0 + 1) as usize] {
                    '#' => false,
                    '.' => true,
                    _ => move_recur(map, dir, (next.0 + 1, next.1), visited),
                }
            }
        },
        '[' => { 
            move_recur(map, dir, (next.0, next.1), visited)
        },
        ']' => { 
            if horizontal {
                move_recur(map, dir, (next.0, next.1), visited)
            } else {
                if map[next.1 as usize][(next.0 + 1) as usize] == '#' {
                    false
                } else {
                    let lr_free = map[next.1 as usize][(next.0 + 1) as usize] == '.';
                    (lr_free || move_recur(map, dir, (next.0 + 1, next.1), visited)) && move_recur(map, dir, (next.0 - 1, next.1), visited)
                }

            }
        },
        _ => false,
    }
}

fn part2(file_path: &str) -> usize {
    // clearscreen::clear().unwrap();
    let mut state = read_state(file_path, true);
    // let mut moves = 0;
    // println!("-- Start -- number of moves: {}", state.moves.len());
    for m in &state.moves {
        // println!("Move: {}", m);
        // state.print_map(None, '-', (0, 0, 0));
        let (x, y) = state.player_pos;
        let mut dir = match m {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0),
        };
        let next = (x + dir.0, y + dir.1);
        let horizontal = dir.1 == 0;
        let (box_pos, do_nothing) = match state.map[next.1 as usize][next.0 as usize] {
            '.' => (None, false),
            '#' => (None, true),
            '[' => {
                if dir.0 < 0 {
                    (Some((next.0 + 1, next.1)), false)
                } else {
                    (Some(next), false)
                }
            },
            ']' => {
                if dir.0 < 0 {
                    (Some(next), false)
                } else {
                    (Some((next.0 - 1, next.1)), false)
                }
            },
            _ => (None, true),
        };
        if do_nothing {
            continue;
        }
        if box_pos.is_some() {
            if horizontal {
                dir.0 *= 2;
            }
            let mut visited = HashSet::new();
            let moved = move_recur(&state.map, dir, box_pos.unwrap(), &mut visited);
            if !moved {
                // if visited.len() > 20 {
                //     println!("{}: {}", "Didn't move".red(), m.to_string().bold());
                //     state.print_map(Some(dir), *m, (160, 0, 0));
                // }
                continue;
            }
            if horizontal {
                dir.0 /= 2;
            }
            // if visited.len() > 20 { 
            //     println!("{}: {}, box moves: {}, Visited {:?}", "Moved".green(), m.to_string().bold(), moves, visited);
            //     state.print_map(Some(dir), *m, (0, 160, 0));
            // }
            // moves += 1;
            // if moves > 280000 {
            //     break;
            // }
            let mut visited_vec: Vec<(i32, i32)> = visited.into_iter().collect();
            match m {
                '^' => visited_vec.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0))),
                'v' => visited_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0))),
                '<' => visited_vec.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1))),
                '>' => visited_vec.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1))),
                _ => (),
            };
            for (x, y) in visited_vec.iter() {
                state.map[(*y + dir.1) as usize][(*x + dir.0) as usize] = state.map[*y as usize][*x as usize];
                state.map[*y as usize][*x as usize] = '.';
            }
        }
        
        state.map[next.1 as usize][next.0 as usize] = '@';
        state.map[y as usize][x as usize] = '.';
        state.player_pos = (next.0, next.1);
    }
    // state.print_map(None, '-', (0, 0, 0));
    state.get_box_sum()
}

pub fn run(part: u8, test: bool) -> usize {
    let file_path = format!("input/{}15.txt", if test { "test/" } else { "" });
    let result = match part {
        1 => part1(&file_path),
        2 => part2(&file_path),
        _ => 0,
    };
    result
}
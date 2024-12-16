use std::fs;

struct State {
    map: Vec<Vec<char>>,
    dim: (usize, usize),
    moves: Vec<char>,
    player_pos: (i32, i32),
}

impl State {
    pub fn print_map(&self) {
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                print!("{}", self.map[y][x]);
            }
            println!();
        }
    }

    pub fn get_box_sum(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                if self.map[y][x] == 'O' {
                    sum += 100 * y + x;
                }
            }
        }
        sum
    }
}

fn read_state(file_path: &str) -> State {
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
    let mut state = read_state(file_path);
    for m in &state.moves {
        let (x, y) = state.player_pos;
        let dir = match m {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => (0, 0),
        };
        let (mut move_player, do_nothing) = match state.map[(y + dir.1) as usize][(x + dir.0) as usize] {
            '.' => (true, false),
            '#' => (false, true),
            'O' => (false, false),
            _ => (false, true),
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
                    'O' => (),
                    _ => (),
                }
                length += 1;
            }
        }
        if move_player {
            state.map[y as usize][x as usize] = '.';
            state.map[(y + dir.1) as usize][(x + dir.0) as usize] = '@';
            state.player_pos = (x + dir.0, y + dir.1);
        }
    }
    state.print_map();
    state.get_box_sum()
}

fn part2(_file_path: &str) -> usize {
    0
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
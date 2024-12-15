use std::{fs, thread, time, collections::HashMap};
use std::cmp;
use regex::Regex;

fn read_robots(file_path: &str) -> Vec<((i32, i32), (i32, i32))> {
    let input = fs::read_to_string(file_path).expect("File not found!");
    let mut robots = Vec::new();
    let re = Regex::new(r"p=(\d{1,3}),(\d{1,3}) v=(-?\d{1,3}),(-?\d{1,3})").unwrap();
    for line in input.lines() {
        for cap in re.captures_iter(&line) {
            robots.push(((cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            (cap[3].parse().unwrap(), cap[4].parse().unwrap())));
        }
    }
    robots
}

fn move_robots(robots: &mut Vec<((i32, i32), (i32, i32))>, w: i32, h: i32) {
    for robot in robots.iter_mut() {
        robot.0.0 += robot.1.0;
        robot.0.1 += robot.1.1;
        if robot.0.0 < 0 {
            robot.0.0 += w;
        }
        else if robot.0.0 >= w {
            robot.0.0 -= w;
        }
        if robot.0.1 < 0 {
            robot.0.1 += h;
        }
        else if robot.0.1 >= h {
            robot.0.1 -= h;
        }
    }
}

fn detect_pattern(state: &mut State) {
    let mut total_diff = 0;

    for y in 0..state.h {
        let mut center_x_diff = 0;
        for robot in state.robots.iter() {
            if robot.0.1 == y {
                center_x_diff += robot.0.0 - state.w / 2;
            }
        }
        total_diff += center_x_diff.abs();
    }
    state.set_pattern_score(cmp::max(5000 - total_diff, 0));
}

fn part1(file_path: &str, w: i32, h: i32) -> i64 {
    let mut robots = read_robots(file_path);
    // println!("{:?}", robots);
    let sec = 100;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for _i in 0..sec {
        move_robots(&mut robots, w, h);
    }
    for robot in robots.iter_mut() {
        if robot.0.0 == w / 2 || robot.0.1 == h / 2 {
            continue;
        }
        if robot.0.0 < w / 2 && robot.0.1 < h / 2 {
            q1 += 1;
        } else if robot.0.0 > w / 2 && robot.0.1 < h / 2 {
            q2 += 1;
        } else if robot.0.0 < w / 2 && robot.0.1 > h / 2 {
            q3 += 1;
        } else {
            q4 += 1;
        }
    }
    println!("q1: {}, q2: {}, q3: {}, q4: {}", q1, q2, q3, q4);
    q1 * q2 * q3 * q4
}

pub struct State {
    robots: Vec<((i32, i32), (i32, i32))>,
    robots_by_y: HashMap<i32, Vec<((i32, i32), (i32, i32))>>,
    sec: i64,
    w: i32,
    h: i32,
    fps: i32,
    pattern_score: i32,
    min_pattern_score: i32,
    max_pattern_score: i32,
    total_pattern_score: i64,
    draw_threshold: i32,
    draw_at: i64,
    stop_at: i64
}

impl State {
    pub fn set_pattern_score(&mut self, score: i32) {
        self.pattern_score = score;
        self.min_pattern_score = cmp::min(self.min_pattern_score, score);
        self.max_pattern_score = cmp::max(self.max_pattern_score, score);
        self.total_pattern_score += score as i64;
    }
    
    pub fn print(&self) {
        println!("pattern score: {}, min pattern score: {}, max pattern score: {}", self.pattern_score, self.min_pattern_score, self.max_pattern_score);
        if self.sec > 0 {
            println!("avg pattern score: {}", self.total_pattern_score / self.sec);
        }
    }
}

pub fn part2_setup() -> State {
    let robots = read_robots("input/14.txt");
    let mut robots_by_y = HashMap::new();
    for robot in robots.iter() {
        let entry = robots_by_y.entry(robot.0.1).or_insert(Vec::new());
        entry.push(*robot);
    }
    State {
        robots: robots,
        robots_by_y: robots_by_y,
        sec: 0,
        w: 101,
        h: 103,
        fps: 1,
        pattern_score: 0,
        min_pattern_score: 5000,
        max_pattern_score: -1,
        total_pattern_score: 0,
        draw_threshold: 2100,
        draw_at: 0,
        stop_at: 1000000
    }
}

pub fn part2_play(state: &mut State) {
    while state.sec < state.stop_at {
        move_robots(&mut state.robots, state.w, state.h);
        
        detect_pattern(state);
        if state.sec >= state.draw_at && state.pattern_score >= state.draw_threshold {
            clearscreen::clear().unwrap();
            println!("Sec: {}, FPS: {}, Stopping at: {}, pattern score: {}", state.sec, state.fps, state.stop_at, state.pattern_score);
    
            for y in 0..state.h {
                let mut row = ".".repeat(state.w as usize);
                for robot in state.robots.iter() {
                    if robot.0.1 == y {
                        row.replace_range(robot.0.0 as usize..robot.0.0 as usize + 1, "#");
                    }
                }
                println!("{}", row);
            }
    
            thread::sleep(time::Duration::from_millis(1000 / state.fps as u64));
        }

        state.sec += 1;
    }
}

pub fn run(part: u8, test: bool) -> i64 {
    let file_path = format!("input/{}14.txt", if test { "test/" } else { "" });
    let (w, h) = match test {
        true => (11, 7),
        false => (101, 103),
    };
    let result = match part {
        1 => part1(&file_path, w, h),
        _ => 0,
    };
    result
}
use std::fs;
use std::collections::HashMap;
use simple_stopwatch::Stopwatch;

fn do_vectors_overlap(a: &Vec<usize>, b: &Vec<usize>, b_range: (usize, usize)) -> bool {
    for i in a {
        for j in b_range.0..b_range.1 {
            if *i == b[j] {
                return true;
            }
        }
    }
    false
}

fn read_rules_and_updates(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules: Vec<Vec<usize>> = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    let mut reading_rules = true;

    let rule_separator = "|";
    let update_separator = ",";
    let mut separator = rule_separator;

    for line in input.lines() {
        if line.is_empty() {
            reading_rules = false;
            separator = update_separator;
            continue;
        }

        let mut numbers: Vec<usize> = Vec::new();
        for number in line.split(separator) {
            numbers.push(number.parse().unwrap());
        }

        if reading_rules {
            rules.push(numbers);
        } else {
            updates.push(numbers);
        }
    }

    (rules, updates)
}

pub fn part1() {
    println!("Day 5, part 1!");
    let input = fs::read_to_string("input/5.txt").expect("File not found!");
  
    let (rules, updates) = read_rules_and_updates(&input);
    let mut sum = 0;

    // Approach A: (primitive and slow)
    let mut sw = Stopwatch::start_new();

    for update in &updates {
        let mut valid = true;
        for rule in &rules {
            // Get index of X and Y in update
            let x_index = update.iter().position(|&x| x == rule[0]);
            // If X is not in update, skip this rule
            if x_index.is_none() {
                continue;
            }
            let y_index = update.iter().position(|&x| x == rule[1]);
            if y_index.is_none() {
                continue;
            }
            // If Y is before X, fail
            if y_index.unwrap() < x_index.unwrap() {
                valid = false;
                break;
            }
        }
        if valid {
            // Add middle number of update to sum
            sum += update[update.len() / 2];
        }
    }
    println!("Approach A: Sum: {}, time spent: {} ms", sum, sw.ms());
    
    // Approach B: (maybe faster). EDIT: It turned out to be ~7 times faster (4 ms compared to 28 ms above)
    sw.restart();

    // Index rules in a HashMap by Y, with value being list of Xs (numbers before) (single for loop)
    let mut rules_by_y: HashMap<usize, Vec<usize>> = HashMap::new();
    for rule in &rules {
        let y = rule[1];
        let x = rule[0];
        let entry = rules_by_y.entry(y).or_insert(Vec::new());
        entry.push(x);
    }

    sum = 0;
    for update in &updates {
        let mut valid = true;
        for i in 0..update.len() {
            let number = update[i];
            // Lookup [...]|Y rule (instant)
            if let Some(xs) = rules_by_y.get(&number) {
                // Array intersect [...] with proceeding numbers. If collision: Fail
                if do_vectors_overlap(&xs, &update, (i + 1, update.len())) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            sum += update[update.len() / 2];
        }
    }
    println!("Approach B: Sum: {}, time spent: {} ms", sum, sw.ms());
}

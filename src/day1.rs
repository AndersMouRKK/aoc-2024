// use std::env;
use std::fs;

fn read_input_into_vectors(sort_vectors: bool) -> (Vec<i32>, Vec<i32>) {
    // Read input from .txt file
    let input = fs::read_to_string("input/1.txt").unwrap();

    // Split input into lines
    let lines: Vec<&str> = input.lines().collect();
    
    // Define to vectors for the parsed integers
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    // Iterate over vector
    for line in lines {
        // Split line into words based on whitespace
        let words: Vec<&str> = line.split_whitespace().collect();

        // Continue if count of words is not 2
        if words.len() != 2 {
            continue;
        }

        // Parse integers from words
        let int1: i32 = words[0].parse().unwrap();
        let int2: i32 = words[1].parse().unwrap();

        // Append integers to vectors
        list1.push(int1);
        list2.push(int2);
    }

     if sort_vectors {
         list1.sort();
         list2.sort();
     }

    // Return the two lists
    (list1, list2)
}

pub fn part1() {
    println!("Day 1, part 1!");

    // Get the two lists of integers
    let (list1, list2) = read_input_into_vectors(true);

    // Define a variable to store the sum of the differences between each pair of integers
    let mut sum: i32 = 0;

    // Iterate over the two lists
    for i in 0..list1.len() {
        // Calculate the absolute difference between the two integers
        let diff: i32 = (list1[i] - list2[i]).abs();

        // Add the difference to the sum
        sum += diff;
    }

    // Print the sum
    println!("Sum of differences: {}", sum);
}

pub fn part2() {
    print!("Day 1, part 2!");

    // Get the two lists of integers, unsorted
    let (list1, list2) = read_input_into_vectors(false);

    // Define a map to store the counts of the values in the second vector
    let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    // Iterate over the second list
    for i in 0..list2.len() {
        // Get the value from the second list
        let value: i32 = list2[i];

        // Get the count of the value from the map
        let count = map.entry(value).or_insert(0);

        // Increment the count
        *count += 1;
    }

    // Define a variable to store the sum of the frequency of the values in the second list in the first list
    let mut sum: i32 = 0;

    // Iterate over the first list
    for i in 0..list1.len() {
        // Get the value from the first list
        let value: i32 = list1[i];

        // Get the count of the value from the map, skip if not found
        let count = match map.get(&value) {
            Some(count) => count,
            None => continue,
        };

        // Add the count times the value to the sum
        sum += count * value;
    }

    // Print the sum
    println!("Sum of frequencies: {}", sum);
}
use std::fs;

fn read_input_into_vector_of_vectors() -> Vec<Vec<i32>> {
    // Read the input file into a string
    let input = fs::read_to_string("input/2.txt").expect("Failed to read input file");

    // Split the input string by newlines
    let lines: Vec<&str> = input.trim().split('\n').collect();

    // Define a vector to store the vectors of integers
    let mut vectors: Vec<Vec<i32>> = Vec::new();

    // Iterate over the lines
    for line in lines {
        // Split the line by whitespace
        let numbers: Vec<&str> = line.split_whitespace().collect();

        // Define a vector to store the integers
        let mut vector: Vec<i32> = Vec::new();

        // Iterate over the numbers
        for number in numbers {
            // Parse the number into an integer
            let n: i32 = number.parse().expect("Failed to parse number");

            // Push the integer onto the vector
            vector.push(n);
        }

        // Push the vector onto the vectors
        vectors.push(vector);
    }

    // Return the vectors
    vectors
}

fn get_vector_without_element(vector: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut new_vector: Vec<i32> = Vec::new();
    for i in 0..vector.len() {
        if i != index {
            new_vector.push(vector[i]);
        }
    }
    new_vector
}

fn is_safe(vector: &Vec<i32>, tolerance: u32) -> bool {
    if vector.len() <= 1 {
        return false;
    }

    let inc: bool = vector[0] < vector[1];
    for i in 0..vector.len() - 1 {
        let diff: i32 = vector[i + 1] - vector[i];
        if (inc && (diff < 1 || diff > 3)) || (!inc && (diff < -3 || diff > -1)) {
            if tolerance == 0 {
                return false;
            }

            if i > 0 && is_safe(&get_vector_without_element(vector, i - 1), tolerance - 1) {
                return true;
            }
            if is_safe(&get_vector_without_element(vector, i), tolerance - 1) {
                return true;
            }

            return is_safe(&get_vector_without_element(vector, i + 1), tolerance - 1);
        }
    }

    true
}

pub fn part1() {
    println!("Day 2, part 1!");

    // Read the input into a vector of vectors
    let vectors = read_input_into_vector_of_vectors();

    let mut count: i32 = 0;
    for vector in vectors {
        if is_safe(&vector, 0) {
            count += 1;
        }
    }

    println!("Number of safe vectors: {}", count);
}

pub fn part2() {
    println!("Day 2, part 2!");

    // Read the input into a vector of vectors
    let vectors = read_input_into_vector_of_vectors();

    let mut count: i32 = 0;
    for vector in vectors {
        if is_safe(&vector, 1) {
            count += 1;
        }
    }

    println!("Number of safe vectors: {}", count);
}

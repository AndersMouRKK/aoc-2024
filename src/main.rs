use std::io;
mod day14;
use simple_stopwatch::Stopwatch;

fn main() {
    // let sw = Stopwatch::start_new();
    // println!("Result: {}, spent {} ms", day14::run(1, false), sw.ms());

    let mut state = day14::part2_setup();
    let mut input = String::new();
    loop {
        println!("Enter command:");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "exit" => break,
            "play" => day14::part2_play(&mut state),
            _ => println!("Unknown command"),
        }
    }
    state.print();
}

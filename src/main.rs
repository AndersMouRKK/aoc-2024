mod day15;
use simple_stopwatch::Stopwatch;

fn main() {
    let sw = Stopwatch::start_new();
    println!("Result: {}, spent {} ms", day15::run(2, false), sw.ms());
}

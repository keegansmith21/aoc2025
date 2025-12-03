mod day_01;
mod day_02;
mod day_03;
mod utils;

use std::env;

fn usage() {
    eprintln!("Usage: cargo run -- <N> [--example]");
    eprintln!("Example: cargo run -- 1 --example");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        return;
    }

    let example = args.contains(&"--example".to_string());
    let day = args[1..]
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(|s| s.as_str());

    let Some(day) = day else {
        eprintln!("No day given");
        usage();
        return;
    };

    match day {
        "1" => day_01::main(example),
        "2" => day_02::main(example),
        "3" => day_03::main(example),
        _ => eprintln!("Unknown day: {}", args[1]),
    }
}

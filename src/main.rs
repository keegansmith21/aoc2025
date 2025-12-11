mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
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
        "4" => day_04::main(example),
        "5" => day_05::main(example),
        "6" => day_06::main(example),
        "7" => day_07::main(example),
        "8" => day_08::main(example),
        "9" => day_09::main(example),
        _ => eprintln!("Unknown day: {}", args[1]),
    }
}

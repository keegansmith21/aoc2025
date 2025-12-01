use crate::utils;

pub fn main(example: bool) {
    let input = utils::read_input(1, example);

    println!("Day 1:");

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve_part1(input: &str) -> i32 {
    let mut counter: i32 = 0;
    let mut dial: i32 = 50;
    for line in input.lines() {
        dial += get_instruction(line);
        dial = dial.rem_euclid(100); // Non-negative modulus
        if dial == 0 {
            counter += 1;
        }
    }
    counter
}

fn solve_part2(input: &str) -> i32 {
    let mut counter: i32 = 0;
    let mut dial: i32 = 50;
    for line in input.lines() {
        counter += count_passes(&dial, &get_instruction(line));
        dial += get_instruction(line);
        dial = dial.rem_euclid(100); // Non-negative modulus
        if dial == 0 {
            counter += 1
        }
    }
    counter
}

fn get_instruction(line: &str) -> i32 {
    let chars: String = line.trim().chars().skip(1).collect();
    let mut n: i32 = chars.parse().unwrap();
    if line.starts_with("L") {
        n = -n;
    }
    n
}

fn count_passes(start: &i32, mv: &i32) -> i32 {
    // There's probably a smart way of doing this but I can't figure it out.
    // Since it matters where you start and which direction you're moving,
    // it seems like you just need a bunch of if statements like this...
    let mut n_passes: i32 = 0;
    if *start == 0 {
        n_passes = (*mv / 100).abs();
    } else if *mv < 0 {
        if mv.abs() > *start {
            n_passes += 1;
        }
        n_passes += ((*mv + *start + 1) / 100).abs();
    } else if *mv > 0 {
        if mv.rem_euclid(100) > (100 - *start) {
            n_passes += 1;
        }
        n_passes += *mv / 100;
    }
    n_passes
}

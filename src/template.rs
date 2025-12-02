use crate::utils;

pub fn main(example: bool) {
    let day: u8 = n;
    let input = utils::read_input(day, example);

    println!("Day {}:", day);

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Day {} - Part 1: {}", day, p1);
    println!("Day {} - Part 2: {}", day, p2);
}

fn solve_part1(_input: &str) -> i64 {
    0
}

fn solve_part2(_input: &str) -> i64 {
    0
}

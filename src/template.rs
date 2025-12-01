use crate::utils;

pub fn main(example: bool) {
    let input = utils::read_input(1, example);

    println!("Day N:");

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve_part1(_input: &str) -> i64 {
    0
}

fn solve_part2(_input: &str) -> i64 {
    0
}

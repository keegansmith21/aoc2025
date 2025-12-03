use crate::utils;

pub fn main(example: bool) {
    let day: u8 = 3;
    let input = utils::read_input(day, example);

    println!("Day {}:", day);

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Day {} - Part 1: {}", day, p1);
    println!("Day {} - Part 2: {}", day, p2);
}

fn solve_part1(input: &str) -> i32 {
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars())
        .map(|n| {
            n.map(|x| {
                x.to_digit(10)
                    .map(|d| d as i32)
                    .unwrap_or_else(|| panic!("Could not parse to int: {}", x))
            })
            .collect()
        })
        .collect();
    lines.iter().map(|l| line_joltage(&l)).sum()
}

fn solve_part2(_input: &str) -> i64 {
    0
}

fn line_joltage(line: &Vec<i32>) -> i32 {
    let n1 = first_largest(line[..line.len()-1]);
    let n2 = first_largest(line[n1.1..]

    0
}

fn first_largest(array: &Vec<i32>) -> (i32, i32) {
    let max: i32 = *array.iter().max().unwrap_or_else(|| panic!(""));
    let idx: i32 = array
        .into_iter()
        .position(|&n| n == max)
        .unwrap_or_else(|| panic!("")) as i32;
    (max, idx)
}

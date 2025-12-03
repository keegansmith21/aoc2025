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

fn solve_part1(input: &str) -> i64 {
    get_lines(input).iter().map(|l| line_joltage(&l, 2)).sum()
}

fn solve_part2(input: &str) -> i64 {
    get_lines(input).iter().map(|l| line_joltage(&l, 12)).sum()
}

fn get_lines(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.chars())
        .map(|n| {
            n.map(|x| {
                x.to_digit(10)
                    .map(|d| d as i64)
                    .unwrap_or_else(|| panic!("Could not parse to int: {}", x))
            })
            .collect()
        })
        .collect()
}

fn line_joltage(line: &Vec<i64>, n: usize) -> i64 {
    // n is the number of numbers to use in the joltage calc
    let len = line.len();
    let mut js: Vec<i64> = Vec::new();
    let mut start = 0;

    // Get the first largest element. Use a sliding window to determine the values to look at.
    // The end of the window is easy - The number of values remaining - 1.
    // The start is the location of the previous largest element + 1.
    for i in 1..n + 1 {
        let end: usize = (len - n + i) as usize;
        let max: i64 = *line[start..end].iter().max().unwrap_or_else(|| panic!(""));

        start += line[start..end]
            .iter()
            .position(|&x| x == max)
            .unwrap_or_else(|| panic!(""))
            + 1;
        js.push(max)
    }
    js.iter()
        .enumerate()
        .map(|(i, j)| j * 10_i64.pow((n - i - 1) as u32))
        .sum()
}

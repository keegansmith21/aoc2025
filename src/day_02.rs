use crate::utils;
use std::ops::RangeInclusive;

pub fn main(example: bool) {
    let day: u8 = 2;
    let input = utils::read_input(day, example);

    println!("Day {}:", day);

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Day {} - Part 1: {}", day, p1);
    println!("Day {} - Part 2: {}", day, p2);
}

fn solve_part1(input: &str) -> i64 {
    let ranges: Vec<(i64, i64)> = get_ranges(input);
    let invalid: i64 = ranges
        .into_iter()
        .map(|r| invalid_values_in_range(&r))
        .sum();
    invalid
}

fn solve_part2(input: &str) -> i64 {
    let ranges: Vec<(i64, i64)> = get_ranges(input);
    let invalid: i64 = ranges
        .into_iter()
        .map(|r| invalid_values_in_range_pt2(&r))
        .sum();
    invalid
}

fn get_ranges(input: &str) -> Vec<(i64, i64)> {
    let ranges: Vec<(i64, i64)> = input
        .split(",")
        .map(|range| {
            let mut nums = range.split("-").map(|x| {
                x.trim()
                    .parse::<i64>()
                    .unwrap_or_else(|_| panic!("Could not parse to i64: {} of len {}", x, x.len()))
            });
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();
    ranges
}

fn invalid_values_in_range(range: &(i64, i64)) -> i64 {
    let mut invalid = 0;
    let mut n_str;
    let mut n_len;
    let mut n_chars: Vec<char>;
    for n in RangeInclusive::new(range.0, range.1) {
        n_str = n.to_string();
        n_len = n_str.len();

        // If not an even number of digits, skip it
        if n_len % 2 == 1 {
            continue;
        }

        n_chars = n_str.chars().collect();
        if n_chars[..(n_len / 2)] == n_chars[(n_len / 2)..] {
            invalid += n;
        }
    }
    return invalid;
}

fn invalid_values_in_range_pt2(range: &(i64, i64)) -> i64 {
    let mut invalid = 0;
    let mut n_len;
    let mut n_chars: Vec<char>;
    let mut chunks;
    for n in RangeInclusive::new(range.0, range.1) {
        n_len = n.to_string().len();
        n_chars = n.to_string().chars().collect();
        for i in 1..(n_len / 2 + 1) {
            if n_len % i != 0 {
                continue;
            }

            chunks = n_chars.chunks(i);
            let first = chunks
                .next()
                .unwrap_or_else(|| panic!("No values in chunk"));

            if chunks.all(|c| c == first) {
                invalid += n;
                break;
            } else {
            }
        }
    }
    return invalid;
}

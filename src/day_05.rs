use crate::utils;
use std::cmp;

pub fn main(example: bool) {
    let day: u8 = 5;
    let input = utils::read_input(day, example);

    println!("Day {}:", day);

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Day {} - Part 1: {}", day, p1);
    println!("Day {} - Part 2: {}", day, p2);
}

fn solve_part1(input: &str) -> i32 {
    let (ranges, ingredients) = parse_input(input);
    let mut fresh: i32 = 0;
    for n in ingredients {
        if ranges.iter().any(|r| is_n_in_range(&n, &r)) {
            fresh += 1;
        }
    }

    fresh
}

fn solve_part2(input: &str) -> i64 {
    let mut ranges = parse_input(input).0;
    loop {
        if converge_ranges(&mut ranges) == false {
            break;
        }
    }
    ranges.iter().map(|r| r.1 - r.0 + 1).sum() // add one because ranges are inclusive
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ingredients: Vec<i64> = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        if line.contains("-") {
            let mut parts = line
                .split("-")
                .map(|r| r.parse::<i64>().expect(&format!("Cannot parse: {}", r)));
            ranges.push((
                parts.next().expect("Could not iterate"),
                parts.next().expect("Could not iterate"),
            ));
        } else {
            ingredients.push(
                line.parse::<i64>()
                    .expect(&format!("Cannot parse: {}", line)),
            );
        }
    }
    (ranges, ingredients)
}

fn is_n_in_range(n: &i64, r: &(i64, i64)) -> bool {
    if (r.0 - n <= 0) && (r.1 - n >= 0) {
        return true;
    }
    false
}

fn converge_ranges(ranges: &mut Vec<(i64, i64)>) -> bool {
    let altered: bool = false;
    let mut combinations: Vec<(usize, usize)> = Vec::new();
    for i in 0..ranges.len() {
        for j in (i + 1)..ranges.len() {
            combinations.push((i, j));
        }
    }

    for c in combinations {
        let r1 = ranges[c.0];
        let r2 = ranges[c.1];
        if do_ranges_overlap(&r1, &r2) {
            ranges.remove(cmp::max(c.0, c.1));
            ranges.remove(cmp::min(c.0, c.1));
            ranges.push((cmp::min(r1.0, r2.0), cmp::max(r1.1, r2.1)));
            return true;
        }
    }

    altered
}

fn do_ranges_overlap(r1: &(i64, i64), r2: &(i64, i64)) -> bool {
    if r1.0 <= r2.1 && r2.0 <= r1.1 {
        return true;
    }
    false
}

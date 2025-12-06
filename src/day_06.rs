use crate::utils;

pub fn main(example: bool) {
    let day: u8 = 6;
    let input = utils::read_input(day, example);

    println!("Day {}:", day);

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Day {} - Part 1: {}", day, p1);
    println!("Day {} - Part 2: {}", day, p2);
}

fn solve_part1(input: &str) -> i64 {
    let (numbers, operations) = parse_input(input);
    let t_numbers = utils::transpose(&numbers);
    operations
        .iter()
        .enumerate()
        .map(|(i, op)| do_operation(&t_numbers[i], op))
        .sum::<i64>()
}

fn solve_part2(input: &str) -> i64 {
    // Transpose the numbers so that they're read properly
    let (numbers, operations) = parse_input_2(input);
    let t_numbers = utils::transpose(&numbers);
    let mut ceph_numbers: Vec<Vec<i64>> = Vec::new();
    let mut cns: Vec<i64> = Vec::new();
    for row in &t_numbers {
        println!("{:?}", row);
        if row.iter().all(|&a| a == ' ') {
            println!("{:?}", row);
            ceph_numbers.push(cns);
            cns = Vec::new();
            continue;
        }
        cns.push(
            row.iter()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Could not parse to i64: {:?}", row)),
        )
    }
    ceph_numbers.push(cns); // Push the last one 

    operations
        .iter()
        .enumerate()
        .map(|(i, op)| do_operation(&ceph_numbers[i], op))
        .sum::<i64>()
}

fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let lines: Vec<&str> = input.lines().collect();
    let operations: Vec<char>;
    operations = lines[lines.len() - 1]
        .split_whitespace()
        .map(|op| {
            op.chars()
                .next()
                .unwrap_or_else(|| panic!("Could not parse '{}'", op))
        })
        .collect();

    let numbers: Vec<Vec<i64>>;
    numbers = lines[..lines.len() - 1]
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|n| {
                    n.parse::<i64>()
                        .unwrap_or_else(|_| panic!("Could not parse '{}'", n))
                })
                .collect()
        })
        .collect();

    (numbers, operations)
}

fn parse_input_2(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let lines: Vec<&str> = input.lines().collect();
    let operations: Vec<char>;
    operations = lines[lines.len() - 1]
        .split_whitespace()
        .map(|op| {
            op.chars()
                .next()
                .unwrap_or_else(|| panic!("Could not parse '{}'", op))
        })
        .collect();

    let numbers: Vec<Vec<char>>;
    numbers = lines[..lines.len() - 1]
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    (numbers, operations)
}

fn do_operation(ns: &Vec<i64>, op: &char) -> i64 {
    match *op {
        '+' => ns.iter().sum(),
        '*' => ns.iter().product(),
        _ => panic!("Could not parse operator: '{}'", op),
    }
}

use crate::utils;
use std::collections::HashMap;

pub fn main(example: bool) {
    let day: u8 = 7;
    let input = utils::read_input(day, example);

    println!("Day {}:", day);

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Day {} - Part 1: {}", day, p1);
    println!("Day {} - Part 2: {}", day, p2);
}

fn solve_part1(input: &str) -> i64 {
    let mut map: Vec<Vec<char>>;
    let _start: usize;
    (map, _start) = parse_input(input);
    let mut splits: i64 = 0;
    for i in 1..map.len() {
        // Location of beams above us
        let beams: &Vec<usize> = &map[i - 1]
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if *v == 'S' || *v == '|' {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        splits += cascade(beams, &mut map[i]);
    }

    splits
}

fn solve_part2(input: &str) -> i64 {
    // Recursive algorithm to find all possibilities
    let map: Vec<Vec<char>>;
    let start: usize;
    (map, start) = parse_input(input);
    let timelines: i64 = get_timelines(&map, start);

    timelines
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, usize) {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start: usize = map[0]
        .iter()
        .position(|c| *c == 'S')
        .unwrap_or_else(|| panic!("Could not find S: {:?}", map[0]));

    return (map, start);
}

fn cascade(beams: &Vec<usize>, new_row: &mut Vec<char>) -> i64 {
    // Beams are the indexes of the beams coming down from the previous row
    let mut splits: i64 = 0;
    for b in beams {
        // There is a beam above a splitter
        if new_row[*b] == '^' {
            splits += 1;
            if new_row[b - 1] != '^' {
                new_row[b - 1] = '|';
            }
            if new_row[b + 1] != '^' {
                new_row[b + 1] = '|';
            }
        } else {
            new_row[*b] = '|';
        }
    }
    splits
}

fn get_timelines(map: &Vec<Vec<char>>, start: usize) -> i64 {
    // Memoized recursive algorithm
    let mut hashmap: HashMap<(usize, usize), i64> = HashMap::new();
    fn timelines(
        map: &Vec<Vec<char>>,
        row: usize,
        beam: usize, // beam is the location of the beam from above
        hashmap: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        // memoized case
        if let Some(v) = hashmap.get(&(row, beam)) {
            return *v;
        }
        // base case - hit the bottom
        if row == map.len() - 1 {
            return 1;
        }
        // split
        if map[row][beam] == '^' {
            let left: i64 = timelines(map, row + 1, beam - 1, hashmap);
            let right: i64 = timelines(map, row + 1, beam + 1, hashmap);
            hashmap.insert((row, beam), left + right);
            return left + right;
        }
        // empty space
        timelines(map, row + 1, beam, hashmap)
    }
    timelines(map, 1, start, &mut hashmap)
}

use crate::utils;
use std::collections::HashMap;

pub fn main(example: bool) {
    let day: u8 = 8;
    let input = utils::read_input(day, example);

    println!("Day {}:", day);

    let p1 = solve_part1(&input, example);
    let p2 = solve_part2(&input);

    println!("Day {} - Part 1: {}", day, p1);
    println!("Day {} - Part 2: {}", day, p2);
}

fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .map(|l| -> (i64, i64, i64) {
            let mut s = l
                .split(",")
                .map(|n| n.parse::<i64>().expect("Could not parse to i64"));
            (
                s.next().expect(""),
                s.next().expect(""),
                s.next().expect(""),
            )
        })
        .collect()
}
fn solve_part1(input: &str, example: bool) -> i64 {
    let coords: Vec<(i64, i64, i64)> = parse_input(input);
    let combos: Vec<((usize, usize), f64)> = make_combinations(&coords);
    let n = if example { 10 } else { 1000 };
    let circuits = create_circuits(&coords, &combos, n).1;
    let mut biggest = circuits
        .values()
        .map(|g| g.len() as i64)
        .collect::<Vec<i64>>();

    biggest.sort_by(|a, b| b.cmp(&a));
    println!("{:?}", biggest);
    biggest[..3].iter().product()
}

fn solve_part2(input: &str) -> i64 {
    let coords: Vec<(i64, i64, i64)> = parse_input(input);
    let combos: Vec<((usize, usize), f64)> = make_combinations(&coords);
    let ns = create_circuits(&coords, &combos, combos.len()).0;
    let x1 = coords[ns.0].0 as i64;
    let x2 = coords[ns.1].0 as i64;
    x1 * x2
}

fn create_circuits(
    coords: &Vec<(i64, i64, i64)>,
    combos: &Vec<((usize, usize), f64)>,
    n: usize,
) -> ((usize, usize), HashMap<u32, Vec<usize>>) {
    // n is the number of combinations to join
    let mut box_groups: HashMap<usize, u32> = HashMap::new(); // Boxes as keys, group number as val
    let mut groups: HashMap<u32, Vec<usize>> = HashMap::new(); // Group number as keys, boxes as val
    // Seed the groups - initially they're all isolated
    for i in 0..coords.len() {
        box_groups.insert(i, i as u32); // Unique group Identifier for each box
        let mut new_vec = Vec::new();
        new_vec.push(i);
        groups.insert(i as u32, new_vec); // Groups keeps track of what boxes are in each group
    }
    let mut b1 = 0;
    let mut b2 = 0;
    for i in 0..n {
        // We need to join the groups that b1 and b2 belong to
        (b1, b2) = combos[i].0;
        let g1 = box_groups[&b1];
        let g2 = box_groups[&b2];
        if g1 == g2 {
            continue;
        };

        // Update the group for all boxes in "group2"
        let merger = groups
            .remove(&g2)
            .expect("Cannot find group but should have..");
        for i in &merger {
            box_groups.insert(*i, g1);
        }
        groups.entry(g1).and_modify(|group| group.extend(merger));
        if groups.len() == 1 {
            return ((b1, b2), groups);
        }
    }
    ((b1, b2), groups)
}

fn make_combinations(coords: &Vec<(i64, i64, i64)>) -> Vec<((usize, usize), f64)> {
    // Given the coordinates of all boxes, finds the distances between all possible combinations of
    // them and then sorts them all by the shorts distances
    let mut coord_combos: Vec<(usize, usize)> = Vec::new();
    let mut distances: Vec<f64> = Vec::new();
    for i in 0..coords.len() - 1 {
        for j in i + 1..coords.len() {
            coord_combos.push((i, j));
            distances.push(distance(coords[i], coords[j]));
        }
    }
    let mut zipped: Vec<_> = coord_combos
        .iter()
        .zip(distances.iter())
        .map(|(a, b)| (*a, *b))
        .collect();
    zipped.sort_by(|z1, z2| z1.1.total_cmp(&z2.1));
    zipped
}

fn distance(c1: (i64, i64, i64), c2: (i64, i64, i64)) -> f64 {
    let sumsqr: f64 =
        (c1.0 - c2.0).pow(2) as f64 + (c1.1 - c2.1).pow(2) as f64 + (c1.2 - c2.2).pow(2) as f64;
    sumsqr.sqrt()
}

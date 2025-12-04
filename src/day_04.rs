use crate::utils;

pub fn main(example: bool) {
    let day: u8 = 4;
    let input = utils::read_input(day, example);

    println!("Day {}:", day);

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Day {} - Part 1: {}", day, p1);
    println!("Day {} - Part 2: {}", day, p2);
}

fn parse_wall(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| match x {
                    '.' => 0,
                    '@' => 1,
                    _ => panic!(""),
                })
                .collect()
        })
        .collect()
}
fn solve_part1(input: &str) -> i32 {
    let wall: Vec<Vec<i32>> = parse_wall(input);
    let mut good_rolls: i32 = 0;
    for i in 0..wall.len() {
        for j in 0..wall[i].len() {
            if wall[i][j] == 0 {
                continue;
            }
            if count_surroundings(&wall, (i, j)) < 4 {
                good_rolls += 1;
            }
        }
    }

    good_rolls
}

fn solve_part2(input: &str) -> i32 {
    let mut wall: Vec<Vec<i32>> = parse_wall(input);
    let mut good_rolls: i32 = 0;
    let mut found_roll: bool;
    loop {
        found_roll = false;
        for i in 0..wall.len() {
            for j in 0..wall[i].len() {
                if wall[i][j] == 0 {
                    continue;
                }
                if count_surroundings(&wall, (i, j)) < 4 {
                    found_roll = true;
                    good_rolls += 1;
                    wall[i][j] = 0;
                }
            }
        }
        if found_roll == false {
            break;
        }
    }
    good_rolls
}

fn count_surroundings(wall: &Vec<Vec<i32>>, loc: (usize, usize)) -> i32 {
    let mut n_rolls: i32 = 0;
    let is_left: bool = loc.1 as i32 - 1 >= 0;
    let is_right: bool = loc.1 + 1 < wall[0].len();
    let is_up: bool = loc.0 as i32 - 1 >= 0;
    let is_down: bool = loc.0 + 1 < wall.len();

    if is_left {
        // # . .
        // # @ .
        // # . .

        n_rolls += wall[loc.0][loc.1 - 1];
        if is_up {
            n_rolls += wall[loc.0 - 1][loc.1 - 1];
        }
        if is_down {
            n_rolls += wall[loc.0 + 1][loc.1 - 1];
        }
    }
    if is_right {
        // . . #
        // . @ #
        // . . #

        n_rolls += wall[loc.0][loc.1 + 1];
        if is_up {
            n_rolls += wall[loc.0 - 1][loc.1 + 1];
        }
        if is_down {
            n_rolls += wall[loc.0 + 1][loc.1 + 1];
        }
    }
    if is_up {
        n_rolls += wall[loc.0 - 1][loc.1];
    }
    if is_down {
        n_rolls += wall[loc.0 + 1][loc.1];
    }

    n_rolls
}

use crate::utils;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

fn write_chairmap_to_file(data: &Vec<Vec<char>>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for row in data {
        let row_string: String = row.iter().collect();
        file.write_all(row_string.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

pub fn main(example: bool) {
    let day: u8 = 9;
    let input = utils::read_input(day, example);

    println!("Day {}:", day);

    let p1 = solve_part1(&input);
    let p2 = solve_part2(&input);

    println!("Day {} - Part 1: {}", day, p1);
    println!("Day {} - Part 2: {}", day, p2);
}

fn solve_part1(input: &str) -> i64 {
    let tiles = parse_input(input);
    let mut area: i64 = 0;
    for i in 0..tiles.len() - 1 {
        for j in i..tiles.len() {
            let a = tiled_area(&tiles[i], &tiles[j]);
            if a > area {
                area = a;
            }
        }
    }
    area
}

fn solve_part2(input: &str) -> i64 {
    let tiles = parse_input(input);
    // Compress the coordinates to their smallest meaningful number. Keep a reverse mapping to undo
    // this later
    let (ymap, xmap, rev_ymap, rev_xmap) = compression_map(&tiles);
    let compressed_tiles = tiles
        .iter()
        .map(|t| (ymap[&t.0], xmap[&t.1]))
        .collect::<Vec<(i64, i64)>>();

    // Create the chain of '#' between each chair
    let mut chairmap = create_outline(&compressed_tiles);
    match write_chairmap_to_file(&chairmap, "chairmap_output.txt") {
        Ok(()) => println!("Wrote map to file"),
        Err(e) => eprintln!("{}", e),
    }

    // Flood fill the map
    fill_chairmap(&mut chairmap);

    // Find the (uncompressed) areas of all valid squares
    let mut areas: Vec<i64> = Vec::new();
    for i in 0..compressed_tiles.len() - 1 {
        for t2 in &compressed_tiles[i..] {
            let t1 = compressed_tiles[i];
            if is_square_valid(&chairmap, &compressed_tiles[i], &t2) {
                let t1_exp = (rev_ymap[t1.0 as usize], rev_xmap[t1.1 as usize]); // decompress
                let t2_exp = (rev_ymap[t2.0 as usize], rev_xmap[t2.1 as usize]);
                areas.push(tiled_area(&t1_exp, &t2_exp));
            }
        }
    }
    *areas.iter().max().expect("")
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|l| {
            let mut s = l.split(",");
            (
                s.next().expect("").parse::<i64>().expect(""),
                s.next().expect("").parse::<i64>().expect(""),
            )
        })
        .collect()
}

fn tiled_area(t1: &(i64, i64), t2: &(i64, i64)) -> i64 {
    ((t1.0 - t2.0).abs() + 1) * ((t1.1 - t2.1).abs() + 1)
}

fn compression_map(
    tiles: &Vec<(i64, i64)>,
) -> (HashMap<i64, i64>, HashMap<i64, i64>, Vec<i64>, Vec<i64>) {
    let mut xmap: HashMap<i64, i64> = HashMap::new();
    let mut ymap: HashMap<i64, i64> = HashMap::new();

    let mut ysorted = tiles.iter().map(|x| x.0).collect::<Vec<i64>>();
    ysorted.sort();
    ysorted.dedup();
    let mut xsorted = tiles.iter().map(|x| x.1).collect::<Vec<i64>>();
    xsorted.sort();
    xsorted.dedup();

    for i in 0..xsorted.len() {
        xmap.insert(xsorted[i], i as i64);
    }
    for i in 0..ysorted.len() {
        ymap.insert(ysorted[i], i as i64);
    }

    (ymap, xmap, ysorted, xsorted)
}

fn create_outline(chairs: &Vec<(i64, i64)>) -> Vec<Vec<char>> {
    // Initial chairmap with only the tiled chairs
    let ymax = chairs.iter().map(|x| x.0).max().expect("") as usize;
    let xmax = chairs.iter().map(|x| x.1).max().expect("") as usize;
    let mut chairmap: Vec<Vec<char>> = vec![vec!['.'; xmax + 1]; ymax + 1];
    for c in chairs {
        chairmap[c.0 as usize][c.1 as usize] = '#'
    }

    // Now draw the lines
    for i in 0..chairs.len() - 1 {
        draw_line(&mut chairmap, &chairs[i], &chairs[i + 1]);
    }
    draw_line(&mut chairmap, &chairs[chairs.len() - 1], &chairs[0]); // Complete the loop

    chairmap
}

fn draw_line(chairmap: &mut Vec<Vec<char>>, c1: &(i64, i64), c2: &(i64, i64)) {
    // Fill the space between the two coordinates c1 and c2
    let start: usize;
    let end: usize;
    if c1.0 == c2.0 {
        // Same Row
        if c1.1 < c2.1 {
            start = c1.1 as usize;
            end = c2.1 as usize;
        } else {
            start = c2.1 as usize;
            end = c1.1 as usize;
        }

        for i in start..end {
            chairmap[c1.0 as usize][i] = '#';
        }
    } else if c1.1 == c2.1 {
        // Same Column
        if c1.0 < c2.0 {
            start = c1.0 as usize;
            end = c2.0 as usize;
        } else {
            start = c2.0 as usize;
            end = c1.0 as usize;
        }
        for i in start..end {
            chairmap[i][c1.1 as usize] = '#';
        }
    } else {
        panic!("Incompatible tiles: {:?}, {:?}", c1, c2);
    }
}

fn fill_chairmap(chairmap: &mut Vec<Vec<char>>) {
    // Fill in the empty space - Find a spot inside the shape first with a parity check
    let mut start: Option<(usize, usize)> = None;
    'search: for y in 0..chairmap.len() {
        for x in 0..chairmap[0].len() {
            if is_inside(&chairmap, x, y) {
                start = Some((y, x));
                break 'search;
            }
        }
    }
    if let Some(s) = start {
        flood_fill(chairmap, &s);
    } else {
        panic!("Could not find starting location")
    }
}

fn is_inside(a: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // This method isn't striclty correct but it'll do for this problem
    if a[y][x] == '#' || y == 0 || x == 0 || y == a.len() - 1 || x == a[0].len() - 1 {
        return false;
    }
    let up_has_boundary = a[..y].iter().any(|r| r[x] == '#');
    let down_has_boundary = a[y..].iter().any(|r| r[x] == '#');
    let left_has_boundary = a[y][..x].contains(&'#');
    let right_has_boundary = a[y][x..].contains(&'#');
    up_has_boundary && down_has_boundary && left_has_boundary && right_has_boundary
}

fn flood_fill(chairmap: &mut Vec<Vec<char>>, loc: &(usize, usize)) {
    if chairmap[loc.0][loc.1] != '#' {
        chairmap[loc.0][loc.1] = '#';
        if loc.0 > 0 {
            let up: (usize, usize) = (loc.0 - 1, loc.1);
            flood_fill(chairmap, &up);
        }
        if loc.0 < chairmap.len() - 1 {
            let down: (usize, usize) = (loc.0 + 1, loc.1);
            flood_fill(chairmap, &down);
        }
        if loc.1 > 0 {
            let left: (usize, usize) = (loc.0, loc.1 - 1);
            flood_fill(chairmap, &left);
        }
        if loc.1 < chairmap[0].len() - 1 {
            let right: (usize, usize) = (loc.0, loc.1 + 1);
            flood_fill(chairmap, &right);
        }
    }
}

fn is_square_valid(chairmap: &Vec<Vec<char>>, t1: &(i64, i64), t2: &(i64, i64)) -> bool {
    let mut squares: Vec<char> = Vec::new();
    let xrange = (t1.1.min(t2.1), t1.1.max(t2.1) + 1);
    let yrange = (t1.0.min(t2.0), t1.0.max(t2.0) + 1);
    for y in yrange.0 as usize..yrange.1 as usize {
        squares.extend(&chairmap[y][xrange.0 as usize..xrange.1 as usize])
    }
    !squares.contains(&'.')
}

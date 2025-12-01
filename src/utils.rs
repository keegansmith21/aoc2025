use std::fs;

pub fn read_input(day: u8, example: bool) -> String {
    if example {
        println!("Reading example input..")
    }
    let filename = if example {
        format!("src/example_{:02}.txt", day)
    } else {
        format!("src/input_{:02}.txt", day)
    };
    fs::read_to_string(&filename).unwrap_or_else(|_| panic!("Failed to read input: {}", filename))
}

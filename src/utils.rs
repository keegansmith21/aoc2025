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

pub fn transpose<T: Clone>(mat: &[Vec<T>]) -> Vec<Vec<T>> {
    let cols = mat[0].len();
    let mut t_mat = vec![Vec::with_capacity(mat.len()); cols];
    for row in mat {
        for (j, val) in row.into_iter().enumerate() {
            t_mat[j].push(val.clone());
        }
    }
    t_mat
}

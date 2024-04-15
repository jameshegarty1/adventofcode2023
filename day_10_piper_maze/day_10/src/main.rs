use std::fs;
use std::cmp::Ordering;

fn find_char_linear(target: char, data: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in data.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == target {
                return Some((i,j))
            }
        } 
    }
    None
}

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut data_vec: Vec<Vec<char>> = Vec::new();


    contents.lines().for_each(|line| {
        let row: Vec<_> = line.chars().collect();
        data_vec.push(row);
    });

    data_vec
}

fn main() {
    let data: Vec<Vec<char>> = parse_file("test_input");

    let target: char = 'S';
    let start_location: (usize,usize) = match find_char_linear(target, &data) {
        Some((i, j)) => (i, j),
        None => (0,0)
    };

    let pointer_one
}

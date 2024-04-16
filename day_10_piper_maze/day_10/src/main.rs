use std::fs;
use std::cmp::Ordering;

enum Tile {
    VerticalPipe,    // '|'
    HorizontalPipe,  // '-'
    BendNE,          // 'L'
    BendNW,          // 'J'
    BendSW,          // '7'
    BendSE,          // 'F'
    Ground,          // '.'
    StartPosition,   // 'S'
}

impl Tile {
   fn from_char(c: char) -> Option<Self> {
        match c {
            '|' => Some(Tile::VerticalPipe),
            '-' => Some(Tile::HorizontalPipe),
            'L' => Some(Tile::BendNE),
            'J' => Some(Tile::BendNW),
            '7' => Some(Tile::BendSW),
            'F' => Some(Tile::BendSE),
            '.' => Some(Tile::Ground),
            'S' => Some(Tile::StartPosition),
            _ => None, // Handle unexpected characters
        }
    }

    fn connections(&self) -> &'static str {
        match self {
            Tile::VerticalPipe => "north-south",
            Tile::HorizontalPipe => "east-west",
            Tile::BendNE => "north-east",
            Tile::BendNW => "north-west",
            Tile::BendSW => "south-west",
            Tile::BendSE => "south-east",
            Tile::Ground => "none",
            Tile::StartPosition => "unknown",
        }
    }     
} 
enum Directions {
    North,
    East,
    South,
    West
}




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

    let first_connections = [start_location]

    let pointer_one = start_location.clone();
    let pointer_two = start_location.clone();

    let completed_loop: bool = false;

    while !completed_loop {
        
    }
}

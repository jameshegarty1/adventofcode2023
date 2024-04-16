use std::{fs, process::exit};

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
    tile_location: (usize,usize),
}

#[derive(Debug,PartialEq)]
enum TileType {
    VerticalPipe,    // '|'
    HorizontalPipe,  // '-'
    BendNE,          // 'L'
    BendNW,          // 'J'
    BendSW,          // '7'
    BendSE,          // 'F'
    Ground,          // '.'
    StartPosition,   // 'S'
}

impl TileType {
   fn from_char(c: char) -> Option<Self> {
        match c {
            '|' => Some(TileType::VerticalPipe),
            '-' => Some(TileType::HorizontalPipe),
            'L' => Some(TileType::BendNE),
            'J' => Some(TileType::BendNW),
            '7' => Some(TileType::BendSW),
            'F' => Some(TileType::BendSE),
            '.' => Some(TileType::Ground),
            'S' => Some(TileType::StartPosition),
            _ => None, // Handle unexpected characters
        }
    }

    fn connections(&self) -> &'static str {
        match self {
            TileType::VerticalPipe => "north-south",
            TileType::HorizontalPipe => "east-west",
            TileType::BendNE => "north-east",
            TileType::BendNW => "north-west",
            TileType::BendSW => "south-west",
            TileType::BendSE => "south-east",
            TileType::Ground => "none",
            TileType::StartPosition => "unknown",
        }
    }     
}

#[derive(Debug)]
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

fn parse_file(file_path: &str) -> Vec<Tile> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut data_vec: Vec<Tile> = Vec::new();

    for (i, row) in contents.lines().enumerate(){
        for (j, ch) in row.chars().enumerate() {
            match TileType::from_char(ch) {
                Some(tile_type) => {
                    let tile_location = (i,j);
                    let tile = Tile {
                        tile_type,
                        tile_location
                    };
                    data_vec.push(tile);
                },
                None => println!("Couldnt match char {ch} to tile!"),
            };            
        }
    }
    data_vec
}

fn get_next_elements(data: &Vec<Tile>, location: &Tile) {
    let north = data.iter().find(|&tile| tile.tile_location == (location.tile_location.0, location.tile_location.1 + 1));
    let east = data.iter().find(|&tile| tile.tile_location == (location.tile_location.0 + 1, location.tile_location.1));
    let south = data.iter().find(|&tile| tile.tile_location == (location.tile_location.0, location.tile_location.1 - 1));
    let west = data.iter().find(|&tile| tile.tile_location == (location.tile_location.0 - 1, location.tile_location.1));

    let north_tile: &Tile;
    let east_tile: &Tile;
    let south_tile: &Tile;
    let west_tile: &Tile;
    match north {
        Some(tile) => {
            println!("North = {:?}",tile);
            north_tile = tile;
        },
        None => println!("North out of bounds at location: {:?}", location.tile_location), 
    }
    match east {
        Some(tile) => {
            println!("East = {:?}",tile);
            east_tile = tile;
        },
        None => println!("East out of bounds at location: {:?}", location.tile_location), 
    }
    match south {
        Some(tile) => {
            println!("South = {:?}",tile);
            south_tile = tile;
        },
        None => println!("South out of bounds at location: {:?}", location.tile_location), 
    }
    match west {
        Some(tile) => {
            println!("West = {:?}",tile);
            west_tile = tile;
        },
        None => println!("West out of bounds at location: {:?}", location.tile_location), 
    }

}

fn main() {
    let data: Vec<Tile> = parse_file("test_input");
    //println!("{:?}",data);
    
    let mut location: &Tile;    
    match data.iter().find(|&tile| tile.tile_type == TileType::StartPosition) {
        Some(tile) => {
            location = tile;
        },
        None => {
            println!("Couldnt find start position.");
            exit(0);
        }
    };



    get_next_elements(&data, &location);


    println!("Start location = {:?}", location);
    /*
    let target: char = 'S';
    let start_location: (usize,usize) = match find_char_linear(target, &data) {
        Some((i, j)) => (i, j),
        None => (0,0)
    };

    let adjacent_tiles = get_adjacent_nodes(start_location);

    let pointer_one = start_location.clone();
    let pointer_two = start_location.clone();

    let completed_loop: bool = false;

    while !completed_loop {
        
    }
    */
}

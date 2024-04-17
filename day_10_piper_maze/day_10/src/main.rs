use std::{fs, process::exit};

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
    tile_location: (i32,i32),
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

impl Directions {
    fn transformation(&self) -> (i32,i32) {
        match self {
            Directions::North => (0,1),
            Directions::East => (1,0),
            Directions::South => (0,-1),
            Directions::West => (-1,0),
        }
    }
}



fn parse_file(file_path: &str) -> Vec<Tile> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut data_vec: Vec<Tile> = Vec::new();

    for (i, row) in contents.lines().enumerate(){
        for (j, ch) in row.chars().enumerate() {
            match TileType::from_char(ch) {
                Some(tile_type) => {
                    let x_i = i as i32; 
                    let x_j = j as i32; 

                    let tile_location = (x_i,x_j);
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
    let directions = [
        Directions::North,
        Directions::East,
        Directions::South,
        Directions::West,
    ];

    let result = directions.iter().map(|dir|{
        let (dx,dy) = dir.transformation();
        let new_x = dx + location.tile_location.0;
        let new_y = dy + location.tile_location.1;
    
        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            let index = new_y as usize * width + new_x as usize;
            Some(tiles[index])
        } else {
            None
        }
    });

    }

    /*
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
    } */


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
    let start_location: (i32,usize) = match find_char_linear(target, &data) {
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

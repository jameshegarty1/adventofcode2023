use std::{fs, process::exit, usize};

#[derive(Debug, Copy, Clone)]
struct Tile {
    tile_type: TileType,
    tile_location: (i32,i32),
}

#[derive(Debug,PartialEq, Copy, Clone)]
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

    fn connections(&self) -> [&'static str; 2] {
        match self {
            TileType::VerticalPipe => ["North", "South"],
            TileType::HorizontalPipe => ["East", "West"],
            TileType::BendNE => ["North", "East"],
            TileType::BendNW => ["North", "West"],
            TileType::BendSW => ["West", "South"],
            TileType::BendSE => ["East", "South"],
            TileType::Ground => ["None", "None"],
            TileType::StartPosition => ["Unknown", "Unknown"],
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

    fn as_string(&self) -> &'static str {
        match self {
            Directions::North => "North",
            Directions::East => "East",
            Directions::South => "South",
            Directions::West => "West",
        }
    }

}



fn parse_file(file_path: &str) -> Vec<Tile> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut data_vec: Vec<Tile> = Vec::new();

    let mut width = 0;
    let mut height = 0;

    for (i, row) in contents.lines().enumerate(){
        height += 1;
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

fn get_first_connections(data: &Vec<Tile>, location: &Tile, height: i32, width: i32) {

    let directions = [
        Directions::North,
        Directions::East,
        Directions::South,
        Directions::West,
    ];

    //let mut next_elements = Vec::new();

    directions.iter().for_each(|dir|{
        let (dx,dy) = dir.transformation();
        let new_x = dx + location.tile_location.0;
        let new_y = dy + location.tile_location.1;
        println!("{new_x},{new_y}");
    
        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            let find_new_tile = data.iter().find(|&tile| tile.tile_location == (new_x, new_y));

            match find_new_tile {
                None => println!("ERROR - couldnt find next tile"),
                Some(tile) => {
                    let  connections = tile.tile_type.connections();
                    let dir_string = dir.as_string();
                    println!("Connections = {:?}",connections);
                    println!("Dir string = {:?}", dir_string);

                    if connections.contains(&dir_string) {
                        println!("Connected tile: {:?}", tile)
                    }
                
                }
            }

        } else {
            println!("Out of bounds!");
        }
    });



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

    let height = data.iter().map(|tile| tile.tile_location.1 ).max().unwrap_or(0);
    let width = data.iter().map(|tile| tile.tile_location.0 ).max().unwrap_or(0);
    get_first_connections(&data, &location, height, width);


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

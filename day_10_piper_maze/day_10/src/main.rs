use std::{fs, process::exit, usize};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tile {
    tile_char: char,
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
    
    fn to_char(&self) -> char {
        match self {
            TileType::VerticalPipe => '|',
            TileType::HorizontalPipe => '-',
            TileType::BendNE => 'L',
            TileType::BendNW => 'J',
            TileType::BendSW => '7',
            TileType::BendSE => 'F',
            TileType::Ground => '.',
            TileType::StartPosition => 'S'
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
            Directions::North => (0,-1),
            Directions::East => (1,0),
            Directions::South => (0,1),
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

    fn from_string(direction_string: &str) -> Option<Directions> {
        match direction_string {
            "North" => Some(Directions::North),
            "East" => Some(Directions::East),
            "South" => Some(Directions::South),
            "West" => Some(Directions::West),
            _ => None
        }

    }

    fn opposite(&self) -> Option<Directions> {
        match self {
            Directions::North => Some(Directions::South),
            Directions::West => Some(Directions::East),
            Directions::South => Some(Directions::North),
            Directions::East => Some(Directions::West),

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
                    let y = i as i32 +1; 
                    let x = j as i32 +1; 

                    let tile_location = (x,y);
                    let tile_char = tile_type.to_char();
                    let tile = Tile {
                        tile_char,
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

fn get_connected_tiles(data: &Vec<Tile>, location: &Tile, last_location: &mut (i32,i32), height: i32, width: i32) -> Option<(Tile, Tile)> {

    let mut directions = Vec::new();
    println!("\nLocation = {:?}",location);


    if location.tile_type != TileType::StartPosition {
        let connection_directions = location.tile_type.connections();
        directions = connection_directions.iter()
            .filter_map(|dir_string| Directions::from_string(dir_string))
            .collect();
    } else {
        directions = vec![Directions::North,Directions::East,Directions::South,Directions::West];

    }

    let mut connected_tiles = Vec::new();

    directions.iter().for_each(|dir|{
        let (dx,dy) = dir.transformation();
        let new_x = dx + location.tile_location.0;
        let new_y = dy + location.tile_location.1;
    
        if new_x >= 0 && new_x <= width as i32 && new_y >= 0 && new_y <= height as i32 {
            let find_new_tile = data.iter().find(|&tile| tile.tile_location == (new_x, new_y));

            //We have a new tile to check 
            match find_new_tile {
                None => println!("ERROR - couldnt find tile, could be out of bounds."),
                Some(tile) => {

                    let connections = tile.tile_type.connections();
                    let dir_opposite = match dir.opposite() {
                        Some(direction) => direction,
                        None => panic!("Crash and burn."),
                    };

                    //I
                    println!("Connections = {:?} opposite direction {:?} is {:?}", connections, dir, dir_opposite);
                    if connections.iter().any(|&i| i == dir_opposite.as_string() ) {
                        println!("Connected tile: {:?}", tile);
                        connected_tiles.push(tile.clone());
                    }
                
                }
            }

        } else {
            println!("Out of bounds!");
        }
    });
    println!("Last location = {:?}",location);
    connected_tiles.retain(|tile| tile.tile_location != *last_location);

    *last_location = (location.tile_location.0, location.tile_location.1);
    dbg!(&connected_tiles);
    match connected_tiles.len() {
        2 => Some((connected_tiles[0],connected_tiles[1])),
        1 => Some((connected_tiles[0],connected_tiles[0])),
        _ => None
    }
}

fn main() {
    let data: Vec<Tile> = parse_file("test_input");
    println!("{:?}",data);
    
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

    let mut last_location = (0,0);
    let height = data.iter().map(|tile| tile.tile_location.1 ).max().unwrap_or(0);
    let width = data.iter().map(|tile| tile.tile_location.0 ).max().unwrap_or(0);
    let mut cursors = match get_connected_tiles(&data, &location, &mut last_location, height, width) {
       Some((cursor1,cursor2)) => (cursor1,cursor2) ,
       None => panic!("Crash and burn")
    };

    let mut steps = 1;
    let mut last_location_0 = cursors.0.tile_location.clone();
    let mut last_location_1 = cursors.1.tile_location.clone();

    while cursors.0 != cursors.1 {
        println!("Cursors = {:?}", cursors);
        cursors.0 = match get_connected_tiles(&data, &cursors.0, &mut last_location_0, height, width) {
            Some((cursor1,cursor2)) => {
                match cursors.0 {
                    cursor1 => cursor2,
                    cursor2 => cursor1,
                    _ => panic!("Crash and burn")
                } 
            },
            None => panic!("Crash and burn")
        };
        cursors.1 = match get_connected_tiles(&data, &cursors.1, &mut last_location_1, height, width) {
            Some((cursor1,cursor2)) => {
                match cursors.0 {
                    cursor1 => cursor2,
                    cursor2 => cursor1,
                    _ => panic!("Crash and burn")
                } 
            },            None => panic!("Crash and burn")
        };
        println!("new cursors = {:?}", cursors);

        steps +=1;
    }

    println!("Steps to finish = {}",steps/2);
}

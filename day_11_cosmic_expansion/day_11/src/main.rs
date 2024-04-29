//#![allow(dead_code)]

use std::{fs};

#[derive(Debug,Clone)]
struct Galaxy {
    id: usize,
    location: (usize,usize),
}

fn parse_file(file_path: &str) -> Vec<Vec<char>>{
    let contents = fs::read_to_string(file_path).expect("Some error occurred");

    let rows: Vec<&str> = contents.lines().collect();
    println!("Initial data:");

    let mut data: Vec<Vec<char>> = Vec::new();

    for row in &rows {
        data.push(row.chars().collect());
        println!("{:?}",row);
    }
    data
}

fn register_galaxies(the_universe: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut id = 0;
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (j,row) in the_universe.iter().enumerate() {
        for (i,element) in row.iter().enumerate() {
            if *element == '#' {
                let galaxy = Galaxy {
                    id,
                    location: (i,j) 
                };
                galaxies.push(galaxy);
                id += 1;
            }
        } 
    }
    galaxies
}

fn get_empty_space(the_universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_columns: Vec<usize> = Vec::new();

    for (j,row) in the_universe.iter().enumerate() {
        if row.iter().all(|e| *e == '.') {
            empty_rows.push(j);
        }
    }

    //Calculate columns with transpose

    let rows = the_universe.len();
    let cols = if rows > 0 { the_universe[0].len() } else { 0 };

    // Transpose the grid
    let mut transposed = vec![vec!['.'; rows]; cols];
    for (i, row) in the_universe.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }

    for (j,row) in transposed.iter().enumerate() {
        if row.iter().all(|e| *e == '.') {
            empty_columns.push(j);
        }
    }

    (empty_rows,empty_columns)
}

fn apply_expansion(galaxies: &mut Vec<Galaxy>, empty_space: &(Vec<usize>, Vec<usize>), expansion_factor: usize) {
    println!("Empty space: \n {:?}", empty_space);
    let (empty_rows,empty_cols) = empty_space;
    let mut offset = 0;
    for element in empty_rows.iter() {
        //println!("Expanding at row: {element} with offset {offset}");
        galaxies.iter_mut().for_each(|galaxy| if galaxy.location.1 > *element+offset {
            //println!("Galaxy before: {:?}",galaxy);
            galaxy.location.1 += expansion_factor - 1;
            //println!("Galaxy after: {:?}", galaxy);
        });
        offset += expansion_factor - 1;

    }

    let mut offset = 0;
    for element in empty_cols.iter() {
        //println!("Expanding at column: {element} with offset {offset}");
        galaxies.iter_mut().for_each(|galaxy| if galaxy.location.0 > *element+offset {
            //println!("Galaxy before: {:?}",galaxy);
            galaxy.location.0 += expansion_factor - 1;
            //println!("Galaxy after: {:?}", galaxy);
        });
        offset += expansion_factor - 1;
    }

}

fn get_galaxy_pairs(galaxies: &Vec<Galaxy>) -> Vec<(Galaxy,Galaxy)> {
    //let pairs: Vec<(Galaxy,Galaxy)> = galaxies.iter().map(|&g| (g,galaxies.iter()))
    let pairs: Vec<(Galaxy,Galaxy)> = galaxies.iter()
        .enumerate()
        .flat_map(|(i,g)| galaxies.iter().skip(i+1).map(move |gg| (g.clone(), gg.clone()))).collect();

    pairs
}

fn print_universe(galaxies: &Vec<Galaxy>, dimensions: &(usize,usize)) {
    for j in 0..dimensions.1 {
        for i in 0..dimensions.0 {
            match galaxies.iter().find(|g| g.location == (i,j)) {
                Some(_) => print!("#"),
                None => print!(".")
            }
        }
        print!("\n");
        
    }
    
}

fn find_sum_of_distances(pairs: &Vec<(Galaxy,Galaxy)>) -> f64 {
    let mut sum_dist = 0 as f64;
    for pair in pairs {
        //println!("Pair: {:?}",pair);
        let x_dist = (pair.0.location.0 as f64 - pair.1.location.0 as f64).abs();
        let y_dist = (pair.0.location.1 as f64 - pair.1.location.1 as f64).abs();

        let dist = x_dist + y_dist;
        sum_dist += dist;
        //println!("Dist: {dist} cum_dist = {sum_dist}");

    }
    sum_dist
}

fn main() {
    let expansion_factor = 1000000;
    let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/input_main");
    let mut galaxies: Vec<Galaxy> = register_galaxies(&the_universe);
    let empty_space = get_empty_space(&the_universe);

    apply_expansion(&mut galaxies, &empty_space, expansion_factor);

    let pairs = get_galaxy_pairs(&galaxies);

    let result = find_sum_of_distances(&pairs);
    println!("Result = {result}");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_parse() {
        let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/test_input");
        assert_eq!(the_universe[0][3], '#');
        assert_eq!(the_universe[9][4], '#');
    }

    #[test]
    fn test_initial_galaxies() {
        let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/test_input");
        let galaxies: Vec<Galaxy> = register_galaxies(&the_universe);
        assert_eq!(galaxies[0].id, 0);
        assert_eq!(galaxies[0].location, (3,0));
        assert_eq!(galaxies[galaxies.len()-1].location, (4,9));
    }

    #[test]
    fn test_find_empty_space() {
        let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/test_input");
        let empty_space: (Vec<usize>,Vec<usize>) = get_empty_space(&the_universe);
        assert_eq!(empty_space.0.len(),2);
        assert_eq!(empty_space.1.len(),3);
    }

    #[test]
    fn test_expansion() {
        let expansion_factor = 2;
        let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/test_input");
        let mut galaxies: Vec<Galaxy> = register_galaxies(&the_universe);
        let empty_space = get_empty_space(&the_universe);

        println!("[TEST] Galaxy 0: {:?}",galaxies[0]);
        println!("[TEST] Galaxy -1: {:?}",galaxies[galaxies.len()-1]);
        //If we have an empty row at index 5, then all Galaxies with j-coordinate > 5 should have
        //it incresed by the (expansion factor - 1)
        apply_expansion(&mut galaxies, &empty_space, 2);

        assert_eq!(galaxies[0].location, (4,0));
        assert_eq!(galaxies[galaxies.len()-1].location, (5, 11));
        assert_eq!(galaxies[1].location, (9,1));
    }

    #[test]
    fn test_expansion_literal() {
        let expansion_factor = 2;
        let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/test_input");
        let mut galaxies: Vec<Galaxy> = register_galaxies(&the_universe);
        let empty_space = get_empty_space(&the_universe);

        let n_empty_rows = empty_space.0.len();
        let n_empty_cols = empty_space.1.len();
        let universe_dimensions = ((the_universe[0].len() + (expansion_factor-1) * n_empty_cols),(the_universe.len() + (expansion_factor-1) * n_empty_rows));
        println!("{n_empty_rows} empty rows, {n_empty_cols} empty cols");
        println!("New universe dimensions: n_rows: {} n_cols {}",universe_dimensions.0, universe_dimensions.1);
        //If we have an empty row at index 5, then all Galaxies with j-coordinate > 5 should have
        //it incresed by the (expansion factor - 1)
        apply_expansion(&mut galaxies, &empty_space, expansion_factor);

        print_universe(&galaxies, &universe_dimensions);

        let expanded = include_str!("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/expanded_test");
        let mut my_expanded = String::new();
        for j in 0..universe_dimensions.1 {
            for i in 0..universe_dimensions.0 {
                match galaxies.iter().find(|g| g.location == (i,j)) {
                    Some(_) => my_expanded.push_str("#"),
                    None => my_expanded.push_str(".")
                }
            }
            my_expanded.push_str("\n");
        
        }

        assert_eq!(my_expanded,expanded);



    }

    #[test]
    fn test_result_1() {
        let expansion_factor = 2;
        let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/test_input");
        let mut galaxies: Vec<Galaxy> = register_galaxies(&the_universe);
        let empty_space = get_empty_space(&the_universe);

        let n_empty_rows = empty_space.0.len();
        let n_empty_cols = empty_space.1.len();
        let universe_dimensions = ((the_universe.len() + (expansion_factor-1) * n_empty_rows), (the_universe[0].len() + (expansion_factor-1) * n_empty_cols));
        println!("{n_empty_rows} empty rows, {n_empty_cols} empty cols");
        println!("New universe dimensions: n_rows: {} n_cols {}",universe_dimensions.0, universe_dimensions.1);
        //If we have an empty row at index 5, then all Galaxies with j-coordinate > 5 should have
        //it incresed by the (expansion factor - 1)
        apply_expansion(&mut galaxies, &empty_space, expansion_factor);


        let pairs = get_galaxy_pairs(&galaxies);

        println!("Pairs count : \n{:?}",pairs.len());

        let result = find_sum_of_distances(&pairs);

        assert_eq!(result,374.0);

    }

    #[test]
    fn test_result_2() {
        let expansion_factor = 10;
        let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/test_input");
        let mut galaxies: Vec<Galaxy> = register_galaxies(&the_universe);
        let empty_space = get_empty_space(&the_universe);

        //If we have an empty row at index 5, then all Galaxies with j-coordinate > 5 should have
        //it incresed by the (expansion factor - 1)
        apply_expansion(&mut galaxies, &empty_space, expansion_factor);

        let pairs = get_galaxy_pairs(&galaxies);

        let result = find_sum_of_distances(&pairs);

        assert_eq!(result,1030.0);

    }
    #[test]
    fn test_result_3() {
        let expansion_factor = 100;
        let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/test_input");
        let mut galaxies: Vec<Galaxy> = register_galaxies(&the_universe);
        let empty_space = get_empty_space(&the_universe);

        //If we have an empty row at index 5, then all Galaxies with j-coordinate > 5 should have
        //it incresed by the (expansion factor - 1)
        apply_expansion(&mut galaxies, &empty_space, expansion_factor);

        let pairs = get_galaxy_pairs(&galaxies);

        let result = find_sum_of_distances(&pairs);

        assert_eq!(result,8410.0);

    }

    #[test]
    fn test_part_1() {
        let expansion_factor = 2;
        let the_universe: Vec<Vec<char>> = parse_file("/Users/jodowd/projects/adventofcode2023/day_11_cosmic_expansion/day_11/src/input_main");
        let mut galaxies: Vec<Galaxy> = register_galaxies(&the_universe);
        let empty_space = get_empty_space(&the_universe);

        //If we have an empty row at index 5, then all Galaxies with j-coordinate > 5 should have
        //it incresed by the (expansion factor - 1)
        apply_expansion(&mut galaxies, &empty_space, expansion_factor);

        let pairs = get_galaxy_pairs(&galaxies);

        let result = find_sum_of_distances(&pairs);

        assert_eq!(result,9556712.0);

    }

}

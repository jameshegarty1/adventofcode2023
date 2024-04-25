use std::{fs};

#[derive(Debug,Clone)]
struct Galaxy {
    id: usize,
    location: (usize,usize),
}

fn parse_file_and_expand_universe(file_path: &str) -> Vec<Vec<char>>{
    let contents = fs::read_to_string(file_path).expect("Some error occurred");

    let mut rows: Vec<&str> = contents.lines().collect();
    println!("Initial data:");
    for row in &rows {
        println!("{:?}",row);
    }

    let empty_rows: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter_map(|(ind,row)| {
            if row.chars().all(|c| c == '.') {
                Some(ind)
            }
            else {
                None
            }
        })
        .collect();

    println!("Empty rows: {:?}",empty_rows);
    let mut offset = 0;
    for i in &empty_rows {
        rows.insert(*i+offset, rows[*i+offset]);
        offset +=1;
    }

    let mut transposed_expanded_rows: Vec<String> = vec![String::new(); rows[0].len()];

    for row in &rows {
        for (i, ch) in row.chars().enumerate() {
            transposed_expanded_rows[i].push(ch);       }
    }

    

    let empty_columns: Vec<usize> = transposed_expanded_rows
        .iter()
        .enumerate()
        .filter_map(|(ind,row)|{
            if row.chars().all(|c| c == '.') {
                Some(ind)
            }
            else {
                None
            }
        })
        .collect();


    println!("Empty columns: {:?}",empty_columns);

    let mut offset = 0;
    for i in &empty_columns {
        println!("Current dataset:");
        for row in &transposed_expanded_rows {
            println!("{:?}",row);
        }


        println!("Re-inserting column at index {} : {:?}",*i+offset,transposed_expanded_rows[*i+offset]);
        transposed_expanded_rows.insert(*i+offset, transposed_expanded_rows[*i+offset].clone());
        offset +=1;
    }
    println!("Current dataset:");
    for row in &transposed_expanded_rows {
        println!("{:?}",row);
    }
    
    let rows_size = transposed_expanded_rows.len();
    let cols_size = transposed_expanded_rows[0].len();

    let mut expanded_data: Vec<Vec<char>> = vec![vec![' ';rows_size];cols_size];

    for (i,row) in transposed_expanded_rows.into_iter().enumerate(){
        for (j,ch) in row.chars().enumerate() {
            expanded_data[j][i] = ch;
        }
    }
    println!("\nData after expansion applied:");
    for row in &expanded_data {
        println!("{:?}",row);
    }

    expanded_data
}

fn generate_galaxies(universe: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut v_galaxies: Vec<Galaxy> = Vec::new();
    let mut ids: usize= 1;
    for (j,row) in universe.iter().enumerate() {
        for (i,ch) in row.iter().enumerate() {
            if *ch == '#' {
                let galaxy = Galaxy {
                    id: ids,
                    location: (i,j)
                };
                v_galaxies.push(galaxy);
                ids += 1;
            
            }
        }
    }

    v_galaxies
}

fn get_galaxy_pairs(galaxies: &Vec<Galaxy>) -> Vec<(Galaxy,Galaxy)> {
    //let pairs: Vec<(Galaxy,Galaxy)> = galaxies.iter().map(|&g| (g,galaxies.iter()))
    let pairs: Vec<(Galaxy,Galaxy)> = galaxies.iter()
        .enumerate()
        .flat_map(|(i,g)| galaxies.iter().skip(i+1).map(move |gg| (g.clone(), gg.clone()))).collect();

    pairs
}

fn find_sum_of_distances(pairs: &Vec<(Galaxy,Galaxy)>) {
    let mut sum_dist = 0 as f64;
    for pair in pairs {
        let x_dist = (pair.0.location.0 as f64 - pair.1.location.0 as f64).abs();
        let y_dist = (pair.0.location.1 as f64 - pair.1.location.1 as f64).abs();

        let dist = x_dist + y_dist;
        sum_dist += dist;
    }
    println!("Sum dist = {sum_dist}");

}

fn main() {
    let the_universe: Vec<Vec<char>> = parse_file_and_expand_universe("input_main");

    let v_galaxies: Vec<Galaxy> = generate_galaxies(&the_universe);

    println!("Galaxies:\n{:?}",v_galaxies);

    let pairs = get_galaxy_pairs(&v_galaxies);

    find_sum_of_distances(&pairs)
}

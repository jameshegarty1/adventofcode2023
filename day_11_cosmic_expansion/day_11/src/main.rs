use std::fs;

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

    for i in &empty_rows {
        rows.insert(*i, rows[*i]);
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

    println!("\nData after expansion applied:");

    for i in &empty_columns {
        transposed_expanded_rows.insert(*i, transposed_expanded_rows[*i].clone());
    }

    let rows_size = transposed_expanded_rows.len();
    let cols_size = transposed_expanded_rows[0].len();

    let mut expanded_data: Vec<Vec<char>> = vec![vec![' ';cols_size];rows_size];

    for (i,row) in transposed_expanded_rows.into_iter().enumerate(){
        for (j,ch) in row.chars().enumerate() {
            expanded_data[i][j] = ch;
        }
    }

    for row in &expanded_data {
        println!("{:?}",row);
    }

    expanded_data
}

fn get_galaxy_pairs(universe: &Vec<Vec<char>>) {
    let mut galaxy_locations: Vec<(usize,usize)> = Vec::new();
    for (j,row) in universe.iter().enumerate() {
        for (i,ch) in row.iter().enumerate() {
            if *ch == '#' {
                galaxy_locations.push((i,j))
            
            }
        }
    }

    println!("Galaxy locations: {:?}",galaxy_locations);

    let pairs: Vec<_> = galaxy_locations.iter()
        .map((|&g| galaxy_locations.iter().map(move |&gn| gn.to_owned() , g)))
        .flatten()
        .collect();
}

fn main() {
    let the_universe: Vec<Vec<char>> = parse_file_and_expand_universe("test_input");

    get_galaxy_pairs(&the_universe);
}

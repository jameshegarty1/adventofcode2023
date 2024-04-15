use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut data_vec: Vec<Vec<i32>> = Vec::new();


    contents.lines().for_each(|line| {
        let items: Result<Vec<i32>,_> = line.split_whitespace().map(str::parse::<i32>).collect();
        match items {
            Ok(nums) => data_vec.push(nums),
            Err(e) => println!("Err: {:?}", e),
        }
    });

    data_vec
}

fn get_diffs(vec: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut is_all_zero: bool = false;
    let mut arrays_of_diffs: Vec<Vec<i32>> = Vec::new();
    arrays_of_diffs.push(vec.clone());

    //Need to update the iterating vector each time until we hit zeros
    let mut iterator_vec: Vec<i32> = vec.clone();
    
    //Loop until 0 differences
    while is_all_zero == false {
        let mut result = Vec::new();
        for adj in iterator_vec.windows(2) {
            result.push(adj[1] - adj[0]);
        }
        arrays_of_diffs.push(result.clone());
        iterator_vec = result.clone();
        if result.iter().filter(|&n| *n == 0).count() == result.len() {
            is_all_zero = true;
        }
    }
    arrays_of_diffs
}

fn solve_part_one(nested_arrays: &mut Vec<Vec<Vec<i32>>>) {
    //To solve part 1, need to find the next element in the first sequence using the algorithm
    for sequence in &mut *nested_arrays {
        //Loop through the sequences backwards starting from the [0 0 0 0] array
        for i in (0..sequence.len()).rev() {

            let mut new_value = 0;
            // If in the zeros array, new value is 0
            if sequence[i].iter().filter(|&n| *n == 0).count() == sequence[i].len() {
                new_value = 0;
                
            } else { //New value is value to the left + value below
                new_value = sequence[i][sequence[i].len()-1] + sequence[i+1][sequence[i+1].len()-1];
            }
            
            sequence[i].push(new_value);
        } 
    }

    //Need to extract the last elemennts from the first array in each
    let extrapolated_values: Vec<i32> = nested_arrays.iter().filter_map(|vec| vec[0].last()).cloned().collect();

    println!("Part 1 - sum = {}", extrapolated_values.iter().sum::<i32>());

}

fn solve_part_two(nested_arrays: &mut Vec<Vec<Vec<i32>>>) {
    //To solve part 2, need to use the same algorithm but backwards
    for sequence in &mut *nested_arrays {
        //Loop through the sequences backwards starting from the [0 0 0 0] array
        for i in (0..sequence.len()).rev() {

            let mut new_value = 0;
            // If in the zeros array, new value is 0
            if sequence[i].iter().filter(|&n| *n == 0).count() == sequence[i].len() {
                new_value = 0;
                
            } else { //New value is value to the left + value below
                new_value = sequence[i][0] - sequence[i+1][0];                
            }
            sequence[i].insert(0, new_value);

        } 
    }


    //Need to extract the last elemennts from the first array in each
    let extrapolated_values: Vec<i32> = nested_arrays.iter().filter_map(|vec| vec[0].first()).cloned().collect();

    println!("Part 2 - sum = {}", extrapolated_values.iter().sum::<i32>());

}

fn main() {
    let data: Vec<Vec<i32>> = parse_file("input_main");
    
    let mut nested_arrays: Vec<Vec<Vec<i32>>> = Vec::new();

    for element in data {
        let differences: Vec<Vec<i32>> = get_diffs(&element);
        nested_arrays.push(differences);
    }

    solve_part_one(&mut nested_arrays);

    for array in &nested_arrays {
        println!("ARRAY = {:?}",array);
    }
    solve_part_two(&mut nested_arrays);
    

}

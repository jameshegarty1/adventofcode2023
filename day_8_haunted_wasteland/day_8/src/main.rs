use std::fs;
use std::collections::HashMap;


fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn convert_to_tuple(input: &str) -> (String, String) {
    let trimmed = input.trim().trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = trimmed.split(',').collect();

    let mut part1 = parts[0].to_string();
    let mut part2 = parts[1].to_string();

    remove_whitespace(&mut part1);
    remove_whitespace(&mut part2);
    (part1, part2)
}



fn main() {
    let mut data = fs::read_to_string("input_file").expect("Unable to read file");

    
    let mut lines = data.lines();

    //First line contains the instructions
    let instructions_string = lines.next().unwrap();

    //Map
    let mut map = HashMap::<String, (String, String)>::new();

    
    for line in lines {
        let mut my_line = String::from(line);
        if !my_line.trim().is_empty() {
            let elements: Vec<&str> = my_line.split("=").collect();
            let mut node_location = elements[0].to_string();
            let node_routes = convert_to_tuple(elements[1]);
            remove_whitespace(&mut node_location);

            map.insert(node_location, node_routes);

        }
    }

    

    let mut reached_zzz = false;

    let mut location = "AAA".to_string();

    let mut steps: usize = 0;

    

    /*
    match map.get(&location) {
        Some(thing) => println!("KEY {location}: ({},{})", thing.0, thing.1),
        None => println!("{location} not found.")

    }
    */

    while reached_zzz == false {
        for c in instructions_string.chars() {
        steps+=1;
        println!("Location = {location } Instruction = {c} steps = {steps}");
           match c {
               'L' => {
                    location = map.get(&location).map_or_else(|| location.clone(), |(next_location, _)| next_location.clone());

                    if location == "ZZZ" {
                        println!("Reached ZZZ in {} steps", steps);
                        reached_zzz = true;
                    }
                    },
               'R' => {
                    location = map.get(&location).map_or_else(|| location.clone(), |(_, next_location)| next_location.clone());

                    if location == "ZZZ" {
                        reached_zzz = true;
                    }
                },
               _ => {               }
           }

        }

        
    }


    println!("Reached ZZZ in {steps} steps");




}

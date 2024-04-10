use std::fs;
use std::collections::HashMap;


fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn convert_to_tuple(input: &str) -> (String, String) {
    let trimmed = input.trim().trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = trimmed.split(',').collect();

    (parts[0].to_string(), parts[1].to_string())
}



fn main() {
    let mut data = fs::read_to_string("test_input").expect("Unable to read file");

    
    let mut lines = data.lines();

    //First line contains the instructions
    let instructions_string = lines.next().unwrap();

    //Map
    let mut map = HashMap::<String, (String, String)>::new();

    
    for line in lines {
        let mut my_line = String::from(line);
        if !my_line.trim().is_empty() {
            let elements: Vec<&str> = my_line.split("=").collect();
            let nodeLocation = elements[0].to_string();
            let nodeRoutes = convert_to_tuple(elements[1]);

            map.insert(nodeLocation, nodeRoutes);

        }
    }

    

    let mut reached_ZZZ = false;

    let mut location = "AAA".to_string();

    for key in map.keys() {
        println!("KEY {key}");
    }

    println!("Looking for {location} in map...");

    match map.get(&location) {
        Some(thing) => println!("KEY {location}: ({},{})", thing.0, thing.1),
        None => println!("{location} not found.")

    }

    while reached_ZZZ == false {
        for c in instructions_string.chars() {
           match c {
               'L' => println!("Need to go to:"),// {} ", &map[location]),
               'R' => println!("RIGHT!"),
               _ => println!("DIDNT UNDERSTAND!"),
           }

        }
        reached_ZZZ = true;
    }




}

use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<usize>> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut data_vec: Vec<Vec<usize>> = Vec::new();


    contents.lines().for_each(|line| { 
        let items: Result<Vec<usize>,_> = line.split_whitespace().map(str::parse::<usize>).collect();
        match items {
            Ok(nums) => data_vec.push(nums),
            Err(e) => println!("Err: {:?}", e),
        }
    });

    data_vec
}

fn get_diffs(vec: &Vec) -> Vec<usize> {
    let result = Vec::new();

    vec.iter().for_each(|i| println!("{i}"));

    result

    

}

fn main() {
    let data = parse_file("test_input");

    println!("{:?}", data);

    for element in data {

        let differences = get_diffs(element);
    }
}

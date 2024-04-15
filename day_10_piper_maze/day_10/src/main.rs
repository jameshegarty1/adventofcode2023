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

fn main() {
    let data: Vec<Vec<i32>> = parse_file("input_main");
}

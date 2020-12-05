use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_input(input_name: &str) -> Result<Vec<String>, std::io::Error> {
    let mut input_path = String::from("input/");
    input_path.push_str(input_name);

    let reader = BufReader::new(File::open(input_path)?);
    reader.lines().collect()
}


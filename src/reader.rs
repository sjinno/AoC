use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_txt(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>()?;
    Ok(lines)
}

pub fn clean_input(lines: Vec<String>) -> String {
    let contents = lines.join("\n");
    let cleaned_inputs = str::replace(&contents, " ", "\n");
    cleaned_inputs
}

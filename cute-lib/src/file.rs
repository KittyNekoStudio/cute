use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(path: String) -> Vec<String> {
    let file = File::open(format!("{path}.cute")).expect("No such file found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line in read_file"))
        .collect()
}

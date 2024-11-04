use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(path: String) -> Vec<String> {
    let file = File::open(format!("{path}.cute")).expect("No such file found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line in read_file"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_read_file() {
        assert_eq!(
            read_file("tests/read-file".to_string()),
            vec!["69".to_string(), "420".to_string()]
        );
    }
}

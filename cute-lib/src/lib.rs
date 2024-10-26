mod binding;
mod environment;
mod expression;
mod file;
mod token;
mod types;
mod utils;
mod value;

const VEC_STARTING_SIZE: usize = 64000;

#[derive(Debug, PartialEq, Clone)]
pub struct Buffer(Vec<String>);

impl Buffer {
    pub fn new() -> Self {
        // TODO! change vec to somthing more efficient
        Self(Vec::with_capacity(VEC_STARTING_SIZE))
    }
    pub fn push(&mut self, string: &str) {
        for str in string.lines() {
            self.0.push(str.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_push() {
        let mut buffer = Buffer::new();
        buffer.push("1 + 3\n3 * 3\n");

        assert_eq!(buffer.0, vec!["1 + 3", "3 * 3"]);
    }
    }

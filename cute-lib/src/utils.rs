use crate::expression::Expression;
use crate::types::Operation;

fn extract(accept: impl Fn(char) -> bool, string: &str) -> (&str, &str) {
    let extracted_end = string
        .char_indices()
        .find_map(|(index, char)| {
            if accept(char) {
                return None;
            } else {
                return Some(index);
            }
        })
        .unwrap_or(string.len());

    let remainder = &string[extracted_end..];
    let extracted = &string[..extracted_end];
    (remainder, extracted)
}

pub fn extract_operation(string: &str) -> (&str, Operation) {
    let (string, _) = extract_whitespace(string);
    let op = match &string[0..1] {
        "+" => Operation::Addition,
        "-" => Operation::Subtraction,
        "*" => Operation::Multiplication,
        "/" => Operation::Division,
        _ => panic!("unreachable"),
    };
    let string = &string[1..];

    (string, op)
}

pub fn extract_number(string: &str) -> (&str, &str) {
    extract(|char| char.is_ascii_digit(), string)
}

pub fn extract_whitespace(string: &str) -> (&str, &str) {
    extract(|char| char.is_whitespace(), string)
}

pub fn extract_until_whitespace(string: &str) -> (&str, &str) {
    extract(|char| !char.is_whitespace(), string)
}

pub fn extract_keyword<'a>(string: &'a str, starting_string: &str) -> Result<&'a str, String> {
    if string.starts_with(starting_string) {
        return Ok(&string[starting_string.len()..]);
    } else {
        return Err(format! {"Error extracting keyword"});
    }
}

pub fn extract_name(string: &str) -> Result<(&str, &str), String> {
    Ok(extract(|char| !char.is_whitespace(), string))
}

pub fn convert_strings_to_expressions(buffer: Vec<String>) -> Vec<Expression> {
    let mut vec = Vec::new();
    for v in buffer {
        vec.push(Expression::new(&v).1);
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Number;
    use crate::value::Value;
    use crate::binding::Binding;

    #[test]
    fn parse_whitespace() {
        assert_eq!(Number::new("   5"), ("", Number(5)));
    }
    #[test]
    fn parse_binding_value_from_expression() {
        assert_eq!(
            Binding::new("let i = 1 + 8").unwrap().1,
            Binding {
                name: "i".to_string(),
                value: Value::Number(Number(9))
            }
        );
    }
    /*#[test]
    fn parse_binding_value_from_int() {
        assert_eq!(
            Binding::new("let i = 10").unwrap().1,
            Binding {
                name: "i".to_string(),
                value: Value::Number(Number(10))
            }
        );
    }*/
}

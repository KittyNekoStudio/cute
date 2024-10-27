/* use crate::expression::Expression;
use crate::utils::*;
use crate::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct Binding {
    pub name: String,
    pub value: Value,
}

impl Binding {
    pub fn new(string: &str) -> Result<(&str, Self), String> {
        let string = extract_keyword(string, "let")?;
        let (string, _) = extract_whitespace(string);

        let (string, name) = extract_name(string)?;
        let (string, _) = extract_whitespace(string);

        println!("{string}");
        let string = extract_keyword(string, "=")?;
        let (string, _) = extract_whitespace(string);

        let (string, expr) = Expression::new(string);
        Ok((
            string,
            Self {
                name: name.to_string(),
                value: expr.eval(),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding::Binding;
    use crate::types::Number;

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
    }*/

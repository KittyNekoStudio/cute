/*use crate::types::Number;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(Number),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding::Binding;
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
}*/

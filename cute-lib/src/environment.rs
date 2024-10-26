use crate::binding::Binding;
use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Environment {
    env: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }
    pub fn get_binding(&self, key: &str) -> Value {
        self.env.get(key).cloned().unwrap()
    }
    pub fn store_binding(&mut self, binding: Binding) {
        self.env.insert(binding.name.to_string(), binding.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Number;

    #[test]
    fn evaluate_binded_value() {
        let mut environment = Environment::new();
        let binding = Binding::new("let i = 10 + 3").unwrap().1;
        environment.store_binding(binding.clone());
        assert_eq!(
            environment.get_binding(&binding.name),
            Value::Number(Number(13))
        );
    }
}

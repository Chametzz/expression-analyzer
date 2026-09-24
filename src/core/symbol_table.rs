use std::collections::HashMap;

pub struct SymbolTable {
    variables: HashMap<String, f64>,
    operators: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            operators: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&f64> {
        self.variables.get(name)
    }

    pub fn register_operator(&mut self, symbol: &str) {
        let counter = self.operators.entry(symbol.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn get_variables(&self) -> &HashMap<String, f64> {
        &self.variables
    }

    pub fn get_operators(&self) -> &HashMap<String, usize> {
        &self.operators
    }
}

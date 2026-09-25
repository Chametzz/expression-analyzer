use crate::core::symbol_table::SymbolTable;
use std::collections::HashMap;

pub fn execute(symbol_table: &SymbolTable) -> &HashMap<String, f64> {
    symbol_table.get_variables()
}
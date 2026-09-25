use std::collections::HashMap;

use crate::symbol_table::SymbolTable;

pub fn execute(symbol_table: &SymbolTable) -> &HashMap<String, usize> {
    symbol_table.get_operators()
}
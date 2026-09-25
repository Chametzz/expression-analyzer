use crate::{core::symbol_table::SymbolTable, views::home};

mod core;
mod pkg;
mod use_cases;
mod views;

fn main() {
    let mut symbol_table = SymbolTable::new();
    home::render(&mut symbol_table);
}

use tabled::{builder::Builder, settings::Style};

use crate::{core::symbol_table::SymbolTable, pkg::terminal};

pub fn render(symbol_table: &SymbolTable) {
    terminal::clear();
    let mut builder = Builder::default();
    builder.push_record(["Variable", "Valor"]);
    let mut table = builder.build();
    table.with(Style::ascii_rounded());
    println!("{}", table);
    terminal::pause();
}

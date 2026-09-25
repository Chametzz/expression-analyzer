use tabled::{builder::Builder, settings::Style};

use crate::{core::symbol_table::SymbolTable, pkg::terminal, use_cases::get_stored_variables};

pub fn render(symbol_table: &SymbolTable) {
    terminal::clear();

    let variables = get_stored_variables::execute(symbol_table);

    let mut builder = Builder::default();
    builder.push_record(["Variable", "Valor"]);

    for (variable, value) in variables {
        builder.push_record([variable.as_str(), &value.to_string()]);
    }

    let mut table = builder.build();
    table.with(Style::ascii_rounded());

    println!("{}", table);
    terminal::pause();
}

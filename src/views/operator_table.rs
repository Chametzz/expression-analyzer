use tabled::{builder::Builder, settings::Style};

use crate::{core::symbol_table::SymbolTable, pkg::terminal, use_cases::get_stored_operators};

pub fn render(symbol_table: &SymbolTable) {
    terminal::clear();

    let operators = get_stored_operators::execute(symbol_table);

    let mut builder = Builder::default();
    builder.push_record(["Operador", "Frecuencia"]);

    for (operator, count) in operators {
        builder.push_record([operator.as_str(), &count.to_string()]);
    }

    let mut table = builder.build();
    table.with(Style::ascii_rounded());

    println!("{}", table);
    terminal::pause();
}

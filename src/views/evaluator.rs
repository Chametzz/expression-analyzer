use crate::{core::symbol_table::SymbolTable, pkg::terminal, use_cases::evaluate_expression};

pub fn render(symbol_table: &mut SymbolTable) {
    loop {
        terminal::clear();

        println!("--- EVALUAR EXPRESIÓN ---\n");

        let raw_input = terminal::read_line("");

        if raw_input.trim().is_empty() {
            println!("\n>>> No ingresó ninguna expresión.");
        } else {
            match evaluate_expression::execute(symbol_table, &raw_input) {
                Ok(result) => println!("\nResultado: {}", result),
                Err(error) => println!("\n>>> Error al evaluar: {:?}", error),
            }
        }

        println!();
        let user_response = terminal::read_line("¿Desea evaluar otra expresión? (S/N): ");

        if !user_response.trim().eq_ignore_ascii_case("s") {
            break;
        }
    }
}

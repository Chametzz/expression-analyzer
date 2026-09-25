use crate::{
    core::symbol_table::{self, SymbolTable},
    pkg::terminal::{self},
    views::{evaluator, operator_table, variable_table},
};

pub fn render(symbol_table: &mut SymbolTable) {
    loop {
        terminal::clear();
        println!("--- ANALIZADOR DE EXPRESIONES ---");
        println!("[1] Ver tabla de variables");
        println!("[2] Ver tabla de operadores");
        println!("[3] Evaluar expresión");
        println!("[exit] Salir");
        let option = terminal::read_line("Selecciona una opción: ");
        match option.trim() {
            "1" => variable_table::render(symbol_table),
            "2" => operator_table::render(symbol_table),
            "3" => evaluator::render(symbol_table),
            "exit" => {
                break;
            }
            _ => {
                println!("Opción no válida. Intente de nuevo");
                terminal::pause();
            }
        }
    }
}

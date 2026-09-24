use crate::pkg::terminal::{self};

pub fn render() {
    loop {
        terminal::clear();
        println!("--- ANALIZADOR DE EXPRESIONES ---");
        println!("[1] Ver tabla de variables");
        println!("[2] Ver tabla de operadores");
        println!("[3] Evaluar expresión");
        println!("[exit] Salir");
        let option = terminal::read_line("Selecciona una opción: ");
        match option.trim() {
            "1" => println!("placeholder"),
            "2" => println!("placeholder"),
            "3" => println!("placeholder"),
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

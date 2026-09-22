use std::io::{self, Write};

#[allow(dead_code)]
pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

#[allow(dead_code)]
pub fn show_cursor(show: bool) {
    if show {
        print!("\x1B[?25h");
    } else {
        print!("\x1B[?25l");
    }
    let _ = io::stdout().flush();
}

#[allow(dead_code)]
pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input");
    buffer.trim().to_string()
}

#[allow(dead_code)]
pub fn pause() {
    show_cursor(true);
    print!("\nPresiona Enter para continuar...");
    let _ = io::stdout().flush();
    let mut unused = String::new();
    let _ = io::stdin().read_line(&mut unused);
    show_cursor(false);
}

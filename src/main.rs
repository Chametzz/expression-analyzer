use crate::views::home;

mod core;
mod pkg;
mod use_cases;
mod views;

fn main() {
    home::render();
}

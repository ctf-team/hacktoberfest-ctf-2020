use crate::console::Console;

mod commands;
mod console;
mod parser;

fn main() {
    let mut console = Console::new();
    console.run();
}

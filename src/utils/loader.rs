use colored::Colorize;
use std::{thread, time::Duration};
use terminal_size::{terminal_size, Width};

use std::io::Write;

pub fn loader() {
    let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    for frame in &frames {
        print!("\r{} Loading...", frame.green());
        std::io::stdout().flush().unwrap(); // force print immediately
        thread::sleep(Duration::from_millis(80));
    }
}

pub fn clear_loader() {
    let width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(20); // fallback если не удалось определить

    print!("\r{}\r", " ".repeat(width));
    std::io::stdout().flush().unwrap();
}

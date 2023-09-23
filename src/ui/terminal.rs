use std::io::Stdout;

use crossterm::{execute, terminal::{Clear, disable_raw_mode}, cursor::MoveTo};

use crate::utils::files::{get_directories};

pub fn display_files(cwd: &str) {
    let files = get_directories(cwd);
    files.iter().for_each(|data| {
        println!("Filename: {}", data.filename);
    })
}

pub fn reset_display() {
    let mut stdout = std::io::stdout();
    execute!(stdout, Clear(crossterm::terminal::ClearType::All));
    execute!(stdout, MoveTo(0,0));
}


pub fn cleanup_gracefully() {
    reset_display();
    disable_raw_mode().ok();
}


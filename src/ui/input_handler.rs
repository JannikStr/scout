use std::io::Write;

use crossterm::{event::{KeyEvent, KeyCode}, execute, cursor::{MoveTo, SavePosition, RestorePosition}, ExecutableCommand};


pub fn handle_keys(evt: KeyEvent) -> bool {
    match evt.code {
        KeyCode::Char('q') => { false }
        KeyCode::Char('?') => {
            show_help();
            true
        }
        _ => { true }
    }
}

fn show_help() {
    let mut stdout = std::io::stdout();
    let (_width, height) = crossterm::terminal::size().unwrap_or((0,0));
    stdout.execute(SavePosition).ok();
    stdout.execute(MoveTo(0, height-1)).ok();
    print!("Use h🠘 j🠛 k🠙 l🠚 for movement and <Enter> to open a file or directory");
    stdout.flush();
    stdout.execute(RestorePosition).ok();
}

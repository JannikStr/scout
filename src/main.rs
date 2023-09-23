use std::time::Duration;

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use scout::ui::input_handler::handle_keys;
use scout::ui::terminal::{self, cleanup_gracefully};
use crossterm::{execute, cursor::MoveTo, terminal::Clear};
use crossterm::event::{read, Event, poll};


fn main() {
    let raw_mode_enabled = enable_raw_mode();

    if raw_mode_enabled.is_err() {
        println!("Failed to enter raw mode!");
        return;
    }

    terminal::reset_display();
    terminal::display_files(std::env::current_dir().unwrap().to_str().unwrap());

    let mut running = true ;
    while running {
        running = handle_input();
    }

    cleanup_gracefully();
}

fn handle_input() -> bool {
    let mut continue_running = true;

    if poll(Duration::from_millis(500)).unwrap() {
        let event = read().unwrap();

        continue_running = match event {
            Event::Key(event) => handle_keys(event),
            _ => { true },
        };
    }
    continue_running
}

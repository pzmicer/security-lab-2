// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod gui;
mod payloads;
mod windows;

use gui::State;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let state = State {
            file_path: args[1].clone(),
            should_archive: args[2].parse().unwrap(),
            should_hash: args[3].parse().unwrap(),
            should_hide: args[4].parse().unwrap(),
        };
        payloads::do_things(&state).unwrap_or_else(|e| {
            windows::show_message(&e.to_string());
        });
    } else {
        if windows::is_elevated() {
            windows::show_message("You can't run this app as administrator!");
            return;
        }
        gui::run();
    }
}

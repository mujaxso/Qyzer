// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(e) = desktop::run() {
        eprintln!("Error while running tauri application: {}", e);
        std::process::exit(1);
    }
}

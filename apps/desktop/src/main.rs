mod app;
mod bootstrap;
mod commands;
mod ui;
mod events;

use std::env;
use eframe::egui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Force X11 backend to avoid Wayland issues
    unsafe {
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }
    
    let args: Vec<String> = env::args().collect();
    let initial_workspace_path = if args.len() >= 2 {
        Some(args[1].clone())
    } else {
        None
    };
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Neote"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Neote",
        options,
        Box::new(|_cc| {
            match initial_workspace_path {
                Some(path) => {
                    match app::App::new(path) {
                        Ok(app) => Box::new(app),
                        Err(e) => {
                            eprintln!("Failed to initialize app with workspace: {}", e);
                            // Start with empty app
                            Box::new(app::App::empty())
                        }
                    }
                }
                None => Box::new(app::App::empty()),
            }
        }),
    ).map_err(|e| e.into())
}

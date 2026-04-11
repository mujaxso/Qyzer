mod app;
mod bootstrap;
mod commands;
mod ui;
mod events;
mod message;
mod state;
mod theme;
mod update;
mod view;
mod explorer;
mod settings;

use app::App;
use iced::{Application, Settings};

fn main() -> iced::Result {
    // Check if we're in a Wayland environment
    let wayland_display = std::env::var("WAYLAND_DISPLAY").is_ok();
    let xdg_session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    
    println!("WAYLAND_DISPLAY: {:?}", std::env::var("WAYLAND_DISPLAY").ok());
    println!("XDG_SESSION_TYPE: {}", xdg_session_type);
    
    // For Hyprland, which is a Wayland compositor, we need to ensure proper backend selection
    // Some users have reported issues with file pickers on Wayland
    // Let's try to force X11 if we're having issues, but first try Wayland
    
    let mut settings = Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            min_size: Some(iced::Size::new(400.0, 300.0)),
            visible: true,
            position: iced::window::Position::Centered,
            resizable: true,
            decorations: true,
            platform_specific: iced::window::PlatformSpecific {
                // On Wayland, try to set app_id for better integration
                application_id: Some("neote".to_string()),
                ..Default::default()
            },
            ..Default::default()
        },
        // Enable antialiasing for better text rendering
        antialiasing: true,
        // Use JetBrains Mono Nerd Font as the default font for better programming experience with icons
        default_font: iced::font::Font::with_name("JetBrainsMono Nerd Font"),
        default_text_size: iced::Pixels(14.0),
        ..Default::default()
    };
    
    // If we suspect Wayland issues, we can try to force X11 backend
    // But let's not force it by default, as Wayland should work
    // Instead, add an environment variable check to allow users to force X11
    if std::env::var("NEOTE_FORCE_X11").is_ok() {
        println!("Forcing X11 backend as requested by NEOTE_FORCE_X11");
        settings.window.platform_specific = iced::window::PlatformSpecific {
            // Force X11 backend
            x11: iced::window::X11Settings {
                override_redirect: false,
                // Other X11 specific settings
                ..Default::default()
            },
            ..Default::default()
        };
    }
    
    App::run(settings)
}

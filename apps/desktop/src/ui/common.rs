// Common UI primitives and helpers for Neote - designed for both core and extensions
use iced::{Color, widget::{button, container, text}};
use crate::theme::SemanticColors;

/// Common spacing constants for consistent UI rhythm
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 12.0;
pub const SPACING_LG: f32 = 16.0;
pub const SPACING_XL: f32 = 24.0;

/// Common border radius values
pub const RADIUS_SM: f32 = 4.0;
pub const RADIUS_MD: f32 = 6.0;
pub const RADIUS_LG: f32 = 8.0;

/// Common sizes for UI elements
pub const ICON_SIZE: f32 = 18.0;
pub const BUTTON_HEIGHT_SM: f32 = 32.0;
pub const BUTTON_HEIGHT_MD: f32 = 40.0;
pub const PANEL_HEADER_HEIGHT: f32 = 48.0;
pub const STATUS_BAR_HEIGHT: f32 = 28.0;

/// Extension-friendly theme access interface
/// This trait allows extensions to access Neote's semantic colors and styles
pub trait ThemeConsumer {
    /// Get the full semantic colors palette
    fn colors(&self) -> SemanticColors;
    
    /// Get primary text color
    fn text_primary(&self) -> Color {
        self.colors().text_primary
    }
    
    /// Get secondary text color
    fn text_secondary(&self) -> Color {
        self.colors().text_secondary
    }
    
    /// Get panel background color
    fn background_panel(&self) -> Color {
        self.colors().panel_background
    }
    
    /// Get elevated panel background color
    fn background_elevated(&self) -> Color {
        self.colors().elevated_panel_background
    }
    
    /// Get accent color
    fn accent(&self) -> Color {
        self.colors().accent
    }
}

/// Common button styles for extensions to use
pub mod buttons {
    use super::*;
    
    /// Primary button style using semantic colors
    pub fn primary(colors: &SemanticColors) -> button::Appearance {
        button::Appearance {
            background: Some(colors.accent.into()),
            border: iced::Border {
                color: colors.accent,
                width: 1.0,
                radius: RADIUS_SM.into(),
            },
            text_color: colors.text_on_accent,
            ..Default::default()
        }
    }
    
    /// Secondary button style using semantic colors
    pub fn secondary(colors: &SemanticColors) -> button::Appearance {
        button::Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: RADIUS_SM.into(),
            },
            text_color: colors.text_secondary,
            ..Default::default()
        }
    }
    
    /// Text button style (minimal appearance)
    pub fn text(colors: &SemanticColors) -> button::Appearance {
        button::Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            text_color: colors.text_secondary,
            ..Default::default()
        }
    }
}

/// Common container styles for extensions to use
pub mod containers {
    use super::*;
    
    /// Panel container style
    pub fn panel(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.panel_background.into()),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: RADIUS_SM.into(),
            },
            ..Default::default()
        }
    }
    
    /// Elevated panel container style
    pub fn elevated(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.elevated_panel_background.into()),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: RADIUS_MD.into(),
            },
            ..Default::default()
        }
    }
    
    /// Card container style (for content blocks)
    pub fn card(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.elevated_panel_background.into()),
            border: iced::Border {
                color: colors.divider,
                width: 1.0,
                radius: RADIUS_MD.into(),
            },
            ..Default::default()
        }
    }
}

/// Common text styles for extensions to use
pub mod texts {
    use super::*;
    
    /// Primary text style
    pub fn primary(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.text_primary),
        }
    }
    
    /// Secondary text style
    pub fn secondary(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.text_secondary),
        }
    }
    
    /// Muted text style
    pub fn muted(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.text_muted),
        }
    }
    
    /// Success text style
    pub fn success(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.success),
        }
    }
    
    /// Warning text style
    pub fn warning(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.warning),
        }
    }
    
    /// Error text style
    pub fn error(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.error),
        }
    }
}

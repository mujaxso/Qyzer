use iced::{Color, widget::{button, container, text, text_input}};
use crate::theme::{current_colors, NeoteTheme, SemanticColors};
use super::common;

/// Get current theme colors from app state
pub fn colors(theme: NeoteTheme) -> SemanticColors {
    current_colors(theme)
}

/// Style helpers for UI components - designed for both core and extensions
pub struct StyleHelpers {
    pub colors: SemanticColors,
    pub tokens: crate::theme::DesignTokens,
}

impl StyleHelpers {
    pub fn new(theme: NeoteTheme) -> Self {
        Self {
            colors: colors(theme),
            tokens: crate::theme::DesignTokens::default(),
        }
    }
    
    /// Get semantic colors for extension use
    pub fn semantic_colors(&self) -> SemanticColors {
        self.colors
    }
    
    /// Implement ThemeConsumer trait for extensions
    pub fn as_theme_consumer(&self) -> impl common::ThemeConsumer {
        self
    }
}

impl common::ThemeConsumer for StyleHelpers {
    fn colors(&self) -> SemanticColors {
        self.colors
    }
}

// Panel container styles
impl StyleHelpers {
    /// Panel container style
    pub fn panel_container(&self) -> container::Appearance {
        common::containers::panel(&self.colors)
    }
    
    /// Elevated panel container style
    pub fn elevated_container(&self) -> container::Appearance {
        common::containers::elevated(&self.colors)
    }
    
    /// Status bar container style
    pub fn status_bar_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.status_bar_background.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            text_color: None,
            shadow: Default::default(),
        }
    }
    
    /// Card container style
    pub fn card_container(&self) -> container::Appearance {
        common::containers::card(&self.colors)
    }
}

// Button styles
impl StyleHelpers {
    /// Primary button style
    pub fn primary_button(&self) -> button::Appearance {
        common::buttons::primary(&self.colors)
    }
    
    /// Secondary button style
    pub fn secondary_button(&self) -> button::Appearance {
        common::buttons::secondary(&self.colors)
    }
    
    /// Text button style
    pub fn text_button(&self) -> button::Appearance {
        common::buttons::text(&self.colors)
    }
}

// Text input styles
impl StyleHelpers {
    /// Text input style
    pub fn text_input(&self) -> text_input::Appearance {
        text_input::Appearance {
            background: self.colors.elevated_panel_background.into(),
            border: iced::Border {
                color: self.colors.border,
                width: 1.0,
                radius: self.tokens.radius_sm.into(),
            },
            icon_color: self.colors.text_muted,
        }
    }
}

// Text styles
impl StyleHelpers {
    /// Primary text style
    pub fn text_primary(&self) -> text::Appearance {
        common::texts::primary(&self.colors)
    }
    
    /// Secondary text style
    pub fn text_secondary(&self) -> text::Appearance {
        common::texts::secondary(&self.colors)
    }
    
    /// Muted text style
    pub fn text_muted(&self) -> text::Appearance {
        common::texts::muted(&self.colors)
    }
    
    /// Success text style
    pub fn text_success(&self) -> text::Appearance {
        common::texts::success(&self.colors)
    }
    
    /// Warning text style
    pub fn text_warning(&self) -> text::Appearance {
        common::texts::warning(&self.colors)
    }
    
    /// Error text style
    pub fn text_error(&self) -> text::Appearance {
        common::texts::error(&self.colors)
    }
    
    /// Text on accent style
    pub fn text_on_accent(&self) -> text::Appearance {
        text::Appearance {
            color: Some(self.colors.text_on_accent),
        }
    }
}

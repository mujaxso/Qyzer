//! Theme and design system for Zaroxi
//! This crate provides color themes, design tokens, and styling utilities

mod colors;
mod zaroxi_theme;

pub use colors::*;
pub use zaroxi_theme::{ZaroxiTheme, SemanticColors, DesignTokens};

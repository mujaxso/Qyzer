//! Syntax layer for Neote IDE.
//!
//! This crate provides Tree-sitter-based syntax parsing, highlighting,
//! and language support for the editor. It's designed to be:
//! - Incremental: updates syntax trees efficiently after edits
//! - Modular: clean separation between parsing, highlighting, and UI
//! - Extensible: easy to add new languages and features
//! - Performant: minimal overhead for large files and frequent edits

pub mod error;
pub mod highlight;
pub mod language;
pub mod manager;

// Re-export main types for convenience
pub use error::SyntaxError;
pub use highlight::{Highlight, HighlightSpan};
pub use language::LanguageId;
pub use manager::SyntaxManager;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_toml_language_detection() {
        use std::path::Path;
        
        assert_eq!(
            LanguageId::from_path(Path::new("Cargo.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new("test.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new(".clippy.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new("pyproject.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new("rustfmt.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new("config.toml")),
            LanguageId::Toml
        );
    }
}

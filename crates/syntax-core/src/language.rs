//! Language identification and grammar loading.

use std::path::Path;
use tree_sitter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageId {
    Rust,
    Toml,
    Markdown,
    PlainText,
}

impl LanguageId {
    /// Determine language from file path.
    pub fn from_path(path: &Path) -> Self {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // Check for TOML files
        if ext.eq_ignore_ascii_case("toml") {
            return LanguageId::Toml;
        }
        
        // Check for Markdown files
        if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown") {
            return LanguageId::Markdown;
        }
        
        // Check for specific TOML filenames
        match name.as_str() {
            "cargo.toml" | "rust-toolchain.toml" | "clippy.toml" | "rustfmt.toml" 
            | ".clippy.toml" | ".rustfmt.toml" | "pyproject.toml" | "taplo.toml" => {
                return LanguageId::Toml;
            }
            _ => {}
        }
        
        match ext {
            "rs" => LanguageId::Rust,
            _ => LanguageId::PlainText,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageId::Rust => "rust",
            LanguageId::Toml => "toml",
            LanguageId::Markdown => "markdown",
            LanguageId::PlainText => "plaintext",
        }
    }

    /// Return the statically linked Tree-sitter language if available.
    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            #[cfg(feature = "rust")]
            LanguageId::Rust => Some(tree_sitter_rust::language()),
            #[cfg(not(feature = "rust"))]
            LanguageId::Rust => None,
            #[cfg(feature = "toml")]
            LanguageId::Toml => Some(tree_sitter_toml::language()),
            #[cfg(not(feature = "toml"))]
            LanguageId::Toml => None,
            #[cfg(feature = "markdown")]
            LanguageId::Markdown => {
                // Load Markdown grammar dynamically from runtime directory
                // This avoids version conflicts with tree-sitter crate
                use crate::runtime::Runtime;
                use std::sync::OnceLock;
                
                static MARKDOWN_LANGUAGE: OnceLock<Option<tree_sitter::Language>> = OnceLock::new();
                
                *MARKDOWN_LANGUAGE.get_or_init(|| {
                    let runtime = Runtime::new();
                    // Try to load markdown grammar from runtime directory
                    match runtime.load_language("markdown") {
                        Ok(lang) => {
                            // Successfully loaded
                            Some(lang)
                        }
                        Err(e) => {
                            // Log error but don't panic
                            eprintln!("Note: Markdown grammar not available: {}", e);
                            eprintln!("To enable Markdown syntax highlighting:");
                            eprintln!("1. Build tree-sitter-markdown grammar");
                            eprintln!("2. Place it in {}", runtime.grammar_library_path("markdown").display());
                            None
                        }
                    }
                })
            }
            #[cfg(not(feature = "markdown"))]
            LanguageId::Markdown => None,
            LanguageId::PlainText => None,
        }
    }
}

//! Registry of available Tree-sitter grammars and their download/compile instructions.

use std::collections::HashMap;

/// Information needed to download and compile a Tree-sitter grammar
#[derive(Debug, Clone)]
pub struct GrammarInfo {
    /// Language identifier (e.g., "markdown", "rust", "python")
    pub language_id: String,
    /// GitHub repository URL
    pub repo_url: String,
    /// Optional subdirectory within the repo (for mono-repos)
    pub subdirectory: Option<String>,
    /// Files needed for compilation
    pub source_files: Vec<String>,
    /// Query files to copy
    pub query_files: Vec<&'static str>,
}

impl GrammarInfo {
    /// Get the default registry of grammars
    pub fn default_registry() -> HashMap<String, Self> {
        let mut registry = HashMap::new();
        
        // Markdown
        registry.insert("markdown".to_string(), GrammarInfo {
            language_id: "markdown".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-markdown".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm", "locals.scm"],
        });
        
        // Rust
        registry.insert("rust".to_string(), GrammarInfo {
            language_id: "rust".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-rust".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm", "locals.scm"],
        });
        
        // TOML
        registry.insert("toml".to_string(), GrammarInfo {
            language_id: "toml".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-toml".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm"],
        });
        
        // JavaScript
        registry.insert("javascript".to_string(), GrammarInfo {
            language_id: "javascript".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-javascript".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm", "locals.scm"],
        });
        
        // TypeScript
        registry.insert("typescript".to_string(), GrammarInfo {
            language_id: "typescript".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-typescript".to_string(),
            subdirectory: Some("typescript".to_string()),
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm", "locals.scm"],
        });
        
        // Python
        registry.insert("python".to_string(), GrammarInfo {
            language_id: "python".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-python".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm", "locals.scm"],
        });
        
        // JSON
        registry.insert("json".to_string(), GrammarInfo {
            language_id: "json".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-json".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm"],
        });
        
        // HTML
        registry.insert("html".to_string(), GrammarInfo {
            language_id: "html".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-html".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm"],
        });
        
        // CSS
        registry.insert("css".to_string(), GrammarInfo {
            language_id: "css".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-css".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm"],
        });
        
        // Go
        registry.insert("go".to_string(), GrammarInfo {
            language_id: "go".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-go".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm", "locals.scm"],
        });
        
        // C
        registry.insert("c".to_string(), GrammarInfo {
            language_id: "c".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-c".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm", "locals.scm"],
        });
        
        // C++
        registry.insert("cpp".to_string(), GrammarInfo {
            language_id: "cpp".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-cpp".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm", "locals.scm"],
        });
        
        // Java
        registry.insert("java".to_string(), GrammarInfo {
            language_id: "java".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-java".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm"],
        });
        
        // Bash
        registry.insert("bash".to_string(), GrammarInfo {
            language_id: "bash".to_string(),
            repo_url: "https://github.com/tree-sitter/tree-sitter-bash".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm", "injections.scm"],
        });
        
        registry
    }
    
    /// Get the grammar info for a language, if available
    pub fn for_language(language_id: &str) -> Option<Self> {
        Self::default_registry().get(language_id).cloned()
    }
    
    /// Get all available language IDs
    pub fn available_languages() -> Vec<String> {
        Self::default_registry().keys().cloned().collect()
    }
}

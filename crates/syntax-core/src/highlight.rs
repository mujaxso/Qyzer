//! Syntax highlighting using Tree-sitter queries.

use crate::error::SyntaxError;
use crate::language::LanguageId;
use tree_sitter::{Query, QueryCursor, Tree};

/// A highlight span in the document
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Highlight type (maps to theme token categories)
    pub highlight: Highlight,
}

/// Highlight types (maps to Tree-sitter capture names)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Highlight {
    Comment,
    String,
    Keyword,
    Function,
    Variable,
    Type,
    Constant,
    Attribute,
    Operator,
    Number,
    Property,
    Namespace,
    Plain,
}

/// Highlight a syntax tree for a given language.
pub fn highlight(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    match language {
        LanguageId::Rust => highlight_with_query(language, source, tree),
        #[cfg(feature = "toml")]
        LanguageId::Toml => highlight_with_query(language, source, tree),
        #[cfg(not(feature = "toml"))]
        LanguageId::Toml => Ok(Vec::new()),
        #[cfg(feature = "markdown")]
        LanguageId::Markdown => highlight_with_query(language, source, tree),
        #[cfg(not(feature = "markdown"))]
        LanguageId::Markdown => Ok(Vec::new()),
        LanguageId::PlainText => Ok(Vec::new()),
    }
}

fn highlight_with_query(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    let query_str = get_query_for_language(language)?;
    let ts_lang = language
        .tree_sitter_language()
        .ok_or_else(|| SyntaxError::LanguageNotSupported(language.as_str().to_string()))?;

    // If the query fails (e.g., because of unrecognized node types), we return empty highlights
    // rather than propagating an error. This allows the editor to keep working without syntax
    // highlighting for that particular language.
    let query = match Query::new(ts_lang, query_str) {
        Ok(q) => q,
        Err(e) => {
            // Log the error for debugging
            eprintln!("DEBUG: Tree-sitter query error for {}: {}", language.as_str(), e);
            return Ok(Vec::new());
        }
    };

    let mut cursor = QueryCursor::new();
    let root_node = tree.root_node();
    let mut spans = Vec::new();

    for match_ in cursor.matches(&query, root_node, source.as_bytes()) {
        for capture in match_.captures {
            let node = capture.node;
            let start = node.start_byte();
            let end = node.end_byte();
            let capture_name = &query.capture_names()[capture.index as usize];
            let highlight = map_capture_name(capture_name);
            spans.push(HighlightSpan {
                start,
                end,
                highlight,
            });
        }
    }

    // Sort spans by start position
    spans.sort_by_key(|span| span.start);
    
    Ok(spans)
}

pub fn map_capture_name(name: &str) -> Highlight {
    match name {
        // Programming language captures
        "comment" => Highlight::Comment,
        "string" => Highlight::String,
        "string.escape" => Highlight::String,
        "escape" => Highlight::String,
        "string.special" => Highlight::String,
        "keyword" => Highlight::Keyword,
        "function" | "function.call" | "function.method" => Highlight::Function,
        "function.macro" | "macro" => Highlight::Function, // Macros use function color
        "variable" | "variable.parameter" => Highlight::Variable,
        "variable.builtin" => Highlight::Type, // Built-in variables like 'self' use type color (amber in dark theme)
        "type" | "type.builtin" => Highlight::Type,
        "constant" | "constant.builtin" => Highlight::Constant,
        "attribute" => Highlight::Attribute,
        "operator" => Highlight::Operator,
        "punctuation.bracket" => Highlight::Operator,
        "punctuation.delimiter" => Highlight::Operator,
        "number" => Highlight::Number,
        "boolean" => Highlight::Constant,
        "property" => Highlight::Property,
        "namespace" => Highlight::Namespace,
        "constructor" => Highlight::Type,
        "label" => Highlight::Variable,
        "mutable_specifier" => Highlight::Keyword,
        "lifetime" => Highlight::Type,  // Lifetimes use type color
        
        // Markdown-specific captures
        "heading" => Highlight::Type,        // Headings use type color (distinct but not gaudy)
        "heading.1" => Highlight::Type,
        "heading.2" => Highlight::Type,
        "heading.3" => Highlight::Type,
        "heading.4" => Highlight::Type,
        "heading.5" => Highlight::Type,
        "heading.6" => Highlight::Type,
        "emphasis" => Highlight::Comment,    // Emphasis uses comment color (elegant)
        "strong" => Highlight::Keyword,      // Strong emphasis uses keyword color
        "link" => Highlight::Variable,       // Links use variable color (recognizable)
        "link_text" => Highlight::Variable,
        "link_url" => Highlight::String,     // URLs use string color
        "link_title" => Highlight::String,
        "inline_code" => Highlight::Constant, // Inline code uses constant color (readable)
        "code_fence" => Highlight::Property, // Code fences use property color
        "code_fence_content" => Highlight::Plain, // Code fence content will be injected
        "blockquote" => Highlight::Comment,  // Blockquotes use comment color
        "list" => Highlight::Property,       // Lists use property color
        "list_item" => Highlight::Property,
        "thematic_break" => Highlight::Operator, // Thematic breaks use operator color
        "html_block" => Highlight::Attribute, // HTML blocks use attribute color
        "html_inline" => Highlight::Attribute,
        "table" => Highlight::Property,      // Tables use property color
        "table_header" => Highlight::Type,
        "table_row" => Highlight::Property,
        "table_cell" => Highlight::Plain,
        _ => Highlight::Plain,
    }
}

pub fn get_query_for_language(language: LanguageId) -> Result<&'static str, SyntaxError> {
    match language {
        LanguageId::Rust => {
            #[cfg(feature = "rust")]
            {
                Ok(include_str!(
                    "../../../runtime/treesitter/languages/rust/queries/highlights.scm"
                ))
            }
            #[cfg(not(feature = "rust"))]
            Err(SyntaxError::LanguageNotSupported(
                "rust feature not enabled".to_string(),
            ))
        }
        LanguageId::Toml => {
            #[cfg(feature = "toml")]
            {
                // Use the official highlight query from the tree-sitter-toml crate
                // This ensures we're using the correct node types for the exact grammar version
                use tree_sitter_toml;
                Ok(tree_sitter_toml::HIGHLIGHT_QUERY)
            }
            #[cfg(not(feature = "toml"))]
            Err(SyntaxError::LanguageNotSupported(
                "toml support not compiled".to_string(),
            ))
        }
        LanguageId::Markdown => {
            #[cfg(feature = "markdown")]
            {
                // Try to get the highlight query from tree-sitter-markdown
                // Different versions may expose it differently
                use tree_sitter_markdown;
                
                // Check for various possible query constant names
                // Some grammars use HIGHLIGHTS_QUERY (plural)
                #[allow(unused_imports)]
                use tree_sitter_markdown as ts_md;
                
                // First try HIGHLIGHTS_QUERY (common in newer versions)
                #[cfg(any(feature = "markdown"))]
                {
                    // If the constant doesn't exist, we'll fall back to a basic query
                    // that we define inline for common markdown constructs
                    const FALLBACK_MARKDOWN_QUERY: &str = "
                        (atx_heading) @heading
                        (setext_heading) @heading
                        (emphasis) @emphasis
                        (strong_emphasis) @strong
                        (inline_code_span) @inline_code
                        (code_fence) @code_fence
                        (block_quote) @blockquote
                        (list_item) @list
                        (thematic_break) @thematic_break
                        (link) @link
                        (link_text) @link_text
                        (link_destination) @link_url
                        (image) @link
                        (image_description) @link_text
                        (image_title) @link_title
                    ";
                    
                    // Attempt to use the crate's query if available
                    // Note: tree-sitter-markdown 0.7 may not expose query constants
                    // For now, use our fallback query
                    Ok(FALLBACK_MARKDOWN_QUERY)
                }
            }
            #[cfg(not(feature = "markdown"))]
            Err(SyntaxError::LanguageNotSupported(
                "markdown support not compiled".to_string(),
            ))
        }
        LanguageId::PlainText => Err(SyntaxError::LanguageNotSupported(
            "plaintext has no syntax queries".to_string(),
        )),
    }
}

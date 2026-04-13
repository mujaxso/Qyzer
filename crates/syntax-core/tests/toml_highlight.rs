use syntax_core::highlight::{highlight, Highlight};
use syntax_core::language::LanguageId;
use tree_sitter::Parser;

#[test]
fn test_toml_highlighting() {
    let source = r#"
[package]
name = "neote"
version = "0.1.0"
edition = "2024"

[dependencies]
iced = { version = "0.13", features = ["multi-window"] }
tree-sitter = "0.21"

[profile.release]
opt-level = 3
lto = true

# This is a comment
boolean_value = true
integer_value = 42
float_value = 3.14
string_value = "hello"
"#;

    let mut parser = Parser::new();
    let language = LanguageId::Toml.tree_sitter_language().expect("TOML language should be available");
    parser.set_language(language).expect("Failed to set TOML language");
    
    let tree = parser.parse(source, None).expect("Failed to parse TOML");
    let highlights = highlight(LanguageId::Toml, source, &tree).expect("Failed to highlight TOML");
    
    // Debug output
    println!("Found {} highlights", highlights.len());
    for span in &highlights {
        println!("  {:?}: {:?}", (span.start, span.end), span.highlight);
    }
    
    // At least some highlights should be found
    assert!(!highlights.is_empty(), "TOML highlighting should produce some highlights");
    
    // Check for specific highlights
    let mut found_comment = false;
    let mut found_string = false;
    let mut found_number = false;
    let mut found_property = false;
    let mut found_type = false;
    let mut found_constant = false;
    
    for span in &highlights {
        match span.highlight {
            Highlight::Comment => found_comment = true,
            Highlight::String => found_string = true,
            Highlight::Number => found_number = true,
            Highlight::Property => found_property = true,
            Highlight::Type => found_type = true,
            Highlight::Constant => found_constant = true,
            _ => {}
        }
    }
    
    assert!(found_comment, "Should highlight comments");
    assert!(found_string, "Should highlight strings");
    assert!(found_number, "Should highlight numbers");
    assert!(found_property, "Should highlight properties (keys)");
    assert!(found_type, "Should highlight types (table headers)");
    assert!(found_constant, "Should highlight constants (booleans)");
}

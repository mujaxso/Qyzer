//! Position and location types for text buffers.
//!
//! Defines line/column positions, offsets, and related operations for
//! navigating and manipulating text.

/// A position in the text buffer, measured in line and column (both 0-indexed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// Helper functions for converting between different position representations.
/// These are thin wrappers around the TextBuffer's methods.
pub mod conversion {
    use super::Position;
    
    /// Convert a character index to a line/column position.
    pub fn char_to_position(buffer: &crate::buffer::TextBuffer, char_idx: usize) -> Result<Position, String> {
        if char_idx > buffer.len_chars() {
            return Err(format!("Character index {} out of bounds", char_idx));
        }
        let line = buffer.char_to_line(char_idx);
        let line_start = buffer.line_to_char(line);
        let column = char_idx - line_start;
        Ok(Position::new(line, column))
    }
    
    /// Convert a line/column position to a character index.
    pub fn position_to_char(buffer: &crate::buffer::TextBuffer, position: Position) -> Result<usize, String> {
        buffer.line_col_to_char(position.line, position.column)
    }
}

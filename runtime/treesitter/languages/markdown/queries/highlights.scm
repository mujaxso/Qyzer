; Markdown highlighting query for tree-sitter-markdown-inline
; Based on actual node types from debug output
; Enhanced with official tree-sitter-markdown query patterns

; Headings
(atx_heading) @heading
(atx_heading_marker) @heading.marker
(setext_heading) @heading
(setext_heading_text) @heading

; Emphasis
(emphasis) @emphasis
(strong_emphasis) @strong_emphasis

; Code
(code_span) @inline_code
(fenced_code_block) @code_block
(fenced_code_block_delimiter) @code_block.delimiter
(code_fence_content) @code_block.content

; Lists
(list) @list
(list_item) @list.item
(list_marker) @list.marker
(task_list_marker) @list.marker

; Blockquotes
(block_quote) @blockquote
(block_quote_marker) @blockquote.marker

; Links
(link) @link
(link_text) @link.text
(link_destination) @link.destination
(link_title) @link.title
(shortcut_link) @link
(full_reference_link) @link
(collapsed_reference_link) @link
(inline_link) @link

; Images
(image) @image
(image_description) @image.description

; HTML
(html_block) @html
(html_inline) @html

; Thematic breaks
(thematic_break) @thematic_break

; Tables
(table) @table
(table_header) @table.header
(table_row) @table.row
(table_cell) @table.cell
(table_delimiter_row) @table.delimiter

; Escape sequences
(backslash_escape) @escape

; Line breaks
(hard_line_break) @line_break

; Strikethrough
(strikethrough) @strikethrough

; Autolinks
(uri_autolink) @link.autolink
(email_autolink) @link.autolink

; Inline content
(paragraph) @paragraph

; Fallback for everything else
(_) @plain

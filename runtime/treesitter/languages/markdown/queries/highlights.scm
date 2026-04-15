; Markdown highlighting query for tree-sitter-markdown-inline
; Based on actual node types from the grammar

; Try to match various inline elements we saw in debug output
(_backslash_escape) @string.escape
(entity_reference) @string
(numeric_character_reference) @string

; Try to match punctuation
[
  "["
  "]"
  "<"
  ">"
  "!"
  "\""
  "#"
  "$"
  "%"
  "&"
  "'"
  "*"
  "+"
  ","
  "-"
  "."
] @operator

; Try to match text content (if such a node exists)
; We'll use wildcard for everything else
(_) @plain

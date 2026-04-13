; TOML highlight queries for Tree‑sitter
; Based on the official tree-sitter-toml grammar repository

(comment) @comment

(string) @string
(escape_sequence) @string.escape

(integer) @number
(float) @number

(boolean) @boolean

(date) @string.special
(time) @string.special
(date_time) @string.special

; Keys
(bare_key) @property
(dotted_key (bare_key) @property)

; Table headers
(table_header (bare_key) @type)
(table_header (dotted_key (bare_key) @type))
(table_array_header (bare_key) @type)
(table_array_header (dotted_key (bare_key) @type))

; Punctuation
"." @punctuation.delimiter
"=" @operator
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"," @punctuation.delimiter

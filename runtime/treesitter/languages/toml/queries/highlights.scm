; TOML highlight queries for Tree‑sitter
; Based on tree-sitter-toml grammar version 0.20

(comment) @comment

(string) @string
(escape_sequence) @string.escape

(integer) @number
(float) @number

(boolean) @boolean

; Basic punctuation that likely exists
"=" @operator
"[" @operator
"]" @operator
"{" @operator
"}" @operator
"," @operator
"." @operator

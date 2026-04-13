; TOML syntax highlighting queries for Tree-sitter

; Comments
(comment) @comment

; Basic values
(string) @string
(integer) @number
(float) @number
(boolean) @constant
(offset_date_time) @string.special
(local_date_time) @string.special
(local_date) @string.special
(local_time) @string.special

; Tables
(table) @type
(table_array_element) @type

; Keys
(bare_key) @variable
(quoted_key) @variable
(dotted_key (bare_key) @variable)
(dotted_key (quoted_key) @variable)

; Key-value pairs
(pair key: (bare_key) @variable)
(pair key: (quoted_key) @variable)
(pair key: (dotted_key (bare_key) @variable))
(pair key: (dotted_key (quoted_key) @variable))

; Array
(array) @punctuation.bracket

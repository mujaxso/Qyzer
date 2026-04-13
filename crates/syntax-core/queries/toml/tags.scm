; TOML tags queries for symbol navigation

(table (bare_key) @definition.type)
(table_array_element (bare_key) @definition.type)

; Key-value pairs can be considered as definitions
(pair key: (bare_key) @definition.field)
(pair key: (quoted_key) @definition.field)
; TOML tags queries for symbol navigation

(table (bare_key) @definition.type)
(table_array_element (bare_key) @definition.type)

; Key-value pairs can be considered as definitions
(pair key: (bare_key) @definition.field)
(pair key: (quoted_key) @definition.field)

; TOML locals queries for variable tracking
; TOML doesn't have scoped variables in the traditional sense,
; but we can track table definitions

(table (bare_key) @definition.type)
(table_array_element (bare_key) @definition.type)

; Reference to tables in dotted keys
(dotted_key (bare_key) @reference)
; TOML locals queries for variable tracking
; TOML doesn't have scoped variables in the traditional sense,
; but we can track table definitions

(table (bare_key) @definition.type)
(table_array_element (bare_key) @definition.type)

; Reference to tables in dotted keys
(dotted_key (bare_key) @reference)

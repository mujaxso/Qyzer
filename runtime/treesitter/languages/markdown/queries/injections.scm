((html_tag) @injection.content
  (#set! injection.language "html"))

((latex_block) @injection.content
  (#set! injection.language "latex"))
; Markdown injections for fenced code blocks
(fenced_code_block
  (info_string) @injection.language
  (code_fence_content) @injection.content)

; Support for inline code if needed
; (code_span) @injection.content

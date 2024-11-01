(comment) @comment

(call_expression
  callee: (_) @callee (#any-of? @callee "compile" "regex::compile")
  (_ [(string_content) (raw_string_content)] @content)
  (#set! language "regex"))

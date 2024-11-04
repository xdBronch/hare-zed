(function_declaration
  (function_attribute)? @context
  "fn" @context
  name: (_) @name) @item

(type_declaration
  "type" @context
  (identifier) @name) @item

(field
  (identifier) @name) @item

(constant_declaration
  "def" @context
  (identifier) @context
  ":" (_)? @context
  "=" (_)) @item

(global_declaration
  "let" @context
  (global_binding
    (identifier) @context
    ":" (_)? @context
    "=" (_))) @item

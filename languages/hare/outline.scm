(function_declaration
  (function_attribute)? @context
  "fn" @context
  name: (_) @name) @item

(type_declaration
  "type" (identifier) @name "=" (_) @context) @item

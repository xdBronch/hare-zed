(function_declaration
  (function_attribute)? @context
  "fn" @context
  name: (_) @name) @item

(type_declaration
  (identifier) @name 
  (_) @context) @item

(field
  (identifier) @name) @item

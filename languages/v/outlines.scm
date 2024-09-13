(comment) @annotation
;; (type_declaration
;;     "type" @context
;;     (type_spec
;;         name: (_) @name)) @item




(struct_declaration (identifier)  "struct" @context @name ) @item
(enum_declaration (identifier)  "enum" @context @name) @item
(union_declaration (identifier)  "union" @context @name ) @item


(function_declaration
    "fn" @context
    name: (identifier) @name
    parameters: (parameter_list
      "("
      ")")) @item



(method_declaration
    "fn" @context
    receiver: (parameter_list
        "(" @context
        (parameter_declaration
            name: (_) @name
            type: (_) @context)
        ")" @context)
    name: (field_identifier) @name
    parameters: (parameter_list
      "("
      ")")) @item

(const_declaration
    "const" @context
    (const_spec
        name: (identifier) @name) @item)

(source_file
    (var_declaration
        "var" @context
        (var_spec
            name: (identifier) @name) @item))

(method_elem
    name: (_) @name
    parameters: (parameter_list
      "(" @context
      ")" @context)) @item

(field_declaration
    name: (_) @name) @item

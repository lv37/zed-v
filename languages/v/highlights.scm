(ERROR) @error

[
 (line_comment)
 (block_comment)
 (shebang)
] @comment


(simple_statement) @variable
(import_path) @variable

(qualified_type
  module: (reference_expression) @variable)
(var_definition_list) @variable
(range
  _ (reference_expression) @variable)
(return_statement
  expression_list: (expression_list) @variable)
(const_definition
  name: (identifier) @constant)

(parameter_declaration
  name: (identifier) @parameter)
(function_declaration
  name: (identifier) @function)
(function_declaration
  receiver: (receiver)
  name: (identifier) @function)

(short_lambda
  (reference_expression) @parameter)
(call_expression
  name: (selector_expression
    field: (reference_expression) @function))
(call_expression
  name: (reference_expression) @function)

(type_reference_expression) @type
(pointer_type) @type
(array_type) @type

(simple_statement
	(selector_expression
  field: (reference_expression) @property))

(int_literal) @number
(interpreted_string_literal) @string
(rune_literal) @string
(escape_sequence) @string.escape

(struct_declaration
  name: (identifier) @type)
(enum_declaration
  name: (identifier) @type)

(struct_field_declaration
  name: (identifier) @property)
(enum_field_definition
  name: (identifier) @property)

[
 "as"
 "asm"
 "assert"
 ;"atomic"
 "break"
 "const"
 "__global"
 "continue"
 "defer"
 "else"
 "enum"
 "fn"
 "for"
 "$for"
 "go"
 "spawn"
 "goto"
 "if"
 "$if"
 "implements"
 "import"
 "in"
 "!in"
 "interface"
 "is"
 "!is"
 "lock"
 "match"
 "module"
 "mut"
 "or"
 "pub"
 "return"
 "rlock"
 "select"
 "shared"
 "static"
 "struct"
 "type"
 "union"
 "unsafe"
] @keyword

[
 (true)
 (false)
] @boolean

[
 "."
 ","
 ":"
 ";"
] @punctuation.delimiter

[
 "("
 ")"
 "{"
 "}"
 "["
 "]"
] @punctuation.bracket

(array_creation) @punctuation.bracket

[
 "++"
 "--"

 "+"
 "-"
 "*"
 "/"
 "%"

 "~"
 "&"
 "|"
 "^"

 "!"
 "&&"
 "||"
 "!="

 "<<"
 ">>"

 "<"
 ">"
 "<="
 ">="

 "+="
 "-="
 "*="
 "/="
 "&="
 "|="
 "^="
 "<<="
 ">>="

 "="
 ":="
 "=="

 "?"
 "<-"
 "$"
 ".."
 "..."
] @operator

(php_tag) @tag
"?>" @tag


; Variables

(relative_scope) @variable.builtin

(variable_name) @variable

((name) @constructor
 (#match? @constructor "^[A-Z]"))

((name) @constant
 (#match? @constant "^_?[A-Z][A-Z\\d_]+$"))
((name) @constant.builtin
 (#match? @constant.builtin "^__[A-Z][A-Z\d_]+__$"))

; Types

(primitive_type) @type.builtin
(cast_type) @type.builtin
(named_type (name) @type) @type
(named_type (qualified_name) @type) @type

; Functions

(array_creation_expression "array" @function.builtin)
(list_literal "list" @function.builtin)

(method_declaration
  name: (name) @function.method)

(function_call_expression
  function: [(qualified_name (name)) (name)] @function)

(scoped_call_expression
  name: (name) @function)

(member_call_expression
  name: (name) @function.method)

(function_definition
  name: (name) @function)

; Member

(property_element
  (variable_name) @property)

(member_access_expression
  name: (variable_name (name)) @property)
(member_access_expression
  name: (name) @property)

; Basic tokens
[
  (string)
  (string_value)
  (encapsed_string)
  (heredoc)
  (heredoc_body)
  (nowdoc_body)
] @string
(boolean) @constant.builtin
(null) @constant.builtin
(integer) @number
(float) @number
(comment) @comment

((name) @variable.builtin
 (#eq? @variable.builtin "this"))

"$" @operator

; Keywords

"abstract" @keyword
"as" @keyword
"break" @keyword
"case" @keyword
"catch" @keyword
"class" @keyword
"const" @keyword
"continue" @keyword
"declare" @keyword
"default" @keyword
"do" @keyword
"echo" @keyword
"else" @keyword
"elseif" @keyword
"enddeclare" @keyword
"endforeach" @keyword
"endif" @keyword
"endswitch" @keyword
"endwhile" @keyword
"extends" @keyword
"final" @keyword
"finally" @keyword
"for" @keyword
"foreach" @keyword
"function" @keyword
"global" @keyword
"if" @keyword
"implements" @keyword
"include_once" @keyword
"include" @keyword
"insteadof" @keyword
"interface" @keyword
"namespace" @keyword
"new" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"require_once" @keyword
"require" @keyword
"return" @keyword
"static" @keyword
"switch" @keyword
"throw" @keyword
"trait" @keyword
"try" @keyword
"use" @keyword
"while" @keyword

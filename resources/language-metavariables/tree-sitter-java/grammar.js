const DIGITS = token(choice('0', seq(/[1-9]/, optional(seq(optional('_'), sep1(/[0-9]+/, /_+/))))));
const DECIMAL_DIGITS = token(sep1(/[0-9]+/, '_'));
const HEX_DIGITS = token(sep1(/[A-Fa-f0-9]+/, '_'));
const PREC = {
  // https://introcs.cs.princeton.edu/java/11precedence/
  COMMENT: 0, // //  /*  */
  ASSIGN: 1, // =  += -=  *=  /=  %=  &=  ^=  |=  <<=  >>=  >>>=
  DECL: 2,
  ELEMENT_VAL: 2,
  TERNARY: 3, // ?:
  OR: 4, // ||
  AND: 5, // &&
  BIT_OR: 6, // |
  BIT_XOR: 7, // ^
  BIT_AND: 8, // &
  EQUALITY: 9, // ==  !=
  GENERIC: 10,
  REL: 10, // <  <=  >  >=  instanceof
  SHIFT: 11, // <<  >>  >>>
  ADD: 12, // +  -
  MULT: 13, // *  /  %
  CAST: 14, // (Type)
  OBJ_INST: 14, // new
  UNARY: 15, // ++a  --a  a++  a--  +  -  !  ~
  ARRAY: 16, // [Index]
  OBJ_ACCESS: 16, // .
  PARENS: 16, // (Expression)
  CLASS_LITERAL: 17, // .
  GRIT_METAVARIABLE: 100,
};

module.exports = grammar({
  name: 'java',

  extras: ($) => [$.line_comment, $.block_comment, /\s/],

  supertypes: ($) => [
    $.expression,
    $.declaration,
    $.statement,
    $.primary_expression,
    $._literal,
    $._type,
    $._simple_type,
    $._unannotated_type,
    $.comment,
    $.module_directive,
  ],

  inline: ($) => [
    $._name,
    $._simple_type,
    $._reserved_identifier,
    $._class_body_declaration,
    $._variable_initializer,
  ],

  conflicts: ($) => [
    [$.modifiers, $.annotated_type, $.receiver_parameter],
    [$.modifiers, $.annotated_type, $.module_declaration, $.package_declaration],
    [$._unannotated_type, $.primary_expression, $.inferred_parameters],
    [$._unannotated_type, $.primary_expression],
    [$._unannotated_type, $.primary_expression, $.scoped_type_identifier],
    [$._unannotated_type, $.scoped_type_identifier],
    [$._unannotated_type, $.generic_type],
    [$.generic_type, $.primary_expression],
    [$.expression, $.statement],
    // Only conflicts in switch expressions
    [$.lambda_expression, $.primary_expression],
    [$.inferred_parameters, $.primary_expression],
    [$.class_literal, $.field_access],
    // GRIT METAVARIABLE CONFLICTS
    [$.element_value_pair, $.identifier],
    [$.statement, $.identifier]
    // GRIT OTHER CONFLICTS
  ],

  word: ($) => $._identifier,

  rules: {
    program: ($) => repeat(field('statements', $.statement)),

    // Literals

    _literal: ($) =>
      choice(
        $.decimal_integer_literal,
        $.hex_integer_literal,
        $.octal_integer_literal,
        $.binary_integer_literal,
        $.decimal_floating_point_literal,
        $.hex_floating_point_literal,
        $.true,
        $.false,
        $.character_literal,
        $.string_literal,
        $.null_literal,
      ),

    decimal_integer_literal: ($) => token(seq(DIGITS, optional(choice('l', 'L')))),

    hex_integer_literal: ($) =>
      token(seq(choice('0x', '0X'), HEX_DIGITS, optional(choice('l', 'L')))),

    octal_integer_literal: ($) =>
      token(seq(choice('0o', '0O', '0'), sep1(/[0-7]+/, '_'), optional(choice('l', 'L')))),

    binary_integer_literal: ($) =>
      token(seq(choice('0b', '0B'), sep1(/[01]+/, '_'), optional(choice('l', 'L')))),

    decimal_floating_point_literal: ($) =>
      token(
        choice(
          seq(
            DECIMAL_DIGITS,
            '.',
            optional(DECIMAL_DIGITS),
            optional(seq(/[eE]/, optional(choice('-', '+')), DECIMAL_DIGITS)),
            optional(/[fFdD]/),
          ),
          seq(
            '.',
            DECIMAL_DIGITS,
            optional(seq(/[eE]/, optional(choice('-', '+')), DECIMAL_DIGITS)),
            optional(/[fFdD]/),
          ),
          seq(DIGITS, /[eEpP]/, optional(choice('-', '+')), DECIMAL_DIGITS, optional(/[fFdD]/)),
          seq(DIGITS, optional(seq(/[eE]/, optional(choice('-', '+')), DECIMAL_DIGITS)), /[fFdD]/),
        ),
      ),

    hex_floating_point_literal: ($) =>
      token(
        seq(
          choice('0x', '0X'),
          choice(seq(HEX_DIGITS, optional('.')), seq(optional(HEX_DIGITS), '.', HEX_DIGITS)),
          optional(seq(/[eEpP]/, optional(choice('-', '+')), DIGITS, optional(/[fFdD]/))),
        ),
      ),

    true: ($) => 'true',

    false: ($) => 'false',

    character_literal: ($) => token(seq("'", repeat1(choice(/[^\\'\n]/, /\\./, /\\\n/)), "'")),

    string_literal: ($) => choice($._string_literal, $._multiline_string_literal),
    _string_literal: ($) =>
      seq(
        '"',
        choice($.grit_metavariable, repeat(choice($.string_fragment, $.escape_sequence))),
        '"',
      ),
    _multiline_string_literal: ($) =>
      seq(
        '"""',
        choice(
          $.grit_metavariable,
          repeat(
            choice(
              alias($._multiline_string_fragment, $.multiline_string_fragment),
              $._escape_sequence,
            ),
          ),
        ),
        '"""',
      ),
    // Workaround to https://github.com/tree-sitter/tree-sitter/issues/1156
    // We give names to the token() constructs containing a regexp
    // so as to obtain a node in the CST.
    //
    string_fragment: ($) => token.immediate(prec(1, /[^"\\]+/)),
    _multiline_string_fragment: () => prec.right(choice(/[^"]+/, seq(/"[^"]*/, repeat(/[^"]+/)))),

    _escape_sequence: ($) =>
      choice(
        prec(2, token.immediate(seq('\\', /[^abfnrtvxu'\"\\\?]/))),
        prec(1, $.escape_sequence),
      ),
    escape_sequence: () =>
      token.immediate(
        seq(
          '\\',
          choice(/[^xu0-7]/, /[0-7]{1,3}/, /x[0-9a-fA-F]{2}/, /u[0-9a-fA-F]{4}/, /u{[0-9a-fA-F]+}/),
        ),
      ),

    null_literal: ($) => 'null',

    // Expressions

    expression: ($) =>
      choice(
        $.assignment_expression,
        $.binary_expression,
        $.instanceof_expression,
        $.lambda_expression,
        $.ternary_expression,
        $.update_expression,
        $.primary_expression,
        $.unary_expression,
        $.cast_expression,
        $.switch_expression,
      ),

    cast_expression: ($) =>
      prec(
        PREC.CAST,
        choice(
          seq('(', field('type', $._type), ')', field('value', $.expression)),
          seq(
            '(',
            sep1(field('type', $._type), '&'),
            ')',
            field('value', choice($.primary_expression, $.lambda_expression)),
          ),
        ),
      ),

    assign: ($) => '=',
    add_assign: ($) => '+=',
    sub_assign: ($) => '-=',
    mul_assign: ($) => '*=',
    div_assign: ($) => '/=',
    and_assign: ($) => '&=',
    or_assign: ($) => '|=',
    xor_assign: ($) => '^=',
    mod_assign: ($) => '%=',
    lshift_assign: ($) => '<<=',
    rshift_assign: ($) => '>>=',
    urshift_assign: ($) => '>>>=',

    assignment_expression: ($) =>
      prec.right(
        PREC.ASSIGN,
        seq(
          field(
            'left',
            choice($.identifier, $._reserved_identifier, $.field_access, $.array_access),
          ),
          field(
            'operator',
            choice(
              $.assign,
              $.add_assign,
              $.sub_assign,
              $.mul_assign,
              $.div_assign,
              $.and_assign,
              $.or_assign,
              $.xor_assign,
              $.mod_assign,
              $.lshift_assign,
              $.rshift_assign,
              $.urshift_assign,
            ),
          ),
          field('right', $.expression),
        ),
      ),

    greater_than: ($) => '>',
    less_than: ($) => '<',
    greater_than_or_equal: ($) => '>=',
    less_than_or_equal: ($) => '<=',
    equal: ($) => '==',
    not_equal: ($) => '!=',
    and: ($) => '&&',
    or: ($) => '||',
    plus: ($) => '+',
    minus: ($) => '-',
    times: ($) => '*',
    divide: ($) => '/',
    bit_and: ($) => '&',
    bit_or: ($) => '|',
    bit_xor: ($) => '^',
    mod: ($) => '%',
    lshift: ($) => '<<',
    rshift: ($) => '>>',
    urshift: ($) => '>>>',

    binary_expression: ($) =>
      choice(
        ...[
          [$.greater_than, PREC.REL],
          [$.less_than, PREC.REL],
          [$.greater_than_or_equal, PREC.REL],
          [$.less_than_or_equal, PREC.REL],
          [$.equal, PREC.EQUALITY],
          [$.not_equal, PREC.EQUALITY],
          [$.and, PREC.AND],
          [$.or, PREC.OR],
          [$.plus, PREC.ADD],
          [$.minus, PREC.ADD],
          [$.times, PREC.MULT],
          [$.divide, PREC.MULT],
          [$.bit_and, PREC.BIT_AND],
          [$.bit_or, PREC.BIT_OR],
          [$.bit_xor, PREC.BIT_XOR],
          [$.mod, PREC.MULT],
          [$.lshift, PREC.SHIFT],
          [$.rshift, PREC.SHIFT],
          [$.urshift, PREC.SHIFT],
        ].map(([operator, precedence]) =>
          prec.left(
            precedence,
            seq(
              field('left', $.expression),
              field('operator', operator),
              field('right', $.expression),
            ),
          ),
        ),
      ),

    instanceof_expression: ($) =>
      prec(
        PREC.REL,
        seq(
          field('left', $.expression),
          'instanceof',
          optional('final'),
          field('right', $._type),
          field('name', optional(choice($.identifier, $._reserved_identifier))),
        ),
      ),

    lambda_expression: ($) =>
      seq(
        field(
          'parameters',
          choice($.identifier, $.formal_parameters, $.inferred_parameters, $._reserved_identifier),
        ),
        '->',
        field('body', choice($.expression, $.block)),
      ),

    inferred_parameters: ($) =>
      seq('(', commaSep1(field('identifier', choice($.identifier, $._reserved_identifier))), ')'),

    ternary_expression: ($) =>
      prec.right(
        PREC.TERNARY,
        seq(
          field('condition', $.expression),
          '?',
          field('consequence', $.expression),
          ':',
          field('alternative', $.expression),
        ),
      ),

    not: ($) => '!',
    bit_not: ($) => '~',

    unary_expression: ($) =>
      choice(
        ...[
          [$.plus, PREC.UNARY],
          [$.minus, PREC.UNARY],
          [$.not, PREC.UNARY],
          [$.bit_not, PREC.UNARY],
        ].map(([operator, precedence]) =>
          prec.left(precedence, seq(field('operator', operator), field('operand', $.expression))),
        ),
      ),

    increment: ($) => '++',
    decrement: ($) => '--',

    update_expression: ($) =>
      prec.left(
        PREC.UNARY,
        choice(
          // Post (in|de)crement is evaluated before pre (in|de)crement
          seq($.expression, $.increment),
          seq($.expression, $.decrement),
          seq($.increment, $.expression),
          seq($.decrement, $.expression),
        ),
      ),

    primary_expression: ($) =>
      choice(
        $._literal,
        $.class_literal,
        $.this,
        $.identifier,
        $._reserved_identifier,
        $.parenthesized_expression,
        $.object_creation_expression,
        $.field_access,
        $.array_access,
        $.method_invocation,
        $.method_reference,
        $.array_creation_expression,
      ),

    array_creation_expression: ($) =>
      prec.right(
        seq(
          'new',
          repeat($._annotation),
          field('type', $._simple_type),
          choice(
            seq(
              field('dimensions', repeat1($.dimensions_expr)),
              field('dimensions', optional($.dimensions)),
            ),
            seq(field('dimensions', $.dimensions), field('value', $.array_initializer)),
          ),
        ),
      ),

    // array_creation_expression: ($) =>
    // prec.right(
    //   seq(
    //     'new',
    //     field('annotations', repeat($._annotation)),
    //     field('type', $._simple_type),
    //     choice(
    //       seq(
    //         field('dimensions', repeat1($.dimensions_expr)),
    //         field('dimensions', optional($.dimensions)),
    //       ),
    //       seq(field('dimensions', $.dimensions), field('value', $.array_initializer)),
    //     ),
    //   ),
    // ),

    dimensions_expr: ($) =>
      seq(field('annotations', repeat($._annotation)), '[', field('dimension', $.expression), ']'),

    parenthesized_expression: ($) => seq('(', field('expression', $.expression), ')'),

    condition: ($) => seq('(', field('condition', $.expression), ')'),

    class_literal: ($) =>
      prec.dynamic(PREC.CLASS_LITERAL, seq(field('type', $._unannotated_type), '.', 'class')),

    object_creation_expression: ($) =>
      choice(
        $._unqualified_object_creation_expression,
        seq(field('class', $.primary_expression), '.', $._unqualified_object_creation_expression),
      ),

    _unqualified_object_creation_expression: ($) =>
      prec.right(
        seq(
          'new',
          field('type_arguments', optional($.type_arguments)),
          field('type', $._simple_type),
          field('arguments', $.argument_list),
          field('body', optional($.class_body)),
        ),
      ),

    field_access: ($) =>
      seq(
        field('object', choice($.primary_expression, $.super)),
        optional(seq('.', $.super)),
        '.',
        field('field', choice($.identifier, $._reserved_identifier, $.this)),
      ),

    array_access: ($) =>
      seq(field('array', $.primary_expression), '[', field('index', $.expression), ']'),

    method_invocation: ($) =>
      seq(
        choice(
          field('name', choice($.identifier, $._reserved_identifier)),
          seq(
            field('object', choice($.primary_expression, $.super)),
            '.',
            optional(seq($.super, '.')),
            field('type_arguments', optional($.type_arguments)),
            field('name', choice($.identifier, $._reserved_identifier)),
          ),
        ),
        field('arguments', $.argument_list),
      ),

    argument_list: ($) => seq('(', commaSep(field('arguments', $.expression)), ')'),

    method_reference: ($) =>
      seq(
        field('class', choice($._type, $.primary_expression, $.super)),
        '::',
        optional(field('type_argument', $.type_arguments)),
        field('method', choice('new', $.identifier)),
      ),

    type_arguments: ($) => seq('<', commaSep(field('type', choice($._type, $.wildcard))), '>'),

    wildcard: ($) =>
      seq(
        field('annotation', repeat($._annotation)),
        '?',
        optional(field('type', $._wildcard_bounds)),
      ),

    _wildcard_bounds: ($) => choice(seq('extends', $._type), seq($.super, $._type)),

    dimensions: ($) =>
      prec.right(repeat1(seq(field('annotations', repeat($._annotation)), '[', ']'))),

    switch_expression: ($) =>
      seq('switch', field('condition', $.parenthesized_expression), field('body', $.switch_block)),

    switch_block: ($) =>
      seq(
        '{',
        field('cases', choice(repeat($.switch_block_statement_group), repeat($.switch_rule))),
        '}',
      ),

    switch_block_statement_group: ($) =>
      prec.left(
        seq(
          repeat1(seq(field('condition', $.switch_label), ':')),
          repeat(field('consequence', $.statement)),
        ),
      ),

    switch_rule: ($) =>
      seq(
        field('condition', $.switch_label),
        '->',
        field('consequence', choice($.expression_statement, $.throw_statement, $.block)),
      ),

    switch_label: ($) => choice(seq('case', commaSep1(field('labels', $.expression))), 'default'),

    // Statements

    statement: ($) =>
      choice(
        $.declaration,
        $.expression_statement,
        $.labeled_statement,
        $.if_statement,
        $.while_statement,
        $.for_statement,
        $.enhanced_for_statement,
        $.block,
        ';',
        $.assert_statement,
        $.do_statement,
        $.break_statement,
        $.continue_statement,
        $.return_statement,
        $.yield_statement,
        $.switch_expression, // switch statements and expressions are identical
        $.synchronized_statement,
        $.local_variable_declaration,
        $.throw_statement,
        $.try_statement,
        $.try_with_resources_statement,
        $.grit_metavariable,
      ),

    block: ($) => seq('{', field('statements', repeat($.statement)), '}'),

    expression_statement: ($) => seq(field('expression', $.expression), ';'),

    labeled_statement: ($) =>
      seq(field('label', $.identifier), ':', field('statement', $.statement)),

    assert_statement: ($) =>
      choice(
        seq('assert', field('assertion', $.expression), ';'),
        seq('assert', field('assertion', $.expression), ':', field('error', $.expression), ';'),
      ),

    do_statement: ($) =>
      seq(
        'do',
        field('body', $.statement),
        'while',
        field('condition', $.parenthesized_expression),
        ';',
      ),

    break_statement: ($) => seq('break', optional(field('label', $.identifier)), ';'),

    continue_statement: ($) => seq('continue', optional(field('label', $.identifier)), ';'),

    return_statement: ($) => seq('return', optional(field('value', $.expression)), ';'),

    yield_statement: ($) => seq('yield', field('value', $.expression), ';'),

    synchronized_statement: ($) =>
      seq('synchronized', field('lock', $.parenthesized_expression), field('body', $.block)),

    throw_statement: ($) => seq('throw', field('error', $.expression), ';'),

    try_statement: ($) =>
      seq(
        'try',
        field('body', $.block),
        choice(
          field('catch', repeat1($.catch_clause)),
          seq(field('catch', repeat($.catch_clause)), field('finally', $.finally_clause)),
        ),
      ),

    catch_clause: ($) =>
      seq('catch', '(', field('parameter', $.catch_formal_parameter), ')', field('body', $.block)),

    catch_formal_parameter: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        field('type', $.catch_type),
        $._variable_declarator_id,
      ),

    catch_type: ($) => sep1(field('type', $._unannotated_type), '|'),

    finally_clause: ($) => seq('finally', field('body', $.block)),

    try_with_resources_statement: ($) =>
      seq(
        'try',
        field('resources', $.resource_specification),
        field('body', $.block),
        field('catch', repeat($.catch_clause)),
        optional(field('finally', $.finally_clause)),
      ),

    resource_specification: ($) =>
      seq('(', sep1(field('resources', $.resource), ';'), optional(';'), ')'),

    resource: ($) =>
      choice(
        seq(
          optional(field('modifiers', $.modifiers)),
          field('type', $._unannotated_type),
          $._variable_declarator_id,
          '=',
          field('value', $.expression),
        ),
        field('resource', $.identifier),
        field('resource', $.field_access),
      ),

    if_statement: ($) =>
      prec.right(
        seq(
          'if',
          field('condition', $.condition),
          field('consequence', $.statement),
          optional(seq('else', field('alternative', $.statement))),
        ),
      ),

    while_statement: ($) =>
      seq('while', field('condition', $.condition), field('body', $.statement)),

    for_statement: ($) =>
      seq(
        'for',
        '(',
        choice(
          field('init', $.local_variable_declaration),
          seq(commaSep(field('init', $.expression)), ';'),
        ),
        field('condition', optional($.expression)),
        ';',
        commaSep(field('update', $.expression)),
        ')',
        field('body', $.statement),
      ),

    enhanced_for_statement: ($) =>
      seq(
        'for',
        '(',
        optional(field('modifiers', $.modifiers)),
        field('type', $._unannotated_type),
        $._variable_declarator_id,
        ':',
        field('value', $.expression),
        ')',
        field('body', $.statement),
      ),

    // Annotations

    _annotation: ($) => choice($.marker_annotation, $.annotation),

    marker_annotation: ($) => seq('@', field('name', $._name)),

    annotation: ($) =>
      seq('@', field('name', $._name), field('arguments', $.annotation_argument_list)),

    annotation_argument_list: ($) =>
      seq('(', field('arguments', choice($._element_value, commaSep($.element_value_pair))), ')'),

    element_value_pair: ($) =>
      choice(
        $.grit_metavariable,
        seq(
          field('key', $.identifier),
          alias($.assign, $._assign),
          field('value', $._element_value),
        ),
      ),

    _element_value: ($) =>
      prec(
        PREC.ELEMENT_VAL,
        choice($.expression, $.element_value_array_initializer, $._annotation),
      ),

    element_value_array_initializer: ($) =>
      seq('{', commaSep($._element_value), optional(','), '}'),

    // Declarations

    declaration: ($) =>
      prec(
        PREC.DECL,
        choice(
          $.module_declaration,
          $.package_declaration,
          $.import_declaration,
          $.class_declaration,
          $.record_declaration,
          $.interface_declaration,
          $.annotation_type_declaration,
          $.enum_declaration,
        ),
      ),

    module_declaration: ($) =>
      seq(
        field('annotations', repeat($._annotation)),
        optional('open'),
        'module',
        field('name', $._name),
        field('body', $.module_body),
      ),

    module_body: ($) => seq('{', field('directives', repeat($.module_directive)), '}'),

    module_directive: ($) =>
      choice(
        $.requires_module_directive,
        $.exports_module_directive,
        $.opens_module_directive,
        $.uses_module_directive,
        $.provides_module_directive,
      ),

    requires_module_directive: ($) =>
      seq(
        'requires',
        repeat(field('modifiers', $.requires_modifier)),
        field('module', $._name),
        ';',
      ),

    requires_modifier: ($) => choice('transitive', 'static'),

    exports_module_directive: ($) =>
      seq(
        'exports',
        field('package', $._name),
        optional(seq('to', field('modules', $._name), repeat(seq(',', field('modules', $._name))))),
        ';',
      ),

    opens_module_directive: ($) =>
      seq(
        'opens',
        field('package', $._name),
        optional(seq('to', field('modules', $._name), repeat(seq(',', field('modules', $._name))))),
        ';',
      ),

    uses_module_directive: ($) => seq('uses', field('type', $._name), ';'),

    provides_module_directive: ($) =>
      seq(
        'provides',
        field('provided', $._name),
        'with',
        field('provider', $._name),
        repeat(seq(',', field('provider', $._name))),
        ';',
      ),

    package_declaration: ($) =>
      seq(field('annotations', repeat($._annotation)), 'package', field('package', $._name), ';'),

    import_declaration: ($) =>
      seq(
        'import',
        optional('static'),
        field('package', $._name),
        optional(seq('.', $.asterisk)),
        ';',
      ),

    asterisk: ($) => '*',

    enum_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        'enum',
        field('name', $.identifier),
        field('interfaces', optional($.super_interfaces)),
        field('body', $.enum_body),
      ),

    enum_body: ($) =>
      seq(
        '{',
        commaSep(field('elements', $.enum_constant)),
        optional(','),
        optional(field('body', $.enum_body_declarations)),
        '}',
      ),

    enum_body_declarations: ($) =>
      seq(';', field('declarations', repeat($._class_body_declaration))),

    enum_constant: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        field('name', $.identifier),
        field('arguments', optional($.argument_list)),
        field('body', optional($.class_body)),
      ),

    class_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        'class',
        field('name', $.identifier),
        optional(field('type_parameters', $.type_parameters)),
        optional(field('superclass', $.superclass)),
        optional(field('interfaces', $.super_interfaces)),
        optional(field('permits', $.permits)),
        field('body', $.class_body),
      ),

    modifier: ($) =>
      choice(
        'public',
        'protected',
        'private',
        'abstract',
        'static',
        'final',
        'strictfp',
        'default',
        'synchronized',
        'native',
        'transient',
        'volatile',
        'sealed',
        'non-sealed',
      ),

    modifiers: ($) => repeat1(field('modifiers', choice($._annotation, $.modifier))),

    type_parameters: ($) => seq('<', commaSep1(field('types', $.type_parameter)), '>'),

    type_parameter: ($) =>
      seq(
        repeat(field('annotation', $._annotation)),
        field('bound', alias($.identifier, $.type_identifier)),
        optional(field('bound', $.type_bound)),
      ),

    type_bound: ($) =>
      seq('extends', field('types', $._type), repeat(seq('&', field('types', $._type)))),

    superclass: ($) => seq('extends', field('type', $._type)),

    super_interfaces: ($) => seq('implements', field('interfaces', $.type_list)),

    type_list: ($) => seq(field('types', $._type), repeat(seq(',', field('types', $._type)))),

    permits: ($) => seq('permits', field('permits', $.type_list)),

    class_body: ($) => seq('{', field('declarations', choice(repeat($._class_body_declaration), $.grit_metavariable)), '}'),

    _class_body_declaration: ($) =>
      choice(
        $.field_declaration,
        $.record_declaration,
        $.method_declaration,
        $.compact_constructor_declaration, // For records.
        $.class_declaration,
        $.interface_declaration,
        $.annotation_type_declaration,
        $.enum_declaration,
        $.block,
        $.static_initializer,
        $.constructor_declaration,
        ';',
      ),

    static_initializer: ($) => seq('static', field('initializer', $.block)),

    constructor_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        $._constructor_declarator,
        optional(field('throws', $.throws)),
        field('body', $.constructor_body),
      ),

    _constructor_declarator: ($) =>
      seq(
        field('type_parameters', optional($.type_parameters)),
        field('name', $.identifier),
        field('parameters', $.formal_parameters),
      ),

    constructor_body: ($) =>
      seq(
        '{',
        optional(field('super', $.explicit_constructor_invocation)),
        repeat(field('body', $.statement)),
        '}',
      ),

    explicit_constructor_invocation: ($) =>
      seq(
        choice(
          seq(
            field('type_arguments', optional($.type_arguments)),
            field('constructor', choice($.this, $.super)),
          ),
          seq(
            field('object', choice($.primary_expression)),
            '.',
            field('type_arguments', optional($.type_arguments)),
            field('constructor', $.super),
          ),
        ),
        field('arguments', $.argument_list),
        ';',
      ),

    _name: ($) => choice($.identifier, $._reserved_identifier, $.scoped_identifier),

    scoped_identifier: ($) => seq(field('scope', $._name), '.', field('name', $.identifier)),

    field_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        field('type', $._unannotated_type),
        $._variable_declarator_list,
        ';',
      ),

    record_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        'record',
        field('name', $.identifier),
        optional(field('type_parameters', $.type_parameters)),
        field('parameters', $.formal_parameters),
        optional(field('interfaces', $.super_interfaces)),
        field('body', $.class_body),
      ),

    annotation_type_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        '@interface',
        field('name', $.identifier),
        field('body', $.annotation_type_body),
      ),

    annotation_type_body: ($) =>
      seq(
        '{',
        repeat(
          field(
            'type',
            choice(
              $.annotation_type_element_declaration,
              $.constant_declaration,
              $.class_declaration,
              $.interface_declaration,
              $.enum_declaration,
              $.annotation_type_declaration,
            ),
          ),
        ),
        '}',
      ),

    annotation_type_element_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        field('type', $._unannotated_type),
        field('name', $.identifier),
        '(',
        ')',
        field('dimensions', optional($.dimensions)),
        optional($._default_value),
        ';',
      ),

    _default_value: ($) => seq('default', field('value', $._element_value)),

    interface_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        'interface',
        field('name', $.identifier),
        field('type_parameters', optional($.type_parameters)),
        optional(field('extends', $.extends_interfaces)),
        optional(field('permits', $.permits)),
        field('body', $.interface_body),
      ),

    extends_interfaces: ($) => seq('extends', field('interfaces', $.type_list)),

    interface_body: ($) =>
      seq(
        '{',
        repeat(
          field(
            'body',
            choice(
              $.constant_declaration,
              $.enum_declaration,
              $.method_declaration,
              $.class_declaration,
              $.interface_declaration,
              $.record_declaration,
              $.annotation_type_declaration,
              ';',
            ),
          ),
        ),
        '}',
      ),

    constant_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        field('type', $._unannotated_type),
        $._variable_declarator_list,
        ';',
      ),

    _variable_declarator_list: ($) => commaSep1(field('declarator', $.variable_declarator)),

    variable_declarator: ($) =>
      seq($._variable_declarator_id, optional(seq('=', field('value', $._variable_initializer)))),

    _variable_declarator_id: ($) =>
      seq(
        field('name', choice($.identifier, $._reserved_identifier)),
        field('dimensions', optional($.dimensions)),
      ),

    _variable_initializer: ($) => choice($.expression, $.array_initializer),

    array_initializer: ($) =>
      seq('{', commaSep(field('elements', $._variable_initializer)), optional(','), '}'),

    // Types

    _type: ($) => choice($._unannotated_type, $.annotated_type),

    _unannotated_type: ($) => choice($._simple_type, $.array_type),

    _simple_type: ($) =>
      choice(
        $.void_type,
        $.integral_type,
        $.floating_point_type,
        $.boolean_type,
        alias($.identifier, $.type_identifier),
        $.scoped_type_identifier,
        $.generic_type,
      ),

    annotated_type: ($) =>
      seq(field('annotations', repeat1($._annotation)), field('type', $._unannotated_type)),

    scoped_type_identifier: ($) =>
      seq(
        field(
          'scope',
          choice(alias($.identifier, $.type_identifier), $.scoped_type_identifier, $.generic_type),
        ),
        '.',
        field('annotations', repeat($._annotation)),
        field('type', alias($.identifier, $.type_identifier)),
      ),

    generic_type: ($) =>
      prec.dynamic(
        PREC.GENERIC,
        seq(
          field('type', choice(alias($.identifier, $.type_identifier), $.scoped_type_identifier)),
          field('arguments', $.type_arguments),
        ),
      ),

    array_type: ($) =>
      seq(field('element', $._unannotated_type), field('dimensions', $.dimensions)),

    integral_type: ($) => choice('byte', 'short', 'int', 'long', 'char'),

    floating_point_type: ($) => choice('float', 'double'),

    boolean_type: ($) => 'boolean',

    void_type: ($) => 'void',

    _method_header: ($) =>
      seq(
        optional(
          seq(
            field('type_parameters', $.type_parameters),
            field('annotations', repeat($._annotation)),
          ),
        ),
        field('type', $._unannotated_type),
        $._method_declarator,
        optional($.throws),
      ),

    _method_declarator: ($) =>
      seq(
        field('name', choice($.identifier, $._reserved_identifier)),
        field('parameters', $.formal_parameters),
        field('dimensions', optional($.dimensions)),
      ),

    formal_parameters: ($) =>
      seq(
        '(',
        optional(field('receiver', $.receiver_parameter)),
        commaSep(field('parameters', choice($.formal_parameter, $.spread_parameter))),
        ')',
      ),

    formal_parameter: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        field('type', $._unannotated_type),
        $._variable_declarator_id,
      ),

    receiver_parameter: ($) =>
      seq(
        field('annotations', repeat($._annotation)),
        field('type', $._unannotated_type),
        optional(seq(field('parameter', $.identifier), '.')),
        $.this,
      ),

    spread_parameter: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        field('type', $._unannotated_type),
        '...',
        field('parameter', $.variable_declarator),
      ),

    throws: ($) => seq('throws', commaSep1(field('errors', $._type))),

    local_variable_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        field('type', $._unannotated_type),
        $._variable_declarator_list,
        ';',
      ),

    method_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        $._method_header,
        choice(field('body', $.block), ';'),
      ),

    compact_constructor_declaration: ($) =>
      seq(
        optional(field('modifiers', $.modifiers)),
        field('name', $.identifier),
        field('body', $.block),
      ),

    _reserved_identifier: ($) => alias(choice('open', 'module', 'record'), $.identifier),

    this: ($) => 'this',

    super: ($) => 'super',

    // https://docs.oracle.com/javase/specs/jls/se8/html/jls-3.html#jls-IdentifierChars
    identifier: ($) => choice($._identifier, $.grit_metavariable),
    _identifier: ($) => /[\p{L}_$][\p{L}\p{Nd}_$]*/,

    // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
    comment: ($) => choice($.line_comment, $.block_comment),

    line_comment: ($) => token(prec(PREC.COMMENT, seq('//', /[^\n]*/))),

    block_comment: ($) => token(prec(PREC.COMMENT, seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/'))),

    grit_metavariable: ($) => token(prec(PREC.GRIT_METAVARIABLE, choice("µ...", /µ[a-zA-Z_][a-zA-Z0-9_]*/))),
  },
});

function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}

function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)));
}

function commaSep(rule) {
  return optional(commaSep1(rule));
}

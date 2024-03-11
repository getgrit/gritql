const
  PREC = {
    primary: 7,
    unary: 6,
    multiplicative: 5,
    additive: 4,
    comparative: 3,
    and: 2,
    or: 1,
    composite_literal: -1,
  },

  unicodeLetter = /\p{L}/,
  unicodeDigit = /[0-9]/,
  letter = choice(unicodeLetter, '_'),

  newline = '\n',
  terminator = choice(newline, ';'),

  hexDigit = /[0-9a-fA-F]/,
  octalDigit = /[0-7]/,
  decimalDigit = /[0-9]/,
  binaryDigit = /[01]/,

  hexDigits = seq(hexDigit, repeat(seq(optional('_'), hexDigit))),
  octalDigits = seq(octalDigit, repeat(seq(optional('_'), octalDigit))),
  decimalDigits = seq(decimalDigit, repeat(seq(optional('_'), decimalDigit))),
  binaryDigits = seq(binaryDigit, repeat(seq(optional('_'), binaryDigit))),

  hexLiteral = seq('0', choice('x', 'X'), optional('_'), hexDigits),
  octalLiteral = seq('0', optional(choice('o', 'O')), optional('_'), octalDigits),
  decimalLiteral = choice('0', seq(/[1-9]/, optional(seq(optional('_'), decimalDigits)))),
  binaryLiteral = seq('0', choice('b', 'B'), optional('_'), binaryDigits),

  intLiteral = choice(binaryLiteral, decimalLiteral, octalLiteral, hexLiteral),

  decimalExponent = seq(choice('e', 'E'), optional(choice('+', '-')), decimalDigits),
  decimalFloatLiteral = choice(
    seq(decimalDigits, '.', optional(decimalDigits), optional(decimalExponent)),
    seq(decimalDigits, decimalExponent),
    seq('.', decimalDigits, optional(decimalExponent)),
  ),

  hexExponent = seq(choice('p', 'P'), optional(choice('+', '-')), decimalDigits),
  hexMantissa = choice(
    seq(optional('_'), hexDigits, '.', optional(hexDigits)),
    seq(optional('_'), hexDigits),
    seq('.', hexDigits),
  ),
  hexFloatLiteral = seq('0', choice('x', 'X'), hexMantissa, hexExponent),

  floatLiteral = choice(decimalFloatLiteral, hexFloatLiteral),

  imaginaryLiteral = seq(choice(decimalDigits, intLiteral, floatLiteral), 'i')

module.exports = grammar({
  name: 'go',

  extras: $ => [
    $.comment,
    /\s/
  ],

  inline: $ => [
    $._type,
    $._type_identifier,
    $.field_identifier,
    $._package_identifier,
    $._top_level_declaration,
    $._string_literal,
  ],

  word: $ => $._identifier,

  conflicts: $ => [
    [$._simple_type, $._expression],
    [$.qualified_type, $._expression],
    [$.generic_type, $._expression],
    [$.generic_type, $._simple_type],
    [$.parameter_declaration, $.type_arguments],
    [$.parameter_declaration, $._simple_type, $._expression],
    [$.parameter_declaration, $.generic_type, $._expression],
    [$.parameter_declaration, $._expression],
    [$.func_literal, $.function_type],
    [$.function_type],
    [$.parameter_declaration, $._simple_type],
    
    // Grit conflicts
    [$.identifier, $.import_declaration]
  ],

  supertypes: $ => [
    $._expression,
    $._type,
    $._simple_type,
    $._statement,
    $._simple_statement,
  ],

  rules: {
    source_file: $ => repeat(field('statements', choice(
      // Unlike a Go compiler, we accept statements at top-level to enable
      // parsing of partial code snippets in documentation (see #63).
      seq($._statement, terminator),
      seq($._top_level_declaration, optional(terminator)),
    ))),

    _top_level_declaration: $ => choice(
      $.package_clause,
      $.function_declaration,
      $.method_declaration,
      $.import_declaration
    ),

    package_clause: $ => seq(
      'package',
      field('package', $._package_identifier)
    ),

    import_declaration: $ => seq(
      'import',
      field('imports', choice(
        $.import_spec,
        $.import_spec_list,
        $.grit_metavariable,
      ))
    ),

    import_spec: $ => seq(
      optional(field('name', choice(
        $.dot,
        $.blank_identifier,
        $._package_identifier
      ))),
      field('path', $._string_literal)
    ),
    dot: _$ => '.',
    blank_identifier: _$ => '_',

    import_spec_list: $ => seq(
      '(',
          choice(
            repeat(seq(
              field('imports', $.import_spec),
              terminator
            )),
            field('imports', $.grit_metavariable),
        ),
      ')'
    ),

    _declaration: $ => choice(
      $.const_declaration,
      $.type_declaration,
      $.var_declaration
    ),

    const_declaration: $ => seq(
      'const',
      choice(
        field('declaration', $.const_spec),
        seq(
          '(',
          repeat(seq(field('declaration', $.const_spec), terminator)),
          ')'
        )
      )
    ),

    const_spec: $ => prec.left(seq(
      field('name', commaSep1($.identifier)),
      optional(seq(
        optional(field('type', $._type)),
        '=',
        field('value', $.expression_list)
      ))
    )),

    var_declaration: $ => seq(
      'var',
      choice(
        field('declaration', $.var_spec),
        seq(
          '(',
          repeat(seq(field('declaration', $.var_spec), terminator)),
          ')'
        )
      )
    ),

    var_spec: $ => seq(
      field('name', commaSep1($.identifier)),
      choice(
        seq(
          field('type', $._type),
          optional(seq('=', field('value', $.expression_list)))
        ),
        seq('=', field('value', $.expression_list))
      )
    ),

    function_declaration: $ => prec.right(1, seq(
      'func',
      field('name', $.identifier),
      field('type_parameters', optional($.type_parameter_list)),
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type))),
      field('body', optional($.block))
    )),

    method_declaration: $ => prec.right(1, seq(
      'func',
      field('receiver', $.parameter_list),
      field('name', $.field_identifier),
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type))),
      field('body', optional($.block))
    )),

    type_parameter_list: $ => seq(
      '[',
      commaSep1(field('parameters', $.parameter_declaration)),
      optional(','),
      ']'
    ),

    parameter_list: $ => seq(
      '(',
      optional(seq(
        commaSep(field('parameters', choice($.parameter_declaration, $.variadic_parameter_declaration))),
        optional(',')
      )),
      ')'
    ),

    parameter_declaration: $ => seq(
      commaSep(field('name', $.identifier)),
      field('type', $._type)
    ),

    variadic_parameter_declaration: $ => seq(
      field('name', optional($.identifier)),
      '...',
      field('type', $._type)
    ),

    type_alias: $ => seq(
      field('name', $._type_identifier),
      '=',
      field('type', $._type)
    ),

    type_declaration: $ => seq(
      'type',
      choice(
        field('type', $.type_spec),
        field('type', $.type_alias),
        seq(
          '(',
          repeat(seq(choice(field('type', $.type_spec), field('type', $.type_alias)), terminator)),
          ')'
        )
      )
    ),

    type_spec: $ => seq(
      field('name', $._type_identifier),
      field('type_parameters', optional($.type_parameter_list)),
      field('type', $._type)
    ),

    field_name_list: $ => commaSep1(field('names', $.field_identifier)),

    expression_list: $ => commaSep1(field('expressions', $._expression)),

    _type: $ => choice(
      $._simple_type,
      $.parenthesized_type
    ),

    parenthesized_type: $ => seq('(', $._type, ')'),

    _simple_type: $ => choice(
      prec.dynamic(-1, $._type_identifier),
      $.generic_type,
      $.qualified_type,
      $.pointer_type,
      $.struct_type,
      $.interface_type,
      $.array_type,
      $.slice_type,
      $.map_type,
      $.channel_type,
      $.function_type
    ),

    generic_type: $ => seq(
      field('type', choice($._type_identifier, $.qualified_type)),
      field('type_arguments', $.type_arguments),
    ),

    type_arguments: $ => prec.dynamic(2, seq(
      '[',
      commaSep1(field('types', $._type)),
      optional(','),
      ']'
    )),

    pointer_type: $ => prec(PREC.unary, seq('*', field('types', $._type))),

    array_type: $ => seq(
      '[',
      field('length', $._expression),
      ']',
      field('element', $._type)
    ),

    implicit_length_array_type: $ => seq(
      '[',
      '...',
      ']',
      field('element', $._type)
    ),

    slice_type: $ => seq(
      '[',
      ']',
      field('element', $._type)
    ),

    struct_type: $ => seq(
      'struct',
      field('elements', $.field_declaration_list)
    ),

    field_declaration_list: $ => seq(
      '{',
      optional(seq(
        field('fields', $.field_declaration),
        repeat(seq(terminator, field('fields', $.field_declaration))),
        optional(terminator)
      )),
      '}'
    ),

    field_declaration: $ => seq(
      choice(
        seq(
          commaSep1(field('name', $.field_identifier)),
          field('type', $._type)
        ),
        seq(
          optional('*'),
          field('type', choice(
            $._type_identifier,
            $.qualified_type
          ))
        )
      ),
      field('tag', optional($._string_literal))
    ),

    interface_type: $ => seq(
      'interface',
      '{',
      optional(seq(
        field('interface', $._interface_body),
        repeat(seq(terminator, field('interface', $._interface_body))),
        optional(terminator)
      )),
      '}'
    ),

    _interface_body: $ => choice(
      $.method_spec, $._interface_type_name, $.constraint_elem, $.struct_elem
    ),

    _interface_type_name: $ => choice($._type_identifier, $.qualified_type),

    constraint_elem: $ => seq(
      field('terms', $.constraint_term),
      repeat(seq('|', field('terms', $.constraint_term)))
    ),

    constraint_term: $ => prec(-1, seq(
      optional('~'),
      field('type', $._type_identifier),
    )),

    struct_elem: $ => seq(
      field('terms', $.struct_term),
      repeat(seq('|', field('terms', $.struct_term)))
    ),

    struct_term: $ => prec(-1, seq(
      optional(choice('~', '*')),
      field('type', $.struct_type)
    )),

    method_spec: $ => seq(
      field('name', $.field_identifier),
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type)))
    ),

    map_type: $ => seq(
      'map',
      '[',
      field('key', $._type),
      ']',
      field('value', $._type)
    ),

    channel_type: $ => choice(
      seq('chan', field('value', $._type)),
      seq('chan', '<-', field('value', $._type)),
      prec(PREC.unary, seq('<-', 'chan', field('value', $._type)))
    ),

    function_type: $ => seq(
      'func',
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type)))
    ),

    block: $ => seq(
      '{',
      optional($._statement_list),
      '}'
    ),

    _statement_list: $ => choice(
      seq(
        field('statements', $._statement),
        repeat(seq(terminator, field('statements', $._statement))),
        optional(seq(
          terminator,
          optional(field('label', alias($.empty_labeled_statement, $.labeled_statement)))
        ))
      ),
      field('label', alias($.empty_labeled_statement, $.labeled_statement))
    ),

    _statement: $ => choice(
      $._declaration,
      $._simple_statement,
      $.return_statement,
      $.go_statement,
      $.defer_statement,
      $.if_statement,
      $.for_statement,
      $.expression_switch_statement,
      $.type_switch_statement,
      $.select_statement,
      $.labeled_statement,
      $.fallthrough_statement,
      $.break_statement,
      $.continue_statement,
      $.goto_statement,
      $.block,
      $.empty_statement
    ),

    empty_statement: _$ => ';',

    _simple_statement: $ => choice(
      $._expression,
      $.send_statement,
      $.inc_statement,
      $.dec_statement,
      $.assignment_statement,
      $.short_var_declaration
    ),

    send_statement: $ => seq(
      field('channel', $._expression),
      '<-',
      field('value', $._expression)
    ),

    receive_statement: $ => seq(
      optional(seq(
        field('left', $.expression_list),
        choice('=', ':=')
      )),
      field('right', $._expression)
    ),

    inc_statement: $ => seq(
      field('expression', $._expression),
      '++'
    ),

    dec_statement: $ => seq(
      field('expression', $._expression),
      '--'
    ),

    // multiplicative_operators
    star: _$ => '*',
    divide: _$ => '/',
    mod: _$ => '%',
    shift_left: _$ => '<<',
    shift_right: _$ => '>>',
    bitwise_and: _$ => '&',
    bitwise_and_not: _$ => '&^',

    // additive_operators
    plus: _$ => '+',
    minus: _$ => '-',
    bitwise_or: _$ => '|',
    bitwise_not: _$ => '^',

    // comparative_operators
    equal: _$ => '==',
    not_equal: _$ => '!=',
    less_than: _$ => '<',
    less_than_or_equal: _$ => '<=',
    greater_than: _$ => '>',
    greater_than_or_equal: _$ => '>=',

    // assignment_operators
    times_equal: _$ => '*=',
    divide_equal: _$ => '/=',
    mod_equal: _$ => '%=',
    shift_left_equal: _$ => '<<=',
    shift_right_equal: _$ => '>>=',
    and_equal: _$ => '&=',
    bitwise_and_not_equal: _$ => '&^=',
    plus_equal: _$ => '+=',
    minus_equal: _$ => '-=',
    or_equal: _$ => '|=',
    bitwise_not_equal: _$ => '^=',
    assign: _$ => '=',

    // logical_operators
    and: _$ => '&&',
    or: _$ => '||',

    assignment_statement: $ => seq(
      field('left', $.expression_list),
      field('operator', choice(
        $.times_equal,
        $.divide_equal,
        $.mod_equal,
        $.shift_left_equal,
        $.shift_right_equal,
        $.and_equal,
        $.bitwise_and_not_equal,
        $.plus_equal,
        $.minus_equal,
        $.or_equal,
        $.bitwise_not_equal,
        $.assign)
      ),
      field('right', $.expression_list)
    ),

    short_var_declaration: $ => seq(
      // TODO: this should really only allow identifier lists, but that causes
      // conflicts between identifiers as expressions vs identifiers here.
      field('left', $.expression_list),
      ':=',
      field('right', $.expression_list)
    ),

    labeled_statement: $ => seq(
      field('label', alias($.identifier, $.label_name)),
      ':',
      $._statement
    ),

    empty_labeled_statement: $ => seq(
      field('label', alias($.identifier, $.label_name)),
      ':'
    ),

    // This is a hack to prevent `fallthrough_statement` from being parsed as
    // a single token. For consistency with `break_statement` etc it should
    // be parsed as a parent node that *contains* a `fallthrough` token.
    fallthrough_statement: _$ => prec.left('fallthrough'),

    break_statement: $ => seq('break', field('label', optional(alias($.identifier, $.label_name)))),

    continue_statement: $ => seq('continue', field('label', optional(alias($.identifier, $.label_name)))),

    goto_statement: $ => seq('goto', field('label', alias($.identifier, $.label_name))),

    return_statement: $ => seq('return', field('value', optional($.expression_list))),

    go_statement: $ => seq('go', field('expresion', $._expression)),

    defer_statement: $ => seq('defer', field('expresion', $._expression)),

    if_statement: $ => seq(
      'if',
      optional(seq(
        field('initializer', $._simple_statement),
        ';'
      )),
      field('condition', $._expression),
      field('consequence', $.block),
      optional(seq(
        'else',
        field('alternative', choice($.block, $.if_statement))
      ))
    ),

    for_statement: $ => seq(
      'for',
      field('iterator', optional(choice($._expression, $.for_clause, $.range_clause))),
      field('body', $.block)
    ),

    for_clause: $ => seq(
      field('initializer', optional($._simple_statement)),
      ';',
      field('condition', optional($._expression)),
      ';',
      field('update', optional($._simple_statement))
    ),

    range_clause: $ => seq(
      optional(seq(
        field('left', $.expression_list),
        choice('=', ':=')
      )),
      'range',
      field('right', $._expression)
    ),

    expression_switch_statement: $ => seq(
      'switch',
      optional(seq(
        field('initializer', $._simple_statement),
        ';'
      )),
      field('value', optional($._expression)),
      '{',
      repeat(field('cases', choice($.expression_case, $.default_case))),
      '}'
    ),

    expression_case: $ => seq(
      'case',
      field('value', $.expression_list),
      ':',
      field('expression', optional($._statement_list))
    ),

    default_case: $ => seq(
      'default',
      ':',
      field('expression', optional($._statement_list))
    ),

    type_switch_statement: $ => seq(
      'switch',
      field('header', $._type_switch_header),
      '{',
      repeat(field('cases', choice($.type_case, $.default_case))),
      '}'
    ),

    _type_switch_header: $ => seq(
      optional(seq(
        field('initializer', $._simple_statement),
        ';'
      )),
      optional(seq(field('alias', $.expression_list), ':=')),
      field('value', $._expression),
      '.',
      '(',
      'type',
      ')'
    ),

    type_case: $ => seq(
      'case',
      field('type', commaSep1($._type)),
      ':',
      field('expression', optional($._statement_list))
    ),

    select_statement: $ => seq(
      'select',
      '{',
      repeat(field('cases', choice($.communication_case, $.default_case))),
      '}'
    ),

    communication_case: $ => seq(
      'case',
      field('communication', choice($.send_statement, $.receive_statement)),
      ':',
      field('expression', optional($._statement_list))
    ),

    _expression: $ => choice(
      $.unary_expression,
      $.binary_expression,
      $.selector_expression,
      $.index_expression,
      $.slice_expression,
      $.call_expression,
      $.type_assertion_expression,
      $.type_conversion_expression,
      $.identifier,
      alias(choice('new', 'make'), $.identifier),
      $.composite_literal,
      $.func_literal,
      $._string_literal,
      $.int_literal,
      $.float_literal,
      $.imaginary_literal,
      $.rune_literal,
      $.nil,
      $.true,
      $.false,
      $.iota,
      $.parenthesized_expression
    ),

    parenthesized_expression: $ => seq(
      '(',
      field('expression', $._expression),
      ')'
    ),

    call_expression: $ => prec(PREC.primary, choice(
      seq(
        field('function', alias(choice('new', 'make'), $.identifier)),
        field('arguments', alias($.special_argument_list, $.argument_list))
      ),
      seq(
        field('function', $._expression),
        field('type_arguments', optional($.type_arguments)),
        field('arguments', $.argument_list)
      )
    )),

    variadic_argument: $ => prec.right(seq(
      field('arguments', $._expression),
      '...'
    )),

    special_argument_list: $ => seq(
      '(',
      field('type', $._type),
      repeat(seq(',', field('arguments', $._expression))),
      optional(','),
      ')'
    ),

    argument_list: $ => seq(
      '(',
      optional(seq(
        field('arguments', choice($._expression, $.variadic_argument)),
        repeat(seq(',', field('arguments', choice($._expression, $.variadic_argument)))),
        optional(',')
      )),
      ')'
    ),

    selector_expression: $ => prec(PREC.primary, seq(
      field('operand', $._expression),
      '.',
      field('field', $.field_identifier)
    )),

    index_expression: $ => prec(PREC.primary, seq(
      field('operand', $._expression),
      '[',
      field('index', $._expression),
      ']'
    )),

    slice_expression: $ => prec(PREC.primary, seq(
      field('operand', $._expression),
      '[',
      choice(
        seq(
          field('start', optional($._expression)),
          ':',
          field('end', optional($._expression))
        ),
        seq(
          field('start', optional($._expression)),
          ':',
          field('end', $._expression),
          ':',
          field('capacity', $._expression)
        )
      ),
      ']'
    )),

    type_assertion_expression: $ => prec(PREC.primary, seq(
      field('operand', $._expression),
      '.',
      '(',
      field('type', $._type),
      ')'
    )),

    type_conversion_expression: $ => prec.dynamic(-1, seq(
      field('type', $._type),
      '(',
      field('operand', $._expression),
      optional(','),
      ')'
    )),

    composite_literal: $ => prec(PREC.composite_literal, seq(
      field('type', choice(
        $.map_type,
        $.slice_type,
        $.array_type,
        $.implicit_length_array_type,
        $.struct_type,
        $._type_identifier,
        $.generic_type,
        $.qualified_type
      )),
      field('body', $.literal_value)
    )),

    literal_value: $ => seq(
      '{',
      optional(
        seq(
          commaSep(field('element', choice($._literal_element, $.keyed_element))),
          optional(','))),
      '}'
    ),

    _literal_element: $ => choice($._expression, $.literal_value),

    // In T{k: v}, the key k may be:
    // - any expression (when T is a map, slice or array),
    // - a field identifier (when T is a struct), or
    // - a _literal_element (when T is an array).
    // The first two cases cannot be distinguished without type information.
    keyed_element: $ => seq(field('key', $._literal_element), ':', field('value', $._literal_element)),

    func_literal: $ => seq(
      'func',
      field('parameters', $.parameter_list),
      field('result', optional(choice($.parameter_list, $._simple_type))),
      field('body', $.block)
    ),

    not: _$ => '!',
    receive: _$ => '<-',

    unary_expression: $ => prec(PREC.unary, seq(
      field('operator', choice($.plus, $.minus, $.not, $.bitwise_not, $.star, $.bitwise_and, $.receive)),
      field('operand', $._expression)
    )),

    binary_expression: $ => {
      const table = [
        [PREC.multiplicative, choice($.star,
          $.divide,
          $.mod,
          $.shift_left,
          $.shift_right,
          $.bitwise_and,
          $.bitwise_and_not)],
        [PREC.additive, choice($.plus,
          $.minus,
          $.bitwise_or,
          $.bitwise_not)],
        [PREC.comparative, choice($.equal,
          $.not_equal,
          $.less_than,
          $.less_than_or_equal,
          $.greater_than,
          $.greater_than_or_equal)],
        [PREC.and, $.and],
        [PREC.or, $.or],
      ];

      return choice(...table.map(([precedence, operator]) =>
        prec.left(precedence, seq(
          field('left', $._expression),
          field('operator', operator),
          field('right', $._expression)
        ))
      ));
    },

    qualified_type: $ => seq(
      field('package', $._package_identifier),
      '.',
      field('name', $._type_identifier)
    ),

    _identifier: _$ => token(seq(
      letter,
      repeat(choice(letter, unicodeDigit))
    )),

    identifier: $ => choice($._identifier, $.grit_metavariable),

    _type_identifier: $ => alias($.identifier, $.type_identifier),
    field_identifier: $ => alias($.identifier, $.field_identifier),
    _package_identifier: $ => alias($.identifier, $.package_identifier),

    _string_literal: $ => choice(
      $.raw_string_literal,
      $.interpreted_string_literal
    ),

    raw_string_literal: _$ => token(seq(
      '`',
      repeat(/[^`]/),
      '`'
    )),

    interpreted_string_literal: $ => seq(
      '"',
      field('fragment',
        optional(choice(
          $.string_literal_fragment,
          $.grit_metavariable
        )),
      ),
      token.immediate('"')
    ),

    string_literal_fragment: $ => repeat1(choice(
      $._interpreted_string_literal_basic_content,
      $.escape_sequence,
    )),
  
    _interpreted_string_literal_basic_content: _$ => token.immediate(prec(1, /[^"\n\\]+/)),

    escape_sequence: _$ => token.immediate(seq(
      '\\',
      choice(
        /[^xuU]/,
        /\d{2,3}/,
        /x[0-9a-fA-F]{2,}/,
        /u[0-9a-fA-F]{4}/,
        /U[0-9a-fA-F]{8}/
      )
    )),

    int_literal: _$ => token(intLiteral),

    float_literal: _$ => token(floatLiteral),

    imaginary_literal: _$ => token(imaginaryLiteral),

    rune_literal: _$ => token(seq(
      "'",
      choice(
        /[^'\\]/,
        seq(
          '\\',
          choice(
            seq('x', hexDigit, hexDigit),
            seq(octalDigit, octalDigit, octalDigit),
            seq('u', hexDigit, hexDigit, hexDigit, hexDigit),
            seq('U', hexDigit, hexDigit, hexDigit, hexDigit, hexDigit, hexDigit, hexDigit, hexDigit),
            seq(choice('a', 'b', 'f', 'n', 'r', 't', 'v', '\\', "'", '"'))
          )
        )
      ),
      "'"
    )),

    nil: _$ => 'nil',
    true: _$ => 'true',
    false: _$ => 'false',
    iota: _$ => 'iota',

    // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
    comment: _$ => token(choice(
      seq('//', /.*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/'
      )
    )),
    grit_metavariable: (_$) => token(prec(100, choice("µ...", /µ[a-zA-Z_][a-zA-Z0-9_]*/))),
  }
})

function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)))
}

function commaSep(rule) {
  return optional(commaSep1(rule))
}

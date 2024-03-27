module.exports = grammar({
  name: 'javascript',

  externals: $ => [
    $._automatic_semicolon,
    $._template_chars,
    $._ternary_qmark,
    $.html_comment,
    '||',
    // We use escape sequence and regex pattern to tell the scanner if we're currently inside a string or template string, in which case
    // it should NOT parse html comments.
    $.escape_sequence,
    $.regex_pattern,
  ],

  extras: $ => [
    $.comment,
    $.html_comment,
    /[\s\p{Zs}\uFEFF\u2028\u2029\u2060\u200B]/,
  ],

  supertypes: ($) => [
    $.statement,
    $.declaration,
    $.expression,
    $.primary_expression,
    $.pattern,
  ],

  inline: ($) => [
    $._call_signature,
    $._formal_parameter,
    $.statement,
    $._expressions,
    $._semicolon,
    $._identifier,
    $._reserved_identifier,
    $._jsx_attribute,
    $._jsx_element_name,
    $._jsx_child,
    $._jsx_element,
    $._jsx_attribute_name,
    $._jsx_attribute_value,
    $._jsx_identifier,
    $._lhs_expression,
  ],

  precedences: ($) => [
    [
      'member',
      'call',
      $.update_expression,
      'unary_void',
      'binary_exp',
      'binary_times',
      'binary_plus',
      'binary_shift',
      'binary_compare',
      'binary_relation',
      'binary_equality',
      'bitwise_and',
      'bitwise_xor',
      'bitwise_or',
      'logical_and',
      'logical_or',
      'ternary',
      $.sequence_expression,
      $.arrow_function,
    ],
    ['assign', $.primary_expression],
    ['member', 'new', 'call', $.expression],
    ['declaration', 'literal'],
    [$.primary_expression, $.statement_block, 'object'],
    [$.import_statement, $.import],
    [$.export_statement, $.primary_expression],
    [$.lexical_declaration, $.primary_expression],
  ],

  conflicts: ($) => [
    [$.primary_expression, $._property_name],
    [$.primary_expression, $._property_name, $.arrow_function],
    [$.primary_expression, $.arrow_function],
    [$.primary_expression, $.method_definition],
    [$.primary_expression, $.rest_pattern],
    [$.primary_expression, $.pattern],
    [$.primary_expression, $._for_header],
    [$.array, $.array_pattern],
    [$.object, $.object_pattern],
    [$.assignment_expression, $.pattern],
    [$.assignment_expression, $.object_assignment_pattern],
    [$.labeled_statement, $._property_name],
    [$.computed_property_name, $.array],
    [$.binary_expression, $._initializer],
    // GRIT METAVARIABLE CONFLICTS
    [$.declaration, $.identifier],
    [$.declaration, $.expression_statement, $.identifier],
    [$.expression_statement, $.identifier],
    [$.declaration, $._property_name, $.identifier],
    [$._property_name, $.identifier],
    [$.pattern, $.identifier],
    [$.parenthesized_expression, $.pattern, $.identifier],
    [$.return_statement, $.identifier],
    [$.throw_statement, $.identifier],
    [$.parenthesized_expression, $.identifier],
    [$.template_substitution, $.identifier],
    [$.subscript_expression, $.identifier],
    [$.switch_case, $.identifier],
    [$._for_header, $.identifier],
    [$.for_statement, $.identifier],
    [$.named_imports, $.identifier],
    // GRIT OTHER CONFLICTS
  ],

  word: ($) => $._primitive_identifier,

  rules: {
    program: ($) =>
      seq(
        field('hash_bang', optional($.hash_bang_line)),
        field('statements', repeat($.statement))
      ),

    hash_bang_line: ($) => /#!.*/,

    //
    // Export declarations
    //

    export_statement: $ => choice(
      seq(
        'export',
        choice(
          seq('*', $._from_clause),
          seq(field('export', $.namespace_export), $._from_clause),
          seq(field('export', $.export_clause), $._from_clause),
          field('export', $.export_clause),
        ),
        $._semicolon,
      ),
      seq(
        repeat(field('decorator', $.decorator)),
        'export',
        choice(
          field('declaration', $.declaration),
          seq(
            field('default', $.default),
            choice(
              field('declaration', $.declaration),
              seq(
                field('declaration', $.expression),
                $._semicolon,
              ),
            ),
          ),
        ),
      ),
    ),

    default: _ => 'default',

    namespace_export: $ => seq(
      '*', 'as', field('module', $._module_export_name),
    ),

    export_clause: $ => seq(
      '{',
      field('specifiers', commaSep($.export_specifier)),
      optional(','),
      '}',
    ),

    export_specifier: $ => seq(
      field('name', $._module_export_name),
      optional(seq(
        'as',
        field('alias', $._module_export_name),
      )),
    ),

    _module_export_name: $ => choice(
      $.identifier,
      $.string,
    ),

    declaration: $ => choice(
      $.function_declaration,
      $.generator_function_declaration,
      $.class_declaration,
      $.lexical_declaration,
      $.variable_declaration,
      $.grit_metavariable,
    ),

    //
    // Import declarations
    //

    import: _ => token('import'),

    import_statement: $ => seq(
      'import',
      choice(
        seq(field('import', $.import_clause), $._from_clause),
        field('source', choice($.string, $.grit_metavariable)),
      ),
      optional(field('attribute', $.import_attribute)),
      $._semicolon,
    ),

    import_clause: $ => choice(
      field('name', $.namespace_import),
      field('name', $.named_imports),
      seq(
        field('default', $.identifier),
        optional(seq(
          ',',
          field('name',
            choice(
              $.namespace_import,
              $.named_imports,
            ),
          ),
        )),
      ),
    ),

    _from_clause: $ => seq(
      'from', field('source', choice($.string, $.grit_metavariable)),
    ),

    namespace_import: $ => seq(
      '*', 'as', field('namespace', $.identifier),
    ),

    named_imports: $ => choice($.grit_metavariable,
      seq(
        '{',
        field('imports', commaSep($.import_specifier)),
        optional(','),
        '}',
      ),
    ),

    // aliased name?
    import_specifier: $ => choice(
      field('name', $.identifier),
      seq(
        field('name', $._module_export_name),
        'as',
        field('alias', $.identifier),
      ),
    ),

    import_attribute: $ => seq('with', $.object),

    //
    // Statements
    //

    statement: $ => choice(
      $.export_statement,
      $.import_statement,
      $.debugger_statement,
      $.expression_statement,
      $.declaration,
      $.statement_block,

      $.if_statement,
      $.switch_statement,
      $.for_statement,
      $.for_in_statement,
      $.while_statement,
      $.do_statement,
      $.try_statement,
      $.with_statement,

      $.break_statement,
      $.continue_statement,
      $.return_statement,
      $.throw_statement,
      $.empty_statement,
      $.labeled_statement,
    ),

    expression_statement: $ => seq(
      field('expression', $._expressions),
      $._semicolon,
    ),

    variable_declaration: $ => seq(
      $.var,
      field('declarations', commaSep1($.variable_declarator)),
      $._semicolon,
    ),

    lexical_declaration: $ => seq(
      field('kind', choice($.let, $.const)),
      field('declarations', commaSep1($.variable_declarator)),
      $._semicolon,
    ),

    variable_declarator: $ => seq(
      field('name', choice($.identifier, $._destructuring_pattern)),
      optional($._initializer),
    ),

    statement_block: $ => prec.right(seq(
      '{',
      field('statements', repeat($.statement)),
      '}',
      optional($._automatic_semicolon),
    )),

    else_clause: $ => seq('else', field('else', $.statement)),

    if_statement: $ => prec.right(seq(
      'if',
      field('condition', $.parenthesized_expression),
      field('consequence', $.statement),
      optional(field('alternative', $.else_clause)),
    )),

    switch_statement: $ => seq(
      'switch',
      field('value', $.parenthesized_expression),
      field('body', $.switch_body),
    ),

    for_statement: $ => seq(
      'for',
      '(',
      field('initializer', choice(
        $.lexical_declaration,
        $.variable_declaration,
        $.expression_statement,
        $.empty_statement,
      )),
      field('condition', choice(
        $.expression_statement,
        $.empty_statement,
      )),
      field('increment', optional($._expressions)),
      ')',
      field('body', $.statement),
    ),

    for_in_statement: $ => seq(
      'for',
      optional('await'),
      $._for_header,
      field('body', $.statement),
    ),

    _for_header: $ => seq(
      '(',
      choice(
        field('left', choice(
          $._lhs_expression,
          $.parenthesized_expression,
        )),
        seq(
          field('kind', $.var),
          field('left', choice(
            $.identifier,
            $._destructuring_pattern,
          )),
          optional($._initializer),
        ),
        seq(
          field('kind', choice($.let, $.const)),
          field('left', choice(
            $.identifier,
            $._destructuring_pattern,
          )),
        ),
      ),
      field('operator', choice($.in, $.of)),
      field('right', $._expressions),
      ')',
    ),

    while_statement: $ => seq(
      'while',
      field('condition', $.parenthesized_expression),
      field('body', $.statement),
    ),

    do_statement: $ => prec.right(seq(
      'do',
      field('body', $.statement),
      'while',
      field('condition', $.parenthesized_expression),
      optional($._semicolon),
    )),

    try_statement: $ => seq(
      'try',
      field('body', $.statement_block),
      optional(field('handler', $.catch_clause)),
      optional(field('finalizer', $.finally_clause)),
    ),

    with_statement: $ => seq(
      'with',
      field('object', $.parenthesized_expression),
      field('body', $.statement),
    ),

    break_statement: $ => seq(
      'break',
      field('label', optional(alias($.identifier, $.statement_identifier))),
      $._semicolon,
    ),

    continue_statement: $ => seq(
      'continue',
      field('label', optional(alias($.identifier, $.statement_identifier))),
      $._semicolon,
    ),

    debugger_statement: $ => seq(
      'debugger',
      $._semicolon,
    ),

    return_statement: $ => seq(
      'return',
      field('expressions', optional($._expressions)),
      $._semicolon,
    ),

    throw_statement: $ => seq(
      'throw',
      field('expressions', $._expressions),
      $._semicolon,
    ),

    empty_statement: _ => ';',

    labeled_statement: $ => prec.dynamic(-1, seq(
      field('label', alias(choice($.identifier, $._reserved_identifier), $.statement_identifier)),
      ':',
      field('body', $.statement),
    )),

    //
    // Statement components
    //

    switch_body: $ => seq(
      '{',
      // field(cases)
      repeat(choice($.switch_case, $.switch_default)),
      '}',
    ),

    switch_case: $ => seq(
      'case',
      field('value', $._expressions),
      ':',
      field('body', repeat($.statement)),
    ),

    switch_default: $ => seq(
      'default',
      ':',
      field('body', repeat($.statement)),
    ),

    catch_clause: $ => seq(
      'catch',
      optional(seq('(', field('parameter', choice($.identifier, $._destructuring_pattern)), ')')),
      field('body', $.statement_block),
    ),

    finally_clause: $ => seq(
      'finally',
      field('body', $.statement_block),
    ),

    parenthesized_expression: $ => seq(
      '(',
      field('expressions', $._expressions),
      ')',
    ),

    //
    // Expressions
    //
    _expressions: $ => choice(
      $.expression,
      $.sequence_expression,
      $.grit_metavariable,
    ),

    expression: $ => choice(
      $.primary_expression,
      $.glimmer_template,
      $._jsx_element,
      $.assignment_expression,
      $.augmented_assignment_expression,
      $.await_expression,
      $.unary_expression,
      $.binary_expression,
      $.ternary_expression,
      $.update_expression,
      $.new_expression,
      $.yield_expression,
    ),

    primary_expression: $ => choice(
      $.subscript_expression,
      $.member_expression,
      $.parenthesized_expression,
      $._identifier,
      alias($._reserved_identifier, $.identifier),
      $.this,
      $.super,
      $.number,
      $.string,
      $.template_string,
      $.regex,
      $.true,
      $.false,
      $.null,
      $.object,
      $.array,
      $.function,
      $.arrow_function,
      $.generator_function,
      $.class,
      $.meta_property,
      $.call_expression,
    ),

    yield_expression: $ => prec.right(seq(
      'yield',
      choice(
        seq('*', field('expression', $.expression)),
        optional(field('expression', $.expression)),
      ))),

    object: $ => prec('object', seq(
      '{',
      field(
        'properties',
        commaSep(optional(choice(
          $.pair,
          $.spread_element,
          $.method_definition,
          alias(
            choice($.identifier, $._reserved_identifier),
            $.shorthand_property_identifier,
          ),
        ))),
      ),
      '}',
    )),

    object_pattern: $ => prec('object', seq(
      '{',
      field(
        'properties',
        commaSep(optional(choice(
          $.pair_pattern,
          $.rest_pattern,
          $.object_assignment_pattern,
          alias(
            choice($.identifier, $._reserved_identifier),
            $.shorthand_property_identifier_pattern,
          ),
        ))),
      ),
      '}',
    )),

    assignment_pattern: $ => seq(
      field('left', $.pattern),
      '=',
      field('right', $.expression),
    ),

    object_assignment_pattern: $ => seq(
      field('left', choice(
        alias(choice($._reserved_identifier, $.identifier), $.shorthand_property_identifier_pattern),
        $._destructuring_pattern,
      )),
      '=',
      field('right', $.expression),
    ),

    array: $ => seq(
      '[',
      field('elements',
        commaSep(optional(choice(
          $.expression,
          $.spread_element,
        ))),
      ),
      ']',
    ),

    array_pattern: $ => seq(
      '[',
      field('elements',
        commaSep(optional(choice(
          $.pattern,
          $.assignment_pattern,
        ))),
      ),
      ']',
    ),

    glimmer_template: $ => choice(
      seq(
        field('open_tag', $.glimmer_opening_tag),
        field('content', repeat($._glimmer_template_content)),
        field('close_tag', $.glimmer_closing_tag),
      ),
      // empty template has no content
      // <template></template>
      seq(
        field('open_tag', $.glimmer_opening_tag),
        field('close_tag', $.glimmer_closing_tag),
      ),
    ),

    _glimmer_template_content: _ => /.{1,}/,
    glimmer_opening_tag: _ => seq('<template>'),
    glimmer_closing_tag: _ => seq('</template>'),

    _jsx_element: $ => choice($.jsx_element, $.jsx_self_closing_element),

    jsx_element: $ => seq(
      field('open_tag', $.jsx_opening_element),
      field('children', repeat($._jsx_child)),
      field('close_tag', $.jsx_closing_element),
    ),

    // Should not contain new lines and should not start or end with a space
    jsx_text: _ => choice(
      /[^{}<>\n& ]([^{}<>\n&]*[^{}<>\n& ])?/,
      /\/\/[^\n]*/,
    ),

    // An entity can be named, numeric (decimal), or numeric (hexadecimal). The
    // longest entity name is 29 characters long, and the HTML spec says that
    // no more will ever be added.
    html_character_reference: _ => /&(#([xX][0-9a-fA-F]{1,6}|[0-9]{1,5})|[A-Za-z]{1,30});/,

    jsx_expression: $ => seq(
      '{',
      field('expression',
        optional(choice(
          $.expression,
          $.sequence_expression,
          $.spread_element,
        )),
      ),
      '}',
    ),

    _jsx_child: $ => choice(
      $.jsx_text,
      $.html_character_reference,
      $._jsx_element,
      $.jsx_expression,
      $.grit_metavariable,
    ),

    jsx_opening_element: $ => prec.dynamic(-1, seq(
      '<',
      optional(seq(
        field('name', $._jsx_element_name),
        repeat(field('attribute', $._jsx_attribute)),
      )),
      '>',
    )),

    jsx_identifier: _ => /[a-zA-Z_$][a-zA-Z\d_$]*-[a-zA-Z\d_$\-]*/,

    _jsx_identifier: $ => choice(
      alias($.jsx_identifier, $.identifier),
      $.identifier,
    ),

    nested_identifier: $ => prec('member', seq(
      field('object', choice($.identifier, alias($.nested_identifier, $.member_expression))),
      '.',
      field('property', alias($.identifier, $.property_identifier)),
    )),

    jsx_namespace_name: $ => seq(field('left', $._jsx_identifier), ':', field('right', $._jsx_identifier)),

    _jsx_element_name: $ => choice(
      $._jsx_identifier,
      alias($.nested_identifier, $.member_expression),
      $.jsx_namespace_name,
    ),

    jsx_closing_element: $ => seq(
      '</',
      optional(field('name', $._jsx_element_name)),
      '>',
    ),

    jsx_self_closing_element: $ => seq(
      '<',
      field('name', $._jsx_element_name),
      repeat(field('attribute', $._jsx_attribute)),
      '/>',
    ),

    _jsx_attribute: $ => choice($.jsx_attribute, $.jsx_expression),

    _jsx_attribute_name: $ => choice(alias($._jsx_identifier, $.property_identifier), $.jsx_namespace_name),

    jsx_attribute: $ => seq(
      field('name', $._jsx_attribute_name),
      optional(seq(
        '=',
        field('value', $._jsx_attribute_value),
      )),
    ),

    _jsx_string: $ => choice(
      seq(
        '"',
        repeat(choice(
          alias($.unescaped_double_jsx_string_fragment, $.string_fragment),
          $.html_character_reference,
        )),
        '"',
      ),
      seq(
        '\'',
        repeat(choice(
          alias($.unescaped_single_jsx_string_fragment, $.string_fragment),
          $.html_character_reference,
        )),
        '\'',
      ),
    ),

    // Workaround to https://github.com/tree-sitter/tree-sitter/issues/1156
    // We give names to the token() constructs containing a regexp
    // so as to obtain a node in the CST.
    //
    unescaped_double_jsx_string_fragment: _ => token.immediate(prec(1, /([^"&]|&[^#A-Za-z])+/)),

    // same here
    unescaped_single_jsx_string_fragment: _ => token.immediate(prec(1, /([^'&]|&[^#A-Za-z])+/)),

    _jsx_attribute_value: $ => choice(
      alias($._jsx_string, $.string),
      $.jsx_expression,
      $._jsx_element,
    ),

    class: $ => prec('literal', seq(
      repeat(field('decorator', $.decorator)),
      'class',
      field('name', optional($.identifier)),
      field('heritage', optional($.class_heritage)),
      field('body', $.class_body),
    )),

    class_declaration: $ => prec('declaration', seq(
      repeat(field('decorator', $.decorator)),
      'class',
      field('name', $.identifier),
      field('heritage', optional($.class_heritage)),
      field('body', $.class_body),
      optional($._automatic_semicolon),
    )),

    class_heritage: $ => seq('extends', field('expression', $.expression)),

    // as of 03/22/2023 function_expression in the original grammar
    function: $ => prec('literal', seq(
      field('async', optional($.async)),
      'function',
      field('name', optional($.identifier)),
      $._call_signature,
      field('body', $.statement_block),
    )),

    async: ($) => 'async',

    let: ($) => 'let',
    const: ($) => 'const',
    var: ($) => 'var',

    in: ($) => 'in',
    of: ($) => 'of',

    function_declaration: $ => prec.right('declaration', seq(
      field('async', optional($.async)),
      'function',
      field('name', $.identifier),
      $._call_signature,
      field('body', $.statement_block),
      optional($._automatic_semicolon),
    )),

    generator_function: $ => prec('literal', seq(
      field('async', optional($.async)),
      'function',
      '*',
      field('name', optional($.identifier)),
      $._call_signature,
      field('body', $.statement_block),
    )),

    generator_function_declaration: $ => prec.right('declaration', seq(
      field('async', optional($.async)),
      'function',
      '*',
      field('name', $.identifier),
      $._call_signature,
      field('body', $.statement_block),
      optional($._automatic_semicolon),
    )),

    arrow_function: $ => seq(
      field('async', optional($.async)),
      choice(
        field('parameters', choice(
          alias($._reserved_identifier, $.identifier),
          $.identifier,
        )),
        $._call_signature,
      ),
      '=>',
      field('body', choice(
        $.expression,
        $.statement_block,
      )),
    ),

    // Override
    _call_signature: $ => $._formal_parameters,
    _formal_parameter: $ => choice($.pattern, $.assignment_pattern),

    optional_chain: _ => '?.',

    chain: ($) => '.',

    call_expression: $ => choice(
      prec('call', seq(
        field('function', choice($.expression, $.import)),
        choice($._arguments, field('arguments', $.template_string)),
      )),
      prec('member', seq(
        field('function', $.primary_expression),
        field('optional_chain', $.optional_chain),
        $._arguments,
      )),
    ),

    new_expression: $ => prec.right('new', seq(
      'new',
      field('constructor', choice($.primary_expression, $.new_expression)),
      optional(prec.dynamic(1, $._arguments)),
    )),

    await_expression: $ => prec('unary_void', seq(
      'await',
      field('expression', $.expression),
    )),

    member_expression: $ => prec('member', seq(
      field('object', choice($.expression, $.primary_expression, $.import)),
      choice(field('chain', $.chain), field('chain', $.optional_chain)),
      field('property', choice(
        $.private_property_identifier,
        alias($.identifier, $.property_identifier))),
    )),

    subscript_expression: $ => prec.right('member', seq(
      field('object', choice($.expression, $.primary_expression)),
      optional(field('optional_chain', $.optional_chain)),
      '[', field('index', $._expressions), ']',
    )),

    _lhs_expression: $ => choice(
      $.member_expression,
      $.subscript_expression,
      $._identifier,
      alias($._reserved_identifier, $.identifier),
      $._destructuring_pattern,
    ),

    assignment_expression: $ => prec.right('assign', seq(
      field('left', choice($.parenthesized_expression, $._lhs_expression)),
      '=',
      field('right', $.expression),
    )),

    _augmented_assignment_lhs: $ => choice(
      $.member_expression,
      $.subscript_expression,
      alias($._reserved_identifier, $.identifier),
      $.identifier,
      $.parenthesized_expression,
    ),

    augmented_assignment_expression: ($) =>
      prec.right(
        'assign',
        seq(
          field('left', $._augmented_assignment_lhs),
          field(
            'operator',
            choice(
              $.plus_equal,
              $.minus_equal,
              $.times_equal,
              $.divide_equal,
              $.modulo_equal,
              $.xor_equal,
              $.and_equal,
              $.or_equal,
              $.right_shift_equal,
              $.unsigned_right_shift_equal,
              $.left_shift_equal,
              $.exponent_equal,
              $.logical_and_equal,
              $.logical_or_equal,
              $.logical_nullish_equal,
            ),
          ),
          field('right', $.expression),
        ),
      ),

    plus_equal: ($) => '+=',
    minus_equal: ($) => '-=',
    times_equal: ($) => '*=',
    divide_equal: ($) => '/=',
    modulo_equal: ($) => '%=',
    xor_equal: ($) => '^=',
    and_equal: ($) => '&=',
    or_equal: ($) => '|=',
    right_shift_equal: ($) => '>>=',
    unsigned_right_shift_equal: ($) => '>>>=',
    left_shift_equal: ($) => '<<=',
    exponent_equal: ($) => '**=',
    logical_and_equal: ($) => '&&=',
    logical_or_equal: ($) => '||=',
    logical_nullish_equal: ($) => '??=',

    _initializer: $ => seq(
      '=',
      field('value', $.expression),
    ),

    _destructuring_pattern: $ => choice(
      $.object_pattern,
      $.array_pattern,
    ),

    spread_element: $ => seq('...', field('expression', $.expression)),

    ternary_expression: $ => prec.right('ternary', seq(
      field('condition', $.expression),
      alias($._ternary_qmark, '?'),
      field('consequence', $.expression),
      ':',
      field('alternative', $.expression),
    )),

    binary_expression: $ => choice(
      ...[
        [$.logical_and, 'logical_and'],
        [$.logical_or, 'logical_or'],
        [$.binary_right_shift, 'binary_shift'],
        [$.binary_unsigned_right_shift, 'binary_shift'],
        [$.binary_left_shift, 'binary_shift'],
        [$.bitwise_and, 'bitwise_and'],
        [$.bitwise_xor, 'bitwise_xor'],
        [$.bitwise_or, 'bitwise_or'],
        [$.plus, 'binary_plus'],
        [$.minus, 'binary_plus'],
        [$.binary_times, 'binary_times'],
        [$.binary_divide, 'binary_times'],
        [$.binary_modulo, 'binary_times'],
        [$.binary_exp, 'binary_exp', 'right'],
        [$.less_than, 'binary_relation'],
        [$.less_than_or_equal, 'binary_relation'],
        [$.equal, 'binary_equality'],
        [$.strict_equal, 'binary_equality'],
        [$.not_equal, 'binary_equality'],
        [$.strict_not_equal, 'binary_equality'],
        [$.greater_than_or_equal, 'binary_relation'],
        [$.greater_than, 'binary_relation'],
        [$.logical_nullish, 'ternary'],
        [$.instanceof, 'binary_relation'],
        [$.in, 'binary_relation'],
      ].map(([operator, precedence, associativity]) =>
        (associativity === 'right' ? prec.right : prec.left)(precedence, seq(
          field('left', operator === 'in' ? choice($.expression, $.private_property_identifier) : $.expression),
          field('operator', operator),
          field('right', $.expression),
        )),
      ),
    ),

    logical_and: ($) => '&&',
    logical_or: ($) => '||',
    binary_right_shift: ($) => '>>',
    binary_unsigned_right_shift: ($) => '>>>',
    binary_left_shift: ($) => '<<',
    bitwise_and: ($) => '&',
    bitwise_xor: ($) => '^',
    bitwise_or: ($) => '|',
    plus: ($) => '+',
    minus: ($) => '-',
    binary_times: ($) => '*',
    binary_divide: ($) => '/',
    binary_modulo: ($) => '%',
    binary_exp: ($) => '**',
    less_than: ($) => '<',
    less_than_or_equal: ($) => '<=',
    equal: ($) => '==',
    strict_equal: ($) => '===',
    not_equal: ($) => '!=',
    strict_not_equal: ($) => '!==',
    greater_than_or_equal: ($) => '>=',
    greater_than: ($) => '>',
    logical_nullish: ($) => '??',
    instanceof: ($) => 'instanceof',

    unary_expression: $ => prec.left('unary_void', seq(
      field('operator', choice($.not, $.bitwise_not, $.minus, $.plus, $.typeof, $.void, $.delete)),
      field('argument', $.expression),
    )),

    not: ($) => '!',
    bitwise_not: ($) => '~',
    typeof: ($) => 'typeof',
    void: ($) => 'void',
    delete: ($) => 'delete',

    update_expression: $ => prec.left(choice(
      seq(
        field('argument', $.expression),
        field('operator', choice($.increment, $.decrement)),
      ),
      seq(
        field('operator', choice($.increment, $.decrement)),
        field('argument', $.expression),
      ),
    )),

    increment: ($) => '++',
    decrement: ($) => '--',

    sequence_expression: $ => prec.right(field('expressions', commaSep1($.expression))),

    //
    // Primitives
    //

    string: $ => choice(
      seq(
        '"',
        field('fragment',
          choice(
            $.grit_metavariable,
          repeat(choice(
            alias($.unescaped_double_string_fragment, $.string_fragment),
            $.escape_sequence,
          )),
        )),
        '"',
      ),
      seq(
        '\'',
        field('fragment',
          choice($.grit_metavariable,
          repeat(choice(
            alias($.unescaped_single_string_fragment, $.string_fragment),
            $.escape_sequence,
          )),
        )),
        '\'',
      ),
    ),

    // Workaround to https://github.com/tree-sitter/tree-sitter/issues/1156
    // We give names to the token() constructs containing a regexp
    // so as to obtain a node in the CST.
    //
    unescaped_double_string_fragment: _ => token.immediate(prec(1, /[^"\\\r\n]+/)),

    // same here
    unescaped_single_string_fragment: _ => token.immediate(prec(1, /[^'\\\r\n]+/)),

    escape_sequence: _ => token.immediate(seq(
      '\\',
      choice(
        /[^xu0-7]/,
        /[0-7]{1,3}/,
        /x[0-9a-fA-F]{2}/,
        /u[0-9a-fA-F]{4}/,
        /u\{[0-9a-fA-F]+\}/,
        /[\r?][\n\u2028\u2029]/,
      ),
    )),

    // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
    comment: $ => choice(
      token(choice(
        seq('//', /.*/),
        seq(
          '/*',
          /[^*]*\*+([^/*][^*]*\*+)*/,
          '/',
        ),
      )),
    ),

    template_content: ($) =>
      field(
        'content',
        repeat1(choice(alias($._template_chars, $.string_fragment), $.escape_sequence, $.template_substitution)),
      ),

    template_string: $ => seq(
      '`',
      optional(field('template', $.template_content)),
      '`',
    ),

    template_substitution: $ => seq(
      '${',
      field('expression', $._expressions),
      '}',
    ),

    regex: $ => seq(
      '/',
      field('pattern', $.regex_pattern),
      token.immediate(prec(1, '/')),
      optional(field('flags', $.regex_flags)),
    ),

    regex_pattern: _ => token.immediate(prec(-1,
      repeat1(choice(
        seq(
          '[',
          repeat(choice(
            seq('\\', /./), // escaped character
            /[^\]\n\\]/, // any character besides ']' or '\n'
          )),
          ']',
        ), // square-bracket-delimited character class
        seq('\\', /./), // escaped character
        /[^/\\\[\n]/, // any character besides '[', '\', '/', '\n'
      )),
    )),

    regex_flags: _ => token.immediate(/[a-z]+/),

    number: _ => {
      const hex_literal = seq(
        choice('0x', '0X'),
        /[\da-fA-F](_?[\da-fA-F])*/,
      );

      const decimal_digits = /\d(_?\d)*/;
      const signed_integer = seq(optional(choice('-', '+')), decimal_digits);
      const exponent_part = seq(choice('e', 'E'), signed_integer);

      const binary_literal = seq(choice('0b', '0B'), /[0-1](_?[0-1])*/);

      const octal_literal = seq(choice('0o', '0O'), /[0-7](_?[0-7])*/);

      const bigint_literal = seq(choice(hex_literal, binary_literal, octal_literal, decimal_digits), 'n');

      const decimal_integer_literal = choice(
        '0',
        seq(optional('0'), /[1-9]/, optional(seq(optional('_'), decimal_digits))),
      );

      const decimal_literal = choice(
        seq(decimal_integer_literal, '.', optional(decimal_digits), optional(exponent_part)),
        seq('.', decimal_digits, optional(exponent_part)),
        seq(decimal_integer_literal, exponent_part),
        seq(decimal_digits),
      );

      return token(choice(
        hex_literal,
        decimal_literal,
        binary_literal,
        octal_literal,
        bigint_literal,
      ));
    },

    // 'undefined' is syntactically a regular identifier in JavaScript.
    // However, its main use is as the read-only global variable whose
    // value is [undefined], for which there's no literal representation
    // unlike 'null'. We gave it its own rule so it's easy to
    // highlight in text editors and other applications.
    _identifier: $ => choice(
      $.undefined,
      $.identifier,
    ),

    
    identifier: ($) => choice($._primitive_identifier, $.grit_metavariable),

    _primitive_identifier: ($) => {
      const alpha =
        /[^\x00-\x1F\s\p{Zs}0-9:;`"'@#.,|^&<=>+\-*/\\%?!~()\[\]{}\uFEFF\u2060\u200B]|\\u[0-9a-fA-F]{4}|\\u\{[0-9a-fA-F]+\}/;
      const alphanumeric =
        /[^\x00-\x1F\s\p{Zs}:;`"'@#.,|^&<=>+\-*/\\%?!~()\[\]{}\uFEFF\u2060\u200B]|\\u[0-9a-fA-F]{4}|\\u\{[0-9a-fA-F]+\}/;
      return token(seq(alpha, repeat(alphanumeric)));
    },

    private_property_identifier: _ => {
      // eslint-disable-next-line max-len
      const alpha = /[^\x00-\x1F\s\p{Zs}0-9:;`"'@#.,|^&<=>+\-*/\\%?!~()\[\]{}\uFEFF\u2060\u200B]|\\u[0-9a-fA-F]{4}|\\u\{[0-9a-fA-F]+\}/;
      // eslint-disable-next-line max-len
      const alphanumeric = /[^\x00-\x1F\s\p{Zs}:;`"'@#.,|^&<=>+\-*/\\%?!~()\[\]{}\uFEFF\u2060\u200B]|\\u[0-9a-fA-F]{4}|\\u\{[0-9a-fA-F]+\}/;
      return token(seq('#', alpha, repeat(alphanumeric)));
    },

    meta_property: _ => seq('new', '.', 'target'),

    this: _ => 'this',
    super: _ => 'super',
    true: _ => 'true',
    false: _ => 'false',
    null: _ => 'null',
    undefined: _ => 'undefined',

    //
    // Expression components
    //

    _arguments: $ => seq(
      '(',
      field('arguments', commaSep(optional(choice($.expression, $.spread_element)))),
      ')',
    ),

    decorator: $ => seq(
      '@',
      field('identifier',
        choice(
          $.identifier,
          alias($.decorator_member_expression, $.member_expression),
          alias($.decorator_call_expression, $.call_expression),
        ),
      ),
    ),

    decorator_member_expression: $ => prec('member', seq(
      field('object', choice(
        $.identifier,
        alias($.decorator_member_expression, $.member_expression),
      )),
      '.',
      field('property', alias($.identifier, $.property_identifier)),
    )),

    decorator_call_expression: $ => prec('call', seq(
      field('function', choice(
        $.identifier,
        alias($.decorator_member_expression, $.member_expression),
      )),
      $._arguments,
    )),

    class_body: $ => seq(
      '{',
      repeat(choice(
        seq(field('member', $.method_definition), optional(';')),
        seq(field('member', $.field_definition), $._semicolon),
        field('member', $.class_static_block),
        field('template', $.glimmer_template),
        ';',
      )),
      '}',
    ),

    static: ($) => 'static',

    field_definition: $ => seq(
      repeat(field('decorator', $.decorator)),
      optional(field('static', $.static)),
      field('property', $._property_name),
      optional($._initializer),
    ),

    _formal_parameters: $ => seq(
      field('parenthesis', alias('(', $.l_parenthesis)),
      optional(seq(field('parameters',
        commaSep1($._formal_parameter)),
        optional(','),
      )),
      field('parenthesis', alias(')', $.r_parenthesis)),
    ),

    class_static_block: $ => seq(
      $.static,
      field('body', $.statement_block),
    ),

    // This negative dynamic precedence ensures that during error recovery,
    // unfinished constructs are generally treated as literal expressions,
    // not patterns.
    pattern: $ => prec.dynamic(-1, choice(
      $._lhs_expression,
      $.rest_pattern,
      $.grit_metavariable,
    )),

    rest_pattern: $ => prec.right(seq(
      '...',
      field('expression', $._lhs_expression),
    )),

    method_definition: $ => seq(
      repeat(field('decorator', $.decorator)),
      field('static', optional(choice($.static, alias(token(seq('static', /\s+/, 'get', /\s*\n/)), $.static_get)))),
      field('async', optional($.async)),
      optional(choice('get', 'set', '*')),
      field('name', $._property_name),
      $._formal_parameters,
      field('body', $.statement_block),
    ),

    pair: $ => seq(
      field('key', $._property_name),
      ':',
      field('value', $.expression),
    ),

    pair_pattern: $ => seq(
      field('key', $._property_name),
      ':',
      field('value', choice($.pattern, $.assignment_pattern)),
    ),

    _property_name: $ => choice(
      alias(choice(
        $.identifier,
        $._reserved_identifier,
      ), $.property_identifier),
      $.private_property_identifier,
      $.string,
      $.number,
      $.computed_property_name,
      $.grit_metavariable,
    ),

    computed_property_name: $ => seq(
      '[',
      field('expression', $.expression),
      ']',
    ),

    _reserved_identifier: $ => choice(
      'get',
      'set',
      $.async,
      $.static,
      'export',
      $.let,
    ),

    _semicolon: $ => choice($._automatic_semicolon, ';'),

    grit_metavariable: ($) => token(prec(100, choice("µ...", /µ[a-zA-Z_][a-zA-Z0-9_]*/))),
  },
});

/**
 * Creates a rule to match one or more of the rules separated by a comma
 *
 * @param {Rule} rule
 *
 * @return {SeqRule}
 *
 */
function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)));
}

/**
 * Creates a rule to optionally match one or more of the rules separated by a comma
 *
 * @param {Rule} rule
 *
 * @return {ChoiceRule}
 *
 */
function commaSep(rule) {
  return optional(commaSep1(rule));
}
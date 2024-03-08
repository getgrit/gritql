const PREC = {
  // this resolves a conflict between the usage of ':' in a lambda vs in a
  // typed parameter. In the case of a lambda, we don't allow typed parameters.
  lambda: -2,
  typed_parameter: -1,
  conditional: -1,

  parenthesized_expression: 1,
  parenthesized_list_splat: 1,
  or: 10,
  and: 11,
  not: 12,
  compare: 13,
  bitwise_or: 14,
  bitwise_and: 15,
  xor: 16,
  shift: 17,
  plus: 18,
  times: 19,
  unary: 20,
  power: 21,
  call: 22,
  grit_metavariable: 100,
};

const SEMICOLON = ';';

module.exports = grammar({
  name: 'python',

  extras: ($) => [$.comment, /[\s\f\uFEFF\u2060\u200B]|\r?\n/, $.line_continuation],

  conflicts: ($) => [
    [$.primary_expression, $.pattern],
    [$.primary_expression, $.list_splat_pattern],
    [$.tuple, $.tuple_pattern],
    [$.list, $.list_pattern],
    [$.with_item, $._collection_elements],
    [$.named_expression, $.as_pattern],
    [$.match_statement, $.primary_expression],
    [$.print_statement, $.primary_expression],
    // GRIT METAVARIABLE CONFLICTS
    [$.pair, $.identifier],
  ],

  supertypes: ($) => [
    $._simple_statement,
    $._compound_statement,
    $.expression,
    $.primary_expression,
    $.pattern,
    $.parameter,
  ],

  externals: ($) => [
    $._newline,
    $._indent,
    $._dedent,
    $.string_start,
    $._string_content,
    $.string_end,

    // Mark comments as external tokens so that the external scanner is always
    // invoked, even if no external token is expected. This allows for better
    // error recovery, because the external scanner can maintain the overall
    // structure by returning dedent tokens whenever a dedent occurs, even
    // if no dedent is expected.
    $.comment,

    // Allow the external scanner to check for the validity of closing brackets
    // so that it can avoid returning dedent tokens between brackets.
    ']',
    ')',
    '}',
  ],

  inline: ($) => [
    $._simple_statement,
    $._compound_statement,
    $._suite,
    $._expressions,
    $._left_hand_side,
    $.keyword_identifier,
  ],

  word: ($) => $._primitive_identifier,

  rules: {
    module: ($) => repeat(field('statements', $._statement)),

    _statement: ($) => choice($._simple_statements, $._compound_statement),

    // Simple statements

    _simple_statements: ($) =>
      seq(sep1($._simple_statement, SEMICOLON), optional(SEMICOLON), $._newline),

    _simple_statement: ($) =>
      choice(
        $.future_import_statement,
        $.import_statement,
        $.import_from_statement,
        $.print_statement,
        $.assert_statement,
        $._expression_statement,
        $.return_statement,
        $.delete_statement,
        $.raise_statement,
        $.pass_statement,
        $.break_statement,
        $.continue_statement,
        $.global_statement,
        $.nonlocal_statement,
        $.exec_statement,
      ),

    import_statement: ($) => seq('import', $._import_list),

    import_prefix: ($) => repeat1('.'),

    relative_import: ($) => seq($.import_prefix, field('name', optional($.dotted_name))),

    future_import_statement: ($) =>
      seq('from', '__future__', 'import', choice($._import_list, seq('(', $._import_list, ')'))),

    import_from_statement: ($) =>
      seq(
        'from',
        field('module_name', choice($.relative_import, $.dotted_name)),
        'import',
        choice($.wildcard_import, $._import_list, seq('(', $._import_list, ')')),
      ),

    _import_list: ($) =>
      seq(commaSep1(field('name', choice($.dotted_name, $.aliased_import))), optional(',')),

    aliased_import: ($) => seq(field('name', $.dotted_name), 'as', field('alias', $.identifier)),

    wildcard_import: ($) => '*',

    print_statement: ($) =>
      choice(
        prec(
          1,
          seq(
            'print',
            field('chevron', $.chevron),
            repeat(seq(',', field('argument', $.expression))),
            optional(','),
          ),
        ),
        prec(
          -3,
          prec.dynamic(-1, seq('print', commaSep1(field('argument', $.expression)), optional(','))),
        ),
      ),

    chevron: ($) => seq('>>', field('expression', $.expression)),

    assert_statement: ($) => seq('assert', commaSep1(field('assertion', $.expression))),

    expression_list: ($) => seq(commaSep1(field('expressions', $.expression)), optional(',')),

    _expression_statement: ($) =>
        choice(
          $.expression,
          $.expression_list,
          $.assignment,
          $.augmented_assignment,
          $.yield,
      ),

    named_expression: ($) =>
      seq(field('name', $._named_expression_lhs), ':=', field('value', $.expression)),

    _named_expression_lhs: ($) => choice($.identifier, $.keyword_identifier),

    return_statement: ($) => seq('return', optional(field('value', $._expressions))),

    delete_statement: ($) => seq('del', field('deleted', $._expressions)),

    _expressions: ($) => choice($.expression, $.expression_list),

    raise_statement: ($) =>
      seq(
        'raise',
        field('exception', optional($._expressions)),
        optional(seq('from', field('cause', $.expression))),
      ),

    pass_statement: ($) => prec.left('pass'),
    break_statement: ($) => prec.left('break'),
    continue_statement: ($) => prec.left('continue'),

    // Compound statements

    _compound_statement: ($) =>
      choice(
        $.if_statement,
        $.for_statement,
        $.while_statement,
        $.try_statement,
        $.with_statement,
        $.function_definition,
        $.class_definition,
        $.decorated_definition,
        $.match_statement,
      ),

    if_statement: ($) =>
      seq(
        'if',
        field('condition', $.expression),
        ':',
        field('consequence', $._suite),
        repeat(field('alternative', $.elif_clause)),
        optional(field('alternative', $.else_clause)),
      ),

    elif_clause: ($) =>
      seq('elif', field('condition', $.expression), ':', field('consequence', $._suite)),

    else_clause: ($) => seq('else', ':', field('body', $._suite)),

    match_statement: ($) =>
      seq(
        'match',
        commaSep1(field('subject', $.expression)),
        optional(','),
        ':',
        field('body', alias($._match_block, $.block)),
      ),

    _match_block: ($) =>
      choice(seq($._indent, repeat(field('alternative', $.case_clause)), $._dedent), $._newline),

    case_clause: ($) =>
      seq(
        'case',
        commaSep1(field('pattern', alias($.expression, $.case_pattern))),
        optional(','),
        optional(field('guard', $.if_clause)),
        ':',
        field('consequence', $._suite),
      ),

    for_statement: ($) =>
      seq(
        optional('async'),
        'for',
        field('left', $._left_hand_side),
        'in',
        field('right', $._expressions),
        ':',
        field('body', $._suite),
        field('alternative', optional($.else_clause)),
      ),

    while_statement: ($) =>
      seq(
        'while',
        field('condition', $.expression),
        ':',
        field('body', $._suite),
        optional(field('alternative', $.else_clause)),
      ),

    try_statement: ($) =>
      seq(
        'try',
        ':',
        field('body', $._suite),
        choice(
          seq(
            repeat1(field('except', $.except_clause)),
            optional(field('else', $.else_clause)),
            optional(field('finally', $.finally_clause)),
          ),
          seq(
            repeat1(field('except', $.except_group_clause)),
            optional(field('else', $.else_clause)),
            optional(field('finallly', $.finally_clause)),
          ),
          field('finally', $.finally_clause),
        ),
      ),

    except_clause: ($) =>
      seq(
        'except',
        optional(
          seq(
            field('exception', $.expression),
            optional(seq(choice('as', ','), field('alias', $.expression))),
          ),
        ),
        ':',
        field('conesequence', $._suite),
      ),

    except_group_clause: ($) =>
      seq(
        'except*',
        seq(field('exception', $.expression), optional(seq('as', field('alias', $.expression)))),
        ':',
        field('consequence', $._suite),
      ),

    finally_clause: ($) => seq('finally', ':', field('consequence', $._suite)),

    with_statement: ($) =>
      seq(optional('async'), 'with', field('context', $.with_clause), ':', field('body', $._suite)),

    with_clause: ($) =>
      choice(
        seq(commaSep1(field('items', $.with_item)), optional(',')),
        seq('(', commaSep1(field('items', $.with_item)), optional(','), ')'),
      ),

    with_item: ($) => prec.dynamic(1, seq(field('value', $.expression))),

    function_definition: ($) =>
      seq(
        optional('async'),
        'def',
        field('name', $.identifier),
        seq('(', optional($._parameters), ')'),
        optional(seq('->', field('return_type', $.type))),
        ':',
        field('body', $._suite),
      ),

    // parameters: ($) => seq('(', optional($._parameters), ')'),

    lambda_parameters: ($) => $._parameters,

    list_splat: ($) => seq('*', field('list', $.expression)),

    dictionary_splat: ($) => seq('**', field('dict', $.expression)),

    global_statement: ($) => seq('global', commaSep1(field('globals', $.identifier))),

    nonlocal_statement: ($) => seq('nonlocal', commaSep1(field('non_locals', $.identifier))),

    exec_statement: ($) =>
      seq(
        'exec',
        field('code', choice($.string, $.identifier)),
        optional(seq('in', field('context', commaSep1($.expression)))),
      ),

    class_definition: ($) =>
      seq(
        'class',
        field('name', $.identifier),
        field('superclasses', optional($.argument_list)),
        ':',
        field('body', $._suite),
      ),

    parenthesized_list_splat: ($) =>
      prec(
        PREC.parenthesized_list_splat,
        seq(
          '(',
          field(
            'list',
            choice(alias($.parenthesized_list_splat, $.parenthesized_expression), $.list_splat),
          ),
          ')',
        ),
      ),

    argument_list: ($) =>
      seq(
        '(',
        optional(
          commaSep1(
            field(
              'arguments',
              choice(
                $.expression,
                $.list_splat,
                $.dictionary_splat,
                alias($.parenthesized_list_splat, $.parenthesized_expression),
                $.keyword_argument,
              ),
            ),
          ),
        ),
        optional(','),
        ')',
      ),

    decorated_definition: ($) =>
      seq(
        repeat1(field('decorators', $.decorator)),
        field('definition', choice($.class_definition, $.function_definition)),
      ),

    decorator: ($) => seq('@', field('value', $.expression), $._newline),

    _suite: ($) =>
      choice(
        alias($._simple_statements, $.block),
        seq($._indent, $.block),
        alias($._newline, $.block),
      ),

    block: ($) => seq(repeat(field('statements', $._statement)), $._dedent),

    expression_list: ($) =>
      prec.right(
        seq(
          field('expressions', $.expression),
          choice(',', seq(repeat1(seq(',', field('expressions', $.expression))), optional(','))),
        ),
      ),

    dotted_name: ($) => sep1(field('name', $.identifier), '.'),

    // Patterns

    _parameters: ($) => seq(commaSep1(field('parameters', $.parameter)), optional(',')),

    _patterns: ($) => seq(commaSep1(field('pattern', $.pattern)), optional(',')),

    parameter: ($) =>
      choice(
        $.identifier,
        $.typed_parameter,
        $.default_parameter,
        $.typed_default_parameter,
        $.list_splat_pattern,
        $.tuple_pattern,
        $.keyword_separator,
        $.positional_separator,
        $.dictionary_splat_pattern,
      ),

    pattern: ($) =>
      choice(
        $.identifier,
        $.keyword_identifier,
        $.subscript,
        $.attribute,
        $.list_splat_pattern,
        $.tuple_pattern,
        $.list_pattern,
      ),

    tuple_pattern: ($) => seq('(', optional($._patterns), ')'),

    list_pattern: ($) => seq('[', optional($._patterns), ']'),

    default_parameter: ($) =>
      seq(field('name', choice($.identifier, $.tuple_pattern)), '=', field('value', $.expression)),

    typed_default_parameter: ($) =>
      prec(
        PREC.typed_parameter,
        seq(
          field('name', $.identifier),
          ':',
          field('type', $.type),
          '=',
          field('value', $.expression),
        ),
      ),

    list_splat_pattern: ($) =>
      seq('*', field('list', choice($.identifier, $.keyword_identifier, $.subscript, $.attribute))),

    dictionary_splat_pattern: ($) =>
      seq(
        '**',
        field('dict', choice($.identifier, $.keyword_identifier, $.subscript, $.attribute)),
      ),

    // Extended patterns (patterns allowed in match statement are far more flexible than simple patterns though still a subset of "expression")

    as_pattern: ($) =>
      prec.left(
        seq(
          field('expression', $.expression),
          'as',
          field('alias', alias($.expression, $.as_pattern_target)),
        ),
      ),

    // Expressions

    expression_within_for_in_clause: ($) =>
      choice(
        field('expression', $.expression),
        field('function', alias($.lambda_within_for_in_clause, $.lambda)),
      ),

    expression: ($) =>
      choice(
        $.comparison_operator,
        $.not_operator,
        $.boolean_operator,
        $.lambda,
        $.primary_expression,
        $.conditional_expression,
        $.named_expression,
        $.as_pattern,
      ),

    primary_expression: ($) =>
      choice(
        $.await,
        $.binary_operator,
        $.identifier,
        $.keyword_identifier,
        $.string,
        $.concatenated_string,
        $.integer,
        $.float,
        $.true,
        $.false,
        $.none,
        $.unary_operator,
        $.attribute,
        $.subscript,
        $.call,
        $.list,
        $.list_comprehension,
        $.dictionary,
        $.dictionary_comprehension,
        $.set,
        $.set_comprehension,
        $.tuple,
        $.parenthesized_expression,
        $.generator_expression,
        $.ellipsis,
        alias($.list_splat_pattern, $.list_splat),
      ),

    not_operator: ($) => prec(PREC.not, seq('not', field('argument', $.expression))),

    boolean_operator: ($) =>
      choice(
        prec.left(
          PREC.and,
          seq(field('left', $.expression), field('operator', $.and), field('right', $.expression)),
        ),
        prec.left(
          PREC.or,
          seq(field('left', $.expression), field('operator', $.or), field('right', $.expression)),
        ),
      ),

    and: (_$) => 'and',
    or: (_$) => 'or',
    plus: (_$) => '+',
    minus: (_$) => '-',
    times: (_$) => '*',
    matrix_multiply: (_$) => '@',
    divide: (_$) => '/',
    modulo: (_$) => '%',
    floor_divide: (_$) => '//',
    power: (_$) => '**',
    bitwise_or: (_$) => '|',
    bitwise_and: (_$) => '&',
    xor: (_$) => '^',
    shift_left: (_$) => '<<',
    shift_right: (_$) => '>>',
    complement: (_$) => '~',

    binary_operator: ($) => {
      const table = [
        [prec.left, $.plus, PREC.plus],
        [prec.left, $.minus, PREC.plus],
        [prec.left, $.times, PREC.times],
        [prec.left, $.matrix_multiply, PREC.times],
        [prec.left, $.divide, PREC.times],
        [prec.left, $.modulo, PREC.times],
        [prec.left, $.floor_divide, PREC.times],
        [prec.right, $.power, PREC.power],
        [prec.left, $.bitwise_or, PREC.bitwise_or],
        [prec.left, $.bitwise_and, PREC.bitwise_and],
        [prec.left, $.xor, PREC.xor],
        [prec.left, $.shift_left, PREC.shift],
        [prec.left, $.shift_right, PREC.shift],
      ];

      return choice(
        ...table.map(([fn, operator, precedence]) =>
          fn(
            precedence,
            seq(
              field('left', $.primary_expression),
              field('operator', operator),
              field('right', $.primary_expression),
            ),
          ),
        ),
      );
    },

    unary_operator: ($) =>
      prec(
        PREC.unary,
        seq(
          field('operator', choice($.plus, $.minus, $.complement)),
          field('argument', $.primary_expression),
        ),
      ),

    less_than: (_$) => '<',
    less_than_or_equal: (_$) => '<=',
    equal: (_$) => '==',
    not_equal: (_$) => '!=',
    greater_than_or_equal: (_$) => '>=',
    greater_than: (_$) => '>',
    not_equal_2: (_$) => '<>',
    in: (_$) => 'in',
    not_in: (_$) => seq('not', 'in'),
    is: (_$) => 'is',
    is_not: (_$) => seq('is', 'not'),

    comparison_operator: ($) =>
      prec.left(
        PREC.compare,
        seq(
          field('left', $.primary_expression),
          repeat1(
            seq(
              field(
                'operators',
                choice(
                  $.less_than,
                  $.less_than_or_equal,
                  $.equal,
                  $.not_equal,
                  $.greater_than_or_equal,
                  $.greater_than,
                  $.not_equal_2,
                  $.in,
                  $.not_in,
                  $.is,
                  $.is_not,
                ),
              ),
              field('right', $.primary_expression),
            ),
          ),
        ),
      ),

    lambda: ($) =>
      prec(
        PREC.lambda,
        seq(
          'lambda',
          field('parameters', optional($.lambda_parameters)),
          ':',
          field('body', $.expression),
        ),
      ),

    lambda_within_for_in_clause: ($) =>
      seq(
        'lambda',
        field('parameters', optional($.lambda_parameters)),
        ':',
        field('body', $.expression_within_for_in_clause),
      ),

    assignment: ($) =>
      seq(
        field('left', $._left_hand_side),
        choice(
          seq('=', field('right', $._right_hand_side)),
          seq(':', field('type', $.type)),
          seq(':', field('type', $.type), '=', field('right', $._right_hand_side)),
        ),
      ),

    plus_equal: (_$) => '+=',
    minus_equal: (_$) => '-=',
    times_equal: (_$) => '*=',
    devide_equal: (_$) => '/=',
    matrix_multiply_equal: (_$) => '@=',
    floor_divide_equal: (_$) => '//=',
    modulo_equal: (_$) => '%=',
    power_equal: (_$) => '**=',
    shift_right_equal: (_$) => '>>=',
    shift_left_equal: (_$) => '<<=',
    bitwise_and_equal: (_$) => '&=',
    bitwise_xor_equal: (_$) => '^=',
    bitwise_or_equal: (_$) => '|=',

    augmented_assignment: ($) =>
      seq(
        field('left', $._left_hand_side),
        field(
          'operator',
          choice(
            $.plus_equal,
            $.minus_equal,
            $.times_equal,
            $.devide_equal,
            $.matrix_multiply_equal,
            $.floor_divide_equal,
            $.modulo_equal,
            $.power_equal,
            $.shift_right_equal,
            $.shift_left_equal,
            $.bitwise_and_equal,
            $.bitwise_xor_equal,
            $.bitwise_or_equal,
          ),
        ),
        field('right', $._right_hand_side),
      ),

    _left_hand_side: ($) => choice($.pattern, $.pattern_list),

    pattern_list: ($) =>
      seq(
        field('patterns', $.pattern),
        choice(',', seq(repeat1(seq(',', field('patterns', $.pattern))), optional(','))),
      ),

    _right_hand_side: ($) =>
      choice(
        $.expression,
        $.expression_list,
        $.assignment,
        $.augmented_assignment,
        $.pattern_list,
        $.yield,
      ),

    yield: ($) =>
      prec.right(
        seq(
          'yield',
          choice(seq('from', $.expression), optional(field('expression', $._expressions))),
        ),
      ),

    attribute: ($) =>
      prec(
        PREC.call,
        seq(field('object', $.primary_expression), '.', field('attribute', $.identifier)),
      ),

    subscript: ($) =>
      prec(
        PREC.call,
        seq(
          field('value', $.primary_expression),
          '[',
          commaSep1(field('subscript', choice($.expression, $.slice))),
          optional(','),
          ']',
        ),
      ),

    slice: ($) =>
      seq(
        field('start', optional($.expression)),
        ':',
        field('stop', optional($.expression)),
        optional(seq(':', field('step', optional($.expression)))),
      ),

    ellipsis: ($) => '...',

    call: ($) =>
      prec(
        PREC.call,
        seq(
          field('function', $.primary_expression),
          field('arguments', choice($.generator_expression, $.argument_list)),
        ),
      ),

    typed_parameter: ($) =>
      prec(
        PREC.typed_parameter,
        seq(
          field('name', choice($.identifier, $.list_splat_pattern, $.dictionary_splat_pattern)),
          ':',
          field('type', $.type),
        ),
      ),

    type: ($) => field('type', $.expression),

    keyword_argument: ($) =>
      seq(
        field('name', choice($.identifier, $.keyword_identifier)),
        '=',
        field('value', $.expression),
      ),

    // Literals

    list: ($) => seq('[', optional($._collection_elements), ']'),

    set: ($) => seq('{', $._collection_elements, '}'),

    tuple: ($) => seq('(', optional($._collection_elements), ')'),

    dictionary: ($) =>
      seq('{', optional(commaSep1(choice($.pair, $.dictionary_splat))), optional(','), '}'),

    pair: ($) =>
      choice(
        $.grit_metavariable,
        seq(field('key', $.expression), ':', field('value', $.expression)),
      ),

    list_comprehension: ($) => seq('[', field('body', $.expression), $._comprehension_clauses, ']'),

    dictionary_comprehension: ($) => seq('{', field('body', $.pair), $._comprehension_clauses, '}'),

    set_comprehension: ($) => seq('{', field('body', $.expression), $._comprehension_clauses, '}'),

    generator_expression: ($) =>
      seq('(', field('body', $.expression), $._comprehension_clauses, ')'),

    // feels like this will be a pain later, maybe needs to be recursive?
    _comprehension_clauses: ($) =>
      seq(
        field('for_in', $.for_in_clause),
        repeat(choice(field('for_in', $.for_in_clause), field('condition', $.if_clause))),
      ),

    parenthesized_expression: ($) =>
      prec(
        PREC.parenthesized_expression,
        seq('(', field('exression', choice($.expression, $.yield)), ')'),
      ),

    _collection_elements: ($) =>
      seq(
        field(
          'elements',
          commaSep1(choice($.expression, $.yield, $.list_splat, $.parenthesized_list_splat)),
        ),
        optional(','),
      ),

    for_in_clause: ($) =>
      prec.left(
        seq(
          optional('async'),
          'for',
          field('left', $._left_hand_side),
          'in',
          field('right', commaSep1($.expression_within_for_in_clause)),
          optional(','),
        ),
      ),

    if_clause: ($) => seq('if', field('condition', $.expression)),

    conditional_expression: ($) =>
      prec.right(
        PREC.conditional,
        seq(
          field('true', $.expression),
          'if',
          field('condition', $.expression),
          'else',
          field('false', $.expression),
        ),
      ),

    concatenated_string: ($) =>
      seq(field('strings', $.string), repeat1(field('strings', $.string))),

    string: ($) =>
      seq(
        $.string_start,
        field(
          'content',
          choice($.grit_metavariable, repeat(choice($.interpolation, $.string_content))),
        ),
        $.string_end,
      ),

    string_content: ($) =>
      prec.right(
        repeat1(
          choice(
            $._escape_interpolation,
            $.escape_sequence,
            $._not_escape_sequence,
            $._string_content,
          ),
        ),
      ),

    interpolation: ($) =>
      seq(
        '{',
        field('expression', $._f_expression),
        optional('='),
        optional(field('type_conversion', $.type_conversion)),
        optional(field('format_specifier', $.format_specifier)),
        '}',
      ),

    _f_expression: ($) => choice($.expression, $.expression_list, $.pattern_list, $.yield),

    _escape_interpolation: ($) => token.immediate(choice('{{', '}}')),

    escape_sequence: ($) =>
      token.immediate(
        prec(
          1,
          seq(
            '\\',
            choice(
              /u[a-fA-F\d]{4}/,
              /U[a-fA-F\d]{8}/,
              /x[a-fA-F\d]{2}/,
              /\d{3}/,
              /\r?\n/,
              /['"abfrntv\\]/,
              /N\{[^}]+\}/,
            ),
          ),
        ),
      ),

    _not_escape_sequence: ($) => token.immediate('\\'),

    format_specifier: ($) =>
      seq(
        ':',
        repeat(choice(token(prec(1, /[^{}\n]+/)), alias($.interpolation, $.format_expression))),
      ),

    type_conversion: ($) => /![a-z]/,

    integer: ($) =>
      token(
        choice(
          seq(choice('0x', '0X'), repeat1(/_?[A-Fa-f0-9]+/), optional(/[Ll]/)),
          seq(choice('0o', '0O'), repeat1(/_?[0-7]+/), optional(/[Ll]/)),
          seq(choice('0b', '0B'), repeat1(/_?[0-1]+/), optional(/[Ll]/)),
          seq(
            repeat1(/[0-9]+_?/),
            choice(
              optional(/[Ll]/), // long numbers
              optional(/[jJ]/), // complex numbers
            ),
          ),
        ),
      ),

    float: ($) => {
      const digits = repeat1(/[0-9]+_?/);
      const exponent = seq(/[eE][\+-]?/, digits);

      return token(
        seq(
          choice(
            seq(digits, '.', optional(digits), optional(exponent)),
            seq(optional(digits), '.', digits, optional(exponent)),
            seq(digits, exponent),
          ),
          optional(choice(/[Ll]/, /[jJ]/)),
        ),
      );
    },

    identifier: ($) => choice($._primitive_identifier, $.grit_metavariable),

    _primitive_identifier: ($) => /[_\p{XID_Start}][_\p{XID_Continue}]*/,

    keyword_identifier: ($) =>
      prec(-3, alias(choice('print', 'exec', 'async', 'await', 'match'), $.identifier)),

    true: ($) => 'True',
    false: ($) => 'False',
    none: ($) => 'None',

    await: ($) => prec(PREC.unary, seq('await', field('expression', $.primary_expression))),

    comment: ($) => token(seq('#', /.*/)),

    line_continuation: ($) => token(seq('\\', choice(seq(optional('\r'), '\n'), '\0'))),

    positional_separator: ($) => '/',
    keyword_separator: ($) => '*',

    grit_metavariable: ($) => token(prec(PREC.grit_metavariable, choice("µ...", /µ[a-zA-Z_][a-zA-Z0-9_]*/))),
  },
});

function commaSep1(rule) {
  return sep1(rule, ',');
}

function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}

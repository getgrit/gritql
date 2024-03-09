module.exports = grammar({
  name: 'css',

  extras: ($) => [/\s/, $.comment],

  externals: ($) => [$._descendant_operator],

  conflicts: ($) => [
    [$._selector, $.declaration],
    [$.block, $._selector],
  ],

  // conflicts: ($) => [
  //   // The following conflicts are all due to the insertion of the 'grit_metavariable' rule
  //   [$._selector, $.identifier],
  //   [$.stylesheet, $._selector, $.identifier],
  //   [$._value, $.identifier],
  //   [$._query, $.identifier],
  //   [$.block, $._selector, $.identifier],
  //   [$._selector, $._value, $.identifier],

  //   [$._selector, $.declaration],
  // ],

  inline: ($) => [$._top_level_item, $._block_item],

  rules: {
    stylesheet: ($) => repeat(field('items', $._top_level_item)),

    _top_level_item: ($) =>
      choice(
        $.declaration,
        $.rule_set,
        $.import_statement,
        $.media_statement,
        $.charset_statement,
        $.namespace_statement,
        $.keyframes_statement,
        $.supports_statement,
        $.at_rule,
      ),

    // Statements

    import_statement: ($) =>
      seq('@import', field('value', $._value), sep(',', field('from', $._query)), ';'),

    media_statement: ($) =>
      seq('@media', sep1(',', field('media_type', $._query)), field('body', $.block)),

    charset_statement: ($) => seq('@charset', field('charset', $._value), ';'),

    namespace_statement: ($) =>
      seq(
        '@namespace',
        optional(field('namespace', alias($.identifier, $.namespace_name))),
        field('value', choice($.string_value, $.call_expression)),
        ';',
      ),

    keyframes_statement: ($) =>
      seq(
        field('annotation', choice('@keyframes', alias(/@[-a-z]+keyframes/, $.at_keyword))),
        field('name', alias($.identifier, $.keyframes_name)),
        field('blocks', $.keyframe_block_list),
      ),

    keyframe_block_list: ($) => seq('{', field('keyframes', repeat($.keyframe_block)), '}'),

    keyframe_block: ($) =>
      seq(field('offset', choice($.from, $.to, $.integer_value)), field('body', $.block)),

    from: ($) => 'from',
    to: ($) => 'to',

    supports_statement: ($) => seq('@supports', field('feature', $._query), field('body', $.block)),

    at_rule: ($) =>
      seq(
        field('rule', $.at_keyword),
        sep(',', field('query', $._query)),
        choice(';', field('body', $.block)),
      ),

    // Rule sets

    rule_set: ($) => seq(field('selectors', $.selectors), field('body', $.block)),

    selectors: ($) => sep1(',', field('selectors', $._selector)),

    block: ($) =>
      seq(
        '{',
        field('item', repeat($._block_item)),
        optional(field('declaration', alias($.last_declaration, $.declaration))),
        '}',
      ),

    _block_item: ($) =>
      choice(
        $.declaration,
        $.rule_set,
        $.import_statement,
        $.media_statement,
        $.charset_statement,
        $.namespace_statement,
        $.keyframes_statement,
        $.supports_statement,
        $.at_rule,
        $.grit_metavariable,
      ),

    // Selectors

    _selector: ($) =>
      choice(
        $.universal_selector,
        alias($.identifier, $.tag_name),
        $.class_selector,
        $.nesting_selector,
        $.pseudo_class_selector,
        $.pseudo_element_selector,
        $.id_selector,
        $.attribute_selector,
        $.string_value,
        $.child_selector,
        $.descendant_selector,
        $.sibling_selector,
        $.adjacent_sibling_selector,
        $.grit_metavariable,
      ),

    nesting_selector: ($) => '&',

    universal_selector: ($) => '*',

    class_selector: ($) =>
      prec(
        1,
        seq(
          optional(field('selector', $._selector)),
          '.',
          field('class', alias($.identifier, $.class_name)),
        ),
      ),

    pseudo_class_selector: ($) =>
      seq(
        optional(field('selector', $._selector)),
        ':',
        field('class', alias($.identifier, $.class_name)),
        optional(field('arguments', alias($.pseudo_class_arguments, $.arguments))),
      ),

    pseudo_element_selector: ($) =>
      seq(
        optional(field('selector', $._selector)),
        '::',
        field('name', alias($.identifier, $.tag_name)),
        optional(field('arguments', alias($.pseudo_element_arguments, $.arguments))),
      ),

    id_selector: ($) =>
      seq(
        optional(field('selector', $._selector)),
        '#',
        field('name', alias($.identifier, $.id_name)),
      ),

    equal: ($) => '=',
    contains_word_equal: ($) => '~=',
    starts_with_equal: ($) => '^=',
    dash_equal: ($) => '|=',
    contains_equal: ($) => '*=',
    ends_equal: ($) => '$=',

    attribute_selector: ($) =>
      seq(
        optional(field('selector', $._selector)),
        '[',
        field('attribute', alias($.identifier, $.attribute_name)),

        optional(
          seq(
            field(
              'selector_type',
              choice(
                $.equal,
                $.contains_word_equal,
                $.starts_with_equal,
                $.dash_equal,
                $.contains_equal,
                $.ends_equal,
              ),
            ),
            field('value', $._value),
          ),
        ),
        ']',
      ),

    child_selector: ($) =>
      prec.left(seq(field('parent', $._selector), '>', field('child', $._selector))),

    descendant_selector: ($) =>
      prec.left(
        seq(
          field('ancestor', $._selector),
          $._descendant_operator,
          field('descendant', $._selector),
        ),
      ),

    sibling_selector: ($) =>
      prec.left(seq(field('sibling', $._selector), '~', field('primary', $._selector))),

    adjacent_sibling_selector: ($) =>
      prec.left(seq(field('first', $._selector), '+', field('second', $._selector))),

    pseudo_class_arguments: ($) =>
      seq(
        token.immediate('('),
        sep(',', field('arguments', choice($._selector, repeat1($._value)))),
        ')',
      ),

    pseudo_element_arguments: ($) =>
      seq(
        token.immediate('('),
        sep(',', field('arguments', choice($._selector, repeat1($._value)))),
        ')',
      ),

    // Declarations

    declaration: ($) =>
      seq(
        field('name', alias($.identifier, $.property_name)),
        ':',
        field('values', $._value),
        repeat(seq(optional(','), field('values', $._value))),
        optional(field('important', $.important)),
        ';',
      ),

    last_declaration: ($) =>
      prec(
        1,
        seq(
          field('name', alias($.identifier, $.property_name)),
          ':',
          field('values', $._value),
          repeat(seq(optional(','), field('values', $._value))),
          optional(field('important', $.important)),
        ),
      ),

    important: ($) => '!important',

    // Media queries

    _query: ($) =>
      choice(
        alias($.identifier, $.keyword_query),
        $.feature_query,
        $.binary_query,
        $.unary_query,
        $.selector_query,
        $.parenthesized_query,
        $.grit_metavariable,
      ),

    feature_query: ($) =>
      seq(
        '(',
        field('name', alias($.identifier, $.feature_name)),
        ':',
        field('value', repeat1($._value)),
        ')',
      ),

    parenthesized_query: ($) => seq('(', field('query', $._query), ')'),

    and: ($) => 'and',
    or: ($) => 'or',

    binary_query: ($) =>
      prec.left(seq($._query, field('operator', choice($.and, $.or)), field('query', $._query))),

    not: ($) => 'not',
    only: ($) => 'only',

    unary_query: ($) =>
      prec(1, seq(field('operator', choice($.not, $.only)), field('query', $._query))),

    selector_query: ($) => seq('selector', '(', field('selector', $._selector), ')'),

    // Property Values

    _value: ($) =>
      prec(
        -1,
        choice(
          alias($.identifier, $.plain_value),
          $.plain_value,
          $.color_value,
          $.integer_value,
          $.float_value,
          $.string_value,
          $.binary_expression,
          $.parenthesized_value,
          $.call_expression,
          $.grit_metavariable,
        ),
      ),

    parenthesized_value: ($) => seq('(', field('value', $._value), ')'),

    color_value: ($) => seq('#', token.immediate(/[0-9a-fA-F]{3,8}/)),

    string_value: ($) =>
      token(choice(seq("'", /([^'\n]|\\(.|\n))*/, "'"), seq('"', /([^"\n]|\\(.|\n))*/, '"'))),

    integer_value: ($) => seq(token(seq(optional(choice('+', '-')), /\d+/)), optional($.unit)),

    float_value: ($) =>
      seq(
        token(
          seq(
            optional(choice('+', '-')),
            /\d*/,
            choice(
              seq('.', /\d+/),
              seq(/[eE]/, optional('-'), /\d+/),
              seq('.', /\d+/, /[eE]/, optional('-'), /\d+/),
            ),
          ),
        ),
        optional($.unit),
      ),

    unit: ($) => token.immediate(/[a-zA-Z%]+/),

    call_expression: ($) =>
      seq(field('name', alias($.identifier, $.function_name)), field('arguments', $.arguments)),

    plus: ($) => '+',
    minus: ($) => '-',
    times: ($) => '*',
    divide: ($) => '/',

    binary_expression: ($) =>
      prec.left(
        seq(
          field('left', $._value),
          field('operator', choice($.plus, $.minus, $.times, $.divide)),
          field('right', $._value),
        ),
      ),

    arguments: ($) =>
      seq(token.immediate('('), sep(choice(',', ';'), field('values', repeat1($._value))), ')'),

    identifier: ($) => /(--|-?[a-zA-Z_])[a-zA-Z0-9-_]*/,

    at_keyword: ($) => /@[a-zA-Z-_]+/,

    comment: ($) => token(seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')),

    plain_value: ($) =>
      token(
        seq(
          repeat(
            choice(
              /[-_]/,
              /\/[^\*\s,;!{}()\[\]]/, // Slash not followed by a '*' (which would be a comment)
            ),
          ),
          /[a-zA-Z]/,
          repeat(
            choice(
              /[^/\s,;!{}()\[\]]/, // Not a slash, not a delimiter character
              /\/[^\*\s,;!{}()\[\]]/, // Slash not followed by a '*' (which would be a comment)
            ),
          ),
        ),
      ),
    grit_metavariable: ($) => token(prec(100, choice("µ...", /µ[a-zA-Z_][a-zA-Z0-9_]*/))),
  },
});

function sep(separator, rule) {
  return optional(sep1(separator, rule));
}

function sep1(separator, rule) {
  return seq(rule, repeat(seq(separator, rule)));
}

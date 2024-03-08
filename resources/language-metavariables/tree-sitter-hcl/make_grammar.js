module.exports = function make_grammar(dialect) {
  const PREC = {
    unary: 7,
    binary_mult: 6,
    binary_add: 5,
    binary_ord: 4,
    binary_comp: 3,
    binary_and: 2,
    binary_or: 1,
    grit_metavariable: 100,
    attribute: 10,

    // if possible prefer string_literals to quoted templates
    string_lit: 2,
    quoted_template: 1,
  };
  return grammar({
    name: dialect,

    externals: ($) => [
      $.quoted_template_start,
      $.quoted_template_end,
      $._template_literal_chunk,
      $.template_interpolation_start,
      $.template_interpolation_end,
      $.template_directive_start,
      $.template_directive_end,
      $.heredoc_identifier,
      $._shim,
    ],

    extras: ($) => [$.comment, $._whitespace],

    conflicts: ($) => [
      // The following conflicts are all due to the insertion of the 'grit_metavariable' rule
      [$.identifier, $.object_elem],
      [$.identifier, $._expr_term, $._literal_value],
      [$._object_elems, $.object_elem],
      [$.identifier, $._expr_term, $._literal_value, $.object_elem],
      [$.body, $.identifier],
    ],

    rules: {
      // also allow objects to handle .tfvars in json format
      config_file: ($) => optional(field('body', $.body)),

      body: ($) =>
        choice(
          $._shim,
          seq(
            optional($._shim),
            repeat1(field('body', choice($.attribute, $.block)))
          ),
        ),

      attribute: ($) => prec(PREC.attribute, choice(
        seq(field('name', $.identifier), '=', field('value', $.expression))
      )),

      block: ($) =>
        seq(
          field('type', $.identifier),
          field('labels', repeat(choice($.string_lit, $.identifier))),
          $.block_start,
          optional(field('body', choice(seq(optional($._shim), $.grit_metavariable), $.body))),
          $.block_end,
        ),

      block_start: ($) => '{',
      block_end: ($) => '}',

      identifier: ($) =>
        choice(
          $.grit_metavariable,
          token(seq(choice(/\p{ID_Start}/, '_'), repeat(choice(/\p{ID_Continue}/, '-')))),
        ),

      expression: ($) => prec.right(field('expression', choice($._expr_term, $.conditional))),

      // operations are documented as expressions, but our real world samples
      // contain instances of operations without parentheses. think for example:
      // x = a == "" && b != ""
      _expr_term: ($) =>
        choice(
          $.grit_metavariable,
          $._literal_value,
          $._template_expr,
          $._collection_value,
          $.variable_expr,
          $.function_call,
          $._for_expr,
          $.operation,
          seq($._expr_term, $._index),
          seq($._expr_term, $.get_attr),
          seq($._expr_term, $._splat),
          seq('(', $.expression, ')'),
        ),

      _literal_value: ($) => choice($.grit_metavariable, $.numeric_lit, $.bool_lit, $.null_lit, $.string_lit),

      numeric_lit: ($) => choice(/[0-9]+(\.[0-9]+([eE][-+]?[0-9]+)?)?/, /0x[0-9a-zA-Z]+/),

      bool_lit: ($) => choice('true', 'false'),

      null_lit: ($) => 'null',

      string_lit: ($) =>
        prec(
          PREC.string_lit,
          seq(
            $.quoted_template_start,
            field('content', optional(choice($.grit_metavariable, $.template_literal))),
            $.quoted_template_end,
          ),
        ),

      _collection_value: ($) => choice($.tuple, $.object),

      _comma: ($) => ',',

      tuple: ($) => seq($.tuple_start, field('elements', optional($._tuple_elems)), $.tuple_end),

      tuple_start: ($) => '[',
      tuple_end: ($) => ']',

      _tuple_elems: ($) =>
        seq($.expression, repeat(seq($._comma, $.expression)), optional($._comma)),

      object: ($) =>
        seq($.object_start, field('elements', optional($._object_elems)), $.object_end),

      object_start: ($) => '{',
      object_end: ($) => '}',

      _object_elems: ($) =>
        choice(
          $.grit_metavariable,
          seq($.object_elem, repeat(seq(optional($._comma), $.object_elem)), optional($._comma))
        ),

      object_elem: ($) =>
        choice(
          $.grit_metavariable,
          seq(field('key', $.expression), choice('=', ':'), field('val', $.expression)),
        ),

      _index: ($) => choice($.new_index, $.legacy_index),

      new_index: ($) => seq('[', field('index', $.expression), ']'),
      legacy_index: ($) => seq('.', /[0-9]+/),

      get_attr: ($) => seq('.', field('key', $.identifier)),

      _splat: ($) => choice($.attr_splat, $.full_splat),

      attr_splat: ($) =>
        prec.right(seq('.*', field('indices', repeat(choice($.get_attr, $._index))))),

      full_splat: ($) =>
        prec.right(seq('[*]', field('indices', repeat(choice($.get_attr, $._index))))),

      _for_expr: ($) => choice($.for_tuple_expr, $.for_object_expr),

      for_tuple_expr: ($) =>
        seq(
          $.tuple_start,
          field('introduction', $.for_intro),
          field('value', $.expression),
          field('condition', optional($.for_cond)),
          $.tuple_end,
        ),

      for_object_expr: ($) =>
        seq(
          $.object_start,
          field('introduction', $.for_intro),
          field('key', $.expression),
          '=>',
          field('value', $.expression),
          optional($.ellipsis),
          field('condition', optional($.for_cond)),
          $.object_end,
        ),

      for_intro: ($) =>
        seq(
          'for',
          field('index', $.identifier),
          optional(seq(',', field('value', $.identifier))),
          'in',
          field('collection', $.expression),
          ':',
        ),

      for_cond: ($) => seq('if', field('condition', $.expression)),

      variable_expr: ($) => prec.right($.identifier),

      function_call: ($) =>
        seq(
          field('name', $.identifier),
          $._function_call_start,
          field('arguments', optional($._function_arguments)),
          $._function_call_end,
        ),

      _function_call_start: ($) => '(',
      _function_call_end: ($) => ')',

      _function_arguments: ($) =>
        prec.right(
          seq(
            $.expression,
            repeat(seq($._comma, $.expression)),
            optional(choice($._comma, $.ellipsis)),
          ),
        ),

      ellipsis: ($) => token('...'),

      conditional: ($) =>
        prec.left(
          seq(
            field('if', $.expression),
            '?',
            field('then', $.expression),
            ':',
            field('else', $.expression),
          ),
        ),

      operation: ($) => choice($.unary_operation, $.binary_operation),

      minus: ($) => '-',
      not: ($) => '!',

      unary_operation: ($) =>
        prec.left(
          PREC.unary,
          seq(field('operator', choice($.minus, $.not)), field('expression', $._expr_term)),
        ),

      times: ($) => '*',
      divide: ($) => '/',
      modulo: ($) => '%',
      plus: ($) => '+',
      greater_than: ($) => '>',
      greater_than_or_equal: ($) => '>=',
      less_than: ($) => '<',
      less_than_or_equal: ($) => '<=',
      equal: ($) => '==',
      not_equal: ($) => '!=',
      and: ($) => '&&',
      or: ($) => '||',

      binary_operation: ($) => {
        const table = [
          [PREC.binary_mult, choice($.times, $.divide, $.modulo)],
          [PREC.binary_add, choice($.plus, $.minus)],
          [
            PREC.binary_ord,
            field(
              'operator',
              choice($.greater_than, $.greater_than_or_equal, $.less_than, $.less_than_or_equal),
            ),
          ],
          [PREC.binary_comp, choice($.equal, $.not_equal)],
          [PREC.binary_and, choice($.and)],
          [PREC.binary_or, choice($.or)],
        ];

        return choice(
          ...table.map(([precedence, operator]) =>
            prec.left(
              precedence,
              seq(
                field('left', $._expr_term),
                field('operator', operator),
                field('right', $._expr_term),
              ),
            ),
          ),
        );
      },

      _template_expr: ($) => choice($.quoted_template, $.heredoc_template),

      quoted_template: ($) =>
        prec(
          PREC.quoted_template,
          seq(
            $.quoted_template_start,
            field('content', optional($._template)),
            $.quoted_template_end,
          ),
        ),

      heredoc_template: ($) =>
        seq(
          $.heredoc_start,
          field('stopper', $.heredoc_identifier),
          field('content', optional($._template)),
          field('stop', $.heredoc_identifier),
        ),

      heredoc_start: ($) => choice('<<', '<<-'),

      strip_marker: ($) => '~',

      _template: ($) =>
        repeat1(choice($.template_interpolation, $.template_directive, $.template_literal)),

      template_literal: ($) => prec.right(repeat1($._template_literal_chunk)),

      template_interpolation: ($) =>
        seq(
          $.template_interpolation_start,
          optional($.strip_marker),
          optional($.expression),
          optional($.strip_marker),
          $.template_interpolation_end,
        ),

      template_directive: ($) => choice($.template_for, $.template_if),

      template_for: ($) => seq($.template_for_start, optional($._template), $.template_for_end),

      template_for_start: ($) =>
        seq(
          $.template_directive_start,
          optional($.strip_marker),
          'for',
          $.identifier,
          optional(seq(',', $.identifier)),
          'in',
          $.expression,
          optional($.strip_marker),
          $.template_directive_end,
        ),

      template_for_end: ($) =>
        seq(
          $.template_directive_start,
          optional($.strip_marker),
          'endfor',
          optional($.strip_marker),
          $.template_directive_end,
        ),

      template_if: ($) =>
        seq(
          $.template_if_intro,
          optional($._template),
          optional(seq($.template_else_intro, optional($._template))),
          $.template_if_end,
        ),

      template_if_intro: ($) =>
        seq(
          $.template_directive_start,
          optional($.strip_marker),
          'if',
          $.expression,
          optional($.strip_marker),
          $.template_directive_end,
        ),

      template_else_intro: ($) =>
        seq(
          $.template_directive_start,
          optional($.strip_marker),
          'else',
          optional($.strip_marker),
          $.template_directive_end,
        ),

      template_if_end: ($) =>
        seq(
          $.template_directive_start,
          optional($.strip_marker),
          'endif',
          optional($.strip_marker),
          $.template_directive_end,
        ),

      // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
      comment: ($) =>
        token(choice(seq('#', /.*/), seq('//', /.*/), seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/'))),

      _whitespace: ($) => token(/\s/),

      grit_metavariable: ($) => token(prec(PREC.grit_metavariable, choice("µ...", /µ[a-zA-Z_][a-zA-Z0-9_]*/))),
    },
  });
};

// Based on tree-sitter-c-sharp grammar
module.exports = grammar({
  name: 'c_sharp',

  externals: $ => [
    $._preproc_directive_trivia,
    $.string_start,
    $.string_content,
    $.string_end,
    $.verbatim_string_start,
    $.interpolation_start,
    $.interpolation_end,
    $.comment,
  ],

  extras: $ => [
    $.comment,
    /\s/
  ],

  rules: {
    // Base metavariable support
    identifier: $ => choice(
      $.grit_metavariable,
      /[A-Za-z_][A-Za-z0-9_]*/
    ),

    // Type system support
    type: $ => choice(
      $.grit_metavariable,
      $.simple_type,
      $.array_type,
      $.pointer_type,
      $.function_pointer_type,
      $.nullable_type,
      $.tuple_type,
      $.generic_type
    ),

    generic_type: $ => seq(
      field('type', choice($.identifier, $.qualified_name)),
      field('type_arguments', $.type_argument_list)
    ),

    type_argument_list: $ => seq(
      '<',
      choice($.grit_metavariable, commaSep1($.type)),
      '>'
    ),

    // Method and property declarations
    method_declaration: $ => seq(
      optional($.attributes),
      repeat($.modifier),
      field('type', $.type),
      field('name', choice($.identifier, $.grit_metavariable)),
      field('parameters', $.parameter_list),
      choice(
        field('body', $.block),
        ';',
        '=>',
        field('expression_body', $._expression)
      )
    ),

    property_declaration: $ => seq(
      optional($.attributes),
      repeat($.modifier),
      field('type', $.type),
      field('name', choice($.identifier, $.grit_metavariable)),
      choice(
        seq(
          '{',
          optional($.accessor_list),
          '}'
        ),
        seq(
          '=>',
          field('expression', $._expression),
          ';'
        )
      )
    ),

    // Expression support
    _expression: $ => choice(
      $.grit_metavariable,
      $.binary_expression,
      $.assignment_expression,
      $.conditional_expression,
      $.unary_expression,
      $.cast_expression,
      $.await_expression,
      $.primary_expression,
      $.lambda_expression,
      $.query_expression,
      $.is_pattern_expression,
      $.grit_metavariable
    ),

    // LINQ support
    query_expression: $ => seq(
      'from',
      field('range_variable', choice($.identifier, $.grit_metavariable)),
      'in',
      field('collection', choice($._expression, $.grit_metavariable)),
      repeat(choice(
        $.where_clause,
        $.join_clause,
        $.let_clause,
        $.group_clause,
        $.select_clause,
        $.orderby_clause
      ))
    ),

    where_clause: $ => seq(
      'where',
      field('condition', choice($._expression, $.grit_metavariable))
    ),

    group_clause: $ => seq(
      'group',
      field('element', choice($._expression, $.grit_metavariable)),
      'by',
      field('key', choice($._expression, $.grit_metavariable))
    ),

    // Statement support
    statement: $ => choice(
      $.grit_metavariable,
      $.block,
      $.break_statement,
      $.checked_statement,
      $.continue_statement,
      $.do_statement,
      $.empty_statement,
      $.expression_statement,
      $.fixed_statement,
      $.for_statement,
      $.foreach_statement,
      $.goto_statement,
      $.if_statement,
      $.labeled_statement,
      $.local_declaration_statement,
      $.local_function_statement,
      $.lock_statement,
      $.return_statement,
      $.switch_statement,
      $.throw_statement,
      $.try_statement,
      $.unsafe_statement,
      $.using_statement,
      $.while_statement,
      $.yield_statement
    ),

    // Attribute support
    attributes: $ => repeat1($.attribute_list),
    
    attribute_list: $ => seq(
      '[',
      choice(
        $.grit_metavariable,
        commaSep1($.attribute)
      ),
      ']'
    ),

    // Base metavariable rule
    grit_metavariable: $ => /\$[A-Za-z_][A-Za-z0-9_]*/,

    // C# 9.0+ top-level statements
    compilation_unit: $ => choice(
      $.grit_metavariable,
      seq(
        repeat(choice(
          $.extern_alias_directive,
          $.using_directive
        )),
        repeat(choice(
          $.statement,
          $.declaration
        ))
      )
    ),

    // Init-only setters and properties
    accessor_declaration: $ => seq(
      optional($.attributes),
      repeat($.modifier),
      choice(
        'get',
        'set',
        'init',  // C# 9.0 init accessor
        'add',
        'remove'
      ),
      choice(
        field('body', $.block),
        ';',
        '=>',
        field('expression_body', $._expression)
      )
    ),

    // File-scoped namespaces
    file_scoped_namespace_declaration: $ => seq(
      'namespace',
      field('name', choice($.identifier, $.qualified_name, $.grit_metavariable)),
      ';',
      repeat($.declaration)
    ),

    // Target-typed new expressions
    object_creation_expression: $ => choice(
      $.grit_metavariable,
      seq(
        'new',
        optional(field('type', $.type)),  // Optional for target-typed new
        field('arguments', $.argument_list),
        optional(field('initializer', $.initializer_expression))
      )
    ),

    // Enhanced pattern matching
    pattern: $ => choice(
      $.grit_metavariable,
      $.constant_pattern,
      $.declaration_pattern,
      $.var_pattern,
      $.discard_pattern,
      $.recursive_pattern,
      $.property_pattern,
      $.relational_pattern,
      $.logical_pattern,
      $.parenthesized_pattern,
      $.list_pattern
    ),

    logical_pattern: $ => choice(
      seq($.pattern, 'and', $.pattern),
      seq($.pattern, 'or', $.pattern),
      seq('not', $.pattern)
    ),

    relational_pattern: $ => seq(
      choice('>', '<', '>=', '<=', '==', '!='),
      choice($._expression, $.grit_metavariable)
    ),

    property_pattern: $ => seq(
      '{',
      optional(commaSep1($.subpattern)),
      '}'
    ),

    subpattern: $ => choice(
      $.grit_metavariable,
      seq(
        field('name', choice(
          $.identifier,
          $.grit_metavariable,
          seq($.identifier, '.', $.identifier)  // Support nested properties
        )),
        ':',
        field('pattern', choice($.pattern, $._expression))
      )
    ),

    // Lambda expressions
    lambda_expression: $ => choice(
      $.grit_metavariable,
      seq(
        field('parameters', choice(
          $.identifier,
          $.parameter_list,
          seq('(', commaSep1(choice($.parameter, $.grit_metavariable)), ')'),
          $.grit_metavariable
        )),
        '=>',
        field('body', choice($._expression, $.block, $.grit_metavariable))
      )
    ),

    // Switch expressions
    switch_expression: $ => seq(
      field('value', choice($._expression, $.grit_metavariable)),
      'switch',
      '{',
      commaSep1($.switch_expression_arm),
      optional(','),
      '}'
    ),

    switch_expression_arm: $ => seq(
      field('pattern', choice($.pattern, $.grit_metavariable)),
      '=>',
      field('expression', choice($._expression, $.grit_metavariable))
    ),

    // List patterns
    list_pattern: $ => seq(
      '[',
      optional(seq(
        commaSep1(choice($.pattern, $.grit_metavariable)),
        optional('..')
      )),
      ']'
    ),

    // Record support
    record_declaration: $ => seq(
      optional($.attributes),
      repeat($.modifier),
      choice('record', 'record class', 'record struct'),
      field('name', choice($.identifier, $.grit_metavariable)),
      optional(field('type_parameters', $.type_parameter_list)),
      optional(field('parameters', $.parameter_list)),
      optional(seq(':', commaSep1($.base_list))),
      optional(field('constraints', $.type_parameter_constraints_clauses)),
      choice(
        field('body', $.declaration_list),
        ';'
      )
    ),

    // C# 10 global using directives
    global_using_directive: $ => seq(
      'global',
      $.using_directive
    ),

    // C# 10 file-scoped namespace with usings
    file_scoped_namespace_with_usings: $ => seq(
      repeat(choice($.using_directive, $.global_using_directive)),
      $.file_scoped_namespace_declaration
    ),

    // C# 10 extended property patterns
    property_pattern: $ => seq(
      '{',
      choice(
        $.grit_metavariable,
        commaSep1($.subpattern)
      ),
      '}'
    ),

    // C# 10 constant interpolated strings
    interpolated_string_expression: $ => choice(
      $.grit_metavariable,
      seq(
        '$"',
        repeat(choice(
          $.interpolated_string_text,
          $.interpolation
        )),
        '"'
      )
    ),

    // C# 11 raw string literals
    raw_string_literal: $ => choice(
      $.grit_metavariable,
      seq(
        '"""',
        repeat(choice($.raw_string_text, $.interpolation)),
        '"""'
      )
    ),

    // C# 11 list patterns
    list_pattern: $ => choice(
      $.grit_metavariable,
      seq(
        '[',
        commaSep1($.pattern),
        optional('..'),
        ']'
      )
    ),

    // C# 11 required members
    required_member: $ => seq(
      'required',
      choice(
        $.field_declaration,
        $.property_declaration,
        $.event_field_declaration
      )
    ),

    // C# 11 static abstract members in interfaces
    interface_member_declaration: $ => choice(
      $.grit_metavariable,
      seq(
        optional($.attributes),
        repeat($.modifier),
        choice(
          'static',
          'abstract'
        ),
        choice(
          $.method_declaration,
          $.property_declaration,
          $.event_declaration,
          $.indexer_declaration
        )
      )
    ),

    // C# 12 primary constructors
    class_declaration_with_primary_constructor: $ => seq(
      optional($.attributes),
      repeat($.modifier),
      'class',
      field('name', choice($.identifier, $.grit_metavariable)),
      field('constructor_parameters', $.parameter_list),
      optional(field('base_list', seq(':', commaSep1($.base_type)))),
      field('body', $.declaration_list)
    ),

    // C# 12 collection expressions
    collection_expression: $ => choice(
      $.grit_metavariable,
      seq(
        '[',
        optional(commaSep1($._expression)),
        optional('..'),
        ']'
      )
    ),

    // Enhanced LINQ support with more granular rules
    query_expression: $ => seq(
      'from',
      field('range_variable', choice($.identifier, $.grit_metavariable)),
      'in',
      field('collection', choice($._expression, $.grit_metavariable)),
      repeat(choice(
        $.where_clause,
        $.join_clause,
        $.let_clause,
        $.group_clause,
        $.select_clause,
        $.orderby_clause
      ))
    ),

    select_clause: $ => seq(
      'select',
      field('expression', choice(
        $._expression,
        $.anonymous_object_creation_expression,
        $.grit_metavariable
      ))
    ),

    anonymous_object_creation_expression: $ => seq(
      'new',
      '{',
      commaSep1($.anonymous_object_member_declaration),
      '}'
    ),

    anonymous_object_member_declaration: $ => choice(
      $.grit_metavariable,
      seq(
        optional(field('name', $.identifier)),
        '=',
        field('expression', $._expression)
      )
    ),

    // Pattern matching expressions
    is_pattern_expression: $ => seq(
      field('expression', choice($._expression, $.grit_metavariable)),
      'is',
      field('pattern', choice(
        $.constant_pattern,
        $.declaration_pattern,
        $.var_pattern,
        $.type_pattern,
        $.property_pattern,
        $.grit_metavariable
      ))
    ),

    declaration_pattern: $ => seq(
      field('type', choice($.type, $.grit_metavariable)),
      field('designation', choice($.identifier, $.grit_metavariable))
    ),

    type_pattern: $ => choice(
      $.type,
      $.grit_metavariable
    ),
  }
});

function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)))
} 
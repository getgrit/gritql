// Precedence is used by the parser to determine which rule to apply when there are two rules that can be applied.
// We use the PREC dict to globally define rule pprecidence
const PREC = {
  COMMENT: 1,
  STRING: 2,

  COMMA: -1,
  OBJECT: -1,
  USER_TYPE: 1,
  DECLARATION: 1,
  SOLIDITY: 1,
  ASSIGN: 0,
  PRAGMA_VALUE: 0,
  EVM_BUILTIN: 1,
  TERNARY: 1,
  TUPLE_EXPRESSION: 1,
  ARRAY_TYPE: 1,
  PARENTHESIZED_EXPRESSION: 2,
  OR: 2,
  AND: 3,
  DECLARATION_TUPLE: 3,
  CALL_ARGUMENT: 4,
  REL: 4,
  PLUS: 5,
  TIMES: 6,
  EXP: 7,
  MEMBER_EXPRESSION: 8,
  TYPEOF: 8,
  DELETE: 8,
  VOID: 8,
  NOT: 9,
  PRAGMA_TOKEN: 10,
  HEX_NUMBER: 10,
  NEG: 10,
  INC: 11,
  CALL: 12,
  NEW: 13,
  REVERT: 14,
  MEMBER: 1,
  GRIT_METAVARIABLE: 100,
};

// The following is the core grammar for Solidity. It accepts Solidity smart contracts between the versions 0.4.x and 0.7.x.
module.exports = grammar({
  name: 'solidity',

  // Extras is an array of tokens that is allowed anywhere in the document.
  extras: ($) => [
    // Allow comments to be placed anywhere in the file
    $.comment,
    // Allow characters such as whitespaces to be placed anywhere in the file
    /[\s\uFEFF\u2060\u200B\u00A0]/,
  ],

  // The word token allows tree-sitter to appropriately handle scenario's where an identifier includes a keyword.
  // Documentation: https://tree-sitter.github.io/tree-sitter/creating-parsers#keywords
  word: ($) => $._identifier,

  conflicts: ($) => [
    // The following conflicts are all due to the insertion of the 'grit_metavariable' rule
    [$._expression, $.identifier],
    [$.contract_body, $.identifier],
    [$._statement, $.identifier, $._expression],
    [$._statement, $.identifier],
    [$._contract_member, $.identifier],
    [$._type_name, $.identifier],
    [$._contract_member, $._type_name, $.identifier],
    [$._expression, $._statement, $._type_name, $.identifier],
    [$._statement, $._type_name, $.identifier],
    [$._expression, $._type_name, $.identifier],

    // The following conflicts are all due to the array type and array access expression ambiguity
    [$._primary_expression, $._type_name],
    [$._primary_expression, $._identifier_path],
    [$._primary_expression, $.member_expression, $._identifier_path],
    [$.member_expression, $._identifier_path],

    // This is to deal with an ambiguity due to different revert styles
    [$._call_arguments, $.tuple_expression],

    [$._parameter_list, $.fallback_receive_definition],
    [$._primary_expression, $.type_cast_expression],
    [$.pragma_value, $._solidity],
    [$.variable_declaration_tuple, $.tuple_expression],

    [$._yul_expression, $.yul_assignment],
    // Ambiguity: identifier ':'
    [$.yul_label, $.yul_identifier],

    // This is to deal with ambiguities arising from different fallback styles
    [$.fallback_receive_definition, $.function_type],
  ],

  rules: {
    //  -- [ Program ] --
    source_file: ($) => seq(field('source_unit', repeat($._source_unit))),

    //  -- [ Source Element ] --
    _source_unit: ($) => choice($._directive, $._declaration),

    //  -- [ Directives ] --
    _directive: ($) => choice($.pragma_directive, $.import_directive),

    // Pragma
    pragma_directive: ($) =>
      seq(
        'pragma',
        field('pragma', choice($.solidity_pragma_token, $.any_pragma_token)),
        $._semicolon,
      ),

    solidity_pragma_token: ($) =>
      prec(
        PREC.PRAGMA_TOKEN,
        seq(
          $._solidity,
          repeat(
            seq(
              field('version_constraint', $._pragma_version_constraint),
              optional(choice('||', '-')),
            ),
          ),
        ),
      ),

    any_pragma_token: ($) => seq(field('name', $.identifier), field('value', $.pragma_value)),

    _solidity: ($) => prec(PREC.SOLIDITY, 'solidity'),
    pragma_value: ($) => prec(PREC.PRAGMA_VALUE, /[^;]+/),

    _pragma_version_constraint: ($) =>
      seq(
        optional(field('operator', $.solidity_version_comparison_operator)),
        field('version', $.solidity_version),
      ),
    solidity_version: ($) => /"?\.? ?(\d|\*)+(\. ?(\d|\*)+ ?(\.(\d|\*)+)?)?"?/,

    less_than_or_equal: ($) => '<=',
    less_than: ($) => '<',
    caret: ($) => '^',
    greater_than: ($) => '>',
    greater_than_or_equal: ($) => '>=',
    tilde: ($) => '~',
    equal: ($) => '=',

    solidity_version_comparison_operator: ($) =>
      choice(
        $.less_than_or_equal,
        $.less_than,
        $.caret,
        $.greater_than,
        $.greater_than_or_equal,
        $.tilde,
        $.equal,
      ),

    // Import
    import_directive: ($) =>
      seq('import', choice($._source_import, seq($._import_clause, $._from_clause)), $._semicolon),

    _source_import: ($) => seq(field('source', $.string), optional($._import_alias)),

    _import_clause: ($) => choice($._single_import, $._multiple_import),

    _from_clause: ($) => seq('from', field('source', $.string)),

    _single_import: ($) =>
      seq(choice('*', field('import_name', $.identifier)), optional($._import_alias)),

    _multiple_import: ($) => seq('{', commaSep($._import_declaration), '}'),

    _import_declaration: ($) => seq(field('import_name', $.identifier), optional($._import_alias)),

    _import_alias: ($) => seq('as', field('alias', $.identifier)),

    //  -- [ Declarations ] --
    _declaration: ($) =>
      choice(
        $.contract_declaration,
        $.interface_declaration,
        $.error_declaration,
        $.library_declaration,
        $.struct_declaration,
        $.enum_declaration,
        $.function_definition,
        $.constant_variable_declaration,
        $.user_defined_type_definition,
      ),

    user_defined_type_definition: ($) =>
      seq(
        'type',
        field('name', $.identifier),
        'is',
        field('type', $._primitive_type),
        $._semicolon,
      ),

    constant_variable_declaration: ($) =>
      seq(
        field('type', $._type_name),
        'constant',
        field('name', $.identifier),
        '=',
        field('value', $._expression),
        $._semicolon,
      ),

    // Contract Declarations
    contract_declaration: ($) =>
      seq(
        optional('abstract'),
        'contract',
        field('name', $.identifier),
        optional($._class_heritage),
        field('body', $.contract_body),
      ),

    error_declaration: ($) =>
      seq(
        'error',
        field('name', $.identifier),
        '(',
        commaSep(field('parameter', $.error_parameter)),
        ')',
        $._semicolon,
      ),

    error_parameter: ($) => seq(field('type', $._type_name), field('name', optional($.identifier))),

    interface_declaration: ($) =>
      seq(
        'interface',
        field('name', $.identifier),
        optional($._class_heritage),
        field('body', $.contract_body),
      ),

    library_declaration: ($) =>
      seq('library', field('name', $.identifier), field('body', $.contract_body)),

    _class_heritage: ($) => seq('is', commaSep1(field('heritage', $.inheritance_specifier))),

    inheritance_specifier: ($) =>
      seq(field('ancestor', $.user_defined_type), optional($._call_arguments)),

    contract_body: ($) => seq('{', field('body', repeat($._contract_member)), '}'),

    _contract_member: ($) =>
      choice(
        $.grit_metavariable,
        $.function_definition,
        $.modifier_definition,
        $.error_declaration,
        $.state_variable_declaration,
        $.struct_declaration,
        $.enum_declaration,
        $.event_definition,
        $.using_directive,
        $.constructor_definition,
        $.fallback_receive_definition,
        $.user_defined_type_definition,
      ),

    struct_declaration: ($) =>
      seq(
        'struct',
        field('name', $.identifier),
        '{',
        repeat1(field('members', $.struct_member)),
        '}',
      ),

    struct_member: ($) =>
      seq(field('type', $._type_name), field('name', $.identifier), $._semicolon),

    enum_declaration: ($) =>
      seq(
        'enum',
        field('name', $.identifier),
        '{',
        commaSep(field('values', alias($.identifier, $.enum_value))),
        '}',
      ),

    event_definition: ($) =>
      seq(
        'event',
        field('name', $.identifier),
        $._event_parameter_list,
        optional('anonymous'),
        $._semicolon,
      ),

    _event_parameter_list: ($) => seq('(', commaSep(field('parameter', $.event_paramater)), ')'),

    event_paramater: ($) =>
      seq(field('type', $._type_name), optional('indexed'), optional(field('name', $.identifier))),

    using_directive: ($) =>
      seq(
        'using',
        field('module', alias($.user_defined_type, $.type_alias)),
        'for',
        field('source', choice($.any_source_type, $._type_name)),
        $._semicolon,
      ),

    any_source_type: ($) => '*',

    // -- [ Statements ] --
    _statement: ($) =>
      choice(
        $.grit_metavariable,
        $.block_statement,
        $.expression_statement,
        $.variable_declaration_statement,
        $.if_statement,
        $.for_statement,
        $.while_statement,
        $.do_while_statement,
        $.continue_statement,
        $.break_statement,
        $.try_statement,
        $.return_statement,
        $.emit_statement,
        $.assembly_statement,
        $.revert_statement,
      ),

    assembly_statement: ($) =>
      seq('assembly', optional('"evmasm"'), '{', field('yul', repeat($._yul_statement)), '}'),

    // -- [ Yul ] --
    _yul_statement: ($) =>
      choice(
        $.yul_block,
        $.yul_variable_declaration,
        $.yul_assignment,
        $.yul_function_call,
        $.yul_if_statement,
        $.yul_for_statement,
        $.yul_switch_statement,
        $.yul_leave,
        $.yul_break,
        $.yul_continue,
        $.yul_function_definition,
        $.yul_label,
        $._yul_literal,
      ),

    yul_label: ($) => seq(field('identifier', $.identifier), ':'),
    yul_leave: ($) => 'leave',
    yul_break: ($) => 'break',
    yul_continue: ($) => 'continue',

    yul_identifier: ($) => field('identifier', $.identifier), ///[a-zA-Z$_]+/,
    _yul_expression: ($) => choice($.yul_path, $.yul_function_call, $._yul_literal),
    yul_path: ($) => prec.left(dotSep1(field('path', $.yul_identifier))),

    // -- Yul Literals --
    _yul_literal: ($) =>
      choice($.yul_decimal_number, $.yul_string_literal, $.yul_hex_number, $.yul_boolean),
    yul_decimal_number: ($) => /0|([1-9][0-9]*)/,
    yul_string_literal: ($) => field('string', $.string),
    yul_hex_number: ($) => /0x[0-9A-Fa-f]*/,
    yul_boolean: ($) => choice('true', 'false'),

    // -- Yul Statements --
    yul_block: ($) => seq('{', field('content', repeat($._yul_statement)), '}'),
    yul_variable_declaration: ($) =>
      prec.left(
        PREC.DECLARATION,
        choice(
          seq(
            'let',
            field('left', $.yul_identifier),
            optional(seq(':=', field('right', $._yul_expression))),
          ),
          seq(
            'let',
            field(
              'left',
              choice(commaSep1($.yul_identifier), seq('(', commaSep1($.yul_identifier), ')')),
            ),
            optional(seq(':=', field('right', $.yul_function_call))),
          ),
        ),
      ),
    _yul_assignment_operator: ($) => choice(':=', seq(':', '=')),
    yul_assignment: ($) =>
      prec.left(
        PREC.ASSIGN,
        choice(
          seq(
            field('path', $.yul_path),
            $._yul_assignment_operator,
            field('value', $._yul_expression),
          ),
          seq(
            commaSep1(field('path', $.yul_path)),
            optional(seq($._yul_assignment_operator, field('value', $.yul_function_call))),
          ),
        ),
      ),
    yul_function_call: ($) =>
      choice(
        seq(
          field('function', choice($.yul_identifier, $.yul_evm_builtin)),
          '(',
          commaSep(field('parameters', $._yul_expression)),
          ')',
        ),
        field('function', $.yul_evm_builtin),
      ),
    yul_if_statement: ($) =>
      seq('if', field('condition', $._yul_expression), field('body', $.yul_block)),
    yul_for_statement: ($) =>
      seq(
        'for',
        field('declaration', $.yul_block),
        field('condition', $._yul_expression),
        field('increment', $.yul_block),
        field('body', $.yul_block),
      ),
    yul_switch_statement: ($) =>
      seq(
        'switch',
        field('condition', $._yul_expression),
        choice(
          seq('default', field('default', $.yul_block)),
          seq(
            repeat1(field('case', $.yul_case_clause)),
            optional(seq('default', field('default', $.yul_block))),
          ),
        ),
      ),

    yul_case_clause: ($) =>
      seq('case', field('condition', $._yul_literal), field('body', $.yul_block)),

    yul_function_definition: ($) =>
      seq(
        'function',
        field('name', $.yul_identifier),
        '(',
        commaSep(field('parameters', $.yul_identifier)),
        ')',
        optional(seq('->', commaSep1(field('returns', $.yul_identifier)))),
        field('body', $.yul_block),
      ),

    yul_evm_builtin: ($) =>
      prec(
        PREC.EVM_BUILTIN,
        choice(
          'stop',
          'add',
          'sub',
          'mul',
          'div',
          'sdiv',
          'mod',
          'smod',
          'exp',
          'not',
          'lt',
          'gt',
          'slt',
          'sgt',
          'eq',
          'iszero',
          'and',
          'or',
          'xor',
          'byte',
          'shl',
          'shr',
          'sar',
          'addmod',
          'mulmod',
          'signextend',
          'keccak256',
          'pop',
          'mload',
          'mstore',
          'mstore8',
          'sload',
          'sstore',
          'msize',
          'gas',
          'address',
          'balance',
          'selfbalance',
          'caller',
          'callvalue',
          'calldataload',
          'calldatasize',
          'calldatacopy',
          'extcodesize',
          'extcodecopy',
          'returndatasize',
          'returndatacopy',
          'extcodehash',
          'create',
          'create2',
          'call',
          'callcode',
          'delegatecall',
          'staticcall',
          'return',
          'revert',
          'selfdestruct',
          'invalid',
          'log0',
          'log1',
          'log2',
          'log3',
          'log4',
          'chainid',
          'origin',
          'gasprice',
          'blockhash',
          'coinbase',
          'timestamp',
          'number',
          'difficulty',
          'gaslimit',
        ),
      ),

    // -- [ Statements ] --
    _unchecked: ($) => 'unchecked',
    block_statement: ($) =>
      seq(optional($._unchecked), '{', repeat(field('content', $._statement)), '}'),
    variable_declaration_statement: ($) =>
      prec(
        PREC.DECLARATION,
        seq(
          choice(
            seq(
              field('declaration', $.variable_declaration),
              optional(seq('=', field('value', $._expression))),
            ),
            seq(
              field('declaration', $.variable_declaration_tuple),
              '=',
              field('value', $._expression),
            ),
          ),
          $._semicolon,
        ),
      ),

    memory_location: ($) => choice('memory', 'storage', 'calldata'),

    variable_declaration: ($) =>
      seq(
        field('type', $._type_name),
        optional(field('location', $.memory_location)),
        field('name', $.identifier),
      ),

    variable_declaration_tuple: ($) =>
      prec(
        PREC.DECLARATION_TUPLE,
        choice(
          seq('(', commaSep(optional(field('decleration', $.variable_declaration))), ')'),
          seq('var', '(', commaSep(optional(field('declaration', $.identifier))), ')'),
        ),
      ),

    expression_statement: ($) => seq(field('expression', $._expression), $._semicolon),

    if_statement: ($) =>
      prec.right(
        seq(
          'if',
          '(',
          field('condition', $._expression),
          ')',
          field('body', $._statement),
          optional(seq('else', field('else', $._statement))),
        ),
      ),

    for_statement: ($) =>
      seq(
        'for',
        '(',
        field(
          'initial',
          choice($.variable_declaration_statement, $.expression_statement, $._semicolon),
        ),
        field('condition', choice($.expression_statement, $._semicolon)),
        field('update', optional($._expression)),
        ')',
        field('body', $._statement),
      ),

    while_statement: ($) =>
      seq('while', '(', field('condition', $._expression), ')', field('body', $._statement)),
    do_while_statement: ($) =>
      seq(
        'do',
        field('body', $._statement),
        'while',
        '(',
        field('condition', $._expression),
        ')',
        $._semicolon,
      ),
    continue_statement: ($) => seq('continue', $._semicolon),
    break_statement: ($) => seq('break', $._semicolon),

    revert_statement: ($) =>
      prec(
        PREC.REVERT,
        seq(
          'revert',
          optional(field('error', $._expression)),
          optional(field('arguments', alias($._call_arguments, $.revert_arguments))),
          $._semicolon,
        ),
      ),

    try_statement: ($) =>
      seq(
        'try',
        field('attempt', $._expression),
        optional(seq('returns', $._parameter_list)),
        field('body', $.block_statement),
        repeat1(field('catch', $.catch_clause)),
      ),

    catch_clause: ($) =>
      seq(
        'catch',
        optional(seq(field('name', optional($.identifier)), $._parameter_list)),
        field('body', $.block_statement),
      ),

    return_statement: ($) => seq('return', field('value', optional($._expression)), $._semicolon),

    emit_statement: ($) =>
      seq('emit', field('name', $._expression), $._call_arguments, $._semicolon),

    //  -- [ Definitions ] --

    // Definitions
    state_variable_declaration: ($) =>
      seq(
        choice(
          $.grit_metavariable,
          seq(
            field('type', $._type_name),
            repeat(
              field(
                'modifier',
                choice(
                  $.visibility, // FIXME: this also allows external
                  'constant',
                  $.override_specifier,
                  $.immutable,
                ),
              ),
            ),
            field('name', $.identifier),
          ),
        ),
        optional(seq('=', field('value', $._expression))),
        $._semicolon,
      ),
    visibility: ($) => choice('public', 'internal', 'private', 'external'),

    state_mutability: ($) => choice('pure', 'view', 'payable'),

    immutable: ($) => 'immutable',

    override_specifier: ($) =>
      seq('override', optional(seq('(', commaSep1(field('type', $.user_defined_type)), ')'))),

    modifier_definition: ($) =>
      seq(
        'modifier',
        field('name', $.identifier),
        optional($._parameter_list),
        field('inheritence', repeat(choice($.virtual, $.override_specifier))),
        choice($._semicolon, field('body', $.function_body)),
      ),

    payable: ($) => 'payable',
    internal: ($) => 'internal',
    public: ($) => 'public',

    constructor_definition: ($) =>
      seq(
        'constructor',
        $._parameter_list,
        field(
          'visibility',
          repeat(choice($.modifier_invocation, $.payable, choice($.internal, $.public))),
        ),
        field('body', $.function_body),
      ),

    // this grammar is seriously messed up
    fallback_receive_definition: ($) =>
      seq(
        field(
          'thing',
          choice(
            seq(
              // optional("function"),
              choice('fallback', 'receive', 'function'),
            ),
            'function',
          ),
        ),
        // #todo: only fallback should get arguments
        $._parameter_list,
        // FIXME: We use repeat to allow for unorderedness. However, this means that the parser
        // accepts more than just the solidity language. The same problem exists for other definition rules.
        repeat(
          field(
            'modifier',
            choice(
              $.visibility,
              $.modifier_invocation,
              $.state_mutability,
              $.virtual,
              $.override_specifier,
            ),
          ),
        ),
        choice($._semicolon, field('body', $.function_body)),
      ),

    function_definition: ($) =>
      seq(
        'function',
        field('name', $.identifier),
        $._parameter_list,
        repeat(
          field(
            'modifier',
            choice(
              $.modifier_invocation,
              $.visibility,
              $.state_mutability,
              $.virtual,
              $.override_specifier,
            ),
          ),
        ),
        field('return_type', optional($.return_type_definition)),
        choice($._semicolon, field('body', $.function_body)),
      ),

    return_type_definition: ($) => seq('returns', $._parameter_list),

    virtual: ($) => 'virtual',
    modifier_invocation: ($) => seq($._identifier_path, optional($._call_arguments)),

    _call_arguments: ($) =>
      prec(PREC.CALL_ARGUMENT, seq('(', commaSep(field('arguments', $.call_argument)), ')')),

    call_argument: ($) =>
      choice(
        field('value', $._expression),
        seq('{', commaSep(field('value', $.call_struct_argument)), '}'),
      ),
    call_struct_argument: ($) =>
      seq(field('name', $.identifier), ':', field('value', $._expression)),

    function_body: ($) => seq('{', repeat(field('body', $._statement)), '}'),

    // Expressions
    _expression: ($) =>
      choice(
        $.grit_metavariable,
        $.binary_expression,
        $.unary_expression,
        $.update_expression,
        $.call_expression,
        // TODO: $.function_call_options_expression,
        $.payable_conversion_expression,
        $.meta_type_expression,
        $._primary_expression,
        $.struct_expression,
        $.ternary_expression,
        $.type_cast_expression,
      ),

    _primary_expression: ($) =>
      choice(
        $.parenthesized_expression,
        $.member_expression,
        $.array_access,
        $.slice_access,
        $._primitive_type,
        $.assignment_expression,
        $.augmented_assignment_expression,
        $.user_defined_type,
        $.tuple_expression,
        $.inline_array_expression,
        $.identifier,
        $._literal,
        $.new_expression,
      ),

    // TODO: back this up with official documentation
    type_cast_expression: ($) =>
      prec.left(
        seq(field('type', $._primitive_type), '(', field('expression', $._expression), ')'),
      ),

    ternary_expression: ($) =>
      prec.left(
        seq(
          field('if', $._expression),
          '?',
          field('then', $._expression),
          ':',
          field('else', $._expression),
        ),
      ),

    // TODO: make sure call arguments are part of solidity
    new_expression: ($) =>
      prec.left(seq('new', field('name', $._type_name), optional($._call_arguments))),

    tuple_expression: ($) =>
      prec(
        PREC.TUPLE_EXPRESSION,
        seq('(', commaSep(optional(field('values', $._expression))), ')'),
      ),

    inline_array_expression: ($) => seq('[', commaSep(field('values', $._expression)), ']'),

    and: ($) => '&&',
    or: ($) => '||',
    shift_right: ($) => '>>',
    shift_right_unsigned: ($) => '>>>',
    shift_left: ($) => '<<',
    bitwise_and: ($) => '&',
    bitwise_xor: ($) => '^',
    bitwise_or: ($) => '|',
    plus: ($) => '+',
    minus: ($) => '-',
    times: ($) => '*',
    divide: ($) => '/',
    modulo: ($) => '%',
    exponent: ($) => '**',
    less_than: ($) => '<',
    less_than_or_equal: ($) => '<=',
    equal: ($) => '==',
    not_equal: ($) => '!=',
    not_equal_strict: ($) => '!==',
    greater_than_or_equal: ($) => '>=',
    greater_than: ($) => '>',

    binary_expression: ($) =>
      choice(
        ...[
          [$.and, PREC.AND],
          [$.or, PREC.OR],
          [$.shift_right, PREC.TIMES],
          [$.shift_right_unsigned, PREC.TIMES],
          [$.shift_left, PREC.TIMES],
          [$.bitwise_and, PREC.AND],
          [$.bitwise_xor, PREC.OR],
          [$.bitwise_or, PREC.OR],
          [$.plus, PREC.PLUS],
          [$.minus, PREC.PLUS],
          [$.times, PREC.TIMES],
          [$.divide, PREC.TIMES],
          [$.modulo, PREC.TIMES],
          [$.exponent, PREC.EXP],
          [$.less_than, PREC.REL],
          [$.less_than_or_equal, PREC.REL],
          [$.equal, PREC.REL],
          [$.not_equal, PREC.REL],
          [$.not_equal_strict, PREC.REL],
          [$.greater_than_or_equal, PREC.REL],
          [$.greater_than, PREC.REL],
        ].map(([operator, precedence]) =>
          prec.left(
            precedence,
            seq(
              field('left', $._expression),
              field('operator', operator),
              field('right', $._expression),
            ),
          ),
        ),
      ),

    not: ($) => '!',
    negation: ($) => '~',
    negative: ($) => '-',
    positive: ($) => '+',
    delete: ($) => 'delete',

    unary_expression: ($) =>
      choice(
        ...[
          [$.not, PREC.NOT],
          [$.negation, PREC.NOT],
          [$.negative, PREC.NEG],
          [$.positive, PREC.NEG],
          [$.delete, PREC.DELETE],
        ].map(([operator, precedence]) =>
          prec.left(precedence, seq(field('operator', operator), field('argument', $._expression))),
        ),
      ),

    increment: ($) => '++',
    decrement: ($) => '--',

    update_expression: ($) =>
      prec.left(
        PREC.INC,
        choice(
          seq(
            field('argument', $._expression),
            field('operator', choice($.increment, $.decrement)),
          ),
          seq(
            field('operator', choice($.increment, $.decrement)),
            field('argument', $._expression),
          ),
        ),
      ),

    member_expression: ($) =>
      prec(
        PREC.MEMBER_EXPRESSION,
        seq(
          field('object', choice($._expression, $.identifier)),
          '.',
          field('property', $.identifier),
        ),
      ),

    array_access: ($) =>
      seq(field('base', $._expression), '[', optional(field('index', $._expression)), ']'),

    slice_access: ($) =>
      seq(
        field('base', $._expression),
        '[',
        optional(field('from', $._expression)),
        ':',
        optional(field('to', $._expression)),
        ']',
      ),

    struct_expression: ($) =>
      seq(
        field('type', $._expression),
        '{',
        commaSep(field('assignments', $.struct_field_assignment)),
        '}',
      ),

    struct_field_assignment: ($) =>
      seq(field('name', $.identifier), ':', field('value', $._expression)),

    parenthesized_expression: ($) =>
      prec(PREC.PARENTHESIZED_EXPRESSION, seq('(', field('expression', $._expression), ')')),

    assignment_expression: ($) =>
      prec.right(
        PREC.ASSIGN,
        seq(field('left', $._expression), '=', field('right', $._expression)),
      ),

    augmented_assignment: ($) =>
      choice('+=', '-=', '*=', '/=', '%=', '^=', '&=', '|=', '>>=', '>>>=', '<<='),

    augmented_assignment_expression: ($) =>
      prec.right(
        PREC.ASSIGN,
        seq(
          field('left', $._expression),
          field('operator', $.augmented_assignment),
          field('right', $._expression),
        ),
      ),

    call_expression: ($) =>
      prec.right(PREC.CALL, seq(field('function', $._expression), $._call_arguments)),

    payable_conversion_expression: ($) => seq('payable', $._call_arguments),
    meta_type_expression: ($) => seq('type', '(', field('type', $._type_name), ')'),

    _type_name: ($) =>
      choice(
        $.grit_metavariable,
        $._primitive_type,
        $.user_defined_type,
        $.mapping,
        $.array_type,
        $.function_type,
      ),

    array_type: ($) =>
      prec(
        PREC.ARRAY_TYPE,
        seq(field('type', $._type_name), '[', optional(field('expression', $._expression)), ']'),
      ),

    function_type: ($) =>
      prec.right(
        seq(
          'function',
          $._parameter_list,
          field('access', repeat(choice($.visibility, $.state_mutability))),
          optional($._return_parameters),
        ),
      ),

    _parameter_list: ($) => seq('(', field('parameters', commaSep($.parameter)), ')'),

    _return_parameters: ($) =>
      seq(
        'returns',
        '(',
        commaSep1(field('parameters', alias($.nameless_parameter, $.return_parameter))),
        ')',
      ),

    nameless_parameter: ($) =>
      seq(field('type', $._type_name), field('location', optional($.storage_location))),

    parameter: ($) =>
      seq(
        field('type', $._type_name),
        optional(field('location', $.storage_location)),
        optional(field('name', $.identifier)),
      ),

    storage_location: ($) => choice('memory', 'storage', 'calldata'),

    user_defined_type: ($) => $._identifier_path,

    _identifier_path: ($) => prec.left(dotSep1(field('path', $.identifier))),

    mapping: ($) =>
      seq(
        'mapping',
        '(',
        field('key_type', $._mapping_key),
        '=>',
        field('value_type', $._type_name),
        ')',
      ),

    _mapping_key: ($) => choice($._primitive_type, $.user_defined_type),

    address_type: ($) => seq('address', optional('payable')),
    bool_type: ($) => 'bool',
    string_type: ($) => 'string',
    var_type: ($) => 'var',

    _primitive_type: ($) =>
      prec.left(
        choice(
          $.address_type,
          $.bool_type,
          $.string_type,
          $.var_type,
          $.int,
          $.uint,
          $.bytes,
          $.fixed,
          $.ufixed,
        ),
      ),

    int: ($) =>
      choice(
        'int',
        'int8',
        'int16',
        'int24',
        'int32',
        'int40',
        'int48',
        'int56',
        'int64',
        'int72',
        'int80',
        'int88',
        'int96',
        'int104',
        'int112',
        'int120',
        'int128',
        'int136',
        'int144',
        'int152',
        'int160',
        'int168',
        'int176',
        'int184',
        'int192',
        'int200',
        'int208',
        'int216',
        'int224',
        'int232',
        'int240',
        'int248',
        'int256',
      ),
    uint: ($) =>
      choice(
        'uint',
        'uint8',
        'uint16',
        'uint24',
        'uint32',
        'uint40',
        'uint48',
        'uint56',
        'uint64',
        'uint72',
        'uint80',
        'uint88',
        'uint96',
        'uint104',
        'uint112',
        'uint120',
        'uint128',
        'uint136',
        'uint144',
        'uint152',
        'uint160',
        'uint168',
        'uint176',
        'uint184',
        'uint192',
        'uint200',
        'uint208',
        'uint216',
        'uint224',
        'uint232',
        'uint240',
        'uint248',
        'uint256',
      ),
    bytes: ($) =>
      choice(
        'byte',
        'bytes',
        'bytes1',
        'bytes2',
        'bytes3',
        'bytes4',
        'bytes5',
        'bytes6',
        'bytes7',
        'bytes8',
        'bytes9',
        'bytes10',
        'bytes11',
        'bytes12',
        'bytes13',
        'bytes14',
        'bytes15',
        'bytes16',
        'bytes17',
        'bytes18',
        'bytes19',
        'bytes20',
        'bytes21',
        'bytes22',
        'bytes23',
        'bytes24',
        'bytes25',
        'bytes26',
        'bytes27',
        'bytes28',
        'bytes29',
        'bytes30',
        'bytes31',
        'bytes32',
      ),

    fixed: ($) => choice('fixed', /fixed([0-9]+)x([0-9]+)/),
    ufixed: ($) => choice('ufixed', /ufixed([0-9]+)x([0-9]+)/),

    _semicolon: ($) => ';',

    identifier: ($) => choice($.grit_metavariable, $._identifier),
    _identifier: ($) => /[a-zA-Z$_][a-zA-Z0-9$_]*/,

    number: ($) => /\d+/,

    _literal: ($) =>
      choice(
        $.string_literal,
        $.number_literal,
        $.boolean_literal,
        $.hex_string_literal,
        $.unicode_string_literal,
      ),

    string_literal: ($) => prec.left(repeat1(field('string', $.string))),
    number_literal: ($) =>
      seq(
        field('number', choice($.decimal_number, $.hex_number)),
        optional(field('unit', $.number_unit)),
      ),
    decimal_number: ($) =>
      choice(/(\d|_)+(\.(\d|_)+)?([eE](-)?(\d|_)+)?/, /\.(\d|_)+([eE](-)?(\d|_)+)?/),
    hex_number: ($) => prec(PREC.HEX_NUMBER, /0[xX]([a-fA-F0-9][a-fA-F0-9]?_?)+/),
    // hex_number: $ => seq(/0[xX]/, optional(optionalDashSeparation($.hex_digit))),
    hex_digit: ($) => /([a-fA-F0-9][a-fA-F0-9])/,
    number_unit: ($) =>
      choice(
        'wei',
        'szabo',
        'finney',
        'gwei',
        'ether',
        'seconds',
        'minutes',
        'hours',
        'days',
        'weeks',
        'years',
      ),
    true: ($) => 'true',
    false: ($) => 'false',
    boolean_literal: ($) => choice($.true, $.false),

    hex_string_literal: ($) =>
      prec.left(
        repeat1(
          seq(
            'hex',
            choice(
              seq('"', optional(optionalDashSeparation($.hex_digit)), '"'),
              seq("'", optional(optionalDashSeparation($.hex_digit)), "'"),
            ),
          ),
        ),
      ),

    _escape_sequence: ($) =>
      token.immediate(
        seq(
          '\\',
          choice(/[^xu0-7]/, /[0-7]{1,3}/, /x[0-9a-fA-F]{2}/, /u[0-9a-fA-F]{4}/, /u{[0-9a-fA-F]+}/),
        ),
      ),
    _single_quoted_unicode_char: ($) => token.immediate(prec(PREC.STRING, /[^'\\\n]+|\\\r?\n/)),
    _double_quoted_unicode_char: ($) => token.immediate(prec(PREC.STRING, /[^"\\\n]+|\\\r?\n/)),
    unicode_string_literal: ($) =>
      prec.left(
        repeat1(
          seq(
            'unicode',
            choice(
              seq('"', repeat($._double_quoted_unicode_char), '"'),
              seq("'", repeat($._single_quoted_unicode_char), "'"),
            ),
          ),
        ),
      ),

    string: ($) =>
      choice(
        seq(
          '"',
          field(
            'content',
            repeat(choice($._string_immediate_elt_inside_double_quote, $._escape_sequence)),
          ),
          '"',
        ),
        seq(
          "'",
          field(
            'content',
            repeat(choice($._string_immediate_elt_inside_quote, $._escape_sequence)),
          ),
          "'",
        ),
      ),
    // We need to name those elts for ocaml-tree-sitter-semgrep.
    _string_immediate_elt_inside_double_quote: ($) =>
      token.immediate(prec(PREC.STRING, /[^"\\\n]+|\\\r?\n/)),
    _string_immediate_elt_inside_quote: ($) =>
      token.immediate(prec(PREC.STRING, /[^'\\\n]+|\\\r?\n/)),

    // Based on: https://github.com/tree-sitter/tree-sitter-c/blob/master/grammar.js#L965
    comment: ($) =>
      token(
        prec(
          PREC.COMMENT,
          choice(seq('//', /([^\r\n])*/), seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')),
        ),
      ),
    grit_metavariable: ($) => token(prec(PREC.GRIT_METAVARIABLE, choice("µ...", /µ[a-zA-Z_][a-zA-Z0-9_]*/))),
  },
});

function dotSep1(rule) {
  return seq(rule, repeat(seq('.', rule)));
}

function dotSep(rule) {
  return optional(dotSep1(rule));
}

function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)), optional(','));
}

function commaSep(rule) {
  return optional(commaSep1(rule));
}

function optionalDashSeparation(rule) {
  return seq(rule, repeat(seq(optional('_'), rule)));
}

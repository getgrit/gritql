#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 678
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 121
#define ALIAS_COUNT 0
#define TOKEN_COUNT 56
#define EXTERNAL_TOKEN_COUNT 8
#define FIELD_COUNT 2
#define MAX_ALIAS_SEQUENCE_LENGTH 10
#define PRODUCTION_ID_COUNT 2

enum ts_symbol_identifiers {
  anon_sym_EQ = 1,
  anon_sym_LBRACE = 2,
  anon_sym_RBRACE = 3,
  sym_identifier = 4,
  anon_sym_LPAREN = 5,
  anon_sym_RPAREN = 6,
  aux_sym_numeric_lit_token1 = 7,
  aux_sym_numeric_lit_token2 = 8,
  anon_sym_true = 9,
  anon_sym_false = 10,
  sym_null_lit = 11,
  anon_sym_COMMA = 12,
  anon_sym_LBRACK = 13,
  anon_sym_RBRACK = 14,
  anon_sym_COLON = 15,
  anon_sym_DOT = 16,
  aux_sym_legacy_index_token1 = 17,
  anon_sym_DOT_STAR = 18,
  anon_sym_LBRACK_STAR_RBRACK = 19,
  anon_sym_EQ_GT = 20,
  anon_sym_for = 21,
  anon_sym_in = 22,
  anon_sym_if = 23,
  sym_ellipsis = 24,
  anon_sym_QMARK = 25,
  anon_sym_DASH = 26,
  anon_sym_BANG = 27,
  anon_sym_STAR = 28,
  anon_sym_SLASH = 29,
  anon_sym_PERCENT = 30,
  anon_sym_PLUS = 31,
  anon_sym_GT = 32,
  anon_sym_GT_EQ = 33,
  anon_sym_LT = 34,
  anon_sym_LT_EQ = 35,
  anon_sym_EQ_EQ = 36,
  anon_sym_BANG_EQ = 37,
  anon_sym_AMP_AMP = 38,
  anon_sym_PIPE_PIPE = 39,
  anon_sym_LT_LT = 40,
  anon_sym_LT_LT_DASH = 41,
  sym_strip_marker = 42,
  anon_sym_endfor = 43,
  anon_sym_else = 44,
  anon_sym_endif = 45,
  sym_comment = 46,
  sym__whitespace = 47,
  sym_quoted_template_start = 48,
  sym_quoted_template_end = 49,
  sym__template_literal_chunk = 50,
  sym_template_interpolation_start = 51,
  sym_template_interpolation_end = 52,
  sym_template_directive_start = 53,
  sym_template_directive_end = 54,
  sym_heredoc_identifier = 55,
  sym_config_file = 56,
  sym_body = 57,
  sym_attribute = 58,
  sym_block = 59,
  sym_block_start = 60,
  sym_block_end = 61,
  sym_expression = 62,
  sym__expr_term = 63,
  sym_literal_value = 64,
  sym_numeric_lit = 65,
  sym_bool_lit = 66,
  sym_string_lit = 67,
  sym_collection_value = 68,
  sym__comma = 69,
  sym_tuple = 70,
  sym_tuple_start = 71,
  sym_tuple_end = 72,
  sym__tuple_elems = 73,
  sym_object = 74,
  sym_object_start = 75,
  sym_object_end = 76,
  sym__object_elems = 77,
  sym_object_elem = 78,
  sym_index = 79,
  sym_new_index = 80,
  sym_legacy_index = 81,
  sym_get_attr = 82,
  sym_splat = 83,
  sym_attr_splat = 84,
  sym_full_splat = 85,
  sym_for_expr = 86,
  sym_for_tuple_expr = 87,
  sym_for_object_expr = 88,
  sym_for_intro = 89,
  sym_for_cond = 90,
  sym_variable_expr = 91,
  sym_function_call = 92,
  sym__function_call_start = 93,
  sym__function_call_end = 94,
  sym_function_arguments = 95,
  sym_conditional = 96,
  sym_operation = 97,
  sym_unary_operation = 98,
  sym_binary_operation = 99,
  sym_template_expr = 100,
  sym_quoted_template = 101,
  sym_heredoc_template = 102,
  sym_heredoc_start = 103,
  aux_sym__template = 104,
  sym_template_literal = 105,
  sym_template_interpolation = 106,
  sym_template_directive = 107,
  sym_template_for = 108,
  sym_template_for_start = 109,
  sym_template_for_end = 110,
  sym_template_if = 111,
  sym_template_if_intro = 112,
  sym_template_else_intro = 113,
  sym_template_if_end = 114,
  aux_sym_body_repeat1 = 115,
  aux_sym_block_repeat1 = 116,
  aux_sym__tuple_elems_repeat1 = 117,
  aux_sym__object_elems_repeat1 = 118,
  aux_sym_attr_splat_repeat1 = 119,
  aux_sym_template_literal_repeat1 = 120,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_EQ] = "=",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [sym_identifier] = "identifier",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [aux_sym_numeric_lit_token1] = "numeric_lit_token1",
  [aux_sym_numeric_lit_token2] = "numeric_lit_token2",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [sym_null_lit] = "null_lit",
  [anon_sym_COMMA] = ",",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_COLON] = ":",
  [anon_sym_DOT] = ".",
  [aux_sym_legacy_index_token1] = "legacy_index_token1",
  [anon_sym_DOT_STAR] = ".*",
  [anon_sym_LBRACK_STAR_RBRACK] = "[*]",
  [anon_sym_EQ_GT] = "=>",
  [anon_sym_for] = "for",
  [anon_sym_in] = "in",
  [anon_sym_if] = "if",
  [sym_ellipsis] = "ellipsis",
  [anon_sym_QMARK] = "\?",
  [anon_sym_DASH] = "-",
  [anon_sym_BANG] = "!",
  [anon_sym_STAR] = "*",
  [anon_sym_SLASH] = "/",
  [anon_sym_PERCENT] = "%",
  [anon_sym_PLUS] = "+",
  [anon_sym_GT] = ">",
  [anon_sym_GT_EQ] = ">=",
  [anon_sym_LT] = "<",
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_AMP_AMP] = "&&",
  [anon_sym_PIPE_PIPE] = "||",
  [anon_sym_LT_LT] = "<<",
  [anon_sym_LT_LT_DASH] = "<<-",
  [sym_strip_marker] = "strip_marker",
  [anon_sym_endfor] = "endfor",
  [anon_sym_else] = "else",
  [anon_sym_endif] = "endif",
  [sym_comment] = "comment",
  [sym__whitespace] = "_whitespace",
  [sym_quoted_template_start] = "quoted_template_start",
  [sym_quoted_template_end] = "quoted_template_end",
  [sym__template_literal_chunk] = "_template_literal_chunk",
  [sym_template_interpolation_start] = "template_interpolation_start",
  [sym_template_interpolation_end] = "template_interpolation_end",
  [sym_template_directive_start] = "template_directive_start",
  [sym_template_directive_end] = "template_directive_end",
  [sym_heredoc_identifier] = "heredoc_identifier",
  [sym_config_file] = "config_file",
  [sym_body] = "body",
  [sym_attribute] = "attribute",
  [sym_block] = "block",
  [sym_block_start] = "block_start",
  [sym_block_end] = "block_end",
  [sym_expression] = "expression",
  [sym__expr_term] = "_expr_term",
  [sym_literal_value] = "literal_value",
  [sym_numeric_lit] = "numeric_lit",
  [sym_bool_lit] = "bool_lit",
  [sym_string_lit] = "string_lit",
  [sym_collection_value] = "collection_value",
  [sym__comma] = "_comma",
  [sym_tuple] = "tuple",
  [sym_tuple_start] = "tuple_start",
  [sym_tuple_end] = "tuple_end",
  [sym__tuple_elems] = "_tuple_elems",
  [sym_object] = "object",
  [sym_object_start] = "object_start",
  [sym_object_end] = "object_end",
  [sym__object_elems] = "_object_elems",
  [sym_object_elem] = "object_elem",
  [sym_index] = "index",
  [sym_new_index] = "new_index",
  [sym_legacy_index] = "legacy_index",
  [sym_get_attr] = "get_attr",
  [sym_splat] = "splat",
  [sym_attr_splat] = "attr_splat",
  [sym_full_splat] = "full_splat",
  [sym_for_expr] = "for_expr",
  [sym_for_tuple_expr] = "for_tuple_expr",
  [sym_for_object_expr] = "for_object_expr",
  [sym_for_intro] = "for_intro",
  [sym_for_cond] = "for_cond",
  [sym_variable_expr] = "variable_expr",
  [sym_function_call] = "function_call",
  [sym__function_call_start] = "_function_call_start",
  [sym__function_call_end] = "_function_call_end",
  [sym_function_arguments] = "function_arguments",
  [sym_conditional] = "conditional",
  [sym_operation] = "operation",
  [sym_unary_operation] = "unary_operation",
  [sym_binary_operation] = "binary_operation",
  [sym_template_expr] = "template_expr",
  [sym_quoted_template] = "quoted_template",
  [sym_heredoc_template] = "heredoc_template",
  [sym_heredoc_start] = "heredoc_start",
  [aux_sym__template] = "_template",
  [sym_template_literal] = "template_literal",
  [sym_template_interpolation] = "template_interpolation",
  [sym_template_directive] = "template_directive",
  [sym_template_for] = "template_for",
  [sym_template_for_start] = "template_for_start",
  [sym_template_for_end] = "template_for_end",
  [sym_template_if] = "template_if",
  [sym_template_if_intro] = "template_if_intro",
  [sym_template_else_intro] = "template_else_intro",
  [sym_template_if_end] = "template_if_end",
  [aux_sym_body_repeat1] = "body_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym__tuple_elems_repeat1] = "_tuple_elems_repeat1",
  [aux_sym__object_elems_repeat1] = "_object_elems_repeat1",
  [aux_sym_attr_splat_repeat1] = "attr_splat_repeat1",
  [aux_sym_template_literal_repeat1] = "template_literal_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [sym_identifier] = sym_identifier,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [aux_sym_numeric_lit_token1] = aux_sym_numeric_lit_token1,
  [aux_sym_numeric_lit_token2] = aux_sym_numeric_lit_token2,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [sym_null_lit] = sym_null_lit,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_DOT] = anon_sym_DOT,
  [aux_sym_legacy_index_token1] = aux_sym_legacy_index_token1,
  [anon_sym_DOT_STAR] = anon_sym_DOT_STAR,
  [anon_sym_LBRACK_STAR_RBRACK] = anon_sym_LBRACK_STAR_RBRACK,
  [anon_sym_EQ_GT] = anon_sym_EQ_GT,
  [anon_sym_for] = anon_sym_for,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_if] = anon_sym_if,
  [sym_ellipsis] = sym_ellipsis,
  [anon_sym_QMARK] = anon_sym_QMARK,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_BANG] = anon_sym_BANG,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_GT_EQ] = anon_sym_GT_EQ,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_AMP_AMP] = anon_sym_AMP_AMP,
  [anon_sym_PIPE_PIPE] = anon_sym_PIPE_PIPE,
  [anon_sym_LT_LT] = anon_sym_LT_LT,
  [anon_sym_LT_LT_DASH] = anon_sym_LT_LT_DASH,
  [sym_strip_marker] = sym_strip_marker,
  [anon_sym_endfor] = anon_sym_endfor,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_endif] = anon_sym_endif,
  [sym_comment] = sym_comment,
  [sym__whitespace] = sym__whitespace,
  [sym_quoted_template_start] = sym_quoted_template_start,
  [sym_quoted_template_end] = sym_quoted_template_end,
  [sym__template_literal_chunk] = sym__template_literal_chunk,
  [sym_template_interpolation_start] = sym_template_interpolation_start,
  [sym_template_interpolation_end] = sym_template_interpolation_end,
  [sym_template_directive_start] = sym_template_directive_start,
  [sym_template_directive_end] = sym_template_directive_end,
  [sym_heredoc_identifier] = sym_heredoc_identifier,
  [sym_config_file] = sym_config_file,
  [sym_body] = sym_body,
  [sym_attribute] = sym_attribute,
  [sym_block] = sym_block,
  [sym_block_start] = sym_block_start,
  [sym_block_end] = sym_block_end,
  [sym_expression] = sym_expression,
  [sym__expr_term] = sym__expr_term,
  [sym_literal_value] = sym_literal_value,
  [sym_numeric_lit] = sym_numeric_lit,
  [sym_bool_lit] = sym_bool_lit,
  [sym_string_lit] = sym_string_lit,
  [sym_collection_value] = sym_collection_value,
  [sym__comma] = sym__comma,
  [sym_tuple] = sym_tuple,
  [sym_tuple_start] = sym_tuple_start,
  [sym_tuple_end] = sym_tuple_end,
  [sym__tuple_elems] = sym__tuple_elems,
  [sym_object] = sym_object,
  [sym_object_start] = sym_object_start,
  [sym_object_end] = sym_object_end,
  [sym__object_elems] = sym__object_elems,
  [sym_object_elem] = sym_object_elem,
  [sym_index] = sym_index,
  [sym_new_index] = sym_new_index,
  [sym_legacy_index] = sym_legacy_index,
  [sym_get_attr] = sym_get_attr,
  [sym_splat] = sym_splat,
  [sym_attr_splat] = sym_attr_splat,
  [sym_full_splat] = sym_full_splat,
  [sym_for_expr] = sym_for_expr,
  [sym_for_tuple_expr] = sym_for_tuple_expr,
  [sym_for_object_expr] = sym_for_object_expr,
  [sym_for_intro] = sym_for_intro,
  [sym_for_cond] = sym_for_cond,
  [sym_variable_expr] = sym_variable_expr,
  [sym_function_call] = sym_function_call,
  [sym__function_call_start] = sym__function_call_start,
  [sym__function_call_end] = sym__function_call_end,
  [sym_function_arguments] = sym_function_arguments,
  [sym_conditional] = sym_conditional,
  [sym_operation] = sym_operation,
  [sym_unary_operation] = sym_unary_operation,
  [sym_binary_operation] = sym_binary_operation,
  [sym_template_expr] = sym_template_expr,
  [sym_quoted_template] = sym_quoted_template,
  [sym_heredoc_template] = sym_heredoc_template,
  [sym_heredoc_start] = sym_heredoc_start,
  [aux_sym__template] = aux_sym__template,
  [sym_template_literal] = sym_template_literal,
  [sym_template_interpolation] = sym_template_interpolation,
  [sym_template_directive] = sym_template_directive,
  [sym_template_for] = sym_template_for,
  [sym_template_for_start] = sym_template_for_start,
  [sym_template_for_end] = sym_template_for_end,
  [sym_template_if] = sym_template_if,
  [sym_template_if_intro] = sym_template_if_intro,
  [sym_template_else_intro] = sym_template_else_intro,
  [sym_template_if_end] = sym_template_if_end,
  [aux_sym_body_repeat1] = aux_sym_body_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym__tuple_elems_repeat1] = aux_sym__tuple_elems_repeat1,
  [aux_sym__object_elems_repeat1] = aux_sym__object_elems_repeat1,
  [aux_sym_attr_splat_repeat1] = aux_sym_attr_splat_repeat1,
  [aux_sym_template_literal_repeat1] = aux_sym_template_literal_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_numeric_lit_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_numeric_lit_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [sym_null_lit] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_legacy_index_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DOT_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK_STAR_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_for] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_in] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [sym_ellipsis] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_QMARK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_LT_DASH] = {
    .visible = true,
    .named = false,
  },
  [sym_strip_marker] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_endfor] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_endif] = {
    .visible = true,
    .named = false,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym__whitespace] = {
    .visible = false,
    .named = true,
  },
  [sym_quoted_template_start] = {
    .visible = true,
    .named = true,
  },
  [sym_quoted_template_end] = {
    .visible = true,
    .named = true,
  },
  [sym__template_literal_chunk] = {
    .visible = false,
    .named = true,
  },
  [sym_template_interpolation_start] = {
    .visible = true,
    .named = true,
  },
  [sym_template_interpolation_end] = {
    .visible = true,
    .named = true,
  },
  [sym_template_directive_start] = {
    .visible = true,
    .named = true,
  },
  [sym_template_directive_end] = {
    .visible = true,
    .named = true,
  },
  [sym_heredoc_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_config_file] = {
    .visible = true,
    .named = true,
  },
  [sym_body] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_block_start] = {
    .visible = true,
    .named = true,
  },
  [sym_block_end] = {
    .visible = true,
    .named = true,
  },
  [sym_expression] = {
    .visible = true,
    .named = true,
  },
  [sym__expr_term] = {
    .visible = false,
    .named = true,
  },
  [sym_literal_value] = {
    .visible = true,
    .named = true,
  },
  [sym_numeric_lit] = {
    .visible = true,
    .named = true,
  },
  [sym_bool_lit] = {
    .visible = true,
    .named = true,
  },
  [sym_string_lit] = {
    .visible = true,
    .named = true,
  },
  [sym_collection_value] = {
    .visible = true,
    .named = true,
  },
  [sym__comma] = {
    .visible = false,
    .named = true,
  },
  [sym_tuple] = {
    .visible = true,
    .named = true,
  },
  [sym_tuple_start] = {
    .visible = true,
    .named = true,
  },
  [sym_tuple_end] = {
    .visible = true,
    .named = true,
  },
  [sym__tuple_elems] = {
    .visible = false,
    .named = true,
  },
  [sym_object] = {
    .visible = true,
    .named = true,
  },
  [sym_object_start] = {
    .visible = true,
    .named = true,
  },
  [sym_object_end] = {
    .visible = true,
    .named = true,
  },
  [sym__object_elems] = {
    .visible = false,
    .named = true,
  },
  [sym_object_elem] = {
    .visible = true,
    .named = true,
  },
  [sym_index] = {
    .visible = true,
    .named = true,
  },
  [sym_new_index] = {
    .visible = true,
    .named = true,
  },
  [sym_legacy_index] = {
    .visible = true,
    .named = true,
  },
  [sym_get_attr] = {
    .visible = true,
    .named = true,
  },
  [sym_splat] = {
    .visible = true,
    .named = true,
  },
  [sym_attr_splat] = {
    .visible = true,
    .named = true,
  },
  [sym_full_splat] = {
    .visible = true,
    .named = true,
  },
  [sym_for_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_for_tuple_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_for_object_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_for_intro] = {
    .visible = true,
    .named = true,
  },
  [sym_for_cond] = {
    .visible = true,
    .named = true,
  },
  [sym_variable_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_function_call] = {
    .visible = true,
    .named = true,
  },
  [sym__function_call_start] = {
    .visible = false,
    .named = true,
  },
  [sym__function_call_end] = {
    .visible = false,
    .named = true,
  },
  [sym_function_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_conditional] = {
    .visible = true,
    .named = true,
  },
  [sym_operation] = {
    .visible = true,
    .named = true,
  },
  [sym_unary_operation] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_operation] = {
    .visible = true,
    .named = true,
  },
  [sym_template_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_quoted_template] = {
    .visible = true,
    .named = true,
  },
  [sym_heredoc_template] = {
    .visible = true,
    .named = true,
  },
  [sym_heredoc_start] = {
    .visible = true,
    .named = true,
  },
  [aux_sym__template] = {
    .visible = false,
    .named = false,
  },
  [sym_template_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_template_interpolation] = {
    .visible = true,
    .named = true,
  },
  [sym_template_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_template_for] = {
    .visible = true,
    .named = true,
  },
  [sym_template_for_start] = {
    .visible = true,
    .named = true,
  },
  [sym_template_for_end] = {
    .visible = true,
    .named = true,
  },
  [sym_template_if] = {
    .visible = true,
    .named = true,
  },
  [sym_template_if_intro] = {
    .visible = true,
    .named = true,
  },
  [sym_template_else_intro] = {
    .visible = true,
    .named = true,
  },
  [sym_template_if_end] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__tuple_elems_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__object_elems_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_attr_splat_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_template_literal_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_key = 1,
  field_val = 2,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_key] = "key",
  [field_val] = "val",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_key, 0},
    {field_val, 2},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 2,
  [4] = 2,
  [5] = 2,
  [6] = 2,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 9,
  [11] = 11,
  [12] = 9,
  [13] = 9,
  [14] = 9,
  [15] = 15,
  [16] = 16,
  [17] = 16,
  [18] = 16,
  [19] = 16,
  [20] = 16,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 21,
  [25] = 21,
  [26] = 23,
  [27] = 23,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 43,
  [46] = 46,
  [47] = 44,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 49,
  [52] = 50,
  [53] = 49,
  [54] = 54,
  [55] = 50,
  [56] = 49,
  [57] = 50,
  [58] = 54,
  [59] = 59,
  [60] = 43,
  [61] = 61,
  [62] = 62,
  [63] = 54,
  [64] = 64,
  [65] = 49,
  [66] = 61,
  [67] = 61,
  [68] = 61,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 54,
  [73] = 44,
  [74] = 74,
  [75] = 75,
  [76] = 44,
  [77] = 50,
  [78] = 54,
  [79] = 79,
  [80] = 42,
  [81] = 43,
  [82] = 44,
  [83] = 42,
  [84] = 42,
  [85] = 61,
  [86] = 86,
  [87] = 43,
  [88] = 42,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 95,
  [96] = 94,
  [97] = 97,
  [98] = 95,
  [99] = 99,
  [100] = 100,
  [101] = 95,
  [102] = 102,
  [103] = 102,
  [104] = 99,
  [105] = 100,
  [106] = 94,
  [107] = 107,
  [108] = 107,
  [109] = 97,
  [110] = 107,
  [111] = 107,
  [112] = 100,
  [113] = 99,
  [114] = 95,
  [115] = 97,
  [116] = 94,
  [117] = 102,
  [118] = 100,
  [119] = 102,
  [120] = 99,
  [121] = 100,
  [122] = 99,
  [123] = 97,
  [124] = 94,
  [125] = 107,
  [126] = 102,
  [127] = 97,
  [128] = 95,
  [129] = 37,
  [130] = 38,
  [131] = 36,
  [132] = 35,
  [133] = 34,
  [134] = 33,
  [135] = 32,
  [136] = 29,
  [137] = 89,
  [138] = 90,
  [139] = 91,
  [140] = 92,
  [141] = 141,
  [142] = 93,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 149,
  [150] = 150,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 157,
  [158] = 158,
  [159] = 159,
  [160] = 160,
  [161] = 161,
  [162] = 162,
  [163] = 163,
  [164] = 164,
  [165] = 165,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 169,
  [170] = 170,
  [171] = 171,
  [172] = 172,
  [173] = 173,
  [174] = 174,
  [175] = 141,
  [176] = 38,
  [177] = 36,
  [178] = 33,
  [179] = 37,
  [180] = 35,
  [181] = 29,
  [182] = 34,
  [183] = 32,
  [184] = 144,
  [185] = 150,
  [186] = 147,
  [187] = 148,
  [188] = 154,
  [189] = 145,
  [190] = 174,
  [191] = 143,
  [192] = 151,
  [193] = 152,
  [194] = 153,
  [195] = 159,
  [196] = 156,
  [197] = 158,
  [198] = 149,
  [199] = 170,
  [200] = 173,
  [201] = 161,
  [202] = 164,
  [203] = 155,
  [204] = 169,
  [205] = 167,
  [206] = 162,
  [207] = 172,
  [208] = 166,
  [209] = 171,
  [210] = 146,
  [211] = 160,
  [212] = 157,
  [213] = 165,
  [214] = 163,
  [215] = 168,
  [216] = 29,
  [217] = 34,
  [218] = 33,
  [219] = 37,
  [220] = 29,
  [221] = 93,
  [222] = 37,
  [223] = 91,
  [224] = 38,
  [225] = 36,
  [226] = 38,
  [227] = 36,
  [228] = 35,
  [229] = 35,
  [230] = 34,
  [231] = 89,
  [232] = 33,
  [233] = 32,
  [234] = 92,
  [235] = 90,
  [236] = 32,
  [237] = 92,
  [238] = 90,
  [239] = 91,
  [240] = 93,
  [241] = 93,
  [242] = 92,
  [243] = 89,
  [244] = 89,
  [245] = 91,
  [246] = 90,
  [247] = 141,
  [248] = 163,
  [249] = 148,
  [250] = 141,
  [251] = 146,
  [252] = 147,
  [253] = 151,
  [254] = 152,
  [255] = 153,
  [256] = 156,
  [257] = 158,
  [258] = 161,
  [259] = 164,
  [260] = 169,
  [261] = 160,
  [262] = 170,
  [263] = 141,
  [264] = 165,
  [265] = 167,
  [266] = 144,
  [267] = 143,
  [268] = 174,
  [269] = 145,
  [270] = 154,
  [271] = 173,
  [272] = 172,
  [273] = 171,
  [274] = 168,
  [275] = 166,
  [276] = 162,
  [277] = 159,
  [278] = 157,
  [279] = 155,
  [280] = 150,
  [281] = 149,
  [282] = 166,
  [283] = 165,
  [284] = 163,
  [285] = 167,
  [286] = 165,
  [287] = 149,
  [288] = 170,
  [289] = 160,
  [290] = 169,
  [291] = 164,
  [292] = 161,
  [293] = 173,
  [294] = 158,
  [295] = 172,
  [296] = 156,
  [297] = 143,
  [298] = 153,
  [299] = 171,
  [300] = 152,
  [301] = 168,
  [302] = 174,
  [303] = 166,
  [304] = 151,
  [305] = 162,
  [306] = 148,
  [307] = 147,
  [308] = 146,
  [309] = 145,
  [310] = 154,
  [311] = 159,
  [312] = 173,
  [313] = 172,
  [314] = 171,
  [315] = 168,
  [316] = 174,
  [317] = 145,
  [318] = 157,
  [319] = 162,
  [320] = 159,
  [321] = 157,
  [322] = 155,
  [323] = 155,
  [324] = 150,
  [325] = 153,
  [326] = 150,
  [327] = 156,
  [328] = 144,
  [329] = 158,
  [330] = 161,
  [331] = 164,
  [332] = 152,
  [333] = 151,
  [334] = 148,
  [335] = 147,
  [336] = 146,
  [337] = 149,
  [338] = 170,
  [339] = 160,
  [340] = 169,
  [341] = 143,
  [342] = 163,
  [343] = 144,
  [344] = 154,
  [345] = 167,
  [346] = 346,
  [347] = 347,
  [348] = 348,
  [349] = 349,
  [350] = 350,
  [351] = 351,
  [352] = 351,
  [353] = 353,
  [354] = 354,
  [355] = 353,
  [356] = 356,
  [357] = 356,
  [358] = 353,
  [359] = 359,
  [360] = 356,
  [361] = 361,
  [362] = 362,
  [363] = 363,
  [364] = 364,
  [365] = 365,
  [366] = 366,
  [367] = 367,
  [368] = 368,
  [369] = 365,
  [370] = 370,
  [371] = 366,
  [372] = 365,
  [373] = 367,
  [374] = 364,
  [375] = 362,
  [376] = 376,
  [377] = 370,
  [378] = 363,
  [379] = 368,
  [380] = 363,
  [381] = 361,
  [382] = 364,
  [383] = 367,
  [384] = 364,
  [385] = 365,
  [386] = 363,
  [387] = 370,
  [388] = 362,
  [389] = 368,
  [390] = 367,
  [391] = 391,
  [392] = 391,
  [393] = 391,
  [394] = 361,
  [395] = 367,
  [396] = 365,
  [397] = 376,
  [398] = 364,
  [399] = 376,
  [400] = 363,
  [401] = 366,
  [402] = 346,
  [403] = 348,
  [404] = 404,
  [405] = 405,
  [406] = 406,
  [407] = 407,
  [408] = 408,
  [409] = 408,
  [410] = 408,
  [411] = 411,
  [412] = 412,
  [413] = 413,
  [414] = 414,
  [415] = 408,
  [416] = 408,
  [417] = 417,
  [418] = 418,
  [419] = 419,
  [420] = 420,
  [421] = 421,
  [422] = 419,
  [423] = 423,
  [424] = 419,
  [425] = 423,
  [426] = 348,
  [427] = 419,
  [428] = 421,
  [429] = 429,
  [430] = 421,
  [431] = 431,
  [432] = 346,
  [433] = 419,
  [434] = 429,
  [435] = 435,
  [436] = 436,
  [437] = 437,
  [438] = 438,
  [439] = 439,
  [440] = 440,
  [441] = 441,
  [442] = 442,
  [443] = 443,
  [444] = 444,
  [445] = 423,
  [446] = 446,
  [447] = 447,
  [448] = 448,
  [449] = 438,
  [450] = 442,
  [451] = 451,
  [452] = 446,
  [453] = 453,
  [454] = 442,
  [455] = 441,
  [456] = 456,
  [457] = 438,
  [458] = 458,
  [459] = 448,
  [460] = 444,
  [461] = 461,
  [462] = 442,
  [463] = 443,
  [464] = 453,
  [465] = 447,
  [466] = 451,
  [467] = 458,
  [468] = 441,
  [469] = 429,
  [470] = 461,
  [471] = 471,
  [472] = 472,
  [473] = 442,
  [474] = 440,
  [475] = 475,
  [476] = 476,
  [477] = 477,
  [478] = 476,
  [479] = 441,
  [480] = 480,
  [481] = 481,
  [482] = 453,
  [483] = 471,
  [484] = 472,
  [485] = 475,
  [486] = 435,
  [487] = 441,
  [488] = 477,
  [489] = 435,
  [490] = 436,
  [491] = 439,
  [492] = 456,
  [493] = 436,
  [494] = 494,
  [495] = 495,
  [496] = 496,
  [497] = 497,
  [498] = 439,
  [499] = 456,
  [500] = 476,
  [501] = 501,
  [502] = 502,
  [503] = 503,
  [504] = 504,
  [505] = 440,
  [506] = 506,
  [507] = 507,
  [508] = 495,
  [509] = 472,
  [510] = 471,
  [511] = 346,
  [512] = 512,
  [513] = 423,
  [514] = 497,
  [515] = 515,
  [516] = 346,
  [517] = 517,
  [518] = 458,
  [519] = 451,
  [520] = 520,
  [521] = 497,
  [522] = 522,
  [523] = 447,
  [524] = 524,
  [525] = 429,
  [526] = 443,
  [527] = 527,
  [528] = 477,
  [529] = 496,
  [530] = 149,
  [531] = 531,
  [532] = 532,
  [533] = 533,
  [534] = 503,
  [535] = 444,
  [536] = 536,
  [537] = 495,
  [538] = 538,
  [539] = 475,
  [540] = 448,
  [541] = 496,
  [542] = 461,
  [543] = 543,
  [544] = 544,
  [545] = 503,
  [546] = 546,
  [547] = 446,
  [548] = 348,
  [549] = 348,
  [550] = 550,
  [551] = 551,
  [552] = 552,
  [553] = 553,
  [554] = 554,
  [555] = 555,
  [556] = 556,
  [557] = 557,
  [558] = 558,
  [559] = 559,
  [560] = 560,
  [561] = 558,
  [562] = 562,
  [563] = 563,
  [564] = 564,
  [565] = 565,
  [566] = 566,
  [567] = 560,
  [568] = 568,
  [569] = 559,
  [570] = 570,
  [571] = 571,
  [572] = 572,
  [573] = 571,
  [574] = 568,
  [575] = 575,
  [576] = 566,
  [577] = 577,
  [578] = 560,
  [579] = 562,
  [580] = 554,
  [581] = 565,
  [582] = 566,
  [583] = 583,
  [584] = 562,
  [585] = 565,
  [586] = 586,
  [587] = 571,
  [588] = 562,
  [589] = 589,
  [590] = 590,
  [591] = 558,
  [592] = 556,
  [593] = 566,
  [594] = 594,
  [595] = 560,
  [596] = 566,
  [597] = 554,
  [598] = 586,
  [599] = 554,
  [600] = 563,
  [601] = 564,
  [602] = 560,
  [603] = 554,
  [604] = 586,
  [605] = 568,
  [606] = 559,
  [607] = 562,
  [608] = 577,
  [609] = 586,
  [610] = 571,
  [611] = 564,
  [612] = 575,
  [613] = 563,
  [614] = 577,
  [615] = 571,
  [616] = 556,
  [617] = 556,
  [618] = 558,
  [619] = 586,
  [620] = 565,
  [621] = 575,
  [622] = 565,
  [623] = 623,
  [624] = 563,
  [625] = 558,
  [626] = 556,
  [627] = 627,
  [628] = 628,
  [629] = 563,
  [630] = 564,
  [631] = 564,
  [632] = 632,
  [633] = 633,
  [634] = 634,
  [635] = 635,
  [636] = 636,
  [637] = 637,
  [638] = 638,
  [639] = 639,
  [640] = 640,
  [641] = 641,
  [642] = 639,
  [643] = 636,
  [644] = 644,
  [645] = 641,
  [646] = 646,
  [647] = 647,
  [648] = 633,
  [649] = 649,
  [650] = 650,
  [651] = 651,
  [652] = 652,
  [653] = 653,
  [654] = 654,
  [655] = 655,
  [656] = 633,
  [657] = 657,
  [658] = 658,
  [659] = 649,
  [660] = 638,
  [661] = 637,
  [662] = 662,
  [663] = 663,
  [664] = 638,
  [665] = 633,
  [666] = 666,
  [667] = 641,
  [668] = 639,
  [669] = 636,
  [670] = 633,
  [671] = 637,
  [672] = 672,
  [673] = 649,
  [674] = 674,
  [675] = 675,
  [676] = 676,
  [677] = 677,
};

static inline bool sym_identifier_character_set_1(int32_t c) {
  return (c < 43360
    ? (c < 4096
      ? (c < 2654
        ? (c < 1808
          ? (c < 902
            ? (c < 216
              ? (c < 'u'
                ? (c < 'a'
                  ? (c < '_'
                    ? (c >= 'A' && c <= 'Z')
                    : c <= '_')
                  : (c <= 'e' || (c < 'o'
                    ? (c >= 'g' && c <= 'm')
                    : c <= 's')))
                : (c <= 'z' || (c < 186
                  ? (c < 181
                    ? c == 170
                    : c <= 181)
                  : (c <= 186 || (c >= 192 && c <= 214)))))
              : (c <= 246 || (c < 750
                ? (c < 736
                  ? (c < 710
                    ? (c >= 248 && c <= 705)
                    : c <= 721)
                  : (c <= 740 || c == 748))
                : (c <= 750 || (c < 890
                  ? (c < 886
                    ? (c >= 880 && c <= 884)
                    : c <= 887)
                  : (c <= 893 || c == 895))))))
            : (c <= 902 || (c < 1488
              ? (c < 1015
                ? (c < 910
                  ? (c < 908
                    ? (c >= 904 && c <= 906)
                    : c <= 908)
                  : (c <= 929 || (c >= 931 && c <= 1013)))
                : (c <= 1153 || (c < 1369
                  ? (c < 1329
                    ? (c >= 1162 && c <= 1327)
                    : c <= 1366)
                  : (c <= 1369 || (c >= 1376 && c <= 1416)))))
              : (c <= 1514 || (c < 1749
                ? (c < 1646
                  ? (c < 1568
                    ? (c >= 1519 && c <= 1522)
                    : c <= 1610)
                  : (c <= 1647 || (c >= 1649 && c <= 1747)))
                : (c <= 1749 || (c < 1786
                  ? (c < 1774
                    ? (c >= 1765 && c <= 1766)
                    : c <= 1775)
                  : (c <= 1788 || c == 1791))))))))
          : (c <= 1808 || (c < 2437
            ? (c < 2112
              ? (c < 2042
                ? (c < 1969
                  ? (c < 1869
                    ? (c >= 1810 && c <= 1839)
                    : c <= 1957)
                  : (c <= 1969 || (c < 2036
                    ? (c >= 1994 && c <= 2026)
                    : c <= 2037)))
                : (c <= 2042 || (c < 2084
                  ? (c < 2074
                    ? (c >= 2048 && c <= 2069)
                    : c <= 2074)
                  : (c <= 2084 || c == 2088))))
              : (c <= 2136 || (c < 2308
                ? (c < 2185
                  ? (c < 2160
                    ? (c >= 2144 && c <= 2154)
                    : c <= 2183)
                  : (c <= 2190 || (c >= 2208 && c <= 2249)))
                : (c <= 2361 || (c < 2392
                  ? (c < 2384
                    ? c == 2365
                    : c <= 2384)
                  : (c <= 2401 || (c >= 2417 && c <= 2432)))))))
            : (c <= 2444 || (c < 2544
              ? (c < 2486
                ? (c < 2474
                  ? (c < 2451
                    ? (c >= 2447 && c <= 2448)
                    : c <= 2472)
                  : (c <= 2480 || c == 2482))
                : (c <= 2489 || (c < 2524
                  ? (c < 2510
                    ? c == 2493
                    : c <= 2510)
                  : (c <= 2525 || (c >= 2527 && c <= 2529)))))
              : (c <= 2545 || (c < 2602
                ? (c < 2575
                  ? (c < 2565
                    ? c == 2556
                    : c <= 2570)
                  : (c <= 2576 || (c >= 2579 && c <= 2600)))
                : (c <= 2608 || (c < 2616
                  ? (c < 2613
                    ? (c >= 2610 && c <= 2611)
                    : c <= 2614)
                  : (c <= 2617 || (c >= 2649 && c <= 2652)))))))))))
        : (c <= 2654 || (c < 3200
          ? (c < 2929
            ? (c < 2809
              ? (c < 2738
                ? (c < 2703
                  ? (c < 2693
                    ? (c >= 2674 && c <= 2676)
                    : c <= 2701)
                  : (c <= 2705 || (c < 2730
                    ? (c >= 2707 && c <= 2728)
                    : c <= 2736)))
                : (c <= 2739 || (c < 2768
                  ? (c < 2749
                    ? (c >= 2741 && c <= 2745)
                    : c <= 2749)
                  : (c <= 2768 || (c >= 2784 && c <= 2785)))))
              : (c <= 2809 || (c < 2866
                ? (c < 2835
                  ? (c < 2831
                    ? (c >= 2821 && c <= 2828)
                    : c <= 2832)
                  : (c <= 2856 || (c >= 2858 && c <= 2864)))
                : (c <= 2867 || (c < 2908
                  ? (c < 2877
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2877)
                  : (c <= 2909 || (c >= 2911 && c <= 2913)))))))
            : (c <= 2929 || (c < 2990
              ? (c < 2969
                ? (c < 2958
                  ? (c < 2949
                    ? c == 2947
                    : c <= 2954)
                  : (c <= 2960 || (c >= 2962 && c <= 2965)))
                : (c <= 2970 || (c < 2979
                  ? (c < 2974
                    ? c == 2972
                    : c <= 2975)
                  : (c <= 2980 || (c >= 2984 && c <= 2986)))))
              : (c <= 3001 || (c < 3114
                ? (c < 3086
                  ? (c < 3077
                    ? c == 3024
                    : c <= 3084)
                  : (c <= 3088 || (c >= 3090 && c <= 3112)))
                : (c <= 3129 || (c < 3165
                  ? (c < 3160
                    ? c == 3133
                    : c <= 3162)
                  : (c <= 3165 || (c >= 3168 && c <= 3169)))))))))
          : (c <= 3200 || (c < 3517
            ? (c < 3342
              ? (c < 3261
                ? (c < 3218
                  ? (c < 3214
                    ? (c >= 3205 && c <= 3212)
                    : c <= 3216)
                  : (c <= 3240 || (c < 3253
                    ? (c >= 3242 && c <= 3251)
                    : c <= 3257)))
                : (c <= 3261 || (c < 3313
                  ? (c < 3296
                    ? (c >= 3293 && c <= 3294)
                    : c <= 3297)
                  : (c <= 3314 || (c >= 3332 && c <= 3340)))))
              : (c <= 3344 || (c < 3423
                ? (c < 3406
                  ? (c < 3389
                    ? (c >= 3346 && c <= 3386)
                    : c <= 3389)
                  : (c <= 3406 || (c >= 3412 && c <= 3414)))
                : (c <= 3425 || (c < 3482
                  ? (c < 3461
                    ? (c >= 3450 && c <= 3455)
                    : c <= 3478)
                  : (c <= 3505 || (c >= 3507 && c <= 3515)))))))
            : (c <= 3517 || (c < 3751
              ? (c < 3713
                ? (c < 3634
                  ? (c < 3585
                    ? (c >= 3520 && c <= 3526)
                    : c <= 3632)
                  : (c <= 3635 || (c >= 3648 && c <= 3654)))
                : (c <= 3714 || (c < 3724
                  ? (c < 3718
                    ? c == 3716
                    : c <= 3722)
                  : (c <= 3747 || c == 3749))))
              : (c <= 3760 || (c < 3804
                ? (c < 3776
                  ? (c < 3773
                    ? (c >= 3762 && c <= 3763)
                    : c <= 3773)
                  : (c <= 3780 || c == 3782))
                : (c <= 3807 || (c < 3913
                  ? (c < 3904
                    ? c == 3840
                    : c <= 3911)
                  : (c <= 3948 || (c >= 3976 && c <= 3980)))))))))))))
      : (c <= 4138 || (c < 8064
        ? (c < 5998
          ? (c < 4786
            ? (c < 4301
              ? (c < 4206
                ? (c < 4186
                  ? (c < 4176
                    ? c == 4159
                    : c <= 4181)
                  : (c <= 4189 || (c < 4197
                    ? c == 4193
                    : c <= 4198)))
                : (c <= 4208 || (c < 4256
                  ? (c < 4238
                    ? (c >= 4213 && c <= 4225)
                    : c <= 4238)
                  : (c <= 4293 || c == 4295))))
              : (c <= 4301 || (c < 4696
                ? (c < 4682
                  ? (c < 4348
                    ? (c >= 4304 && c <= 4346)
                    : c <= 4680)
                  : (c <= 4685 || (c >= 4688 && c <= 4694)))
                : (c <= 4696 || (c < 4746
                  ? (c < 4704
                    ? (c >= 4698 && c <= 4701)
                    : c <= 4744)
                  : (c <= 4749 || (c >= 4752 && c <= 4784)))))))
            : (c <= 4789 || (c < 5112
              ? (c < 4824
                ? (c < 4802
                  ? (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)
                  : (c <= 4805 || (c >= 4808 && c <= 4822)))
                : (c <= 4880 || (c < 4992
                  ? (c < 4888
                    ? (c >= 4882 && c <= 4885)
                    : c <= 4954)
                  : (c <= 5007 || (c >= 5024 && c <= 5109)))))
              : (c <= 5117 || (c < 5870
                ? (c < 5761
                  ? (c < 5743
                    ? (c >= 5121 && c <= 5740)
                    : c <= 5759)
                  : (c <= 5786 || (c >= 5792 && c <= 5866)))
                : (c <= 5880 || (c < 5952
                  ? (c < 5919
                    ? (c >= 5888 && c <= 5905)
                    : c <= 5937)
                  : (c <= 5969 || (c >= 5984 && c <= 5996)))))))))
          : (c <= 6000 || (c < 7168
            ? (c < 6528
              ? (c < 6314
                ? (c < 6108
                  ? (c < 6103
                    ? (c >= 6016 && c <= 6067)
                    : c <= 6103)
                  : (c <= 6108 || (c < 6272
                    ? (c >= 6176 && c <= 6264)
                    : c <= 6312)))
                : (c <= 6314 || (c < 6480
                  ? (c < 6400
                    ? (c >= 6320 && c <= 6389)
                    : c <= 6430)
                  : (c <= 6509 || (c >= 6512 && c <= 6516)))))
              : (c <= 6571 || (c < 6917
                ? (c < 6688
                  ? (c < 6656
                    ? (c >= 6576 && c <= 6601)
                    : c <= 6678)
                  : (c <= 6740 || c == 6823))
                : (c <= 6963 || (c < 7086
                  ? (c < 7043
                    ? (c >= 6981 && c <= 6988)
                    : c <= 7072)
                  : (c <= 7087 || (c >= 7098 && c <= 7141)))))))
            : (c <= 7203 || (c < 7424
              ? (c < 7357
                ? (c < 7296
                  ? (c < 7258
                    ? (c >= 7245 && c <= 7247)
                    : c <= 7293)
                  : (c <= 7304 || (c >= 7312 && c <= 7354)))
                : (c <= 7359 || (c < 7413
                  ? (c < 7406
                    ? (c >= 7401 && c <= 7404)
                    : c <= 7411)
                  : (c <= 7414 || c == 7418))))
              : (c <= 7615 || (c < 8016
                ? (c < 7968
                  ? (c < 7960
                    ? (c >= 7680 && c <= 7957)
                    : c <= 7965)
                  : (c <= 8005 || (c >= 8008 && c <= 8013)))
                : (c <= 8023 || (c < 8029
                  ? (c < 8027
                    ? c == 8025
                    : c <= 8027)
                  : (c <= 8029 || (c >= 8031 && c <= 8061)))))))))))
        : (c <= 8116 || (c < 11728
          ? (c < 8490
            ? (c < 8319
              ? (c < 8150
                ? (c < 8130
                  ? (c < 8126
                    ? (c >= 8118 && c <= 8124)
                    : c <= 8126)
                  : (c <= 8132 || (c < 8144
                    ? (c >= 8134 && c <= 8140)
                    : c <= 8147)))
                : (c <= 8155 || (c < 8182
                  ? (c < 8178
                    ? (c >= 8160 && c <= 8172)
                    : c <= 8180)
                  : (c <= 8188 || c == 8305))))
              : (c <= 8319 || (c < 8469
                ? (c < 8455
                  ? (c < 8450
                    ? (c >= 8336 && c <= 8348)
                    : c <= 8450)
                  : (c <= 8455 || (c >= 8458 && c <= 8467)))
                : (c <= 8469 || (c < 8486
                  ? (c < 8484
                    ? (c >= 8472 && c <= 8477)
                    : c <= 8484)
                  : (c <= 8486 || c == 8488))))))
            : (c <= 8505 || (c < 11565
              ? (c < 11264
                ? (c < 8526
                  ? (c < 8517
                    ? (c >= 8508 && c <= 8511)
                    : c <= 8521)
                  : (c <= 8526 || (c >= 8544 && c <= 8584)))
                : (c <= 11492 || (c < 11520
                  ? (c < 11506
                    ? (c >= 11499 && c <= 11502)
                    : c <= 11507)
                  : (c <= 11557 || c == 11559))))
              : (c <= 11565 || (c < 11688
                ? (c < 11648
                  ? (c < 11631
                    ? (c >= 11568 && c <= 11623)
                    : c <= 11631)
                  : (c <= 11670 || (c >= 11680 && c <= 11686)))
                : (c <= 11694 || (c < 11712
                  ? (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)
                  : (c <= 11718 || (c >= 11720 && c <= 11726)))))))))
          : (c <= 11734 || (c < 42560
            ? (c < 12549
              ? (c < 12344
                ? (c < 12321
                  ? (c < 12293
                    ? (c >= 11736 && c <= 11742)
                    : c <= 12295)
                  : (c <= 12329 || (c >= 12337 && c <= 12341)))
                : (c <= 12348 || (c < 12449
                  ? (c < 12443
                    ? (c >= 12353 && c <= 12438)
                    : c <= 12447)
                  : (c <= 12538 || (c >= 12540 && c <= 12543)))))
              : (c <= 12591 || (c < 19968
                ? (c < 12784
                  ? (c < 12704
                    ? (c >= 12593 && c <= 12686)
                    : c <= 12735)
                  : (c <= 12799 || (c >= 13312 && c <= 19903)))
                : (c <= 42124 || (c < 42512
                  ? (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)
                  : (c <= 42527 || (c >= 42538 && c <= 42539)))))))
            : (c <= 42606 || (c < 43011
              ? (c < 42891
                ? (c < 42775
                  ? (c < 42656
                    ? (c >= 42623 && c <= 42653)
                    : c <= 42735)
                  : (c <= 42783 || (c >= 42786 && c <= 42888)))
                : (c <= 42954 || (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c >= 42994 && c <= 43009)))))
              : (c <= 43013 || (c < 43250
                ? (c < 43072
                  ? (c < 43020
                    ? (c >= 43015 && c <= 43018)
                    : c <= 43042)
                  : (c <= 43123 || (c >= 43138 && c <= 43187)))
                : (c <= 43255 || (c < 43274
                  ? (c < 43261
                    ? c == 43259
                    : c <= 43262)
                  : (c <= 43301 || (c >= 43312 && c <= 43334)))))))))))))))
    : (c <= 43388 || (c < 70751
      ? (c < 66967
        ? (c < 64323
          ? (c < 43785
            ? (c < 43646
              ? (c < 43520
                ? (c < 43488
                  ? (c < 43471
                    ? (c >= 43396 && c <= 43442)
                    : c <= 43471)
                  : (c <= 43492 || (c < 43514
                    ? (c >= 43494 && c <= 43503)
                    : c <= 43518)))
                : (c <= 43560 || (c < 43616
                  ? (c < 43588
                    ? (c >= 43584 && c <= 43586)
                    : c <= 43595)
                  : (c <= 43638 || c == 43642))))
              : (c <= 43695 || (c < 43714
                ? (c < 43705
                  ? (c < 43701
                    ? c == 43697
                    : c <= 43702)
                  : (c <= 43709 || c == 43712))
                : (c <= 43714 || (c < 43762
                  ? (c < 43744
                    ? (c >= 43739 && c <= 43741)
                    : c <= 43754)
                  : (c <= 43764 || (c >= 43777 && c <= 43782)))))))
            : (c <= 43790 || (c < 63744
              ? (c < 43868
                ? (c < 43816
                  ? (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)
                  : (c <= 43822 || (c >= 43824 && c <= 43866)))
                : (c <= 43881 || (c < 55216
                  ? (c < 44032
                    ? (c >= 43888 && c <= 44002)
                    : c <= 55203)
                  : (c <= 55238 || (c >= 55243 && c <= 55291)))))
              : (c <= 64109 || (c < 64287
                ? (c < 64275
                  ? (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)
                  : (c <= 64279 || c == 64285))
                : (c <= 64296 || (c < 64318
                  ? (c < 64312
                    ? (c >= 64298 && c <= 64310)
                    : c <= 64316)
                  : (c <= 64318 || (c >= 64320 && c <= 64321)))))))))
          : (c <= 64324 || (c < 65664
            ? (c < 65474
              ? (c < 65136
                ? (c < 64848
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64829)
                  : (c <= 64911 || (c < 65008
                    ? (c >= 64914 && c <= 64967)
                    : c <= 65019)))
                : (c <= 65140 || (c < 65345
                  ? (c < 65313
                    ? (c >= 65142 && c <= 65276)
                    : c <= 65338)
                  : (c <= 65370 || (c >= 65382 && c <= 65470)))))
              : (c <= 65479 || (c < 65549
                ? (c < 65498
                  ? (c < 65490
                    ? (c >= 65482 && c <= 65487)
                    : c <= 65495)
                  : (c <= 65500 || (c >= 65536 && c <= 65547)))
                : (c <= 65574 || (c < 65599
                  ? (c < 65596
                    ? (c >= 65576 && c <= 65594)
                    : c <= 65597)
                  : (c <= 65613 || (c >= 65616 && c <= 65629)))))))
            : (c <= 65786 || (c < 66513
              ? (c < 66349
                ? (c < 66208
                  ? (c < 66176
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66204)
                  : (c <= 66256 || (c >= 66304 && c <= 66335)))
                : (c <= 66378 || (c < 66464
                  ? (c < 66432
                    ? (c >= 66384 && c <= 66421)
                    : c <= 66461)
                  : (c <= 66499 || (c >= 66504 && c <= 66511)))))
              : (c <= 66517 || (c < 66864
                ? (c < 66776
                  ? (c < 66736
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66771)
                  : (c <= 66811 || (c >= 66816 && c <= 66855)))
                : (c <= 66915 || (c < 66956
                  ? (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)
                  : (c <= 66962 || (c >= 66964 && c <= 66965)))))))))))
        : (c <= 66977 || (c < 69296
          ? (c < 67872
            ? (c < 67592
              ? (c < 67424
                ? (c < 67003
                  ? (c < 66995
                    ? (c >= 66979 && c <= 66993)
                    : c <= 67001)
                  : (c <= 67004 || (c < 67392
                    ? (c >= 67072 && c <= 67382)
                    : c <= 67413)))
                : (c <= 67431 || (c < 67506
                  ? (c < 67463
                    ? (c >= 67456 && c <= 67461)
                    : c <= 67504)
                  : (c <= 67514 || (c >= 67584 && c <= 67589)))))
              : (c <= 67592 || (c < 67680
                ? (c < 67644
                  ? (c < 67639
                    ? (c >= 67594 && c <= 67637)
                    : c <= 67640)
                  : (c <= 67644 || (c >= 67647 && c <= 67669)))
                : (c <= 67702 || (c < 67828
                  ? (c < 67808
                    ? (c >= 67712 && c <= 67742)
                    : c <= 67826)
                  : (c <= 67829 || (c >= 67840 && c <= 67861)))))))
            : (c <= 67897 || (c < 68297
              ? (c < 68117
                ? (c < 68096
                  ? (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)
                  : (c <= 68096 || (c >= 68112 && c <= 68115)))
                : (c <= 68119 || (c < 68224
                  ? (c < 68192
                    ? (c >= 68121 && c <= 68149)
                    : c <= 68220)
                  : (c <= 68252 || (c >= 68288 && c <= 68295)))))
              : (c <= 68324 || (c < 68608
                ? (c < 68448
                  ? (c < 68416
                    ? (c >= 68352 && c <= 68405)
                    : c <= 68437)
                  : (c <= 68466 || (c >= 68480 && c <= 68497)))
                : (c <= 68680 || (c < 68864
                  ? (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)
                  : (c <= 68899 || (c >= 69248 && c <= 69289)))))))))
          : (c <= 69297 || (c < 70108
            ? (c < 69763
              ? (c < 69552
                ? (c < 69424
                  ? (c < 69415
                    ? (c >= 69376 && c <= 69404)
                    : c <= 69415)
                  : (c <= 69445 || (c >= 69488 && c <= 69505)))
                : (c <= 69572 || (c < 69745
                  ? (c < 69635
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69687)
                  : (c <= 69746 || c == 69749))))
              : (c <= 69807 || (c < 69968
                ? (c < 69956
                  ? (c < 69891
                    ? (c >= 69840 && c <= 69864)
                    : c <= 69926)
                  : (c <= 69956 || c == 69959))
                : (c <= 70002 || (c < 70081
                  ? (c < 70019
                    ? c == 70006
                    : c <= 70066)
                  : (c <= 70084 || c == 70106))))))
            : (c <= 70108 || (c < 70415
              ? (c < 70282
                ? (c < 70272
                  ? (c < 70163
                    ? (c >= 70144 && c <= 70161)
                    : c <= 70187)
                  : (c <= 70278 || c == 70280))
                : (c <= 70285 || (c < 70320
                  ? (c < 70303
                    ? (c >= 70287 && c <= 70301)
                    : c <= 70312)
                  : (c <= 70366 || (c >= 70405 && c <= 70412)))))
              : (c <= 70416 || (c < 70461
                ? (c < 70450
                  ? (c < 70442
                    ? (c >= 70419 && c <= 70440)
                    : c <= 70448)
                  : (c <= 70451 || (c >= 70453 && c <= 70457)))
                : (c <= 70461 || (c < 70656
                  ? (c < 70493
                    ? c == 70480
                    : c <= 70497)
                  : (c <= 70708 || (c >= 70727 && c <= 70730)))))))))))))
      : (c <= 70753 || (c < 119966
        ? (c < 73063
          ? (c < 72096
            ? (c < 71488
              ? (c < 71168
                ? (c < 70855
                  ? (c < 70852
                    ? (c >= 70784 && c <= 70831)
                    : c <= 70853)
                  : (c <= 70855 || (c < 71128
                    ? (c >= 71040 && c <= 71086)
                    : c <= 71131)))
                : (c <= 71215 || (c < 71352
                  ? (c < 71296
                    ? c == 71236
                    : c <= 71338)
                  : (c <= 71352 || (c >= 71424 && c <= 71450)))))
              : (c <= 71494 || (c < 71948
                ? (c < 71935
                  ? (c < 71840
                    ? (c >= 71680 && c <= 71723)
                    : c <= 71903)
                  : (c <= 71942 || c == 71945))
                : (c <= 71955 || (c < 71999
                  ? (c < 71960
                    ? (c >= 71957 && c <= 71958)
                    : c <= 71983)
                  : (c <= 71999 || c == 72001))))))
            : (c <= 72103 || (c < 72368
              ? (c < 72203
                ? (c < 72163
                  ? (c < 72161
                    ? (c >= 72106 && c <= 72144)
                    : c <= 72161)
                  : (c <= 72163 || c == 72192))
                : (c <= 72242 || (c < 72284
                  ? (c < 72272
                    ? c == 72250
                    : c <= 72272)
                  : (c <= 72329 || c == 72349))))
              : (c <= 72440 || (c < 72960
                ? (c < 72768
                  ? (c < 72714
                    ? (c >= 72704 && c <= 72712)
                    : c <= 72750)
                  : (c <= 72768 || (c >= 72818 && c <= 72847)))
                : (c <= 72966 || (c < 73030
                  ? (c < 72971
                    ? (c >= 72968 && c <= 72969)
                    : c <= 73008)
                  : (c <= 73030 || (c >= 73056 && c <= 73061)))))))))
          : (c <= 73064 || (c < 94032
            ? (c < 92160
              ? (c < 74752
                ? (c < 73440
                  ? (c < 73112
                    ? (c >= 73066 && c <= 73097)
                    : c <= 73112)
                  : (c <= 73458 || (c < 73728
                    ? c == 73648
                    : c <= 74649)))
                : (c <= 74862 || (c < 77824
                  ? (c < 77712
                    ? (c >= 74880 && c <= 75075)
                    : c <= 77808)
                  : (c <= 78894 || (c >= 82944 && c <= 83526)))))
              : (c <= 92728 || (c < 92992
                ? (c < 92880
                  ? (c < 92784
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92862)
                  : (c <= 92909 || (c >= 92928 && c <= 92975)))
                : (c <= 92995 || (c < 93760
                  ? (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)
                  : (c <= 93823 || (c >= 93952 && c <= 94026)))))))
            : (c <= 94032 || (c < 110592
              ? (c < 100352
                ? (c < 94179
                  ? (c < 94176
                    ? (c >= 94099 && c <= 94111)
                    : c <= 94177)
                  : (c <= 94179 || (c >= 94208 && c <= 100343)))
                : (c <= 101589 || (c < 110581
                  ? (c < 110576
                    ? (c >= 101632 && c <= 101640)
                    : c <= 110579)
                  : (c <= 110587 || (c >= 110589 && c <= 110590)))))
              : (c <= 110882 || (c < 113776
                ? (c < 110960
                  ? (c < 110948
                    ? (c >= 110928 && c <= 110930)
                    : c <= 110951)
                  : (c <= 111355 || (c >= 113664 && c <= 113770)))
                : (c <= 113788 || (c < 119808
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 119892 || (c >= 119894 && c <= 119964)))))))))))
        : (c <= 119967 || (c < 126464
          ? (c < 120598
            ? (c < 120094
              ? (c < 119997
                ? (c < 119977
                  ? (c < 119973
                    ? c == 119970
                    : c <= 119974)
                  : (c <= 119980 || (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)))
                : (c <= 120003 || (c < 120077
                  ? (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)
                  : (c <= 120084 || (c >= 120086 && c <= 120092)))))
              : (c <= 120121 || (c < 120146
                ? (c < 120134
                  ? (c < 120128
                    ? (c >= 120123 && c <= 120126)
                    : c <= 120132)
                  : (c <= 120134 || (c >= 120138 && c <= 120144)))
                : (c <= 120485 || (c < 120540
                  ? (c < 120514
                    ? (c >= 120488 && c <= 120512)
                    : c <= 120538)
                  : (c <= 120570 || (c >= 120572 && c <= 120596)))))))
            : (c <= 120628 || (c < 123214
              ? (c < 120746
                ? (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c >= 120714 && c <= 120744)))
                : (c <= 120770 || (c < 123136
                  ? (c < 122624
                    ? (c >= 120772 && c <= 120779)
                    : c <= 122654)
                  : (c <= 123180 || (c >= 123191 && c <= 123197)))))
              : (c <= 123214 || (c < 124909
                ? (c < 124896
                  ? (c < 123584
                    ? (c >= 123536 && c <= 123565)
                    : c <= 123627)
                  : (c <= 124902 || (c >= 124904 && c <= 124907)))
                : (c <= 124910 || (c < 125184
                  ? (c < 124928
                    ? (c >= 124912 && c <= 124926)
                    : c <= 125124)
                  : (c <= 125251 || c == 125259))))))))
          : (c <= 126467 || (c < 126559
            ? (c < 126535
              ? (c < 126505
                ? (c < 126500
                  ? (c < 126497
                    ? (c >= 126469 && c <= 126495)
                    : c <= 126498)
                  : (c <= 126500 || c == 126503))
                : (c <= 126514 || (c < 126523
                  ? (c < 126521
                    ? (c >= 126516 && c <= 126519)
                    : c <= 126521)
                  : (c <= 126523 || c == 126530))))
              : (c <= 126535 || (c < 126548
                ? (c < 126541
                  ? (c < 126539
                    ? c == 126537
                    : c <= 126539)
                  : (c <= 126543 || (c >= 126545 && c <= 126546)))
                : (c <= 126548 || (c < 126555
                  ? (c < 126553
                    ? c == 126551
                    : c <= 126553)
                  : (c <= 126555 || c == 126557))))))
            : (c <= 126559 || (c < 126625
              ? (c < 126580
                ? (c < 126567
                  ? (c < 126564
                    ? (c >= 126561 && c <= 126562)
                    : c <= 126564)
                  : (c <= 126570 || (c >= 126572 && c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c >= 126603 && c <= 126619)))))
              : (c <= 126627 || (c < 177984
                ? (c < 131072
                  ? (c < 126635
                    ? (c >= 126629 && c <= 126633)
                    : c <= 126651)
                  : (c <= 173791 || (c >= 173824 && c <= 177976)))
                : (c <= 178205 || (c < 194560
                  ? (c < 183984
                    ? (c >= 178208 && c <= 183969)
                    : c <= 191456)
                  : (c <= 195101 || (c >= 196608 && c <= 201546)))))))))))))))));
}

static inline bool sym_identifier_character_set_2(int32_t c) {
  return (c < 43396
    ? (c < 4176
      ? (c < 2703
        ? (c < 1969
          ? (c < 910
            ? (c < 736
              ? (c < 186
                ? (c < 'a'
                  ? (c < '_'
                    ? (c >= 'A' && c <= 'Z')
                    : c <= '_')
                  : (c <= 'z' || (c < 181
                    ? c == 170
                    : c <= 181)))
                : (c <= 186 || (c < 248
                  ? (c < 216
                    ? (c >= 192 && c <= 214)
                    : c <= 246)
                  : (c <= 705 || (c >= 710 && c <= 721)))))
              : (c <= 740 || (c < 890
                ? (c < 880
                  ? (c < 750
                    ? c == 748
                    : c <= 750)
                  : (c <= 884 || (c >= 886 && c <= 887)))
                : (c <= 893 || (c < 904
                  ? (c < 902
                    ? c == 895
                    : c <= 902)
                  : (c <= 906 || c == 908))))))
            : (c <= 929 || (c < 1646
              ? (c < 1369
                ? (c < 1162
                  ? (c < 1015
                    ? (c >= 931 && c <= 1013)
                    : c <= 1153)
                  : (c <= 1327 || (c >= 1329 && c <= 1366)))
                : (c <= 1369 || (c < 1519
                  ? (c < 1488
                    ? (c >= 1376 && c <= 1416)
                    : c <= 1514)
                  : (c <= 1522 || (c >= 1568 && c <= 1610)))))
              : (c <= 1647 || (c < 1786
                ? (c < 1765
                  ? (c < 1749
                    ? (c >= 1649 && c <= 1747)
                    : c <= 1749)
                  : (c <= 1766 || (c >= 1774 && c <= 1775)))
                : (c <= 1788 || (c < 1810
                  ? (c < 1808
                    ? c == 1791
                    : c <= 1808)
                  : (c <= 1839 || (c >= 1869 && c <= 1957)))))))))
          : (c <= 1969 || (c < 2474
            ? (c < 2185
              ? (c < 2084
                ? (c < 2042
                  ? (c < 2036
                    ? (c >= 1994 && c <= 2026)
                    : c <= 2037)
                  : (c <= 2042 || (c < 2074
                    ? (c >= 2048 && c <= 2069)
                    : c <= 2074)))
                : (c <= 2084 || (c < 2144
                  ? (c < 2112
                    ? c == 2088
                    : c <= 2136)
                  : (c <= 2154 || (c >= 2160 && c <= 2183)))))
              : (c <= 2190 || (c < 2392
                ? (c < 2365
                  ? (c < 2308
                    ? (c >= 2208 && c <= 2249)
                    : c <= 2361)
                  : (c <= 2365 || c == 2384))
                : (c <= 2401 || (c < 2447
                  ? (c < 2437
                    ? (c >= 2417 && c <= 2432)
                    : c <= 2444)
                  : (c <= 2448 || (c >= 2451 && c <= 2472)))))))
            : (c <= 2480 || (c < 2575
              ? (c < 2524
                ? (c < 2493
                  ? (c < 2486
                    ? c == 2482
                    : c <= 2489)
                  : (c <= 2493 || c == 2510))
                : (c <= 2525 || (c < 2556
                  ? (c < 2544
                    ? (c >= 2527 && c <= 2529)
                    : c <= 2545)
                  : (c <= 2556 || (c >= 2565 && c <= 2570)))))
              : (c <= 2576 || (c < 2616
                ? (c < 2610
                  ? (c < 2602
                    ? (c >= 2579 && c <= 2600)
                    : c <= 2608)
                  : (c <= 2611 || (c >= 2613 && c <= 2614)))
                : (c <= 2617 || (c < 2674
                  ? (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)
                  : (c <= 2676 || (c >= 2693 && c <= 2701)))))))))))
        : (c <= 2705 || (c < 3218
          ? (c < 2958
            ? (c < 2835
              ? (c < 2768
                ? (c < 2738
                  ? (c < 2730
                    ? (c >= 2707 && c <= 2728)
                    : c <= 2736)
                  : (c <= 2739 || (c < 2749
                    ? (c >= 2741 && c <= 2745)
                    : c <= 2749)))
                : (c <= 2768 || (c < 2821
                  ? (c < 2809
                    ? (c >= 2784 && c <= 2785)
                    : c <= 2809)
                  : (c <= 2828 || (c >= 2831 && c <= 2832)))))
              : (c <= 2856 || (c < 2908
                ? (c < 2869
                  ? (c < 2866
                    ? (c >= 2858 && c <= 2864)
                    : c <= 2867)
                  : (c <= 2873 || c == 2877))
                : (c <= 2909 || (c < 2947
                  ? (c < 2929
                    ? (c >= 2911 && c <= 2913)
                    : c <= 2929)
                  : (c <= 2947 || (c >= 2949 && c <= 2954)))))))
            : (c <= 2960 || (c < 3086
              ? (c < 2979
                ? (c < 2972
                  ? (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)
                  : (c <= 2972 || (c >= 2974 && c <= 2975)))
                : (c <= 2980 || (c < 3024
                  ? (c < 2990
                    ? (c >= 2984 && c <= 2986)
                    : c <= 3001)
                  : (c <= 3024 || (c >= 3077 && c <= 3084)))))
              : (c <= 3088 || (c < 3165
                ? (c < 3133
                  ? (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)
                  : (c <= 3133 || (c >= 3160 && c <= 3162)))
                : (c <= 3165 || (c < 3205
                  ? (c < 3200
                    ? (c >= 3168 && c <= 3169)
                    : c <= 3200)
                  : (c <= 3212 || (c >= 3214 && c <= 3216)))))))))
          : (c <= 3240 || (c < 3585
            ? (c < 3389
              ? (c < 3296
                ? (c < 3261
                  ? (c < 3253
                    ? (c >= 3242 && c <= 3251)
                    : c <= 3257)
                  : (c <= 3261 || (c >= 3293 && c <= 3294)))
                : (c <= 3297 || (c < 3342
                  ? (c < 3332
                    ? (c >= 3313 && c <= 3314)
                    : c <= 3340)
                  : (c <= 3344 || (c >= 3346 && c <= 3386)))))
              : (c <= 3389 || (c < 3461
                ? (c < 3423
                  ? (c < 3412
                    ? c == 3406
                    : c <= 3414)
                  : (c <= 3425 || (c >= 3450 && c <= 3455)))
                : (c <= 3478 || (c < 3517
                  ? (c < 3507
                    ? (c >= 3482 && c <= 3505)
                    : c <= 3515)
                  : (c <= 3517 || (c >= 3520 && c <= 3526)))))))
            : (c <= 3632 || (c < 3773
              ? (c < 3718
                ? (c < 3713
                  ? (c < 3648
                    ? (c >= 3634 && c <= 3635)
                    : c <= 3654)
                  : (c <= 3714 || c == 3716))
                : (c <= 3722 || (c < 3751
                  ? (c < 3749
                    ? (c >= 3724 && c <= 3747)
                    : c <= 3749)
                  : (c <= 3760 || (c >= 3762 && c <= 3763)))))
              : (c <= 3773 || (c < 3904
                ? (c < 3804
                  ? (c < 3782
                    ? (c >= 3776 && c <= 3780)
                    : c <= 3782)
                  : (c <= 3807 || c == 3840))
                : (c <= 3911 || (c < 4096
                  ? (c < 3976
                    ? (c >= 3913 && c <= 3948)
                    : c <= 3980)
                  : (c <= 4138 || c == 4159))))))))))))
      : (c <= 4181 || (c < 8118
        ? (c < 6103
          ? (c < 4800
            ? (c < 4348
              ? (c < 4238
                ? (c < 4197
                  ? (c < 4193
                    ? (c >= 4186 && c <= 4189)
                    : c <= 4193)
                  : (c <= 4198 || (c < 4213
                    ? (c >= 4206 && c <= 4208)
                    : c <= 4225)))
                : (c <= 4238 || (c < 4301
                  ? (c < 4295
                    ? (c >= 4256 && c <= 4293)
                    : c <= 4295)
                  : (c <= 4301 || (c >= 4304 && c <= 4346)))))
              : (c <= 4680 || (c < 4704
                ? (c < 4696
                  ? (c < 4688
                    ? (c >= 4682 && c <= 4685)
                    : c <= 4694)
                  : (c <= 4696 || (c >= 4698 && c <= 4701)))
                : (c <= 4744 || (c < 4786
                  ? (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)
                  : (c <= 4789 || (c >= 4792 && c <= 4798)))))))
            : (c <= 4800 || (c < 5743
              ? (c < 4888
                ? (c < 4824
                  ? (c < 4808
                    ? (c >= 4802 && c <= 4805)
                    : c <= 4822)
                  : (c <= 4880 || (c >= 4882 && c <= 4885)))
                : (c <= 4954 || (c < 5112
                  ? (c < 5024
                    ? (c >= 4992 && c <= 5007)
                    : c <= 5109)
                  : (c <= 5117 || (c >= 5121 && c <= 5740)))))
              : (c <= 5759 || (c < 5919
                ? (c < 5870
                  ? (c < 5792
                    ? (c >= 5761 && c <= 5786)
                    : c <= 5866)
                  : (c <= 5880 || (c >= 5888 && c <= 5905)))
                : (c <= 5937 || (c < 5998
                  ? (c < 5984
                    ? (c >= 5952 && c <= 5969)
                    : c <= 5996)
                  : (c <= 6000 || (c >= 6016 && c <= 6067)))))))))
          : (c <= 6103 || (c < 7245
            ? (c < 6576
              ? (c < 6320
                ? (c < 6272
                  ? (c < 6176
                    ? c == 6108
                    : c <= 6264)
                  : (c <= 6312 || c == 6314))
                : (c <= 6389 || (c < 6512
                  ? (c < 6480
                    ? (c >= 6400 && c <= 6430)
                    : c <= 6509)
                  : (c <= 6516 || (c >= 6528 && c <= 6571)))))
              : (c <= 6601 || (c < 6981
                ? (c < 6823
                  ? (c < 6688
                    ? (c >= 6656 && c <= 6678)
                    : c <= 6740)
                  : (c <= 6823 || (c >= 6917 && c <= 6963)))
                : (c <= 6988 || (c < 7098
                  ? (c < 7086
                    ? (c >= 7043 && c <= 7072)
                    : c <= 7087)
                  : (c <= 7141 || (c >= 7168 && c <= 7203)))))))
            : (c <= 7247 || (c < 7680
              ? (c < 7401
                ? (c < 7312
                  ? (c < 7296
                    ? (c >= 7258 && c <= 7293)
                    : c <= 7304)
                  : (c <= 7354 || (c >= 7357 && c <= 7359)))
                : (c <= 7404 || (c < 7418
                  ? (c < 7413
                    ? (c >= 7406 && c <= 7411)
                    : c <= 7414)
                  : (c <= 7418 || (c >= 7424 && c <= 7615)))))
              : (c <= 7957 || (c < 8025
                ? (c < 8008
                  ? (c < 7968
                    ? (c >= 7960 && c <= 7965)
                    : c <= 8005)
                  : (c <= 8013 || (c >= 8016 && c <= 8023)))
                : (c <= 8025 || (c < 8031
                  ? (c < 8029
                    ? c == 8027
                    : c <= 8029)
                  : (c <= 8061 || (c >= 8064 && c <= 8116)))))))))))
        : (c <= 8124 || (c < 11736
          ? (c < 8508
            ? (c < 8336
              ? (c < 8160
                ? (c < 8134
                  ? (c < 8130
                    ? c == 8126
                    : c <= 8132)
                  : (c <= 8140 || (c < 8150
                    ? (c >= 8144 && c <= 8147)
                    : c <= 8155)))
                : (c <= 8172 || (c < 8305
                  ? (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)
                  : (c <= 8305 || c == 8319))))
              : (c <= 8348 || (c < 8472
                ? (c < 8458
                  ? (c < 8455
                    ? c == 8450
                    : c <= 8455)
                  : (c <= 8467 || c == 8469))
                : (c <= 8477 || (c < 8488
                  ? (c < 8486
                    ? c == 8484
                    : c <= 8486)
                  : (c <= 8488 || (c >= 8490 && c <= 8505)))))))
            : (c <= 8511 || (c < 11568
              ? (c < 11499
                ? (c < 8544
                  ? (c < 8526
                    ? (c >= 8517 && c <= 8521)
                    : c <= 8526)
                  : (c <= 8584 || (c >= 11264 && c <= 11492)))
                : (c <= 11502 || (c < 11559
                  ? (c < 11520
                    ? (c >= 11506 && c <= 11507)
                    : c <= 11557)
                  : (c <= 11559 || c == 11565))))
              : (c <= 11623 || (c < 11696
                ? (c < 11680
                  ? (c < 11648
                    ? c == 11631
                    : c <= 11670)
                  : (c <= 11686 || (c >= 11688 && c <= 11694)))
                : (c <= 11702 || (c < 11720
                  ? (c < 11712
                    ? (c >= 11704 && c <= 11710)
                    : c <= 11718)
                  : (c <= 11726 || (c >= 11728 && c <= 11734)))))))))
          : (c <= 11742 || (c < 42623
            ? (c < 12593
              ? (c < 12353
                ? (c < 12337
                  ? (c < 12321
                    ? (c >= 12293 && c <= 12295)
                    : c <= 12329)
                  : (c <= 12341 || (c >= 12344 && c <= 12348)))
                : (c <= 12438 || (c < 12540
                  ? (c < 12449
                    ? (c >= 12443 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c >= 12549 && c <= 12591)))))
              : (c <= 12686 || (c < 42192
                ? (c < 13312
                  ? (c < 12784
                    ? (c >= 12704 && c <= 12735)
                    : c <= 12799)
                  : (c <= 19903 || (c >= 19968 && c <= 42124)))
                : (c <= 42237 || (c < 42538
                  ? (c < 42512
                    ? (c >= 42240 && c <= 42508)
                    : c <= 42527)
                  : (c <= 42539 || (c >= 42560 && c <= 42606)))))))
            : (c <= 42653 || (c < 43015
              ? (c < 42960
                ? (c < 42786
                  ? (c < 42775
                    ? (c >= 42656 && c <= 42735)
                    : c <= 42783)
                  : (c <= 42888 || (c >= 42891 && c <= 42954)))
                : (c <= 42961 || (c < 42994
                  ? (c < 42965
                    ? c == 42963
                    : c <= 42969)
                  : (c <= 43009 || (c >= 43011 && c <= 43013)))))
              : (c <= 43018 || (c < 43259
                ? (c < 43138
                  ? (c < 43072
                    ? (c >= 43020 && c <= 43042)
                    : c <= 43123)
                  : (c <= 43187 || (c >= 43250 && c <= 43255)))
                : (c <= 43259 || (c < 43312
                  ? (c < 43274
                    ? (c >= 43261 && c <= 43262)
                    : c <= 43301)
                  : (c <= 43334 || (c >= 43360 && c <= 43388)))))))))))))))
    : (c <= 43442 || (c < 70784
      ? (c < 66979
        ? (c < 64326
          ? (c < 43793
            ? (c < 43697
              ? (c < 43584
                ? (c < 43494
                  ? (c < 43488
                    ? c == 43471
                    : c <= 43492)
                  : (c <= 43503 || (c < 43520
                    ? (c >= 43514 && c <= 43518)
                    : c <= 43560)))
                : (c <= 43586 || (c < 43642
                  ? (c < 43616
                    ? (c >= 43588 && c <= 43595)
                    : c <= 43638)
                  : (c <= 43642 || (c >= 43646 && c <= 43695)))))
              : (c <= 43697 || (c < 43739
                ? (c < 43712
                  ? (c < 43705
                    ? (c >= 43701 && c <= 43702)
                    : c <= 43709)
                  : (c <= 43712 || c == 43714))
                : (c <= 43741 || (c < 43777
                  ? (c < 43762
                    ? (c >= 43744 && c <= 43754)
                    : c <= 43764)
                  : (c <= 43782 || (c >= 43785 && c <= 43790)))))))
            : (c <= 43798 || (c < 64112
              ? (c < 43888
                ? (c < 43824
                  ? (c < 43816
                    ? (c >= 43808 && c <= 43814)
                    : c <= 43822)
                  : (c <= 43866 || (c >= 43868 && c <= 43881)))
                : (c <= 44002 || (c < 55243
                  ? (c < 55216
                    ? (c >= 44032 && c <= 55203)
                    : c <= 55238)
                  : (c <= 55291 || (c >= 63744 && c <= 64109)))))
              : (c <= 64217 || (c < 64298
                ? (c < 64285
                  ? (c < 64275
                    ? (c >= 64256 && c <= 64262)
                    : c <= 64279)
                  : (c <= 64285 || (c >= 64287 && c <= 64296)))
                : (c <= 64310 || (c < 64320
                  ? (c < 64318
                    ? (c >= 64312 && c <= 64316)
                    : c <= 64318)
                  : (c <= 64321 || (c >= 64323 && c <= 64324)))))))))
          : (c <= 64433 || (c < 65856
            ? (c < 65482
              ? (c < 65142
                ? (c < 64914
                  ? (c < 64848
                    ? (c >= 64467 && c <= 64829)
                    : c <= 64911)
                  : (c <= 64967 || (c < 65136
                    ? (c >= 65008 && c <= 65019)
                    : c <= 65140)))
                : (c <= 65276 || (c < 65382
                  ? (c < 65345
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65370)
                  : (c <= 65470 || (c >= 65474 && c <= 65479)))))
              : (c <= 65487 || (c < 65576
                ? (c < 65536
                  ? (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)
                  : (c <= 65547 || (c >= 65549 && c <= 65574)))
                : (c <= 65594 || (c < 65616
                  ? (c < 65599
                    ? (c >= 65596 && c <= 65597)
                    : c <= 65613)
                  : (c <= 65629 || (c >= 65664 && c <= 65786)))))))
            : (c <= 65908 || (c < 66560
              ? (c < 66384
                ? (c < 66304
                  ? (c < 66208
                    ? (c >= 66176 && c <= 66204)
                    : c <= 66256)
                  : (c <= 66335 || (c >= 66349 && c <= 66378)))
                : (c <= 66421 || (c < 66504
                  ? (c < 66464
                    ? (c >= 66432 && c <= 66461)
                    : c <= 66499)
                  : (c <= 66511 || (c >= 66513 && c <= 66517)))))
              : (c <= 66717 || (c < 66928
                ? (c < 66816
                  ? (c < 66776
                    ? (c >= 66736 && c <= 66771)
                    : c <= 66811)
                  : (c <= 66855 || (c >= 66864 && c <= 66915)))
                : (c <= 66938 || (c < 66964
                  ? (c < 66956
                    ? (c >= 66940 && c <= 66954)
                    : c <= 66962)
                  : (c <= 66965 || (c >= 66967 && c <= 66977)))))))))))
        : (c <= 66993 || (c < 69376
          ? (c < 67968
            ? (c < 67594
              ? (c < 67456
                ? (c < 67072
                  ? (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)
                  : (c <= 67382 || (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)))
                : (c <= 67461 || (c < 67584
                  ? (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)
                  : (c <= 67589 || c == 67592))))
              : (c <= 67637 || (c < 67712
                ? (c < 67647
                  ? (c < 67644
                    ? (c >= 67639 && c <= 67640)
                    : c <= 67644)
                  : (c <= 67669 || (c >= 67680 && c <= 67702)))
                : (c <= 67742 || (c < 67840
                  ? (c < 67828
                    ? (c >= 67808 && c <= 67826)
                    : c <= 67829)
                  : (c <= 67861 || (c >= 67872 && c <= 67897)))))))
            : (c <= 68023 || (c < 68352
              ? (c < 68121
                ? (c < 68112
                  ? (c < 68096
                    ? (c >= 68030 && c <= 68031)
                    : c <= 68096)
                  : (c <= 68115 || (c >= 68117 && c <= 68119)))
                : (c <= 68149 || (c < 68288
                  ? (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)
                  : (c <= 68295 || (c >= 68297 && c <= 68324)))))
              : (c <= 68405 || (c < 68736
                ? (c < 68480
                  ? (c < 68448
                    ? (c >= 68416 && c <= 68437)
                    : c <= 68466)
                  : (c <= 68497 || (c >= 68608 && c <= 68680)))
                : (c <= 68786 || (c < 69248
                  ? (c < 68864
                    ? (c >= 68800 && c <= 68850)
                    : c <= 68899)
                  : (c <= 69289 || (c >= 69296 && c <= 69297)))))))))
          : (c <= 69404 || (c < 70144
            ? (c < 69840
              ? (c < 69600
                ? (c < 69488
                  ? (c < 69424
                    ? c == 69415
                    : c <= 69445)
                  : (c <= 69505 || (c >= 69552 && c <= 69572)))
                : (c <= 69622 || (c < 69749
                  ? (c < 69745
                    ? (c >= 69635 && c <= 69687)
                    : c <= 69746)
                  : (c <= 69749 || (c >= 69763 && c <= 69807)))))
              : (c <= 69864 || (c < 70006
                ? (c < 69959
                  ? (c < 69956
                    ? (c >= 69891 && c <= 69926)
                    : c <= 69956)
                  : (c <= 69959 || (c >= 69968 && c <= 70002)))
                : (c <= 70006 || (c < 70106
                  ? (c < 70081
                    ? (c >= 70019 && c <= 70066)
                    : c <= 70084)
                  : (c <= 70106 || c == 70108))))))
            : (c <= 70161 || (c < 70419
              ? (c < 70287
                ? (c < 70280
                  ? (c < 70272
                    ? (c >= 70163 && c <= 70187)
                    : c <= 70278)
                  : (c <= 70280 || (c >= 70282 && c <= 70285)))
                : (c <= 70301 || (c < 70405
                  ? (c < 70320
                    ? (c >= 70303 && c <= 70312)
                    : c <= 70366)
                  : (c <= 70412 || (c >= 70415 && c <= 70416)))))
              : (c <= 70440 || (c < 70480
                ? (c < 70453
                  ? (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)
                  : (c <= 70457 || c == 70461))
                : (c <= 70480 || (c < 70727
                  ? (c < 70656
                    ? (c >= 70493 && c <= 70497)
                    : c <= 70708)
                  : (c <= 70730 || (c >= 70751 && c <= 70753)))))))))))))
      : (c <= 70831 || (c < 119966
        ? (c < 73066
          ? (c < 72106
            ? (c < 71680
              ? (c < 71236
                ? (c < 71040
                  ? (c < 70855
                    ? (c >= 70852 && c <= 70853)
                    : c <= 70855)
                  : (c <= 71086 || (c < 71168
                    ? (c >= 71128 && c <= 71131)
                    : c <= 71215)))
                : (c <= 71236 || (c < 71424
                  ? (c < 71352
                    ? (c >= 71296 && c <= 71338)
                    : c <= 71352)
                  : (c <= 71450 || (c >= 71488 && c <= 71494)))))
              : (c <= 71723 || (c < 71957
                ? (c < 71945
                  ? (c < 71935
                    ? (c >= 71840 && c <= 71903)
                    : c <= 71942)
                  : (c <= 71945 || (c >= 71948 && c <= 71955)))
                : (c <= 71958 || (c < 72001
                  ? (c < 71999
                    ? (c >= 71960 && c <= 71983)
                    : c <= 71999)
                  : (c <= 72001 || (c >= 72096 && c <= 72103)))))))
            : (c <= 72144 || (c < 72704
              ? (c < 72250
                ? (c < 72192
                  ? (c < 72163
                    ? c == 72161
                    : c <= 72163)
                  : (c <= 72192 || (c >= 72203 && c <= 72242)))
                : (c <= 72250 || (c < 72349
                  ? (c < 72284
                    ? c == 72272
                    : c <= 72329)
                  : (c <= 72349 || (c >= 72368 && c <= 72440)))))
              : (c <= 72712 || (c < 72968
                ? (c < 72818
                  ? (c < 72768
                    ? (c >= 72714 && c <= 72750)
                    : c <= 72768)
                  : (c <= 72847 || (c >= 72960 && c <= 72966)))
                : (c <= 72969 || (c < 73056
                  ? (c < 73030
                    ? (c >= 72971 && c <= 73008)
                    : c <= 73030)
                  : (c <= 73061 || (c >= 73063 && c <= 73064)))))))))
          : (c <= 73097 || (c < 94032
            ? (c < 92160
              ? (c < 74752
                ? (c < 73648
                  ? (c < 73440
                    ? c == 73112
                    : c <= 73458)
                  : (c <= 73648 || (c >= 73728 && c <= 74649)))
                : (c <= 74862 || (c < 77824
                  ? (c < 77712
                    ? (c >= 74880 && c <= 75075)
                    : c <= 77808)
                  : (c <= 78894 || (c >= 82944 && c <= 83526)))))
              : (c <= 92728 || (c < 92992
                ? (c < 92880
                  ? (c < 92784
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92862)
                  : (c <= 92909 || (c >= 92928 && c <= 92975)))
                : (c <= 92995 || (c < 93760
                  ? (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)
                  : (c <= 93823 || (c >= 93952 && c <= 94026)))))))
            : (c <= 94032 || (c < 110592
              ? (c < 100352
                ? (c < 94179
                  ? (c < 94176
                    ? (c >= 94099 && c <= 94111)
                    : c <= 94177)
                  : (c <= 94179 || (c >= 94208 && c <= 100343)))
                : (c <= 101589 || (c < 110581
                  ? (c < 110576
                    ? (c >= 101632 && c <= 101640)
                    : c <= 110579)
                  : (c <= 110587 || (c >= 110589 && c <= 110590)))))
              : (c <= 110882 || (c < 113776
                ? (c < 110960
                  ? (c < 110948
                    ? (c >= 110928 && c <= 110930)
                    : c <= 110951)
                  : (c <= 111355 || (c >= 113664 && c <= 113770)))
                : (c <= 113788 || (c < 119808
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 119892 || (c >= 119894 && c <= 119964)))))))))))
        : (c <= 119967 || (c < 126464
          ? (c < 120598
            ? (c < 120094
              ? (c < 119997
                ? (c < 119977
                  ? (c < 119973
                    ? c == 119970
                    : c <= 119974)
                  : (c <= 119980 || (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)))
                : (c <= 120003 || (c < 120077
                  ? (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)
                  : (c <= 120084 || (c >= 120086 && c <= 120092)))))
              : (c <= 120121 || (c < 120146
                ? (c < 120134
                  ? (c < 120128
                    ? (c >= 120123 && c <= 120126)
                    : c <= 120132)
                  : (c <= 120134 || (c >= 120138 && c <= 120144)))
                : (c <= 120485 || (c < 120540
                  ? (c < 120514
                    ? (c >= 120488 && c <= 120512)
                    : c <= 120538)
                  : (c <= 120570 || (c >= 120572 && c <= 120596)))))))
            : (c <= 120628 || (c < 123214
              ? (c < 120746
                ? (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c >= 120714 && c <= 120744)))
                : (c <= 120770 || (c < 123136
                  ? (c < 122624
                    ? (c >= 120772 && c <= 120779)
                    : c <= 122654)
                  : (c <= 123180 || (c >= 123191 && c <= 123197)))))
              : (c <= 123214 || (c < 124909
                ? (c < 124896
                  ? (c < 123584
                    ? (c >= 123536 && c <= 123565)
                    : c <= 123627)
                  : (c <= 124902 || (c >= 124904 && c <= 124907)))
                : (c <= 124910 || (c < 125184
                  ? (c < 124928
                    ? (c >= 124912 && c <= 124926)
                    : c <= 125124)
                  : (c <= 125251 || c == 125259))))))))
          : (c <= 126467 || (c < 126559
            ? (c < 126535
              ? (c < 126505
                ? (c < 126500
                  ? (c < 126497
                    ? (c >= 126469 && c <= 126495)
                    : c <= 126498)
                  : (c <= 126500 || c == 126503))
                : (c <= 126514 || (c < 126523
                  ? (c < 126521
                    ? (c >= 126516 && c <= 126519)
                    : c <= 126521)
                  : (c <= 126523 || c == 126530))))
              : (c <= 126535 || (c < 126548
                ? (c < 126541
                  ? (c < 126539
                    ? c == 126537
                    : c <= 126539)
                  : (c <= 126543 || (c >= 126545 && c <= 126546)))
                : (c <= 126548 || (c < 126555
                  ? (c < 126553
                    ? c == 126551
                    : c <= 126553)
                  : (c <= 126555 || c == 126557))))))
            : (c <= 126559 || (c < 126625
              ? (c < 126580
                ? (c < 126567
                  ? (c < 126564
                    ? (c >= 126561 && c <= 126562)
                    : c <= 126564)
                  : (c <= 126570 || (c >= 126572 && c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c >= 126603 && c <= 126619)))))
              : (c <= 126627 || (c < 177984
                ? (c < 131072
                  ? (c < 126635
                    ? (c >= 126629 && c <= 126633)
                    : c <= 126651)
                  : (c <= 173791 || (c >= 173824 && c <= 177976)))
                : (c <= 178205 || (c < 194560
                  ? (c < 183984
                    ? (c >= 178208 && c <= 183969)
                    : c <= 191456)
                  : (c <= 195101 || (c >= 196608 && c <= 201546)))))))))))))))));
}

static inline bool sym_identifier_character_set_3(int32_t c) {
  return (c < 43520
    ? (c < 3749
      ? (c < 2730
        ? (c < 1984
          ? (c < 910
            ? (c < 216
              ? (c < 'p'
                ? (c < 'A'
                  ? (c < '0'
                    ? c == '-'
                    : c <= '9')
                  : (c <= 'Z' || (c < 'b'
                    ? c == '_'
                    : c <= 'n')))
                : (c <= 'z' || (c < 183
                  ? (c < 181
                    ? c == 170
                    : c <= 181)
                  : (c <= 183 || (c < 192
                    ? c == 186
                    : c <= 214)))))
              : (c <= 246 || (c < 768
                ? (c < 736
                  ? (c < 710
                    ? (c >= 248 && c <= 705)
                    : c <= 721)
                  : (c <= 740 || (c < 750
                    ? c == 748
                    : c <= 750)))
                : (c <= 884 || (c < 895
                  ? (c < 890
                    ? (c >= 886 && c <= 887)
                    : c <= 893)
                  : (c <= 895 || (c < 908
                    ? (c >= 902 && c <= 906)
                    : c <= 908)))))))
            : (c <= 929 || (c < 1479
              ? (c < 1369
                ? (c < 1155
                  ? (c < 1015
                    ? (c >= 931 && c <= 1013)
                    : c <= 1153)
                  : (c <= 1159 || (c < 1329
                    ? (c >= 1162 && c <= 1327)
                    : c <= 1366)))
                : (c <= 1369 || (c < 1471
                  ? (c < 1425
                    ? (c >= 1376 && c <= 1416)
                    : c <= 1469)
                  : (c <= 1471 || (c < 1476
                    ? (c >= 1473 && c <= 1474)
                    : c <= 1477)))))
              : (c <= 1479 || (c < 1749
                ? (c < 1552
                  ? (c < 1519
                    ? (c >= 1488 && c <= 1514)
                    : c <= 1522)
                  : (c <= 1562 || (c < 1646
                    ? (c >= 1568 && c <= 1641)
                    : c <= 1747)))
                : (c <= 1756 || (c < 1791
                  ? (c < 1770
                    ? (c >= 1759 && c <= 1768)
                    : c <= 1788)
                  : (c <= 1791 || (c < 1869
                    ? (c >= 1808 && c <= 1866)
                    : c <= 1969)))))))))
          : (c <= 2037 || (c < 2534
            ? (c < 2437
              ? (c < 2160
                ? (c < 2048
                  ? (c < 2045
                    ? c == 2042
                    : c <= 2045)
                  : (c <= 2093 || (c < 2144
                    ? (c >= 2112 && c <= 2139)
                    : c <= 2154)))
                : (c <= 2183 || (c < 2275
                  ? (c < 2200
                    ? (c >= 2185 && c <= 2190)
                    : c <= 2273)
                  : (c <= 2403 || (c < 2417
                    ? (c >= 2406 && c <= 2415)
                    : c <= 2435)))))
              : (c <= 2444 || (c < 2492
                ? (c < 2474
                  ? (c < 2451
                    ? (c >= 2447 && c <= 2448)
                    : c <= 2472)
                  : (c <= 2480 || (c < 2486
                    ? c == 2482
                    : c <= 2489)))
                : (c <= 2500 || (c < 2519
                  ? (c < 2507
                    ? (c >= 2503 && c <= 2504)
                    : c <= 2510)
                  : (c <= 2519 || (c < 2527
                    ? (c >= 2524 && c <= 2525)
                    : c <= 2531)))))))
            : (c <= 2545 || (c < 2622
              ? (c < 2579
                ? (c < 2561
                  ? (c < 2558
                    ? c == 2556
                    : c <= 2558)
                  : (c <= 2563 || (c < 2575
                    ? (c >= 2565 && c <= 2570)
                    : c <= 2576)))
                : (c <= 2600 || (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c < 2620
                    ? (c >= 2616 && c <= 2617)
                    : c <= 2620)))))
              : (c <= 2626 || (c < 2662
                ? (c < 2641
                  ? (c < 2635
                    ? (c >= 2631 && c <= 2632)
                    : c <= 2637)
                  : (c <= 2641 || (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)))
                : (c <= 2677 || (c < 2703
                  ? (c < 2693
                    ? (c >= 2689 && c <= 2691)
                    : c <= 2701)
                  : (c <= 2705 || (c >= 2707 && c <= 2728)))))))))))
        : (c <= 2736 || (c < 3157
          ? (c < 2929
            ? (c < 2831
              ? (c < 2768
                ? (c < 2748
                  ? (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)
                  : (c <= 2757 || (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)))
                : (c <= 2768 || (c < 2809
                  ? (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)
                  : (c <= 2815 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))))
              : (c <= 2832 || (c < 2887
                ? (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)))
                : (c <= 2888 || (c < 2908
                  ? (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)
                  : (c <= 2909 || (c < 2918
                    ? (c >= 2911 && c <= 2915)
                    : c <= 2927)))))))
            : (c <= 2929 || (c < 3014
              ? (c < 2972
                ? (c < 2958
                  ? (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)
                  : (c <= 2960 || (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)))
                : (c <= 2972 || (c < 2984
                  ? (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)
                  : (c <= 2986 || (c < 3006
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3010)))))
              : (c <= 3016 || (c < 3086
                ? (c < 3031
                  ? (c < 3024
                    ? (c >= 3018 && c <= 3021)
                    : c <= 3024)
                  : (c <= 3031 || (c < 3072
                    ? (c >= 3046 && c <= 3055)
                    : c <= 3084)))
                : (c <= 3088 || (c < 3132
                  ? (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)
                  : (c <= 3140 || (c < 3146
                    ? (c >= 3142 && c <= 3144)
                    : c <= 3149)))))))))
          : (c <= 3158 || (c < 3412
            ? (c < 3270
              ? (c < 3205
                ? (c < 3168
                  ? (c < 3165
                    ? (c >= 3160 && c <= 3162)
                    : c <= 3165)
                  : (c <= 3171 || (c < 3200
                    ? (c >= 3174 && c <= 3183)
                    : c <= 3203)))
                : (c <= 3212 || (c < 3242
                  ? (c < 3218
                    ? (c >= 3214 && c <= 3216)
                    : c <= 3240)
                  : (c <= 3251 || (c < 3260
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3268)))))
              : (c <= 3272 || (c < 3313
                ? (c < 3293
                  ? (c < 3285
                    ? (c >= 3274 && c <= 3277)
                    : c <= 3286)
                  : (c <= 3294 || (c < 3302
                    ? (c >= 3296 && c <= 3299)
                    : c <= 3311)))
                : (c <= 3314 || (c < 3346
                  ? (c < 3342
                    ? (c >= 3328 && c <= 3340)
                    : c <= 3344)
                  : (c <= 3396 || (c < 3402
                    ? (c >= 3398 && c <= 3400)
                    : c <= 3406)))))))
            : (c <= 3415 || (c < 3542
              ? (c < 3482
                ? (c < 3450
                  ? (c < 3430
                    ? (c >= 3423 && c <= 3427)
                    : c <= 3439)
                  : (c <= 3455 || (c < 3461
                    ? (c >= 3457 && c <= 3459)
                    : c <= 3478)))
                : (c <= 3505 || (c < 3520
                  ? (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)
                  : (c <= 3526 || (c < 3535
                    ? c == 3530
                    : c <= 3540)))))
              : (c <= 3542 || (c < 3664
                ? (c < 3570
                  ? (c < 3558
                    ? (c >= 3544 && c <= 3551)
                    : c <= 3567)
                  : (c <= 3571 || (c < 3648
                    ? (c >= 3585 && c <= 3642)
                    : c <= 3662)))
                : (c <= 3673 || (c < 3718
                  ? (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)
                  : (c <= 3722 || (c >= 3724 && c <= 3747)))))))))))))
      : (c <= 3749 || (c < 7960
        ? (c < 5761
          ? (c < 4304
            ? (c < 3897
              ? (c < 3804
                ? (c < 3782
                  ? (c < 3776
                    ? (c >= 3751 && c <= 3773)
                    : c <= 3780)
                  : (c <= 3782 || (c < 3792
                    ? (c >= 3784 && c <= 3789)
                    : c <= 3801)))
                : (c <= 3807 || (c < 3872
                  ? (c < 3864
                    ? c == 3840
                    : c <= 3865)
                  : (c <= 3881 || (c < 3895
                    ? c == 3893
                    : c <= 3895)))))
              : (c <= 3897 || (c < 4038
                ? (c < 3953
                  ? (c < 3913
                    ? (c >= 3902 && c <= 3911)
                    : c <= 3948)
                  : (c <= 3972 || (c < 3993
                    ? (c >= 3974 && c <= 3991)
                    : c <= 4028)))
                : (c <= 4038 || (c < 4256
                  ? (c < 4176
                    ? (c >= 4096 && c <= 4169)
                    : c <= 4253)
                  : (c <= 4293 || (c < 4301
                    ? c == 4295
                    : c <= 4301)))))))
            : (c <= 4346 || (c < 4802
              ? (c < 4704
                ? (c < 4688
                  ? (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)
                  : (c <= 4694 || (c < 4698
                    ? c == 4696
                    : c <= 4701)))
                : (c <= 4744 || (c < 4786
                  ? (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)
                  : (c <= 4789 || (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)))))
              : (c <= 4805 || (c < 4969
                ? (c < 4882
                  ? (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)
                  : (c <= 4885 || (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)))
                : (c <= 4977 || (c < 5112
                  ? (c < 5024
                    ? (c >= 4992 && c <= 5007)
                    : c <= 5109)
                  : (c <= 5117 || (c < 5743
                    ? (c >= 5121 && c <= 5740)
                    : c <= 5759)))))))))
          : (c <= 5786 || (c < 6576
            ? (c < 6112
              ? (c < 5984
                ? (c < 5888
                  ? (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)
                  : (c <= 5909 || (c < 5952
                    ? (c >= 5919 && c <= 5940)
                    : c <= 5971)))
                : (c <= 5996 || (c < 6016
                  ? (c < 6002
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6003)
                  : (c <= 6099 || (c < 6108
                    ? c == 6103
                    : c <= 6109)))))
              : (c <= 6121 || (c < 6400
                ? (c < 6176
                  ? (c < 6159
                    ? (c >= 6155 && c <= 6157)
                    : c <= 6169)
                  : (c <= 6264 || (c < 6320
                    ? (c >= 6272 && c <= 6314)
                    : c <= 6389)))
                : (c <= 6430 || (c < 6470
                  ? (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)
                  : (c <= 6509 || (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)))))))
            : (c <= 6601 || (c < 7019
              ? (c < 6800
                ? (c < 6688
                  ? (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)
                  : (c <= 6750 || (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)))
                : (c <= 6809 || (c < 6847
                  ? (c < 6832
                    ? c == 6823
                    : c <= 6845)
                  : (c <= 6862 || (c < 6992
                    ? (c >= 6912 && c <= 6988)
                    : c <= 7001)))))
              : (c <= 7027 || (c < 7312
                ? (c < 7232
                  ? (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)
                  : (c <= 7241 || (c < 7296
                    ? (c >= 7245 && c <= 7293)
                    : c <= 7304)))
                : (c <= 7354 || (c < 7380
                  ? (c < 7376
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7378)
                  : (c <= 7418 || (c >= 7424 && c <= 7957)))))))))))
        : (c <= 7965 || (c < 11688
          ? (c < 8417
            ? (c < 8134
              ? (c < 8029
                ? (c < 8016
                  ? (c < 8008
                    ? (c >= 7968 && c <= 8005)
                    : c <= 8013)
                  : (c <= 8023 || (c < 8027
                    ? c == 8025
                    : c <= 8027)))
                : (c <= 8029 || (c < 8118
                  ? (c < 8064
                    ? (c >= 8031 && c <= 8061)
                    : c <= 8116)
                  : (c <= 8124 || (c < 8130
                    ? c == 8126
                    : c <= 8132)))))
              : (c <= 8140 || (c < 8255
                ? (c < 8160
                  ? (c < 8150
                    ? (c >= 8144 && c <= 8147)
                    : c <= 8155)
                  : (c <= 8172 || (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)))
                : (c <= 8256 || (c < 8319
                  ? (c < 8305
                    ? c == 8276
                    : c <= 8305)
                  : (c <= 8319 || (c < 8400
                    ? (c >= 8336 && c <= 8348)
                    : c <= 8412)))))))
            : (c <= 8417 || (c < 8517
              ? (c < 8472
                ? (c < 8455
                  ? (c < 8450
                    ? (c >= 8421 && c <= 8432)
                    : c <= 8450)
                  : (c <= 8455 || (c < 8469
                    ? (c >= 8458 && c <= 8467)
                    : c <= 8469)))
                : (c <= 8477 || (c < 8488
                  ? (c < 8486
                    ? c == 8484
                    : c <= 8486)
                  : (c <= 8488 || (c < 8508
                    ? (c >= 8490 && c <= 8505)
                    : c <= 8511)))))
              : (c <= 8521 || (c < 11559
                ? (c < 11264
                  ? (c < 8544
                    ? c == 8526
                    : c <= 8584)
                  : (c <= 11492 || (c < 11520
                    ? (c >= 11499 && c <= 11507)
                    : c <= 11557)))
                : (c <= 11559 || (c < 11631
                  ? (c < 11568
                    ? c == 11565
                    : c <= 11623)
                  : (c <= 11631 || (c < 11680
                    ? (c >= 11647 && c <= 11670)
                    : c <= 11686)))))))))
          : (c <= 11694 || (c < 42512
            ? (c < 12353
              ? (c < 11736
                ? (c < 11712
                  ? (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)
                  : (c <= 11718 || (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)))
                : (c <= 11742 || (c < 12321
                  ? (c < 12293
                    ? (c >= 11744 && c <= 11775)
                    : c <= 12295)
                  : (c <= 12335 || (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)))))
              : (c <= 12438 || (c < 12704
                ? (c < 12540
                  ? (c < 12449
                    ? (c >= 12441 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)))
                : (c <= 12735 || (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 42124 || (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)))))))
            : (c <= 42539 || (c < 43072
              ? (c < 42891
                ? (c < 42623
                  ? (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)
                  : (c <= 42737 || (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)))
                : (c <= 42954 || (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)))))
              : (c <= 43123 || (c < 43312
                ? (c < 43232
                  ? (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)
                  : (c <= 43255 || (c < 43261
                    ? c == 43259
                    : c <= 43309)))
                : (c <= 43347 || (c < 43471
                  ? (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)
                  : (c <= 43481 || (c >= 43488 && c <= 43518)))))))))))))))
    : (c <= 43574 || (c < 71488
      ? (c < 67647
        ? (c < 65482
          ? (c < 64275
            ? (c < 43816
              ? (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))
              : (c <= 43822 || (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))))
            : (c <= 64279 || (c < 65024
              ? (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64848
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64829)
                  : (c <= 64911 || (c < 65008
                    ? (c >= 64914 && c <= 64967)
                    : c <= 65019)))))
              : (c <= 65039 || (c < 65296
                ? (c < 65101
                  ? (c < 65075
                    ? (c >= 65056 && c <= 65071)
                    : c <= 65076)
                  : (c <= 65103 || (c < 65142
                    ? (c >= 65136 && c <= 65140)
                    : c <= 65276)))
                : (c <= 65305 || (c < 65345
                  ? (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)
                  : (c <= 65370 || (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)))))))))
          : (c <= 65487 || (c < 66736
            ? (c < 66176
              ? (c < 65596
                ? (c < 65536
                  ? (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)
                  : (c <= 65547 || (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)))
                : (c <= 65597 || (c < 65664
                  ? (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)
                  : (c <= 65786 || (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)))))
              : (c <= 66204 || (c < 66432
                ? (c < 66304
                  ? (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)
                  : (c <= 66335 || (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)))))))
            : (c <= 66771 || (c < 67072
              ? (c < 66956
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)))
                : (c <= 66962 || (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))))
              : (c <= 67382 || (c < 67584
                ? (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)))
                : (c <= 67589 || (c < 67639
                  ? (c < 67594
                    ? c == 67592
                    : c <= 67637)
                  : (c <= 67640 || c == 67644))))))))))
        : (c <= 67669 || (c < 70006
          ? (c < 68608
            ? (c < 68117
              ? (c < 67872
                ? (c < 67808
                  ? (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)
                  : (c <= 67826 || (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)))
                : (c <= 67897 || (c < 68096
                  ? (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)
                  : (c <= 68099 || (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)))))
              : (c <= 68119 || (c < 68288
                ? (c < 68159
                  ? (c < 68152
                    ? (c >= 68121 && c <= 68149)
                    : c <= 68154)
                  : (c <= 68159 || (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)))
                : (c <= 68295 || (c < 68416
                  ? (c < 68352
                    ? (c >= 68297 && c <= 68326)
                    : c <= 68405)
                  : (c <= 68437 || (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)))))))
            : (c <= 68680 || (c < 69552
              ? (c < 69291
                ? (c < 68864
                  ? (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)
                  : (c <= 68903 || (c < 69248
                    ? (c >= 68912 && c <= 68921)
                    : c <= 69289)))
                : (c <= 69292 || (c < 69415
                  ? (c < 69376
                    ? (c >= 69296 && c <= 69297)
                    : c <= 69404)
                  : (c <= 69415 || (c < 69488
                    ? (c >= 69424 && c <= 69456)
                    : c <= 69509)))))
              : (c <= 69572 || (c < 69840
                ? (c < 69734
                  ? (c < 69632
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69702)
                  : (c <= 69749 || (c < 69826
                    ? (c >= 69759 && c <= 69818)
                    : c <= 69826)))
                : (c <= 69864 || (c < 69942
                  ? (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)
                  : (c <= 69951 || (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)))))))))
          : (c <= 70006 || (c < 70475
            ? (c < 70303
              ? (c < 70163
                ? (c < 70094
                  ? (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)
                  : (c <= 70106 || (c < 70144
                    ? c == 70108
                    : c <= 70161)))
                : (c <= 70199 || (c < 70280
                  ? (c < 70272
                    ? c == 70206
                    : c <= 70278)
                  : (c <= 70280 || (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)))))
              : (c <= 70312 || (c < 70419
                ? (c < 70400
                  ? (c < 70384
                    ? (c >= 70320 && c <= 70378)
                    : c <= 70393)
                  : (c <= 70403 || (c < 70415
                    ? (c >= 70405 && c <= 70412)
                    : c <= 70416)))
                : (c <= 70440 || (c < 70453
                  ? (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)
                  : (c <= 70457 || (c < 70471
                    ? (c >= 70459 && c <= 70468)
                    : c <= 70472)))))))
            : (c <= 70477 || (c < 71040
              ? (c < 70656
                ? (c < 70493
                  ? (c < 70487
                    ? c == 70480
                    : c <= 70487)
                  : (c <= 70499 || (c < 70512
                    ? (c >= 70502 && c <= 70508)
                    : c <= 70516)))
                : (c <= 70730 || (c < 70784
                  ? (c < 70750
                    ? (c >= 70736 && c <= 70745)
                    : c <= 70753)
                  : (c <= 70853 || (c < 70864
                    ? c == 70855
                    : c <= 70873)))))
              : (c <= 71093 || (c < 71296
                ? (c < 71168
                  ? (c < 71128
                    ? (c >= 71096 && c <= 71104)
                    : c <= 71133)
                  : (c <= 71232 || (c < 71248
                    ? c == 71236
                    : c <= 71257)))
                : (c <= 71352 || (c < 71453
                  ? (c < 71424
                    ? (c >= 71360 && c <= 71369)
                    : c <= 71450)
                  : (c <= 71467 || (c >= 71472 && c <= 71481)))))))))))))
      : (c <= 71494 || (c < 119977
        ? (c < 92160
          ? (c < 72818
            ? (c < 72106
              ? (c < 71957
                ? (c < 71935
                  ? (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)
                  : (c <= 71942 || (c < 71948
                    ? c == 71945
                    : c <= 71955)))
                : (c <= 71958 || (c < 71995
                  ? (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)
                  : (c <= 72003 || (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)))))
              : (c <= 72151 || (c < 72349
                ? (c < 72192
                  ? (c < 72163
                    ? (c >= 72154 && c <= 72161)
                    : c <= 72164)
                  : (c <= 72254 || (c < 72272
                    ? c == 72263
                    : c <= 72345)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72758 || (c < 72784
                    ? (c >= 72760 && c <= 72768)
                    : c <= 72793)))))))
            : (c <= 72847 || (c < 73066
              ? (c < 73018
                ? (c < 72960
                  ? (c < 72873
                    ? (c >= 72850 && c <= 72871)
                    : c <= 72886)
                  : (c <= 72966 || (c < 72971
                    ? (c >= 72968 && c <= 72969)
                    : c <= 73014)))
                : (c <= 73018 || (c < 73040
                  ? (c < 73023
                    ? (c >= 73020 && c <= 73021)
                    : c <= 73031)
                  : (c <= 73049 || (c < 73063
                    ? (c >= 73056 && c <= 73061)
                    : c <= 73064)))))
              : (c <= 73102 || (c < 73728
                ? (c < 73120
                  ? (c < 73107
                    ? (c >= 73104 && c <= 73105)
                    : c <= 73112)
                  : (c <= 73129 || (c < 73648
                    ? (c >= 73440 && c <= 73462)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c < 82944
                    ? (c >= 77824 && c <= 78894)
                    : c <= 83526)))))))))
          : (c <= 92728 || (c < 110589
            ? (c < 93760
              ? (c < 92912
                ? (c < 92784
                  ? (c < 92768
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92777)
                  : (c <= 92862 || (c < 92880
                    ? (c >= 92864 && c <= 92873)
                    : c <= 92909)))
                : (c <= 92916 || (c < 93008
                  ? (c < 92992
                    ? (c >= 92928 && c <= 92982)
                    : c <= 92995)
                  : (c <= 93017 || (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)))))
              : (c <= 93823 || (c < 94192
                ? (c < 94095
                  ? (c < 94031
                    ? (c >= 93952 && c <= 94026)
                    : c <= 94087)
                  : (c <= 94111 || (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)))
                : (c <= 94193 || (c < 101632
                  ? (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)
                  : (c <= 101640 || (c < 110581
                    ? (c >= 110576 && c <= 110579)
                    : c <= 110587)))))))
            : (c <= 110590 || (c < 119141
              ? (c < 113776
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))
                : (c <= 113788 || (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 118576
                    ? (c >= 118528 && c <= 118573)
                    : c <= 118598)))))
              : (c <= 119145 || (c < 119808
                ? (c < 119173
                  ? (c < 119163
                    ? (c >= 119149 && c <= 119154)
                    : c <= 119170)
                  : (c <= 119179 || (c < 119362
                    ? (c >= 119210 && c <= 119213)
                    : c <= 119364)))
                : (c <= 119892 || (c < 119970
                  ? (c < 119966
                    ? (c >= 119894 && c <= 119964)
                    : c <= 119967)
                  : (c <= 119970 || (c >= 119973 && c <= 119974)))))))))))
        : (c <= 119980 || (c < 124912
          ? (c < 120772
            ? (c < 120138
              ? (c < 120077
                ? (c < 119997
                  ? (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)
                  : (c <= 120003 || (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)))
                : (c <= 120084 || (c < 120123
                  ? (c < 120094
                    ? (c >= 120086 && c <= 120092)
                    : c <= 120121)
                  : (c <= 120126 || (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)))))
              : (c <= 120144 || (c < 120598
                ? (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c < 120572
                    ? (c >= 120540 && c <= 120570)
                    : c <= 120596)))
                : (c <= 120628 || (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c < 120746
                    ? (c >= 120714 && c <= 120744)
                    : c <= 120770)))))))
            : (c <= 120779 || (c < 122915
              ? (c < 121499
                ? (c < 121403
                  ? (c < 121344
                    ? (c >= 120782 && c <= 120831)
                    : c <= 121398)
                  : (c <= 121452 || (c < 121476
                    ? c == 121461
                    : c <= 121476)))
                : (c <= 121503 || (c < 122880
                  ? (c < 122624
                    ? (c >= 121505 && c <= 121519)
                    : c <= 122654)
                  : (c <= 122886 || (c < 122907
                    ? (c >= 122888 && c <= 122904)
                    : c <= 122913)))))
              : (c <= 122916 || (c < 123536
                ? (c < 123184
                  ? (c < 123136
                    ? (c >= 122918 && c <= 122922)
                    : c <= 123180)
                  : (c <= 123197 || (c < 123214
                    ? (c >= 123200 && c <= 123209)
                    : c <= 123214)))
                : (c <= 123566 || (c < 124904
                  ? (c < 124896
                    ? (c >= 123584 && c <= 123641)
                    : c <= 124902)
                  : (c <= 124907 || (c >= 124909 && c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static inline bool sym_identifier_character_set_4(int32_t c) {
  return (c < 43520
    ? (c < 3749
      ? (c < 2730
        ? (c < 1984
          ? (c < 910
            ? (c < 216
              ? (c < 'f'
                ? (c < 'A'
                  ? (c < '0'
                    ? c == '-'
                    : c <= '9')
                  : (c <= 'Z' || (c < 'a'
                    ? c == '_'
                    : c <= 'd')))
                : (c <= 'z' || (c < 183
                  ? (c < 181
                    ? c == 170
                    : c <= 181)
                  : (c <= 183 || (c < 192
                    ? c == 186
                    : c <= 214)))))
              : (c <= 246 || (c < 768
                ? (c < 736
                  ? (c < 710
                    ? (c >= 248 && c <= 705)
                    : c <= 721)
                  : (c <= 740 || (c < 750
                    ? c == 748
                    : c <= 750)))
                : (c <= 884 || (c < 895
                  ? (c < 890
                    ? (c >= 886 && c <= 887)
                    : c <= 893)
                  : (c <= 895 || (c < 908
                    ? (c >= 902 && c <= 906)
                    : c <= 908)))))))
            : (c <= 929 || (c < 1479
              ? (c < 1369
                ? (c < 1155
                  ? (c < 1015
                    ? (c >= 931 && c <= 1013)
                    : c <= 1153)
                  : (c <= 1159 || (c < 1329
                    ? (c >= 1162 && c <= 1327)
                    : c <= 1366)))
                : (c <= 1369 || (c < 1471
                  ? (c < 1425
                    ? (c >= 1376 && c <= 1416)
                    : c <= 1469)
                  : (c <= 1471 || (c < 1476
                    ? (c >= 1473 && c <= 1474)
                    : c <= 1477)))))
              : (c <= 1479 || (c < 1749
                ? (c < 1552
                  ? (c < 1519
                    ? (c >= 1488 && c <= 1514)
                    : c <= 1522)
                  : (c <= 1562 || (c < 1646
                    ? (c >= 1568 && c <= 1641)
                    : c <= 1747)))
                : (c <= 1756 || (c < 1791
                  ? (c < 1770
                    ? (c >= 1759 && c <= 1768)
                    : c <= 1788)
                  : (c <= 1791 || (c < 1869
                    ? (c >= 1808 && c <= 1866)
                    : c <= 1969)))))))))
          : (c <= 2037 || (c < 2534
            ? (c < 2437
              ? (c < 2160
                ? (c < 2048
                  ? (c < 2045
                    ? c == 2042
                    : c <= 2045)
                  : (c <= 2093 || (c < 2144
                    ? (c >= 2112 && c <= 2139)
                    : c <= 2154)))
                : (c <= 2183 || (c < 2275
                  ? (c < 2200
                    ? (c >= 2185 && c <= 2190)
                    : c <= 2273)
                  : (c <= 2403 || (c < 2417
                    ? (c >= 2406 && c <= 2415)
                    : c <= 2435)))))
              : (c <= 2444 || (c < 2492
                ? (c < 2474
                  ? (c < 2451
                    ? (c >= 2447 && c <= 2448)
                    : c <= 2472)
                  : (c <= 2480 || (c < 2486
                    ? c == 2482
                    : c <= 2489)))
                : (c <= 2500 || (c < 2519
                  ? (c < 2507
                    ? (c >= 2503 && c <= 2504)
                    : c <= 2510)
                  : (c <= 2519 || (c < 2527
                    ? (c >= 2524 && c <= 2525)
                    : c <= 2531)))))))
            : (c <= 2545 || (c < 2622
              ? (c < 2579
                ? (c < 2561
                  ? (c < 2558
                    ? c == 2556
                    : c <= 2558)
                  : (c <= 2563 || (c < 2575
                    ? (c >= 2565 && c <= 2570)
                    : c <= 2576)))
                : (c <= 2600 || (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c < 2620
                    ? (c >= 2616 && c <= 2617)
                    : c <= 2620)))))
              : (c <= 2626 || (c < 2662
                ? (c < 2641
                  ? (c < 2635
                    ? (c >= 2631 && c <= 2632)
                    : c <= 2637)
                  : (c <= 2641 || (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)))
                : (c <= 2677 || (c < 2703
                  ? (c < 2693
                    ? (c >= 2689 && c <= 2691)
                    : c <= 2701)
                  : (c <= 2705 || (c >= 2707 && c <= 2728)))))))))))
        : (c <= 2736 || (c < 3157
          ? (c < 2929
            ? (c < 2831
              ? (c < 2768
                ? (c < 2748
                  ? (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)
                  : (c <= 2757 || (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)))
                : (c <= 2768 || (c < 2809
                  ? (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)
                  : (c <= 2815 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))))
              : (c <= 2832 || (c < 2887
                ? (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)))
                : (c <= 2888 || (c < 2908
                  ? (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)
                  : (c <= 2909 || (c < 2918
                    ? (c >= 2911 && c <= 2915)
                    : c <= 2927)))))))
            : (c <= 2929 || (c < 3014
              ? (c < 2972
                ? (c < 2958
                  ? (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)
                  : (c <= 2960 || (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)))
                : (c <= 2972 || (c < 2984
                  ? (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)
                  : (c <= 2986 || (c < 3006
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3010)))))
              : (c <= 3016 || (c < 3086
                ? (c < 3031
                  ? (c < 3024
                    ? (c >= 3018 && c <= 3021)
                    : c <= 3024)
                  : (c <= 3031 || (c < 3072
                    ? (c >= 3046 && c <= 3055)
                    : c <= 3084)))
                : (c <= 3088 || (c < 3132
                  ? (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)
                  : (c <= 3140 || (c < 3146
                    ? (c >= 3142 && c <= 3144)
                    : c <= 3149)))))))))
          : (c <= 3158 || (c < 3412
            ? (c < 3270
              ? (c < 3205
                ? (c < 3168
                  ? (c < 3165
                    ? (c >= 3160 && c <= 3162)
                    : c <= 3165)
                  : (c <= 3171 || (c < 3200
                    ? (c >= 3174 && c <= 3183)
                    : c <= 3203)))
                : (c <= 3212 || (c < 3242
                  ? (c < 3218
                    ? (c >= 3214 && c <= 3216)
                    : c <= 3240)
                  : (c <= 3251 || (c < 3260
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3268)))))
              : (c <= 3272 || (c < 3313
                ? (c < 3293
                  ? (c < 3285
                    ? (c >= 3274 && c <= 3277)
                    : c <= 3286)
                  : (c <= 3294 || (c < 3302
                    ? (c >= 3296 && c <= 3299)
                    : c <= 3311)))
                : (c <= 3314 || (c < 3346
                  ? (c < 3342
                    ? (c >= 3328 && c <= 3340)
                    : c <= 3344)
                  : (c <= 3396 || (c < 3402
                    ? (c >= 3398 && c <= 3400)
                    : c <= 3406)))))))
            : (c <= 3415 || (c < 3542
              ? (c < 3482
                ? (c < 3450
                  ? (c < 3430
                    ? (c >= 3423 && c <= 3427)
                    : c <= 3439)
                  : (c <= 3455 || (c < 3461
                    ? (c >= 3457 && c <= 3459)
                    : c <= 3478)))
                : (c <= 3505 || (c < 3520
                  ? (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)
                  : (c <= 3526 || (c < 3535
                    ? c == 3530
                    : c <= 3540)))))
              : (c <= 3542 || (c < 3664
                ? (c < 3570
                  ? (c < 3558
                    ? (c >= 3544 && c <= 3551)
                    : c <= 3567)
                  : (c <= 3571 || (c < 3648
                    ? (c >= 3585 && c <= 3642)
                    : c <= 3662)))
                : (c <= 3673 || (c < 3718
                  ? (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)
                  : (c <= 3722 || (c >= 3724 && c <= 3747)))))))))))))
      : (c <= 3749 || (c < 7960
        ? (c < 5761
          ? (c < 4304
            ? (c < 3897
              ? (c < 3804
                ? (c < 3782
                  ? (c < 3776
                    ? (c >= 3751 && c <= 3773)
                    : c <= 3780)
                  : (c <= 3782 || (c < 3792
                    ? (c >= 3784 && c <= 3789)
                    : c <= 3801)))
                : (c <= 3807 || (c < 3872
                  ? (c < 3864
                    ? c == 3840
                    : c <= 3865)
                  : (c <= 3881 || (c < 3895
                    ? c == 3893
                    : c <= 3895)))))
              : (c <= 3897 || (c < 4038
                ? (c < 3953
                  ? (c < 3913
                    ? (c >= 3902 && c <= 3911)
                    : c <= 3948)
                  : (c <= 3972 || (c < 3993
                    ? (c >= 3974 && c <= 3991)
                    : c <= 4028)))
                : (c <= 4038 || (c < 4256
                  ? (c < 4176
                    ? (c >= 4096 && c <= 4169)
                    : c <= 4253)
                  : (c <= 4293 || (c < 4301
                    ? c == 4295
                    : c <= 4301)))))))
            : (c <= 4346 || (c < 4802
              ? (c < 4704
                ? (c < 4688
                  ? (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)
                  : (c <= 4694 || (c < 4698
                    ? c == 4696
                    : c <= 4701)))
                : (c <= 4744 || (c < 4786
                  ? (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)
                  : (c <= 4789 || (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)))))
              : (c <= 4805 || (c < 4969
                ? (c < 4882
                  ? (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)
                  : (c <= 4885 || (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)))
                : (c <= 4977 || (c < 5112
                  ? (c < 5024
                    ? (c >= 4992 && c <= 5007)
                    : c <= 5109)
                  : (c <= 5117 || (c < 5743
                    ? (c >= 5121 && c <= 5740)
                    : c <= 5759)))))))))
          : (c <= 5786 || (c < 6576
            ? (c < 6112
              ? (c < 5984
                ? (c < 5888
                  ? (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)
                  : (c <= 5909 || (c < 5952
                    ? (c >= 5919 && c <= 5940)
                    : c <= 5971)))
                : (c <= 5996 || (c < 6016
                  ? (c < 6002
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6003)
                  : (c <= 6099 || (c < 6108
                    ? c == 6103
                    : c <= 6109)))))
              : (c <= 6121 || (c < 6400
                ? (c < 6176
                  ? (c < 6159
                    ? (c >= 6155 && c <= 6157)
                    : c <= 6169)
                  : (c <= 6264 || (c < 6320
                    ? (c >= 6272 && c <= 6314)
                    : c <= 6389)))
                : (c <= 6430 || (c < 6470
                  ? (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)
                  : (c <= 6509 || (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)))))))
            : (c <= 6601 || (c < 7019
              ? (c < 6800
                ? (c < 6688
                  ? (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)
                  : (c <= 6750 || (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)))
                : (c <= 6809 || (c < 6847
                  ? (c < 6832
                    ? c == 6823
                    : c <= 6845)
                  : (c <= 6862 || (c < 6992
                    ? (c >= 6912 && c <= 6988)
                    : c <= 7001)))))
              : (c <= 7027 || (c < 7312
                ? (c < 7232
                  ? (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)
                  : (c <= 7241 || (c < 7296
                    ? (c >= 7245 && c <= 7293)
                    : c <= 7304)))
                : (c <= 7354 || (c < 7380
                  ? (c < 7376
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7378)
                  : (c <= 7418 || (c >= 7424 && c <= 7957)))))))))))
        : (c <= 7965 || (c < 11688
          ? (c < 8417
            ? (c < 8134
              ? (c < 8029
                ? (c < 8016
                  ? (c < 8008
                    ? (c >= 7968 && c <= 8005)
                    : c <= 8013)
                  : (c <= 8023 || (c < 8027
                    ? c == 8025
                    : c <= 8027)))
                : (c <= 8029 || (c < 8118
                  ? (c < 8064
                    ? (c >= 8031 && c <= 8061)
                    : c <= 8116)
                  : (c <= 8124 || (c < 8130
                    ? c == 8126
                    : c <= 8132)))))
              : (c <= 8140 || (c < 8255
                ? (c < 8160
                  ? (c < 8150
                    ? (c >= 8144 && c <= 8147)
                    : c <= 8155)
                  : (c <= 8172 || (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)))
                : (c <= 8256 || (c < 8319
                  ? (c < 8305
                    ? c == 8276
                    : c <= 8305)
                  : (c <= 8319 || (c < 8400
                    ? (c >= 8336 && c <= 8348)
                    : c <= 8412)))))))
            : (c <= 8417 || (c < 8517
              ? (c < 8472
                ? (c < 8455
                  ? (c < 8450
                    ? (c >= 8421 && c <= 8432)
                    : c <= 8450)
                  : (c <= 8455 || (c < 8469
                    ? (c >= 8458 && c <= 8467)
                    : c <= 8469)))
                : (c <= 8477 || (c < 8488
                  ? (c < 8486
                    ? c == 8484
                    : c <= 8486)
                  : (c <= 8488 || (c < 8508
                    ? (c >= 8490 && c <= 8505)
                    : c <= 8511)))))
              : (c <= 8521 || (c < 11559
                ? (c < 11264
                  ? (c < 8544
                    ? c == 8526
                    : c <= 8584)
                  : (c <= 11492 || (c < 11520
                    ? (c >= 11499 && c <= 11507)
                    : c <= 11557)))
                : (c <= 11559 || (c < 11631
                  ? (c < 11568
                    ? c == 11565
                    : c <= 11623)
                  : (c <= 11631 || (c < 11680
                    ? (c >= 11647 && c <= 11670)
                    : c <= 11686)))))))))
          : (c <= 11694 || (c < 42512
            ? (c < 12353
              ? (c < 11736
                ? (c < 11712
                  ? (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)
                  : (c <= 11718 || (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)))
                : (c <= 11742 || (c < 12321
                  ? (c < 12293
                    ? (c >= 11744 && c <= 11775)
                    : c <= 12295)
                  : (c <= 12335 || (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)))))
              : (c <= 12438 || (c < 12704
                ? (c < 12540
                  ? (c < 12449
                    ? (c >= 12441 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)))
                : (c <= 12735 || (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 42124 || (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)))))))
            : (c <= 42539 || (c < 43072
              ? (c < 42891
                ? (c < 42623
                  ? (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)
                  : (c <= 42737 || (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)))
                : (c <= 42954 || (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)))))
              : (c <= 43123 || (c < 43312
                ? (c < 43232
                  ? (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)
                  : (c <= 43255 || (c < 43261
                    ? c == 43259
                    : c <= 43309)))
                : (c <= 43347 || (c < 43471
                  ? (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)
                  : (c <= 43481 || (c >= 43488 && c <= 43518)))))))))))))))
    : (c <= 43574 || (c < 71488
      ? (c < 67647
        ? (c < 65482
          ? (c < 64275
            ? (c < 43816
              ? (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))
              : (c <= 43822 || (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))))
            : (c <= 64279 || (c < 65024
              ? (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64848
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64829)
                  : (c <= 64911 || (c < 65008
                    ? (c >= 64914 && c <= 64967)
                    : c <= 65019)))))
              : (c <= 65039 || (c < 65296
                ? (c < 65101
                  ? (c < 65075
                    ? (c >= 65056 && c <= 65071)
                    : c <= 65076)
                  : (c <= 65103 || (c < 65142
                    ? (c >= 65136 && c <= 65140)
                    : c <= 65276)))
                : (c <= 65305 || (c < 65345
                  ? (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)
                  : (c <= 65370 || (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)))))))))
          : (c <= 65487 || (c < 66736
            ? (c < 66176
              ? (c < 65596
                ? (c < 65536
                  ? (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)
                  : (c <= 65547 || (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)))
                : (c <= 65597 || (c < 65664
                  ? (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)
                  : (c <= 65786 || (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)))))
              : (c <= 66204 || (c < 66432
                ? (c < 66304
                  ? (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)
                  : (c <= 66335 || (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)))))))
            : (c <= 66771 || (c < 67072
              ? (c < 66956
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)))
                : (c <= 66962 || (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))))
              : (c <= 67382 || (c < 67584
                ? (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)))
                : (c <= 67589 || (c < 67639
                  ? (c < 67594
                    ? c == 67592
                    : c <= 67637)
                  : (c <= 67640 || c == 67644))))))))))
        : (c <= 67669 || (c < 70006
          ? (c < 68608
            ? (c < 68117
              ? (c < 67872
                ? (c < 67808
                  ? (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)
                  : (c <= 67826 || (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)))
                : (c <= 67897 || (c < 68096
                  ? (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)
                  : (c <= 68099 || (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)))))
              : (c <= 68119 || (c < 68288
                ? (c < 68159
                  ? (c < 68152
                    ? (c >= 68121 && c <= 68149)
                    : c <= 68154)
                  : (c <= 68159 || (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)))
                : (c <= 68295 || (c < 68416
                  ? (c < 68352
                    ? (c >= 68297 && c <= 68326)
                    : c <= 68405)
                  : (c <= 68437 || (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)))))))
            : (c <= 68680 || (c < 69552
              ? (c < 69291
                ? (c < 68864
                  ? (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)
                  : (c <= 68903 || (c < 69248
                    ? (c >= 68912 && c <= 68921)
                    : c <= 69289)))
                : (c <= 69292 || (c < 69415
                  ? (c < 69376
                    ? (c >= 69296 && c <= 69297)
                    : c <= 69404)
                  : (c <= 69415 || (c < 69488
                    ? (c >= 69424 && c <= 69456)
                    : c <= 69509)))))
              : (c <= 69572 || (c < 69840
                ? (c < 69734
                  ? (c < 69632
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69702)
                  : (c <= 69749 || (c < 69826
                    ? (c >= 69759 && c <= 69818)
                    : c <= 69826)))
                : (c <= 69864 || (c < 69942
                  ? (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)
                  : (c <= 69951 || (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)))))))))
          : (c <= 70006 || (c < 70475
            ? (c < 70303
              ? (c < 70163
                ? (c < 70094
                  ? (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)
                  : (c <= 70106 || (c < 70144
                    ? c == 70108
                    : c <= 70161)))
                : (c <= 70199 || (c < 70280
                  ? (c < 70272
                    ? c == 70206
                    : c <= 70278)
                  : (c <= 70280 || (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)))))
              : (c <= 70312 || (c < 70419
                ? (c < 70400
                  ? (c < 70384
                    ? (c >= 70320 && c <= 70378)
                    : c <= 70393)
                  : (c <= 70403 || (c < 70415
                    ? (c >= 70405 && c <= 70412)
                    : c <= 70416)))
                : (c <= 70440 || (c < 70453
                  ? (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)
                  : (c <= 70457 || (c < 70471
                    ? (c >= 70459 && c <= 70468)
                    : c <= 70472)))))))
            : (c <= 70477 || (c < 71040
              ? (c < 70656
                ? (c < 70493
                  ? (c < 70487
                    ? c == 70480
                    : c <= 70487)
                  : (c <= 70499 || (c < 70512
                    ? (c >= 70502 && c <= 70508)
                    : c <= 70516)))
                : (c <= 70730 || (c < 70784
                  ? (c < 70750
                    ? (c >= 70736 && c <= 70745)
                    : c <= 70753)
                  : (c <= 70853 || (c < 70864
                    ? c == 70855
                    : c <= 70873)))))
              : (c <= 71093 || (c < 71296
                ? (c < 71168
                  ? (c < 71128
                    ? (c >= 71096 && c <= 71104)
                    : c <= 71133)
                  : (c <= 71232 || (c < 71248
                    ? c == 71236
                    : c <= 71257)))
                : (c <= 71352 || (c < 71453
                  ? (c < 71424
                    ? (c >= 71360 && c <= 71369)
                    : c <= 71450)
                  : (c <= 71467 || (c >= 71472 && c <= 71481)))))))))))))
      : (c <= 71494 || (c < 119977
        ? (c < 92160
          ? (c < 72818
            ? (c < 72106
              ? (c < 71957
                ? (c < 71935
                  ? (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)
                  : (c <= 71942 || (c < 71948
                    ? c == 71945
                    : c <= 71955)))
                : (c <= 71958 || (c < 71995
                  ? (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)
                  : (c <= 72003 || (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)))))
              : (c <= 72151 || (c < 72349
                ? (c < 72192
                  ? (c < 72163
                    ? (c >= 72154 && c <= 72161)
                    : c <= 72164)
                  : (c <= 72254 || (c < 72272
                    ? c == 72263
                    : c <= 72345)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72758 || (c < 72784
                    ? (c >= 72760 && c <= 72768)
                    : c <= 72793)))))))
            : (c <= 72847 || (c < 73066
              ? (c < 73018
                ? (c < 72960
                  ? (c < 72873
                    ? (c >= 72850 && c <= 72871)
                    : c <= 72886)
                  : (c <= 72966 || (c < 72971
                    ? (c >= 72968 && c <= 72969)
                    : c <= 73014)))
                : (c <= 73018 || (c < 73040
                  ? (c < 73023
                    ? (c >= 73020 && c <= 73021)
                    : c <= 73031)
                  : (c <= 73049 || (c < 73063
                    ? (c >= 73056 && c <= 73061)
                    : c <= 73064)))))
              : (c <= 73102 || (c < 73728
                ? (c < 73120
                  ? (c < 73107
                    ? (c >= 73104 && c <= 73105)
                    : c <= 73112)
                  : (c <= 73129 || (c < 73648
                    ? (c >= 73440 && c <= 73462)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c < 82944
                    ? (c >= 77824 && c <= 78894)
                    : c <= 83526)))))))))
          : (c <= 92728 || (c < 110589
            ? (c < 93760
              ? (c < 92912
                ? (c < 92784
                  ? (c < 92768
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92777)
                  : (c <= 92862 || (c < 92880
                    ? (c >= 92864 && c <= 92873)
                    : c <= 92909)))
                : (c <= 92916 || (c < 93008
                  ? (c < 92992
                    ? (c >= 92928 && c <= 92982)
                    : c <= 92995)
                  : (c <= 93017 || (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)))))
              : (c <= 93823 || (c < 94192
                ? (c < 94095
                  ? (c < 94031
                    ? (c >= 93952 && c <= 94026)
                    : c <= 94087)
                  : (c <= 94111 || (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)))
                : (c <= 94193 || (c < 101632
                  ? (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)
                  : (c <= 101640 || (c < 110581
                    ? (c >= 110576 && c <= 110579)
                    : c <= 110587)))))))
            : (c <= 110590 || (c < 119141
              ? (c < 113776
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))
                : (c <= 113788 || (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 118576
                    ? (c >= 118528 && c <= 118573)
                    : c <= 118598)))))
              : (c <= 119145 || (c < 119808
                ? (c < 119173
                  ? (c < 119163
                    ? (c >= 119149 && c <= 119154)
                    : c <= 119170)
                  : (c <= 119179 || (c < 119362
                    ? (c >= 119210 && c <= 119213)
                    : c <= 119364)))
                : (c <= 119892 || (c < 119970
                  ? (c < 119966
                    ? (c >= 119894 && c <= 119964)
                    : c <= 119967)
                  : (c <= 119970 || (c >= 119973 && c <= 119974)))))))))))
        : (c <= 119980 || (c < 124912
          ? (c < 120772
            ? (c < 120138
              ? (c < 120077
                ? (c < 119997
                  ? (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)
                  : (c <= 120003 || (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)))
                : (c <= 120084 || (c < 120123
                  ? (c < 120094
                    ? (c >= 120086 && c <= 120092)
                    : c <= 120121)
                  : (c <= 120126 || (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)))))
              : (c <= 120144 || (c < 120598
                ? (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c < 120572
                    ? (c >= 120540 && c <= 120570)
                    : c <= 120596)))
                : (c <= 120628 || (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c < 120746
                    ? (c >= 120714 && c <= 120744)
                    : c <= 120770)))))))
            : (c <= 120779 || (c < 122915
              ? (c < 121499
                ? (c < 121403
                  ? (c < 121344
                    ? (c >= 120782 && c <= 120831)
                    : c <= 121398)
                  : (c <= 121452 || (c < 121476
                    ? c == 121461
                    : c <= 121476)))
                : (c <= 121503 || (c < 122880
                  ? (c < 122624
                    ? (c >= 121505 && c <= 121519)
                    : c <= 122654)
                  : (c <= 122886 || (c < 122907
                    ? (c >= 122888 && c <= 122904)
                    : c <= 122913)))))
              : (c <= 122916 || (c < 123536
                ? (c < 123184
                  ? (c < 123136
                    ? (c >= 122918 && c <= 122922)
                    : c <= 123180)
                  : (c <= 123197 || (c < 123214
                    ? (c >= 123200 && c <= 123209)
                    : c <= 123214)))
                : (c <= 123566 || (c < 124904
                  ? (c < 124896
                    ? (c >= 123584 && c <= 123641)
                    : c <= 124902)
                  : (c <= 124907 || (c >= 124909 && c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static inline bool sym_identifier_character_set_5(int32_t c) {
  return (c < 43520
    ? (c < 3749
      ? (c < 2730
        ? (c < 1984
          ? (c < 910
            ? (c < 216
              ? (c < 'm'
                ? (c < 'A'
                  ? (c < '0'
                    ? c == '-'
                    : c <= '9')
                  : (c <= 'Z' || (c < 'a'
                    ? c == '_'
                    : c <= 'k')))
                : (c <= 'z' || (c < 183
                  ? (c < 181
                    ? c == 170
                    : c <= 181)
                  : (c <= 183 || (c < 192
                    ? c == 186
                    : c <= 214)))))
              : (c <= 246 || (c < 768
                ? (c < 736
                  ? (c < 710
                    ? (c >= 248 && c <= 705)
                    : c <= 721)
                  : (c <= 740 || (c < 750
                    ? c == 748
                    : c <= 750)))
                : (c <= 884 || (c < 895
                  ? (c < 890
                    ? (c >= 886 && c <= 887)
                    : c <= 893)
                  : (c <= 895 || (c < 908
                    ? (c >= 902 && c <= 906)
                    : c <= 908)))))))
            : (c <= 929 || (c < 1479
              ? (c < 1369
                ? (c < 1155
                  ? (c < 1015
                    ? (c >= 931 && c <= 1013)
                    : c <= 1153)
                  : (c <= 1159 || (c < 1329
                    ? (c >= 1162 && c <= 1327)
                    : c <= 1366)))
                : (c <= 1369 || (c < 1471
                  ? (c < 1425
                    ? (c >= 1376 && c <= 1416)
                    : c <= 1469)
                  : (c <= 1471 || (c < 1476
                    ? (c >= 1473 && c <= 1474)
                    : c <= 1477)))))
              : (c <= 1479 || (c < 1749
                ? (c < 1552
                  ? (c < 1519
                    ? (c >= 1488 && c <= 1514)
                    : c <= 1522)
                  : (c <= 1562 || (c < 1646
                    ? (c >= 1568 && c <= 1641)
                    : c <= 1747)))
                : (c <= 1756 || (c < 1791
                  ? (c < 1770
                    ? (c >= 1759 && c <= 1768)
                    : c <= 1788)
                  : (c <= 1791 || (c < 1869
                    ? (c >= 1808 && c <= 1866)
                    : c <= 1969)))))))))
          : (c <= 2037 || (c < 2534
            ? (c < 2437
              ? (c < 2160
                ? (c < 2048
                  ? (c < 2045
                    ? c == 2042
                    : c <= 2045)
                  : (c <= 2093 || (c < 2144
                    ? (c >= 2112 && c <= 2139)
                    : c <= 2154)))
                : (c <= 2183 || (c < 2275
                  ? (c < 2200
                    ? (c >= 2185 && c <= 2190)
                    : c <= 2273)
                  : (c <= 2403 || (c < 2417
                    ? (c >= 2406 && c <= 2415)
                    : c <= 2435)))))
              : (c <= 2444 || (c < 2492
                ? (c < 2474
                  ? (c < 2451
                    ? (c >= 2447 && c <= 2448)
                    : c <= 2472)
                  : (c <= 2480 || (c < 2486
                    ? c == 2482
                    : c <= 2489)))
                : (c <= 2500 || (c < 2519
                  ? (c < 2507
                    ? (c >= 2503 && c <= 2504)
                    : c <= 2510)
                  : (c <= 2519 || (c < 2527
                    ? (c >= 2524 && c <= 2525)
                    : c <= 2531)))))))
            : (c <= 2545 || (c < 2622
              ? (c < 2579
                ? (c < 2561
                  ? (c < 2558
                    ? c == 2556
                    : c <= 2558)
                  : (c <= 2563 || (c < 2575
                    ? (c >= 2565 && c <= 2570)
                    : c <= 2576)))
                : (c <= 2600 || (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c < 2620
                    ? (c >= 2616 && c <= 2617)
                    : c <= 2620)))))
              : (c <= 2626 || (c < 2662
                ? (c < 2641
                  ? (c < 2635
                    ? (c >= 2631 && c <= 2632)
                    : c <= 2637)
                  : (c <= 2641 || (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)))
                : (c <= 2677 || (c < 2703
                  ? (c < 2693
                    ? (c >= 2689 && c <= 2691)
                    : c <= 2701)
                  : (c <= 2705 || (c >= 2707 && c <= 2728)))))))))))
        : (c <= 2736 || (c < 3157
          ? (c < 2929
            ? (c < 2831
              ? (c < 2768
                ? (c < 2748
                  ? (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)
                  : (c <= 2757 || (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)))
                : (c <= 2768 || (c < 2809
                  ? (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)
                  : (c <= 2815 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))))
              : (c <= 2832 || (c < 2887
                ? (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)))
                : (c <= 2888 || (c < 2908
                  ? (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)
                  : (c <= 2909 || (c < 2918
                    ? (c >= 2911 && c <= 2915)
                    : c <= 2927)))))))
            : (c <= 2929 || (c < 3014
              ? (c < 2972
                ? (c < 2958
                  ? (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)
                  : (c <= 2960 || (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)))
                : (c <= 2972 || (c < 2984
                  ? (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)
                  : (c <= 2986 || (c < 3006
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3010)))))
              : (c <= 3016 || (c < 3086
                ? (c < 3031
                  ? (c < 3024
                    ? (c >= 3018 && c <= 3021)
                    : c <= 3024)
                  : (c <= 3031 || (c < 3072
                    ? (c >= 3046 && c <= 3055)
                    : c <= 3084)))
                : (c <= 3088 || (c < 3132
                  ? (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)
                  : (c <= 3140 || (c < 3146
                    ? (c >= 3142 && c <= 3144)
                    : c <= 3149)))))))))
          : (c <= 3158 || (c < 3412
            ? (c < 3270
              ? (c < 3205
                ? (c < 3168
                  ? (c < 3165
                    ? (c >= 3160 && c <= 3162)
                    : c <= 3165)
                  : (c <= 3171 || (c < 3200
                    ? (c >= 3174 && c <= 3183)
                    : c <= 3203)))
                : (c <= 3212 || (c < 3242
                  ? (c < 3218
                    ? (c >= 3214 && c <= 3216)
                    : c <= 3240)
                  : (c <= 3251 || (c < 3260
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3268)))))
              : (c <= 3272 || (c < 3313
                ? (c < 3293
                  ? (c < 3285
                    ? (c >= 3274 && c <= 3277)
                    : c <= 3286)
                  : (c <= 3294 || (c < 3302
                    ? (c >= 3296 && c <= 3299)
                    : c <= 3311)))
                : (c <= 3314 || (c < 3346
                  ? (c < 3342
                    ? (c >= 3328 && c <= 3340)
                    : c <= 3344)
                  : (c <= 3396 || (c < 3402
                    ? (c >= 3398 && c <= 3400)
                    : c <= 3406)))))))
            : (c <= 3415 || (c < 3542
              ? (c < 3482
                ? (c < 3450
                  ? (c < 3430
                    ? (c >= 3423 && c <= 3427)
                    : c <= 3439)
                  : (c <= 3455 || (c < 3461
                    ? (c >= 3457 && c <= 3459)
                    : c <= 3478)))
                : (c <= 3505 || (c < 3520
                  ? (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)
                  : (c <= 3526 || (c < 3535
                    ? c == 3530
                    : c <= 3540)))))
              : (c <= 3542 || (c < 3664
                ? (c < 3570
                  ? (c < 3558
                    ? (c >= 3544 && c <= 3551)
                    : c <= 3567)
                  : (c <= 3571 || (c < 3648
                    ? (c >= 3585 && c <= 3642)
                    : c <= 3662)))
                : (c <= 3673 || (c < 3718
                  ? (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)
                  : (c <= 3722 || (c >= 3724 && c <= 3747)))))))))))))
      : (c <= 3749 || (c < 7960
        ? (c < 5761
          ? (c < 4304
            ? (c < 3897
              ? (c < 3804
                ? (c < 3782
                  ? (c < 3776
                    ? (c >= 3751 && c <= 3773)
                    : c <= 3780)
                  : (c <= 3782 || (c < 3792
                    ? (c >= 3784 && c <= 3789)
                    : c <= 3801)))
                : (c <= 3807 || (c < 3872
                  ? (c < 3864
                    ? c == 3840
                    : c <= 3865)
                  : (c <= 3881 || (c < 3895
                    ? c == 3893
                    : c <= 3895)))))
              : (c <= 3897 || (c < 4038
                ? (c < 3953
                  ? (c < 3913
                    ? (c >= 3902 && c <= 3911)
                    : c <= 3948)
                  : (c <= 3972 || (c < 3993
                    ? (c >= 3974 && c <= 3991)
                    : c <= 4028)))
                : (c <= 4038 || (c < 4256
                  ? (c < 4176
                    ? (c >= 4096 && c <= 4169)
                    : c <= 4253)
                  : (c <= 4293 || (c < 4301
                    ? c == 4295
                    : c <= 4301)))))))
            : (c <= 4346 || (c < 4802
              ? (c < 4704
                ? (c < 4688
                  ? (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)
                  : (c <= 4694 || (c < 4698
                    ? c == 4696
                    : c <= 4701)))
                : (c <= 4744 || (c < 4786
                  ? (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)
                  : (c <= 4789 || (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)))))
              : (c <= 4805 || (c < 4969
                ? (c < 4882
                  ? (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)
                  : (c <= 4885 || (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)))
                : (c <= 4977 || (c < 5112
                  ? (c < 5024
                    ? (c >= 4992 && c <= 5007)
                    : c <= 5109)
                  : (c <= 5117 || (c < 5743
                    ? (c >= 5121 && c <= 5740)
                    : c <= 5759)))))))))
          : (c <= 5786 || (c < 6576
            ? (c < 6112
              ? (c < 5984
                ? (c < 5888
                  ? (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)
                  : (c <= 5909 || (c < 5952
                    ? (c >= 5919 && c <= 5940)
                    : c <= 5971)))
                : (c <= 5996 || (c < 6016
                  ? (c < 6002
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6003)
                  : (c <= 6099 || (c < 6108
                    ? c == 6103
                    : c <= 6109)))))
              : (c <= 6121 || (c < 6400
                ? (c < 6176
                  ? (c < 6159
                    ? (c >= 6155 && c <= 6157)
                    : c <= 6169)
                  : (c <= 6264 || (c < 6320
                    ? (c >= 6272 && c <= 6314)
                    : c <= 6389)))
                : (c <= 6430 || (c < 6470
                  ? (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)
                  : (c <= 6509 || (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)))))))
            : (c <= 6601 || (c < 7019
              ? (c < 6800
                ? (c < 6688
                  ? (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)
                  : (c <= 6750 || (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)))
                : (c <= 6809 || (c < 6847
                  ? (c < 6832
                    ? c == 6823
                    : c <= 6845)
                  : (c <= 6862 || (c < 6992
                    ? (c >= 6912 && c <= 6988)
                    : c <= 7001)))))
              : (c <= 7027 || (c < 7312
                ? (c < 7232
                  ? (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)
                  : (c <= 7241 || (c < 7296
                    ? (c >= 7245 && c <= 7293)
                    : c <= 7304)))
                : (c <= 7354 || (c < 7380
                  ? (c < 7376
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7378)
                  : (c <= 7418 || (c >= 7424 && c <= 7957)))))))))))
        : (c <= 7965 || (c < 11688
          ? (c < 8417
            ? (c < 8134
              ? (c < 8029
                ? (c < 8016
                  ? (c < 8008
                    ? (c >= 7968 && c <= 8005)
                    : c <= 8013)
                  : (c <= 8023 || (c < 8027
                    ? c == 8025
                    : c <= 8027)))
                : (c <= 8029 || (c < 8118
                  ? (c < 8064
                    ? (c >= 8031 && c <= 8061)
                    : c <= 8116)
                  : (c <= 8124 || (c < 8130
                    ? c == 8126
                    : c <= 8132)))))
              : (c <= 8140 || (c < 8255
                ? (c < 8160
                  ? (c < 8150
                    ? (c >= 8144 && c <= 8147)
                    : c <= 8155)
                  : (c <= 8172 || (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)))
                : (c <= 8256 || (c < 8319
                  ? (c < 8305
                    ? c == 8276
                    : c <= 8305)
                  : (c <= 8319 || (c < 8400
                    ? (c >= 8336 && c <= 8348)
                    : c <= 8412)))))))
            : (c <= 8417 || (c < 8517
              ? (c < 8472
                ? (c < 8455
                  ? (c < 8450
                    ? (c >= 8421 && c <= 8432)
                    : c <= 8450)
                  : (c <= 8455 || (c < 8469
                    ? (c >= 8458 && c <= 8467)
                    : c <= 8469)))
                : (c <= 8477 || (c < 8488
                  ? (c < 8486
                    ? c == 8484
                    : c <= 8486)
                  : (c <= 8488 || (c < 8508
                    ? (c >= 8490 && c <= 8505)
                    : c <= 8511)))))
              : (c <= 8521 || (c < 11559
                ? (c < 11264
                  ? (c < 8544
                    ? c == 8526
                    : c <= 8584)
                  : (c <= 11492 || (c < 11520
                    ? (c >= 11499 && c <= 11507)
                    : c <= 11557)))
                : (c <= 11559 || (c < 11631
                  ? (c < 11568
                    ? c == 11565
                    : c <= 11623)
                  : (c <= 11631 || (c < 11680
                    ? (c >= 11647 && c <= 11670)
                    : c <= 11686)))))))))
          : (c <= 11694 || (c < 42512
            ? (c < 12353
              ? (c < 11736
                ? (c < 11712
                  ? (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)
                  : (c <= 11718 || (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)))
                : (c <= 11742 || (c < 12321
                  ? (c < 12293
                    ? (c >= 11744 && c <= 11775)
                    : c <= 12295)
                  : (c <= 12335 || (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)))))
              : (c <= 12438 || (c < 12704
                ? (c < 12540
                  ? (c < 12449
                    ? (c >= 12441 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)))
                : (c <= 12735 || (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 42124 || (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)))))))
            : (c <= 42539 || (c < 43072
              ? (c < 42891
                ? (c < 42623
                  ? (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)
                  : (c <= 42737 || (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)))
                : (c <= 42954 || (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)))))
              : (c <= 43123 || (c < 43312
                ? (c < 43232
                  ? (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)
                  : (c <= 43255 || (c < 43261
                    ? c == 43259
                    : c <= 43309)))
                : (c <= 43347 || (c < 43471
                  ? (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)
                  : (c <= 43481 || (c >= 43488 && c <= 43518)))))))))))))))
    : (c <= 43574 || (c < 71488
      ? (c < 67647
        ? (c < 65482
          ? (c < 64275
            ? (c < 43816
              ? (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))
              : (c <= 43822 || (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))))
            : (c <= 64279 || (c < 65024
              ? (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64848
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64829)
                  : (c <= 64911 || (c < 65008
                    ? (c >= 64914 && c <= 64967)
                    : c <= 65019)))))
              : (c <= 65039 || (c < 65296
                ? (c < 65101
                  ? (c < 65075
                    ? (c >= 65056 && c <= 65071)
                    : c <= 65076)
                  : (c <= 65103 || (c < 65142
                    ? (c >= 65136 && c <= 65140)
                    : c <= 65276)))
                : (c <= 65305 || (c < 65345
                  ? (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)
                  : (c <= 65370 || (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)))))))))
          : (c <= 65487 || (c < 66736
            ? (c < 66176
              ? (c < 65596
                ? (c < 65536
                  ? (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)
                  : (c <= 65547 || (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)))
                : (c <= 65597 || (c < 65664
                  ? (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)
                  : (c <= 65786 || (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)))))
              : (c <= 66204 || (c < 66432
                ? (c < 66304
                  ? (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)
                  : (c <= 66335 || (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)))))))
            : (c <= 66771 || (c < 67072
              ? (c < 66956
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)))
                : (c <= 66962 || (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))))
              : (c <= 67382 || (c < 67584
                ? (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)))
                : (c <= 67589 || (c < 67639
                  ? (c < 67594
                    ? c == 67592
                    : c <= 67637)
                  : (c <= 67640 || c == 67644))))))))))
        : (c <= 67669 || (c < 70006
          ? (c < 68608
            ? (c < 68117
              ? (c < 67872
                ? (c < 67808
                  ? (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)
                  : (c <= 67826 || (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)))
                : (c <= 67897 || (c < 68096
                  ? (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)
                  : (c <= 68099 || (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)))))
              : (c <= 68119 || (c < 68288
                ? (c < 68159
                  ? (c < 68152
                    ? (c >= 68121 && c <= 68149)
                    : c <= 68154)
                  : (c <= 68159 || (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)))
                : (c <= 68295 || (c < 68416
                  ? (c < 68352
                    ? (c >= 68297 && c <= 68326)
                    : c <= 68405)
                  : (c <= 68437 || (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)))))))
            : (c <= 68680 || (c < 69552
              ? (c < 69291
                ? (c < 68864
                  ? (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)
                  : (c <= 68903 || (c < 69248
                    ? (c >= 68912 && c <= 68921)
                    : c <= 69289)))
                : (c <= 69292 || (c < 69415
                  ? (c < 69376
                    ? (c >= 69296 && c <= 69297)
                    : c <= 69404)
                  : (c <= 69415 || (c < 69488
                    ? (c >= 69424 && c <= 69456)
                    : c <= 69509)))))
              : (c <= 69572 || (c < 69840
                ? (c < 69734
                  ? (c < 69632
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69702)
                  : (c <= 69749 || (c < 69826
                    ? (c >= 69759 && c <= 69818)
                    : c <= 69826)))
                : (c <= 69864 || (c < 69942
                  ? (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)
                  : (c <= 69951 || (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)))))))))
          : (c <= 70006 || (c < 70475
            ? (c < 70303
              ? (c < 70163
                ? (c < 70094
                  ? (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)
                  : (c <= 70106 || (c < 70144
                    ? c == 70108
                    : c <= 70161)))
                : (c <= 70199 || (c < 70280
                  ? (c < 70272
                    ? c == 70206
                    : c <= 70278)
                  : (c <= 70280 || (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)))))
              : (c <= 70312 || (c < 70419
                ? (c < 70400
                  ? (c < 70384
                    ? (c >= 70320 && c <= 70378)
                    : c <= 70393)
                  : (c <= 70403 || (c < 70415
                    ? (c >= 70405 && c <= 70412)
                    : c <= 70416)))
                : (c <= 70440 || (c < 70453
                  ? (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)
                  : (c <= 70457 || (c < 70471
                    ? (c >= 70459 && c <= 70468)
                    : c <= 70472)))))))
            : (c <= 70477 || (c < 71040
              ? (c < 70656
                ? (c < 70493
                  ? (c < 70487
                    ? c == 70480
                    : c <= 70487)
                  : (c <= 70499 || (c < 70512
                    ? (c >= 70502 && c <= 70508)
                    : c <= 70516)))
                : (c <= 70730 || (c < 70784
                  ? (c < 70750
                    ? (c >= 70736 && c <= 70745)
                    : c <= 70753)
                  : (c <= 70853 || (c < 70864
                    ? c == 70855
                    : c <= 70873)))))
              : (c <= 71093 || (c < 71296
                ? (c < 71168
                  ? (c < 71128
                    ? (c >= 71096 && c <= 71104)
                    : c <= 71133)
                  : (c <= 71232 || (c < 71248
                    ? c == 71236
                    : c <= 71257)))
                : (c <= 71352 || (c < 71453
                  ? (c < 71424
                    ? (c >= 71360 && c <= 71369)
                    : c <= 71450)
                  : (c <= 71467 || (c >= 71472 && c <= 71481)))))))))))))
      : (c <= 71494 || (c < 119977
        ? (c < 92160
          ? (c < 72818
            ? (c < 72106
              ? (c < 71957
                ? (c < 71935
                  ? (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)
                  : (c <= 71942 || (c < 71948
                    ? c == 71945
                    : c <= 71955)))
                : (c <= 71958 || (c < 71995
                  ? (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)
                  : (c <= 72003 || (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)))))
              : (c <= 72151 || (c < 72349
                ? (c < 72192
                  ? (c < 72163
                    ? (c >= 72154 && c <= 72161)
                    : c <= 72164)
                  : (c <= 72254 || (c < 72272
                    ? c == 72263
                    : c <= 72345)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72758 || (c < 72784
                    ? (c >= 72760 && c <= 72768)
                    : c <= 72793)))))))
            : (c <= 72847 || (c < 73066
              ? (c < 73018
                ? (c < 72960
                  ? (c < 72873
                    ? (c >= 72850 && c <= 72871)
                    : c <= 72886)
                  : (c <= 72966 || (c < 72971
                    ? (c >= 72968 && c <= 72969)
                    : c <= 73014)))
                : (c <= 73018 || (c < 73040
                  ? (c < 73023
                    ? (c >= 73020 && c <= 73021)
                    : c <= 73031)
                  : (c <= 73049 || (c < 73063
                    ? (c >= 73056 && c <= 73061)
                    : c <= 73064)))))
              : (c <= 73102 || (c < 73728
                ? (c < 73120
                  ? (c < 73107
                    ? (c >= 73104 && c <= 73105)
                    : c <= 73112)
                  : (c <= 73129 || (c < 73648
                    ? (c >= 73440 && c <= 73462)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c < 82944
                    ? (c >= 77824 && c <= 78894)
                    : c <= 83526)))))))))
          : (c <= 92728 || (c < 110589
            ? (c < 93760
              ? (c < 92912
                ? (c < 92784
                  ? (c < 92768
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92777)
                  : (c <= 92862 || (c < 92880
                    ? (c >= 92864 && c <= 92873)
                    : c <= 92909)))
                : (c <= 92916 || (c < 93008
                  ? (c < 92992
                    ? (c >= 92928 && c <= 92982)
                    : c <= 92995)
                  : (c <= 93017 || (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)))))
              : (c <= 93823 || (c < 94192
                ? (c < 94095
                  ? (c < 94031
                    ? (c >= 93952 && c <= 94026)
                    : c <= 94087)
                  : (c <= 94111 || (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)))
                : (c <= 94193 || (c < 101632
                  ? (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)
                  : (c <= 101640 || (c < 110581
                    ? (c >= 110576 && c <= 110579)
                    : c <= 110587)))))))
            : (c <= 110590 || (c < 119141
              ? (c < 113776
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))
                : (c <= 113788 || (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 118576
                    ? (c >= 118528 && c <= 118573)
                    : c <= 118598)))))
              : (c <= 119145 || (c < 119808
                ? (c < 119173
                  ? (c < 119163
                    ? (c >= 119149 && c <= 119154)
                    : c <= 119170)
                  : (c <= 119179 || (c < 119362
                    ? (c >= 119210 && c <= 119213)
                    : c <= 119364)))
                : (c <= 119892 || (c < 119970
                  ? (c < 119966
                    ? (c >= 119894 && c <= 119964)
                    : c <= 119967)
                  : (c <= 119970 || (c >= 119973 && c <= 119974)))))))))))
        : (c <= 119980 || (c < 124912
          ? (c < 120772
            ? (c < 120138
              ? (c < 120077
                ? (c < 119997
                  ? (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)
                  : (c <= 120003 || (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)))
                : (c <= 120084 || (c < 120123
                  ? (c < 120094
                    ? (c >= 120086 && c <= 120092)
                    : c <= 120121)
                  : (c <= 120126 || (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)))))
              : (c <= 120144 || (c < 120598
                ? (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c < 120572
                    ? (c >= 120540 && c <= 120570)
                    : c <= 120596)))
                : (c <= 120628 || (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c < 120746
                    ? (c >= 120714 && c <= 120744)
                    : c <= 120770)))))))
            : (c <= 120779 || (c < 122915
              ? (c < 121499
                ? (c < 121403
                  ? (c < 121344
                    ? (c >= 120782 && c <= 120831)
                    : c <= 121398)
                  : (c <= 121452 || (c < 121476
                    ? c == 121461
                    : c <= 121476)))
                : (c <= 121503 || (c < 122880
                  ? (c < 122624
                    ? (c >= 121505 && c <= 121519)
                    : c <= 122654)
                  : (c <= 122886 || (c < 122907
                    ? (c >= 122888 && c <= 122904)
                    : c <= 122913)))))
              : (c <= 122916 || (c < 123536
                ? (c < 123184
                  ? (c < 123136
                    ? (c >= 122918 && c <= 122922)
                    : c <= 123180)
                  : (c <= 123197 || (c < 123214
                    ? (c >= 123200 && c <= 123209)
                    : c <= 123214)))
                : (c <= 123566 || (c < 124904
                  ? (c < 124896
                    ? (c >= 123584 && c <= 123641)
                    : c <= 124902)
                  : (c <= 124907 || (c >= 124909 && c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static inline bool sym_identifier_character_set_6(int32_t c) {
  return (c < 43520
    ? (c < 3749
      ? (c < 2730
        ? (c < 1984
          ? (c < 910
            ? (c < 216
              ? (c < 's'
                ? (c < 'A'
                  ? (c < '0'
                    ? c == '-'
                    : c <= '9')
                  : (c <= 'Z' || (c < 'a'
                    ? c == '_'
                    : c <= 'q')))
                : (c <= 'z' || (c < 183
                  ? (c < 181
                    ? c == 170
                    : c <= 181)
                  : (c <= 183 || (c < 192
                    ? c == 186
                    : c <= 214)))))
              : (c <= 246 || (c < 768
                ? (c < 736
                  ? (c < 710
                    ? (c >= 248 && c <= 705)
                    : c <= 721)
                  : (c <= 740 || (c < 750
                    ? c == 748
                    : c <= 750)))
                : (c <= 884 || (c < 895
                  ? (c < 890
                    ? (c >= 886 && c <= 887)
                    : c <= 893)
                  : (c <= 895 || (c < 908
                    ? (c >= 902 && c <= 906)
                    : c <= 908)))))))
            : (c <= 929 || (c < 1479
              ? (c < 1369
                ? (c < 1155
                  ? (c < 1015
                    ? (c >= 931 && c <= 1013)
                    : c <= 1153)
                  : (c <= 1159 || (c < 1329
                    ? (c >= 1162 && c <= 1327)
                    : c <= 1366)))
                : (c <= 1369 || (c < 1471
                  ? (c < 1425
                    ? (c >= 1376 && c <= 1416)
                    : c <= 1469)
                  : (c <= 1471 || (c < 1476
                    ? (c >= 1473 && c <= 1474)
                    : c <= 1477)))))
              : (c <= 1479 || (c < 1749
                ? (c < 1552
                  ? (c < 1519
                    ? (c >= 1488 && c <= 1514)
                    : c <= 1522)
                  : (c <= 1562 || (c < 1646
                    ? (c >= 1568 && c <= 1641)
                    : c <= 1747)))
                : (c <= 1756 || (c < 1791
                  ? (c < 1770
                    ? (c >= 1759 && c <= 1768)
                    : c <= 1788)
                  : (c <= 1791 || (c < 1869
                    ? (c >= 1808 && c <= 1866)
                    : c <= 1969)))))))))
          : (c <= 2037 || (c < 2534
            ? (c < 2437
              ? (c < 2160
                ? (c < 2048
                  ? (c < 2045
                    ? c == 2042
                    : c <= 2045)
                  : (c <= 2093 || (c < 2144
                    ? (c >= 2112 && c <= 2139)
                    : c <= 2154)))
                : (c <= 2183 || (c < 2275
                  ? (c < 2200
                    ? (c >= 2185 && c <= 2190)
                    : c <= 2273)
                  : (c <= 2403 || (c < 2417
                    ? (c >= 2406 && c <= 2415)
                    : c <= 2435)))))
              : (c <= 2444 || (c < 2492
                ? (c < 2474
                  ? (c < 2451
                    ? (c >= 2447 && c <= 2448)
                    : c <= 2472)
                  : (c <= 2480 || (c < 2486
                    ? c == 2482
                    : c <= 2489)))
                : (c <= 2500 || (c < 2519
                  ? (c < 2507
                    ? (c >= 2503 && c <= 2504)
                    : c <= 2510)
                  : (c <= 2519 || (c < 2527
                    ? (c >= 2524 && c <= 2525)
                    : c <= 2531)))))))
            : (c <= 2545 || (c < 2622
              ? (c < 2579
                ? (c < 2561
                  ? (c < 2558
                    ? c == 2556
                    : c <= 2558)
                  : (c <= 2563 || (c < 2575
                    ? (c >= 2565 && c <= 2570)
                    : c <= 2576)))
                : (c <= 2600 || (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c < 2620
                    ? (c >= 2616 && c <= 2617)
                    : c <= 2620)))))
              : (c <= 2626 || (c < 2662
                ? (c < 2641
                  ? (c < 2635
                    ? (c >= 2631 && c <= 2632)
                    : c <= 2637)
                  : (c <= 2641 || (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)))
                : (c <= 2677 || (c < 2703
                  ? (c < 2693
                    ? (c >= 2689 && c <= 2691)
                    : c <= 2701)
                  : (c <= 2705 || (c >= 2707 && c <= 2728)))))))))))
        : (c <= 2736 || (c < 3157
          ? (c < 2929
            ? (c < 2831
              ? (c < 2768
                ? (c < 2748
                  ? (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)
                  : (c <= 2757 || (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)))
                : (c <= 2768 || (c < 2809
                  ? (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)
                  : (c <= 2815 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))))
              : (c <= 2832 || (c < 2887
                ? (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)))
                : (c <= 2888 || (c < 2908
                  ? (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)
                  : (c <= 2909 || (c < 2918
                    ? (c >= 2911 && c <= 2915)
                    : c <= 2927)))))))
            : (c <= 2929 || (c < 3014
              ? (c < 2972
                ? (c < 2958
                  ? (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)
                  : (c <= 2960 || (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)))
                : (c <= 2972 || (c < 2984
                  ? (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)
                  : (c <= 2986 || (c < 3006
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3010)))))
              : (c <= 3016 || (c < 3086
                ? (c < 3031
                  ? (c < 3024
                    ? (c >= 3018 && c <= 3021)
                    : c <= 3024)
                  : (c <= 3031 || (c < 3072
                    ? (c >= 3046 && c <= 3055)
                    : c <= 3084)))
                : (c <= 3088 || (c < 3132
                  ? (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)
                  : (c <= 3140 || (c < 3146
                    ? (c >= 3142 && c <= 3144)
                    : c <= 3149)))))))))
          : (c <= 3158 || (c < 3412
            ? (c < 3270
              ? (c < 3205
                ? (c < 3168
                  ? (c < 3165
                    ? (c >= 3160 && c <= 3162)
                    : c <= 3165)
                  : (c <= 3171 || (c < 3200
                    ? (c >= 3174 && c <= 3183)
                    : c <= 3203)))
                : (c <= 3212 || (c < 3242
                  ? (c < 3218
                    ? (c >= 3214 && c <= 3216)
                    : c <= 3240)
                  : (c <= 3251 || (c < 3260
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3268)))))
              : (c <= 3272 || (c < 3313
                ? (c < 3293
                  ? (c < 3285
                    ? (c >= 3274 && c <= 3277)
                    : c <= 3286)
                  : (c <= 3294 || (c < 3302
                    ? (c >= 3296 && c <= 3299)
                    : c <= 3311)))
                : (c <= 3314 || (c < 3346
                  ? (c < 3342
                    ? (c >= 3328 && c <= 3340)
                    : c <= 3344)
                  : (c <= 3396 || (c < 3402
                    ? (c >= 3398 && c <= 3400)
                    : c <= 3406)))))))
            : (c <= 3415 || (c < 3542
              ? (c < 3482
                ? (c < 3450
                  ? (c < 3430
                    ? (c >= 3423 && c <= 3427)
                    : c <= 3439)
                  : (c <= 3455 || (c < 3461
                    ? (c >= 3457 && c <= 3459)
                    : c <= 3478)))
                : (c <= 3505 || (c < 3520
                  ? (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)
                  : (c <= 3526 || (c < 3535
                    ? c == 3530
                    : c <= 3540)))))
              : (c <= 3542 || (c < 3664
                ? (c < 3570
                  ? (c < 3558
                    ? (c >= 3544 && c <= 3551)
                    : c <= 3567)
                  : (c <= 3571 || (c < 3648
                    ? (c >= 3585 && c <= 3642)
                    : c <= 3662)))
                : (c <= 3673 || (c < 3718
                  ? (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)
                  : (c <= 3722 || (c >= 3724 && c <= 3747)))))))))))))
      : (c <= 3749 || (c < 7960
        ? (c < 5761
          ? (c < 4304
            ? (c < 3897
              ? (c < 3804
                ? (c < 3782
                  ? (c < 3776
                    ? (c >= 3751 && c <= 3773)
                    : c <= 3780)
                  : (c <= 3782 || (c < 3792
                    ? (c >= 3784 && c <= 3789)
                    : c <= 3801)))
                : (c <= 3807 || (c < 3872
                  ? (c < 3864
                    ? c == 3840
                    : c <= 3865)
                  : (c <= 3881 || (c < 3895
                    ? c == 3893
                    : c <= 3895)))))
              : (c <= 3897 || (c < 4038
                ? (c < 3953
                  ? (c < 3913
                    ? (c >= 3902 && c <= 3911)
                    : c <= 3948)
                  : (c <= 3972 || (c < 3993
                    ? (c >= 3974 && c <= 3991)
                    : c <= 4028)))
                : (c <= 4038 || (c < 4256
                  ? (c < 4176
                    ? (c >= 4096 && c <= 4169)
                    : c <= 4253)
                  : (c <= 4293 || (c < 4301
                    ? c == 4295
                    : c <= 4301)))))))
            : (c <= 4346 || (c < 4802
              ? (c < 4704
                ? (c < 4688
                  ? (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)
                  : (c <= 4694 || (c < 4698
                    ? c == 4696
                    : c <= 4701)))
                : (c <= 4744 || (c < 4786
                  ? (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)
                  : (c <= 4789 || (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)))))
              : (c <= 4805 || (c < 4969
                ? (c < 4882
                  ? (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)
                  : (c <= 4885 || (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)))
                : (c <= 4977 || (c < 5112
                  ? (c < 5024
                    ? (c >= 4992 && c <= 5007)
                    : c <= 5109)
                  : (c <= 5117 || (c < 5743
                    ? (c >= 5121 && c <= 5740)
                    : c <= 5759)))))))))
          : (c <= 5786 || (c < 6576
            ? (c < 6112
              ? (c < 5984
                ? (c < 5888
                  ? (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)
                  : (c <= 5909 || (c < 5952
                    ? (c >= 5919 && c <= 5940)
                    : c <= 5971)))
                : (c <= 5996 || (c < 6016
                  ? (c < 6002
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6003)
                  : (c <= 6099 || (c < 6108
                    ? c == 6103
                    : c <= 6109)))))
              : (c <= 6121 || (c < 6400
                ? (c < 6176
                  ? (c < 6159
                    ? (c >= 6155 && c <= 6157)
                    : c <= 6169)
                  : (c <= 6264 || (c < 6320
                    ? (c >= 6272 && c <= 6314)
                    : c <= 6389)))
                : (c <= 6430 || (c < 6470
                  ? (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)
                  : (c <= 6509 || (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)))))))
            : (c <= 6601 || (c < 7019
              ? (c < 6800
                ? (c < 6688
                  ? (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)
                  : (c <= 6750 || (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)))
                : (c <= 6809 || (c < 6847
                  ? (c < 6832
                    ? c == 6823
                    : c <= 6845)
                  : (c <= 6862 || (c < 6992
                    ? (c >= 6912 && c <= 6988)
                    : c <= 7001)))))
              : (c <= 7027 || (c < 7312
                ? (c < 7232
                  ? (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)
                  : (c <= 7241 || (c < 7296
                    ? (c >= 7245 && c <= 7293)
                    : c <= 7304)))
                : (c <= 7354 || (c < 7380
                  ? (c < 7376
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7378)
                  : (c <= 7418 || (c >= 7424 && c <= 7957)))))))))))
        : (c <= 7965 || (c < 11688
          ? (c < 8417
            ? (c < 8134
              ? (c < 8029
                ? (c < 8016
                  ? (c < 8008
                    ? (c >= 7968 && c <= 8005)
                    : c <= 8013)
                  : (c <= 8023 || (c < 8027
                    ? c == 8025
                    : c <= 8027)))
                : (c <= 8029 || (c < 8118
                  ? (c < 8064
                    ? (c >= 8031 && c <= 8061)
                    : c <= 8116)
                  : (c <= 8124 || (c < 8130
                    ? c == 8126
                    : c <= 8132)))))
              : (c <= 8140 || (c < 8255
                ? (c < 8160
                  ? (c < 8150
                    ? (c >= 8144 && c <= 8147)
                    : c <= 8155)
                  : (c <= 8172 || (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)))
                : (c <= 8256 || (c < 8319
                  ? (c < 8305
                    ? c == 8276
                    : c <= 8305)
                  : (c <= 8319 || (c < 8400
                    ? (c >= 8336 && c <= 8348)
                    : c <= 8412)))))))
            : (c <= 8417 || (c < 8517
              ? (c < 8472
                ? (c < 8455
                  ? (c < 8450
                    ? (c >= 8421 && c <= 8432)
                    : c <= 8450)
                  : (c <= 8455 || (c < 8469
                    ? (c >= 8458 && c <= 8467)
                    : c <= 8469)))
                : (c <= 8477 || (c < 8488
                  ? (c < 8486
                    ? c == 8484
                    : c <= 8486)
                  : (c <= 8488 || (c < 8508
                    ? (c >= 8490 && c <= 8505)
                    : c <= 8511)))))
              : (c <= 8521 || (c < 11559
                ? (c < 11264
                  ? (c < 8544
                    ? c == 8526
                    : c <= 8584)
                  : (c <= 11492 || (c < 11520
                    ? (c >= 11499 && c <= 11507)
                    : c <= 11557)))
                : (c <= 11559 || (c < 11631
                  ? (c < 11568
                    ? c == 11565
                    : c <= 11623)
                  : (c <= 11631 || (c < 11680
                    ? (c >= 11647 && c <= 11670)
                    : c <= 11686)))))))))
          : (c <= 11694 || (c < 42512
            ? (c < 12353
              ? (c < 11736
                ? (c < 11712
                  ? (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)
                  : (c <= 11718 || (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)))
                : (c <= 11742 || (c < 12321
                  ? (c < 12293
                    ? (c >= 11744 && c <= 11775)
                    : c <= 12295)
                  : (c <= 12335 || (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)))))
              : (c <= 12438 || (c < 12704
                ? (c < 12540
                  ? (c < 12449
                    ? (c >= 12441 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)))
                : (c <= 12735 || (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 42124 || (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)))))))
            : (c <= 42539 || (c < 43072
              ? (c < 42891
                ? (c < 42623
                  ? (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)
                  : (c <= 42737 || (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)))
                : (c <= 42954 || (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)))))
              : (c <= 43123 || (c < 43312
                ? (c < 43232
                  ? (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)
                  : (c <= 43255 || (c < 43261
                    ? c == 43259
                    : c <= 43309)))
                : (c <= 43347 || (c < 43471
                  ? (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)
                  : (c <= 43481 || (c >= 43488 && c <= 43518)))))))))))))))
    : (c <= 43574 || (c < 71488
      ? (c < 67647
        ? (c < 65482
          ? (c < 64275
            ? (c < 43816
              ? (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))
              : (c <= 43822 || (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))))
            : (c <= 64279 || (c < 65024
              ? (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64848
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64829)
                  : (c <= 64911 || (c < 65008
                    ? (c >= 64914 && c <= 64967)
                    : c <= 65019)))))
              : (c <= 65039 || (c < 65296
                ? (c < 65101
                  ? (c < 65075
                    ? (c >= 65056 && c <= 65071)
                    : c <= 65076)
                  : (c <= 65103 || (c < 65142
                    ? (c >= 65136 && c <= 65140)
                    : c <= 65276)))
                : (c <= 65305 || (c < 65345
                  ? (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)
                  : (c <= 65370 || (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)))))))))
          : (c <= 65487 || (c < 66736
            ? (c < 66176
              ? (c < 65596
                ? (c < 65536
                  ? (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)
                  : (c <= 65547 || (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)))
                : (c <= 65597 || (c < 65664
                  ? (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)
                  : (c <= 65786 || (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)))))
              : (c <= 66204 || (c < 66432
                ? (c < 66304
                  ? (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)
                  : (c <= 66335 || (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)))))))
            : (c <= 66771 || (c < 67072
              ? (c < 66956
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)))
                : (c <= 66962 || (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))))
              : (c <= 67382 || (c < 67584
                ? (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)))
                : (c <= 67589 || (c < 67639
                  ? (c < 67594
                    ? c == 67592
                    : c <= 67637)
                  : (c <= 67640 || c == 67644))))))))))
        : (c <= 67669 || (c < 70006
          ? (c < 68608
            ? (c < 68117
              ? (c < 67872
                ? (c < 67808
                  ? (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)
                  : (c <= 67826 || (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)))
                : (c <= 67897 || (c < 68096
                  ? (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)
                  : (c <= 68099 || (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)))))
              : (c <= 68119 || (c < 68288
                ? (c < 68159
                  ? (c < 68152
                    ? (c >= 68121 && c <= 68149)
                    : c <= 68154)
                  : (c <= 68159 || (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)))
                : (c <= 68295 || (c < 68416
                  ? (c < 68352
                    ? (c >= 68297 && c <= 68326)
                    : c <= 68405)
                  : (c <= 68437 || (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)))))))
            : (c <= 68680 || (c < 69552
              ? (c < 69291
                ? (c < 68864
                  ? (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)
                  : (c <= 68903 || (c < 69248
                    ? (c >= 68912 && c <= 68921)
                    : c <= 69289)))
                : (c <= 69292 || (c < 69415
                  ? (c < 69376
                    ? (c >= 69296 && c <= 69297)
                    : c <= 69404)
                  : (c <= 69415 || (c < 69488
                    ? (c >= 69424 && c <= 69456)
                    : c <= 69509)))))
              : (c <= 69572 || (c < 69840
                ? (c < 69734
                  ? (c < 69632
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69702)
                  : (c <= 69749 || (c < 69826
                    ? (c >= 69759 && c <= 69818)
                    : c <= 69826)))
                : (c <= 69864 || (c < 69942
                  ? (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)
                  : (c <= 69951 || (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)))))))))
          : (c <= 70006 || (c < 70475
            ? (c < 70303
              ? (c < 70163
                ? (c < 70094
                  ? (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)
                  : (c <= 70106 || (c < 70144
                    ? c == 70108
                    : c <= 70161)))
                : (c <= 70199 || (c < 70280
                  ? (c < 70272
                    ? c == 70206
                    : c <= 70278)
                  : (c <= 70280 || (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)))))
              : (c <= 70312 || (c < 70419
                ? (c < 70400
                  ? (c < 70384
                    ? (c >= 70320 && c <= 70378)
                    : c <= 70393)
                  : (c <= 70403 || (c < 70415
                    ? (c >= 70405 && c <= 70412)
                    : c <= 70416)))
                : (c <= 70440 || (c < 70453
                  ? (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)
                  : (c <= 70457 || (c < 70471
                    ? (c >= 70459 && c <= 70468)
                    : c <= 70472)))))))
            : (c <= 70477 || (c < 71040
              ? (c < 70656
                ? (c < 70493
                  ? (c < 70487
                    ? c == 70480
                    : c <= 70487)
                  : (c <= 70499 || (c < 70512
                    ? (c >= 70502 && c <= 70508)
                    : c <= 70516)))
                : (c <= 70730 || (c < 70784
                  ? (c < 70750
                    ? (c >= 70736 && c <= 70745)
                    : c <= 70753)
                  : (c <= 70853 || (c < 70864
                    ? c == 70855
                    : c <= 70873)))))
              : (c <= 71093 || (c < 71296
                ? (c < 71168
                  ? (c < 71128
                    ? (c >= 71096 && c <= 71104)
                    : c <= 71133)
                  : (c <= 71232 || (c < 71248
                    ? c == 71236
                    : c <= 71257)))
                : (c <= 71352 || (c < 71453
                  ? (c < 71424
                    ? (c >= 71360 && c <= 71369)
                    : c <= 71450)
                  : (c <= 71467 || (c >= 71472 && c <= 71481)))))))))))))
      : (c <= 71494 || (c < 119977
        ? (c < 92160
          ? (c < 72818
            ? (c < 72106
              ? (c < 71957
                ? (c < 71935
                  ? (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)
                  : (c <= 71942 || (c < 71948
                    ? c == 71945
                    : c <= 71955)))
                : (c <= 71958 || (c < 71995
                  ? (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)
                  : (c <= 72003 || (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)))))
              : (c <= 72151 || (c < 72349
                ? (c < 72192
                  ? (c < 72163
                    ? (c >= 72154 && c <= 72161)
                    : c <= 72164)
                  : (c <= 72254 || (c < 72272
                    ? c == 72263
                    : c <= 72345)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72758 || (c < 72784
                    ? (c >= 72760 && c <= 72768)
                    : c <= 72793)))))))
            : (c <= 72847 || (c < 73066
              ? (c < 73018
                ? (c < 72960
                  ? (c < 72873
                    ? (c >= 72850 && c <= 72871)
                    : c <= 72886)
                  : (c <= 72966 || (c < 72971
                    ? (c >= 72968 && c <= 72969)
                    : c <= 73014)))
                : (c <= 73018 || (c < 73040
                  ? (c < 73023
                    ? (c >= 73020 && c <= 73021)
                    : c <= 73031)
                  : (c <= 73049 || (c < 73063
                    ? (c >= 73056 && c <= 73061)
                    : c <= 73064)))))
              : (c <= 73102 || (c < 73728
                ? (c < 73120
                  ? (c < 73107
                    ? (c >= 73104 && c <= 73105)
                    : c <= 73112)
                  : (c <= 73129 || (c < 73648
                    ? (c >= 73440 && c <= 73462)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c < 82944
                    ? (c >= 77824 && c <= 78894)
                    : c <= 83526)))))))))
          : (c <= 92728 || (c < 110589
            ? (c < 93760
              ? (c < 92912
                ? (c < 92784
                  ? (c < 92768
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92777)
                  : (c <= 92862 || (c < 92880
                    ? (c >= 92864 && c <= 92873)
                    : c <= 92909)))
                : (c <= 92916 || (c < 93008
                  ? (c < 92992
                    ? (c >= 92928 && c <= 92982)
                    : c <= 92995)
                  : (c <= 93017 || (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)))))
              : (c <= 93823 || (c < 94192
                ? (c < 94095
                  ? (c < 94031
                    ? (c >= 93952 && c <= 94026)
                    : c <= 94087)
                  : (c <= 94111 || (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)))
                : (c <= 94193 || (c < 101632
                  ? (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)
                  : (c <= 101640 || (c < 110581
                    ? (c >= 110576 && c <= 110579)
                    : c <= 110587)))))))
            : (c <= 110590 || (c < 119141
              ? (c < 113776
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))
                : (c <= 113788 || (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 118576
                    ? (c >= 118528 && c <= 118573)
                    : c <= 118598)))))
              : (c <= 119145 || (c < 119808
                ? (c < 119173
                  ? (c < 119163
                    ? (c >= 119149 && c <= 119154)
                    : c <= 119170)
                  : (c <= 119179 || (c < 119362
                    ? (c >= 119210 && c <= 119213)
                    : c <= 119364)))
                : (c <= 119892 || (c < 119970
                  ? (c < 119966
                    ? (c >= 119894 && c <= 119964)
                    : c <= 119967)
                  : (c <= 119970 || (c >= 119973 && c <= 119974)))))))))))
        : (c <= 119980 || (c < 124912
          ? (c < 120772
            ? (c < 120138
              ? (c < 120077
                ? (c < 119997
                  ? (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)
                  : (c <= 120003 || (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)))
                : (c <= 120084 || (c < 120123
                  ? (c < 120094
                    ? (c >= 120086 && c <= 120092)
                    : c <= 120121)
                  : (c <= 120126 || (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)))))
              : (c <= 120144 || (c < 120598
                ? (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c < 120572
                    ? (c >= 120540 && c <= 120570)
                    : c <= 120596)))
                : (c <= 120628 || (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c < 120746
                    ? (c >= 120714 && c <= 120744)
                    : c <= 120770)))))))
            : (c <= 120779 || (c < 122915
              ? (c < 121499
                ? (c < 121403
                  ? (c < 121344
                    ? (c >= 120782 && c <= 120831)
                    : c <= 121398)
                  : (c <= 121452 || (c < 121476
                    ? c == 121461
                    : c <= 121476)))
                : (c <= 121503 || (c < 122880
                  ? (c < 122624
                    ? (c >= 121505 && c <= 121519)
                    : c <= 122654)
                  : (c <= 122886 || (c < 122907
                    ? (c >= 122888 && c <= 122904)
                    : c <= 122913)))))
              : (c <= 122916 || (c < 123536
                ? (c < 123184
                  ? (c < 123136
                    ? (c >= 122918 && c <= 122922)
                    : c <= 123180)
                  : (c <= 123197 || (c < 123214
                    ? (c >= 123200 && c <= 123209)
                    : c <= 123214)))
                : (c <= 123566 || (c < 124904
                  ? (c < 124896
                    ? (c >= 123584 && c <= 123641)
                    : c <= 124902)
                  : (c <= 124907 || (c >= 124909 && c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static inline bool sym_identifier_character_set_7(int32_t c) {
  return (c < 43520
    ? (c < 3749
      ? (c < 2730
        ? (c < 1984
          ? (c < 910
            ? (c < 216
              ? (c < 't'
                ? (c < 'A'
                  ? (c < '0'
                    ? c == '-'
                    : c <= '9')
                  : (c <= 'Z' || (c < 'a'
                    ? c == '_'
                    : c <= 'r')))
                : (c <= 'z' || (c < 183
                  ? (c < 181
                    ? c == 170
                    : c <= 181)
                  : (c <= 183 || (c < 192
                    ? c == 186
                    : c <= 214)))))
              : (c <= 246 || (c < 768
                ? (c < 736
                  ? (c < 710
                    ? (c >= 248 && c <= 705)
                    : c <= 721)
                  : (c <= 740 || (c < 750
                    ? c == 748
                    : c <= 750)))
                : (c <= 884 || (c < 895
                  ? (c < 890
                    ? (c >= 886 && c <= 887)
                    : c <= 893)
                  : (c <= 895 || (c < 908
                    ? (c >= 902 && c <= 906)
                    : c <= 908)))))))
            : (c <= 929 || (c < 1479
              ? (c < 1369
                ? (c < 1155
                  ? (c < 1015
                    ? (c >= 931 && c <= 1013)
                    : c <= 1153)
                  : (c <= 1159 || (c < 1329
                    ? (c >= 1162 && c <= 1327)
                    : c <= 1366)))
                : (c <= 1369 || (c < 1471
                  ? (c < 1425
                    ? (c >= 1376 && c <= 1416)
                    : c <= 1469)
                  : (c <= 1471 || (c < 1476
                    ? (c >= 1473 && c <= 1474)
                    : c <= 1477)))))
              : (c <= 1479 || (c < 1749
                ? (c < 1552
                  ? (c < 1519
                    ? (c >= 1488 && c <= 1514)
                    : c <= 1522)
                  : (c <= 1562 || (c < 1646
                    ? (c >= 1568 && c <= 1641)
                    : c <= 1747)))
                : (c <= 1756 || (c < 1791
                  ? (c < 1770
                    ? (c >= 1759 && c <= 1768)
                    : c <= 1788)
                  : (c <= 1791 || (c < 1869
                    ? (c >= 1808 && c <= 1866)
                    : c <= 1969)))))))))
          : (c <= 2037 || (c < 2534
            ? (c < 2437
              ? (c < 2160
                ? (c < 2048
                  ? (c < 2045
                    ? c == 2042
                    : c <= 2045)
                  : (c <= 2093 || (c < 2144
                    ? (c >= 2112 && c <= 2139)
                    : c <= 2154)))
                : (c <= 2183 || (c < 2275
                  ? (c < 2200
                    ? (c >= 2185 && c <= 2190)
                    : c <= 2273)
                  : (c <= 2403 || (c < 2417
                    ? (c >= 2406 && c <= 2415)
                    : c <= 2435)))))
              : (c <= 2444 || (c < 2492
                ? (c < 2474
                  ? (c < 2451
                    ? (c >= 2447 && c <= 2448)
                    : c <= 2472)
                  : (c <= 2480 || (c < 2486
                    ? c == 2482
                    : c <= 2489)))
                : (c <= 2500 || (c < 2519
                  ? (c < 2507
                    ? (c >= 2503 && c <= 2504)
                    : c <= 2510)
                  : (c <= 2519 || (c < 2527
                    ? (c >= 2524 && c <= 2525)
                    : c <= 2531)))))))
            : (c <= 2545 || (c < 2622
              ? (c < 2579
                ? (c < 2561
                  ? (c < 2558
                    ? c == 2556
                    : c <= 2558)
                  : (c <= 2563 || (c < 2575
                    ? (c >= 2565 && c <= 2570)
                    : c <= 2576)))
                : (c <= 2600 || (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c < 2620
                    ? (c >= 2616 && c <= 2617)
                    : c <= 2620)))))
              : (c <= 2626 || (c < 2662
                ? (c < 2641
                  ? (c < 2635
                    ? (c >= 2631 && c <= 2632)
                    : c <= 2637)
                  : (c <= 2641 || (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)))
                : (c <= 2677 || (c < 2703
                  ? (c < 2693
                    ? (c >= 2689 && c <= 2691)
                    : c <= 2701)
                  : (c <= 2705 || (c >= 2707 && c <= 2728)))))))))))
        : (c <= 2736 || (c < 3157
          ? (c < 2929
            ? (c < 2831
              ? (c < 2768
                ? (c < 2748
                  ? (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)
                  : (c <= 2757 || (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)))
                : (c <= 2768 || (c < 2809
                  ? (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)
                  : (c <= 2815 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))))
              : (c <= 2832 || (c < 2887
                ? (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)))
                : (c <= 2888 || (c < 2908
                  ? (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)
                  : (c <= 2909 || (c < 2918
                    ? (c >= 2911 && c <= 2915)
                    : c <= 2927)))))))
            : (c <= 2929 || (c < 3014
              ? (c < 2972
                ? (c < 2958
                  ? (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)
                  : (c <= 2960 || (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)))
                : (c <= 2972 || (c < 2984
                  ? (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)
                  : (c <= 2986 || (c < 3006
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3010)))))
              : (c <= 3016 || (c < 3086
                ? (c < 3031
                  ? (c < 3024
                    ? (c >= 3018 && c <= 3021)
                    : c <= 3024)
                  : (c <= 3031 || (c < 3072
                    ? (c >= 3046 && c <= 3055)
                    : c <= 3084)))
                : (c <= 3088 || (c < 3132
                  ? (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)
                  : (c <= 3140 || (c < 3146
                    ? (c >= 3142 && c <= 3144)
                    : c <= 3149)))))))))
          : (c <= 3158 || (c < 3412
            ? (c < 3270
              ? (c < 3205
                ? (c < 3168
                  ? (c < 3165
                    ? (c >= 3160 && c <= 3162)
                    : c <= 3165)
                  : (c <= 3171 || (c < 3200
                    ? (c >= 3174 && c <= 3183)
                    : c <= 3203)))
                : (c <= 3212 || (c < 3242
                  ? (c < 3218
                    ? (c >= 3214 && c <= 3216)
                    : c <= 3240)
                  : (c <= 3251 || (c < 3260
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3268)))))
              : (c <= 3272 || (c < 3313
                ? (c < 3293
                  ? (c < 3285
                    ? (c >= 3274 && c <= 3277)
                    : c <= 3286)
                  : (c <= 3294 || (c < 3302
                    ? (c >= 3296 && c <= 3299)
                    : c <= 3311)))
                : (c <= 3314 || (c < 3346
                  ? (c < 3342
                    ? (c >= 3328 && c <= 3340)
                    : c <= 3344)
                  : (c <= 3396 || (c < 3402
                    ? (c >= 3398 && c <= 3400)
                    : c <= 3406)))))))
            : (c <= 3415 || (c < 3542
              ? (c < 3482
                ? (c < 3450
                  ? (c < 3430
                    ? (c >= 3423 && c <= 3427)
                    : c <= 3439)
                  : (c <= 3455 || (c < 3461
                    ? (c >= 3457 && c <= 3459)
                    : c <= 3478)))
                : (c <= 3505 || (c < 3520
                  ? (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)
                  : (c <= 3526 || (c < 3535
                    ? c == 3530
                    : c <= 3540)))))
              : (c <= 3542 || (c < 3664
                ? (c < 3570
                  ? (c < 3558
                    ? (c >= 3544 && c <= 3551)
                    : c <= 3567)
                  : (c <= 3571 || (c < 3648
                    ? (c >= 3585 && c <= 3642)
                    : c <= 3662)))
                : (c <= 3673 || (c < 3718
                  ? (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)
                  : (c <= 3722 || (c >= 3724 && c <= 3747)))))))))))))
      : (c <= 3749 || (c < 7960
        ? (c < 5761
          ? (c < 4304
            ? (c < 3897
              ? (c < 3804
                ? (c < 3782
                  ? (c < 3776
                    ? (c >= 3751 && c <= 3773)
                    : c <= 3780)
                  : (c <= 3782 || (c < 3792
                    ? (c >= 3784 && c <= 3789)
                    : c <= 3801)))
                : (c <= 3807 || (c < 3872
                  ? (c < 3864
                    ? c == 3840
                    : c <= 3865)
                  : (c <= 3881 || (c < 3895
                    ? c == 3893
                    : c <= 3895)))))
              : (c <= 3897 || (c < 4038
                ? (c < 3953
                  ? (c < 3913
                    ? (c >= 3902 && c <= 3911)
                    : c <= 3948)
                  : (c <= 3972 || (c < 3993
                    ? (c >= 3974 && c <= 3991)
                    : c <= 4028)))
                : (c <= 4038 || (c < 4256
                  ? (c < 4176
                    ? (c >= 4096 && c <= 4169)
                    : c <= 4253)
                  : (c <= 4293 || (c < 4301
                    ? c == 4295
                    : c <= 4301)))))))
            : (c <= 4346 || (c < 4802
              ? (c < 4704
                ? (c < 4688
                  ? (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)
                  : (c <= 4694 || (c < 4698
                    ? c == 4696
                    : c <= 4701)))
                : (c <= 4744 || (c < 4786
                  ? (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)
                  : (c <= 4789 || (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)))))
              : (c <= 4805 || (c < 4969
                ? (c < 4882
                  ? (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)
                  : (c <= 4885 || (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)))
                : (c <= 4977 || (c < 5112
                  ? (c < 5024
                    ? (c >= 4992 && c <= 5007)
                    : c <= 5109)
                  : (c <= 5117 || (c < 5743
                    ? (c >= 5121 && c <= 5740)
                    : c <= 5759)))))))))
          : (c <= 5786 || (c < 6576
            ? (c < 6112
              ? (c < 5984
                ? (c < 5888
                  ? (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)
                  : (c <= 5909 || (c < 5952
                    ? (c >= 5919 && c <= 5940)
                    : c <= 5971)))
                : (c <= 5996 || (c < 6016
                  ? (c < 6002
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6003)
                  : (c <= 6099 || (c < 6108
                    ? c == 6103
                    : c <= 6109)))))
              : (c <= 6121 || (c < 6400
                ? (c < 6176
                  ? (c < 6159
                    ? (c >= 6155 && c <= 6157)
                    : c <= 6169)
                  : (c <= 6264 || (c < 6320
                    ? (c >= 6272 && c <= 6314)
                    : c <= 6389)))
                : (c <= 6430 || (c < 6470
                  ? (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)
                  : (c <= 6509 || (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)))))))
            : (c <= 6601 || (c < 7019
              ? (c < 6800
                ? (c < 6688
                  ? (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)
                  : (c <= 6750 || (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)))
                : (c <= 6809 || (c < 6847
                  ? (c < 6832
                    ? c == 6823
                    : c <= 6845)
                  : (c <= 6862 || (c < 6992
                    ? (c >= 6912 && c <= 6988)
                    : c <= 7001)))))
              : (c <= 7027 || (c < 7312
                ? (c < 7232
                  ? (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)
                  : (c <= 7241 || (c < 7296
                    ? (c >= 7245 && c <= 7293)
                    : c <= 7304)))
                : (c <= 7354 || (c < 7380
                  ? (c < 7376
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7378)
                  : (c <= 7418 || (c >= 7424 && c <= 7957)))))))))))
        : (c <= 7965 || (c < 11688
          ? (c < 8417
            ? (c < 8134
              ? (c < 8029
                ? (c < 8016
                  ? (c < 8008
                    ? (c >= 7968 && c <= 8005)
                    : c <= 8013)
                  : (c <= 8023 || (c < 8027
                    ? c == 8025
                    : c <= 8027)))
                : (c <= 8029 || (c < 8118
                  ? (c < 8064
                    ? (c >= 8031 && c <= 8061)
                    : c <= 8116)
                  : (c <= 8124 || (c < 8130
                    ? c == 8126
                    : c <= 8132)))))
              : (c <= 8140 || (c < 8255
                ? (c < 8160
                  ? (c < 8150
                    ? (c >= 8144 && c <= 8147)
                    : c <= 8155)
                  : (c <= 8172 || (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)))
                : (c <= 8256 || (c < 8319
                  ? (c < 8305
                    ? c == 8276
                    : c <= 8305)
                  : (c <= 8319 || (c < 8400
                    ? (c >= 8336 && c <= 8348)
                    : c <= 8412)))))))
            : (c <= 8417 || (c < 8517
              ? (c < 8472
                ? (c < 8455
                  ? (c < 8450
                    ? (c >= 8421 && c <= 8432)
                    : c <= 8450)
                  : (c <= 8455 || (c < 8469
                    ? (c >= 8458 && c <= 8467)
                    : c <= 8469)))
                : (c <= 8477 || (c < 8488
                  ? (c < 8486
                    ? c == 8484
                    : c <= 8486)
                  : (c <= 8488 || (c < 8508
                    ? (c >= 8490 && c <= 8505)
                    : c <= 8511)))))
              : (c <= 8521 || (c < 11559
                ? (c < 11264
                  ? (c < 8544
                    ? c == 8526
                    : c <= 8584)
                  : (c <= 11492 || (c < 11520
                    ? (c >= 11499 && c <= 11507)
                    : c <= 11557)))
                : (c <= 11559 || (c < 11631
                  ? (c < 11568
                    ? c == 11565
                    : c <= 11623)
                  : (c <= 11631 || (c < 11680
                    ? (c >= 11647 && c <= 11670)
                    : c <= 11686)))))))))
          : (c <= 11694 || (c < 42512
            ? (c < 12353
              ? (c < 11736
                ? (c < 11712
                  ? (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)
                  : (c <= 11718 || (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)))
                : (c <= 11742 || (c < 12321
                  ? (c < 12293
                    ? (c >= 11744 && c <= 11775)
                    : c <= 12295)
                  : (c <= 12335 || (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)))))
              : (c <= 12438 || (c < 12704
                ? (c < 12540
                  ? (c < 12449
                    ? (c >= 12441 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)))
                : (c <= 12735 || (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 42124 || (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)))))))
            : (c <= 42539 || (c < 43072
              ? (c < 42891
                ? (c < 42623
                  ? (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)
                  : (c <= 42737 || (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)))
                : (c <= 42954 || (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)))))
              : (c <= 43123 || (c < 43312
                ? (c < 43232
                  ? (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)
                  : (c <= 43255 || (c < 43261
                    ? c == 43259
                    : c <= 43309)))
                : (c <= 43347 || (c < 43471
                  ? (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)
                  : (c <= 43481 || (c >= 43488 && c <= 43518)))))))))))))))
    : (c <= 43574 || (c < 71488
      ? (c < 67647
        ? (c < 65482
          ? (c < 64275
            ? (c < 43816
              ? (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))
              : (c <= 43822 || (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))))
            : (c <= 64279 || (c < 65024
              ? (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64848
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64829)
                  : (c <= 64911 || (c < 65008
                    ? (c >= 64914 && c <= 64967)
                    : c <= 65019)))))
              : (c <= 65039 || (c < 65296
                ? (c < 65101
                  ? (c < 65075
                    ? (c >= 65056 && c <= 65071)
                    : c <= 65076)
                  : (c <= 65103 || (c < 65142
                    ? (c >= 65136 && c <= 65140)
                    : c <= 65276)))
                : (c <= 65305 || (c < 65345
                  ? (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)
                  : (c <= 65370 || (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)))))))))
          : (c <= 65487 || (c < 66736
            ? (c < 66176
              ? (c < 65596
                ? (c < 65536
                  ? (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)
                  : (c <= 65547 || (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)))
                : (c <= 65597 || (c < 65664
                  ? (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)
                  : (c <= 65786 || (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)))))
              : (c <= 66204 || (c < 66432
                ? (c < 66304
                  ? (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)
                  : (c <= 66335 || (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)))))))
            : (c <= 66771 || (c < 67072
              ? (c < 66956
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)))
                : (c <= 66962 || (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))))
              : (c <= 67382 || (c < 67584
                ? (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)))
                : (c <= 67589 || (c < 67639
                  ? (c < 67594
                    ? c == 67592
                    : c <= 67637)
                  : (c <= 67640 || c == 67644))))))))))
        : (c <= 67669 || (c < 70006
          ? (c < 68608
            ? (c < 68117
              ? (c < 67872
                ? (c < 67808
                  ? (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)
                  : (c <= 67826 || (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)))
                : (c <= 67897 || (c < 68096
                  ? (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)
                  : (c <= 68099 || (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)))))
              : (c <= 68119 || (c < 68288
                ? (c < 68159
                  ? (c < 68152
                    ? (c >= 68121 && c <= 68149)
                    : c <= 68154)
                  : (c <= 68159 || (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)))
                : (c <= 68295 || (c < 68416
                  ? (c < 68352
                    ? (c >= 68297 && c <= 68326)
                    : c <= 68405)
                  : (c <= 68437 || (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)))))))
            : (c <= 68680 || (c < 69552
              ? (c < 69291
                ? (c < 68864
                  ? (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)
                  : (c <= 68903 || (c < 69248
                    ? (c >= 68912 && c <= 68921)
                    : c <= 69289)))
                : (c <= 69292 || (c < 69415
                  ? (c < 69376
                    ? (c >= 69296 && c <= 69297)
                    : c <= 69404)
                  : (c <= 69415 || (c < 69488
                    ? (c >= 69424 && c <= 69456)
                    : c <= 69509)))))
              : (c <= 69572 || (c < 69840
                ? (c < 69734
                  ? (c < 69632
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69702)
                  : (c <= 69749 || (c < 69826
                    ? (c >= 69759 && c <= 69818)
                    : c <= 69826)))
                : (c <= 69864 || (c < 69942
                  ? (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)
                  : (c <= 69951 || (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)))))))))
          : (c <= 70006 || (c < 70475
            ? (c < 70303
              ? (c < 70163
                ? (c < 70094
                  ? (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)
                  : (c <= 70106 || (c < 70144
                    ? c == 70108
                    : c <= 70161)))
                : (c <= 70199 || (c < 70280
                  ? (c < 70272
                    ? c == 70206
                    : c <= 70278)
                  : (c <= 70280 || (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)))))
              : (c <= 70312 || (c < 70419
                ? (c < 70400
                  ? (c < 70384
                    ? (c >= 70320 && c <= 70378)
                    : c <= 70393)
                  : (c <= 70403 || (c < 70415
                    ? (c >= 70405 && c <= 70412)
                    : c <= 70416)))
                : (c <= 70440 || (c < 70453
                  ? (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)
                  : (c <= 70457 || (c < 70471
                    ? (c >= 70459 && c <= 70468)
                    : c <= 70472)))))))
            : (c <= 70477 || (c < 71040
              ? (c < 70656
                ? (c < 70493
                  ? (c < 70487
                    ? c == 70480
                    : c <= 70487)
                  : (c <= 70499 || (c < 70512
                    ? (c >= 70502 && c <= 70508)
                    : c <= 70516)))
                : (c <= 70730 || (c < 70784
                  ? (c < 70750
                    ? (c >= 70736 && c <= 70745)
                    : c <= 70753)
                  : (c <= 70853 || (c < 70864
                    ? c == 70855
                    : c <= 70873)))))
              : (c <= 71093 || (c < 71296
                ? (c < 71168
                  ? (c < 71128
                    ? (c >= 71096 && c <= 71104)
                    : c <= 71133)
                  : (c <= 71232 || (c < 71248
                    ? c == 71236
                    : c <= 71257)))
                : (c <= 71352 || (c < 71453
                  ? (c < 71424
                    ? (c >= 71360 && c <= 71369)
                    : c <= 71450)
                  : (c <= 71467 || (c >= 71472 && c <= 71481)))))))))))))
      : (c <= 71494 || (c < 119977
        ? (c < 92160
          ? (c < 72818
            ? (c < 72106
              ? (c < 71957
                ? (c < 71935
                  ? (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)
                  : (c <= 71942 || (c < 71948
                    ? c == 71945
                    : c <= 71955)))
                : (c <= 71958 || (c < 71995
                  ? (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)
                  : (c <= 72003 || (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)))))
              : (c <= 72151 || (c < 72349
                ? (c < 72192
                  ? (c < 72163
                    ? (c >= 72154 && c <= 72161)
                    : c <= 72164)
                  : (c <= 72254 || (c < 72272
                    ? c == 72263
                    : c <= 72345)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72758 || (c < 72784
                    ? (c >= 72760 && c <= 72768)
                    : c <= 72793)))))))
            : (c <= 72847 || (c < 73066
              ? (c < 73018
                ? (c < 72960
                  ? (c < 72873
                    ? (c >= 72850 && c <= 72871)
                    : c <= 72886)
                  : (c <= 72966 || (c < 72971
                    ? (c >= 72968 && c <= 72969)
                    : c <= 73014)))
                : (c <= 73018 || (c < 73040
                  ? (c < 73023
                    ? (c >= 73020 && c <= 73021)
                    : c <= 73031)
                  : (c <= 73049 || (c < 73063
                    ? (c >= 73056 && c <= 73061)
                    : c <= 73064)))))
              : (c <= 73102 || (c < 73728
                ? (c < 73120
                  ? (c < 73107
                    ? (c >= 73104 && c <= 73105)
                    : c <= 73112)
                  : (c <= 73129 || (c < 73648
                    ? (c >= 73440 && c <= 73462)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c < 82944
                    ? (c >= 77824 && c <= 78894)
                    : c <= 83526)))))))))
          : (c <= 92728 || (c < 110589
            ? (c < 93760
              ? (c < 92912
                ? (c < 92784
                  ? (c < 92768
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92777)
                  : (c <= 92862 || (c < 92880
                    ? (c >= 92864 && c <= 92873)
                    : c <= 92909)))
                : (c <= 92916 || (c < 93008
                  ? (c < 92992
                    ? (c >= 92928 && c <= 92982)
                    : c <= 92995)
                  : (c <= 93017 || (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)))))
              : (c <= 93823 || (c < 94192
                ? (c < 94095
                  ? (c < 94031
                    ? (c >= 93952 && c <= 94026)
                    : c <= 94087)
                  : (c <= 94111 || (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)))
                : (c <= 94193 || (c < 101632
                  ? (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)
                  : (c <= 101640 || (c < 110581
                    ? (c >= 110576 && c <= 110579)
                    : c <= 110587)))))))
            : (c <= 110590 || (c < 119141
              ? (c < 113776
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))
                : (c <= 113788 || (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 118576
                    ? (c >= 118528 && c <= 118573)
                    : c <= 118598)))))
              : (c <= 119145 || (c < 119808
                ? (c < 119173
                  ? (c < 119163
                    ? (c >= 119149 && c <= 119154)
                    : c <= 119170)
                  : (c <= 119179 || (c < 119362
                    ? (c >= 119210 && c <= 119213)
                    : c <= 119364)))
                : (c <= 119892 || (c < 119970
                  ? (c < 119966
                    ? (c >= 119894 && c <= 119964)
                    : c <= 119967)
                  : (c <= 119970 || (c >= 119973 && c <= 119974)))))))))))
        : (c <= 119980 || (c < 124912
          ? (c < 120772
            ? (c < 120138
              ? (c < 120077
                ? (c < 119997
                  ? (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)
                  : (c <= 120003 || (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)))
                : (c <= 120084 || (c < 120123
                  ? (c < 120094
                    ? (c >= 120086 && c <= 120092)
                    : c <= 120121)
                  : (c <= 120126 || (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)))))
              : (c <= 120144 || (c < 120598
                ? (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c < 120572
                    ? (c >= 120540 && c <= 120570)
                    : c <= 120596)))
                : (c <= 120628 || (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c < 120746
                    ? (c >= 120714 && c <= 120744)
                    : c <= 120770)))))))
            : (c <= 120779 || (c < 122915
              ? (c < 121499
                ? (c < 121403
                  ? (c < 121344
                    ? (c >= 120782 && c <= 120831)
                    : c <= 121398)
                  : (c <= 121452 || (c < 121476
                    ? c == 121461
                    : c <= 121476)))
                : (c <= 121503 || (c < 122880
                  ? (c < 122624
                    ? (c >= 121505 && c <= 121519)
                    : c <= 122654)
                  : (c <= 122886 || (c < 122907
                    ? (c >= 122888 && c <= 122904)
                    : c <= 122913)))))
              : (c <= 122916 || (c < 123536
                ? (c < 123184
                  ? (c < 123136
                    ? (c >= 122918 && c <= 122922)
                    : c <= 123180)
                  : (c <= 123197 || (c < 123214
                    ? (c >= 123200 && c <= 123209)
                    : c <= 123214)))
                : (c <= 123566 || (c < 124904
                  ? (c < 124896
                    ? (c >= 123584 && c <= 123641)
                    : c <= 124902)
                  : (c <= 124907 || (c >= 124909 && c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static inline bool sym_identifier_character_set_8(int32_t c) {
  return (c < 43520
    ? (c < 3749
      ? (c < 2730
        ? (c < 1984
          ? (c < 910
            ? (c < 216
              ? (c < 'v'
                ? (c < 'A'
                  ? (c < '0'
                    ? c == '-'
                    : c <= '9')
                  : (c <= 'Z' || (c < 'a'
                    ? c == '_'
                    : c <= 't')))
                : (c <= 'z' || (c < 183
                  ? (c < 181
                    ? c == 170
                    : c <= 181)
                  : (c <= 183 || (c < 192
                    ? c == 186
                    : c <= 214)))))
              : (c <= 246 || (c < 768
                ? (c < 736
                  ? (c < 710
                    ? (c >= 248 && c <= 705)
                    : c <= 721)
                  : (c <= 740 || (c < 750
                    ? c == 748
                    : c <= 750)))
                : (c <= 884 || (c < 895
                  ? (c < 890
                    ? (c >= 886 && c <= 887)
                    : c <= 893)
                  : (c <= 895 || (c < 908
                    ? (c >= 902 && c <= 906)
                    : c <= 908)))))))
            : (c <= 929 || (c < 1479
              ? (c < 1369
                ? (c < 1155
                  ? (c < 1015
                    ? (c >= 931 && c <= 1013)
                    : c <= 1153)
                  : (c <= 1159 || (c < 1329
                    ? (c >= 1162 && c <= 1327)
                    : c <= 1366)))
                : (c <= 1369 || (c < 1471
                  ? (c < 1425
                    ? (c >= 1376 && c <= 1416)
                    : c <= 1469)
                  : (c <= 1471 || (c < 1476
                    ? (c >= 1473 && c <= 1474)
                    : c <= 1477)))))
              : (c <= 1479 || (c < 1749
                ? (c < 1552
                  ? (c < 1519
                    ? (c >= 1488 && c <= 1514)
                    : c <= 1522)
                  : (c <= 1562 || (c < 1646
                    ? (c >= 1568 && c <= 1641)
                    : c <= 1747)))
                : (c <= 1756 || (c < 1791
                  ? (c < 1770
                    ? (c >= 1759 && c <= 1768)
                    : c <= 1788)
                  : (c <= 1791 || (c < 1869
                    ? (c >= 1808 && c <= 1866)
                    : c <= 1969)))))))))
          : (c <= 2037 || (c < 2534
            ? (c < 2437
              ? (c < 2160
                ? (c < 2048
                  ? (c < 2045
                    ? c == 2042
                    : c <= 2045)
                  : (c <= 2093 || (c < 2144
                    ? (c >= 2112 && c <= 2139)
                    : c <= 2154)))
                : (c <= 2183 || (c < 2275
                  ? (c < 2200
                    ? (c >= 2185 && c <= 2190)
                    : c <= 2273)
                  : (c <= 2403 || (c < 2417
                    ? (c >= 2406 && c <= 2415)
                    : c <= 2435)))))
              : (c <= 2444 || (c < 2492
                ? (c < 2474
                  ? (c < 2451
                    ? (c >= 2447 && c <= 2448)
                    : c <= 2472)
                  : (c <= 2480 || (c < 2486
                    ? c == 2482
                    : c <= 2489)))
                : (c <= 2500 || (c < 2519
                  ? (c < 2507
                    ? (c >= 2503 && c <= 2504)
                    : c <= 2510)
                  : (c <= 2519 || (c < 2527
                    ? (c >= 2524 && c <= 2525)
                    : c <= 2531)))))))
            : (c <= 2545 || (c < 2622
              ? (c < 2579
                ? (c < 2561
                  ? (c < 2558
                    ? c == 2556
                    : c <= 2558)
                  : (c <= 2563 || (c < 2575
                    ? (c >= 2565 && c <= 2570)
                    : c <= 2576)))
                : (c <= 2600 || (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c < 2620
                    ? (c >= 2616 && c <= 2617)
                    : c <= 2620)))))
              : (c <= 2626 || (c < 2662
                ? (c < 2641
                  ? (c < 2635
                    ? (c >= 2631 && c <= 2632)
                    : c <= 2637)
                  : (c <= 2641 || (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)))
                : (c <= 2677 || (c < 2703
                  ? (c < 2693
                    ? (c >= 2689 && c <= 2691)
                    : c <= 2701)
                  : (c <= 2705 || (c >= 2707 && c <= 2728)))))))))))
        : (c <= 2736 || (c < 3157
          ? (c < 2929
            ? (c < 2831
              ? (c < 2768
                ? (c < 2748
                  ? (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)
                  : (c <= 2757 || (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)))
                : (c <= 2768 || (c < 2809
                  ? (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)
                  : (c <= 2815 || (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)))))
              : (c <= 2832 || (c < 2887
                ? (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)))
                : (c <= 2888 || (c < 2908
                  ? (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)
                  : (c <= 2909 || (c < 2918
                    ? (c >= 2911 && c <= 2915)
                    : c <= 2927)))))))
            : (c <= 2929 || (c < 3014
              ? (c < 2972
                ? (c < 2958
                  ? (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)
                  : (c <= 2960 || (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)))
                : (c <= 2972 || (c < 2984
                  ? (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)
                  : (c <= 2986 || (c < 3006
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3010)))))
              : (c <= 3016 || (c < 3086
                ? (c < 3031
                  ? (c < 3024
                    ? (c >= 3018 && c <= 3021)
                    : c <= 3024)
                  : (c <= 3031 || (c < 3072
                    ? (c >= 3046 && c <= 3055)
                    : c <= 3084)))
                : (c <= 3088 || (c < 3132
                  ? (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)
                  : (c <= 3140 || (c < 3146
                    ? (c >= 3142 && c <= 3144)
                    : c <= 3149)))))))))
          : (c <= 3158 || (c < 3412
            ? (c < 3270
              ? (c < 3205
                ? (c < 3168
                  ? (c < 3165
                    ? (c >= 3160 && c <= 3162)
                    : c <= 3165)
                  : (c <= 3171 || (c < 3200
                    ? (c >= 3174 && c <= 3183)
                    : c <= 3203)))
                : (c <= 3212 || (c < 3242
                  ? (c < 3218
                    ? (c >= 3214 && c <= 3216)
                    : c <= 3240)
                  : (c <= 3251 || (c < 3260
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3268)))))
              : (c <= 3272 || (c < 3313
                ? (c < 3293
                  ? (c < 3285
                    ? (c >= 3274 && c <= 3277)
                    : c <= 3286)
                  : (c <= 3294 || (c < 3302
                    ? (c >= 3296 && c <= 3299)
                    : c <= 3311)))
                : (c <= 3314 || (c < 3346
                  ? (c < 3342
                    ? (c >= 3328 && c <= 3340)
                    : c <= 3344)
                  : (c <= 3396 || (c < 3402
                    ? (c >= 3398 && c <= 3400)
                    : c <= 3406)))))))
            : (c <= 3415 || (c < 3542
              ? (c < 3482
                ? (c < 3450
                  ? (c < 3430
                    ? (c >= 3423 && c <= 3427)
                    : c <= 3439)
                  : (c <= 3455 || (c < 3461
                    ? (c >= 3457 && c <= 3459)
                    : c <= 3478)))
                : (c <= 3505 || (c < 3520
                  ? (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)
                  : (c <= 3526 || (c < 3535
                    ? c == 3530
                    : c <= 3540)))))
              : (c <= 3542 || (c < 3664
                ? (c < 3570
                  ? (c < 3558
                    ? (c >= 3544 && c <= 3551)
                    : c <= 3567)
                  : (c <= 3571 || (c < 3648
                    ? (c >= 3585 && c <= 3642)
                    : c <= 3662)))
                : (c <= 3673 || (c < 3718
                  ? (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)
                  : (c <= 3722 || (c >= 3724 && c <= 3747)))))))))))))
      : (c <= 3749 || (c < 7960
        ? (c < 5761
          ? (c < 4304
            ? (c < 3897
              ? (c < 3804
                ? (c < 3782
                  ? (c < 3776
                    ? (c >= 3751 && c <= 3773)
                    : c <= 3780)
                  : (c <= 3782 || (c < 3792
                    ? (c >= 3784 && c <= 3789)
                    : c <= 3801)))
                : (c <= 3807 || (c < 3872
                  ? (c < 3864
                    ? c == 3840
                    : c <= 3865)
                  : (c <= 3881 || (c < 3895
                    ? c == 3893
                    : c <= 3895)))))
              : (c <= 3897 || (c < 4038
                ? (c < 3953
                  ? (c < 3913
                    ? (c >= 3902 && c <= 3911)
                    : c <= 3948)
                  : (c <= 3972 || (c < 3993
                    ? (c >= 3974 && c <= 3991)
                    : c <= 4028)))
                : (c <= 4038 || (c < 4256
                  ? (c < 4176
                    ? (c >= 4096 && c <= 4169)
                    : c <= 4253)
                  : (c <= 4293 || (c < 4301
                    ? c == 4295
                    : c <= 4301)))))))
            : (c <= 4346 || (c < 4802
              ? (c < 4704
                ? (c < 4688
                  ? (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)
                  : (c <= 4694 || (c < 4698
                    ? c == 4696
                    : c <= 4701)))
                : (c <= 4744 || (c < 4786
                  ? (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)
                  : (c <= 4789 || (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)))))
              : (c <= 4805 || (c < 4969
                ? (c < 4882
                  ? (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)
                  : (c <= 4885 || (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)))
                : (c <= 4977 || (c < 5112
                  ? (c < 5024
                    ? (c >= 4992 && c <= 5007)
                    : c <= 5109)
                  : (c <= 5117 || (c < 5743
                    ? (c >= 5121 && c <= 5740)
                    : c <= 5759)))))))))
          : (c <= 5786 || (c < 6576
            ? (c < 6112
              ? (c < 5984
                ? (c < 5888
                  ? (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)
                  : (c <= 5909 || (c < 5952
                    ? (c >= 5919 && c <= 5940)
                    : c <= 5971)))
                : (c <= 5996 || (c < 6016
                  ? (c < 6002
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6003)
                  : (c <= 6099 || (c < 6108
                    ? c == 6103
                    : c <= 6109)))))
              : (c <= 6121 || (c < 6400
                ? (c < 6176
                  ? (c < 6159
                    ? (c >= 6155 && c <= 6157)
                    : c <= 6169)
                  : (c <= 6264 || (c < 6320
                    ? (c >= 6272 && c <= 6314)
                    : c <= 6389)))
                : (c <= 6430 || (c < 6470
                  ? (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)
                  : (c <= 6509 || (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)))))))
            : (c <= 6601 || (c < 7019
              ? (c < 6800
                ? (c < 6688
                  ? (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)
                  : (c <= 6750 || (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)))
                : (c <= 6809 || (c < 6847
                  ? (c < 6832
                    ? c == 6823
                    : c <= 6845)
                  : (c <= 6862 || (c < 6992
                    ? (c >= 6912 && c <= 6988)
                    : c <= 7001)))))
              : (c <= 7027 || (c < 7312
                ? (c < 7232
                  ? (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)
                  : (c <= 7241 || (c < 7296
                    ? (c >= 7245 && c <= 7293)
                    : c <= 7304)))
                : (c <= 7354 || (c < 7380
                  ? (c < 7376
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7378)
                  : (c <= 7418 || (c >= 7424 && c <= 7957)))))))))))
        : (c <= 7965 || (c < 11688
          ? (c < 8417
            ? (c < 8134
              ? (c < 8029
                ? (c < 8016
                  ? (c < 8008
                    ? (c >= 7968 && c <= 8005)
                    : c <= 8013)
                  : (c <= 8023 || (c < 8027
                    ? c == 8025
                    : c <= 8027)))
                : (c <= 8029 || (c < 8118
                  ? (c < 8064
                    ? (c >= 8031 && c <= 8061)
                    : c <= 8116)
                  : (c <= 8124 || (c < 8130
                    ? c == 8126
                    : c <= 8132)))))
              : (c <= 8140 || (c < 8255
                ? (c < 8160
                  ? (c < 8150
                    ? (c >= 8144 && c <= 8147)
                    : c <= 8155)
                  : (c <= 8172 || (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)))
                : (c <= 8256 || (c < 8319
                  ? (c < 8305
                    ? c == 8276
                    : c <= 8305)
                  : (c <= 8319 || (c < 8400
                    ? (c >= 8336 && c <= 8348)
                    : c <= 8412)))))))
            : (c <= 8417 || (c < 8517
              ? (c < 8472
                ? (c < 8455
                  ? (c < 8450
                    ? (c >= 8421 && c <= 8432)
                    : c <= 8450)
                  : (c <= 8455 || (c < 8469
                    ? (c >= 8458 && c <= 8467)
                    : c <= 8469)))
                : (c <= 8477 || (c < 8488
                  ? (c < 8486
                    ? c == 8484
                    : c <= 8486)
                  : (c <= 8488 || (c < 8508
                    ? (c >= 8490 && c <= 8505)
                    : c <= 8511)))))
              : (c <= 8521 || (c < 11559
                ? (c < 11264
                  ? (c < 8544
                    ? c == 8526
                    : c <= 8584)
                  : (c <= 11492 || (c < 11520
                    ? (c >= 11499 && c <= 11507)
                    : c <= 11557)))
                : (c <= 11559 || (c < 11631
                  ? (c < 11568
                    ? c == 11565
                    : c <= 11623)
                  : (c <= 11631 || (c < 11680
                    ? (c >= 11647 && c <= 11670)
                    : c <= 11686)))))))))
          : (c <= 11694 || (c < 42512
            ? (c < 12353
              ? (c < 11736
                ? (c < 11712
                  ? (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)
                  : (c <= 11718 || (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)))
                : (c <= 11742 || (c < 12321
                  ? (c < 12293
                    ? (c >= 11744 && c <= 11775)
                    : c <= 12295)
                  : (c <= 12335 || (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)))))
              : (c <= 12438 || (c < 12704
                ? (c < 12540
                  ? (c < 12449
                    ? (c >= 12441 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)))
                : (c <= 12735 || (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 42124 || (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)))))))
            : (c <= 42539 || (c < 43072
              ? (c < 42891
                ? (c < 42623
                  ? (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)
                  : (c <= 42737 || (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)))
                : (c <= 42954 || (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)))))
              : (c <= 43123 || (c < 43312
                ? (c < 43232
                  ? (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)
                  : (c <= 43255 || (c < 43261
                    ? c == 43259
                    : c <= 43309)))
                : (c <= 43347 || (c < 43471
                  ? (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)
                  : (c <= 43481 || (c >= 43488 && c <= 43518)))))))))))))))
    : (c <= 43574 || (c < 71488
      ? (c < 67647
        ? (c < 65482
          ? (c < 64275
            ? (c < 43816
              ? (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))
              : (c <= 43822 || (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))))
            : (c <= 64279 || (c < 65024
              ? (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64848
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64829)
                  : (c <= 64911 || (c < 65008
                    ? (c >= 64914 && c <= 64967)
                    : c <= 65019)))))
              : (c <= 65039 || (c < 65296
                ? (c < 65101
                  ? (c < 65075
                    ? (c >= 65056 && c <= 65071)
                    : c <= 65076)
                  : (c <= 65103 || (c < 65142
                    ? (c >= 65136 && c <= 65140)
                    : c <= 65276)))
                : (c <= 65305 || (c < 65345
                  ? (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)
                  : (c <= 65370 || (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)))))))))
          : (c <= 65487 || (c < 66736
            ? (c < 66176
              ? (c < 65596
                ? (c < 65536
                  ? (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)
                  : (c <= 65547 || (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)))
                : (c <= 65597 || (c < 65664
                  ? (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)
                  : (c <= 65786 || (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)))))
              : (c <= 66204 || (c < 66432
                ? (c < 66304
                  ? (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)
                  : (c <= 66335 || (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)))))))
            : (c <= 66771 || (c < 67072
              ? (c < 66956
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)))
                : (c <= 66962 || (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))))
              : (c <= 67382 || (c < 67584
                ? (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)))
                : (c <= 67589 || (c < 67639
                  ? (c < 67594
                    ? c == 67592
                    : c <= 67637)
                  : (c <= 67640 || c == 67644))))))))))
        : (c <= 67669 || (c < 70006
          ? (c < 68608
            ? (c < 68117
              ? (c < 67872
                ? (c < 67808
                  ? (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)
                  : (c <= 67826 || (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)))
                : (c <= 67897 || (c < 68096
                  ? (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)
                  : (c <= 68099 || (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)))))
              : (c <= 68119 || (c < 68288
                ? (c < 68159
                  ? (c < 68152
                    ? (c >= 68121 && c <= 68149)
                    : c <= 68154)
                  : (c <= 68159 || (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)))
                : (c <= 68295 || (c < 68416
                  ? (c < 68352
                    ? (c >= 68297 && c <= 68326)
                    : c <= 68405)
                  : (c <= 68437 || (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)))))))
            : (c <= 68680 || (c < 69552
              ? (c < 69291
                ? (c < 68864
                  ? (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)
                  : (c <= 68903 || (c < 69248
                    ? (c >= 68912 && c <= 68921)
                    : c <= 69289)))
                : (c <= 69292 || (c < 69415
                  ? (c < 69376
                    ? (c >= 69296 && c <= 69297)
                    : c <= 69404)
                  : (c <= 69415 || (c < 69488
                    ? (c >= 69424 && c <= 69456)
                    : c <= 69509)))))
              : (c <= 69572 || (c < 69840
                ? (c < 69734
                  ? (c < 69632
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69702)
                  : (c <= 69749 || (c < 69826
                    ? (c >= 69759 && c <= 69818)
                    : c <= 69826)))
                : (c <= 69864 || (c < 69942
                  ? (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)
                  : (c <= 69951 || (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)))))))))
          : (c <= 70006 || (c < 70475
            ? (c < 70303
              ? (c < 70163
                ? (c < 70094
                  ? (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)
                  : (c <= 70106 || (c < 70144
                    ? c == 70108
                    : c <= 70161)))
                : (c <= 70199 || (c < 70280
                  ? (c < 70272
                    ? c == 70206
                    : c <= 70278)
                  : (c <= 70280 || (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)))))
              : (c <= 70312 || (c < 70419
                ? (c < 70400
                  ? (c < 70384
                    ? (c >= 70320 && c <= 70378)
                    : c <= 70393)
                  : (c <= 70403 || (c < 70415
                    ? (c >= 70405 && c <= 70412)
                    : c <= 70416)))
                : (c <= 70440 || (c < 70453
                  ? (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)
                  : (c <= 70457 || (c < 70471
                    ? (c >= 70459 && c <= 70468)
                    : c <= 70472)))))))
            : (c <= 70477 || (c < 71040
              ? (c < 70656
                ? (c < 70493
                  ? (c < 70487
                    ? c == 70480
                    : c <= 70487)
                  : (c <= 70499 || (c < 70512
                    ? (c >= 70502 && c <= 70508)
                    : c <= 70516)))
                : (c <= 70730 || (c < 70784
                  ? (c < 70750
                    ? (c >= 70736 && c <= 70745)
                    : c <= 70753)
                  : (c <= 70853 || (c < 70864
                    ? c == 70855
                    : c <= 70873)))))
              : (c <= 71093 || (c < 71296
                ? (c < 71168
                  ? (c < 71128
                    ? (c >= 71096 && c <= 71104)
                    : c <= 71133)
                  : (c <= 71232 || (c < 71248
                    ? c == 71236
                    : c <= 71257)))
                : (c <= 71352 || (c < 71453
                  ? (c < 71424
                    ? (c >= 71360 && c <= 71369)
                    : c <= 71450)
                  : (c <= 71467 || (c >= 71472 && c <= 71481)))))))))))))
      : (c <= 71494 || (c < 119977
        ? (c < 92160
          ? (c < 72818
            ? (c < 72106
              ? (c < 71957
                ? (c < 71935
                  ? (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)
                  : (c <= 71942 || (c < 71948
                    ? c == 71945
                    : c <= 71955)))
                : (c <= 71958 || (c < 71995
                  ? (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)
                  : (c <= 72003 || (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)))))
              : (c <= 72151 || (c < 72349
                ? (c < 72192
                  ? (c < 72163
                    ? (c >= 72154 && c <= 72161)
                    : c <= 72164)
                  : (c <= 72254 || (c < 72272
                    ? c == 72263
                    : c <= 72345)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72758 || (c < 72784
                    ? (c >= 72760 && c <= 72768)
                    : c <= 72793)))))))
            : (c <= 72847 || (c < 73066
              ? (c < 73018
                ? (c < 72960
                  ? (c < 72873
                    ? (c >= 72850 && c <= 72871)
                    : c <= 72886)
                  : (c <= 72966 || (c < 72971
                    ? (c >= 72968 && c <= 72969)
                    : c <= 73014)))
                : (c <= 73018 || (c < 73040
                  ? (c < 73023
                    ? (c >= 73020 && c <= 73021)
                    : c <= 73031)
                  : (c <= 73049 || (c < 73063
                    ? (c >= 73056 && c <= 73061)
                    : c <= 73064)))))
              : (c <= 73102 || (c < 73728
                ? (c < 73120
                  ? (c < 73107
                    ? (c >= 73104 && c <= 73105)
                    : c <= 73112)
                  : (c <= 73129 || (c < 73648
                    ? (c >= 73440 && c <= 73462)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c < 82944
                    ? (c >= 77824 && c <= 78894)
                    : c <= 83526)))))))))
          : (c <= 92728 || (c < 110589
            ? (c < 93760
              ? (c < 92912
                ? (c < 92784
                  ? (c < 92768
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92777)
                  : (c <= 92862 || (c < 92880
                    ? (c >= 92864 && c <= 92873)
                    : c <= 92909)))
                : (c <= 92916 || (c < 93008
                  ? (c < 92992
                    ? (c >= 92928 && c <= 92982)
                    : c <= 92995)
                  : (c <= 93017 || (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)))))
              : (c <= 93823 || (c < 94192
                ? (c < 94095
                  ? (c < 94031
                    ? (c >= 93952 && c <= 94026)
                    : c <= 94087)
                  : (c <= 94111 || (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)))
                : (c <= 94193 || (c < 101632
                  ? (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)
                  : (c <= 101640 || (c < 110581
                    ? (c >= 110576 && c <= 110579)
                    : c <= 110587)))))))
            : (c <= 110590 || (c < 119141
              ? (c < 113776
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))
                : (c <= 113788 || (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 118576
                    ? (c >= 118528 && c <= 118573)
                    : c <= 118598)))))
              : (c <= 119145 || (c < 119808
                ? (c < 119173
                  ? (c < 119163
                    ? (c >= 119149 && c <= 119154)
                    : c <= 119170)
                  : (c <= 119179 || (c < 119362
                    ? (c >= 119210 && c <= 119213)
                    : c <= 119364)))
                : (c <= 119892 || (c < 119970
                  ? (c < 119966
                    ? (c >= 119894 && c <= 119964)
                    : c <= 119967)
                  : (c <= 119970 || (c >= 119973 && c <= 119974)))))))))))
        : (c <= 119980 || (c < 124912
          ? (c < 120772
            ? (c < 120138
              ? (c < 120077
                ? (c < 119997
                  ? (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)
                  : (c <= 120003 || (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)))
                : (c <= 120084 || (c < 120123
                  ? (c < 120094
                    ? (c >= 120086 && c <= 120092)
                    : c <= 120121)
                  : (c <= 120126 || (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)))))
              : (c <= 120144 || (c < 120598
                ? (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c < 120572
                    ? (c >= 120540 && c <= 120570)
                    : c <= 120596)))
                : (c <= 120628 || (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c < 120746
                    ? (c >= 120714 && c <= 120744)
                    : c <= 120770)))))))
            : (c <= 120779 || (c < 122915
              ? (c < 121499
                ? (c < 121403
                  ? (c < 121344
                    ? (c >= 120782 && c <= 120831)
                    : c <= 121398)
                  : (c <= 121452 || (c < 121476
                    ? c == 121461
                    : c <= 121476)))
                : (c <= 121503 || (c < 122880
                  ? (c < 122624
                    ? (c >= 121505 && c <= 121519)
                    : c <= 122654)
                  : (c <= 122886 || (c < 122907
                    ? (c >= 122888 && c <= 122904)
                    : c <= 122913)))))
              : (c <= 122916 || (c < 123536
                ? (c < 123184
                  ? (c < 123136
                    ? (c >= 122918 && c <= 122922)
                    : c <= 123180)
                  : (c <= 123197 || (c < 123214
                    ? (c >= 123200 && c <= 123209)
                    : c <= 123214)))
                : (c <= 123566 || (c < 124904
                  ? (c < 124896
                    ? (c >= 123584 && c <= 123641)
                    : c <= 124902)
                  : (c <= 124907 || (c >= 124909 && c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static inline bool sym_identifier_character_set_9(int32_t c) {
  return (c < 43520
    ? (c < 3751
      ? (c < 2738
        ? (c < 2042
          ? (c < 931
            ? (c < 248
              ? (c < 170
                ? (c < 'A'
                  ? (c < '0'
                    ? c == '-'
                    : c <= '9')
                  : (c <= 'Z' || (c < 'b'
                    ? c == '_'
                    : c <= 'z')))
                : (c <= 170 || (c < 186
                  ? (c < 183
                    ? c == 181
                    : c <= 183)
                  : (c <= 186 || (c < 216
                    ? (c >= 192 && c <= 214)
                    : c <= 246)))))
              : (c <= 705 || (c < 886
                ? (c < 748
                  ? (c < 736
                    ? (c >= 710 && c <= 721)
                    : c <= 740)
                  : (c <= 748 || (c < 768
                    ? c == 750
                    : c <= 884)))
                : (c <= 887 || (c < 902
                  ? (c < 895
                    ? (c >= 890 && c <= 893)
                    : c <= 895)
                  : (c <= 906 || (c < 910
                    ? c == 908
                    : c <= 929)))))))
            : (c <= 1013 || (c < 1488
              ? (c < 1376
                ? (c < 1162
                  ? (c < 1155
                    ? (c >= 1015 && c <= 1153)
                    : c <= 1159)
                  : (c <= 1327 || (c < 1369
                    ? (c >= 1329 && c <= 1366)
                    : c <= 1369)))
                : (c <= 1416 || (c < 1473
                  ? (c < 1471
                    ? (c >= 1425 && c <= 1469)
                    : c <= 1471)
                  : (c <= 1474 || (c < 1479
                    ? (c >= 1476 && c <= 1477)
                    : c <= 1479)))))
              : (c <= 1514 || (c < 1759
                ? (c < 1568
                  ? (c < 1552
                    ? (c >= 1519 && c <= 1522)
                    : c <= 1562)
                  : (c <= 1641 || (c < 1749
                    ? (c >= 1646 && c <= 1747)
                    : c <= 1756)))
                : (c <= 1768 || (c < 1808
                  ? (c < 1791
                    ? (c >= 1770 && c <= 1788)
                    : c <= 1791)
                  : (c <= 1866 || (c < 1984
                    ? (c >= 1869 && c <= 1969)
                    : c <= 2037)))))))))
          : (c <= 2042 || (c < 2556
            ? (c < 2447
              ? (c < 2185
                ? (c < 2112
                  ? (c < 2048
                    ? c == 2045
                    : c <= 2093)
                  : (c <= 2139 || (c < 2160
                    ? (c >= 2144 && c <= 2154)
                    : c <= 2183)))
                : (c <= 2190 || (c < 2406
                  ? (c < 2275
                    ? (c >= 2200 && c <= 2273)
                    : c <= 2403)
                  : (c <= 2415 || (c < 2437
                    ? (c >= 2417 && c <= 2435)
                    : c <= 2444)))))
              : (c <= 2448 || (c < 2503
                ? (c < 2482
                  ? (c < 2474
                    ? (c >= 2451 && c <= 2472)
                    : c <= 2480)
                  : (c <= 2482 || (c < 2492
                    ? (c >= 2486 && c <= 2489)
                    : c <= 2500)))
                : (c <= 2504 || (c < 2524
                  ? (c < 2519
                    ? (c >= 2507 && c <= 2510)
                    : c <= 2519)
                  : (c <= 2525 || (c < 2534
                    ? (c >= 2527 && c <= 2531)
                    : c <= 2545)))))))
            : (c <= 2556 || (c < 2631
              ? (c < 2602
                ? (c < 2565
                  ? (c < 2561
                    ? c == 2558
                    : c <= 2563)
                  : (c <= 2570 || (c < 2579
                    ? (c >= 2575 && c <= 2576)
                    : c <= 2600)))
                : (c <= 2608 || (c < 2616
                  ? (c < 2613
                    ? (c >= 2610 && c <= 2611)
                    : c <= 2614)
                  : (c <= 2617 || (c < 2622
                    ? c == 2620
                    : c <= 2626)))))
              : (c <= 2632 || (c < 2689
                ? (c < 2649
                  ? (c < 2641
                    ? (c >= 2635 && c <= 2637)
                    : c <= 2641)
                  : (c <= 2652 || (c < 2662
                    ? c == 2654
                    : c <= 2677)))
                : (c <= 2691 || (c < 2707
                  ? (c < 2703
                    ? (c >= 2693 && c <= 2701)
                    : c <= 2705)
                  : (c <= 2728 || (c >= 2730 && c <= 2736)))))))))))
        : (c <= 2739 || (c < 3160
          ? (c < 2946
            ? (c < 2835
              ? (c < 2784
                ? (c < 2759
                  ? (c < 2748
                    ? (c >= 2741 && c <= 2745)
                    : c <= 2757)
                  : (c <= 2761 || (c < 2768
                    ? (c >= 2763 && c <= 2765)
                    : c <= 2768)))
                : (c <= 2787 || (c < 2817
                  ? (c < 2809
                    ? (c >= 2790 && c <= 2799)
                    : c <= 2815)
                  : (c <= 2819 || (c < 2831
                    ? (c >= 2821 && c <= 2828)
                    : c <= 2832)))))
              : (c <= 2856 || (c < 2891
                ? (c < 2869
                  ? (c < 2866
                    ? (c >= 2858 && c <= 2864)
                    : c <= 2867)
                  : (c <= 2873 || (c < 2887
                    ? (c >= 2876 && c <= 2884)
                    : c <= 2888)))
                : (c <= 2893 || (c < 2911
                  ? (c < 2908
                    ? (c >= 2901 && c <= 2903)
                    : c <= 2909)
                  : (c <= 2915 || (c < 2929
                    ? (c >= 2918 && c <= 2927)
                    : c <= 2929)))))))
            : (c <= 2947 || (c < 3018
              ? (c < 2974
                ? (c < 2962
                  ? (c < 2958
                    ? (c >= 2949 && c <= 2954)
                    : c <= 2960)
                  : (c <= 2965 || (c < 2972
                    ? (c >= 2969 && c <= 2970)
                    : c <= 2972)))
                : (c <= 2975 || (c < 2990
                  ? (c < 2984
                    ? (c >= 2979 && c <= 2980)
                    : c <= 2986)
                  : (c <= 3001 || (c < 3014
                    ? (c >= 3006 && c <= 3010)
                    : c <= 3016)))))
              : (c <= 3021 || (c < 3090
                ? (c < 3046
                  ? (c < 3031
                    ? c == 3024
                    : c <= 3031)
                  : (c <= 3055 || (c < 3086
                    ? (c >= 3072 && c <= 3084)
                    : c <= 3088)))
                : (c <= 3112 || (c < 3142
                  ? (c < 3132
                    ? (c >= 3114 && c <= 3129)
                    : c <= 3140)
                  : (c <= 3144 || (c < 3157
                    ? (c >= 3146 && c <= 3149)
                    : c <= 3158)))))))))
          : (c <= 3162 || (c < 3423
            ? (c < 3274
              ? (c < 3214
                ? (c < 3174
                  ? (c < 3168
                    ? c == 3165
                    : c <= 3171)
                  : (c <= 3183 || (c < 3205
                    ? (c >= 3200 && c <= 3203)
                    : c <= 3212)))
                : (c <= 3216 || (c < 3253
                  ? (c < 3242
                    ? (c >= 3218 && c <= 3240)
                    : c <= 3251)
                  : (c <= 3257 || (c < 3270
                    ? (c >= 3260 && c <= 3268)
                    : c <= 3272)))))
              : (c <= 3277 || (c < 3328
                ? (c < 3296
                  ? (c < 3293
                    ? (c >= 3285 && c <= 3286)
                    : c <= 3294)
                  : (c <= 3299 || (c < 3313
                    ? (c >= 3302 && c <= 3311)
                    : c <= 3314)))
                : (c <= 3340 || (c < 3398
                  ? (c < 3346
                    ? (c >= 3342 && c <= 3344)
                    : c <= 3396)
                  : (c <= 3400 || (c < 3412
                    ? (c >= 3402 && c <= 3406)
                    : c <= 3415)))))))
            : (c <= 3427 || (c < 3544
              ? (c < 3507
                ? (c < 3457
                  ? (c < 3450
                    ? (c >= 3430 && c <= 3439)
                    : c <= 3455)
                  : (c <= 3459 || (c < 3482
                    ? (c >= 3461 && c <= 3478)
                    : c <= 3505)))
                : (c <= 3515 || (c < 3530
                  ? (c < 3520
                    ? c == 3517
                    : c <= 3526)
                  : (c <= 3530 || (c < 3542
                    ? (c >= 3535 && c <= 3540)
                    : c <= 3542)))))
              : (c <= 3551 || (c < 3713
                ? (c < 3585
                  ? (c < 3570
                    ? (c >= 3558 && c <= 3567)
                    : c <= 3571)
                  : (c <= 3642 || (c < 3664
                    ? (c >= 3648 && c <= 3662)
                    : c <= 3673)))
                : (c <= 3714 || (c < 3724
                  ? (c < 3718
                    ? c == 3716
                    : c <= 3722)
                  : (c <= 3747 || c == 3749))))))))))))
      : (c <= 3773 || (c < 7968
        ? (c < 5792
          ? (c < 4348
            ? (c < 3902
              ? (c < 3840
                ? (c < 3784
                  ? (c < 3782
                    ? (c >= 3776 && c <= 3780)
                    : c <= 3782)
                  : (c <= 3789 || (c < 3804
                    ? (c >= 3792 && c <= 3801)
                    : c <= 3807)))
                : (c <= 3840 || (c < 3893
                  ? (c < 3872
                    ? (c >= 3864 && c <= 3865)
                    : c <= 3881)
                  : (c <= 3893 || (c < 3897
                    ? c == 3895
                    : c <= 3897)))))
              : (c <= 3911 || (c < 4096
                ? (c < 3974
                  ? (c < 3953
                    ? (c >= 3913 && c <= 3948)
                    : c <= 3972)
                  : (c <= 3991 || (c < 4038
                    ? (c >= 3993 && c <= 4028)
                    : c <= 4038)))
                : (c <= 4169 || (c < 4295
                  ? (c < 4256
                    ? (c >= 4176 && c <= 4253)
                    : c <= 4293)
                  : (c <= 4295 || (c < 4304
                    ? c == 4301
                    : c <= 4346)))))))
            : (c <= 4680 || (c < 4808
              ? (c < 4746
                ? (c < 4696
                  ? (c < 4688
                    ? (c >= 4682 && c <= 4685)
                    : c <= 4694)
                  : (c <= 4696 || (c < 4704
                    ? (c >= 4698 && c <= 4701)
                    : c <= 4744)))
                : (c <= 4749 || (c < 4792
                  ? (c < 4786
                    ? (c >= 4752 && c <= 4784)
                    : c <= 4789)
                  : (c <= 4798 || (c < 4802
                    ? c == 4800
                    : c <= 4805)))))
              : (c <= 4822 || (c < 4992
                ? (c < 4888
                  ? (c < 4882
                    ? (c >= 4824 && c <= 4880)
                    : c <= 4885)
                  : (c <= 4954 || (c < 4969
                    ? (c >= 4957 && c <= 4959)
                    : c <= 4977)))
                : (c <= 5007 || (c < 5121
                  ? (c < 5112
                    ? (c >= 5024 && c <= 5109)
                    : c <= 5117)
                  : (c <= 5740 || (c < 5761
                    ? (c >= 5743 && c <= 5759)
                    : c <= 5786)))))))))
          : (c <= 5866 || (c < 6608
            ? (c < 6155
              ? (c < 5998
                ? (c < 5919
                  ? (c < 5888
                    ? (c >= 5870 && c <= 5880)
                    : c <= 5909)
                  : (c <= 5940 || (c < 5984
                    ? (c >= 5952 && c <= 5971)
                    : c <= 5996)))
                : (c <= 6000 || (c < 6103
                  ? (c < 6016
                    ? (c >= 6002 && c <= 6003)
                    : c <= 6099)
                  : (c <= 6103 || (c < 6112
                    ? (c >= 6108 && c <= 6109)
                    : c <= 6121)))))
              : (c <= 6157 || (c < 6432
                ? (c < 6272
                  ? (c < 6176
                    ? (c >= 6159 && c <= 6169)
                    : c <= 6264)
                  : (c <= 6314 || (c < 6400
                    ? (c >= 6320 && c <= 6389)
                    : c <= 6430)))
                : (c <= 6443 || (c < 6512
                  ? (c < 6470
                    ? (c >= 6448 && c <= 6459)
                    : c <= 6509)
                  : (c <= 6516 || (c < 6576
                    ? (c >= 6528 && c <= 6571)
                    : c <= 6601)))))))
            : (c <= 6618 || (c < 7040
              ? (c < 6823
                ? (c < 6752
                  ? (c < 6688
                    ? (c >= 6656 && c <= 6683)
                    : c <= 6750)
                  : (c <= 6780 || (c < 6800
                    ? (c >= 6783 && c <= 6793)
                    : c <= 6809)))
                : (c <= 6823 || (c < 6912
                  ? (c < 6847
                    ? (c >= 6832 && c <= 6845)
                    : c <= 6862)
                  : (c <= 6988 || (c < 7019
                    ? (c >= 6992 && c <= 7001)
                    : c <= 7027)))))
              : (c <= 7155 || (c < 7357
                ? (c < 7245
                  ? (c < 7232
                    ? (c >= 7168 && c <= 7223)
                    : c <= 7241)
                  : (c <= 7293 || (c < 7312
                    ? (c >= 7296 && c <= 7304)
                    : c <= 7354)))
                : (c <= 7359 || (c < 7424
                  ? (c < 7380
                    ? (c >= 7376 && c <= 7378)
                    : c <= 7418)
                  : (c <= 7957 || (c >= 7960 && c <= 7965)))))))))))
        : (c <= 8005 || (c < 11688
          ? (c < 8421
            ? (c < 8144
              ? (c < 8031
                ? (c < 8025
                  ? (c < 8016
                    ? (c >= 8008 && c <= 8013)
                    : c <= 8023)
                  : (c <= 8025 || (c < 8029
                    ? c == 8027
                    : c <= 8029)))
                : (c <= 8061 || (c < 8126
                  ? (c < 8118
                    ? (c >= 8064 && c <= 8116)
                    : c <= 8124)
                  : (c <= 8126 || (c < 8134
                    ? (c >= 8130 && c <= 8132)
                    : c <= 8140)))))
              : (c <= 8147 || (c < 8276
                ? (c < 8178
                  ? (c < 8160
                    ? (c >= 8150 && c <= 8155)
                    : c <= 8172)
                  : (c <= 8180 || (c < 8255
                    ? (c >= 8182 && c <= 8188)
                    : c <= 8256)))
                : (c <= 8276 || (c < 8336
                  ? (c < 8319
                    ? c == 8305
                    : c <= 8319)
                  : (c <= 8348 || (c < 8417
                    ? (c >= 8400 && c <= 8412)
                    : c <= 8417)))))))
            : (c <= 8432 || (c < 8526
              ? (c < 8484
                ? (c < 8458
                  ? (c < 8455
                    ? c == 8450
                    : c <= 8455)
                  : (c <= 8467 || (c < 8472
                    ? c == 8469
                    : c <= 8477)))
                : (c <= 8484 || (c < 8490
                  ? (c < 8488
                    ? c == 8486
                    : c <= 8488)
                  : (c <= 8505 || (c < 8517
                    ? (c >= 8508 && c <= 8511)
                    : c <= 8521)))))
              : (c <= 8526 || (c < 11565
                ? (c < 11499
                  ? (c < 11264
                    ? (c >= 8544 && c <= 8584)
                    : c <= 11492)
                  : (c <= 11507 || (c < 11559
                    ? (c >= 11520 && c <= 11557)
                    : c <= 11559)))
                : (c <= 11565 || (c < 11647
                  ? (c < 11631
                    ? (c >= 11568 && c <= 11623)
                    : c <= 11631)
                  : (c <= 11670 || (c >= 11680 && c <= 11686)))))))))
          : (c <= 11694 || (c < 42512
            ? (c < 12353
              ? (c < 11736
                ? (c < 11712
                  ? (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)
                  : (c <= 11718 || (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)))
                : (c <= 11742 || (c < 12321
                  ? (c < 12293
                    ? (c >= 11744 && c <= 11775)
                    : c <= 12295)
                  : (c <= 12335 || (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)))))
              : (c <= 12438 || (c < 12704
                ? (c < 12540
                  ? (c < 12449
                    ? (c >= 12441 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)))
                : (c <= 12735 || (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 42124 || (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)))))))
            : (c <= 42539 || (c < 43072
              ? (c < 42891
                ? (c < 42623
                  ? (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)
                  : (c <= 42737 || (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)))
                : (c <= 42954 || (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)))))
              : (c <= 43123 || (c < 43312
                ? (c < 43232
                  ? (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)
                  : (c <= 43255 || (c < 43261
                    ? c == 43259
                    : c <= 43309)))
                : (c <= 43347 || (c < 43471
                  ? (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)
                  : (c <= 43481 || (c >= 43488 && c <= 43518)))))))))))))))
    : (c <= 43574 || (c < 71488
      ? (c < 67647
        ? (c < 65482
          ? (c < 64275
            ? (c < 43816
              ? (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))
              : (c <= 43822 || (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))))
            : (c <= 64279 || (c < 65024
              ? (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64848
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64829)
                  : (c <= 64911 || (c < 65008
                    ? (c >= 64914 && c <= 64967)
                    : c <= 65019)))))
              : (c <= 65039 || (c < 65296
                ? (c < 65101
                  ? (c < 65075
                    ? (c >= 65056 && c <= 65071)
                    : c <= 65076)
                  : (c <= 65103 || (c < 65142
                    ? (c >= 65136 && c <= 65140)
                    : c <= 65276)))
                : (c <= 65305 || (c < 65345
                  ? (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)
                  : (c <= 65370 || (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)))))))))
          : (c <= 65487 || (c < 66736
            ? (c < 66176
              ? (c < 65596
                ? (c < 65536
                  ? (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)
                  : (c <= 65547 || (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)))
                : (c <= 65597 || (c < 65664
                  ? (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)
                  : (c <= 65786 || (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)))))
              : (c <= 66204 || (c < 66432
                ? (c < 66304
                  ? (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)
                  : (c <= 66335 || (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)))))))
            : (c <= 66771 || (c < 67072
              ? (c < 66956
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)))
                : (c <= 66962 || (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))))
              : (c <= 67382 || (c < 67584
                ? (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)))
                : (c <= 67589 || (c < 67639
                  ? (c < 67594
                    ? c == 67592
                    : c <= 67637)
                  : (c <= 67640 || c == 67644))))))))))
        : (c <= 67669 || (c < 70006
          ? (c < 68608
            ? (c < 68117
              ? (c < 67872
                ? (c < 67808
                  ? (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)
                  : (c <= 67826 || (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)))
                : (c <= 67897 || (c < 68096
                  ? (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)
                  : (c <= 68099 || (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)))))
              : (c <= 68119 || (c < 68288
                ? (c < 68159
                  ? (c < 68152
                    ? (c >= 68121 && c <= 68149)
                    : c <= 68154)
                  : (c <= 68159 || (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)))
                : (c <= 68295 || (c < 68416
                  ? (c < 68352
                    ? (c >= 68297 && c <= 68326)
                    : c <= 68405)
                  : (c <= 68437 || (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)))))))
            : (c <= 68680 || (c < 69552
              ? (c < 69291
                ? (c < 68864
                  ? (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)
                  : (c <= 68903 || (c < 69248
                    ? (c >= 68912 && c <= 68921)
                    : c <= 69289)))
                : (c <= 69292 || (c < 69415
                  ? (c < 69376
                    ? (c >= 69296 && c <= 69297)
                    : c <= 69404)
                  : (c <= 69415 || (c < 69488
                    ? (c >= 69424 && c <= 69456)
                    : c <= 69509)))))
              : (c <= 69572 || (c < 69840
                ? (c < 69734
                  ? (c < 69632
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69702)
                  : (c <= 69749 || (c < 69826
                    ? (c >= 69759 && c <= 69818)
                    : c <= 69826)))
                : (c <= 69864 || (c < 69942
                  ? (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)
                  : (c <= 69951 || (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)))))))))
          : (c <= 70006 || (c < 70475
            ? (c < 70303
              ? (c < 70163
                ? (c < 70094
                  ? (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)
                  : (c <= 70106 || (c < 70144
                    ? c == 70108
                    : c <= 70161)))
                : (c <= 70199 || (c < 70280
                  ? (c < 70272
                    ? c == 70206
                    : c <= 70278)
                  : (c <= 70280 || (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)))))
              : (c <= 70312 || (c < 70419
                ? (c < 70400
                  ? (c < 70384
                    ? (c >= 70320 && c <= 70378)
                    : c <= 70393)
                  : (c <= 70403 || (c < 70415
                    ? (c >= 70405 && c <= 70412)
                    : c <= 70416)))
                : (c <= 70440 || (c < 70453
                  ? (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)
                  : (c <= 70457 || (c < 70471
                    ? (c >= 70459 && c <= 70468)
                    : c <= 70472)))))))
            : (c <= 70477 || (c < 71040
              ? (c < 70656
                ? (c < 70493
                  ? (c < 70487
                    ? c == 70480
                    : c <= 70487)
                  : (c <= 70499 || (c < 70512
                    ? (c >= 70502 && c <= 70508)
                    : c <= 70516)))
                : (c <= 70730 || (c < 70784
                  ? (c < 70750
                    ? (c >= 70736 && c <= 70745)
                    : c <= 70753)
                  : (c <= 70853 || (c < 70864
                    ? c == 70855
                    : c <= 70873)))))
              : (c <= 71093 || (c < 71296
                ? (c < 71168
                  ? (c < 71128
                    ? (c >= 71096 && c <= 71104)
                    : c <= 71133)
                  : (c <= 71232 || (c < 71248
                    ? c == 71236
                    : c <= 71257)))
                : (c <= 71352 || (c < 71453
                  ? (c < 71424
                    ? (c >= 71360 && c <= 71369)
                    : c <= 71450)
                  : (c <= 71467 || (c >= 71472 && c <= 71481)))))))))))))
      : (c <= 71494 || (c < 119977
        ? (c < 92160
          ? (c < 72818
            ? (c < 72106
              ? (c < 71957
                ? (c < 71935
                  ? (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)
                  : (c <= 71942 || (c < 71948
                    ? c == 71945
                    : c <= 71955)))
                : (c <= 71958 || (c < 71995
                  ? (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)
                  : (c <= 72003 || (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)))))
              : (c <= 72151 || (c < 72349
                ? (c < 72192
                  ? (c < 72163
                    ? (c >= 72154 && c <= 72161)
                    : c <= 72164)
                  : (c <= 72254 || (c < 72272
                    ? c == 72263
                    : c <= 72345)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72758 || (c < 72784
                    ? (c >= 72760 && c <= 72768)
                    : c <= 72793)))))))
            : (c <= 72847 || (c < 73066
              ? (c < 73018
                ? (c < 72960
                  ? (c < 72873
                    ? (c >= 72850 && c <= 72871)
                    : c <= 72886)
                  : (c <= 72966 || (c < 72971
                    ? (c >= 72968 && c <= 72969)
                    : c <= 73014)))
                : (c <= 73018 || (c < 73040
                  ? (c < 73023
                    ? (c >= 73020 && c <= 73021)
                    : c <= 73031)
                  : (c <= 73049 || (c < 73063
                    ? (c >= 73056 && c <= 73061)
                    : c <= 73064)))))
              : (c <= 73102 || (c < 73728
                ? (c < 73120
                  ? (c < 73107
                    ? (c >= 73104 && c <= 73105)
                    : c <= 73112)
                  : (c <= 73129 || (c < 73648
                    ? (c >= 73440 && c <= 73462)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c < 82944
                    ? (c >= 77824 && c <= 78894)
                    : c <= 83526)))))))))
          : (c <= 92728 || (c < 110589
            ? (c < 93760
              ? (c < 92912
                ? (c < 92784
                  ? (c < 92768
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92777)
                  : (c <= 92862 || (c < 92880
                    ? (c >= 92864 && c <= 92873)
                    : c <= 92909)))
                : (c <= 92916 || (c < 93008
                  ? (c < 92992
                    ? (c >= 92928 && c <= 92982)
                    : c <= 92995)
                  : (c <= 93017 || (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)))))
              : (c <= 93823 || (c < 94192
                ? (c < 94095
                  ? (c < 94031
                    ? (c >= 93952 && c <= 94026)
                    : c <= 94087)
                  : (c <= 94111 || (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)))
                : (c <= 94193 || (c < 101632
                  ? (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)
                  : (c <= 101640 || (c < 110581
                    ? (c >= 110576 && c <= 110579)
                    : c <= 110587)))))))
            : (c <= 110590 || (c < 119141
              ? (c < 113776
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))
                : (c <= 113788 || (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 118576
                    ? (c >= 118528 && c <= 118573)
                    : c <= 118598)))))
              : (c <= 119145 || (c < 119808
                ? (c < 119173
                  ? (c < 119163
                    ? (c >= 119149 && c <= 119154)
                    : c <= 119170)
                  : (c <= 119179 || (c < 119362
                    ? (c >= 119210 && c <= 119213)
                    : c <= 119364)))
                : (c <= 119892 || (c < 119970
                  ? (c < 119966
                    ? (c >= 119894 && c <= 119964)
                    : c <= 119967)
                  : (c <= 119970 || (c >= 119973 && c <= 119974)))))))))))
        : (c <= 119980 || (c < 124912
          ? (c < 120772
            ? (c < 120138
              ? (c < 120077
                ? (c < 119997
                  ? (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)
                  : (c <= 120003 || (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)))
                : (c <= 120084 || (c < 120123
                  ? (c < 120094
                    ? (c >= 120086 && c <= 120092)
                    : c <= 120121)
                  : (c <= 120126 || (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)))))
              : (c <= 120144 || (c < 120598
                ? (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c < 120572
                    ? (c >= 120540 && c <= 120570)
                    : c <= 120596)))
                : (c <= 120628 || (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c < 120746
                    ? (c >= 120714 && c <= 120744)
                    : c <= 120770)))))))
            : (c <= 120779 || (c < 122915
              ? (c < 121499
                ? (c < 121403
                  ? (c < 121344
                    ? (c >= 120782 && c <= 120831)
                    : c <= 121398)
                  : (c <= 121452 || (c < 121476
                    ? c == 121461
                    : c <= 121476)))
                : (c <= 121503 || (c < 122880
                  ? (c < 122624
                    ? (c >= 121505 && c <= 121519)
                    : c <= 122654)
                  : (c <= 122886 || (c < 122907
                    ? (c >= 122888 && c <= 122904)
                    : c <= 122913)))))
              : (c <= 122916 || (c < 123536
                ? (c < 123184
                  ? (c < 123136
                    ? (c >= 122918 && c <= 122922)
                    : c <= 123180)
                  : (c <= 123197 || (c < 123214
                    ? (c >= 123200 && c <= 123209)
                    : c <= 123214)))
                : (c <= 123566 || (c < 124904
                  ? (c < 124896
                    ? (c >= 123584 && c <= 123641)
                    : c <= 124902)
                  : (c <= 124907 || (c >= 124909 && c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static inline bool sym_identifier_character_set_10(int32_t c) {
  return (c < 43520
    ? (c < 3751
      ? (c < 2738
        ? (c < 2042
          ? (c < 931
            ? (c < 248
              ? (c < 170
                ? (c < 'A'
                  ? (c < '0'
                    ? c == '-'
                    : c <= '9')
                  : (c <= 'Z' || (c < 'a'
                    ? c == '_'
                    : c <= 'z')))
                : (c <= 170 || (c < 186
                  ? (c < 183
                    ? c == 181
                    : c <= 183)
                  : (c <= 186 || (c < 216
                    ? (c >= 192 && c <= 214)
                    : c <= 246)))))
              : (c <= 705 || (c < 886
                ? (c < 748
                  ? (c < 736
                    ? (c >= 710 && c <= 721)
                    : c <= 740)
                  : (c <= 748 || (c < 768
                    ? c == 750
                    : c <= 884)))
                : (c <= 887 || (c < 902
                  ? (c < 895
                    ? (c >= 890 && c <= 893)
                    : c <= 895)
                  : (c <= 906 || (c < 910
                    ? c == 908
                    : c <= 929)))))))
            : (c <= 1013 || (c < 1488
              ? (c < 1376
                ? (c < 1162
                  ? (c < 1155
                    ? (c >= 1015 && c <= 1153)
                    : c <= 1159)
                  : (c <= 1327 || (c < 1369
                    ? (c >= 1329 && c <= 1366)
                    : c <= 1369)))
                : (c <= 1416 || (c < 1473
                  ? (c < 1471
                    ? (c >= 1425 && c <= 1469)
                    : c <= 1471)
                  : (c <= 1474 || (c < 1479
                    ? (c >= 1476 && c <= 1477)
                    : c <= 1479)))))
              : (c <= 1514 || (c < 1759
                ? (c < 1568
                  ? (c < 1552
                    ? (c >= 1519 && c <= 1522)
                    : c <= 1562)
                  : (c <= 1641 || (c < 1749
                    ? (c >= 1646 && c <= 1747)
                    : c <= 1756)))
                : (c <= 1768 || (c < 1808
                  ? (c < 1791
                    ? (c >= 1770 && c <= 1788)
                    : c <= 1791)
                  : (c <= 1866 || (c < 1984
                    ? (c >= 1869 && c <= 1969)
                    : c <= 2037)))))))))
          : (c <= 2042 || (c < 2556
            ? (c < 2447
              ? (c < 2185
                ? (c < 2112
                  ? (c < 2048
                    ? c == 2045
                    : c <= 2093)
                  : (c <= 2139 || (c < 2160
                    ? (c >= 2144 && c <= 2154)
                    : c <= 2183)))
                : (c <= 2190 || (c < 2406
                  ? (c < 2275
                    ? (c >= 2200 && c <= 2273)
                    : c <= 2403)
                  : (c <= 2415 || (c < 2437
                    ? (c >= 2417 && c <= 2435)
                    : c <= 2444)))))
              : (c <= 2448 || (c < 2503
                ? (c < 2482
                  ? (c < 2474
                    ? (c >= 2451 && c <= 2472)
                    : c <= 2480)
                  : (c <= 2482 || (c < 2492
                    ? (c >= 2486 && c <= 2489)
                    : c <= 2500)))
                : (c <= 2504 || (c < 2524
                  ? (c < 2519
                    ? (c >= 2507 && c <= 2510)
                    : c <= 2519)
                  : (c <= 2525 || (c < 2534
                    ? (c >= 2527 && c <= 2531)
                    : c <= 2545)))))))
            : (c <= 2556 || (c < 2631
              ? (c < 2602
                ? (c < 2565
                  ? (c < 2561
                    ? c == 2558
                    : c <= 2563)
                  : (c <= 2570 || (c < 2579
                    ? (c >= 2575 && c <= 2576)
                    : c <= 2600)))
                : (c <= 2608 || (c < 2616
                  ? (c < 2613
                    ? (c >= 2610 && c <= 2611)
                    : c <= 2614)
                  : (c <= 2617 || (c < 2622
                    ? c == 2620
                    : c <= 2626)))))
              : (c <= 2632 || (c < 2689
                ? (c < 2649
                  ? (c < 2641
                    ? (c >= 2635 && c <= 2637)
                    : c <= 2641)
                  : (c <= 2652 || (c < 2662
                    ? c == 2654
                    : c <= 2677)))
                : (c <= 2691 || (c < 2707
                  ? (c < 2703
                    ? (c >= 2693 && c <= 2701)
                    : c <= 2705)
                  : (c <= 2728 || (c >= 2730 && c <= 2736)))))))))))
        : (c <= 2739 || (c < 3160
          ? (c < 2946
            ? (c < 2835
              ? (c < 2784
                ? (c < 2759
                  ? (c < 2748
                    ? (c >= 2741 && c <= 2745)
                    : c <= 2757)
                  : (c <= 2761 || (c < 2768
                    ? (c >= 2763 && c <= 2765)
                    : c <= 2768)))
                : (c <= 2787 || (c < 2817
                  ? (c < 2809
                    ? (c >= 2790 && c <= 2799)
                    : c <= 2815)
                  : (c <= 2819 || (c < 2831
                    ? (c >= 2821 && c <= 2828)
                    : c <= 2832)))))
              : (c <= 2856 || (c < 2891
                ? (c < 2869
                  ? (c < 2866
                    ? (c >= 2858 && c <= 2864)
                    : c <= 2867)
                  : (c <= 2873 || (c < 2887
                    ? (c >= 2876 && c <= 2884)
                    : c <= 2888)))
                : (c <= 2893 || (c < 2911
                  ? (c < 2908
                    ? (c >= 2901 && c <= 2903)
                    : c <= 2909)
                  : (c <= 2915 || (c < 2929
                    ? (c >= 2918 && c <= 2927)
                    : c <= 2929)))))))
            : (c <= 2947 || (c < 3018
              ? (c < 2974
                ? (c < 2962
                  ? (c < 2958
                    ? (c >= 2949 && c <= 2954)
                    : c <= 2960)
                  : (c <= 2965 || (c < 2972
                    ? (c >= 2969 && c <= 2970)
                    : c <= 2972)))
                : (c <= 2975 || (c < 2990
                  ? (c < 2984
                    ? (c >= 2979 && c <= 2980)
                    : c <= 2986)
                  : (c <= 3001 || (c < 3014
                    ? (c >= 3006 && c <= 3010)
                    : c <= 3016)))))
              : (c <= 3021 || (c < 3090
                ? (c < 3046
                  ? (c < 3031
                    ? c == 3024
                    : c <= 3031)
                  : (c <= 3055 || (c < 3086
                    ? (c >= 3072 && c <= 3084)
                    : c <= 3088)))
                : (c <= 3112 || (c < 3142
                  ? (c < 3132
                    ? (c >= 3114 && c <= 3129)
                    : c <= 3140)
                  : (c <= 3144 || (c < 3157
                    ? (c >= 3146 && c <= 3149)
                    : c <= 3158)))))))))
          : (c <= 3162 || (c < 3423
            ? (c < 3274
              ? (c < 3214
                ? (c < 3174
                  ? (c < 3168
                    ? c == 3165
                    : c <= 3171)
                  : (c <= 3183 || (c < 3205
                    ? (c >= 3200 && c <= 3203)
                    : c <= 3212)))
                : (c <= 3216 || (c < 3253
                  ? (c < 3242
                    ? (c >= 3218 && c <= 3240)
                    : c <= 3251)
                  : (c <= 3257 || (c < 3270
                    ? (c >= 3260 && c <= 3268)
                    : c <= 3272)))))
              : (c <= 3277 || (c < 3328
                ? (c < 3296
                  ? (c < 3293
                    ? (c >= 3285 && c <= 3286)
                    : c <= 3294)
                  : (c <= 3299 || (c < 3313
                    ? (c >= 3302 && c <= 3311)
                    : c <= 3314)))
                : (c <= 3340 || (c < 3398
                  ? (c < 3346
                    ? (c >= 3342 && c <= 3344)
                    : c <= 3396)
                  : (c <= 3400 || (c < 3412
                    ? (c >= 3402 && c <= 3406)
                    : c <= 3415)))))))
            : (c <= 3427 || (c < 3544
              ? (c < 3507
                ? (c < 3457
                  ? (c < 3450
                    ? (c >= 3430 && c <= 3439)
                    : c <= 3455)
                  : (c <= 3459 || (c < 3482
                    ? (c >= 3461 && c <= 3478)
                    : c <= 3505)))
                : (c <= 3515 || (c < 3530
                  ? (c < 3520
                    ? c == 3517
                    : c <= 3526)
                  : (c <= 3530 || (c < 3542
                    ? (c >= 3535 && c <= 3540)
                    : c <= 3542)))))
              : (c <= 3551 || (c < 3713
                ? (c < 3585
                  ? (c < 3570
                    ? (c >= 3558 && c <= 3567)
                    : c <= 3571)
                  : (c <= 3642 || (c < 3664
                    ? (c >= 3648 && c <= 3662)
                    : c <= 3673)))
                : (c <= 3714 || (c < 3724
                  ? (c < 3718
                    ? c == 3716
                    : c <= 3722)
                  : (c <= 3747 || c == 3749))))))))))))
      : (c <= 3773 || (c < 7968
        ? (c < 5792
          ? (c < 4348
            ? (c < 3902
              ? (c < 3840
                ? (c < 3784
                  ? (c < 3782
                    ? (c >= 3776 && c <= 3780)
                    : c <= 3782)
                  : (c <= 3789 || (c < 3804
                    ? (c >= 3792 && c <= 3801)
                    : c <= 3807)))
                : (c <= 3840 || (c < 3893
                  ? (c < 3872
                    ? (c >= 3864 && c <= 3865)
                    : c <= 3881)
                  : (c <= 3893 || (c < 3897
                    ? c == 3895
                    : c <= 3897)))))
              : (c <= 3911 || (c < 4096
                ? (c < 3974
                  ? (c < 3953
                    ? (c >= 3913 && c <= 3948)
                    : c <= 3972)
                  : (c <= 3991 || (c < 4038
                    ? (c >= 3993 && c <= 4028)
                    : c <= 4038)))
                : (c <= 4169 || (c < 4295
                  ? (c < 4256
                    ? (c >= 4176 && c <= 4253)
                    : c <= 4293)
                  : (c <= 4295 || (c < 4304
                    ? c == 4301
                    : c <= 4346)))))))
            : (c <= 4680 || (c < 4808
              ? (c < 4746
                ? (c < 4696
                  ? (c < 4688
                    ? (c >= 4682 && c <= 4685)
                    : c <= 4694)
                  : (c <= 4696 || (c < 4704
                    ? (c >= 4698 && c <= 4701)
                    : c <= 4744)))
                : (c <= 4749 || (c < 4792
                  ? (c < 4786
                    ? (c >= 4752 && c <= 4784)
                    : c <= 4789)
                  : (c <= 4798 || (c < 4802
                    ? c == 4800
                    : c <= 4805)))))
              : (c <= 4822 || (c < 4992
                ? (c < 4888
                  ? (c < 4882
                    ? (c >= 4824 && c <= 4880)
                    : c <= 4885)
                  : (c <= 4954 || (c < 4969
                    ? (c >= 4957 && c <= 4959)
                    : c <= 4977)))
                : (c <= 5007 || (c < 5121
                  ? (c < 5112
                    ? (c >= 5024 && c <= 5109)
                    : c <= 5117)
                  : (c <= 5740 || (c < 5761
                    ? (c >= 5743 && c <= 5759)
                    : c <= 5786)))))))))
          : (c <= 5866 || (c < 6608
            ? (c < 6155
              ? (c < 5998
                ? (c < 5919
                  ? (c < 5888
                    ? (c >= 5870 && c <= 5880)
                    : c <= 5909)
                  : (c <= 5940 || (c < 5984
                    ? (c >= 5952 && c <= 5971)
                    : c <= 5996)))
                : (c <= 6000 || (c < 6103
                  ? (c < 6016
                    ? (c >= 6002 && c <= 6003)
                    : c <= 6099)
                  : (c <= 6103 || (c < 6112
                    ? (c >= 6108 && c <= 6109)
                    : c <= 6121)))))
              : (c <= 6157 || (c < 6432
                ? (c < 6272
                  ? (c < 6176
                    ? (c >= 6159 && c <= 6169)
                    : c <= 6264)
                  : (c <= 6314 || (c < 6400
                    ? (c >= 6320 && c <= 6389)
                    : c <= 6430)))
                : (c <= 6443 || (c < 6512
                  ? (c < 6470
                    ? (c >= 6448 && c <= 6459)
                    : c <= 6509)
                  : (c <= 6516 || (c < 6576
                    ? (c >= 6528 && c <= 6571)
                    : c <= 6601)))))))
            : (c <= 6618 || (c < 7040
              ? (c < 6823
                ? (c < 6752
                  ? (c < 6688
                    ? (c >= 6656 && c <= 6683)
                    : c <= 6750)
                  : (c <= 6780 || (c < 6800
                    ? (c >= 6783 && c <= 6793)
                    : c <= 6809)))
                : (c <= 6823 || (c < 6912
                  ? (c < 6847
                    ? (c >= 6832 && c <= 6845)
                    : c <= 6862)
                  : (c <= 6988 || (c < 7019
                    ? (c >= 6992 && c <= 7001)
                    : c <= 7027)))))
              : (c <= 7155 || (c < 7357
                ? (c < 7245
                  ? (c < 7232
                    ? (c >= 7168 && c <= 7223)
                    : c <= 7241)
                  : (c <= 7293 || (c < 7312
                    ? (c >= 7296 && c <= 7304)
                    : c <= 7354)))
                : (c <= 7359 || (c < 7424
                  ? (c < 7380
                    ? (c >= 7376 && c <= 7378)
                    : c <= 7418)
                  : (c <= 7957 || (c >= 7960 && c <= 7965)))))))))))
        : (c <= 8005 || (c < 11688
          ? (c < 8421
            ? (c < 8144
              ? (c < 8031
                ? (c < 8025
                  ? (c < 8016
                    ? (c >= 8008 && c <= 8013)
                    : c <= 8023)
                  : (c <= 8025 || (c < 8029
                    ? c == 8027
                    : c <= 8029)))
                : (c <= 8061 || (c < 8126
                  ? (c < 8118
                    ? (c >= 8064 && c <= 8116)
                    : c <= 8124)
                  : (c <= 8126 || (c < 8134
                    ? (c >= 8130 && c <= 8132)
                    : c <= 8140)))))
              : (c <= 8147 || (c < 8276
                ? (c < 8178
                  ? (c < 8160
                    ? (c >= 8150 && c <= 8155)
                    : c <= 8172)
                  : (c <= 8180 || (c < 8255
                    ? (c >= 8182 && c <= 8188)
                    : c <= 8256)))
                : (c <= 8276 || (c < 8336
                  ? (c < 8319
                    ? c == 8305
                    : c <= 8319)
                  : (c <= 8348 || (c < 8417
                    ? (c >= 8400 && c <= 8412)
                    : c <= 8417)))))))
            : (c <= 8432 || (c < 8526
              ? (c < 8484
                ? (c < 8458
                  ? (c < 8455
                    ? c == 8450
                    : c <= 8455)
                  : (c <= 8467 || (c < 8472
                    ? c == 8469
                    : c <= 8477)))
                : (c <= 8484 || (c < 8490
                  ? (c < 8488
                    ? c == 8486
                    : c <= 8488)
                  : (c <= 8505 || (c < 8517
                    ? (c >= 8508 && c <= 8511)
                    : c <= 8521)))))
              : (c <= 8526 || (c < 11565
                ? (c < 11499
                  ? (c < 11264
                    ? (c >= 8544 && c <= 8584)
                    : c <= 11492)
                  : (c <= 11507 || (c < 11559
                    ? (c >= 11520 && c <= 11557)
                    : c <= 11559)))
                : (c <= 11565 || (c < 11647
                  ? (c < 11631
                    ? (c >= 11568 && c <= 11623)
                    : c <= 11631)
                  : (c <= 11670 || (c >= 11680 && c <= 11686)))))))))
          : (c <= 11694 || (c < 42512
            ? (c < 12353
              ? (c < 11736
                ? (c < 11712
                  ? (c < 11704
                    ? (c >= 11696 && c <= 11702)
                    : c <= 11710)
                  : (c <= 11718 || (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)))
                : (c <= 11742 || (c < 12321
                  ? (c < 12293
                    ? (c >= 11744 && c <= 11775)
                    : c <= 12295)
                  : (c <= 12335 || (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)))))
              : (c <= 12438 || (c < 12704
                ? (c < 12540
                  ? (c < 12449
                    ? (c >= 12441 && c <= 12447)
                    : c <= 12538)
                  : (c <= 12543 || (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)))
                : (c <= 12735 || (c < 19968
                  ? (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)
                  : (c <= 42124 || (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)))))))
            : (c <= 42539 || (c < 43072
              ? (c < 42891
                ? (c < 42623
                  ? (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)
                  : (c <= 42737 || (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)))
                : (c <= 42954 || (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)))))
              : (c <= 43123 || (c < 43312
                ? (c < 43232
                  ? (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)
                  : (c <= 43255 || (c < 43261
                    ? c == 43259
                    : c <= 43309)))
                : (c <= 43347 || (c < 43471
                  ? (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)
                  : (c <= 43481 || (c >= 43488 && c <= 43518)))))))))))))))
    : (c <= 43574 || (c < 71488
      ? (c < 67647
        ? (c < 65482
          ? (c < 64275
            ? (c < 43816
              ? (c < 43744
                ? (c < 43616
                  ? (c < 43600
                    ? (c >= 43584 && c <= 43597)
                    : c <= 43609)
                  : (c <= 43638 || (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)))
                : (c <= 43759 || (c < 43785
                  ? (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)
                  : (c <= 43790 || (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)))))
              : (c <= 43822 || (c < 44032
                ? (c < 43888
                  ? (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)
                  : (c <= 44010 || (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)))
                : (c <= 55203 || (c < 63744
                  ? (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)
                  : (c <= 64109 || (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)))))))
            : (c <= 64279 || (c < 65024
              ? (c < 64323
                ? (c < 64312
                  ? (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)
                  : (c <= 64316 || (c < 64320
                    ? c == 64318
                    : c <= 64321)))
                : (c <= 64324 || (c < 64848
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64829)
                  : (c <= 64911 || (c < 65008
                    ? (c >= 64914 && c <= 64967)
                    : c <= 65019)))))
              : (c <= 65039 || (c < 65296
                ? (c < 65101
                  ? (c < 65075
                    ? (c >= 65056 && c <= 65071)
                    : c <= 65076)
                  : (c <= 65103 || (c < 65142
                    ? (c >= 65136 && c <= 65140)
                    : c <= 65276)))
                : (c <= 65305 || (c < 65345
                  ? (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)
                  : (c <= 65370 || (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)))))))))
          : (c <= 65487 || (c < 66736
            ? (c < 66176
              ? (c < 65596
                ? (c < 65536
                  ? (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)
                  : (c <= 65547 || (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)))
                : (c <= 65597 || (c < 65664
                  ? (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)
                  : (c <= 65786 || (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)))))
              : (c <= 66204 || (c < 66432
                ? (c < 66304
                  ? (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)
                  : (c <= 66335 || (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)))
                : (c <= 66461 || (c < 66513
                  ? (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)
                  : (c <= 66517 || (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)))))))
            : (c <= 66771 || (c < 67072
              ? (c < 66956
                ? (c < 66864
                  ? (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)
                  : (c <= 66915 || (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)))
                : (c <= 66962 || (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))))
              : (c <= 67382 || (c < 67584
                ? (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)))
                : (c <= 67589 || (c < 67639
                  ? (c < 67594
                    ? c == 67592
                    : c <= 67637)
                  : (c <= 67640 || c == 67644))))))))))
        : (c <= 67669 || (c < 70006
          ? (c < 68608
            ? (c < 68117
              ? (c < 67872
                ? (c < 67808
                  ? (c < 67712
                    ? (c >= 67680 && c <= 67702)
                    : c <= 67742)
                  : (c <= 67826 || (c < 67840
                    ? (c >= 67828 && c <= 67829)
                    : c <= 67861)))
                : (c <= 67897 || (c < 68096
                  ? (c < 68030
                    ? (c >= 67968 && c <= 68023)
                    : c <= 68031)
                  : (c <= 68099 || (c < 68108
                    ? (c >= 68101 && c <= 68102)
                    : c <= 68115)))))
              : (c <= 68119 || (c < 68288
                ? (c < 68159
                  ? (c < 68152
                    ? (c >= 68121 && c <= 68149)
                    : c <= 68154)
                  : (c <= 68159 || (c < 68224
                    ? (c >= 68192 && c <= 68220)
                    : c <= 68252)))
                : (c <= 68295 || (c < 68416
                  ? (c < 68352
                    ? (c >= 68297 && c <= 68326)
                    : c <= 68405)
                  : (c <= 68437 || (c < 68480
                    ? (c >= 68448 && c <= 68466)
                    : c <= 68497)))))))
            : (c <= 68680 || (c < 69552
              ? (c < 69291
                ? (c < 68864
                  ? (c < 68800
                    ? (c >= 68736 && c <= 68786)
                    : c <= 68850)
                  : (c <= 68903 || (c < 69248
                    ? (c >= 68912 && c <= 68921)
                    : c <= 69289)))
                : (c <= 69292 || (c < 69415
                  ? (c < 69376
                    ? (c >= 69296 && c <= 69297)
                    : c <= 69404)
                  : (c <= 69415 || (c < 69488
                    ? (c >= 69424 && c <= 69456)
                    : c <= 69509)))))
              : (c <= 69572 || (c < 69840
                ? (c < 69734
                  ? (c < 69632
                    ? (c >= 69600 && c <= 69622)
                    : c <= 69702)
                  : (c <= 69749 || (c < 69826
                    ? (c >= 69759 && c <= 69818)
                    : c <= 69826)))
                : (c <= 69864 || (c < 69942
                  ? (c < 69888
                    ? (c >= 69872 && c <= 69881)
                    : c <= 69940)
                  : (c <= 69951 || (c < 69968
                    ? (c >= 69956 && c <= 69959)
                    : c <= 70003)))))))))
          : (c <= 70006 || (c < 70475
            ? (c < 70303
              ? (c < 70163
                ? (c < 70094
                  ? (c < 70089
                    ? (c >= 70016 && c <= 70084)
                    : c <= 70092)
                  : (c <= 70106 || (c < 70144
                    ? c == 70108
                    : c <= 70161)))
                : (c <= 70199 || (c < 70280
                  ? (c < 70272
                    ? c == 70206
                    : c <= 70278)
                  : (c <= 70280 || (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)))))
              : (c <= 70312 || (c < 70419
                ? (c < 70400
                  ? (c < 70384
                    ? (c >= 70320 && c <= 70378)
                    : c <= 70393)
                  : (c <= 70403 || (c < 70415
                    ? (c >= 70405 && c <= 70412)
                    : c <= 70416)))
                : (c <= 70440 || (c < 70453
                  ? (c < 70450
                    ? (c >= 70442 && c <= 70448)
                    : c <= 70451)
                  : (c <= 70457 || (c < 70471
                    ? (c >= 70459 && c <= 70468)
                    : c <= 70472)))))))
            : (c <= 70477 || (c < 71040
              ? (c < 70656
                ? (c < 70493
                  ? (c < 70487
                    ? c == 70480
                    : c <= 70487)
                  : (c <= 70499 || (c < 70512
                    ? (c >= 70502 && c <= 70508)
                    : c <= 70516)))
                : (c <= 70730 || (c < 70784
                  ? (c < 70750
                    ? (c >= 70736 && c <= 70745)
                    : c <= 70753)
                  : (c <= 70853 || (c < 70864
                    ? c == 70855
                    : c <= 70873)))))
              : (c <= 71093 || (c < 71296
                ? (c < 71168
                  ? (c < 71128
                    ? (c >= 71096 && c <= 71104)
                    : c <= 71133)
                  : (c <= 71232 || (c < 71248
                    ? c == 71236
                    : c <= 71257)))
                : (c <= 71352 || (c < 71453
                  ? (c < 71424
                    ? (c >= 71360 && c <= 71369)
                    : c <= 71450)
                  : (c <= 71467 || (c >= 71472 && c <= 71481)))))))))))))
      : (c <= 71494 || (c < 119977
        ? (c < 92160
          ? (c < 72818
            ? (c < 72106
              ? (c < 71957
                ? (c < 71935
                  ? (c < 71840
                    ? (c >= 71680 && c <= 71738)
                    : c <= 71913)
                  : (c <= 71942 || (c < 71948
                    ? c == 71945
                    : c <= 71955)))
                : (c <= 71958 || (c < 71995
                  ? (c < 71991
                    ? (c >= 71960 && c <= 71989)
                    : c <= 71992)
                  : (c <= 72003 || (c < 72096
                    ? (c >= 72016 && c <= 72025)
                    : c <= 72103)))))
              : (c <= 72151 || (c < 72349
                ? (c < 72192
                  ? (c < 72163
                    ? (c >= 72154 && c <= 72161)
                    : c <= 72164)
                  : (c <= 72254 || (c < 72272
                    ? c == 72263
                    : c <= 72345)))
                : (c <= 72349 || (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72758 || (c < 72784
                    ? (c >= 72760 && c <= 72768)
                    : c <= 72793)))))))
            : (c <= 72847 || (c < 73066
              ? (c < 73018
                ? (c < 72960
                  ? (c < 72873
                    ? (c >= 72850 && c <= 72871)
                    : c <= 72886)
                  : (c <= 72966 || (c < 72971
                    ? (c >= 72968 && c <= 72969)
                    : c <= 73014)))
                : (c <= 73018 || (c < 73040
                  ? (c < 73023
                    ? (c >= 73020 && c <= 73021)
                    : c <= 73031)
                  : (c <= 73049 || (c < 73063
                    ? (c >= 73056 && c <= 73061)
                    : c <= 73064)))))
              : (c <= 73102 || (c < 73728
                ? (c < 73120
                  ? (c < 73107
                    ? (c >= 73104 && c <= 73105)
                    : c <= 73112)
                  : (c <= 73129 || (c < 73648
                    ? (c >= 73440 && c <= 73462)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c < 82944
                    ? (c >= 77824 && c <= 78894)
                    : c <= 83526)))))))))
          : (c <= 92728 || (c < 110589
            ? (c < 93760
              ? (c < 92912
                ? (c < 92784
                  ? (c < 92768
                    ? (c >= 92736 && c <= 92766)
                    : c <= 92777)
                  : (c <= 92862 || (c < 92880
                    ? (c >= 92864 && c <= 92873)
                    : c <= 92909)))
                : (c <= 92916 || (c < 93008
                  ? (c < 92992
                    ? (c >= 92928 && c <= 92982)
                    : c <= 92995)
                  : (c <= 93017 || (c < 93053
                    ? (c >= 93027 && c <= 93047)
                    : c <= 93071)))))
              : (c <= 93823 || (c < 94192
                ? (c < 94095
                  ? (c < 94031
                    ? (c >= 93952 && c <= 94026)
                    : c <= 94087)
                  : (c <= 94111 || (c < 94179
                    ? (c >= 94176 && c <= 94177)
                    : c <= 94180)))
                : (c <= 94193 || (c < 101632
                  ? (c < 100352
                    ? (c >= 94208 && c <= 100343)
                    : c <= 101589)
                  : (c <= 101640 || (c < 110581
                    ? (c >= 110576 && c <= 110579)
                    : c <= 110587)))))))
            : (c <= 110590 || (c < 119141
              ? (c < 113776
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c < 113664
                    ? (c >= 110960 && c <= 111355)
                    : c <= 113770)))
                : (c <= 113788 || (c < 113821
                  ? (c < 113808
                    ? (c >= 113792 && c <= 113800)
                    : c <= 113817)
                  : (c <= 113822 || (c < 118576
                    ? (c >= 118528 && c <= 118573)
                    : c <= 118598)))))
              : (c <= 119145 || (c < 119808
                ? (c < 119173
                  ? (c < 119163
                    ? (c >= 119149 && c <= 119154)
                    : c <= 119170)
                  : (c <= 119179 || (c < 119362
                    ? (c >= 119210 && c <= 119213)
                    : c <= 119364)))
                : (c <= 119892 || (c < 119970
                  ? (c < 119966
                    ? (c >= 119894 && c <= 119964)
                    : c <= 119967)
                  : (c <= 119970 || (c >= 119973 && c <= 119974)))))))))))
        : (c <= 119980 || (c < 124912
          ? (c < 120772
            ? (c < 120138
              ? (c < 120077
                ? (c < 119997
                  ? (c < 119995
                    ? (c >= 119982 && c <= 119993)
                    : c <= 119995)
                  : (c <= 120003 || (c < 120071
                    ? (c >= 120005 && c <= 120069)
                    : c <= 120074)))
                : (c <= 120084 || (c < 120123
                  ? (c < 120094
                    ? (c >= 120086 && c <= 120092)
                    : c <= 120121)
                  : (c <= 120126 || (c < 120134
                    ? (c >= 120128 && c <= 120132)
                    : c <= 120134)))))
              : (c <= 120144 || (c < 120598
                ? (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c < 120572
                    ? (c >= 120540 && c <= 120570)
                    : c <= 120596)))
                : (c <= 120628 || (c < 120688
                  ? (c < 120656
                    ? (c >= 120630 && c <= 120654)
                    : c <= 120686)
                  : (c <= 120712 || (c < 120746
                    ? (c >= 120714 && c <= 120744)
                    : c <= 120770)))))))
            : (c <= 120779 || (c < 122915
              ? (c < 121499
                ? (c < 121403
                  ? (c < 121344
                    ? (c >= 120782 && c <= 120831)
                    : c <= 121398)
                  : (c <= 121452 || (c < 121476
                    ? c == 121461
                    : c <= 121476)))
                : (c <= 121503 || (c < 122880
                  ? (c < 122624
                    ? (c >= 121505 && c <= 121519)
                    : c <= 122654)
                  : (c <= 122886 || (c < 122907
                    ? (c >= 122888 && c <= 122904)
                    : c <= 122913)))))
              : (c <= 122916 || (c < 123536
                ? (c < 123184
                  ? (c < 123136
                    ? (c >= 122918 && c <= 122922)
                    : c <= 123180)
                  : (c <= 123197 || (c < 123214
                    ? (c >= 123200 && c <= 123209)
                    : c <= 123214)))
                : (c <= 123566 || (c < 124904
                  ? (c < 124896
                    ? (c >= 123584 && c <= 123641)
                    : c <= 124902)
                  : (c <= 124907 || (c >= 124909 && c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  switch (state) {
    case 0:
      if (eof) ADVANCE(42);
      if (lookahead == '!') ADVANCE(92);
      if (lookahead == '#') ADVANCE(113);
      if (lookahead == '%') ADVANCE(95);
      if (lookahead == '&') ADVANCE(5);
      if (lookahead == '(') ADVANCE(60);
      if (lookahead == ')') ADVANCE(61);
      if (lookahead == '*') ADVANCE(93);
      if (lookahead == '+') ADVANCE(96);
      if (lookahead == ',') ADVANCE(73);
      if (lookahead == '-') ADVANCE(90);
      if (lookahead == '.') ADVANCE(79);
      if (lookahead == '/') ADVANCE(94);
      if (lookahead == ':') ADVANCE(77);
      if (lookahead == '<') ADVANCE(99);
      if (lookahead == '=') ADVANCE(44);
      if (lookahead == '>') ADVANCE(97);
      if (lookahead == '?') ADVANCE(89);
      if (lookahead == '[') ADVANCE(75);
      if (lookahead == ']') ADVANCE(76);
      if (lookahead == 'e') ADVANCE(24);
      if (lookahead == 'f') ADVANCE(15);
      if (lookahead == 'i') ADVANCE(21);
      if (lookahead == 'n') ADVANCE(34);
      if (lookahead == 't') ADVANCE(31);
      if (lookahead == '{') ADVANCE(45);
      if (lookahead == '|') ADVANCE(36);
      if (lookahead == '}') ADVANCE(46);
      if (lookahead == '~') ADVANCE(108);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(114);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(63);
      END_STATE();
    case 1:
      if (lookahead == '!') ADVANCE(92);
      if (lookahead == '#') ADVANCE(113);
      if (lookahead == '%') ADVANCE(95);
      if (lookahead == '&') ADVANCE(5);
      if (lookahead == '(') ADVANCE(60);
      if (lookahead == ')') ADVANCE(61);
      if (lookahead == '*') ADVANCE(93);
      if (lookahead == '+') ADVANCE(96);
      if (lookahead == ',') ADVANCE(73);
      if (lookahead == '-') ADVANCE(90);
      if (lookahead == '.') ADVANCE(78);
      if (lookahead == '/') ADVANCE(94);
      if (lookahead == '0') ADVANCE(62);
      if (lookahead == '<') ADVANCE(99);
      if (lookahead == '=') ADVANCE(13);
      if (lookahead == '>') ADVANCE(97);
      if (lookahead == '?') ADVANCE(89);
      if (lookahead == '[') ADVANCE(75);
      if (lookahead == ']') ADVANCE(76);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(59);
      if (lookahead == 'f') ADVANCE(58);
      if (lookahead == 'n') ADVANCE(57);
      if (lookahead == 't') ADVANCE(54);
      if (lookahead == '{') ADVANCE(45);
      if (lookahead == '|') ADVANCE(36);
      if (lookahead == '}') ADVANCE(46);
      if (lookahead == '~') ADVANCE(108);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(114);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(63);
      END_STATE();
    case 2:
      if (lookahead == '!') ADVANCE(11);
      if (lookahead == '#') ADVANCE(113);
      if (lookahead == '%') ADVANCE(95);
      if (lookahead == '&') ADVANCE(5);
      if (lookahead == '(') ADVANCE(60);
      if (lookahead == '*') ADVANCE(93);
      if (lookahead == '+') ADVANCE(96);
      if (lookahead == '-') ADVANCE(90);
      if (lookahead == '.') ADVANCE(79);
      if (lookahead == '/') ADVANCE(94);
      if (lookahead == '<') ADVANCE(100);
      if (lookahead == '=') ADVANCE(12);
      if (lookahead == '>') ADVANCE(97);
      if (lookahead == '?') ADVANCE(89);
      if (lookahead == '[') ADVANCE(75);
      if (lookahead == ']') ADVANCE(76);
      if (lookahead == 'i') ADVANCE(20);
      if (lookahead == '|') ADVANCE(36);
      if (lookahead == '}') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(114);
      END_STATE();
    case 3:
      if (lookahead == '!') ADVANCE(91);
      if (lookahead == '#') ADVANCE(113);
      if (lookahead == '(') ADVANCE(60);
      if (lookahead == '-') ADVANCE(90);
      if (lookahead == '/') ADVANCE(6);
      if (lookahead == '0') ADVANCE(62);
      if (lookahead == ':') ADVANCE(77);
      if (lookahead == '<') ADVANCE(10);
      if (lookahead == '=') ADVANCE(43);
      if (lookahead == '?') ADVANCE(89);
      if (lookahead == '[') ADVANCE(74);
      if (lookahead == ']') ADVANCE(76);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(59);
      if (lookahead == 'f') ADVANCE(47);
      if (lookahead == 'n') ADVANCE(57);
      if (lookahead == 't') ADVANCE(54);
      if (lookahead == '{') ADVANCE(45);
      if (lookahead == '}') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(114);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(63);
      END_STATE();
    case 4:
      if (lookahead == '#') ADVANCE(113);
      if (lookahead == '/') ADVANCE(6);
      if (lookahead == '=') ADVANCE(43);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(59);
      if (lookahead == '{') ADVANCE(45);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(114);
      END_STATE();
    case 5:
      if (lookahead == '&') ADVANCE(104);
      END_STATE();
    case 6:
      if (lookahead == '*') ADVANCE(8);
      if (lookahead == '/') ADVANCE(113);
      END_STATE();
    case 7:
      if (lookahead == '*') ADVANCE(7);
      if (lookahead == '/') ADVANCE(112);
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 8:
      if (lookahead == '*') ADVANCE(7);
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 9:
      if (lookahead == '.') ADVANCE(88);
      END_STATE();
    case 10:
      if (lookahead == '<') ADVANCE(106);
      END_STATE();
    case 11:
      if (lookahead == '=') ADVANCE(103);
      END_STATE();
    case 12:
      if (lookahead == '=') ADVANCE(102);
      END_STATE();
    case 13:
      if (lookahead == '=') ADVANCE(102);
      if (lookahead == '>') ADVANCE(83);
      END_STATE();
    case 14:
      if (lookahead == ']') ADVANCE(82);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(27);
      if (lookahead == 'o') ADVANCE(29);
      END_STATE();
    case 16:
      if (lookahead == 'd') ADVANCE(22);
      END_STATE();
    case 17:
      if (lookahead == 'e') ADVANCE(110);
      END_STATE();
    case 18:
      if (lookahead == 'e') ADVANCE(67);
      END_STATE();
    case 19:
      if (lookahead == 'e') ADVANCE(69);
      END_STATE();
    case 20:
      if (lookahead == 'f') ADVANCE(87);
      END_STATE();
    case 21:
      if (lookahead == 'f') ADVANCE(87);
      if (lookahead == 'n') ADVANCE(86);
      END_STATE();
    case 22:
      if (lookahead == 'f') ADVANCE(28);
      if (lookahead == 'i') ADVANCE(23);
      END_STATE();
    case 23:
      if (lookahead == 'f') ADVANCE(111);
      END_STATE();
    case 24:
      if (lookahead == 'l') ADVANCE(32);
      if (lookahead == 'n') ADVANCE(16);
      END_STATE();
    case 25:
      if (lookahead == 'l') ADVANCE(71);
      END_STATE();
    case 26:
      if (lookahead == 'l') ADVANCE(25);
      END_STATE();
    case 27:
      if (lookahead == 'l') ADVANCE(33);
      END_STATE();
    case 28:
      if (lookahead == 'o') ADVANCE(30);
      END_STATE();
    case 29:
      if (lookahead == 'r') ADVANCE(84);
      END_STATE();
    case 30:
      if (lookahead == 'r') ADVANCE(109);
      END_STATE();
    case 31:
      if (lookahead == 'r') ADVANCE(35);
      END_STATE();
    case 32:
      if (lookahead == 's') ADVANCE(17);
      END_STATE();
    case 33:
      if (lookahead == 's') ADVANCE(19);
      END_STATE();
    case 34:
      if (lookahead == 'u') ADVANCE(26);
      END_STATE();
    case 35:
      if (lookahead == 'u') ADVANCE(18);
      END_STATE();
    case 36:
      if (lookahead == '|') ADVANCE(105);
      END_STATE();
    case 37:
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(65);
      END_STATE();
    case 38:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(64);
      END_STATE();
    case 39:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(65);
      END_STATE();
    case 40:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(66);
      END_STATE();
    case 41:
      if (eof) ADVANCE(42);
      if (lookahead == '!') ADVANCE(11);
      if (lookahead == '#') ADVANCE(113);
      if (lookahead == '%') ADVANCE(95);
      if (lookahead == '&') ADVANCE(5);
      if (lookahead == '(') ADVANCE(60);
      if (lookahead == ')') ADVANCE(61);
      if (lookahead == '*') ADVANCE(93);
      if (lookahead == '+') ADVANCE(96);
      if (lookahead == ',') ADVANCE(73);
      if (lookahead == '-') ADVANCE(90);
      if (lookahead == '.') ADVANCE(79);
      if (lookahead == '/') ADVANCE(94);
      if (lookahead == ':') ADVANCE(77);
      if (lookahead == '<') ADVANCE(100);
      if (lookahead == '=') ADVANCE(44);
      if (lookahead == '>') ADVANCE(97);
      if (lookahead == '?') ADVANCE(89);
      if (lookahead == '[') ADVANCE(75);
      if (lookahead == ']') ADVANCE(76);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(59);
      if (lookahead == '{') ADVANCE(45);
      if (lookahead == '|') ADVANCE(36);
      if (lookahead == '}') ADVANCE(46);
      if (lookahead == '~') ADVANCE(108);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(114);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(80);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(102);
      if (lookahead == '>') ADVANCE(83);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(59);
      if (lookahead == 'a') ADVANCE(50);
      if (lookahead == 'o') ADVANCE(53);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_4(lookahead)) ADVANCE(59);
      if (lookahead == 'e') ADVANCE(68);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_4(lookahead)) ADVANCE(59);
      if (lookahead == 'e') ADVANCE(70);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_5(lookahead)) ADVANCE(59);
      if (lookahead == 'l') ADVANCE(55);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_5(lookahead)) ADVANCE(59);
      if (lookahead == 'l') ADVANCE(72);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_5(lookahead)) ADVANCE(59);
      if (lookahead == 'l') ADVANCE(51);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_6(lookahead)) ADVANCE(59);
      if (lookahead == 'r') ADVANCE(85);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_6(lookahead)) ADVANCE(59);
      if (lookahead == 'r') ADVANCE(56);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_7(lookahead)) ADVANCE(59);
      if (lookahead == 's') ADVANCE(49);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_8(lookahead)) ADVANCE(59);
      if (lookahead == 'u') ADVANCE(48);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_8(lookahead)) ADVANCE(59);
      if (lookahead == 'u') ADVANCE(52);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_9(lookahead)) ADVANCE(59);
      if (lookahead == 'a') ADVANCE(50);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_identifier);
      if (sym_identifier_character_set_10(lookahead)) ADVANCE(59);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(aux_sym_numeric_lit_token1);
      if (lookahead == '.') ADVANCE(38);
      if (lookahead == 'x') ADVANCE(40);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(63);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(aux_sym_numeric_lit_token1);
      if (lookahead == '.') ADVANCE(38);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(63);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(aux_sym_numeric_lit_token1);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(37);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(64);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(aux_sym_numeric_lit_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(65);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(aux_sym_numeric_lit_token2);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(66);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(anon_sym_true);
      if (sym_identifier_character_set_10(lookahead)) ADVANCE(59);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_false);
      if (sym_identifier_character_set_10(lookahead)) ADVANCE(59);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_null_lit);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_null_lit);
      if (sym_identifier_character_set_10(lookahead)) ADVANCE(59);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      if (lookahead == '*') ADVANCE(14);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (lookahead == '*') ADVANCE(81);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (lookahead == '*') ADVANCE(81);
      if (lookahead == '.') ADVANCE(9);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(aux_sym_legacy_index_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(80);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(anon_sym_DOT_STAR);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(anon_sym_LBRACK_STAR_RBRACK);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(anon_sym_EQ_GT);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym_for);
      if (sym_identifier_character_set_10(lookahead)) ADVANCE(59);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(anon_sym_in);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(sym_ellipsis);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(anon_sym_QMARK);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(103);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(8);
      if (lookahead == '/') ADVANCE(113);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(98);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '<') ADVANCE(106);
      if (lookahead == '=') ADVANCE(101);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(101);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_AMP_AMP);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_PIPE_PIPE);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_LT_LT);
      if (lookahead == '-') ADVANCE(107);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_LT_LT_DASH);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(sym_strip_marker);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_endfor);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_endif);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(113);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(sym__whitespace);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 41},
  [2] = {.lex_state = 3, .external_lex_state = 2},
  [3] = {.lex_state = 3, .external_lex_state = 2},
  [4] = {.lex_state = 3, .external_lex_state = 2},
  [5] = {.lex_state = 3, .external_lex_state = 2},
  [6] = {.lex_state = 3, .external_lex_state = 2},
  [7] = {.lex_state = 1, .external_lex_state = 2},
  [8] = {.lex_state = 1, .external_lex_state = 2},
  [9] = {.lex_state = 3, .external_lex_state = 2},
  [10] = {.lex_state = 3, .external_lex_state = 2},
  [11] = {.lex_state = 1, .external_lex_state = 2},
  [12] = {.lex_state = 3, .external_lex_state = 2},
  [13] = {.lex_state = 3, .external_lex_state = 2},
  [14] = {.lex_state = 3, .external_lex_state = 2},
  [15] = {.lex_state = 1, .external_lex_state = 2},
  [16] = {.lex_state = 1, .external_lex_state = 2},
  [17] = {.lex_state = 1, .external_lex_state = 2},
  [18] = {.lex_state = 1, .external_lex_state = 2},
  [19] = {.lex_state = 1, .external_lex_state = 2},
  [20] = {.lex_state = 1, .external_lex_state = 2},
  [21] = {.lex_state = 1, .external_lex_state = 3},
  [22] = {.lex_state = 1, .external_lex_state = 2},
  [23] = {.lex_state = 1, .external_lex_state = 3},
  [24] = {.lex_state = 1, .external_lex_state = 3},
  [25] = {.lex_state = 1, .external_lex_state = 3},
  [26] = {.lex_state = 1, .external_lex_state = 3},
  [27] = {.lex_state = 1, .external_lex_state = 3},
  [28] = {.lex_state = 1, .external_lex_state = 2},
  [29] = {.lex_state = 1, .external_lex_state = 2},
  [30] = {.lex_state = 1, .external_lex_state = 2},
  [31] = {.lex_state = 1, .external_lex_state = 2},
  [32] = {.lex_state = 1, .external_lex_state = 2},
  [33] = {.lex_state = 1, .external_lex_state = 2},
  [34] = {.lex_state = 1, .external_lex_state = 2},
  [35] = {.lex_state = 1, .external_lex_state = 2},
  [36] = {.lex_state = 1, .external_lex_state = 2},
  [37] = {.lex_state = 1, .external_lex_state = 2},
  [38] = {.lex_state = 1, .external_lex_state = 2},
  [39] = {.lex_state = 1, .external_lex_state = 2},
  [40] = {.lex_state = 1, .external_lex_state = 2},
  [41] = {.lex_state = 1, .external_lex_state = 2},
  [42] = {.lex_state = 1, .external_lex_state = 2},
  [43] = {.lex_state = 1, .external_lex_state = 2},
  [44] = {.lex_state = 1, .external_lex_state = 2},
  [45] = {.lex_state = 1, .external_lex_state = 2},
  [46] = {.lex_state = 1, .external_lex_state = 2},
  [47] = {.lex_state = 1, .external_lex_state = 2},
  [48] = {.lex_state = 1, .external_lex_state = 2},
  [49] = {.lex_state = 1, .external_lex_state = 2},
  [50] = {.lex_state = 1, .external_lex_state = 2},
  [51] = {.lex_state = 1, .external_lex_state = 2},
  [52] = {.lex_state = 1, .external_lex_state = 2},
  [53] = {.lex_state = 1, .external_lex_state = 2},
  [54] = {.lex_state = 1, .external_lex_state = 2},
  [55] = {.lex_state = 1, .external_lex_state = 2},
  [56] = {.lex_state = 1, .external_lex_state = 2},
  [57] = {.lex_state = 1, .external_lex_state = 2},
  [58] = {.lex_state = 1, .external_lex_state = 2},
  [59] = {.lex_state = 1, .external_lex_state = 2},
  [60] = {.lex_state = 1, .external_lex_state = 2},
  [61] = {.lex_state = 1, .external_lex_state = 2},
  [62] = {.lex_state = 1, .external_lex_state = 2},
  [63] = {.lex_state = 1, .external_lex_state = 2},
  [64] = {.lex_state = 1, .external_lex_state = 2},
  [65] = {.lex_state = 1, .external_lex_state = 2},
  [66] = {.lex_state = 1, .external_lex_state = 2},
  [67] = {.lex_state = 1, .external_lex_state = 2},
  [68] = {.lex_state = 1, .external_lex_state = 2},
  [69] = {.lex_state = 1, .external_lex_state = 2},
  [70] = {.lex_state = 1, .external_lex_state = 2},
  [71] = {.lex_state = 1, .external_lex_state = 2},
  [72] = {.lex_state = 1, .external_lex_state = 2},
  [73] = {.lex_state = 1, .external_lex_state = 2},
  [74] = {.lex_state = 1, .external_lex_state = 2},
  [75] = {.lex_state = 1, .external_lex_state = 2},
  [76] = {.lex_state = 1, .external_lex_state = 2},
  [77] = {.lex_state = 1, .external_lex_state = 2},
  [78] = {.lex_state = 1, .external_lex_state = 2},
  [79] = {.lex_state = 1, .external_lex_state = 2},
  [80] = {.lex_state = 1, .external_lex_state = 2},
  [81] = {.lex_state = 1, .external_lex_state = 2},
  [82] = {.lex_state = 1, .external_lex_state = 2},
  [83] = {.lex_state = 1, .external_lex_state = 2},
  [84] = {.lex_state = 1, .external_lex_state = 2},
  [85] = {.lex_state = 1, .external_lex_state = 2},
  [86] = {.lex_state = 1, .external_lex_state = 2},
  [87] = {.lex_state = 1, .external_lex_state = 2},
  [88] = {.lex_state = 1, .external_lex_state = 2},
  [89] = {.lex_state = 1, .external_lex_state = 2},
  [90] = {.lex_state = 1, .external_lex_state = 2},
  [91] = {.lex_state = 1, .external_lex_state = 2},
  [92] = {.lex_state = 1, .external_lex_state = 2},
  [93] = {.lex_state = 1, .external_lex_state = 2},
  [94] = {.lex_state = 1, .external_lex_state = 2},
  [95] = {.lex_state = 1, .external_lex_state = 2},
  [96] = {.lex_state = 1, .external_lex_state = 2},
  [97] = {.lex_state = 1, .external_lex_state = 2},
  [98] = {.lex_state = 1, .external_lex_state = 2},
  [99] = {.lex_state = 1, .external_lex_state = 2},
  [100] = {.lex_state = 1, .external_lex_state = 2},
  [101] = {.lex_state = 1, .external_lex_state = 2},
  [102] = {.lex_state = 1, .external_lex_state = 2},
  [103] = {.lex_state = 1, .external_lex_state = 2},
  [104] = {.lex_state = 1, .external_lex_state = 2},
  [105] = {.lex_state = 1, .external_lex_state = 2},
  [106] = {.lex_state = 1, .external_lex_state = 2},
  [107] = {.lex_state = 1, .external_lex_state = 2},
  [108] = {.lex_state = 1, .external_lex_state = 2},
  [109] = {.lex_state = 1, .external_lex_state = 2},
  [110] = {.lex_state = 1, .external_lex_state = 2},
  [111] = {.lex_state = 1, .external_lex_state = 2},
  [112] = {.lex_state = 1, .external_lex_state = 2},
  [113] = {.lex_state = 1, .external_lex_state = 2},
  [114] = {.lex_state = 1, .external_lex_state = 2},
  [115] = {.lex_state = 1, .external_lex_state = 2},
  [116] = {.lex_state = 1, .external_lex_state = 2},
  [117] = {.lex_state = 1, .external_lex_state = 2},
  [118] = {.lex_state = 1, .external_lex_state = 2},
  [119] = {.lex_state = 1, .external_lex_state = 2},
  [120] = {.lex_state = 1, .external_lex_state = 2},
  [121] = {.lex_state = 1, .external_lex_state = 2},
  [122] = {.lex_state = 1, .external_lex_state = 2},
  [123] = {.lex_state = 1, .external_lex_state = 2},
  [124] = {.lex_state = 1, .external_lex_state = 2},
  [125] = {.lex_state = 1, .external_lex_state = 2},
  [126] = {.lex_state = 1, .external_lex_state = 2},
  [127] = {.lex_state = 1, .external_lex_state = 2},
  [128] = {.lex_state = 1, .external_lex_state = 2},
  [129] = {.lex_state = 41},
  [130] = {.lex_state = 41},
  [131] = {.lex_state = 41},
  [132] = {.lex_state = 41},
  [133] = {.lex_state = 41},
  [134] = {.lex_state = 41},
  [135] = {.lex_state = 41},
  [136] = {.lex_state = 41},
  [137] = {.lex_state = 41},
  [138] = {.lex_state = 41},
  [139] = {.lex_state = 41},
  [140] = {.lex_state = 41},
  [141] = {.lex_state = 1, .external_lex_state = 2},
  [142] = {.lex_state = 41},
  [143] = {.lex_state = 1, .external_lex_state = 2},
  [144] = {.lex_state = 1, .external_lex_state = 2},
  [145] = {.lex_state = 1, .external_lex_state = 2},
  [146] = {.lex_state = 1, .external_lex_state = 2},
  [147] = {.lex_state = 1, .external_lex_state = 2},
  [148] = {.lex_state = 1, .external_lex_state = 2},
  [149] = {.lex_state = 1, .external_lex_state = 2},
  [150] = {.lex_state = 1, .external_lex_state = 2},
  [151] = {.lex_state = 1, .external_lex_state = 2},
  [152] = {.lex_state = 1, .external_lex_state = 2},
  [153] = {.lex_state = 1, .external_lex_state = 2},
  [154] = {.lex_state = 1, .external_lex_state = 2},
  [155] = {.lex_state = 1, .external_lex_state = 2},
  [156] = {.lex_state = 1, .external_lex_state = 2},
  [157] = {.lex_state = 1, .external_lex_state = 2},
  [158] = {.lex_state = 1, .external_lex_state = 2},
  [159] = {.lex_state = 1, .external_lex_state = 2},
  [160] = {.lex_state = 1, .external_lex_state = 2},
  [161] = {.lex_state = 1, .external_lex_state = 2},
  [162] = {.lex_state = 1, .external_lex_state = 2},
  [163] = {.lex_state = 1, .external_lex_state = 2},
  [164] = {.lex_state = 1, .external_lex_state = 2},
  [165] = {.lex_state = 1, .external_lex_state = 2},
  [166] = {.lex_state = 1, .external_lex_state = 2},
  [167] = {.lex_state = 1, .external_lex_state = 2},
  [168] = {.lex_state = 1, .external_lex_state = 2},
  [169] = {.lex_state = 1, .external_lex_state = 2},
  [170] = {.lex_state = 1, .external_lex_state = 2},
  [171] = {.lex_state = 1, .external_lex_state = 2},
  [172] = {.lex_state = 1, .external_lex_state = 2},
  [173] = {.lex_state = 1, .external_lex_state = 2},
  [174] = {.lex_state = 1, .external_lex_state = 2},
  [175] = {.lex_state = 41},
  [176] = {.lex_state = 2},
  [177] = {.lex_state = 2},
  [178] = {.lex_state = 2},
  [179] = {.lex_state = 2},
  [180] = {.lex_state = 2},
  [181] = {.lex_state = 2},
  [182] = {.lex_state = 2},
  [183] = {.lex_state = 2},
  [184] = {.lex_state = 41},
  [185] = {.lex_state = 41},
  [186] = {.lex_state = 41},
  [187] = {.lex_state = 41},
  [188] = {.lex_state = 41},
  [189] = {.lex_state = 41},
  [190] = {.lex_state = 41},
  [191] = {.lex_state = 41},
  [192] = {.lex_state = 41},
  [193] = {.lex_state = 41},
  [194] = {.lex_state = 41},
  [195] = {.lex_state = 41},
  [196] = {.lex_state = 41},
  [197] = {.lex_state = 41},
  [198] = {.lex_state = 41},
  [199] = {.lex_state = 41},
  [200] = {.lex_state = 41},
  [201] = {.lex_state = 41},
  [202] = {.lex_state = 41},
  [203] = {.lex_state = 41},
  [204] = {.lex_state = 41},
  [205] = {.lex_state = 41},
  [206] = {.lex_state = 41},
  [207] = {.lex_state = 41},
  [208] = {.lex_state = 41},
  [209] = {.lex_state = 41},
  [210] = {.lex_state = 41},
  [211] = {.lex_state = 41},
  [212] = {.lex_state = 41},
  [213] = {.lex_state = 41},
  [214] = {.lex_state = 41},
  [215] = {.lex_state = 41},
  [216] = {.lex_state = 41, .external_lex_state = 4},
  [217] = {.lex_state = 41, .external_lex_state = 4},
  [218] = {.lex_state = 41, .external_lex_state = 4},
  [219] = {.lex_state = 41, .external_lex_state = 5},
  [220] = {.lex_state = 41, .external_lex_state = 5},
  [221] = {.lex_state = 2},
  [222] = {.lex_state = 41, .external_lex_state = 4},
  [223] = {.lex_state = 2},
  [224] = {.lex_state = 41, .external_lex_state = 4},
  [225] = {.lex_state = 41, .external_lex_state = 4},
  [226] = {.lex_state = 41, .external_lex_state = 5},
  [227] = {.lex_state = 41, .external_lex_state = 5},
  [228] = {.lex_state = 41, .external_lex_state = 4},
  [229] = {.lex_state = 41, .external_lex_state = 5},
  [230] = {.lex_state = 41, .external_lex_state = 5},
  [231] = {.lex_state = 2},
  [232] = {.lex_state = 41, .external_lex_state = 5},
  [233] = {.lex_state = 41, .external_lex_state = 5},
  [234] = {.lex_state = 2},
  [235] = {.lex_state = 2},
  [236] = {.lex_state = 41, .external_lex_state = 4},
  [237] = {.lex_state = 41, .external_lex_state = 5},
  [238] = {.lex_state = 41, .external_lex_state = 5},
  [239] = {.lex_state = 41, .external_lex_state = 4},
  [240] = {.lex_state = 41, .external_lex_state = 4},
  [241] = {.lex_state = 41, .external_lex_state = 5},
  [242] = {.lex_state = 41, .external_lex_state = 4},
  [243] = {.lex_state = 41, .external_lex_state = 5},
  [244] = {.lex_state = 41, .external_lex_state = 4},
  [245] = {.lex_state = 41, .external_lex_state = 5},
  [246] = {.lex_state = 41, .external_lex_state = 4},
  [247] = {.lex_state = 2},
  [248] = {.lex_state = 2},
  [249] = {.lex_state = 2},
  [250] = {.lex_state = 41, .external_lex_state = 4},
  [251] = {.lex_state = 2},
  [252] = {.lex_state = 2},
  [253] = {.lex_state = 2},
  [254] = {.lex_state = 2},
  [255] = {.lex_state = 2},
  [256] = {.lex_state = 2},
  [257] = {.lex_state = 2},
  [258] = {.lex_state = 2},
  [259] = {.lex_state = 2},
  [260] = {.lex_state = 2},
  [261] = {.lex_state = 2},
  [262] = {.lex_state = 2},
  [263] = {.lex_state = 41, .external_lex_state = 5},
  [264] = {.lex_state = 2},
  [265] = {.lex_state = 2},
  [266] = {.lex_state = 2},
  [267] = {.lex_state = 2},
  [268] = {.lex_state = 2},
  [269] = {.lex_state = 2},
  [270] = {.lex_state = 2},
  [271] = {.lex_state = 2},
  [272] = {.lex_state = 2},
  [273] = {.lex_state = 2},
  [274] = {.lex_state = 2},
  [275] = {.lex_state = 2},
  [276] = {.lex_state = 2},
  [277] = {.lex_state = 2},
  [278] = {.lex_state = 2},
  [279] = {.lex_state = 2},
  [280] = {.lex_state = 2},
  [281] = {.lex_state = 2},
  [282] = {.lex_state = 41, .external_lex_state = 4},
  [283] = {.lex_state = 41, .external_lex_state = 5},
  [284] = {.lex_state = 41, .external_lex_state = 5},
  [285] = {.lex_state = 41, .external_lex_state = 4},
  [286] = {.lex_state = 41, .external_lex_state = 4},
  [287] = {.lex_state = 41, .external_lex_state = 5},
  [288] = {.lex_state = 41, .external_lex_state = 4},
  [289] = {.lex_state = 41, .external_lex_state = 4},
  [290] = {.lex_state = 41, .external_lex_state = 4},
  [291] = {.lex_state = 41, .external_lex_state = 4},
  [292] = {.lex_state = 41, .external_lex_state = 4},
  [293] = {.lex_state = 41, .external_lex_state = 5},
  [294] = {.lex_state = 41, .external_lex_state = 4},
  [295] = {.lex_state = 41, .external_lex_state = 5},
  [296] = {.lex_state = 41, .external_lex_state = 4},
  [297] = {.lex_state = 41, .external_lex_state = 4},
  [298] = {.lex_state = 41, .external_lex_state = 4},
  [299] = {.lex_state = 41, .external_lex_state = 5},
  [300] = {.lex_state = 41, .external_lex_state = 4},
  [301] = {.lex_state = 41, .external_lex_state = 5},
  [302] = {.lex_state = 41, .external_lex_state = 4},
  [303] = {.lex_state = 41, .external_lex_state = 5},
  [304] = {.lex_state = 41, .external_lex_state = 4},
  [305] = {.lex_state = 41, .external_lex_state = 5},
  [306] = {.lex_state = 41, .external_lex_state = 4},
  [307] = {.lex_state = 41, .external_lex_state = 4},
  [308] = {.lex_state = 41, .external_lex_state = 4},
  [309] = {.lex_state = 41, .external_lex_state = 4},
  [310] = {.lex_state = 41, .external_lex_state = 4},
  [311] = {.lex_state = 41, .external_lex_state = 5},
  [312] = {.lex_state = 41, .external_lex_state = 4},
  [313] = {.lex_state = 41, .external_lex_state = 4},
  [314] = {.lex_state = 41, .external_lex_state = 4},
  [315] = {.lex_state = 41, .external_lex_state = 4},
  [316] = {.lex_state = 41, .external_lex_state = 5},
  [317] = {.lex_state = 41, .external_lex_state = 5},
  [318] = {.lex_state = 41, .external_lex_state = 5},
  [319] = {.lex_state = 41, .external_lex_state = 4},
  [320] = {.lex_state = 41, .external_lex_state = 4},
  [321] = {.lex_state = 41, .external_lex_state = 4},
  [322] = {.lex_state = 41, .external_lex_state = 5},
  [323] = {.lex_state = 41, .external_lex_state = 4},
  [324] = {.lex_state = 41, .external_lex_state = 5},
  [325] = {.lex_state = 41, .external_lex_state = 5},
  [326] = {.lex_state = 41, .external_lex_state = 4},
  [327] = {.lex_state = 41, .external_lex_state = 5},
  [328] = {.lex_state = 41, .external_lex_state = 4},
  [329] = {.lex_state = 41, .external_lex_state = 5},
  [330] = {.lex_state = 41, .external_lex_state = 5},
  [331] = {.lex_state = 41, .external_lex_state = 5},
  [332] = {.lex_state = 41, .external_lex_state = 5},
  [333] = {.lex_state = 41, .external_lex_state = 5},
  [334] = {.lex_state = 41, .external_lex_state = 5},
  [335] = {.lex_state = 41, .external_lex_state = 5},
  [336] = {.lex_state = 41, .external_lex_state = 5},
  [337] = {.lex_state = 41, .external_lex_state = 4},
  [338] = {.lex_state = 41, .external_lex_state = 5},
  [339] = {.lex_state = 41, .external_lex_state = 5},
  [340] = {.lex_state = 41, .external_lex_state = 5},
  [341] = {.lex_state = 41, .external_lex_state = 5},
  [342] = {.lex_state = 41, .external_lex_state = 4},
  [343] = {.lex_state = 41, .external_lex_state = 5},
  [344] = {.lex_state = 41, .external_lex_state = 5},
  [345] = {.lex_state = 41, .external_lex_state = 5},
  [346] = {.lex_state = 1, .external_lex_state = 2},
  [347] = {.lex_state = 1, .external_lex_state = 2},
  [348] = {.lex_state = 1, .external_lex_state = 2},
  [349] = {.lex_state = 1, .external_lex_state = 2},
  [350] = {.lex_state = 3, .external_lex_state = 2},
  [351] = {.lex_state = 3, .external_lex_state = 2},
  [352] = {.lex_state = 1, .external_lex_state = 2},
  [353] = {.lex_state = 0, .external_lex_state = 6},
  [354] = {.lex_state = 1, .external_lex_state = 2},
  [355] = {.lex_state = 0, .external_lex_state = 6},
  [356] = {.lex_state = 0, .external_lex_state = 6},
  [357] = {.lex_state = 0, .external_lex_state = 6},
  [358] = {.lex_state = 0, .external_lex_state = 6},
  [359] = {.lex_state = 1, .external_lex_state = 2},
  [360] = {.lex_state = 0, .external_lex_state = 6},
  [361] = {.lex_state = 0, .external_lex_state = 6},
  [362] = {.lex_state = 0, .external_lex_state = 6},
  [363] = {.lex_state = 0, .external_lex_state = 7},
  [364] = {.lex_state = 0, .external_lex_state = 8},
  [365] = {.lex_state = 0, .external_lex_state = 8},
  [366] = {.lex_state = 0, .external_lex_state = 8},
  [367] = {.lex_state = 0, .external_lex_state = 7},
  [368] = {.lex_state = 0, .external_lex_state = 6},
  [369] = {.lex_state = 0, .external_lex_state = 8},
  [370] = {.lex_state = 0, .external_lex_state = 6},
  [371] = {.lex_state = 0, .external_lex_state = 7},
  [372] = {.lex_state = 0, .external_lex_state = 8},
  [373] = {.lex_state = 0, .external_lex_state = 7},
  [374] = {.lex_state = 0, .external_lex_state = 8},
  [375] = {.lex_state = 0, .external_lex_state = 6},
  [376] = {.lex_state = 0, .external_lex_state = 6},
  [377] = {.lex_state = 0, .external_lex_state = 6},
  [378] = {.lex_state = 0, .external_lex_state = 7},
  [379] = {.lex_state = 0, .external_lex_state = 6},
  [380] = {.lex_state = 0, .external_lex_state = 7},
  [381] = {.lex_state = 0, .external_lex_state = 6},
  [382] = {.lex_state = 0, .external_lex_state = 8},
  [383] = {.lex_state = 0, .external_lex_state = 7},
  [384] = {.lex_state = 0, .external_lex_state = 8},
  [385] = {.lex_state = 0, .external_lex_state = 8},
  [386] = {.lex_state = 0, .external_lex_state = 7},
  [387] = {.lex_state = 0, .external_lex_state = 6},
  [388] = {.lex_state = 0, .external_lex_state = 6},
  [389] = {.lex_state = 0, .external_lex_state = 6},
  [390] = {.lex_state = 0, .external_lex_state = 7},
  [391] = {.lex_state = 0, .external_lex_state = 6},
  [392] = {.lex_state = 0, .external_lex_state = 6},
  [393] = {.lex_state = 0, .external_lex_state = 6},
  [394] = {.lex_state = 0, .external_lex_state = 6},
  [395] = {.lex_state = 0, .external_lex_state = 7},
  [396] = {.lex_state = 0, .external_lex_state = 8},
  [397] = {.lex_state = 0, .external_lex_state = 6},
  [398] = {.lex_state = 0, .external_lex_state = 8},
  [399] = {.lex_state = 0, .external_lex_state = 6},
  [400] = {.lex_state = 0, .external_lex_state = 7},
  [401] = {.lex_state = 0, .external_lex_state = 6},
  [402] = {.lex_state = 41},
  [403] = {.lex_state = 41},
  [404] = {.lex_state = 41},
  [405] = {.lex_state = 41},
  [406] = {.lex_state = 4, .external_lex_state = 2},
  [407] = {.lex_state = 0},
  [408] = {.lex_state = 0},
  [409] = {.lex_state = 0},
  [410] = {.lex_state = 0},
  [411] = {.lex_state = 0},
  [412] = {.lex_state = 41},
  [413] = {.lex_state = 41, .external_lex_state = 2},
  [414] = {.lex_state = 41},
  [415] = {.lex_state = 0},
  [416] = {.lex_state = 0},
  [417] = {.lex_state = 41, .external_lex_state = 2},
  [418] = {.lex_state = 0},
  [419] = {.lex_state = 0},
  [420] = {.lex_state = 0},
  [421] = {.lex_state = 0},
  [422] = {.lex_state = 0},
  [423] = {.lex_state = 0, .external_lex_state = 8},
  [424] = {.lex_state = 0},
  [425] = {.lex_state = 0, .external_lex_state = 7},
  [426] = {.lex_state = 0},
  [427] = {.lex_state = 0},
  [428] = {.lex_state = 0},
  [429] = {.lex_state = 0, .external_lex_state = 8},
  [430] = {.lex_state = 0},
  [431] = {.lex_state = 0},
  [432] = {.lex_state = 0},
  [433] = {.lex_state = 0},
  [434] = {.lex_state = 0, .external_lex_state = 7},
  [435] = {.lex_state = 0},
  [436] = {.lex_state = 0, .external_lex_state = 8},
  [437] = {.lex_state = 0, .external_lex_state = 9},
  [438] = {.lex_state = 0},
  [439] = {.lex_state = 0, .external_lex_state = 8},
  [440] = {.lex_state = 0, .external_lex_state = 8},
  [441] = {.lex_state = 0, .external_lex_state = 8},
  [442] = {.lex_state = 0},
  [443] = {.lex_state = 0, .external_lex_state = 8},
  [444] = {.lex_state = 0, .external_lex_state = 8},
  [445] = {.lex_state = 0, .external_lex_state = 6},
  [446] = {.lex_state = 0, .external_lex_state = 7},
  [447] = {.lex_state = 0, .external_lex_state = 8},
  [448] = {.lex_state = 0, .external_lex_state = 8},
  [449] = {.lex_state = 0},
  [450] = {.lex_state = 0},
  [451] = {.lex_state = 0, .external_lex_state = 8},
  [452] = {.lex_state = 0, .external_lex_state = 8},
  [453] = {.lex_state = 0},
  [454] = {.lex_state = 0},
  [455] = {.lex_state = 0, .external_lex_state = 8},
  [456] = {.lex_state = 0, .external_lex_state = 8},
  [457] = {.lex_state = 0},
  [458] = {.lex_state = 0, .external_lex_state = 8},
  [459] = {.lex_state = 0, .external_lex_state = 7},
  [460] = {.lex_state = 0, .external_lex_state = 7},
  [461] = {.lex_state = 0, .external_lex_state = 8},
  [462] = {.lex_state = 0},
  [463] = {.lex_state = 0, .external_lex_state = 7},
  [464] = {.lex_state = 0},
  [465] = {.lex_state = 0, .external_lex_state = 7},
  [466] = {.lex_state = 0, .external_lex_state = 7},
  [467] = {.lex_state = 0, .external_lex_state = 7},
  [468] = {.lex_state = 0, .external_lex_state = 8},
  [469] = {.lex_state = 0, .external_lex_state = 6},
  [470] = {.lex_state = 0, .external_lex_state = 7},
  [471] = {.lex_state = 0, .external_lex_state = 7},
  [472] = {.lex_state = 0, .external_lex_state = 7},
  [473] = {.lex_state = 0},
  [474] = {.lex_state = 0, .external_lex_state = 7},
  [475] = {.lex_state = 0, .external_lex_state = 7},
  [476] = {.lex_state = 0, .external_lex_state = 7},
  [477] = {.lex_state = 0, .external_lex_state = 7},
  [478] = {.lex_state = 0, .external_lex_state = 8},
  [479] = {.lex_state = 0, .external_lex_state = 8},
  [480] = {.lex_state = 0},
  [481] = {.lex_state = 41},
  [482] = {.lex_state = 0},
  [483] = {.lex_state = 0, .external_lex_state = 8},
  [484] = {.lex_state = 0, .external_lex_state = 8},
  [485] = {.lex_state = 0, .external_lex_state = 8},
  [486] = {.lex_state = 0},
  [487] = {.lex_state = 0, .external_lex_state = 8},
  [488] = {.lex_state = 0, .external_lex_state = 8},
  [489] = {.lex_state = 0},
  [490] = {.lex_state = 0, .external_lex_state = 7},
  [491] = {.lex_state = 0, .external_lex_state = 7},
  [492] = {.lex_state = 0, .external_lex_state = 7},
  [493] = {.lex_state = 0, .external_lex_state = 6},
  [494] = {.lex_state = 41},
  [495] = {.lex_state = 0},
  [496] = {.lex_state = 0},
  [497] = {.lex_state = 0, .external_lex_state = 5},
  [498] = {.lex_state = 0, .external_lex_state = 6},
  [499] = {.lex_state = 0, .external_lex_state = 6},
  [500] = {.lex_state = 0, .external_lex_state = 6},
  [501] = {.lex_state = 0, .external_lex_state = 6},
  [502] = {.lex_state = 41},
  [503] = {.lex_state = 0, .external_lex_state = 5},
  [504] = {.lex_state = 0, .external_lex_state = 6},
  [505] = {.lex_state = 0, .external_lex_state = 6},
  [506] = {.lex_state = 41},
  [507] = {.lex_state = 0, .external_lex_state = 6},
  [508] = {.lex_state = 0},
  [509] = {.lex_state = 0, .external_lex_state = 6},
  [510] = {.lex_state = 0, .external_lex_state = 6},
  [511] = {.lex_state = 0, .external_lex_state = 5},
  [512] = {.lex_state = 0, .external_lex_state = 6},
  [513] = {.lex_state = 0, .external_lex_state = 9},
  [514] = {.lex_state = 0, .external_lex_state = 5},
  [515] = {.lex_state = 0, .external_lex_state = 4},
  [516] = {.lex_state = 0, .external_lex_state = 4},
  [517] = {.lex_state = 0},
  [518] = {.lex_state = 0, .external_lex_state = 6},
  [519] = {.lex_state = 0, .external_lex_state = 6},
  [520] = {.lex_state = 0, .external_lex_state = 6},
  [521] = {.lex_state = 0, .external_lex_state = 5},
  [522] = {.lex_state = 41, .external_lex_state = 2},
  [523] = {.lex_state = 0, .external_lex_state = 6},
  [524] = {.lex_state = 0, .external_lex_state = 4},
  [525] = {.lex_state = 0, .external_lex_state = 9},
  [526] = {.lex_state = 0, .external_lex_state = 6},
  [527] = {.lex_state = 0},
  [528] = {.lex_state = 0, .external_lex_state = 6},
  [529] = {.lex_state = 0},
  [530] = {.lex_state = 41, .external_lex_state = 2},
  [531] = {.lex_state = 0, .external_lex_state = 4},
  [532] = {.lex_state = 0, .external_lex_state = 6},
  [533] = {.lex_state = 3},
  [534] = {.lex_state = 0, .external_lex_state = 5},
  [535] = {.lex_state = 0, .external_lex_state = 6},
  [536] = {.lex_state = 0, .external_lex_state = 4},
  [537] = {.lex_state = 0},
  [538] = {.lex_state = 0, .external_lex_state = 6},
  [539] = {.lex_state = 0, .external_lex_state = 6},
  [540] = {.lex_state = 0, .external_lex_state = 6},
  [541] = {.lex_state = 0},
  [542] = {.lex_state = 0, .external_lex_state = 6},
  [543] = {.lex_state = 0, .external_lex_state = 6},
  [544] = {.lex_state = 0, .external_lex_state = 6},
  [545] = {.lex_state = 0, .external_lex_state = 5},
  [546] = {.lex_state = 0, .external_lex_state = 4},
  [547] = {.lex_state = 0, .external_lex_state = 6},
  [548] = {.lex_state = 0, .external_lex_state = 5},
  [549] = {.lex_state = 0, .external_lex_state = 4},
  [550] = {.lex_state = 41},
  [551] = {.lex_state = 0, .external_lex_state = 6},
  [552] = {.lex_state = 0, .external_lex_state = 4},
  [553] = {.lex_state = 0, .external_lex_state = 6},
  [554] = {.lex_state = 0},
  [555] = {.lex_state = 0, .external_lex_state = 4},
  [556] = {.lex_state = 41},
  [557] = {.lex_state = 0, .external_lex_state = 4},
  [558] = {.lex_state = 0},
  [559] = {.lex_state = 0, .external_lex_state = 4},
  [560] = {.lex_state = 0},
  [561] = {.lex_state = 0},
  [562] = {.lex_state = 0},
  [563] = {.lex_state = 0},
  [564] = {.lex_state = 0},
  [565] = {.lex_state = 0},
  [566] = {.lex_state = 1},
  [567] = {.lex_state = 0},
  [568] = {.lex_state = 0, .external_lex_state = 4},
  [569] = {.lex_state = 0, .external_lex_state = 4},
  [570] = {.lex_state = 41},
  [571] = {.lex_state = 0},
  [572] = {.lex_state = 0},
  [573] = {.lex_state = 0},
  [574] = {.lex_state = 0, .external_lex_state = 4},
  [575] = {.lex_state = 0, .external_lex_state = 4},
  [576] = {.lex_state = 1},
  [577] = {.lex_state = 0, .external_lex_state = 4},
  [578] = {.lex_state = 0},
  [579] = {.lex_state = 0},
  [580] = {.lex_state = 0},
  [581] = {.lex_state = 0},
  [582] = {.lex_state = 1},
  [583] = {.lex_state = 0},
  [584] = {.lex_state = 0},
  [585] = {.lex_state = 0},
  [586] = {.lex_state = 0},
  [587] = {.lex_state = 0},
  [588] = {.lex_state = 0},
  [589] = {.lex_state = 0},
  [590] = {.lex_state = 0},
  [591] = {.lex_state = 0},
  [592] = {.lex_state = 41},
  [593] = {.lex_state = 1},
  [594] = {.lex_state = 0},
  [595] = {.lex_state = 0},
  [596] = {.lex_state = 1},
  [597] = {.lex_state = 0},
  [598] = {.lex_state = 0},
  [599] = {.lex_state = 0},
  [600] = {.lex_state = 0},
  [601] = {.lex_state = 0},
  [602] = {.lex_state = 0},
  [603] = {.lex_state = 0},
  [604] = {.lex_state = 0},
  [605] = {.lex_state = 0, .external_lex_state = 4},
  [606] = {.lex_state = 0, .external_lex_state = 4},
  [607] = {.lex_state = 0},
  [608] = {.lex_state = 0, .external_lex_state = 4},
  [609] = {.lex_state = 0},
  [610] = {.lex_state = 0},
  [611] = {.lex_state = 0},
  [612] = {.lex_state = 0, .external_lex_state = 4},
  [613] = {.lex_state = 0},
  [614] = {.lex_state = 0, .external_lex_state = 4},
  [615] = {.lex_state = 0},
  [616] = {.lex_state = 41},
  [617] = {.lex_state = 41},
  [618] = {.lex_state = 0},
  [619] = {.lex_state = 0},
  [620] = {.lex_state = 0},
  [621] = {.lex_state = 0, .external_lex_state = 4},
  [622] = {.lex_state = 0},
  [623] = {.lex_state = 0},
  [624] = {.lex_state = 0},
  [625] = {.lex_state = 0},
  [626] = {.lex_state = 41},
  [627] = {.lex_state = 0},
  [628] = {.lex_state = 0},
  [629] = {.lex_state = 0},
  [630] = {.lex_state = 0},
  [631] = {.lex_state = 0},
  [632] = {.lex_state = 41},
  [633] = {.lex_state = 0, .external_lex_state = 10},
  [634] = {.lex_state = 41},
  [635] = {.lex_state = 0, .external_lex_state = 4},
  [636] = {.lex_state = 0, .external_lex_state = 5},
  [637] = {.lex_state = 0, .external_lex_state = 4},
  [638] = {.lex_state = 0, .external_lex_state = 4},
  [639] = {.lex_state = 0, .external_lex_state = 4},
  [640] = {.lex_state = 0, .external_lex_state = 11},
  [641] = {.lex_state = 0, .external_lex_state = 4},
  [642] = {.lex_state = 0, .external_lex_state = 4},
  [643] = {.lex_state = 0, .external_lex_state = 5},
  [644] = {.lex_state = 0, .external_lex_state = 4},
  [645] = {.lex_state = 0, .external_lex_state = 4},
  [646] = {.lex_state = 0, .external_lex_state = 4},
  [647] = {.lex_state = 0, .external_lex_state = 4},
  [648] = {.lex_state = 0, .external_lex_state = 10},
  [649] = {.lex_state = 0, .external_lex_state = 5},
  [650] = {.lex_state = 0},
  [651] = {.lex_state = 0},
  [652] = {.lex_state = 41},
  [653] = {.lex_state = 0},
  [654] = {.lex_state = 0, .external_lex_state = 4},
  [655] = {.lex_state = 0, .external_lex_state = 4},
  [656] = {.lex_state = 0, .external_lex_state = 10},
  [657] = {.lex_state = 41},
  [658] = {.lex_state = 0, .external_lex_state = 10},
  [659] = {.lex_state = 0, .external_lex_state = 5},
  [660] = {.lex_state = 0, .external_lex_state = 4},
  [661] = {.lex_state = 0, .external_lex_state = 4},
  [662] = {.lex_state = 41},
  [663] = {.lex_state = 0},
  [664] = {.lex_state = 0, .external_lex_state = 4},
  [665] = {.lex_state = 0, .external_lex_state = 10},
  [666] = {.lex_state = 0, .external_lex_state = 4},
  [667] = {.lex_state = 0, .external_lex_state = 4},
  [668] = {.lex_state = 0, .external_lex_state = 4},
  [669] = {.lex_state = 0, .external_lex_state = 5},
  [670] = {.lex_state = 0, .external_lex_state = 10},
  [671] = {.lex_state = 0, .external_lex_state = 4},
  [672] = {.lex_state = 41},
  [673] = {.lex_state = 0, .external_lex_state = 5},
  [674] = {.lex_state = 0},
  [675] = {.lex_state = 0},
  [676] = {.lex_state = 0},
  [677] = {.lex_state = 0, .external_lex_state = 4},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [aux_sym_numeric_lit_token1] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [sym_null_lit] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [aux_sym_legacy_index_token1] = ACTIONS(1),
    [anon_sym_DOT_STAR] = ACTIONS(1),
    [anon_sym_LBRACK_STAR_RBRACK] = ACTIONS(1),
    [anon_sym_EQ_GT] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [sym_ellipsis] = ACTIONS(1),
    [anon_sym_QMARK] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_GT_EQ] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_AMP_AMP] = ACTIONS(1),
    [anon_sym_PIPE_PIPE] = ACTIONS(1),
    [anon_sym_LT_LT] = ACTIONS(1),
    [anon_sym_LT_LT_DASH] = ACTIONS(1),
    [sym_strip_marker] = ACTIONS(1),
    [anon_sym_endfor] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_endif] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [sym__whitespace] = ACTIONS(3),
    [sym_quoted_template_start] = ACTIONS(1),
    [sym_quoted_template_end] = ACTIONS(1),
    [sym__template_literal_chunk] = ACTIONS(1),
    [sym_template_interpolation_start] = ACTIONS(1),
    [sym_template_interpolation_end] = ACTIONS(1),
    [sym_template_directive_start] = ACTIONS(1),
    [sym_template_directive_end] = ACTIONS(1),
    [sym_heredoc_identifier] = ACTIONS(1),
  },
  [1] = {
    [sym_config_file] = STATE(676),
    [sym_body] = STATE(675),
    [sym_attribute] = STATE(414),
    [sym_block] = STATE(414),
    [sym_object] = STATE(675),
    [sym_object_start] = STATE(15),
    [aux_sym_body_repeat1] = STATE(414),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [sym_identifier] = ACTIONS(9),
    [sym_comment] = ACTIONS(3),
    [sym__whitespace] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 30,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      anon_sym_RBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(8), 1,
      sym_object_elem,
    STATE(12), 1,
      sym_tuple_start,
    STATE(58), 1,
      sym_for_intro,
    STATE(153), 1,
      sym_object_end,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(620), 1,
      sym__object_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [107] = 30,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(39), 1,
      anon_sym_RBRACE,
    STATE(4), 1,
      sym_object_start,
    STATE(8), 1,
      sym_object_elem,
    STATE(12), 1,
      sym_tuple_start,
    STATE(54), 1,
      sym_for_intro,
    STATE(255), 1,
      sym_object_end,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(581), 1,
      sym__object_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [214] = 30,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(41), 1,
      anon_sym_RBRACE,
    STATE(4), 1,
      sym_object_start,
    STATE(8), 1,
      sym_object_elem,
    STATE(12), 1,
      sym_tuple_start,
    STATE(78), 1,
      sym_for_intro,
    STATE(194), 1,
      sym_object_end,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(585), 1,
      sym__object_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [321] = 30,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(43), 1,
      anon_sym_RBRACE,
    STATE(4), 1,
      sym_object_start,
    STATE(8), 1,
      sym_object_elem,
    STATE(12), 1,
      sym_tuple_start,
    STATE(72), 1,
      sym_for_intro,
    STATE(298), 1,
      sym_object_end,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(622), 1,
      sym__object_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [428] = 30,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    STATE(4), 1,
      sym_object_start,
    STATE(8), 1,
      sym_object_elem,
    STATE(12), 1,
      sym_tuple_start,
    STATE(63), 1,
      sym_for_intro,
    STATE(325), 1,
      sym_object_end,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(565), 1,
      sym__object_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [535] = 28,
    ACTIONS(47), 1,
      anon_sym_LBRACE,
    ACTIONS(50), 1,
      anon_sym_RBRACE,
    ACTIONS(52), 1,
      sym_identifier,
    ACTIONS(55), 1,
      anon_sym_LPAREN,
    ACTIONS(58), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(61), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(67), 1,
      sym_null_lit,
    ACTIONS(70), 1,
      anon_sym_COMMA,
    ACTIONS(73), 1,
      anon_sym_LBRACK,
    ACTIONS(79), 1,
      anon_sym_LT_LT,
    ACTIONS(82), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(85), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(40), 1,
      sym__comma,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(64), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(76), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(7), 2,
      sym_object_elem,
      aux_sym__object_elems_repeat1,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [637] = 28,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(88), 1,
      anon_sym_RBRACE,
    ACTIONS(90), 1,
      anon_sym_COMMA,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(22), 1,
      sym__comma,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(11), 2,
      sym_object_elem,
      aux_sym__object_elems_repeat1,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [739] = 29,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(92), 1,
      anon_sym_RBRACK,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(52), 1,
      sym_for_intro,
    STATE(264), 1,
      sym_tuple_end,
    STATE(403), 1,
      sym_conditional,
    STATE(431), 1,
      sym_expression,
    STATE(560), 1,
      sym__tuple_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [843] = 29,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(94), 1,
      anon_sym_RBRACK,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(50), 1,
      sym_for_intro,
    STATE(286), 1,
      sym_tuple_end,
    STATE(403), 1,
      sym_conditional,
    STATE(431), 1,
      sym_expression,
    STATE(602), 1,
      sym__tuple_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [947] = 28,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(96), 1,
      anon_sym_RBRACE,
    ACTIONS(98), 1,
      anon_sym_COMMA,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(28), 1,
      sym__comma,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(7), 2,
      sym_object_elem,
      aux_sym__object_elems_repeat1,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1049] = 29,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(100), 1,
      anon_sym_RBRACK,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(77), 1,
      sym_for_intro,
    STATE(213), 1,
      sym_tuple_end,
    STATE(403), 1,
      sym_conditional,
    STATE(431), 1,
      sym_expression,
    STATE(567), 1,
      sym__tuple_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1153] = 29,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(102), 1,
      anon_sym_RBRACK,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(57), 1,
      sym_for_intro,
    STATE(283), 1,
      sym_tuple_end,
    STATE(403), 1,
      sym_conditional,
    STATE(431), 1,
      sym_expression,
    STATE(578), 1,
      sym__tuple_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1257] = 29,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(29), 1,
      anon_sym_for,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(104), 1,
      anon_sym_RBRACK,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(55), 1,
      sym_for_intro,
    STATE(165), 1,
      sym_tuple_end,
    STATE(403), 1,
      sym_conditional,
    STATE(431), 1,
      sym_expression,
    STATE(595), 1,
      sym__tuple_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1361] = 28,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(41), 1,
      anon_sym_RBRACE,
    STATE(4), 1,
      sym_object_start,
    STATE(8), 1,
      sym_object_elem,
    STATE(12), 1,
      sym_tuple_start,
    STATE(194), 1,
      sym_object_end,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(585), 1,
      sym__object_elems,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1462] = 27,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(106), 1,
      anon_sym_RPAREN,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(297), 1,
      sym__function_call_end,
    STATE(403), 1,
      sym_conditional,
    STATE(407), 1,
      sym_expression,
    STATE(597), 1,
      sym_function_arguments,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1560] = 27,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(108), 1,
      anon_sym_RPAREN,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(143), 1,
      sym__function_call_end,
    STATE(403), 1,
      sym_conditional,
    STATE(407), 1,
      sym_expression,
    STATE(599), 1,
      sym_function_arguments,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1658] = 27,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(110), 1,
      anon_sym_RPAREN,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(191), 1,
      sym__function_call_end,
    STATE(403), 1,
      sym_conditional,
    STATE(407), 1,
      sym_expression,
    STATE(554), 1,
      sym_function_arguments,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1756] = 27,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(112), 1,
      anon_sym_RPAREN,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(341), 1,
      sym__function_call_end,
    STATE(403), 1,
      sym_conditional,
    STATE(407), 1,
      sym_expression,
    STATE(603), 1,
      sym_function_arguments,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1854] = 27,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(114), 1,
      anon_sym_RPAREN,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(267), 1,
      sym__function_call_end,
    STATE(403), 1,
      sym_conditional,
    STATE(407), 1,
      sym_expression,
    STATE(580), 1,
      sym_function_arguments,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [1952] = 26,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(130), 1,
      sym_strip_marker,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    ACTIONS(134), 1,
      sym_template_interpolation_end,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(534), 1,
      sym_expression,
    STATE(548), 1,
      sym_conditional,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(219), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2047] = 26,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(96), 1,
      anon_sym_RBRACE,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(349), 1,
      sym_object_elem,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2142] = 26,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    ACTIONS(136), 1,
      sym_strip_marker,
    ACTIONS(138), 1,
      sym_template_interpolation_end,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(497), 1,
      sym_expression,
    STATE(548), 1,
      sym_conditional,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(219), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2237] = 26,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    ACTIONS(140), 1,
      sym_strip_marker,
    ACTIONS(142), 1,
      sym_template_interpolation_end,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(545), 1,
      sym_expression,
    STATE(548), 1,
      sym_conditional,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(219), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2332] = 26,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    ACTIONS(144), 1,
      sym_strip_marker,
    ACTIONS(146), 1,
      sym_template_interpolation_end,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(503), 1,
      sym_expression,
    STATE(548), 1,
      sym_conditional,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(219), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2427] = 26,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    ACTIONS(148), 1,
      sym_strip_marker,
    ACTIONS(150), 1,
      sym_template_interpolation_end,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(521), 1,
      sym_expression,
    STATE(548), 1,
      sym_conditional,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(219), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2522] = 26,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    ACTIONS(152), 1,
      sym_strip_marker,
    ACTIONS(154), 1,
      sym_template_interpolation_end,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(514), 1,
      sym_expression,
    STATE(548), 1,
      sym_conditional,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(219), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2617] = 26,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(156), 1,
      anon_sym_RBRACE,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(349), 1,
      sym_object_elem,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2712] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(170), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(169), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(160), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(158), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [2766] = 25,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(162), 1,
      anon_sym_RPAREN,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(420), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2858] = 25,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(164), 1,
      anon_sym_RPAREN,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(420), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [2950] = 8,
    ACTIONS(172), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(170), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(170), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(169), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 11,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(166), 18,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [3008] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(170), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(169), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(166), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [3062] = 9,
    ACTIONS(172), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(170), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(174), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(170), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(169), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 11,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(166), 16,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [3122] = 11,
    ACTIONS(172), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(170), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(174), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(176), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(178), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(170), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(169), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 9,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_LT_LT,
    ACTIONS(166), 14,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [3186] = 12,
    ACTIONS(172), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(170), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(174), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(176), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(178), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(180), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(170), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(169), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 9,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_LT_LT,
    ACTIONS(166), 12,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [3252] = 18,
    ACTIONS(172), 1,
      anon_sym_SLASH,
    ACTIONS(186), 1,
      anon_sym_LBRACK,
    ACTIONS(188), 1,
      anon_sym_DOT,
    ACTIONS(190), 1,
      anon_sym_DOT_STAR,
    ACTIONS(192), 1,
      anon_sym_LBRACK_STAR_RBRACK,
    ACTIONS(194), 1,
      anon_sym_AMP_AMP,
    ACTIONS(196), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(170), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(174), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(176), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(178), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(180), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(170), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(169), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(184), 7,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_BANG,
      anon_sym_LT_LT,
    ACTIONS(182), 8,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_QMARK,
      anon_sym_LT_LT_DASH,
  [3330] = 13,
    ACTIONS(172), 1,
      anon_sym_SLASH,
    ACTIONS(194), 1,
      anon_sym_AMP_AMP,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(170), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(174), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(176), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(178), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(180), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(170), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(169), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 9,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_LT_LT,
    ACTIONS(166), 11,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [3398] = 25,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(198), 1,
      anon_sym_RBRACK,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(420), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [3490] = 25,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(349), 1,
      sym_object_elem,
    STATE(403), 1,
      sym_conditional,
    STATE(533), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [3582] = 25,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    ACTIONS(200), 1,
      anon_sym_RBRACK,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(420), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [3674] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(561), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [3763] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(613), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [3852] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(402), 1,
      sym_expression,
    STATE(403), 1,
      sym_conditional,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [3941] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(563), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4030] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(515), 1,
      sym_expression,
    STATE(549), 1,
      sym_conditional,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(222), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4119] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(511), 1,
      sym_expression,
    STATE(548), 1,
      sym_conditional,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(219), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4208] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(589), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4297] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(416), 1,
      sym_expression,
    STATE(426), 1,
      sym_conditional,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4386] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(419), 1,
      sym_expression,
    STATE(426), 1,
      sym_conditional,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4475] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(408), 1,
      sym_expression,
    STATE(426), 1,
      sym_conditional,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4564] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(422), 1,
      sym_expression,
    STATE(426), 1,
      sym_conditional,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4653] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(415), 1,
      sym_expression,
    STATE(426), 1,
      sym_conditional,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4742] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(576), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4831] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(426), 1,
      sym_conditional,
    STATE(427), 1,
      sym_expression,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [4920] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(409), 1,
      sym_expression,
    STATE(426), 1,
      sym_conditional,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5009] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(424), 1,
      sym_expression,
    STATE(426), 1,
      sym_conditional,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5098] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(582), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5187] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(234), 1,
      sym_identifier,
    ACTIONS(236), 1,
      anon_sym_LPAREN,
    ACTIONS(238), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(240), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(244), 1,
      sym_null_lit,
    ACTIONS(248), 1,
      sym_quoted_template_start,
    STATE(2), 1,
      sym_object_start,
    STATE(14), 1,
      sym_tuple_start,
    STATE(347), 1,
      sym_expression,
    STATE(348), 1,
      sym_conditional,
    STATE(670), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(242), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(246), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(152), 2,
      sym_tuple,
      sym_object,
    STATE(156), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(158), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(161), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(151), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(37), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5276] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(624), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5365] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(631), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5454] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(524), 1,
      sym_expression,
    STATE(549), 1,
      sym_conditional,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(222), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5543] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(593), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5632] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(527), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5721] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(410), 1,
      sym_expression,
    STATE(426), 1,
      sym_conditional,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5810] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(611), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5899] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(601), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [5988] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(564), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6077] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(481), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6166] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(420), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6255] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(546), 1,
      sym_expression,
    STATE(549), 1,
      sym_conditional,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(222), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6344] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(566), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6433] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(426), 1,
      sym_conditional,
    STATE(432), 1,
      sym_expression,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6522] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(531), 1,
      sym_expression,
    STATE(549), 1,
      sym_conditional,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(222), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6611] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(572), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6700] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(234), 1,
      sym_identifier,
    ACTIONS(236), 1,
      anon_sym_LPAREN,
    ACTIONS(238), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(240), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(244), 1,
      sym_null_lit,
    ACTIONS(248), 1,
      sym_quoted_template_start,
    STATE(2), 1,
      sym_object_start,
    STATE(14), 1,
      sym_tuple_start,
    STATE(346), 1,
      sym_expression,
    STATE(348), 1,
      sym_conditional,
    STATE(670), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(242), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(246), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(152), 2,
      sym_tuple,
      sym_object,
    STATE(156), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(158), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(161), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(151), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(37), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6789] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(426), 1,
      sym_conditional,
    STATE(433), 1,
      sym_expression,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(179), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6878] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(596), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [6967] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(536), 1,
      sym_expression,
    STATE(549), 1,
      sym_conditional,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(222), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7056] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(591), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7145] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(600), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7234] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(516), 1,
      sym_expression,
    STATE(549), 1,
      sym_conditional,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(222), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7323] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(558), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7412] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(618), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7501] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(630), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7590] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(549), 1,
      sym_conditional,
    STATE(552), 1,
      sym_expression,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(222), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7679] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(629), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7768] = 24,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(403), 1,
      sym_conditional,
    STATE(625), 1,
      sym_expression,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(129), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [7857] = 7,
    ACTIONS(186), 1,
      anon_sym_LBRACK,
    ACTIONS(188), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(90), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(252), 10,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(250), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [7911] = 7,
    ACTIONS(186), 1,
      anon_sym_LBRACK,
    ACTIONS(188), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(93), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(256), 10,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(254), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [7965] = 7,
    ACTIONS(186), 1,
      anon_sym_LBRACK,
    ACTIONS(188), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(92), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(260), 10,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(258), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [8019] = 7,
    ACTIONS(186), 1,
      anon_sym_LBRACK,
    ACTIONS(188), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(93), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(264), 10,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(262), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [8073] = 7,
    ACTIONS(270), 1,
      anon_sym_LBRACK,
    ACTIONS(273), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(160), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(93), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(268), 10,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(266), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [8127] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(131), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8210] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(234), 1,
      sym_identifier,
    ACTIONS(236), 1,
      anon_sym_LPAREN,
    ACTIONS(238), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(240), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(244), 1,
      sym_null_lit,
    ACTIONS(248), 1,
      sym_quoted_template_start,
    STATE(2), 1,
      sym_object_start,
    STATE(14), 1,
      sym_tuple_start,
    STATE(670), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(242), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(246), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(152), 2,
      sym_tuple,
      sym_object,
    STATE(156), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(158), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(161), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(151), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(34), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8293] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(225), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8376] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(228), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8459] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(217), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8542] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(218), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8625] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(236), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8708] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(230), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8791] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(226), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8874] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(234), 1,
      sym_identifier,
    ACTIONS(236), 1,
      anon_sym_LPAREN,
    ACTIONS(238), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(240), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(244), 1,
      sym_null_lit,
    ACTIONS(248), 1,
      sym_quoted_template_start,
    STATE(2), 1,
      sym_object_start,
    STATE(14), 1,
      sym_tuple_start,
    STATE(670), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(242), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(246), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(152), 2,
      sym_tuple,
      sym_object,
    STATE(156), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(158), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(161), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(151), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(38), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [8957] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(232), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9040] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(233), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9123] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(234), 1,
      sym_identifier,
    ACTIONS(236), 1,
      anon_sym_LPAREN,
    ACTIONS(238), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(240), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(244), 1,
      sym_null_lit,
    ACTIONS(248), 1,
      sym_quoted_template_start,
    STATE(2), 1,
      sym_object_start,
    STATE(14), 1,
      sym_tuple_start,
    STATE(670), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(242), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(246), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(152), 2,
      sym_tuple,
      sym_object,
    STATE(156), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(158), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(161), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(151), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(36), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9206] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(220), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9289] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(216), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9372] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(234), 1,
      sym_identifier,
    ACTIONS(236), 1,
      anon_sym_LPAREN,
    ACTIONS(238), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(240), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(244), 1,
      sym_null_lit,
    ACTIONS(248), 1,
      sym_quoted_template_start,
    STATE(2), 1,
      sym_object_start,
    STATE(14), 1,
      sym_tuple_start,
    STATE(670), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(242), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(246), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(152), 2,
      sym_tuple,
      sym_object,
    STATE(156), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(158), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(161), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(151), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(35), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9455] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(136), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9538] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(181), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9621] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(183), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9704] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(178), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9787] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(182), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9870] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(180), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [9953] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(177), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10036] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(218), 1,
      sym_identifier,
    ACTIONS(220), 1,
      anon_sym_LPAREN,
    ACTIONS(222), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(224), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(228), 1,
      sym_null_lit,
    ACTIONS(232), 1,
      sym_quoted_template_start,
    STATE(3), 1,
      sym_object_start,
    STATE(9), 1,
      sym_tuple_start,
    STATE(665), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(226), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(230), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(254), 2,
      sym_tuple,
      sym_object,
    STATE(256), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(257), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(258), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(253), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(176), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10119] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(135), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10202] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(202), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_LPAREN,
    ACTIONS(206), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(208), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(212), 1,
      sym_null_lit,
    ACTIONS(216), 1,
      sym_quoted_template_start,
    STATE(5), 1,
      sym_object_start,
    STATE(10), 1,
      sym_tuple_start,
    STATE(656), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(210), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(214), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(292), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(294), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(296), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(300), 2,
      sym_tuple,
      sym_object,
    STATE(304), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(224), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10285] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(234), 1,
      sym_identifier,
    ACTIONS(236), 1,
      anon_sym_LPAREN,
    ACTIONS(238), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(240), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(244), 1,
      sym_null_lit,
    ACTIONS(248), 1,
      sym_quoted_template_start,
    STATE(2), 1,
      sym_object_start,
    STATE(14), 1,
      sym_tuple_start,
    STATE(670), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(242), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(246), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(152), 2,
      sym_tuple,
      sym_object,
    STATE(156), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(158), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(161), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(151), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(33), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10368] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(234), 1,
      sym_identifier,
    ACTIONS(236), 1,
      anon_sym_LPAREN,
    ACTIONS(238), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(240), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(244), 1,
      sym_null_lit,
    ACTIONS(248), 1,
      sym_quoted_template_start,
    STATE(2), 1,
      sym_object_start,
    STATE(14), 1,
      sym_tuple_start,
    STATE(670), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(242), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(246), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(152), 2,
      sym_tuple,
      sym_object,
    STATE(156), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(158), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(161), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(151), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(32), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10451] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(134), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10534] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(229), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10617] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(116), 1,
      sym_identifier,
    ACTIONS(118), 1,
      anon_sym_LPAREN,
    ACTIONS(120), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(122), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(126), 1,
      sym_null_lit,
    ACTIONS(132), 1,
      sym_quoted_template_start,
    STATE(6), 1,
      sym_object_start,
    STATE(13), 1,
      sym_tuple_start,
    STATE(633), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(124), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(128), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(327), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(329), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(330), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(332), 2,
      sym_tuple,
      sym_object,
    STATE(333), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(227), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10700] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(234), 1,
      sym_identifier,
    ACTIONS(236), 1,
      anon_sym_LPAREN,
    ACTIONS(238), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(240), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(244), 1,
      sym_null_lit,
    ACTIONS(248), 1,
      sym_quoted_template_start,
    STATE(2), 1,
      sym_object_start,
    STATE(14), 1,
      sym_tuple_start,
    STATE(670), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(242), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(246), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(152), 2,
      sym_tuple,
      sym_object,
    STATE(156), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(158), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(161), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(151), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(29), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10783] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(130), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10866] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(132), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [10949] = 22,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      sym_identifier,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    ACTIONS(19), 1,
      aux_sym_numeric_lit_token1,
    ACTIONS(21), 1,
      aux_sym_numeric_lit_token2,
    ACTIONS(25), 1,
      sym_null_lit,
    ACTIONS(27), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_LT_LT,
    ACTIONS(35), 1,
      anon_sym_LT_LT_DASH,
    ACTIONS(37), 1,
      sym_quoted_template_start,
    STATE(4), 1,
      sym_object_start,
    STATE(12), 1,
      sym_tuple_start,
    STATE(648), 1,
      sym_heredoc_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(31), 2,
      anon_sym_DASH,
      anon_sym_BANG,
    STATE(193), 2,
      sym_tuple,
      sym_object,
    STATE(196), 2,
      sym_for_tuple_expr,
      sym_for_object_expr,
    STATE(197), 2,
      sym_unary_operation,
      sym_binary_operation,
    STATE(201), 2,
      sym_quoted_template,
      sym_heredoc_template,
    STATE(192), 3,
      sym_numeric_lit,
      sym_bool_lit,
      sym_string_lit,
    STATE(133), 8,
      sym__expr_term,
      sym_literal_value,
      sym_collection_value,
      sym_for_expr,
      sym_variable_expr,
      sym_function_call,
      sym_operation,
      sym_template_expr,
  [11032] = 18,
    ACTIONS(184), 1,
      anon_sym_EQ,
    ACTIONS(276), 1,
      anon_sym_LBRACK,
    ACTIONS(278), 1,
      anon_sym_DOT,
    ACTIONS(280), 1,
      anon_sym_DOT_STAR,
    ACTIONS(282), 1,
      anon_sym_LBRACK_STAR_RBRACK,
    ACTIONS(288), 1,
      anon_sym_SLASH,
    ACTIONS(296), 1,
      anon_sym_AMP_AMP,
    ACTIONS(298), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(284), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(286), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(290), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(292), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(294), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(199), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(204), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(182), 10,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
  [11106] = 13,
    ACTIONS(288), 1,
      anon_sym_SLASH,
    ACTIONS(296), 1,
      anon_sym_AMP_AMP,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(284), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(286), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(290), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(292), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(294), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(199), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(168), 3,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
    STATE(204), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 13,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_PIPE_PIPE,
  [11170] = 12,
    ACTIONS(288), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(284), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(286), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(290), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(292), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(294), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(199), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(168), 3,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
    STATE(204), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 14,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11232] = 11,
    ACTIONS(288), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(284), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(286), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(290), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(292), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    STATE(199), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(168), 3,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
    STATE(204), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 16,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11292] = 9,
    ACTIONS(288), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(284), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(286), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    STATE(199), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(204), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 5,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 18,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11348] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(199), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(204), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11398] = 8,
    ACTIONS(288), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(286), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    STATE(199), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(204), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 5,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 20,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11452] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(199), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(204), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(160), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(158), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11502] = 7,
    ACTIONS(276), 1,
      anon_sym_LBRACK,
    ACTIONS(278), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(138), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(252), 4,
      anon_sym_EQ,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(250), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11552] = 7,
    ACTIONS(276), 1,
      anon_sym_LBRACK,
    ACTIONS(278), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(142), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(256), 4,
      anon_sym_EQ,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(254), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11602] = 7,
    ACTIONS(276), 1,
      anon_sym_LBRACK,
    ACTIONS(278), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(140), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(260), 4,
      anon_sym_EQ,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(258), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11652] = 7,
    ACTIONS(276), 1,
      anon_sym_LBRACK,
    ACTIONS(278), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(142), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(264), 4,
      anon_sym_EQ,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(262), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11702] = 5,
    ACTIONS(304), 1,
      anon_sym_LPAREN,
    STATE(17), 1,
      sym__function_call_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(302), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(300), 19,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [11748] = 7,
    ACTIONS(306), 1,
      anon_sym_LBRACK,
    ACTIONS(309), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(211), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(142), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(268), 4,
      anon_sym_EQ,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(266), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [11798] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(314), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(312), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [11839] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(318), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(316), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [11880] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(322), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(320), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [11921] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(326), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(324), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [11962] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(330), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(328), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12003] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(334), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(332), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12044] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(338), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(336), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12085] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(342), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(340), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12126] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(346), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(344), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12167] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(350), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(348), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12208] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(354), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(352), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12249] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(358), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(356), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12290] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(362), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(360), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12331] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(366), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(364), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12372] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(370), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(368), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12413] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(374), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(372), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12454] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(378), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(376), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12495] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(382), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(380), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12536] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(386), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(384), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12577] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(390), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(388), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12618] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(394), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(392), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12659] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(398), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(396), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12700] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(402), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(400), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12741] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(406), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(404), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12782] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(410), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(408), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12823] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(414), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(412), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12864] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(418), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(416), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12905] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(422), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(420), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12946] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(426), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(424), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [12987] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(430), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(428), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [13028] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(434), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(432), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [13069] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(438), 12,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_BANG,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
      anon_sym_LT_LT,
    ACTIONS(436), 20,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_LT_LT_DASH,
  [13110] = 5,
    ACTIONS(440), 1,
      anon_sym_LPAREN,
    STATE(18), 1,
      sym__function_call_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(302), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(300), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13153] = 13,
    ACTIONS(446), 1,
      anon_sym_SLASH,
    ACTIONS(454), 1,
      anon_sym_AMP_AMP,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(168), 2,
      anon_sym_LBRACK,
      anon_sym_DOT,
    ACTIONS(442), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(444), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(448), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(450), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(452), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(262), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(260), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 8,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_PIPE_PIPE,
  [13211] = 12,
    ACTIONS(446), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(168), 2,
      anon_sym_LBRACK,
      anon_sym_DOT,
    ACTIONS(442), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(444), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(448), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(450), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(452), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(262), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(260), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 9,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13267] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(262), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(260), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13311] = 17,
    ACTIONS(446), 1,
      anon_sym_SLASH,
    ACTIONS(454), 1,
      anon_sym_AMP_AMP,
    ACTIONS(456), 1,
      anon_sym_LBRACK,
    ACTIONS(458), 1,
      anon_sym_DOT,
    ACTIONS(460), 1,
      anon_sym_DOT_STAR,
    ACTIONS(462), 1,
      anon_sym_LBRACK_STAR_RBRACK,
    ACTIONS(464), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(442), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(444), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(448), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(450), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(452), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(262), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(260), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(182), 5,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
  [13377] = 11,
    ACTIONS(446), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(168), 2,
      anon_sym_LBRACK,
      anon_sym_DOT,
    ACTIONS(442), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(444), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(448), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(450), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(262), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(260), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 11,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13431] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(262), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(260), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(160), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(158), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13475] = 9,
    ACTIONS(446), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(442), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(444), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(262), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(260), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 4,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 13,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13525] = 8,
    ACTIONS(446), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(444), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(262), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(260), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 4,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 15,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13573] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(318), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(316), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13610] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(342), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(340), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13647] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(330), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(328), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13684] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(334), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(332), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13721] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(358), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(356), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13758] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(322), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(320), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13795] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(438), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(436), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13832] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(314), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(312), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13869] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(346), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(344), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13906] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(350), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(348), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13943] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(354), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(352), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [13980] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(378), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(376), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14017] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(366), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(364), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14054] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(374), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(372), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14091] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(338), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(336), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14128] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(422), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(420), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14165] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(434), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(432), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14202] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(386), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(384), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14239] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(398), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(396), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14276] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(362), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(360), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14313] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(418), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(416), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14350] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(410), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(408), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14387] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(390), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(388), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14424] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(430), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(428), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14461] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(406), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(404), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14498] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(426), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(424), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14535] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(326), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(324), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14572] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(382), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(380), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14609] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(370), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(368), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14646] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(402), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(400), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14683] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(394), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(392), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14720] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(414), 6,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(412), 22,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [14757] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(288), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(290), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(160), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(158), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [14799] = 9,
    ACTIONS(470), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(466), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(468), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    STATE(288), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(290), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 4,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 11,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [14847] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(288), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(290), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [14889] = 17,
    ACTIONS(472), 1,
      anon_sym_LBRACK,
    ACTIONS(474), 1,
      anon_sym_DOT,
    ACTIONS(476), 1,
      anon_sym_DOT_STAR,
    ACTIONS(478), 1,
      anon_sym_LBRACK_STAR_RBRACK,
    ACTIONS(484), 1,
      anon_sym_SLASH,
    ACTIONS(492), 1,
      anon_sym_AMP_AMP,
    ACTIONS(494), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(480), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(482), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(486), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(488), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(490), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(338), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(182), 3,
      sym_template_interpolation_end,
      anon_sym_QMARK,
      sym_strip_marker,
    STATE(340), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
  [14953] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(338), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(340), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(160), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(158), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [14995] = 7,
    ACTIONS(496), 1,
      anon_sym_LBRACK,
    ACTIONS(499), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(268), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(221), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(266), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [15039] = 17,
    ACTIONS(470), 1,
      anon_sym_SLASH,
    ACTIONS(502), 1,
      anon_sym_LBRACK,
    ACTIONS(504), 1,
      anon_sym_DOT,
    ACTIONS(506), 1,
      anon_sym_DOT_STAR,
    ACTIONS(508), 1,
      anon_sym_LBRACK_STAR_RBRACK,
    ACTIONS(516), 1,
      anon_sym_AMP_AMP,
    ACTIONS(518), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(466), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(468), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(510), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(512), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(514), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(288), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(182), 3,
      sym_template_directive_end,
      anon_sym_QMARK,
      sym_strip_marker,
    STATE(290), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
  [15103] = 7,
    ACTIONS(456), 1,
      anon_sym_LBRACK,
    ACTIONS(458), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(260), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(234), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(258), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [15147] = 13,
    ACTIONS(470), 1,
      anon_sym_SLASH,
    ACTIONS(516), 1,
      anon_sym_AMP_AMP,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(168), 2,
      anon_sym_LBRACK,
      anon_sym_DOT,
    ACTIONS(466), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(468), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(510), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(512), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(514), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(288), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(290), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 6,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15203] = 12,
    ACTIONS(470), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(168), 2,
      anon_sym_LBRACK,
      anon_sym_DOT,
    ACTIONS(466), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(468), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(510), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(512), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(514), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(288), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(290), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 7,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15257] = 13,
    ACTIONS(484), 1,
      anon_sym_SLASH,
    ACTIONS(492), 1,
      anon_sym_AMP_AMP,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(168), 2,
      anon_sym_LBRACK,
      anon_sym_DOT,
    ACTIONS(480), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(482), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(486), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(488), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(490), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(338), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(340), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 6,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15313] = 12,
    ACTIONS(484), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(168), 2,
      anon_sym_LBRACK,
      anon_sym_DOT,
    ACTIONS(480), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(482), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(486), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(488), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    ACTIONS(490), 2,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    STATE(338), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(340), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 7,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15367] = 11,
    ACTIONS(470), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(168), 2,
      anon_sym_LBRACK,
      anon_sym_DOT,
    ACTIONS(466), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(468), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(510), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(512), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    STATE(288), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(290), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 9,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15419] = 11,
    ACTIONS(484), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(168), 2,
      anon_sym_LBRACK,
      anon_sym_DOT,
    ACTIONS(480), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(482), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(486), 2,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(488), 2,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
    STATE(338), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(340), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(166), 9,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15471] = 9,
    ACTIONS(484), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(480), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(482), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    STATE(338), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(340), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 4,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 11,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15519] = 7,
    ACTIONS(456), 1,
      anon_sym_LBRACK,
    ACTIONS(458), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(252), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(235), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(250), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [15563] = 6,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(338), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(340), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15605] = 8,
    ACTIONS(484), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(482), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    STATE(338), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(340), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 4,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 13,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15651] = 7,
    ACTIONS(456), 1,
      anon_sym_LBRACK,
    ACTIONS(458), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(264), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(221), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(262), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [15695] = 7,
    ACTIONS(456), 1,
      anon_sym_LBRACK,
    ACTIONS(458), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(261), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(256), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(221), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(254), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [15739] = 8,
    ACTIONS(470), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(468), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    STATE(288), 2,
      sym_attr_splat,
      sym_full_splat,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    STATE(290), 3,
      sym_index,
      sym_get_attr,
      sym_splat,
    ACTIONS(168), 4,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(166), 13,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15785] = 7,
    ACTIONS(472), 1,
      anon_sym_LBRACK,
    ACTIONS(474), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(264), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(241), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(262), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15827] = 7,
    ACTIONS(472), 1,
      anon_sym_LBRACK,
    ACTIONS(474), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(256), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(241), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(254), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15869] = 7,
    ACTIONS(502), 1,
      anon_sym_LBRACK,
    ACTIONS(504), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(260), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(242), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(258), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15911] = 7,
    ACTIONS(520), 1,
      anon_sym_LBRACK,
    ACTIONS(523), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(268), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(240), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(266), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15953] = 7,
    ACTIONS(526), 1,
      anon_sym_LBRACK,
    ACTIONS(529), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(268), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(241), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(266), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [15995] = 7,
    ACTIONS(502), 1,
      anon_sym_LBRACK,
    ACTIONS(504), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(264), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(240), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(262), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [16037] = 7,
    ACTIONS(472), 1,
      anon_sym_LBRACK,
    ACTIONS(474), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(252), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(238), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(250), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [16079] = 7,
    ACTIONS(502), 1,
      anon_sym_LBRACK,
    ACTIONS(504), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(252), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(246), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(250), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [16121] = 7,
    ACTIONS(472), 1,
      anon_sym_LBRACK,
    ACTIONS(474), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(339), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(260), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(237), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(258), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [16163] = 7,
    ACTIONS(502), 1,
      anon_sym_LBRACK,
    ACTIONS(504), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(289), 2,
      sym_new_index,
      sym_legacy_index,
    ACTIONS(256), 3,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    STATE(240), 3,
      sym_index,
      sym_get_attr,
      aux_sym_attr_splat_repeat1,
    ACTIONS(254), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [16205] = 5,
    ACTIONS(532), 1,
      anon_sym_LPAREN,
    STATE(20), 1,
      sym__function_call_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(302), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(300), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16242] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(394), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(392), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16273] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(334), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(332), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16304] = 5,
    ACTIONS(534), 1,
      anon_sym_LPAREN,
    STATE(16), 1,
      sym__function_call_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(302), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(300), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [16339] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(326), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(324), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16370] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(330), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(328), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16401] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(346), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(344), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16432] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(350), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(348), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16463] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(354), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(352), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16494] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(366), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(364), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16525] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(374), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(372), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16556] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(386), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(384), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16587] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(398), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(396), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16618] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(418), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(416), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16649] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(382), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(380), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16680] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(422), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(420), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16711] = 5,
    ACTIONS(536), 1,
      anon_sym_LPAREN,
    STATE(19), 1,
      sym__function_call_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(302), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(300), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [16746] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(402), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(400), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16777] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(410), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(408), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16808] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(318), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(316), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16839] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(314), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(312), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16870] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(438), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(436), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16901] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(322), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(320), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16932] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(358), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(356), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16963] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(434), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(432), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [16994] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(430), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(428), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17025] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(426), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(424), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17056] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(414), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(412), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17087] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(406), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(404), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17118] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(390), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(388), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17149] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(378), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(376), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17180] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(370), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(368), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17211] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(362), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(360), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17242] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(342), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(340), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17273] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(338), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(336), 17,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [17304] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(406), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(404), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17333] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(402), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(400), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17362] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(394), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(392), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17391] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(410), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(408), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17420] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(402), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(400), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17449] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(338), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(336), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17478] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(422), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(420), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17507] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(382), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(380), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17536] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(418), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(416), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17565] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(398), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(396), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17594] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(386), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(384), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17623] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(434), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(432), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17652] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(374), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(372), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17681] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(430), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(428), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17710] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(366), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(364), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17739] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(314), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(312), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17768] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(354), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(352), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17797] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(426), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(424), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17826] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(350), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(348), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17855] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(414), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(412), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17884] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(438), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(436), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17913] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(406), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(404), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17942] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(346), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(344), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [17971] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(390), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(388), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18000] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(334), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(332), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18029] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(330), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(328), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18058] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(326), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(324), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18087] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(322), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(320), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18116] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(358), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(356), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18145] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(378), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(376), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18174] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(434), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(432), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18203] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(430), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(428), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18232] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(426), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(424), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18261] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(414), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(412), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18290] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(438), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(436), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18319] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(322), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(320), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18348] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(370), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(368), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18377] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(390), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(388), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18406] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(378), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(376), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18435] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(370), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(368), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18464] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(362), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(360), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18493] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(362), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(360), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18522] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(342), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(340), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18551] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(354), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(352), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18580] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(342), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(340), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18609] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(366), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(364), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18638] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(318), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(316), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18667] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(374), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(372), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18696] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(386), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(384), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18725] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(398), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(396), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18754] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(350), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(348), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18783] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(346), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(344), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18812] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(334), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(332), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18841] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(330), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(328), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18870] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(326), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(324), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18899] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(338), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(336), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18928] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(422), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(420), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18957] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(382), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(380), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [18986] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(418), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(416), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [19015] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(314), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(312), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [19044] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(394), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(392), 15,
      sym_template_directive_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [19073] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(318), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(316), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [19102] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(358), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(356), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [19131] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(410), 5,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_SLASH,
      anon_sym_GT,
      anon_sym_LT,
    ACTIONS(408), 15,
      sym_template_interpolation_end,
      anon_sym_DOT_STAR,
      anon_sym_LBRACK_STAR_RBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_GT_EQ,
      anon_sym_LT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      sym_strip_marker,
  [19160] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(540), 6,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LT_LT,
    ACTIONS(538), 11,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_LT_LT_DASH,
  [19186] = 4,
    ACTIONS(546), 1,
      anon_sym_QMARK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(544), 6,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LT_LT,
    ACTIONS(542), 10,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_LT_LT_DASH,
  [19214] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(184), 6,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LT_LT,
    ACTIONS(182), 11,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_QMARK,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_LT_LT_DASH,
  [19240] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(548), 6,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LT_LT,
    ACTIONS(50), 10,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_COMMA,
      anon_sym_LBRACK,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_LT_LT_DASH,
  [19265] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(552), 7,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_for,
      anon_sym_LT_LT,
    ACTIONS(550), 9,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_LT_LT_DASH,
  [19290] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(556), 7,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_for,
      anon_sym_LT_LT,
    ACTIONS(554), 9,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_LBRACK,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_LT_LT_DASH,
  [19315] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(556), 6,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LT_LT,
    ACTIONS(554), 9,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_LBRACK,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_LT_LT_DASH,
  [19339] = 11,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(562), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(381), 1,
      sym_template_else_intro,
    STATE(436), 1,
      sym_template_if_end,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19378] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(566), 6,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LT_LT,
    ACTIONS(564), 8,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_LBRACK,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_LT_LT_DASH,
  [19401] = 11,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(568), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(394), 1,
      sym_template_else_intro,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(490), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19440] = 11,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(568), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(368), 1,
      sym_template_else_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(477), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(355), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19479] = 11,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(562), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(379), 1,
      sym_template_else_intro,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(488), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(353), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19518] = 11,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(570), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(361), 1,
      sym_template_else_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(493), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19557] = 3,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(574), 6,
      sym_identifier,
      aux_sym_numeric_lit_token1,
      anon_sym_true,
      anon_sym_false,
      sym_null_lit,
      anon_sym_LT_LT,
    ACTIONS(572), 8,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      anon_sym_LPAREN,
      aux_sym_numeric_lit_token2,
      anon_sym_LBRACK,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_LT_LT_DASH,
  [19580] = 11,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(570), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(389), 1,
      sym_template_else_intro,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(528), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(358), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19619] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(576), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(510), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(362), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19655] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(576), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(526), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19691] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(584), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(367), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19727] = 10,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(586), 1,
      sym_quoted_template_end,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(366), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19763] = 11,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    ACTIONS(592), 1,
      sym_quoted_template_end,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    STATE(468), 1,
      sym_template_literal,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(384), 3,
      aux_sym__template,
      sym_template_interpolation,
      sym_template_directive,
  [19801] = 10,
    ACTIONS(594), 1,
      sym_quoted_template_end,
    ACTIONS(596), 1,
      sym__template_literal_chunk,
    ACTIONS(599), 1,
      sym_template_interpolation_start,
    ACTIONS(602), 1,
      sym_template_directive_start,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(366), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19837] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(605), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(371), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19873] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(607), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(490), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(393), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19909] = 11,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    ACTIONS(609), 1,
      sym_quoted_template_end,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    STATE(479), 1,
      sym_template_literal,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(364), 3,
      aux_sym__template,
      sym_template_interpolation,
      sym_template_directive,
  [19947] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(611), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(491), 1,
      sym_template_for_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [19983] = 10,
    ACTIONS(594), 1,
      sym_heredoc_identifier,
    ACTIONS(602), 1,
      sym_template_directive_start,
    ACTIONS(613), 1,
      sym__template_literal_chunk,
    ACTIONS(616), 1,
      sym_template_interpolation_start,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(371), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20019] = 11,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    ACTIONS(619), 1,
      sym_quoted_template_end,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    STATE(441), 1,
      sym_template_literal,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(374), 3,
      aux_sym__template,
      sym_template_interpolation,
      sym_template_directive,
  [20057] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(621), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(371), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20093] = 10,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    ACTIONS(623), 1,
      sym_quoted_template_end,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(366), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20129] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(625), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(443), 1,
      sym_template_if_end,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20165] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(627), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(539), 1,
      sym_template_for_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(387), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20201] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(629), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(439), 1,
      sym_template_for_end,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20237] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(631), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(395), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20273] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(625), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(436), 1,
      sym_template_if_end,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(391), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20309] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(633), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(373), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20345] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(625), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(483), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(375), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20381] = 10,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    ACTIONS(635), 1,
      sym_quoted_template_end,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(366), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20417] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(637), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(371), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20453] = 10,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    ACTIONS(639), 1,
      sym_quoted_template_end,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(366), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20489] = 11,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    ACTIONS(641), 1,
      sym_quoted_template_end,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    STATE(487), 1,
      sym_template_literal,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(382), 3,
      aux_sym__template,
      sym_template_interpolation,
      sym_template_directive,
  [20527] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(643), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(390), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20563] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(627), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(498), 1,
      sym_template_for_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20599] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(607), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(463), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20635] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(576), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(493), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(392), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20671] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(645), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(371), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20707] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(625), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(483), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20743] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(576), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(510), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20779] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(607), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(471), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20815] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(607), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(471), 1,
      sym_template_if_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(388), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20851] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(647), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(371), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20887] = 11,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    ACTIONS(649), 1,
      sym_quoted_template_end,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    STATE(455), 1,
      sym_template_literal,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(398), 3,
      aux_sym__template,
      sym_template_interpolation,
      sym_template_directive,
  [20925] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(629), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(485), 1,
      sym_template_for_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(377), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20961] = 10,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(588), 1,
      sym__template_literal_chunk,
    ACTIONS(590), 1,
      sym_template_interpolation_start,
    ACTIONS(651), 1,
      sym_quoted_template_end,
    STATE(357), 1,
      sym_template_if_intro,
    STATE(397), 1,
      sym_template_for_start,
    STATE(423), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(452), 2,
      sym_template_for,
      sym_template_if,
    STATE(366), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [20997] = 10,
    ACTIONS(558), 1,
      sym__template_literal_chunk,
    ACTIONS(560), 1,
      sym_template_interpolation_start,
    ACTIONS(611), 1,
      sym_template_directive_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    STATE(475), 1,
      sym_template_for_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(370), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [21033] = 10,
    ACTIONS(578), 1,
      sym__template_literal_chunk,
    ACTIONS(580), 1,
      sym_template_interpolation_start,
    ACTIONS(582), 1,
      sym_template_directive_start,
    ACTIONS(653), 1,
      sym_heredoc_identifier,
    STATE(356), 1,
      sym_template_if_intro,
    STATE(399), 1,
      sym_template_for_start,
    STATE(425), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(446), 2,
      sym_template_for,
      sym_template_if,
    STATE(383), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [21069] = 9,
    ACTIONS(602), 1,
      sym_template_directive_start,
    ACTIONS(655), 1,
      sym__template_literal_chunk,
    ACTIONS(658), 1,
      sym_template_interpolation_start,
    STATE(360), 1,
      sym_template_if_intro,
    STATE(376), 1,
      sym_template_for_start,
    STATE(445), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(547), 2,
      sym_template_for,
      sym_template_if,
    STATE(401), 4,
      aux_sym__template,
      sym_template_literal,
      sym_template_interpolation,
      sym_template_directive,
  [21102] = 3,
    ACTIONS(540), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(538), 10,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
  [21122] = 3,
    ACTIONS(184), 1,
      anon_sym_EQ,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(182), 10,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_COLON,
      anon_sym_EQ_GT,
      sym_ellipsis,
      anon_sym_QMARK,
  [21142] = 6,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(661), 1,
      anon_sym_RBRACE,
    STATE(494), 1,
      sym_block_end,
    STATE(590), 1,
      sym_body,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(414), 3,
      sym_attribute,
      sym_block,
      aux_sym_body_repeat1,
  [21164] = 6,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(661), 1,
      anon_sym_RBRACE,
    STATE(502), 1,
      sym_block_end,
    STATE(627), 1,
      sym_body,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(414), 3,
      sym_attribute,
      sym_block,
      aux_sym_body_repeat1,
  [21186] = 7,
    ACTIONS(663), 1,
      anon_sym_EQ,
    ACTIONS(665), 1,
      anon_sym_LBRACE,
    ACTIONS(667), 1,
      sym_identifier,
    ACTIONS(669), 1,
      sym_quoted_template_start,
    STATE(405), 1,
      sym_block_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(413), 2,
      sym_string_lit,
      aux_sym_block_repeat1,
  [21210] = 7,
    ACTIONS(671), 1,
      anon_sym_RPAREN,
    ACTIONS(673), 1,
      anon_sym_COMMA,
    ACTIONS(675), 1,
      sym_ellipsis,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    STATE(30), 1,
      sym__comma,
    STATE(418), 1,
      aux_sym__tuple_elems_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21233] = 7,
    ACTIONS(39), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(681), 1,
      sym_ellipsis,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    STATE(278), 1,
      sym_object_end,
    STATE(607), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21256] = 7,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    ACTIONS(685), 1,
      sym_ellipsis,
    STATE(318), 1,
      sym_object_end,
    STATE(584), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21279] = 7,
    ACTIONS(41), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    ACTIONS(687), 1,
      sym_ellipsis,
    STATE(212), 1,
      sym_object_end,
    STATE(562), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21302] = 5,
    ACTIONS(691), 1,
      anon_sym_COMMA,
    STATE(70), 1,
      sym__comma,
    STATE(411), 1,
      aux_sym__tuple_elems_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(689), 3,
      anon_sym_RPAREN,
      anon_sym_RBRACK,
      sym_ellipsis,
  [21321] = 4,
    ACTIONS(696), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(694), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    STATE(412), 3,
      sym_attribute,
      sym_block,
      aux_sym_body_repeat1,
  [21338] = 6,
    ACTIONS(665), 1,
      anon_sym_LBRACE,
    ACTIONS(669), 1,
      sym_quoted_template_start,
    ACTIONS(699), 1,
      sym_identifier,
    STATE(404), 1,
      sym_block_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(417), 2,
      sym_string_lit,
      aux_sym_block_repeat1,
  [21359] = 4,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(701), 2,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
    STATE(412), 3,
      sym_attribute,
      sym_block,
      aux_sym_body_repeat1,
  [21376] = 7,
    ACTIONS(13), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    ACTIONS(703), 1,
      sym_ellipsis,
    STATE(157), 1,
      sym_object_end,
    STATE(588), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21399] = 7,
    ACTIONS(43), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    ACTIONS(705), 1,
      sym_ellipsis,
    STATE(321), 1,
      sym_object_end,
    STATE(579), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21422] = 5,
    ACTIONS(707), 1,
      anon_sym_LBRACE,
    ACTIONS(709), 1,
      sym_identifier,
    ACTIONS(712), 1,
      sym_quoted_template_start,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    STATE(417), 2,
      sym_string_lit,
      aux_sym_block_repeat1,
  [21440] = 6,
    ACTIONS(162), 1,
      anon_sym_RPAREN,
    ACTIONS(715), 1,
      anon_sym_COMMA,
    ACTIONS(717), 1,
      sym_ellipsis,
    STATE(31), 1,
      sym__comma,
    STATE(411), 1,
      aux_sym__tuple_elems_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21460] = 6,
    ACTIONS(94), 1,
      anon_sym_RBRACK,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    STATE(282), 1,
      sym_tuple_end,
    STATE(586), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21480] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(689), 4,
      anon_sym_RPAREN,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      sym_ellipsis,
  [21494] = 6,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(723), 1,
      sym_strip_marker,
    ACTIONS(725), 1,
      anon_sym_else,
    ACTIONS(727), 1,
      anon_sym_endif,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21514] = 6,
    ACTIONS(92), 1,
      anon_sym_RBRACK,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    STATE(275), 1,
      sym_tuple_end,
    STATE(598), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21534] = 4,
    ACTIONS(731), 1,
      sym__template_literal_chunk,
    STATE(429), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(729), 3,
      sym_quoted_template_end,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21550] = 6,
    ACTIONS(102), 1,
      anon_sym_RBRACK,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    STATE(303), 1,
      sym_tuple_end,
    STATE(609), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21570] = 4,
    ACTIONS(733), 1,
      sym__template_literal_chunk,
    STATE(434), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(729), 3,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [21586] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(182), 5,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
  [21598] = 6,
    ACTIONS(104), 1,
      anon_sym_RBRACK,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    STATE(166), 1,
      sym_tuple_end,
    STATE(619), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21618] = 6,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(725), 1,
      anon_sym_else,
    ACTIONS(735), 1,
      sym_strip_marker,
    ACTIONS(737), 1,
      anon_sym_endif,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21638] = 4,
    ACTIONS(741), 1,
      sym__template_literal_chunk,
    STATE(429), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(739), 3,
      sym_quoted_template_end,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21654] = 6,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(725), 1,
      anon_sym_else,
    ACTIONS(744), 1,
      sym_strip_marker,
    ACTIONS(746), 1,
      anon_sym_endif,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21674] = 6,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(748), 1,
      anon_sym_COMMA,
    ACTIONS(750), 1,
      anon_sym_RBRACK,
    STATE(39), 1,
      sym__comma,
    STATE(480), 1,
      aux_sym__tuple_elems_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21694] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(538), 5,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
      anon_sym_if,
      sym_ellipsis,
      anon_sym_QMARK,
  [21706] = 6,
    ACTIONS(100), 1,
      anon_sym_RBRACK,
    ACTIONS(679), 1,
      anon_sym_if,
    ACTIONS(683), 1,
      anon_sym_QMARK,
    STATE(208), 1,
      sym_tuple_end,
    STATE(604), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21726] = 4,
    ACTIONS(752), 1,
      sym__template_literal_chunk,
    STATE(434), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(739), 3,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [21742] = 5,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(727), 1,
      anon_sym_endif,
    ACTIONS(755), 1,
      sym_strip_marker,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21759] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(757), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21770] = 5,
    ACTIONS(759), 1,
      sym_quoted_template_end,
    ACTIONS(761), 1,
      sym__template_literal_chunk,
    STATE(513), 1,
      aux_sym_template_literal_repeat1,
    STATE(640), 1,
      sym_template_literal,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21787] = 5,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(767), 1,
      anon_sym_else,
    ACTIONS(769), 1,
      anon_sym_endif,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21804] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(771), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21815] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(773), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21826] = 3,
    ACTIONS(775), 1,
      sym_quoted_template_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(777), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21839] = 5,
    ACTIONS(41), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    STATE(203), 1,
      sym_object_end,
    STATE(610), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21856] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(779), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21867] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(781), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21878] = 4,
    ACTIONS(783), 1,
      sym__template_literal_chunk,
    STATE(469), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(729), 2,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21893] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(785), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [21904] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(787), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21915] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(789), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21926] = 5,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(767), 1,
      anon_sym_else,
    ACTIONS(791), 1,
      anon_sym_endif,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21943] = 5,
    ACTIONS(43), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    STATE(323), 1,
      sym_object_end,
    STATE(571), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21960] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(793), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21971] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(785), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [21982] = 5,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(795), 1,
      sym_strip_marker,
    ACTIONS(797), 1,
      anon_sym_endfor,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [21999] = 5,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    STATE(322), 1,
      sym_object_end,
    STATE(573), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22016] = 3,
    ACTIONS(799), 1,
      sym_quoted_template_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(777), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22029] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(801), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22040] = 5,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(767), 1,
      anon_sym_else,
    ACTIONS(803), 1,
      anon_sym_endif,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22057] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(805), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22068] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(789), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22079] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(781), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22090] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(807), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22101] = 5,
    ACTIONS(13), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    STATE(155), 1,
      sym_object_end,
    STATE(587), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22118] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(779), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22129] = 5,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(809), 1,
      sym_strip_marker,
    ACTIONS(811), 1,
      anon_sym_endfor,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22146] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(787), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22157] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(793), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22168] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(805), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22179] = 3,
    ACTIONS(813), 1,
      sym_quoted_template_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(777), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22192] = 4,
    ACTIONS(815), 1,
      sym__template_literal_chunk,
    STATE(469), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(739), 2,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22207] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(807), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22218] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(818), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22229] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(820), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22240] = 5,
    ACTIONS(39), 1,
      anon_sym_RBRACE,
    ACTIONS(679), 1,
      anon_sym_if,
    STATE(279), 1,
      sym_object_end,
    STATE(615), 1,
      sym_for_cond,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22257] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(773), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22268] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(822), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22279] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(824), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22290] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(826), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22301] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(824), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22312] = 3,
    ACTIONS(828), 1,
      sym_quoted_template_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(777), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22325] = 5,
    ACTIONS(198), 1,
      anon_sym_RBRACK,
    ACTIONS(830), 1,
      anon_sym_COMMA,
    STATE(41), 1,
      sym__comma,
    STATE(411), 1,
      aux_sym__tuple_elems_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22342] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(832), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
  [22355] = 5,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(834), 1,
      sym_strip_marker,
    ACTIONS(836), 1,
      anon_sym_endfor,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22372] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(818), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22383] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(820), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22394] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(822), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22405] = 5,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(737), 1,
      anon_sym_endif,
    ACTIONS(838), 1,
      sym_strip_marker,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22422] = 3,
    ACTIONS(840), 1,
      sym_quoted_template_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(777), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22435] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(826), 4,
      sym_quoted_template_end,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22446] = 5,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(746), 1,
      anon_sym_endif,
    ACTIONS(842), 1,
      sym_strip_marker,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22463] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(757), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22474] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(771), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22485] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(801), 4,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
      sym_heredoc_identifier,
  [22496] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(757), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22506] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(844), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
  [22516] = 4,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(791), 1,
      anon_sym_endif,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22530] = 4,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(846), 1,
      anon_sym_endfor,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22544] = 4,
    ACTIONS(140), 1,
      sym_strip_marker,
    ACTIONS(142), 1,
      sym_template_interpolation_end,
    ACTIONS(848), 1,
      anon_sym_QMARK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22558] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(771), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22568] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(801), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22578] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(824), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22588] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(850), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22598] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(852), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
  [22608] = 4,
    ACTIONS(848), 1,
      anon_sym_QMARK,
    ACTIONS(854), 1,
      sym_strip_marker,
    ACTIONS(856), 1,
      sym_template_interpolation_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22622] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(858), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22632] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(773), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22642] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(860), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
  [22652] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(862), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22662] = 4,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(769), 1,
      anon_sym_endif,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22676] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(820), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22686] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(818), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22696] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(538), 3,
      sym_template_interpolation_end,
      anon_sym_QMARK,
      sym_strip_marker,
  [22706] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(864), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22716] = 4,
    ACTIONS(729), 1,
      sym_quoted_template_end,
    ACTIONS(866), 1,
      sym__template_literal_chunk,
    STATE(525), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22730] = 4,
    ACTIONS(144), 1,
      sym_strip_marker,
    ACTIONS(146), 1,
      sym_template_interpolation_end,
    ACTIONS(848), 1,
      anon_sym_QMARK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22744] = 4,
    ACTIONS(868), 1,
      anon_sym_QMARK,
    ACTIONS(870), 1,
      sym_strip_marker,
    ACTIONS(872), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22758] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(538), 3,
      sym_template_directive_end,
      anon_sym_QMARK,
      sym_strip_marker,
  [22768] = 4,
    ACTIONS(719), 1,
      anon_sym_for,
    ACTIONS(721), 1,
      anon_sym_if,
    ACTIONS(874), 1,
      sym_strip_marker,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22782] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(805), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22792] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(793), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22802] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(876), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22812] = 4,
    ACTIONS(130), 1,
      sym_strip_marker,
    ACTIONS(134), 1,
      sym_template_interpolation_end,
    ACTIONS(848), 1,
      anon_sym_QMARK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22826] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(396), 3,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      sym_identifier,
  [22836] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(787), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22846] = 4,
    ACTIONS(868), 1,
      anon_sym_QMARK,
    ACTIONS(878), 1,
      sym_strip_marker,
    ACTIONS(880), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22860] = 4,
    ACTIONS(739), 1,
      sym_quoted_template_end,
    ACTIONS(882), 1,
      sym__template_literal_chunk,
    STATE(525), 1,
      aux_sym_template_literal_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22874] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(779), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22884] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(885), 2,
      anon_sym_RBRACE,
      anon_sym_RBRACK,
  [22896] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(826), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22906] = 4,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(887), 1,
      anon_sym_endfor,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22920] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(336), 3,
      sym_quoted_template_start,
      anon_sym_LBRACE,
      sym_identifier,
  [22930] = 4,
    ACTIONS(868), 1,
      anon_sym_QMARK,
    ACTIONS(889), 1,
      sym_strip_marker,
    ACTIONS(891), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22944] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(893), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22954] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(895), 2,
      anon_sym_EQ,
      anon_sym_COLON,
  [22966] = 4,
    ACTIONS(848), 1,
      anon_sym_QMARK,
    ACTIONS(897), 1,
      sym_strip_marker,
    ACTIONS(899), 1,
      sym_template_interpolation_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [22980] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(781), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [22990] = 4,
    ACTIONS(868), 1,
      anon_sym_QMARK,
    ACTIONS(901), 1,
      sym_strip_marker,
    ACTIONS(903), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23004] = 4,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(803), 1,
      anon_sym_endif,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23018] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(905), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [23028] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(822), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [23038] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(789), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [23048] = 4,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(907), 1,
      anon_sym_endfor,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23062] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(807), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [23072] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(909), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [23082] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(911), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [23092] = 4,
    ACTIONS(848), 1,
      anon_sym_QMARK,
    ACTIONS(913), 1,
      sym_strip_marker,
    ACTIONS(915), 1,
      sym_template_interpolation_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23106] = 4,
    ACTIONS(868), 1,
      anon_sym_QMARK,
    ACTIONS(917), 1,
      sym_strip_marker,
    ACTIONS(919), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23120] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(785), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [23130] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(182), 3,
      sym_template_interpolation_end,
      anon_sym_QMARK,
      sym_strip_marker,
  [23140] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(182), 3,
      sym_template_directive_end,
      anon_sym_QMARK,
      sym_strip_marker,
  [23150] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(921), 3,
      ts_builtin_sym_end,
      anon_sym_RBRACE,
      sym_identifier,
  [23160] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(923), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [23170] = 4,
    ACTIONS(868), 1,
      anon_sym_QMARK,
    ACTIONS(925), 1,
      sym_strip_marker,
    ACTIONS(927), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23184] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(929), 3,
      sym__template_literal_chunk,
      sym_template_interpolation_start,
      sym_template_directive_start,
  [23194] = 3,
    ACTIONS(931), 1,
      anon_sym_RPAREN,
    STATE(209), 1,
      sym__function_call_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23205] = 3,
    ACTIONS(933), 1,
      sym_strip_marker,
    ACTIONS(935), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23216] = 3,
    ACTIONS(937), 1,
      sym_identifier,
    ACTIONS(939), 1,
      aux_sym_legacy_index_token1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23227] = 3,
    ACTIONS(941), 1,
      sym_strip_marker,
    ACTIONS(943), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23238] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(945), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23249] = 3,
    ACTIONS(947), 1,
      sym_strip_marker,
    ACTIONS(949), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23260] = 3,
    ACTIONS(92), 1,
      anon_sym_RBRACK,
    STATE(271), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23271] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(951), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23282] = 3,
    ACTIONS(41), 1,
      anon_sym_RBRACE,
    STATE(203), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23293] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(953), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23304] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(955), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23315] = 3,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    STATE(345), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23326] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(957), 1,
      anon_sym_EQ_GT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23337] = 3,
    ACTIONS(100), 1,
      anon_sym_RBRACK,
    STATE(200), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23348] = 3,
    ACTIONS(959), 1,
      sym_strip_marker,
    ACTIONS(961), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23359] = 3,
    ACTIONS(963), 1,
      sym_strip_marker,
    ACTIONS(965), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23370] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
    ACTIONS(967), 2,
      anon_sym_RBRACE,
      sym_identifier,
  [23379] = 3,
    ACTIONS(43), 1,
      anon_sym_RBRACE,
    STATE(326), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23390] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(969), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23401] = 3,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    STATE(324), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23412] = 3,
    ACTIONS(971), 1,
      sym_strip_marker,
    ACTIONS(973), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23423] = 3,
    ACTIONS(975), 1,
      sym_strip_marker,
    ACTIONS(977), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23434] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(979), 1,
      anon_sym_EQ_GT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23445] = 3,
    ACTIONS(981), 1,
      sym_strip_marker,
    ACTIONS(983), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23456] = 3,
    ACTIONS(102), 1,
      anon_sym_RBRACK,
    STATE(293), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23467] = 3,
    ACTIONS(43), 1,
      anon_sym_RBRACE,
    STATE(323), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23478] = 3,
    ACTIONS(985), 1,
      anon_sym_RPAREN,
    STATE(273), 1,
      sym__function_call_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23489] = 3,
    ACTIONS(39), 1,
      anon_sym_RBRACE,
    STATE(265), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23500] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(987), 1,
      anon_sym_EQ_GT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23511] = 3,
    ACTIONS(989), 1,
      anon_sym_COMMA,
    ACTIONS(991), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23522] = 3,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    STATE(322), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23533] = 3,
    ACTIONS(41), 1,
      anon_sym_RBRACE,
    STATE(205), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23544] = 3,
    ACTIONS(94), 1,
      anon_sym_RBRACK,
    STATE(320), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23555] = 3,
    ACTIONS(13), 1,
      anon_sym_RBRACE,
    STATE(150), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23566] = 3,
    ACTIONS(13), 1,
      anon_sym_RBRACE,
    STATE(155), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23577] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(993), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23588] = 3,
    ACTIONS(661), 1,
      anon_sym_RBRACE,
    STATE(550), 1,
      sym_block_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23599] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(995), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23610] = 3,
    ACTIONS(997), 1,
      sym_identifier,
    ACTIONS(999), 1,
      aux_sym_legacy_index_token1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23621] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1001), 1,
      anon_sym_EQ_GT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23632] = 3,
    ACTIONS(763), 1,
      anon_sym_for,
    ACTIONS(765), 1,
      anon_sym_if,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23643] = 3,
    ACTIONS(104), 1,
      anon_sym_RBRACK,
    STATE(173), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23654] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1003), 1,
      anon_sym_EQ_GT,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23665] = 3,
    ACTIONS(1005), 1,
      anon_sym_RPAREN,
    STATE(314), 1,
      sym__function_call_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23676] = 3,
    ACTIONS(92), 1,
      anon_sym_RBRACK,
    STATE(277), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23687] = 3,
    ACTIONS(1007), 1,
      anon_sym_RPAREN,
    STATE(171), 1,
      sym__function_call_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23698] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1009), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23709] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1011), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23720] = 3,
    ACTIONS(94), 1,
      anon_sym_RBRACK,
    STATE(312), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23731] = 3,
    ACTIONS(1013), 1,
      anon_sym_RPAREN,
    STATE(299), 1,
      sym__function_call_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23742] = 3,
    ACTIONS(100), 1,
      anon_sym_RBRACK,
    STATE(195), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23753] = 3,
    ACTIONS(1015), 1,
      sym_strip_marker,
    ACTIONS(1017), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23764] = 3,
    ACTIONS(1019), 1,
      sym_strip_marker,
    ACTIONS(1021), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23775] = 3,
    ACTIONS(39), 1,
      anon_sym_RBRACE,
    STATE(279), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23786] = 3,
    ACTIONS(1023), 1,
      sym_strip_marker,
    ACTIONS(1025), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23797] = 3,
    ACTIONS(102), 1,
      anon_sym_RBRACK,
    STATE(311), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23808] = 3,
    ACTIONS(41), 1,
      anon_sym_RBRACE,
    STATE(185), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23819] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1027), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23830] = 3,
    ACTIONS(1029), 1,
      sym_strip_marker,
    ACTIONS(1031), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23841] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1033), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23852] = 3,
    ACTIONS(1035), 1,
      sym_strip_marker,
    ACTIONS(1037), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23863] = 3,
    ACTIONS(39), 1,
      anon_sym_RBRACE,
    STATE(280), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23874] = 3,
    ACTIONS(1039), 1,
      sym_identifier,
    ACTIONS(1041), 1,
      aux_sym_legacy_index_token1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23885] = 3,
    ACTIONS(1043), 1,
      sym_identifier,
    ACTIONS(1045), 1,
      aux_sym_legacy_index_token1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23896] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1047), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23907] = 3,
    ACTIONS(104), 1,
      anon_sym_RBRACK,
    STATE(159), 1,
      sym_tuple_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23918] = 3,
    ACTIONS(13), 1,
      anon_sym_RBRACE,
    STATE(167), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23929] = 3,
    ACTIONS(1049), 1,
      sym_strip_marker,
    ACTIONS(1051), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23940] = 3,
    ACTIONS(43), 1,
      anon_sym_RBRACE,
    STATE(285), 1,
      sym_object_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23951] = 3,
    ACTIONS(1053), 1,
      anon_sym_COMMA,
    ACTIONS(1055), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23962] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1057), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23973] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1059), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23984] = 3,
    ACTIONS(1061), 1,
      sym_identifier,
    ACTIONS(1063), 1,
      aux_sym_legacy_index_token1,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [23995] = 3,
    ACTIONS(661), 1,
      anon_sym_RBRACE,
    STATE(494), 1,
      sym_block_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24006] = 3,
    ACTIONS(1065), 1,
      anon_sym_COMMA,
    ACTIONS(1067), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24017] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1069), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24028] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1071), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24039] = 3,
    ACTIONS(677), 1,
      anon_sym_QMARK,
    ACTIONS(1073), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24050] = 2,
    ACTIONS(1075), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24058] = 2,
    ACTIONS(1077), 1,
      sym_heredoc_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24066] = 2,
    ACTIONS(1079), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24074] = 2,
    ACTIONS(872), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24082] = 2,
    ACTIONS(1081), 1,
      sym_template_interpolation_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24090] = 2,
    ACTIONS(1083), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24098] = 2,
    ACTIONS(1085), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24106] = 2,
    ACTIONS(1051), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24114] = 2,
    ACTIONS(1087), 1,
      sym_quoted_template_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24122] = 2,
    ACTIONS(1037), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24130] = 2,
    ACTIONS(1031), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24138] = 2,
    ACTIONS(1089), 1,
      sym_template_interpolation_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24146] = 2,
    ACTIONS(935), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24154] = 2,
    ACTIONS(1025), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24162] = 2,
    ACTIONS(1091), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24170] = 2,
    ACTIONS(1093), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24178] = 2,
    ACTIONS(1095), 1,
      sym_heredoc_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24186] = 2,
    ACTIONS(856), 1,
      sym_template_interpolation_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24194] = 2,
    ACTIONS(164), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24202] = 2,
    ACTIONS(1097), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24210] = 2,
    ACTIONS(1099), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24218] = 2,
    ACTIONS(1101), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24226] = 2,
    ACTIONS(880), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24234] = 2,
    ACTIONS(919), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24242] = 2,
    ACTIONS(1103), 1,
      sym_heredoc_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24250] = 2,
    ACTIONS(1105), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24258] = 2,
    ACTIONS(1107), 1,
      sym_heredoc_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24266] = 2,
    ACTIONS(899), 1,
      sym_template_interpolation_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24274] = 2,
    ACTIONS(1109), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24282] = 2,
    ACTIONS(1111), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24290] = 2,
    ACTIONS(1113), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24298] = 2,
    ACTIONS(162), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24306] = 2,
    ACTIONS(1115), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24314] = 2,
    ACTIONS(1117), 1,
      sym_heredoc_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24322] = 2,
    ACTIONS(1119), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24330] = 2,
    ACTIONS(983), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24338] = 2,
    ACTIONS(977), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24346] = 2,
    ACTIONS(1121), 1,
      sym_template_interpolation_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24354] = 2,
    ACTIONS(1123), 1,
      sym_heredoc_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24362] = 2,
    ACTIONS(1125), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24370] = 2,
    ACTIONS(1127), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24378] = 2,
    ACTIONS(915), 1,
      sym_template_interpolation_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24386] = 2,
    ACTIONS(1129), 1,
      anon_sym_in,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24394] = 2,
    ACTIONS(1131), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24402] = 2,
    ACTIONS(1133), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
  [24410] = 2,
    ACTIONS(927), 1,
      sym_template_directive_end,
    ACTIONS(3), 2,
      sym_comment,
      sym__whitespace,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 107,
  [SMALL_STATE(4)] = 214,
  [SMALL_STATE(5)] = 321,
  [SMALL_STATE(6)] = 428,
  [SMALL_STATE(7)] = 535,
  [SMALL_STATE(8)] = 637,
  [SMALL_STATE(9)] = 739,
  [SMALL_STATE(10)] = 843,
  [SMALL_STATE(11)] = 947,
  [SMALL_STATE(12)] = 1049,
  [SMALL_STATE(13)] = 1153,
  [SMALL_STATE(14)] = 1257,
  [SMALL_STATE(15)] = 1361,
  [SMALL_STATE(16)] = 1462,
  [SMALL_STATE(17)] = 1560,
  [SMALL_STATE(18)] = 1658,
  [SMALL_STATE(19)] = 1756,
  [SMALL_STATE(20)] = 1854,
  [SMALL_STATE(21)] = 1952,
  [SMALL_STATE(22)] = 2047,
  [SMALL_STATE(23)] = 2142,
  [SMALL_STATE(24)] = 2237,
  [SMALL_STATE(25)] = 2332,
  [SMALL_STATE(26)] = 2427,
  [SMALL_STATE(27)] = 2522,
  [SMALL_STATE(28)] = 2617,
  [SMALL_STATE(29)] = 2712,
  [SMALL_STATE(30)] = 2766,
  [SMALL_STATE(31)] = 2858,
  [SMALL_STATE(32)] = 2950,
  [SMALL_STATE(33)] = 3008,
  [SMALL_STATE(34)] = 3062,
  [SMALL_STATE(35)] = 3122,
  [SMALL_STATE(36)] = 3186,
  [SMALL_STATE(37)] = 3252,
  [SMALL_STATE(38)] = 3330,
  [SMALL_STATE(39)] = 3398,
  [SMALL_STATE(40)] = 3490,
  [SMALL_STATE(41)] = 3582,
  [SMALL_STATE(42)] = 3674,
  [SMALL_STATE(43)] = 3763,
  [SMALL_STATE(44)] = 3852,
  [SMALL_STATE(45)] = 3941,
  [SMALL_STATE(46)] = 4030,
  [SMALL_STATE(47)] = 4119,
  [SMALL_STATE(48)] = 4208,
  [SMALL_STATE(49)] = 4297,
  [SMALL_STATE(50)] = 4386,
  [SMALL_STATE(51)] = 4475,
  [SMALL_STATE(52)] = 4564,
  [SMALL_STATE(53)] = 4653,
  [SMALL_STATE(54)] = 4742,
  [SMALL_STATE(55)] = 4831,
  [SMALL_STATE(56)] = 4920,
  [SMALL_STATE(57)] = 5009,
  [SMALL_STATE(58)] = 5098,
  [SMALL_STATE(59)] = 5187,
  [SMALL_STATE(60)] = 5276,
  [SMALL_STATE(61)] = 5365,
  [SMALL_STATE(62)] = 5454,
  [SMALL_STATE(63)] = 5543,
  [SMALL_STATE(64)] = 5632,
  [SMALL_STATE(65)] = 5721,
  [SMALL_STATE(66)] = 5810,
  [SMALL_STATE(67)] = 5899,
  [SMALL_STATE(68)] = 5988,
  [SMALL_STATE(69)] = 6077,
  [SMALL_STATE(70)] = 6166,
  [SMALL_STATE(71)] = 6255,
  [SMALL_STATE(72)] = 6344,
  [SMALL_STATE(73)] = 6433,
  [SMALL_STATE(74)] = 6522,
  [SMALL_STATE(75)] = 6611,
  [SMALL_STATE(76)] = 6700,
  [SMALL_STATE(77)] = 6789,
  [SMALL_STATE(78)] = 6878,
  [SMALL_STATE(79)] = 6967,
  [SMALL_STATE(80)] = 7056,
  [SMALL_STATE(81)] = 7145,
  [SMALL_STATE(82)] = 7234,
  [SMALL_STATE(83)] = 7323,
  [SMALL_STATE(84)] = 7412,
  [SMALL_STATE(85)] = 7501,
  [SMALL_STATE(86)] = 7590,
  [SMALL_STATE(87)] = 7679,
  [SMALL_STATE(88)] = 7768,
  [SMALL_STATE(89)] = 7857,
  [SMALL_STATE(90)] = 7911,
  [SMALL_STATE(91)] = 7965,
  [SMALL_STATE(92)] = 8019,
  [SMALL_STATE(93)] = 8073,
  [SMALL_STATE(94)] = 8127,
  [SMALL_STATE(95)] = 8210,
  [SMALL_STATE(96)] = 8293,
  [SMALL_STATE(97)] = 8376,
  [SMALL_STATE(98)] = 8459,
  [SMALL_STATE(99)] = 8542,
  [SMALL_STATE(100)] = 8625,
  [SMALL_STATE(101)] = 8708,
  [SMALL_STATE(102)] = 8791,
  [SMALL_STATE(103)] = 8874,
  [SMALL_STATE(104)] = 8957,
  [SMALL_STATE(105)] = 9040,
  [SMALL_STATE(106)] = 9123,
  [SMALL_STATE(107)] = 9206,
  [SMALL_STATE(108)] = 9289,
  [SMALL_STATE(109)] = 9372,
  [SMALL_STATE(110)] = 9455,
  [SMALL_STATE(111)] = 9538,
  [SMALL_STATE(112)] = 9621,
  [SMALL_STATE(113)] = 9704,
  [SMALL_STATE(114)] = 9787,
  [SMALL_STATE(115)] = 9870,
  [SMALL_STATE(116)] = 9953,
  [SMALL_STATE(117)] = 10036,
  [SMALL_STATE(118)] = 10119,
  [SMALL_STATE(119)] = 10202,
  [SMALL_STATE(120)] = 10285,
  [SMALL_STATE(121)] = 10368,
  [SMALL_STATE(122)] = 10451,
  [SMALL_STATE(123)] = 10534,
  [SMALL_STATE(124)] = 10617,
  [SMALL_STATE(125)] = 10700,
  [SMALL_STATE(126)] = 10783,
  [SMALL_STATE(127)] = 10866,
  [SMALL_STATE(128)] = 10949,
  [SMALL_STATE(129)] = 11032,
  [SMALL_STATE(130)] = 11106,
  [SMALL_STATE(131)] = 11170,
  [SMALL_STATE(132)] = 11232,
  [SMALL_STATE(133)] = 11292,
  [SMALL_STATE(134)] = 11348,
  [SMALL_STATE(135)] = 11398,
  [SMALL_STATE(136)] = 11452,
  [SMALL_STATE(137)] = 11502,
  [SMALL_STATE(138)] = 11552,
  [SMALL_STATE(139)] = 11602,
  [SMALL_STATE(140)] = 11652,
  [SMALL_STATE(141)] = 11702,
  [SMALL_STATE(142)] = 11748,
  [SMALL_STATE(143)] = 11798,
  [SMALL_STATE(144)] = 11839,
  [SMALL_STATE(145)] = 11880,
  [SMALL_STATE(146)] = 11921,
  [SMALL_STATE(147)] = 11962,
  [SMALL_STATE(148)] = 12003,
  [SMALL_STATE(149)] = 12044,
  [SMALL_STATE(150)] = 12085,
  [SMALL_STATE(151)] = 12126,
  [SMALL_STATE(152)] = 12167,
  [SMALL_STATE(153)] = 12208,
  [SMALL_STATE(154)] = 12249,
  [SMALL_STATE(155)] = 12290,
  [SMALL_STATE(156)] = 12331,
  [SMALL_STATE(157)] = 12372,
  [SMALL_STATE(158)] = 12413,
  [SMALL_STATE(159)] = 12454,
  [SMALL_STATE(160)] = 12495,
  [SMALL_STATE(161)] = 12536,
  [SMALL_STATE(162)] = 12577,
  [SMALL_STATE(163)] = 12618,
  [SMALL_STATE(164)] = 12659,
  [SMALL_STATE(165)] = 12700,
  [SMALL_STATE(166)] = 12741,
  [SMALL_STATE(167)] = 12782,
  [SMALL_STATE(168)] = 12823,
  [SMALL_STATE(169)] = 12864,
  [SMALL_STATE(170)] = 12905,
  [SMALL_STATE(171)] = 12946,
  [SMALL_STATE(172)] = 12987,
  [SMALL_STATE(173)] = 13028,
  [SMALL_STATE(174)] = 13069,
  [SMALL_STATE(175)] = 13110,
  [SMALL_STATE(176)] = 13153,
  [SMALL_STATE(177)] = 13211,
  [SMALL_STATE(178)] = 13267,
  [SMALL_STATE(179)] = 13311,
  [SMALL_STATE(180)] = 13377,
  [SMALL_STATE(181)] = 13431,
  [SMALL_STATE(182)] = 13475,
  [SMALL_STATE(183)] = 13525,
  [SMALL_STATE(184)] = 13573,
  [SMALL_STATE(185)] = 13610,
  [SMALL_STATE(186)] = 13647,
  [SMALL_STATE(187)] = 13684,
  [SMALL_STATE(188)] = 13721,
  [SMALL_STATE(189)] = 13758,
  [SMALL_STATE(190)] = 13795,
  [SMALL_STATE(191)] = 13832,
  [SMALL_STATE(192)] = 13869,
  [SMALL_STATE(193)] = 13906,
  [SMALL_STATE(194)] = 13943,
  [SMALL_STATE(195)] = 13980,
  [SMALL_STATE(196)] = 14017,
  [SMALL_STATE(197)] = 14054,
  [SMALL_STATE(198)] = 14091,
  [SMALL_STATE(199)] = 14128,
  [SMALL_STATE(200)] = 14165,
  [SMALL_STATE(201)] = 14202,
  [SMALL_STATE(202)] = 14239,
  [SMALL_STATE(203)] = 14276,
  [SMALL_STATE(204)] = 14313,
  [SMALL_STATE(205)] = 14350,
  [SMALL_STATE(206)] = 14387,
  [SMALL_STATE(207)] = 14424,
  [SMALL_STATE(208)] = 14461,
  [SMALL_STATE(209)] = 14498,
  [SMALL_STATE(210)] = 14535,
  [SMALL_STATE(211)] = 14572,
  [SMALL_STATE(212)] = 14609,
  [SMALL_STATE(213)] = 14646,
  [SMALL_STATE(214)] = 14683,
  [SMALL_STATE(215)] = 14720,
  [SMALL_STATE(216)] = 14757,
  [SMALL_STATE(217)] = 14799,
  [SMALL_STATE(218)] = 14847,
  [SMALL_STATE(219)] = 14889,
  [SMALL_STATE(220)] = 14953,
  [SMALL_STATE(221)] = 14995,
  [SMALL_STATE(222)] = 15039,
  [SMALL_STATE(223)] = 15103,
  [SMALL_STATE(224)] = 15147,
  [SMALL_STATE(225)] = 15203,
  [SMALL_STATE(226)] = 15257,
  [SMALL_STATE(227)] = 15313,
  [SMALL_STATE(228)] = 15367,
  [SMALL_STATE(229)] = 15419,
  [SMALL_STATE(230)] = 15471,
  [SMALL_STATE(231)] = 15519,
  [SMALL_STATE(232)] = 15563,
  [SMALL_STATE(233)] = 15605,
  [SMALL_STATE(234)] = 15651,
  [SMALL_STATE(235)] = 15695,
  [SMALL_STATE(236)] = 15739,
  [SMALL_STATE(237)] = 15785,
  [SMALL_STATE(238)] = 15827,
  [SMALL_STATE(239)] = 15869,
  [SMALL_STATE(240)] = 15911,
  [SMALL_STATE(241)] = 15953,
  [SMALL_STATE(242)] = 15995,
  [SMALL_STATE(243)] = 16037,
  [SMALL_STATE(244)] = 16079,
  [SMALL_STATE(245)] = 16121,
  [SMALL_STATE(246)] = 16163,
  [SMALL_STATE(247)] = 16205,
  [SMALL_STATE(248)] = 16242,
  [SMALL_STATE(249)] = 16273,
  [SMALL_STATE(250)] = 16304,
  [SMALL_STATE(251)] = 16339,
  [SMALL_STATE(252)] = 16370,
  [SMALL_STATE(253)] = 16401,
  [SMALL_STATE(254)] = 16432,
  [SMALL_STATE(255)] = 16463,
  [SMALL_STATE(256)] = 16494,
  [SMALL_STATE(257)] = 16525,
  [SMALL_STATE(258)] = 16556,
  [SMALL_STATE(259)] = 16587,
  [SMALL_STATE(260)] = 16618,
  [SMALL_STATE(261)] = 16649,
  [SMALL_STATE(262)] = 16680,
  [SMALL_STATE(263)] = 16711,
  [SMALL_STATE(264)] = 16746,
  [SMALL_STATE(265)] = 16777,
  [SMALL_STATE(266)] = 16808,
  [SMALL_STATE(267)] = 16839,
  [SMALL_STATE(268)] = 16870,
  [SMALL_STATE(269)] = 16901,
  [SMALL_STATE(270)] = 16932,
  [SMALL_STATE(271)] = 16963,
  [SMALL_STATE(272)] = 16994,
  [SMALL_STATE(273)] = 17025,
  [SMALL_STATE(274)] = 17056,
  [SMALL_STATE(275)] = 17087,
  [SMALL_STATE(276)] = 17118,
  [SMALL_STATE(277)] = 17149,
  [SMALL_STATE(278)] = 17180,
  [SMALL_STATE(279)] = 17211,
  [SMALL_STATE(280)] = 17242,
  [SMALL_STATE(281)] = 17273,
  [SMALL_STATE(282)] = 17304,
  [SMALL_STATE(283)] = 17333,
  [SMALL_STATE(284)] = 17362,
  [SMALL_STATE(285)] = 17391,
  [SMALL_STATE(286)] = 17420,
  [SMALL_STATE(287)] = 17449,
  [SMALL_STATE(288)] = 17478,
  [SMALL_STATE(289)] = 17507,
  [SMALL_STATE(290)] = 17536,
  [SMALL_STATE(291)] = 17565,
  [SMALL_STATE(292)] = 17594,
  [SMALL_STATE(293)] = 17623,
  [SMALL_STATE(294)] = 17652,
  [SMALL_STATE(295)] = 17681,
  [SMALL_STATE(296)] = 17710,
  [SMALL_STATE(297)] = 17739,
  [SMALL_STATE(298)] = 17768,
  [SMALL_STATE(299)] = 17797,
  [SMALL_STATE(300)] = 17826,
  [SMALL_STATE(301)] = 17855,
  [SMALL_STATE(302)] = 17884,
  [SMALL_STATE(303)] = 17913,
  [SMALL_STATE(304)] = 17942,
  [SMALL_STATE(305)] = 17971,
  [SMALL_STATE(306)] = 18000,
  [SMALL_STATE(307)] = 18029,
  [SMALL_STATE(308)] = 18058,
  [SMALL_STATE(309)] = 18087,
  [SMALL_STATE(310)] = 18116,
  [SMALL_STATE(311)] = 18145,
  [SMALL_STATE(312)] = 18174,
  [SMALL_STATE(313)] = 18203,
  [SMALL_STATE(314)] = 18232,
  [SMALL_STATE(315)] = 18261,
  [SMALL_STATE(316)] = 18290,
  [SMALL_STATE(317)] = 18319,
  [SMALL_STATE(318)] = 18348,
  [SMALL_STATE(319)] = 18377,
  [SMALL_STATE(320)] = 18406,
  [SMALL_STATE(321)] = 18435,
  [SMALL_STATE(322)] = 18464,
  [SMALL_STATE(323)] = 18493,
  [SMALL_STATE(324)] = 18522,
  [SMALL_STATE(325)] = 18551,
  [SMALL_STATE(326)] = 18580,
  [SMALL_STATE(327)] = 18609,
  [SMALL_STATE(328)] = 18638,
  [SMALL_STATE(329)] = 18667,
  [SMALL_STATE(330)] = 18696,
  [SMALL_STATE(331)] = 18725,
  [SMALL_STATE(332)] = 18754,
  [SMALL_STATE(333)] = 18783,
  [SMALL_STATE(334)] = 18812,
  [SMALL_STATE(335)] = 18841,
  [SMALL_STATE(336)] = 18870,
  [SMALL_STATE(337)] = 18899,
  [SMALL_STATE(338)] = 18928,
  [SMALL_STATE(339)] = 18957,
  [SMALL_STATE(340)] = 18986,
  [SMALL_STATE(341)] = 19015,
  [SMALL_STATE(342)] = 19044,
  [SMALL_STATE(343)] = 19073,
  [SMALL_STATE(344)] = 19102,
  [SMALL_STATE(345)] = 19131,
  [SMALL_STATE(346)] = 19160,
  [SMALL_STATE(347)] = 19186,
  [SMALL_STATE(348)] = 19214,
  [SMALL_STATE(349)] = 19240,
  [SMALL_STATE(350)] = 19265,
  [SMALL_STATE(351)] = 19290,
  [SMALL_STATE(352)] = 19315,
  [SMALL_STATE(353)] = 19339,
  [SMALL_STATE(354)] = 19378,
  [SMALL_STATE(355)] = 19401,
  [SMALL_STATE(356)] = 19440,
  [SMALL_STATE(357)] = 19479,
  [SMALL_STATE(358)] = 19518,
  [SMALL_STATE(359)] = 19557,
  [SMALL_STATE(360)] = 19580,
  [SMALL_STATE(361)] = 19619,
  [SMALL_STATE(362)] = 19655,
  [SMALL_STATE(363)] = 19691,
  [SMALL_STATE(364)] = 19727,
  [SMALL_STATE(365)] = 19763,
  [SMALL_STATE(366)] = 19801,
  [SMALL_STATE(367)] = 19837,
  [SMALL_STATE(368)] = 19873,
  [SMALL_STATE(369)] = 19909,
  [SMALL_STATE(370)] = 19947,
  [SMALL_STATE(371)] = 19983,
  [SMALL_STATE(372)] = 20019,
  [SMALL_STATE(373)] = 20057,
  [SMALL_STATE(374)] = 20093,
  [SMALL_STATE(375)] = 20129,
  [SMALL_STATE(376)] = 20165,
  [SMALL_STATE(377)] = 20201,
  [SMALL_STATE(378)] = 20237,
  [SMALL_STATE(379)] = 20273,
  [SMALL_STATE(380)] = 20309,
  [SMALL_STATE(381)] = 20345,
  [SMALL_STATE(382)] = 20381,
  [SMALL_STATE(383)] = 20417,
  [SMALL_STATE(384)] = 20453,
  [SMALL_STATE(385)] = 20489,
  [SMALL_STATE(386)] = 20527,
  [SMALL_STATE(387)] = 20563,
  [SMALL_STATE(388)] = 20599,
  [SMALL_STATE(389)] = 20635,
  [SMALL_STATE(390)] = 20671,
  [SMALL_STATE(391)] = 20707,
  [SMALL_STATE(392)] = 20743,
  [SMALL_STATE(393)] = 20779,
  [SMALL_STATE(394)] = 20815,
  [SMALL_STATE(395)] = 20851,
  [SMALL_STATE(396)] = 20887,
  [SMALL_STATE(397)] = 20925,
  [SMALL_STATE(398)] = 20961,
  [SMALL_STATE(399)] = 20997,
  [SMALL_STATE(400)] = 21033,
  [SMALL_STATE(401)] = 21069,
  [SMALL_STATE(402)] = 21102,
  [SMALL_STATE(403)] = 21122,
  [SMALL_STATE(404)] = 21142,
  [SMALL_STATE(405)] = 21164,
  [SMALL_STATE(406)] = 21186,
  [SMALL_STATE(407)] = 21210,
  [SMALL_STATE(408)] = 21233,
  [SMALL_STATE(409)] = 21256,
  [SMALL_STATE(410)] = 21279,
  [SMALL_STATE(411)] = 21302,
  [SMALL_STATE(412)] = 21321,
  [SMALL_STATE(413)] = 21338,
  [SMALL_STATE(414)] = 21359,
  [SMALL_STATE(415)] = 21376,
  [SMALL_STATE(416)] = 21399,
  [SMALL_STATE(417)] = 21422,
  [SMALL_STATE(418)] = 21440,
  [SMALL_STATE(419)] = 21460,
  [SMALL_STATE(420)] = 21480,
  [SMALL_STATE(421)] = 21494,
  [SMALL_STATE(422)] = 21514,
  [SMALL_STATE(423)] = 21534,
  [SMALL_STATE(424)] = 21550,
  [SMALL_STATE(425)] = 21570,
  [SMALL_STATE(426)] = 21586,
  [SMALL_STATE(427)] = 21598,
  [SMALL_STATE(428)] = 21618,
  [SMALL_STATE(429)] = 21638,
  [SMALL_STATE(430)] = 21654,
  [SMALL_STATE(431)] = 21674,
  [SMALL_STATE(432)] = 21694,
  [SMALL_STATE(433)] = 21706,
  [SMALL_STATE(434)] = 21726,
  [SMALL_STATE(435)] = 21742,
  [SMALL_STATE(436)] = 21759,
  [SMALL_STATE(437)] = 21770,
  [SMALL_STATE(438)] = 21787,
  [SMALL_STATE(439)] = 21804,
  [SMALL_STATE(440)] = 21815,
  [SMALL_STATE(441)] = 21826,
  [SMALL_STATE(442)] = 21839,
  [SMALL_STATE(443)] = 21856,
  [SMALL_STATE(444)] = 21867,
  [SMALL_STATE(445)] = 21878,
  [SMALL_STATE(446)] = 21893,
  [SMALL_STATE(447)] = 21904,
  [SMALL_STATE(448)] = 21915,
  [SMALL_STATE(449)] = 21926,
  [SMALL_STATE(450)] = 21943,
  [SMALL_STATE(451)] = 21960,
  [SMALL_STATE(452)] = 21971,
  [SMALL_STATE(453)] = 21982,
  [SMALL_STATE(454)] = 21999,
  [SMALL_STATE(455)] = 22016,
  [SMALL_STATE(456)] = 22029,
  [SMALL_STATE(457)] = 22040,
  [SMALL_STATE(458)] = 22057,
  [SMALL_STATE(459)] = 22068,
  [SMALL_STATE(460)] = 22079,
  [SMALL_STATE(461)] = 22090,
  [SMALL_STATE(462)] = 22101,
  [SMALL_STATE(463)] = 22118,
  [SMALL_STATE(464)] = 22129,
  [SMALL_STATE(465)] = 22146,
  [SMALL_STATE(466)] = 22157,
  [SMALL_STATE(467)] = 22168,
  [SMALL_STATE(468)] = 22179,
  [SMALL_STATE(469)] = 22192,
  [SMALL_STATE(470)] = 22207,
  [SMALL_STATE(471)] = 22218,
  [SMALL_STATE(472)] = 22229,
  [SMALL_STATE(473)] = 22240,
  [SMALL_STATE(474)] = 22257,
  [SMALL_STATE(475)] = 22268,
  [SMALL_STATE(476)] = 22279,
  [SMALL_STATE(477)] = 22290,
  [SMALL_STATE(478)] = 22301,
  [SMALL_STATE(479)] = 22312,
  [SMALL_STATE(480)] = 22325,
  [SMALL_STATE(481)] = 22342,
  [SMALL_STATE(482)] = 22355,
  [SMALL_STATE(483)] = 22372,
  [SMALL_STATE(484)] = 22383,
  [SMALL_STATE(485)] = 22394,
  [SMALL_STATE(486)] = 22405,
  [SMALL_STATE(487)] = 22422,
  [SMALL_STATE(488)] = 22435,
  [SMALL_STATE(489)] = 22446,
  [SMALL_STATE(490)] = 22463,
  [SMALL_STATE(491)] = 22474,
  [SMALL_STATE(492)] = 22485,
  [SMALL_STATE(493)] = 22496,
  [SMALL_STATE(494)] = 22506,
  [SMALL_STATE(495)] = 22516,
  [SMALL_STATE(496)] = 22530,
  [SMALL_STATE(497)] = 22544,
  [SMALL_STATE(498)] = 22558,
  [SMALL_STATE(499)] = 22568,
  [SMALL_STATE(500)] = 22578,
  [SMALL_STATE(501)] = 22588,
  [SMALL_STATE(502)] = 22598,
  [SMALL_STATE(503)] = 22608,
  [SMALL_STATE(504)] = 22622,
  [SMALL_STATE(505)] = 22632,
  [SMALL_STATE(506)] = 22642,
  [SMALL_STATE(507)] = 22652,
  [SMALL_STATE(508)] = 22662,
  [SMALL_STATE(509)] = 22676,
  [SMALL_STATE(510)] = 22686,
  [SMALL_STATE(511)] = 22696,
  [SMALL_STATE(512)] = 22706,
  [SMALL_STATE(513)] = 22716,
  [SMALL_STATE(514)] = 22730,
  [SMALL_STATE(515)] = 22744,
  [SMALL_STATE(516)] = 22758,
  [SMALL_STATE(517)] = 22768,
  [SMALL_STATE(518)] = 22782,
  [SMALL_STATE(519)] = 22792,
  [SMALL_STATE(520)] = 22802,
  [SMALL_STATE(521)] = 22812,
  [SMALL_STATE(522)] = 22826,
  [SMALL_STATE(523)] = 22836,
  [SMALL_STATE(524)] = 22846,
  [SMALL_STATE(525)] = 22860,
  [SMALL_STATE(526)] = 22874,
  [SMALL_STATE(527)] = 22884,
  [SMALL_STATE(528)] = 22896,
  [SMALL_STATE(529)] = 22906,
  [SMALL_STATE(530)] = 22920,
  [SMALL_STATE(531)] = 22930,
  [SMALL_STATE(532)] = 22944,
  [SMALL_STATE(533)] = 22954,
  [SMALL_STATE(534)] = 22966,
  [SMALL_STATE(535)] = 22980,
  [SMALL_STATE(536)] = 22990,
  [SMALL_STATE(537)] = 23004,
  [SMALL_STATE(538)] = 23018,
  [SMALL_STATE(539)] = 23028,
  [SMALL_STATE(540)] = 23038,
  [SMALL_STATE(541)] = 23048,
  [SMALL_STATE(542)] = 23062,
  [SMALL_STATE(543)] = 23072,
  [SMALL_STATE(544)] = 23082,
  [SMALL_STATE(545)] = 23092,
  [SMALL_STATE(546)] = 23106,
  [SMALL_STATE(547)] = 23120,
  [SMALL_STATE(548)] = 23130,
  [SMALL_STATE(549)] = 23140,
  [SMALL_STATE(550)] = 23150,
  [SMALL_STATE(551)] = 23160,
  [SMALL_STATE(552)] = 23170,
  [SMALL_STATE(553)] = 23184,
  [SMALL_STATE(554)] = 23194,
  [SMALL_STATE(555)] = 23205,
  [SMALL_STATE(556)] = 23216,
  [SMALL_STATE(557)] = 23227,
  [SMALL_STATE(558)] = 23238,
  [SMALL_STATE(559)] = 23249,
  [SMALL_STATE(560)] = 23260,
  [SMALL_STATE(561)] = 23271,
  [SMALL_STATE(562)] = 23282,
  [SMALL_STATE(563)] = 23293,
  [SMALL_STATE(564)] = 23304,
  [SMALL_STATE(565)] = 23315,
  [SMALL_STATE(566)] = 23326,
  [SMALL_STATE(567)] = 23337,
  [SMALL_STATE(568)] = 23348,
  [SMALL_STATE(569)] = 23359,
  [SMALL_STATE(570)] = 23370,
  [SMALL_STATE(571)] = 23379,
  [SMALL_STATE(572)] = 23390,
  [SMALL_STATE(573)] = 23401,
  [SMALL_STATE(574)] = 23412,
  [SMALL_STATE(575)] = 23423,
  [SMALL_STATE(576)] = 23434,
  [SMALL_STATE(577)] = 23445,
  [SMALL_STATE(578)] = 23456,
  [SMALL_STATE(579)] = 23467,
  [SMALL_STATE(580)] = 23478,
  [SMALL_STATE(581)] = 23489,
  [SMALL_STATE(582)] = 23500,
  [SMALL_STATE(583)] = 23511,
  [SMALL_STATE(584)] = 23522,
  [SMALL_STATE(585)] = 23533,
  [SMALL_STATE(586)] = 23544,
  [SMALL_STATE(587)] = 23555,
  [SMALL_STATE(588)] = 23566,
  [SMALL_STATE(589)] = 23577,
  [SMALL_STATE(590)] = 23588,
  [SMALL_STATE(591)] = 23599,
  [SMALL_STATE(592)] = 23610,
  [SMALL_STATE(593)] = 23621,
  [SMALL_STATE(594)] = 23632,
  [SMALL_STATE(595)] = 23643,
  [SMALL_STATE(596)] = 23654,
  [SMALL_STATE(597)] = 23665,
  [SMALL_STATE(598)] = 23676,
  [SMALL_STATE(599)] = 23687,
  [SMALL_STATE(600)] = 23698,
  [SMALL_STATE(601)] = 23709,
  [SMALL_STATE(602)] = 23720,
  [SMALL_STATE(603)] = 23731,
  [SMALL_STATE(604)] = 23742,
  [SMALL_STATE(605)] = 23753,
  [SMALL_STATE(606)] = 23764,
  [SMALL_STATE(607)] = 23775,
  [SMALL_STATE(608)] = 23786,
  [SMALL_STATE(609)] = 23797,
  [SMALL_STATE(610)] = 23808,
  [SMALL_STATE(611)] = 23819,
  [SMALL_STATE(612)] = 23830,
  [SMALL_STATE(613)] = 23841,
  [SMALL_STATE(614)] = 23852,
  [SMALL_STATE(615)] = 23863,
  [SMALL_STATE(616)] = 23874,
  [SMALL_STATE(617)] = 23885,
  [SMALL_STATE(618)] = 23896,
  [SMALL_STATE(619)] = 23907,
  [SMALL_STATE(620)] = 23918,
  [SMALL_STATE(621)] = 23929,
  [SMALL_STATE(622)] = 23940,
  [SMALL_STATE(623)] = 23951,
  [SMALL_STATE(624)] = 23962,
  [SMALL_STATE(625)] = 23973,
  [SMALL_STATE(626)] = 23984,
  [SMALL_STATE(627)] = 23995,
  [SMALL_STATE(628)] = 24006,
  [SMALL_STATE(629)] = 24017,
  [SMALL_STATE(630)] = 24028,
  [SMALL_STATE(631)] = 24039,
  [SMALL_STATE(632)] = 24050,
  [SMALL_STATE(633)] = 24058,
  [SMALL_STATE(634)] = 24066,
  [SMALL_STATE(635)] = 24074,
  [SMALL_STATE(636)] = 24082,
  [SMALL_STATE(637)] = 24090,
  [SMALL_STATE(638)] = 24098,
  [SMALL_STATE(639)] = 24106,
  [SMALL_STATE(640)] = 24114,
  [SMALL_STATE(641)] = 24122,
  [SMALL_STATE(642)] = 24130,
  [SMALL_STATE(643)] = 24138,
  [SMALL_STATE(644)] = 24146,
  [SMALL_STATE(645)] = 24154,
  [SMALL_STATE(646)] = 24162,
  [SMALL_STATE(647)] = 24170,
  [SMALL_STATE(648)] = 24178,
  [SMALL_STATE(649)] = 24186,
  [SMALL_STATE(650)] = 24194,
  [SMALL_STATE(651)] = 24202,
  [SMALL_STATE(652)] = 24210,
  [SMALL_STATE(653)] = 24218,
  [SMALL_STATE(654)] = 24226,
  [SMALL_STATE(655)] = 24234,
  [SMALL_STATE(656)] = 24242,
  [SMALL_STATE(657)] = 24250,
  [SMALL_STATE(658)] = 24258,
  [SMALL_STATE(659)] = 24266,
  [SMALL_STATE(660)] = 24274,
  [SMALL_STATE(661)] = 24282,
  [SMALL_STATE(662)] = 24290,
  [SMALL_STATE(663)] = 24298,
  [SMALL_STATE(664)] = 24306,
  [SMALL_STATE(665)] = 24314,
  [SMALL_STATE(666)] = 24322,
  [SMALL_STATE(667)] = 24330,
  [SMALL_STATE(668)] = 24338,
  [SMALL_STATE(669)] = 24346,
  [SMALL_STATE(670)] = 24354,
  [SMALL_STATE(671)] = 24362,
  [SMALL_STATE(672)] = 24370,
  [SMALL_STATE(673)] = 24378,
  [SMALL_STATE(674)] = 24386,
  [SMALL_STATE(675)] = 24394,
  [SMALL_STATE(676)] = 24402,
  [SMALL_STATE(677)] = 24410,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_config_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(352),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(406),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(351),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(175),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(186),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(187),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(192),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(350),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(657),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(658),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(658),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(365),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(251),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(308),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(336),
  [47] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(351),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__object_elems_repeat1, 2),
  [52] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(175),
  [55] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(83),
  [58] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(186),
  [61] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(186),
  [64] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(187),
  [67] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(192),
  [70] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(40),
  [73] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(350),
  [76] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(110),
  [79] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(658),
  [82] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(658),
  [85] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__object_elems_repeat1, 2), SHIFT_REPEAT(365),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__object_elems, 1),
  [90] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [92] = {.entry = {.count = 1, .reusable = true}}, SHIFT(248),
  [94] = {.entry = {.count = 1, .reusable = true}}, SHIFT(342),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__object_elems, 2),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(214),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(284),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [106] = {.entry = {.count = 1, .reusable = true}}, SHIFT(297),
  [108] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [110] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(341),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(267),
  [116] = {.entry = {.count = 1, .reusable = false}}, SHIFT(263),
  [118] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [120] = {.entry = {.count = 1, .reusable = false}}, SHIFT(335),
  [122] = {.entry = {.count = 1, .reusable = true}}, SHIFT(335),
  [124] = {.entry = {.count = 1, .reusable = false}}, SHIFT(334),
  [126] = {.entry = {.count = 1, .reusable = false}}, SHIFT(333),
  [128] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [130] = {.entry = {.count = 1, .reusable = true}}, SHIFT(659),
  [132] = {.entry = {.count = 1, .reusable = true}}, SHIFT(372),
  [134] = {.entry = {.count = 1, .reusable = true}}, SHIFT(456),
  [136] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [138] = {.entry = {.count = 1, .reusable = true}}, SHIFT(542),
  [140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(673),
  [142] = {.entry = {.count = 1, .reusable = true}}, SHIFT(499),
  [144] = {.entry = {.count = 1, .reusable = true}}, SHIFT(649),
  [146] = {.entry = {.count = 1, .reusable = true}}, SHIFT(492),
  [148] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [150] = {.entry = {.count = 1, .reusable = true}}, SHIFT(461),
  [152] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [154] = {.entry = {.count = 1, .reusable = true}}, SHIFT(470),
  [156] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__object_elems, 3),
  [158] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_operation, 2),
  [160] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_operation, 2),
  [162] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_arguments, 2),
  [164] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_arguments, 3),
  [166] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_operation, 3),
  [168] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_operation, 3),
  [170] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [172] = {.entry = {.count = 1, .reusable = false}}, SHIFT(120),
  [174] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [176] = {.entry = {.count = 1, .reusable = false}}, SHIFT(95),
  [178] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [180] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [182] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1),
  [184] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 1),
  [186] = {.entry = {.count = 1, .reusable = false}}, SHIFT(67),
  [188] = {.entry = {.count = 1, .reusable = false}}, SHIFT(592),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [194] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [196] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [198] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__tuple_elems, 2),
  [200] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__tuple_elems, 3),
  [202] = {.entry = {.count = 1, .reusable = false}}, SHIFT(250),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [206] = {.entry = {.count = 1, .reusable = false}}, SHIFT(307),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(307),
  [210] = {.entry = {.count = 1, .reusable = false}}, SHIFT(306),
  [212] = {.entry = {.count = 1, .reusable = false}}, SHIFT(304),
  [214] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [216] = {.entry = {.count = 1, .reusable = true}}, SHIFT(369),
  [218] = {.entry = {.count = 1, .reusable = false}}, SHIFT(247),
  [220] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [222] = {.entry = {.count = 1, .reusable = false}}, SHIFT(252),
  [224] = {.entry = {.count = 1, .reusable = true}}, SHIFT(252),
  [226] = {.entry = {.count = 1, .reusable = false}}, SHIFT(249),
  [228] = {.entry = {.count = 1, .reusable = false}}, SHIFT(253),
  [230] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [232] = {.entry = {.count = 1, .reusable = true}}, SHIFT(385),
  [234] = {.entry = {.count = 1, .reusable = false}}, SHIFT(141),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [238] = {.entry = {.count = 1, .reusable = false}}, SHIFT(147),
  [240] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [242] = {.entry = {.count = 1, .reusable = false}}, SHIFT(148),
  [244] = {.entry = {.count = 1, .reusable = false}}, SHIFT(151),
  [246] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(396),
  [250] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_full_splat, 1),
  [252] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_full_splat, 1),
  [254] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_full_splat, 2),
  [256] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_full_splat, 2),
  [258] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr_splat, 1),
  [260] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr_splat, 1),
  [262] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr_splat, 2),
  [264] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr_splat, 2),
  [266] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attr_splat_repeat1, 2),
  [268] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2),
  [270] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(67),
  [273] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(592),
  [276] = {.entry = {.count = 1, .reusable = false}}, SHIFT(61),
  [278] = {.entry = {.count = 1, .reusable = false}}, SHIFT(556),
  [280] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [288] = {.entry = {.count = 1, .reusable = false}}, SHIFT(122),
  [290] = {.entry = {.count = 1, .reusable = false}}, SHIFT(128),
  [292] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [298] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [300] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_expr, 1),
  [302] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable_expr, 1),
  [304] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [306] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(61),
  [309] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(556),
  [312] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 3),
  [314] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 3),
  [316] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_quoted_template, 3),
  [318] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_quoted_template, 3),
  [320] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_get_attr, 2),
  [322] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_get_attr, 2),
  [324] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_end, 1),
  [326] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_end, 1),
  [328] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_numeric_lit, 1),
  [330] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_numeric_lit, 1),
  [332] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bool_lit, 1),
  [334] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bool_lit, 1),
  [336] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_lit, 3),
  [338] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_lit, 3),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_object_expr, 8),
  [342] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_object_expr, 8),
  [344] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_literal_value, 1),
  [346] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_literal_value, 1),
  [348] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_collection_value, 1),
  [350] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_collection_value, 1),
  [352] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object, 2),
  [354] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object, 2),
  [356] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_legacy_index, 2),
  [358] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_legacy_index, 2),
  [360] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_object_expr, 7),
  [362] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_object_expr, 7),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_expr, 1),
  [366] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_expr, 1),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_object_expr, 6),
  [370] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_object_expr, 6),
  [372] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_operation, 1),
  [374] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_operation, 1),
  [376] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_tuple_expr, 5),
  [378] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_tuple_expr, 5),
  [380] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_index, 1),
  [382] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_index, 1),
  [384] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_expr, 1),
  [386] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_template_expr, 1),
  [388] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_heredoc_template, 4),
  [390] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_heredoc_template, 4),
  [392] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tuple_end, 1),
  [394] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tuple_end, 1),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_lit, 2),
  [398] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string_lit, 2),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tuple, 2),
  [402] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tuple, 2),
  [404] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_tuple_expr, 4),
  [406] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_tuple_expr, 4),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object, 3),
  [410] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object, 3),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_new_index, 3),
  [414] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_new_index, 3),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr_term, 2),
  [418] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr_term, 2),
  [420] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_splat, 1),
  [422] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_splat, 1),
  [424] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_call, 4),
  [426] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_call, 4),
  [428] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_heredoc_template, 3),
  [430] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_heredoc_template, 3),
  [432] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tuple, 3),
  [434] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tuple, 3),
  [436] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr_term, 3),
  [438] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr_term, 3),
  [440] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [442] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [444] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [446] = {.entry = {.count = 1, .reusable = false}}, SHIFT(113),
  [448] = {.entry = {.count = 1, .reusable = false}}, SHIFT(114),
  [450] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [452] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [454] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [456] = {.entry = {.count = 1, .reusable = false}}, SHIFT(85),
  [458] = {.entry = {.count = 1, .reusable = false}}, SHIFT(626),
  [460] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [462] = {.entry = {.count = 1, .reusable = true}}, SHIFT(231),
  [464] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [466] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [468] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [470] = {.entry = {.count = 1, .reusable = false}}, SHIFT(99),
  [472] = {.entry = {.count = 1, .reusable = false}}, SHIFT(68),
  [474] = {.entry = {.count = 1, .reusable = false}}, SHIFT(616),
  [476] = {.entry = {.count = 1, .reusable = true}}, SHIFT(245),
  [478] = {.entry = {.count = 1, .reusable = true}}, SHIFT(243),
  [480] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [482] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [484] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [486] = {.entry = {.count = 1, .reusable = false}}, SHIFT(101),
  [488] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [490] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [492] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [494] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [496] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(85),
  [499] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(626),
  [502] = {.entry = {.count = 1, .reusable = false}}, SHIFT(66),
  [504] = {.entry = {.count = 1, .reusable = false}}, SHIFT(617),
  [506] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [508] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [510] = {.entry = {.count = 1, .reusable = false}}, SHIFT(98),
  [512] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [514] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [516] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [518] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [520] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(66),
  [523] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(617),
  [526] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(68),
  [529] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr_splat_repeat1, 2), SHIFT_REPEAT(616),
  [532] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [534] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [536] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [538] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_conditional, 5),
  [540] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_conditional, 5),
  [542] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_elem, 3, .production_id = 1),
  [544] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_elem, 3, .production_id = 1),
  [546] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [548] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__object_elems_repeat1, 2),
  [550] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tuple_start, 1),
  [552] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_tuple_start, 1),
  [554] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_object_start, 1),
  [556] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_object_start, 1),
  [558] = {.entry = {.count = 1, .reusable = true}}, SHIFT(445),
  [560] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [562] = {.entry = {.count = 1, .reusable = true}}, SHIFT(430),
  [564] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_intro, 7),
  [566] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_intro, 7),
  [568] = {.entry = {.count = 1, .reusable = true}}, SHIFT(428),
  [570] = {.entry = {.count = 1, .reusable = true}}, SHIFT(421),
  [572] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_intro, 5),
  [574] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_intro, 5),
  [576] = {.entry = {.count = 1, .reusable = true}}, SHIFT(435),
  [578] = {.entry = {.count = 1, .reusable = true}}, SHIFT(425),
  [580] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [582] = {.entry = {.count = 1, .reusable = true}}, SHIFT(517),
  [584] = {.entry = {.count = 1, .reusable = true}}, SHIFT(313),
  [586] = {.entry = {.count = 1, .reusable = true}}, SHIFT(328),
  [588] = {.entry = {.count = 1, .reusable = true}}, SHIFT(423),
  [590] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [592] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [594] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__template, 2),
  [596] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__template, 2), SHIFT_REPEAT(423),
  [599] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__template, 2), SHIFT_REPEAT(26),
  [602] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__template, 2), SHIFT_REPEAT(517),
  [605] = {.entry = {.count = 1, .reusable = true}}, SHIFT(319),
  [607] = {.entry = {.count = 1, .reusable = true}}, SHIFT(486),
  [609] = {.entry = {.count = 1, .reusable = true}}, SHIFT(291),
  [611] = {.entry = {.count = 1, .reusable = true}}, SHIFT(453),
  [613] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__template, 2), SHIFT_REPEAT(425),
  [616] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__template, 2), SHIFT_REPEAT(27),
  [619] = {.entry = {.count = 1, .reusable = true}}, SHIFT(331),
  [621] = {.entry = {.count = 1, .reusable = true}}, SHIFT(276),
  [623] = {.entry = {.count = 1, .reusable = true}}, SHIFT(343),
  [625] = {.entry = {.count = 1, .reusable = true}}, SHIFT(489),
  [627] = {.entry = {.count = 1, .reusable = true}}, SHIFT(464),
  [629] = {.entry = {.count = 1, .reusable = true}}, SHIFT(482),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(272),
  [635] = {.entry = {.count = 1, .reusable = true}}, SHIFT(266),
  [637] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [639] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [641] = {.entry = {.count = 1, .reusable = true}}, SHIFT(259),
  [643] = {.entry = {.count = 1, .reusable = true}}, SHIFT(295),
  [645] = {.entry = {.count = 1, .reusable = true}}, SHIFT(305),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [649] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [651] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [653] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [655] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__template, 2), SHIFT_REPEAT(445),
  [658] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__template, 2), SHIFT_REPEAT(23),
  [661] = {.entry = {.count = 1, .reusable = true}}, SHIFT(506),
  [663] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [665] = {.entry = {.count = 1, .reusable = true}}, SHIFT(570),
  [667] = {.entry = {.count = 1, .reusable = true}}, SHIFT(413),
  [669] = {.entry = {.count = 1, .reusable = true}}, SHIFT(437),
  [671] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_arguments, 1),
  [673] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [675] = {.entry = {.count = 1, .reusable = true}}, SHIFT(663),
  [677] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [679] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [681] = {.entry = {.count = 1, .reusable = true}}, SHIFT(473),
  [683] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [685] = {.entry = {.count = 1, .reusable = true}}, SHIFT(454),
  [687] = {.entry = {.count = 1, .reusable = true}}, SHIFT(442),
  [689] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__tuple_elems_repeat1, 2),
  [691] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__tuple_elems_repeat1, 2), SHIFT_REPEAT(70),
  [694] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_body_repeat1, 2),
  [696] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_body_repeat1, 2), SHIFT_REPEAT(406),
  [699] = {.entry = {.count = 1, .reusable = true}}, SHIFT(417),
  [701] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_body, 1),
  [703] = {.entry = {.count = 1, .reusable = true}}, SHIFT(462),
  [705] = {.entry = {.count = 1, .reusable = true}}, SHIFT(450),
  [707] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2),
  [709] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(417),
  [712] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(437),
  [715] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [717] = {.entry = {.count = 1, .reusable = true}}, SHIFT(650),
  [719] = {.entry = {.count = 1, .reusable = true}}, SHIFT(652),
  [721] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [723] = {.entry = {.count = 1, .reusable = true}}, SHIFT(457),
  [725] = {.entry = {.count = 1, .reusable = true}}, SHIFT(557),
  [727] = {.entry = {.count = 1, .reusable = true}}, SHIFT(569),
  [729] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_literal, 1),
  [731] = {.entry = {.count = 1, .reusable = true}}, SHIFT(429),
  [733] = {.entry = {.count = 1, .reusable = true}}, SHIFT(434),
  [735] = {.entry = {.count = 1, .reusable = true}}, SHIFT(449),
  [737] = {.entry = {.count = 1, .reusable = true}}, SHIFT(606),
  [739] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_template_literal_repeat1, 2),
  [741] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_literal_repeat1, 2), SHIFT_REPEAT(429),
  [744] = {.entry = {.count = 1, .reusable = true}}, SHIFT(438),
  [746] = {.entry = {.count = 1, .reusable = true}}, SHIFT(559),
  [748] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [750] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__tuple_elems, 1),
  [752] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_literal_repeat1, 2), SHIFT_REPEAT(434),
  [755] = {.entry = {.count = 1, .reusable = true}}, SHIFT(537),
  [757] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if, 3),
  [759] = {.entry = {.count = 1, .reusable = true}}, SHIFT(522),
  [761] = {.entry = {.count = 1, .reusable = true}}, SHIFT(513),
  [763] = {.entry = {.count = 1, .reusable = true}}, SHIFT(672),
  [765] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [767] = {.entry = {.count = 1, .reusable = true}}, SHIFT(555),
  [769] = {.entry = {.count = 1, .reusable = true}}, SHIFT(608),
  [771] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for, 3),
  [773] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for_end, 3),
  [775] = {.entry = {.count = 1, .reusable = true}}, SHIFT(287),
  [777] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__template, 1),
  [779] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if, 5),
  [781] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for_end, 5),
  [783] = {.entry = {.count = 1, .reusable = true}}, SHIFT(469),
  [785] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_directive, 1),
  [787] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if_end, 4),
  [789] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if_end, 5),
  [791] = {.entry = {.count = 1, .reusable = true}}, SHIFT(614),
  [793] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for_end, 4),
  [795] = {.entry = {.count = 1, .reusable = true}}, SHIFT(541),
  [797] = {.entry = {.count = 1, .reusable = true}}, SHIFT(605),
  [799] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [801] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_interpolation, 3),
  [803] = {.entry = {.count = 1, .reusable = true}}, SHIFT(577),
  [805] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_interpolation, 5),
  [807] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_interpolation, 2),
  [809] = {.entry = {.count = 1, .reusable = true}}, SHIFT(529),
  [811] = {.entry = {.count = 1, .reusable = true}}, SHIFT(568),
  [813] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [815] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_literal_repeat1, 2), SHIFT_REPEAT(469),
  [818] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if, 4),
  [820] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if_end, 3),
  [822] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for, 2),
  [824] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_interpolation, 4),
  [826] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if, 2),
  [828] = {.entry = {.count = 1, .reusable = true}}, SHIFT(337),
  [830] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [832] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 3),
  [834] = {.entry = {.count = 1, .reusable = true}}, SHIFT(496),
  [836] = {.entry = {.count = 1, .reusable = true}}, SHIFT(574),
  [838] = {.entry = {.count = 1, .reusable = true}}, SHIFT(495),
  [840] = {.entry = {.count = 1, .reusable = true}}, SHIFT(281),
  [842] = {.entry = {.count = 1, .reusable = true}}, SHIFT(508),
  [844] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4),
  [846] = {.entry = {.count = 1, .reusable = true}}, SHIFT(621),
  [848] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [850] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_else_intro, 3),
  [852] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [854] = {.entry = {.count = 1, .reusable = true}}, SHIFT(643),
  [856] = {.entry = {.count = 1, .reusable = true}}, SHIFT(476),
  [858] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for_start, 7),
  [860] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block_end, 1),
  [862] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if_intro, 4),
  [864] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for_start, 8),
  [866] = {.entry = {.count = 1, .reusable = true}}, SHIFT(525),
  [868] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [870] = {.entry = {.count = 1, .reusable = true}}, SHIFT(646),
  [872] = {.entry = {.count = 1, .reusable = true}}, SHIFT(520),
  [874] = {.entry = {.count = 1, .reusable = true}}, SHIFT(594),
  [876] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for_start, 9),
  [878] = {.entry = {.count = 1, .reusable = true}}, SHIFT(647),
  [880] = {.entry = {.count = 1, .reusable = true}}, SHIFT(532),
  [882] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_template_literal_repeat1, 2), SHIFT_REPEAT(525),
  [885] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_cond, 2),
  [887] = {.entry = {.count = 1, .reusable = true}}, SHIFT(575),
  [889] = {.entry = {.count = 1, .reusable = true}}, SHIFT(677),
  [891] = {.entry = {.count = 1, .reusable = true}}, SHIFT(553),
  [893] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if_intro, 5),
  [895] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [897] = {.entry = {.count = 1, .reusable = true}}, SHIFT(636),
  [899] = {.entry = {.count = 1, .reusable = true}}, SHIFT(478),
  [901] = {.entry = {.count = 1, .reusable = true}}, SHIFT(654),
  [903] = {.entry = {.count = 1, .reusable = true}}, SHIFT(507),
  [905] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_else_intro, 4),
  [907] = {.entry = {.count = 1, .reusable = true}}, SHIFT(612),
  [909] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_else_intro, 5),
  [911] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for_start, 10),
  [913] = {.entry = {.count = 1, .reusable = true}}, SHIFT(669),
  [915] = {.entry = {.count = 1, .reusable = true}}, SHIFT(500),
  [917] = {.entry = {.count = 1, .reusable = true}}, SHIFT(635),
  [919] = {.entry = {.count = 1, .reusable = true}}, SHIFT(512),
  [921] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 5),
  [923] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_if_intro, 6),
  [925] = {.entry = {.count = 1, .reusable = true}}, SHIFT(655),
  [927] = {.entry = {.count = 1, .reusable = true}}, SHIFT(504),
  [929] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_template_for_start, 6),
  [931] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [933] = {.entry = {.count = 1, .reusable = true}}, SHIFT(666),
  [935] = {.entry = {.count = 1, .reusable = true}}, SHIFT(538),
  [937] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [939] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [941] = {.entry = {.count = 1, .reusable = true}}, SHIFT(644),
  [943] = {.entry = {.count = 1, .reusable = true}}, SHIFT(501),
  [945] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [947] = {.entry = {.count = 1, .reusable = true}}, SHIFT(645),
  [949] = {.entry = {.count = 1, .reusable = true}}, SHIFT(484),
  [951] = {.entry = {.count = 1, .reusable = true}}, SHIFT(316),
  [953] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [955] = {.entry = {.count = 1, .reusable = true}}, SHIFT(301),
  [957] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [959] = {.entry = {.count = 1, .reusable = true}}, SHIFT(668),
  [961] = {.entry = {.count = 1, .reusable = true}}, SHIFT(505),
  [963] = {.entry = {.count = 1, .reusable = true}}, SHIFT(667),
  [965] = {.entry = {.count = 1, .reusable = true}}, SHIFT(509),
  [967] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block_start, 1),
  [969] = {.entry = {.count = 1, .reusable = true}}, SHIFT(354),
  [971] = {.entry = {.count = 1, .reusable = true}}, SHIFT(639),
  [973] = {.entry = {.count = 1, .reusable = true}}, SHIFT(440),
  [975] = {.entry = {.count = 1, .reusable = true}}, SHIFT(664),
  [977] = {.entry = {.count = 1, .reusable = true}}, SHIFT(519),
  [979] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [981] = {.entry = {.count = 1, .reusable = true}}, SHIFT(661),
  [983] = {.entry = {.count = 1, .reusable = true}}, SHIFT(523),
  [985] = {.entry = {.count = 1, .reusable = true}}, SHIFT(273),
  [987] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [989] = {.entry = {.count = 1, .reusable = true}}, SHIFT(634),
  [991] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [993] = {.entry = {.count = 1, .reusable = true}}, SHIFT(359),
  [995] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [997] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [999] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [1001] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [1003] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [1005] = {.entry = {.count = 1, .reusable = true}}, SHIFT(314),
  [1007] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [1009] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [1011] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [1013] = {.entry = {.count = 1, .reusable = true}}, SHIFT(299),
  [1015] = {.entry = {.count = 1, .reusable = true}}, SHIFT(642),
  [1017] = {.entry = {.count = 1, .reusable = true}}, SHIFT(474),
  [1019] = {.entry = {.count = 1, .reusable = true}}, SHIFT(641),
  [1021] = {.entry = {.count = 1, .reusable = true}}, SHIFT(472),
  [1023] = {.entry = {.count = 1, .reusable = true}}, SHIFT(671),
  [1025] = {.entry = {.count = 1, .reusable = true}}, SHIFT(447),
  [1027] = {.entry = {.count = 1, .reusable = true}}, SHIFT(315),
  [1029] = {.entry = {.count = 1, .reusable = true}}, SHIFT(638),
  [1031] = {.entry = {.count = 1, .reusable = true}}, SHIFT(466),
  [1033] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [1035] = {.entry = {.count = 1, .reusable = true}}, SHIFT(637),
  [1037] = {.entry = {.count = 1, .reusable = true}}, SHIFT(465),
  [1039] = {.entry = {.count = 1, .reusable = true}}, SHIFT(317),
  [1041] = {.entry = {.count = 1, .reusable = true}}, SHIFT(344),
  [1043] = {.entry = {.count = 1, .reusable = true}}, SHIFT(309),
  [1045] = {.entry = {.count = 1, .reusable = true}}, SHIFT(310),
  [1047] = {.entry = {.count = 1, .reusable = true}}, SHIFT(302),
  [1049] = {.entry = {.count = 1, .reusable = true}}, SHIFT(660),
  [1051] = {.entry = {.count = 1, .reusable = true}}, SHIFT(451),
  [1053] = {.entry = {.count = 1, .reusable = true}}, SHIFT(632),
  [1055] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [1057] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [1059] = {.entry = {.count = 1, .reusable = true}}, SHIFT(268),
  [1061] = {.entry = {.count = 1, .reusable = true}}, SHIFT(269),
  [1063] = {.entry = {.count = 1, .reusable = true}}, SHIFT(270),
  [1065] = {.entry = {.count = 1, .reusable = true}}, SHIFT(662),
  [1067] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [1069] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [1071] = {.entry = {.count = 1, .reusable = true}}, SHIFT(274),
  [1073] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [1075] = {.entry = {.count = 1, .reusable = true}}, SHIFT(674),
  [1077] = {.entry = {.count = 1, .reusable = true}}, SHIFT(386),
  [1079] = {.entry = {.count = 1, .reusable = true}}, SHIFT(653),
  [1081] = {.entry = {.count = 1, .reusable = true}}, SHIFT(458),
  [1083] = {.entry = {.count = 1, .reusable = true}}, SHIFT(459),
  [1085] = {.entry = {.count = 1, .reusable = true}}, SHIFT(460),
  [1087] = {.entry = {.count = 1, .reusable = true}}, SHIFT(530),
  [1089] = {.entry = {.count = 1, .reusable = true}}, SHIFT(467),
  [1091] = {.entry = {.count = 1, .reusable = true}}, SHIFT(544),
  [1093] = {.entry = {.count = 1, .reusable = true}}, SHIFT(551),
  [1095] = {.entry = {.count = 1, .reusable = true}}, SHIFT(400),
  [1097] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [1099] = {.entry = {.count = 1, .reusable = true}}, SHIFT(583),
  [1101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [1103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(363),
  [1105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(628),
  [1107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_heredoc_start, 1),
  [1109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(444),
  [1111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(540),
  [1113] = {.entry = {.count = 1, .reusable = true}}, SHIFT(651),
  [1115] = {.entry = {.count = 1, .reusable = true}}, SHIFT(535),
  [1117] = {.entry = {.count = 1, .reusable = true}}, SHIFT(380),
  [1119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(543),
  [1121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(518),
  [1123] = {.entry = {.count = 1, .reusable = true}}, SHIFT(378),
  [1125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(448),
  [1127] = {.entry = {.count = 1, .reusable = true}}, SHIFT(623),
  [1129] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [1131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_config_file, 1),
  [1133] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

enum ts_external_scanner_symbol_identifiers {
  ts_external_token_quoted_template_start = 0,
  ts_external_token_quoted_template_end = 1,
  ts_external_token__template_literal_chunk = 2,
  ts_external_token_template_interpolation_start = 3,
  ts_external_token_template_interpolation_end = 4,
  ts_external_token_template_directive_start = 5,
  ts_external_token_template_directive_end = 6,
  ts_external_token_heredoc_identifier = 7,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token_quoted_template_start] = sym_quoted_template_start,
  [ts_external_token_quoted_template_end] = sym_quoted_template_end,
  [ts_external_token__template_literal_chunk] = sym__template_literal_chunk,
  [ts_external_token_template_interpolation_start] = sym_template_interpolation_start,
  [ts_external_token_template_interpolation_end] = sym_template_interpolation_end,
  [ts_external_token_template_directive_start] = sym_template_directive_start,
  [ts_external_token_template_directive_end] = sym_template_directive_end,
  [ts_external_token_heredoc_identifier] = sym_heredoc_identifier,
};

static const bool ts_external_scanner_states[12][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token_quoted_template_start] = true,
    [ts_external_token_quoted_template_end] = true,
    [ts_external_token__template_literal_chunk] = true,
    [ts_external_token_template_interpolation_start] = true,
    [ts_external_token_template_interpolation_end] = true,
    [ts_external_token_template_directive_start] = true,
    [ts_external_token_template_directive_end] = true,
    [ts_external_token_heredoc_identifier] = true,
  },
  [2] = {
    [ts_external_token_quoted_template_start] = true,
  },
  [3] = {
    [ts_external_token_quoted_template_start] = true,
    [ts_external_token_template_interpolation_end] = true,
  },
  [4] = {
    [ts_external_token_template_directive_end] = true,
  },
  [5] = {
    [ts_external_token_template_interpolation_end] = true,
  },
  [6] = {
    [ts_external_token__template_literal_chunk] = true,
    [ts_external_token_template_interpolation_start] = true,
    [ts_external_token_template_directive_start] = true,
  },
  [7] = {
    [ts_external_token__template_literal_chunk] = true,
    [ts_external_token_template_interpolation_start] = true,
    [ts_external_token_template_directive_start] = true,
    [ts_external_token_heredoc_identifier] = true,
  },
  [8] = {
    [ts_external_token_quoted_template_end] = true,
    [ts_external_token__template_literal_chunk] = true,
    [ts_external_token_template_interpolation_start] = true,
    [ts_external_token_template_directive_start] = true,
  },
  [9] = {
    [ts_external_token_quoted_template_end] = true,
    [ts_external_token__template_literal_chunk] = true,
  },
  [10] = {
    [ts_external_token_heredoc_identifier] = true,
  },
  [11] = {
    [ts_external_token_quoted_template_end] = true,
  },
};

#ifdef __cplusplus
extern "C" {
#endif
void *tree_sitter_terraform_external_scanner_create(void);
void tree_sitter_terraform_external_scanner_destroy(void *);
bool tree_sitter_terraform_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_terraform_external_scanner_serialize(void *, char *);
void tree_sitter_terraform_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_terraform(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .external_scanner = {
      &ts_external_scanner_states[0][0],
      ts_external_scanner_symbol_map,
      tree_sitter_terraform_external_scanner_create,
      tree_sitter_terraform_external_scanner_destroy,
      tree_sitter_terraform_external_scanner_scan,
      tree_sitter_terraform_external_scanner_serialize,
      tree_sitter_terraform_external_scanner_deserialize,
    },
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif

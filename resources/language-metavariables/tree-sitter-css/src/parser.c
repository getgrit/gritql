#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 312
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 108
#define ALIAS_COUNT 9
#define TOKEN_COUNT 53
#define EXTERNAL_TOKEN_COUNT 1
#define FIELD_COUNT 36
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 73

enum ts_symbol_identifiers {
  anon_sym_ATimport = 1,
  anon_sym_COMMA = 2,
  anon_sym_SEMI = 3,
  anon_sym_ATmedia = 4,
  anon_sym_ATcharset = 5,
  anon_sym_ATnamespace = 6,
  anon_sym_ATkeyframes = 7,
  aux_sym_keyframes_statement_token1 = 8,
  anon_sym_LBRACE = 9,
  anon_sym_RBRACE = 10,
  sym_from = 11,
  sym_to = 12,
  anon_sym_ATsupports = 13,
  sym_nesting_selector = 14,
  anon_sym_STAR = 15,
  anon_sym_DOT = 16,
  anon_sym_COLON = 17,
  anon_sym_COLON_COLON = 18,
  anon_sym_POUND = 19,
  sym_equal = 20,
  sym_contains_word_equal = 21,
  sym_starts_with_equal = 22,
  sym_dash_equal = 23,
  sym_contains_equal = 24,
  sym_ends_equal = 25,
  anon_sym_LBRACK = 26,
  anon_sym_RBRACK = 27,
  anon_sym_GT = 28,
  anon_sym_TILDE = 29,
  anon_sym_PLUS = 30,
  anon_sym_LPAREN = 31,
  anon_sym_RPAREN = 32,
  sym_important = 33,
  anon_sym_LPAREN2 = 34,
  sym_and = 35,
  sym_or = 36,
  sym_not = 37,
  sym_only = 38,
  anon_sym_selector = 39,
  aux_sym_color_value_token1 = 40,
  sym_string_value = 41,
  aux_sym_integer_value_token1 = 42,
  aux_sym_float_value_token1 = 43,
  sym_unit = 44,
  sym_minus = 45,
  sym_divide = 46,
  sym_identifier = 47,
  sym_at_keyword = 48,
  sym_comment = 49,
  sym_plain_value = 50,
  sym_grit_metavariable = 51,
  sym__descendant_operator = 52,
  sym_stylesheet = 53,
  sym_import_statement = 54,
  sym_media_statement = 55,
  sym_charset_statement = 56,
  sym_namespace_statement = 57,
  sym_keyframes_statement = 58,
  sym_keyframe_block_list = 59,
  sym_keyframe_block = 60,
  sym_supports_statement = 61,
  sym_at_rule = 62,
  sym_rule_set = 63,
  sym_selectors = 64,
  sym_block = 65,
  sym__selector = 66,
  sym_universal_selector = 67,
  sym_class_selector = 68,
  sym_pseudo_class_selector = 69,
  sym_pseudo_element_selector = 70,
  sym_id_selector = 71,
  sym_attribute_selector = 72,
  sym_child_selector = 73,
  sym_descendant_selector = 74,
  sym_sibling_selector = 75,
  sym_adjacent_sibling_selector = 76,
  sym_pseudo_class_arguments = 77,
  sym_pseudo_element_arguments = 78,
  sym_declaration = 79,
  sym_last_declaration = 80,
  sym__query = 81,
  sym_feature_query = 82,
  sym_parenthesized_query = 83,
  sym_binary_query = 84,
  sym_unary_query = 85,
  sym_selector_query = 86,
  sym__value = 87,
  sym_parenthesized_value = 88,
  sym_color_value = 89,
  sym_integer_value = 90,
  sym_float_value = 91,
  sym_call_expression = 92,
  sym_plus = 93,
  sym_times = 94,
  sym_binary_expression = 95,
  sym_arguments = 96,
  aux_sym_stylesheet_repeat1 = 97,
  aux_sym_import_statement_repeat1 = 98,
  aux_sym_media_statement_repeat1 = 99,
  aux_sym_keyframe_block_list_repeat1 = 100,
  aux_sym_at_rule_repeat1 = 101,
  aux_sym_selectors_repeat1 = 102,
  aux_sym_block_repeat1 = 103,
  aux_sym_pseudo_class_arguments_repeat1 = 104,
  aux_sym_pseudo_class_arguments_repeat2 = 105,
  aux_sym_declaration_repeat1 = 106,
  aux_sym_arguments_repeat1 = 107,
  alias_sym_class_name = 108,
  alias_sym_feature_name = 109,
  alias_sym_function_name = 110,
  alias_sym_id_name = 111,
  alias_sym_keyframes_name = 112,
  alias_sym_keyword_query = 113,
  alias_sym_namespace_name = 114,
  alias_sym_property_name = 115,
  alias_sym_tag_name = 116,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_ATimport] = "@import",
  [anon_sym_COMMA] = ",",
  [anon_sym_SEMI] = ";",
  [anon_sym_ATmedia] = "@media",
  [anon_sym_ATcharset] = "@charset",
  [anon_sym_ATnamespace] = "@namespace",
  [anon_sym_ATkeyframes] = "@keyframes",
  [aux_sym_keyframes_statement_token1] = "at_keyword",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [sym_from] = "from",
  [sym_to] = "to",
  [anon_sym_ATsupports] = "@supports",
  [sym_nesting_selector] = "nesting_selector",
  [anon_sym_STAR] = "*",
  [anon_sym_DOT] = ".",
  [anon_sym_COLON] = ":",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_POUND] = "#",
  [sym_equal] = "equal",
  [sym_contains_word_equal] = "contains_word_equal",
  [sym_starts_with_equal] = "starts_with_equal",
  [sym_dash_equal] = "dash_equal",
  [sym_contains_equal] = "contains_equal",
  [sym_ends_equal] = "ends_equal",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_GT] = ">",
  [anon_sym_TILDE] = "~",
  [anon_sym_PLUS] = "+",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_important] = "important",
  [anon_sym_LPAREN2] = "(",
  [sym_and] = "and",
  [sym_or] = "or",
  [sym_not] = "not",
  [sym_only] = "only",
  [anon_sym_selector] = "selector",
  [aux_sym_color_value_token1] = "color_value_token1",
  [sym_string_value] = "string_value",
  [aux_sym_integer_value_token1] = "integer_value_token1",
  [aux_sym_float_value_token1] = "float_value_token1",
  [sym_unit] = "unit",
  [sym_minus] = "minus",
  [sym_divide] = "divide",
  [sym_identifier] = "attribute_name",
  [sym_at_keyword] = "at_keyword",
  [sym_comment] = "comment",
  [sym_plain_value] = "plain_value",
  [sym_grit_metavariable] = "grit_metavariable",
  [sym__descendant_operator] = "_descendant_operator",
  [sym_stylesheet] = "stylesheet",
  [sym_import_statement] = "import_statement",
  [sym_media_statement] = "media_statement",
  [sym_charset_statement] = "charset_statement",
  [sym_namespace_statement] = "namespace_statement",
  [sym_keyframes_statement] = "keyframes_statement",
  [sym_keyframe_block_list] = "keyframe_block_list",
  [sym_keyframe_block] = "keyframe_block",
  [sym_supports_statement] = "supports_statement",
  [sym_at_rule] = "at_rule",
  [sym_rule_set] = "rule_set",
  [sym_selectors] = "selectors",
  [sym_block] = "block",
  [sym__selector] = "_selector",
  [sym_universal_selector] = "universal_selector",
  [sym_class_selector] = "class_selector",
  [sym_pseudo_class_selector] = "pseudo_class_selector",
  [sym_pseudo_element_selector] = "pseudo_element_selector",
  [sym_id_selector] = "id_selector",
  [sym_attribute_selector] = "attribute_selector",
  [sym_child_selector] = "child_selector",
  [sym_descendant_selector] = "descendant_selector",
  [sym_sibling_selector] = "sibling_selector",
  [sym_adjacent_sibling_selector] = "adjacent_sibling_selector",
  [sym_pseudo_class_arguments] = "arguments",
  [sym_pseudo_element_arguments] = "arguments",
  [sym_declaration] = "declaration",
  [sym_last_declaration] = "declaration",
  [sym__query] = "_query",
  [sym_feature_query] = "feature_query",
  [sym_parenthesized_query] = "parenthesized_query",
  [sym_binary_query] = "binary_query",
  [sym_unary_query] = "unary_query",
  [sym_selector_query] = "selector_query",
  [sym__value] = "_value",
  [sym_parenthesized_value] = "parenthesized_value",
  [sym_color_value] = "color_value",
  [sym_integer_value] = "integer_value",
  [sym_float_value] = "float_value",
  [sym_call_expression] = "call_expression",
  [sym_plus] = "plus",
  [sym_times] = "times",
  [sym_binary_expression] = "binary_expression",
  [sym_arguments] = "arguments",
  [aux_sym_stylesheet_repeat1] = "stylesheet_repeat1",
  [aux_sym_import_statement_repeat1] = "import_statement_repeat1",
  [aux_sym_media_statement_repeat1] = "media_statement_repeat1",
  [aux_sym_keyframe_block_list_repeat1] = "keyframe_block_list_repeat1",
  [aux_sym_at_rule_repeat1] = "at_rule_repeat1",
  [aux_sym_selectors_repeat1] = "selectors_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
  [aux_sym_pseudo_class_arguments_repeat1] = "pseudo_class_arguments_repeat1",
  [aux_sym_pseudo_class_arguments_repeat2] = "pseudo_class_arguments_repeat2",
  [aux_sym_declaration_repeat1] = "declaration_repeat1",
  [aux_sym_arguments_repeat1] = "arguments_repeat1",
  [alias_sym_class_name] = "class_name",
  [alias_sym_feature_name] = "feature_name",
  [alias_sym_function_name] = "function_name",
  [alias_sym_id_name] = "id_name",
  [alias_sym_keyframes_name] = "keyframes_name",
  [alias_sym_keyword_query] = "keyword_query",
  [alias_sym_namespace_name] = "namespace_name",
  [alias_sym_property_name] = "property_name",
  [alias_sym_tag_name] = "tag_name",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_ATimport] = anon_sym_ATimport,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_ATmedia] = anon_sym_ATmedia,
  [anon_sym_ATcharset] = anon_sym_ATcharset,
  [anon_sym_ATnamespace] = anon_sym_ATnamespace,
  [anon_sym_ATkeyframes] = anon_sym_ATkeyframes,
  [aux_sym_keyframes_statement_token1] = sym_at_keyword,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [sym_from] = sym_from,
  [sym_to] = sym_to,
  [anon_sym_ATsupports] = anon_sym_ATsupports,
  [sym_nesting_selector] = sym_nesting_selector,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_POUND] = anon_sym_POUND,
  [sym_equal] = sym_equal,
  [sym_contains_word_equal] = sym_contains_word_equal,
  [sym_starts_with_equal] = sym_starts_with_equal,
  [sym_dash_equal] = sym_dash_equal,
  [sym_contains_equal] = sym_contains_equal,
  [sym_ends_equal] = sym_ends_equal,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_important] = sym_important,
  [anon_sym_LPAREN2] = anon_sym_LPAREN,
  [sym_and] = sym_and,
  [sym_or] = sym_or,
  [sym_not] = sym_not,
  [sym_only] = sym_only,
  [anon_sym_selector] = anon_sym_selector,
  [aux_sym_color_value_token1] = aux_sym_color_value_token1,
  [sym_string_value] = sym_string_value,
  [aux_sym_integer_value_token1] = aux_sym_integer_value_token1,
  [aux_sym_float_value_token1] = aux_sym_float_value_token1,
  [sym_unit] = sym_unit,
  [sym_minus] = sym_minus,
  [sym_divide] = sym_divide,
  [sym_identifier] = sym_identifier,
  [sym_at_keyword] = sym_at_keyword,
  [sym_comment] = sym_comment,
  [sym_plain_value] = sym_plain_value,
  [sym_grit_metavariable] = sym_grit_metavariable,
  [sym__descendant_operator] = sym__descendant_operator,
  [sym_stylesheet] = sym_stylesheet,
  [sym_import_statement] = sym_import_statement,
  [sym_media_statement] = sym_media_statement,
  [sym_charset_statement] = sym_charset_statement,
  [sym_namespace_statement] = sym_namespace_statement,
  [sym_keyframes_statement] = sym_keyframes_statement,
  [sym_keyframe_block_list] = sym_keyframe_block_list,
  [sym_keyframe_block] = sym_keyframe_block,
  [sym_supports_statement] = sym_supports_statement,
  [sym_at_rule] = sym_at_rule,
  [sym_rule_set] = sym_rule_set,
  [sym_selectors] = sym_selectors,
  [sym_block] = sym_block,
  [sym__selector] = sym__selector,
  [sym_universal_selector] = sym_universal_selector,
  [sym_class_selector] = sym_class_selector,
  [sym_pseudo_class_selector] = sym_pseudo_class_selector,
  [sym_pseudo_element_selector] = sym_pseudo_element_selector,
  [sym_id_selector] = sym_id_selector,
  [sym_attribute_selector] = sym_attribute_selector,
  [sym_child_selector] = sym_child_selector,
  [sym_descendant_selector] = sym_descendant_selector,
  [sym_sibling_selector] = sym_sibling_selector,
  [sym_adjacent_sibling_selector] = sym_adjacent_sibling_selector,
  [sym_pseudo_class_arguments] = sym_arguments,
  [sym_pseudo_element_arguments] = sym_arguments,
  [sym_declaration] = sym_declaration,
  [sym_last_declaration] = sym_declaration,
  [sym__query] = sym__query,
  [sym_feature_query] = sym_feature_query,
  [sym_parenthesized_query] = sym_parenthesized_query,
  [sym_binary_query] = sym_binary_query,
  [sym_unary_query] = sym_unary_query,
  [sym_selector_query] = sym_selector_query,
  [sym__value] = sym__value,
  [sym_parenthesized_value] = sym_parenthesized_value,
  [sym_color_value] = sym_color_value,
  [sym_integer_value] = sym_integer_value,
  [sym_float_value] = sym_float_value,
  [sym_call_expression] = sym_call_expression,
  [sym_plus] = sym_plus,
  [sym_times] = sym_times,
  [sym_binary_expression] = sym_binary_expression,
  [sym_arguments] = sym_arguments,
  [aux_sym_stylesheet_repeat1] = aux_sym_stylesheet_repeat1,
  [aux_sym_import_statement_repeat1] = aux_sym_import_statement_repeat1,
  [aux_sym_media_statement_repeat1] = aux_sym_media_statement_repeat1,
  [aux_sym_keyframe_block_list_repeat1] = aux_sym_keyframe_block_list_repeat1,
  [aux_sym_at_rule_repeat1] = aux_sym_at_rule_repeat1,
  [aux_sym_selectors_repeat1] = aux_sym_selectors_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
  [aux_sym_pseudo_class_arguments_repeat1] = aux_sym_pseudo_class_arguments_repeat1,
  [aux_sym_pseudo_class_arguments_repeat2] = aux_sym_pseudo_class_arguments_repeat2,
  [aux_sym_declaration_repeat1] = aux_sym_declaration_repeat1,
  [aux_sym_arguments_repeat1] = aux_sym_arguments_repeat1,
  [alias_sym_class_name] = alias_sym_class_name,
  [alias_sym_feature_name] = alias_sym_feature_name,
  [alias_sym_function_name] = alias_sym_function_name,
  [alias_sym_id_name] = alias_sym_id_name,
  [alias_sym_keyframes_name] = alias_sym_keyframes_name,
  [alias_sym_keyword_query] = alias_sym_keyword_query,
  [alias_sym_namespace_name] = alias_sym_namespace_name,
  [alias_sym_property_name] = alias_sym_property_name,
  [alias_sym_tag_name] = alias_sym_tag_name,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_ATimport] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATmedia] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATcharset] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATnamespace] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ATkeyframes] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_keyframes_statement_token1] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [sym_from] = {
    .visible = true,
    .named = true,
  },
  [sym_to] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_ATsupports] = {
    .visible = true,
    .named = false,
  },
  [sym_nesting_selector] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [sym_equal] = {
    .visible = true,
    .named = true,
  },
  [sym_contains_word_equal] = {
    .visible = true,
    .named = true,
  },
  [sym_starts_with_equal] = {
    .visible = true,
    .named = true,
  },
  [sym_dash_equal] = {
    .visible = true,
    .named = true,
  },
  [sym_contains_equal] = {
    .visible = true,
    .named = true,
  },
  [sym_ends_equal] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_important] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LPAREN2] = {
    .visible = true,
    .named = false,
  },
  [sym_and] = {
    .visible = true,
    .named = true,
  },
  [sym_or] = {
    .visible = true,
    .named = true,
  },
  [sym_not] = {
    .visible = true,
    .named = true,
  },
  [sym_only] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_selector] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_color_value_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_string_value] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_integer_value_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_float_value_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_unit] = {
    .visible = true,
    .named = true,
  },
  [sym_minus] = {
    .visible = true,
    .named = true,
  },
  [sym_divide] = {
    .visible = true,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_at_keyword] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_plain_value] = {
    .visible = true,
    .named = true,
  },
  [sym_grit_metavariable] = {
    .visible = true,
    .named = true,
  },
  [sym__descendant_operator] = {
    .visible = false,
    .named = true,
  },
  [sym_stylesheet] = {
    .visible = true,
    .named = true,
  },
  [sym_import_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_media_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_charset_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_namespace_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_keyframes_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_keyframe_block_list] = {
    .visible = true,
    .named = true,
  },
  [sym_keyframe_block] = {
    .visible = true,
    .named = true,
  },
  [sym_supports_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_at_rule] = {
    .visible = true,
    .named = true,
  },
  [sym_rule_set] = {
    .visible = true,
    .named = true,
  },
  [sym_selectors] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym__selector] = {
    .visible = false,
    .named = true,
  },
  [sym_universal_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_class_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_pseudo_class_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_pseudo_element_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_id_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_child_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_descendant_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_sibling_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_adjacent_sibling_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_pseudo_class_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_pseudo_element_arguments] = {
    .visible = true,
    .named = true,
  },
  [sym_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_last_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym__query] = {
    .visible = false,
    .named = true,
  },
  [sym_feature_query] = {
    .visible = true,
    .named = true,
  },
  [sym_parenthesized_query] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_query] = {
    .visible = true,
    .named = true,
  },
  [sym_unary_query] = {
    .visible = true,
    .named = true,
  },
  [sym_selector_query] = {
    .visible = true,
    .named = true,
  },
  [sym__value] = {
    .visible = false,
    .named = true,
  },
  [sym_parenthesized_value] = {
    .visible = true,
    .named = true,
  },
  [sym_color_value] = {
    .visible = true,
    .named = true,
  },
  [sym_integer_value] = {
    .visible = true,
    .named = true,
  },
  [sym_float_value] = {
    .visible = true,
    .named = true,
  },
  [sym_call_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_plus] = {
    .visible = true,
    .named = true,
  },
  [sym_times] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_arguments] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_stylesheet_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_import_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_media_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_keyframe_block_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_at_rule_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_selectors_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_pseudo_class_arguments_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_pseudo_class_arguments_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_declaration_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_arguments_repeat1] = {
    .visible = false,
    .named = false,
  },
  [alias_sym_class_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_feature_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_function_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_id_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_keyframes_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_keyword_query] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_namespace_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_property_name] = {
    .visible = true,
    .named = true,
  },
  [alias_sym_tag_name] = {
    .visible = true,
    .named = true,
  },
};

enum ts_field_identifiers {
  field_ancestor = 1,
  field_annotation = 2,
  field_arguments = 3,
  field_attribute = 4,
  field_blocks = 5,
  field_body = 6,
  field_charset = 7,
  field_child = 8,
  field_class = 9,
  field_declaration = 10,
  field_descendant = 11,
  field_feature = 12,
  field_first = 13,
  field_from = 14,
  field_important = 15,
  field_item = 16,
  field_items = 17,
  field_keyframes = 18,
  field_left = 19,
  field_media_type = 20,
  field_name = 21,
  field_namespace = 22,
  field_offset = 23,
  field_operator = 24,
  field_parent = 25,
  field_primary = 26,
  field_query = 27,
  field_right = 28,
  field_rule = 29,
  field_second = 30,
  field_selector = 31,
  field_selector_type = 32,
  field_selectors = 33,
  field_sibling = 34,
  field_value = 35,
  field_values = 36,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_ancestor] = "ancestor",
  [field_annotation] = "annotation",
  [field_arguments] = "arguments",
  [field_attribute] = "attribute",
  [field_blocks] = "blocks",
  [field_body] = "body",
  [field_charset] = "charset",
  [field_child] = "child",
  [field_class] = "class",
  [field_declaration] = "declaration",
  [field_descendant] = "descendant",
  [field_feature] = "feature",
  [field_first] = "first",
  [field_from] = "from",
  [field_important] = "important",
  [field_item] = "item",
  [field_items] = "items",
  [field_keyframes] = "keyframes",
  [field_left] = "left",
  [field_media_type] = "media_type",
  [field_name] = "name",
  [field_namespace] = "namespace",
  [field_offset] = "offset",
  [field_operator] = "operator",
  [field_parent] = "parent",
  [field_primary] = "primary",
  [field_query] = "query",
  [field_right] = "right",
  [field_rule] = "rule",
  [field_second] = "second",
  [field_selector] = "selector",
  [field_selector_type] = "selector_type",
  [field_selectors] = "selectors",
  [field_sibling] = "sibling",
  [field_value] = "value",
  [field_values] = "values",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [2] = {.index = 0, .length = 1},
  [3] = {.index = 1, .length = 1},
  [4] = {.index = 2, .length = 1},
  [7] = {.index = 3, .length = 1},
  [8] = {.index = 4, .length = 1},
  [9] = {.index = 4, .length = 1},
  [10] = {.index = 5, .length = 1},
  [11] = {.index = 6, .length = 2},
  [12] = {.index = 8, .length = 2},
  [13] = {.index = 10, .length = 2},
  [14] = {.index = 12, .length = 2},
  [15] = {.index = 14, .length = 2},
  [16] = {.index = 16, .length = 1},
  [17] = {.index = 17, .length = 2},
  [18] = {.index = 19, .length = 2},
  [19] = {.index = 21, .length = 1},
  [20] = {.index = 22, .length = 3},
  [21] = {.index = 25, .length = 2},
  [22] = {.index = 27, .length = 2},
  [23] = {.index = 29, .length = 2},
  [24] = {.index = 31, .length = 1},
  [25] = {.index = 32, .length = 2},
  [26] = {.index = 34, .length = 3},
  [27] = {.index = 37, .length = 2},
  [28] = {.index = 39, .length = 1},
  [29] = {.index = 40, .length = 2},
  [30] = {.index = 42, .length = 2},
  [31] = {.index = 42, .length = 2},
  [32] = {.index = 44, .length = 2},
  [33] = {.index = 46, .length = 2},
  [34] = {.index = 48, .length = 2},
  [35] = {.index = 50, .length = 2},
  [36] = {.index = 52, .length = 3},
  [37] = {.index = 55, .length = 2},
  [38] = {.index = 57, .length = 1},
  [39] = {.index = 58, .length = 1},
  [40] = {.index = 59, .length = 2},
  [41] = {.index = 61, .length = 3},
  [42] = {.index = 64, .length = 2},
  [43] = {.index = 66, .length = 2},
  [44] = {.index = 68, .length = 2},
  [45] = {.index = 70, .length = 1},
  [46] = {.index = 71, .length = 1},
  [47] = {.index = 72, .length = 1},
  [48] = {.index = 73, .length = 3},
  [49] = {.index = 76, .length = 4},
  [50] = {.index = 80, .length = 2},
  [51] = {.index = 82, .length = 3},
  [52] = {.index = 85, .length = 3},
  [53] = {.index = 88, .length = 2},
  [54] = {.index = 90, .length = 1},
  [55] = {.index = 91, .length = 1},
  [56] = {.index = 92, .length = 3},
  [57] = {.index = 95, .length = 2},
  [58] = {.index = 97, .length = 1},
  [59] = {.index = 98, .length = 2},
  [60] = {.index = 100, .length = 1},
  [61] = {.index = 101, .length = 1},
  [62] = {.index = 102, .length = 3},
  [63] = {.index = 105, .length = 3},
  [64] = {.index = 108, .length = 3},
  [65] = {.index = 111, .length = 2},
  [66] = {.index = 113, .length = 2},
  [67] = {.index = 115, .length = 2},
  [68] = {.index = 117, .length = 2},
  [69] = {.index = 119, .length = 2},
  [70] = {.index = 121, .length = 2},
  [71] = {.index = 123, .length = 4},
  [72] = {.index = 127, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_items, 0},
  [1] =
    {field_selectors, 0},
  [2] =
    {field_items, 0, .inherited = true},
  [3] =
    {field_class, 1},
  [4] =
    {field_name, 1},
  [5] =
    {field_rule, 0},
  [6] =
    {field_body, 1},
    {field_rule, 0},
  [8] =
    {field_body, 1},
    {field_selectors, 0},
  [10] =
    {field_selectors, 0},
    {field_selectors, 1, .inherited = true},
  [12] =
    {field_items, 0, .inherited = true},
    {field_items, 1, .inherited = true},
  [14] =
    {field_arguments, 1},
    {field_name, 0},
  [16] =
    {field_value, 1},
  [17] =
    {field_operator, 0},
    {field_query, 1},
  [19] =
    {field_body, 2},
    {field_media_type, 1},
  [21] =
    {field_charset, 1},
  [22] =
    {field_annotation, 0},
    {field_blocks, 2},
    {field_name, 1},
  [25] =
    {field_body, 2},
    {field_feature, 1},
  [27] =
    {field_arguments, 2},
    {field_class, 1},
  [29] =
    {field_arguments, 2},
    {field_name, 1},
  [31] =
    {field_attribute, 1},
  [32] =
    {field_query, 1},
    {field_rule, 0},
  [34] =
    {field_body, 2},
    {field_query, 1},
    {field_rule, 0},
  [37] =
    {field_ancestor, 0},
    {field_descendant, 2},
  [39] =
    {field_selectors, 1},
  [40] =
    {field_class, 2},
    {field_selector, 0},
  [42] =
    {field_name, 2},
    {field_selector, 0},
  [44] =
    {field_child, 2},
    {field_parent, 0},
  [46] =
    {field_primary, 2},
    {field_sibling, 0},
  [48] =
    {field_first, 0},
    {field_second, 2},
  [50] =
    {field_selectors, 0, .inherited = true},
    {field_selectors, 1, .inherited = true},
  [52] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
  [55] =
    {field_from, 2},
    {field_value, 1},
  [57] =
    {field_query, 1},
  [58] =
    {field_media_type, 1},
  [59] =
    {field_operator, 1},
    {field_query, 2},
  [61] =
    {field_body, 3},
    {field_media_type, 1},
    {field_media_type, 2, .inherited = true},
  [64] =
    {field_media_type, 0, .inherited = true},
    {field_media_type, 1, .inherited = true},
  [66] =
    {field_namespace, 1},
    {field_value, 2},
  [68] =
    {field_name, 0},
    {field_values, 2},
  [70] =
    {field_values, 0},
  [71] =
    {field_declaration, 1},
  [72] =
    {field_item, 1},
  [73] =
    {field_query, 1},
    {field_query, 2, .inherited = true},
    {field_rule, 0},
  [76] =
    {field_body, 3},
    {field_query, 1},
    {field_query, 2, .inherited = true},
    {field_rule, 0},
  [80] =
    {field_query, 0, .inherited = true},
    {field_query, 1, .inherited = true},
  [82] =
    {field_arguments, 3},
    {field_class, 2},
    {field_selector, 0},
  [85] =
    {field_arguments, 3},
    {field_name, 2},
    {field_selector, 0},
  [88] =
    {field_attribute, 2},
    {field_selector, 0},
  [90] =
    {field_values, 1},
  [91] =
    {field_from, 1},
  [92] =
    {field_from, 2},
    {field_from, 3, .inherited = true},
    {field_value, 1},
  [95] =
    {field_from, 0, .inherited = true},
    {field_from, 1, .inherited = true},
  [97] =
    {field_selector, 2},
  [98] =
    {field_body, 1},
    {field_offset, 0},
  [100] =
    {field_keyframes, 1},
  [101] =
    {field_arguments, 1},
  [102] =
    {field_attribute, 1},
    {field_selector_type, 2},
    {field_value, 3},
  [105] =
    {field_important, 3},
    {field_name, 0},
    {field_values, 2},
  [108] =
    {field_name, 0},
    {field_values, 2},
    {field_values, 3, .inherited = true},
  [111] =
    {field_values, 0, .inherited = true},
    {field_values, 1, .inherited = true},
  [113] =
    {field_declaration, 2},
    {field_item, 1},
  [115] =
    {field_values, 1},
    {field_values, 2, .inherited = true},
  [117] =
    {field_name, 1},
    {field_value, 3},
  [119] =
    {field_arguments, 1},
    {field_arguments, 2, .inherited = true},
  [121] =
    {field_arguments, 0, .inherited = true},
    {field_arguments, 1, .inherited = true},
  [123] =
    {field_important, 4},
    {field_name, 0},
    {field_values, 2},
    {field_values, 3, .inherited = true},
  [127] =
    {field_attribute, 2},
    {field_selector, 0},
    {field_selector_type, 3},
    {field_value, 4},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
  [1] = {
    [0] = alias_sym_tag_name,
  },
  [5] = {
    [0] = sym_plain_value,
  },
  [6] = {
    [0] = alias_sym_keyword_query,
  },
  [7] = {
    [1] = alias_sym_class_name,
  },
  [8] = {
    [1] = alias_sym_tag_name,
  },
  [9] = {
    [1] = alias_sym_id_name,
  },
  [15] = {
    [0] = alias_sym_function_name,
  },
  [20] = {
    [1] = alias_sym_keyframes_name,
  },
  [22] = {
    [1] = alias_sym_class_name,
  },
  [23] = {
    [1] = alias_sym_tag_name,
  },
  [29] = {
    [2] = alias_sym_class_name,
  },
  [30] = {
    [2] = alias_sym_tag_name,
  },
  [31] = {
    [2] = alias_sym_id_name,
  },
  [43] = {
    [1] = alias_sym_namespace_name,
  },
  [44] = {
    [0] = alias_sym_property_name,
  },
  [51] = {
    [2] = alias_sym_class_name,
  },
  [52] = {
    [2] = alias_sym_tag_name,
  },
  [63] = {
    [0] = alias_sym_property_name,
  },
  [64] = {
    [0] = alias_sym_property_name,
  },
  [68] = {
    [1] = alias_sym_feature_name,
  },
  [71] = {
    [0] = alias_sym_property_name,
  },
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 2,
  [5] = 3,
  [6] = 3,
  [7] = 2,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 15,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 26,
  [29] = 29,
  [30] = 26,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 31,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 38,
  [45] = 45,
  [46] = 40,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 52,
  [61] = 57,
  [62] = 58,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 59,
  [68] = 55,
  [69] = 54,
  [70] = 53,
  [71] = 71,
  [72] = 34,
  [73] = 65,
  [74] = 49,
  [75] = 33,
  [76] = 76,
  [77] = 39,
  [78] = 37,
  [79] = 41,
  [80] = 42,
  [81] = 43,
  [82] = 45,
  [83] = 76,
  [84] = 51,
  [85] = 85,
  [86] = 85,
  [87] = 66,
  [88] = 71,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 91,
  [93] = 90,
  [94] = 89,
  [95] = 32,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 103,
  [106] = 106,
  [107] = 107,
  [108] = 103,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 116,
  [120] = 120,
  [121] = 121,
  [122] = 118,
  [123] = 123,
  [124] = 121,
  [125] = 120,
  [126] = 126,
  [127] = 116,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 117,
  [132] = 118,
  [133] = 130,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 64,
  [139] = 50,
  [140] = 140,
  [141] = 106,
  [142] = 142,
  [143] = 96,
  [144] = 144,
  [145] = 145,
  [146] = 99,
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
  [157] = 155,
  [158] = 158,
  [159] = 159,
  [160] = 160,
  [161] = 161,
  [162] = 162,
  [163] = 163,
  [164] = 164,
  [165] = 149,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 169,
  [170] = 170,
  [171] = 171,
  [172] = 172,
  [173] = 173,
  [174] = 174,
  [175] = 175,
  [176] = 176,
  [177] = 177,
  [178] = 178,
  [179] = 179,
  [180] = 180,
  [181] = 181,
  [182] = 104,
  [183] = 183,
  [184] = 184,
  [185] = 111,
  [186] = 183,
  [187] = 112,
  [188] = 101,
  [189] = 107,
  [190] = 109,
  [191] = 191,
  [192] = 110,
  [193] = 193,
  [194] = 194,
  [195] = 50,
  [196] = 64,
  [197] = 197,
  [198] = 198,
  [199] = 99,
  [200] = 96,
  [201] = 201,
  [202] = 106,
  [203] = 111,
  [204] = 204,
  [205] = 205,
  [206] = 206,
  [207] = 205,
  [208] = 208,
  [209] = 209,
  [210] = 201,
  [211] = 104,
  [212] = 110,
  [213] = 107,
  [214] = 109,
  [215] = 215,
  [216] = 205,
  [217] = 217,
  [218] = 209,
  [219] = 219,
  [220] = 101,
  [221] = 208,
  [222] = 222,
  [223] = 204,
  [224] = 112,
  [225] = 225,
  [226] = 226,
  [227] = 227,
  [228] = 228,
  [229] = 229,
  [230] = 230,
  [231] = 231,
  [232] = 225,
  [233] = 233,
  [234] = 234,
  [235] = 234,
  [236] = 236,
  [237] = 237,
  [238] = 237,
  [239] = 236,
  [240] = 240,
  [241] = 241,
  [242] = 242,
  [243] = 243,
  [244] = 240,
  [245] = 243,
  [246] = 240,
  [247] = 247,
  [248] = 248,
  [249] = 37,
  [250] = 250,
  [251] = 251,
  [252] = 31,
  [253] = 89,
  [254] = 250,
  [255] = 55,
  [256] = 256,
  [257] = 257,
  [258] = 258,
  [259] = 259,
  [260] = 260,
  [261] = 261,
  [262] = 262,
  [263] = 263,
  [264] = 263,
  [265] = 265,
  [266] = 266,
  [267] = 267,
  [268] = 268,
  [269] = 268,
  [270] = 270,
  [271] = 271,
  [272] = 272,
  [273] = 273,
  [274] = 272,
  [275] = 275,
  [276] = 276,
  [277] = 277,
  [278] = 278,
  [279] = 276,
  [280] = 96,
  [281] = 281,
  [282] = 282,
  [283] = 282,
  [284] = 284,
  [285] = 285,
  [286] = 286,
  [287] = 287,
  [288] = 288,
  [289] = 289,
  [290] = 290,
  [291] = 284,
  [292] = 288,
  [293] = 293,
  [294] = 294,
  [295] = 295,
  [296] = 296,
  [297] = 297,
  [298] = 298,
  [299] = 289,
  [300] = 300,
  [301] = 301,
  [302] = 282,
  [303] = 289,
  [304] = 304,
  [305] = 285,
  [306] = 287,
  [307] = 307,
  [308] = 308,
  [309] = 296,
  [310] = 288,
  [311] = 308,
};

static inline bool sym_plain_value_character_set_1(int32_t c) {
  return (c < ';'
    ? (c < ' '
      ? (c < '\t'
        ? c == 0
        : c <= '\r')
      : (c <= '!' || (c < ','
        ? (c >= '(' && c <= '*')
        : c <= ',')))
    : (c <= ';' || (c < '{'
      ? (c < ']'
        ? c == '['
        : c <= ']')
      : (c <= '{' || c == '}'))));
}

static inline bool sym_plain_value_character_set_2(int32_t c) {
  return (c < ';'
    ? (c < ' '
      ? (c < '\t'
        ? c == 0
        : c <= '\r')
      : (c <= '!' || (c < ','
        ? (c >= '(' && c <= ')')
        : c <= ',')))
    : (c <= ';' || (c < '{'
      ? (c < ']'
        ? c == '['
        : c <= ']')
      : (c <= '{' || c == '}'))));
}

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(77);
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '"') ADVANCE(6);
      if (lookahead == '#') ADVANCE(98);
      if (lookahead == '$') ADVANCE(29);
      if (lookahead == '&') ADVANCE(91);
      if (lookahead == '\'') ADVANCE(9);
      if (lookahead == '(') ADVANCE(112);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(93);
      if (lookahead == '+') ADVANCE(111);
      if (lookahead == ',') ADVANCE(79);
      if (lookahead == '-') ADVANCE(186);
      if (lookahead == '.') ADVANCE(95);
      if (lookahead == '/') ADVANCE(189);
      if (lookahead == ':') ADVANCE(96);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == '=') ADVANCE(99);
      if (lookahead == '>') ADVANCE(107);
      if (lookahead == '@') ADVANCE(35);
      if (lookahead == '[') ADVANCE(105);
      if (lookahead == ']') ADVANCE(106);
      if (lookahead == '^') ADVANCE(31);
      if (lookahead == 'a') ADVANCE(46);
      if (lookahead == 'f') ADVANCE(57);
      if (lookahead == 'n') ADVANCE(49);
      if (lookahead == 'o') ADVANCE(47);
      if (lookahead == 's') ADVANCE(39);
      if (lookahead == 't') ADVANCE(50);
      if (lookahead == '{') ADVANCE(86);
      if (lookahead == '|') ADVANCE(32);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == '~') ADVANCE(109);
      if (lookahead == 181) ADVANCE(23);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(20);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(74)
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('b' <= lookahead && lookahead <= 'd')) ADVANCE(68);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(162);
      END_STATE();
    case 1:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '"') ADVANCE(6);
      if (lookahead == '#') ADVANCE(98);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == '\'') ADVANCE(9);
      if (lookahead == '(') ADVANCE(115);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(111);
      if (lookahead == ',') ADVANCE(79);
      if (lookahead == '-') ADVANCE(187);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == '/') ADVANCE(190);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == '_') ADVANCE(192);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == 181) ADVANCE(23);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(181);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(182);
      END_STATE();
    case 2:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '"') ADVANCE(6);
      if (lookahead == '#') ADVANCE(98);
      if (lookahead == '&') ADVANCE(91);
      if (lookahead == '\'') ADVANCE(9);
      if (lookahead == '(') ADVANCE(115);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(111);
      if (lookahead == ',') ADVANCE(79);
      if (lookahead == '-') ADVANCE(187);
      if (lookahead == '.') ADVANCE(95);
      if (lookahead == '/') ADVANCE(190);
      if (lookahead == ':') ADVANCE(96);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == '>') ADVANCE(107);
      if (lookahead == '[') ADVANCE(105);
      if (lookahead == '_') ADVANCE(192);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == '~') ADVANCE(108);
      if (lookahead == 181) ADVANCE(23);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(191);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      END_STATE();
    case 3:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '"') ADVANCE(6);
      if (lookahead == '#') ADVANCE(98);
      if (lookahead == '\'') ADVANCE(9);
      if (lookahead == '(') ADVANCE(112);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(111);
      if (lookahead == ',') ADVANCE(79);
      if (lookahead == '-') ADVANCE(187);
      if (lookahead == '.') ADVANCE(95);
      if (lookahead == '/') ADVANCE(190);
      if (lookahead == ':') ADVANCE(96);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == '>') ADVANCE(107);
      if (lookahead == '[') ADVANCE(105);
      if (lookahead == '_') ADVANCE(192);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == '~') ADVANCE(108);
      if (lookahead == 181) ADVANCE(23);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(191);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(4)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      END_STATE();
    case 4:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '"') ADVANCE(6);
      if (lookahead == '#') ADVANCE(98);
      if (lookahead == '\'') ADVANCE(9);
      if (lookahead == '(') ADVANCE(115);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(111);
      if (lookahead == ',') ADVANCE(79);
      if (lookahead == '-') ADVANCE(187);
      if (lookahead == '.') ADVANCE(95);
      if (lookahead == '/') ADVANCE(190);
      if (lookahead == ':') ADVANCE(96);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == '>') ADVANCE(107);
      if (lookahead == '[') ADVANCE(105);
      if (lookahead == '_') ADVANCE(192);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == '~') ADVANCE(108);
      if (lookahead == 181) ADVANCE(23);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(191);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(4)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      END_STATE();
    case 5:
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '"') ADVANCE(6);
      if (lookahead == '#') ADVANCE(98);
      if (lookahead == '\'') ADVANCE(9);
      if (lookahead == '(') ADVANCE(115);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(111);
      if (lookahead == ',') ADVANCE(79);
      if (lookahead == '-') ADVANCE(187);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == '/') ADVANCE(190);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == '_') ADVANCE(192);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == 181) ADVANCE(23);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(191);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      END_STATE();
    case 6:
      if (lookahead == '"') ADVANCE(157);
      if (lookahead == '\\') ADVANCE(72);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(6);
      END_STATE();
    case 7:
      if (lookahead == '$') ADVANCE(29);
      if (lookahead == '*') ADVANCE(30);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(66);
      if (lookahead == '/') ADVANCE(15);
      if (lookahead == '=') ADVANCE(99);
      if (lookahead == ']') ADVANCE(106);
      if (lookahead == '^') ADVANCE(31);
      if (lookahead == 'f') ADVANCE(56);
      if (lookahead == 't') ADVANCE(50);
      if (lookahead == '|') ADVANCE(32);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == '~') ADVANCE(33);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(7)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(163);
      END_STATE();
    case 8:
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == '(') ADVANCE(115);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(110);
      if (lookahead == '-') ADVANCE(188);
      if (lookahead == '/') ADVANCE(189);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == '_') ADVANCE(209);
      if (lookahead == 'n') ADVANCE(174);
      if (lookahead == 'o') ADVANCE(173);
      if (lookahead == 's') ADVANCE(170);
      if (lookahead == 181) ADVANCE(23);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(13)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 9:
      if (lookahead == '\'') ADVANCE(157);
      if (lookahead == '\\') ADVANCE(73);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(9);
      END_STATE();
    case 10:
      if (lookahead == '(') ADVANCE(112);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(110);
      if (lookahead == '-') ADVANCE(185);
      if (lookahead == '/') ADVANCE(189);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == ']') ADVANCE(106);
      if (lookahead == '{') ADVANCE(86);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(14)
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 11:
      if (lookahead == '(') ADVANCE(112);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(110);
      if (lookahead == '-') ADVANCE(188);
      if (lookahead == '/') ADVANCE(189);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == 'n') ADVANCE(201);
      if (lookahead == 'o') ADVANCE(200);
      if (lookahead == 's') ADVANCE(197);
      if (lookahead == 181) ADVANCE(23);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(13)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 12:
      if (lookahead == '(') ADVANCE(115);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(110);
      if (lookahead == '-') ADVANCE(188);
      if (lookahead == '/') ADVANCE(189);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == 'n') ADVANCE(201);
      if (lookahead == 'o') ADVANCE(200);
      if (lookahead == 's') ADVANCE(197);
      if (lookahead == '{') ADVANCE(86);
      if (lookahead == 181) ADVANCE(23);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(12)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 13:
      if (lookahead == '(') ADVANCE(115);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(110);
      if (lookahead == '-') ADVANCE(188);
      if (lookahead == '/') ADVANCE(189);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == 'n') ADVANCE(201);
      if (lookahead == 'o') ADVANCE(200);
      if (lookahead == 's') ADVANCE(197);
      if (lookahead == 181) ADVANCE(23);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(13)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 14:
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(110);
      if (lookahead == '-') ADVANCE(185);
      if (lookahead == '/') ADVANCE(189);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == ']') ADVANCE(106);
      if (lookahead == '{') ADVANCE(86);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(14)
      END_STATE();
    case 15:
      if (lookahead == '*') ADVANCE(17);
      END_STATE();
    case 16:
      if (lookahead == '*') ADVANCE(16);
      if (lookahead == '/') ADVANCE(258);
      if (lookahead != 0) ADVANCE(17);
      END_STATE();
    case 17:
      if (lookahead == '*') ADVANCE(16);
      if (lookahead != 0) ADVANCE(17);
      END_STATE();
    case 18:
      if (lookahead == '-') ADVANCE(69);
      if (lookahead == '/') ADVANCE(15);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(19)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(208);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 19:
      if (lookahead == '-') ADVANCE(69);
      if (lookahead == '/') ADVANCE(15);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(19)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 20:
      if (lookahead == '-') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(167);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(67);
      END_STATE();
    case 21:
      if (lookahead == '-') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(166);
      END_STATE();
    case 22:
      if (lookahead == '-') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(145);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(154);
      END_STATE();
    case 23:
      if (lookahead == '.') ADVANCE(25);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(261);
      END_STATE();
    case 24:
      if (lookahead == '.') ADVANCE(260);
      END_STATE();
    case 25:
      if (lookahead == '.') ADVANCE(24);
      END_STATE();
    case 26:
      if (lookahead == '/') ADVANCE(15);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(26)
      END_STATE();
    case 27:
      if (lookahead == '/') ADVANCE(15);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(26)
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(68);
      END_STATE();
    case 28:
      if (lookahead == '/') ADVANCE(70);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(28);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(259);
      END_STATE();
    case 29:
      if (lookahead == '=') ADVANCE(104);
      END_STATE();
    case 30:
      if (lookahead == '=') ADVANCE(103);
      END_STATE();
    case 31:
      if (lookahead == '=') ADVANCE(101);
      END_STATE();
    case 32:
      if (lookahead == '=') ADVANCE(102);
      END_STATE();
    case 33:
      if (lookahead == '=') ADVANCE(100);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(48);
      END_STATE();
    case 35:
      if (lookahead == 'c') ADVANCE(228);
      if (lookahead == 'i') ADVANCE(230);
      if (lookahead == 'k') ADVANCE(218);
      if (lookahead == 'm') ADVANCE(219);
      if (lookahead == 'n') ADVANCE(213);
      if (lookahead == 's') ADVANCE(253);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 36:
      if (lookahead == 'c') ADVANCE(62);
      END_STATE();
    case 37:
      if (lookahead == 'd') ADVANCE(116);
      END_STATE();
    case 38:
      if (lookahead == 'e') ADVANCE(36);
      END_STATE();
    case 39:
      if (lookahead == 'e') ADVANCE(42);
      END_STATE();
    case 40:
      if (lookahead == 'i') ADVANCE(43);
      END_STATE();
    case 41:
      if (lookahead == 'l') ADVANCE(63);
      END_STATE();
    case 42:
      if (lookahead == 'l') ADVANCE(38);
      END_STATE();
    case 43:
      if (lookahead == 'm') ADVANCE(54);
      END_STATE();
    case 44:
      if (lookahead == 'm') ADVANCE(88);
      END_STATE();
    case 45:
      if (lookahead == 'n') ADVANCE(37);
      END_STATE();
    case 46:
      if (lookahead == 'n') ADVANCE(37);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(67);
      END_STATE();
    case 47:
      if (lookahead == 'n') ADVANCE(41);
      if (lookahead == 'r') ADVANCE(117);
      END_STATE();
    case 48:
      if (lookahead == 'n') ADVANCE(61);
      END_STATE();
    case 49:
      if (lookahead == 'o') ADVANCE(59);
      END_STATE();
    case 50:
      if (lookahead == 'o') ADVANCE(89);
      END_STATE();
    case 51:
      if (lookahead == 'o') ADVANCE(44);
      END_STATE();
    case 52:
      if (lookahead == 'o') ADVANCE(58);
      END_STATE();
    case 53:
      if (lookahead == 'o') ADVANCE(55);
      END_STATE();
    case 54:
      if (lookahead == 'p') ADVANCE(52);
      END_STATE();
    case 55:
      if (lookahead == 'r') ADVANCE(124);
      END_STATE();
    case 56:
      if (lookahead == 'r') ADVANCE(51);
      END_STATE();
    case 57:
      if (lookahead == 'r') ADVANCE(51);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(67);
      END_STATE();
    case 58:
      if (lookahead == 'r') ADVANCE(60);
      END_STATE();
    case 59:
      if (lookahead == 't') ADVANCE(118);
      END_STATE();
    case 60:
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 61:
      if (lookahead == 't') ADVANCE(114);
      END_STATE();
    case 62:
      if (lookahead == 't') ADVANCE(53);
      END_STATE();
    case 63:
      if (lookahead == 'y') ADVANCE(121);
      END_STATE();
    case 64:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(165);
      END_STATE();
    case 65:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(166);
      END_STATE();
    case 66:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(163);
      END_STATE();
    case 67:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(154);
      END_STATE();
    case 68:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(67);
      END_STATE();
    case 69:
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 70:
      if (!sym_plain_value_character_set_1(lookahead)) ADVANCE(28);
      END_STATE();
    case 71:
      if (!sym_plain_value_character_set_1(lookahead)) ADVANCE(259);
      END_STATE();
    case 72:
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(6);
      if (lookahead == '"') ADVANCE(158);
      if (lookahead == '\\') ADVANCE(72);
      END_STATE();
    case 73:
      if (lookahead != 0 &&
          lookahead != '\'' &&
          lookahead != '\\') ADVANCE(9);
      if (lookahead == '\'') ADVANCE(159);
      if (lookahead == '\\') ADVANCE(73);
      END_STATE();
    case 74:
      if (eof) ADVANCE(77);
      if (lookahead == '!') ADVANCE(40);
      if (lookahead == '"') ADVANCE(6);
      if (lookahead == '#') ADVANCE(98);
      if (lookahead == '$') ADVANCE(29);
      if (lookahead == '&') ADVANCE(91);
      if (lookahead == '\'') ADVANCE(9);
      if (lookahead == '(') ADVANCE(115);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(93);
      if (lookahead == '+') ADVANCE(111);
      if (lookahead == ',') ADVANCE(79);
      if (lookahead == '-') ADVANCE(186);
      if (lookahead == '.') ADVANCE(95);
      if (lookahead == '/') ADVANCE(189);
      if (lookahead == ':') ADVANCE(96);
      if (lookahead == ';') ADVANCE(80);
      if (lookahead == '=') ADVANCE(99);
      if (lookahead == '>') ADVANCE(107);
      if (lookahead == '@') ADVANCE(35);
      if (lookahead == '[') ADVANCE(105);
      if (lookahead == ']') ADVANCE(106);
      if (lookahead == '^') ADVANCE(31);
      if (lookahead == 'a') ADVANCE(45);
      if (lookahead == 'f') ADVANCE(56);
      if (lookahead == 'n') ADVANCE(49);
      if (lookahead == 'o') ADVANCE(47);
      if (lookahead == 's') ADVANCE(39);
      if (lookahead == 't') ADVANCE(50);
      if (lookahead == '{') ADVANCE(86);
      if (lookahead == '|') ADVANCE(32);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == '~') ADVANCE(109);
      if (lookahead == 181) ADVANCE(23);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(21);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(74)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      END_STATE();
    case 75:
      if (eof) ADVANCE(77);
      if (lookahead == '"') ADVANCE(6);
      if (lookahead == '#') ADVANCE(98);
      if (lookahead == '&') ADVANCE(91);
      if (lookahead == '\'') ADVANCE(9);
      if (lookahead == '(') ADVANCE(112);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(110);
      if (lookahead == ',') ADVANCE(79);
      if (lookahead == '-') ADVANCE(69);
      if (lookahead == '.') ADVANCE(94);
      if (lookahead == '/') ADVANCE(15);
      if (lookahead == ':') ADVANCE(96);
      if (lookahead == '>') ADVANCE(107);
      if (lookahead == '@') ADVANCE(35);
      if (lookahead == '[') ADVANCE(105);
      if (lookahead == '{') ADVANCE(86);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == '~') ADVANCE(108);
      if (lookahead == 181) ADVANCE(23);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(76)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 76:
      if (eof) ADVANCE(77);
      if (lookahead == '"') ADVANCE(6);
      if (lookahead == '#') ADVANCE(98);
      if (lookahead == '&') ADVANCE(91);
      if (lookahead == '\'') ADVANCE(9);
      if (lookahead == ')') ADVANCE(113);
      if (lookahead == '*') ADVANCE(92);
      if (lookahead == '+') ADVANCE(110);
      if (lookahead == ',') ADVANCE(79);
      if (lookahead == '-') ADVANCE(69);
      if (lookahead == '.') ADVANCE(94);
      if (lookahead == '/') ADVANCE(15);
      if (lookahead == ':') ADVANCE(96);
      if (lookahead == '>') ADVANCE(107);
      if (lookahead == '@') ADVANCE(35);
      if (lookahead == '[') ADVANCE(105);
      if (lookahead == '{') ADVANCE(86);
      if (lookahead == '}') ADVANCE(87);
      if (lookahead == '~') ADVANCE(108);
      if (lookahead == 181) ADVANCE(23);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(76)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(anon_sym_ATimport);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(anon_sym_ATmedia);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(anon_sym_ATcharset);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(anon_sym_ATnamespace);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_ATkeyframes);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(aux_sym_keyframes_statement_token1);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(sym_from);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(sym_to);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_ATsupports);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(sym_nesting_selector);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_STAR);
      if (lookahead == '=') ADVANCE(103);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(165);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(97);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(sym_equal);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(sym_contains_word_equal);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(sym_starts_with_equal);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(sym_dash_equal);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(sym_contains_equal);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(sym_ends_equal);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_TILDE);
      if (lookahead == '=') ADVANCE(100);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_PLUS);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(sym_important);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(anon_sym_LPAREN2);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(sym_and);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(sym_or);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(sym_not);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(sym_not);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(sym_not);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(sym_only);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(sym_only);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(sym_only);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(anon_sym_selector);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(anon_sym_selector);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(anon_sym_selector);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(166);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(140);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(127);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(143);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(148);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(141);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(146);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(142);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(150);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-') ADVANCE(65);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(144);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(152);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(128);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(127);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(134);
      END_STATE();
    case 136:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(130);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(150);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(137);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(131);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(148);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(138);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(129);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(146);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(135);
      END_STATE();
    case 139:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(132);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(152);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(136);
      END_STATE();
    case 140:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(166);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(140);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(127);
      END_STATE();
    case 142:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(143);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(148);
      END_STATE();
    case 143:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(141);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(146);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(142);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(150);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(144);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(152);
      END_STATE();
    case 146:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(127);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(156);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(146);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(147);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(148);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(149);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(150);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(151);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 154:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(152);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(153);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(aux_sym_color_value_token1);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(sym_string_value);
      END_STATE();
    case 158:
      ACCEPT_TOKEN(sym_string_value);
      if (lookahead == '"') ADVANCE(157);
      if (lookahead == '\\') ADVANCE(72);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(6);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(sym_string_value);
      if (lookahead == '\'') ADVANCE(157);
      if (lookahead == '\\') ADVANCE(73);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(9);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(aux_sym_integer_value_token1);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      END_STATE();
    case 161:
      ACCEPT_TOKEN(aux_sym_integer_value_token1);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(133);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(154);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(139);
      END_STATE();
    case 162:
      ACCEPT_TOKEN(aux_sym_integer_value_token1);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(22);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(67);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(161);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(aux_sym_integer_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(163);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(aux_sym_float_value_token1);
      if (lookahead == '/') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(164);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      if (!sym_plain_value_character_set_2(lookahead)) ADVANCE(259);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(aux_sym_float_value_token1);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(165);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(aux_sym_float_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(166);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(aux_sym_float_value_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(145);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(154);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'c') ADVANCE(178);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'e') ADVANCE(168);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'e') ADVANCE(172);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'l') ADVANCE(179);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'l') ADVANCE(169);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'n') ADVANCE(171);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'o') ADVANCE(177);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'o') ADVANCE(176);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'r') ADVANCE(125);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 't') ADVANCE(119);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 't') ADVANCE(175);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == 'y') ADVANCE(122);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(184);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(180);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(183);
      if (lookahead == '-') ADVANCE(193);
      if (lookahead == '/') ADVANCE(71);
      if (lookahead == '_') ADVANCE(194);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(164);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(182);
      if (!sym_plain_value_character_set_2(lookahead)) ADVANCE(259);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%') ADVANCE(183);
      if (lookahead == '/') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(194);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(182);
      if (!sym_plain_value_character_set_2(lookahead)) ADVANCE(259);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '/') ADVANCE(71);
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(183);
      if (!sym_plain_value_character_set_2(lookahead)) ADVANCE(259);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(sym_unit);
      if (lookahead == '%' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(184);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(sym_minus);
      END_STATE();
    case 186:
      ACCEPT_TOKEN(sym_minus);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(sym_minus);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(192);
      if (lookahead == '.') ADVANCE(64);
      if (lookahead == '/') ADVANCE(70);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(191);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(160);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(sym_minus);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 189:
      ACCEPT_TOKEN(sym_divide);
      if (lookahead == '*') ADVANCE(17);
      END_STATE();
    case 190:
      ACCEPT_TOKEN(sym_divide);
      if (lookahead == '*') ADVANCE(17);
      if (!sym_plain_value_character_set_2(lookahead)) ADVANCE(28);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '-') ADVANCE(193);
      if (lookahead == '/') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(164);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      if (!sym_plain_value_character_set_2(lookahead)) ADVANCE(259);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(70);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(192);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(209);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(71);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(164);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      if (!sym_plain_value_character_set_2(lookahead)) ADVANCE(259);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '/') ADVANCE(71);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(194);
      if (!sym_plain_value_character_set_2(lookahead)) ADVANCE(259);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(205);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(195);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(199);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 198:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(206);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 199:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(196);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 200:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(198);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(204);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 202:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(203);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 203:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(126);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(120);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(202);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 206:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'y') ADVANCE(123);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(155);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(207);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 209:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(209);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(240);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(81);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 212:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(216);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(231);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(232);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 215:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'a') ADVANCE(233);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 216:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'c') ADVANCE(220);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 217:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'd') ADVANCE(229);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 218:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(254);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(217);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(83);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 221:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(248);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 222:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(251);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(246);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 224:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(247);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'e') ADVANCE(255);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 226:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'f') ADVANCE(243);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 227:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'f') ADVANCE(244);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 228:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'h') ADVANCE(210);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 229:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'i') ADVANCE(211);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'm') ADVANCE(236);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'm') ADVANCE(221);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'm') ADVANCE(223);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 233:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'm') ADVANCE(224);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 234:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'o') ADVANCE(241);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 235:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'o') ADVANCE(242);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 236:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'p') ADVANCE(234);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 237:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'p') ADVANCE(212);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'p') ADVANCE(235);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'p') ADVANCE(238);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 240:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'r') ADVANCE(249);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 241:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'r') ADVANCE(250);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'r') ADVANCE(252);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 243:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'r') ADVANCE(214);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'r') ADVANCE(215);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 's') ADVANCE(90);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 246:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 's') ADVANCE(84);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 247:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 's') ADVANCE(85);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 's') ADVANCE(237);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 's') ADVANCE(222);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 250:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 't') ADVANCE(78);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 251:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 't') ADVANCE(82);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 252:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 't') ADVANCE(245);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 253:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'u') ADVANCE(239);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 254:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'y') ADVANCE(226);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 255:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == 'y') ADVANCE(227);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 256:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == 'k') ADVANCE(225);
      if (lookahead == '-' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(256);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_') ADVANCE(257);
      END_STATE();
    case 257:
      ACCEPT_TOKEN(sym_at_keyword);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(257);
      END_STATE();
    case 258:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 259:
      ACCEPT_TOKEN(sym_plain_value);
      if (lookahead == '/') ADVANCE(71);
      if (!sym_plain_value_character_set_2(lookahead)) ADVANCE(259);
      END_STATE();
    case 260:
      ACCEPT_TOKEN(sym_grit_metavariable);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(sym_grit_metavariable);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(261);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 75},
  [2] = {.lex_state = 75},
  [3] = {.lex_state = 75},
  [4] = {.lex_state = 75},
  [5] = {.lex_state = 75},
  [6] = {.lex_state = 75},
  [7] = {.lex_state = 75},
  [8] = {.lex_state = 75},
  [9] = {.lex_state = 75},
  [10] = {.lex_state = 75},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 2},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 2},
  [16] = {.lex_state = 2},
  [17] = {.lex_state = 75, .external_lex_state = 1},
  [18] = {.lex_state = 3, .external_lex_state = 1},
  [19] = {.lex_state = 2, .external_lex_state = 1},
  [20] = {.lex_state = 75},
  [21] = {.lex_state = 75},
  [22] = {.lex_state = 75},
  [23] = {.lex_state = 75},
  [24] = {.lex_state = 75},
  [25] = {.lex_state = 75},
  [26] = {.lex_state = 2},
  [27] = {.lex_state = 2},
  [28] = {.lex_state = 2},
  [29] = {.lex_state = 2},
  [30] = {.lex_state = 2},
  [31] = {.lex_state = 75},
  [32] = {.lex_state = 75},
  [33] = {.lex_state = 75},
  [34] = {.lex_state = 75},
  [35] = {.lex_state = 2},
  [36] = {.lex_state = 75},
  [37] = {.lex_state = 75},
  [38] = {.lex_state = 2},
  [39] = {.lex_state = 75},
  [40] = {.lex_state = 12},
  [41] = {.lex_state = 75},
  [42] = {.lex_state = 75},
  [43] = {.lex_state = 75},
  [44] = {.lex_state = 2},
  [45] = {.lex_state = 75},
  [46] = {.lex_state = 12},
  [47] = {.lex_state = 2},
  [48] = {.lex_state = 2},
  [49] = {.lex_state = 75},
  [50] = {.lex_state = 2},
  [51] = {.lex_state = 75},
  [52] = {.lex_state = 75},
  [53] = {.lex_state = 75},
  [54] = {.lex_state = 75},
  [55] = {.lex_state = 75},
  [56] = {.lex_state = 2},
  [57] = {.lex_state = 75},
  [58] = {.lex_state = 75},
  [59] = {.lex_state = 75},
  [60] = {.lex_state = 75},
  [61] = {.lex_state = 75},
  [62] = {.lex_state = 75},
  [63] = {.lex_state = 75},
  [64] = {.lex_state = 3},
  [65] = {.lex_state = 75},
  [66] = {.lex_state = 75},
  [67] = {.lex_state = 75},
  [68] = {.lex_state = 75},
  [69] = {.lex_state = 75},
  [70] = {.lex_state = 75},
  [71] = {.lex_state = 75},
  [72] = {.lex_state = 75},
  [73] = {.lex_state = 75},
  [74] = {.lex_state = 75},
  [75] = {.lex_state = 75},
  [76] = {.lex_state = 75},
  [77] = {.lex_state = 75},
  [78] = {.lex_state = 75},
  [79] = {.lex_state = 75},
  [80] = {.lex_state = 75},
  [81] = {.lex_state = 75},
  [82] = {.lex_state = 75},
  [83] = {.lex_state = 75},
  [84] = {.lex_state = 75},
  [85] = {.lex_state = 75},
  [86] = {.lex_state = 75},
  [87] = {.lex_state = 75},
  [88] = {.lex_state = 75},
  [89] = {.lex_state = 75},
  [90] = {.lex_state = 75},
  [91] = {.lex_state = 75},
  [92] = {.lex_state = 75},
  [93] = {.lex_state = 75},
  [94] = {.lex_state = 75},
  [95] = {.lex_state = 75},
  [96] = {.lex_state = 1},
  [97] = {.lex_state = 2},
  [98] = {.lex_state = 2},
  [99] = {.lex_state = 1},
  [100] = {.lex_state = 2},
  [101] = {.lex_state = 2},
  [102] = {.lex_state = 2},
  [103] = {.lex_state = 2},
  [104] = {.lex_state = 2},
  [105] = {.lex_state = 2},
  [106] = {.lex_state = 2},
  [107] = {.lex_state = 2},
  [108] = {.lex_state = 2},
  [109] = {.lex_state = 2},
  [110] = {.lex_state = 2},
  [111] = {.lex_state = 2},
  [112] = {.lex_state = 2},
  [113] = {.lex_state = 2},
  [114] = {.lex_state = 2},
  [115] = {.lex_state = 2},
  [116] = {.lex_state = 2},
  [117] = {.lex_state = 2},
  [118] = {.lex_state = 2},
  [119] = {.lex_state = 2},
  [120] = {.lex_state = 12},
  [121] = {.lex_state = 2},
  [122] = {.lex_state = 2},
  [123] = {.lex_state = 2},
  [124] = {.lex_state = 2},
  [125] = {.lex_state = 12},
  [126] = {.lex_state = 2},
  [127] = {.lex_state = 2},
  [128] = {.lex_state = 2},
  [129] = {.lex_state = 2},
  [130] = {.lex_state = 2},
  [131] = {.lex_state = 2},
  [132] = {.lex_state = 2},
  [133] = {.lex_state = 2},
  [134] = {.lex_state = 75, .external_lex_state = 1},
  [135] = {.lex_state = 75, .external_lex_state = 1},
  [136] = {.lex_state = 75, .external_lex_state = 1},
  [137] = {.lex_state = 75, .external_lex_state = 1},
  [138] = {.lex_state = 11},
  [139] = {.lex_state = 12},
  [140] = {.lex_state = 75, .external_lex_state = 1},
  [141] = {.lex_state = 12},
  [142] = {.lex_state = 75, .external_lex_state = 1},
  [143] = {.lex_state = 8},
  [144] = {.lex_state = 75, .external_lex_state = 1},
  [145] = {.lex_state = 75, .external_lex_state = 1},
  [146] = {.lex_state = 8},
  [147] = {.lex_state = 75, .external_lex_state = 1},
  [148] = {.lex_state = 12},
  [149] = {.lex_state = 12},
  [150] = {.lex_state = 75, .external_lex_state = 1},
  [151] = {.lex_state = 75, .external_lex_state = 1},
  [152] = {.lex_state = 75, .external_lex_state = 1},
  [153] = {.lex_state = 12},
  [154] = {.lex_state = 12},
  [155] = {.lex_state = 12},
  [156] = {.lex_state = 12},
  [157] = {.lex_state = 12},
  [158] = {.lex_state = 75, .external_lex_state = 1},
  [159] = {.lex_state = 75, .external_lex_state = 1},
  [160] = {.lex_state = 75, .external_lex_state = 1},
  [161] = {.lex_state = 75, .external_lex_state = 1},
  [162] = {.lex_state = 75, .external_lex_state = 1},
  [163] = {.lex_state = 75, .external_lex_state = 1},
  [164] = {.lex_state = 12},
  [165] = {.lex_state = 12},
  [166] = {.lex_state = 75, .external_lex_state = 1},
  [167] = {.lex_state = 75, .external_lex_state = 1},
  [168] = {.lex_state = 75, .external_lex_state = 1},
  [169] = {.lex_state = 75, .external_lex_state = 1},
  [170] = {.lex_state = 75, .external_lex_state = 1},
  [171] = {.lex_state = 75, .external_lex_state = 1},
  [172] = {.lex_state = 75, .external_lex_state = 1},
  [173] = {.lex_state = 12},
  [174] = {.lex_state = 75, .external_lex_state = 1},
  [175] = {.lex_state = 75, .external_lex_state = 1},
  [176] = {.lex_state = 75, .external_lex_state = 1},
  [177] = {.lex_state = 75, .external_lex_state = 1},
  [178] = {.lex_state = 75, .external_lex_state = 1},
  [179] = {.lex_state = 75, .external_lex_state = 1},
  [180] = {.lex_state = 75, .external_lex_state = 1},
  [181] = {.lex_state = 75, .external_lex_state = 1},
  [182] = {.lex_state = 12},
  [183] = {.lex_state = 75, .external_lex_state = 1},
  [184] = {.lex_state = 75, .external_lex_state = 1},
  [185] = {.lex_state = 12},
  [186] = {.lex_state = 75, .external_lex_state = 1},
  [187] = {.lex_state = 12},
  [188] = {.lex_state = 12},
  [189] = {.lex_state = 12},
  [190] = {.lex_state = 12},
  [191] = {.lex_state = 75, .external_lex_state = 1},
  [192] = {.lex_state = 12},
  [193] = {.lex_state = 75, .external_lex_state = 1},
  [194] = {.lex_state = 75, .external_lex_state = 1},
  [195] = {.lex_state = 10},
  [196] = {.lex_state = 10},
  [197] = {.lex_state = 2},
  [198] = {.lex_state = 2},
  [199] = {.lex_state = 10},
  [200] = {.lex_state = 10},
  [201] = {.lex_state = 10},
  [202] = {.lex_state = 10},
  [203] = {.lex_state = 10},
  [204] = {.lex_state = 7},
  [205] = {.lex_state = 10},
  [206] = {.lex_state = 7},
  [207] = {.lex_state = 10},
  [208] = {.lex_state = 7},
  [209] = {.lex_state = 0},
  [210] = {.lex_state = 10},
  [211] = {.lex_state = 10},
  [212] = {.lex_state = 10},
  [213] = {.lex_state = 10},
  [214] = {.lex_state = 10},
  [215] = {.lex_state = 7},
  [216] = {.lex_state = 10},
  [217] = {.lex_state = 10},
  [218] = {.lex_state = 0},
  [219] = {.lex_state = 10},
  [220] = {.lex_state = 10},
  [221] = {.lex_state = 7},
  [222] = {.lex_state = 7},
  [223] = {.lex_state = 7},
  [224] = {.lex_state = 10},
  [225] = {.lex_state = 0},
  [226] = {.lex_state = 0},
  [227] = {.lex_state = 0},
  [228] = {.lex_state = 0},
  [229] = {.lex_state = 0},
  [230] = {.lex_state = 0},
  [231] = {.lex_state = 0},
  [232] = {.lex_state = 0},
  [233] = {.lex_state = 0},
  [234] = {.lex_state = 75},
  [235] = {.lex_state = 75},
  [236] = {.lex_state = 0},
  [237] = {.lex_state = 0},
  [238] = {.lex_state = 0},
  [239] = {.lex_state = 0},
  [240] = {.lex_state = 0},
  [241] = {.lex_state = 0},
  [242] = {.lex_state = 0},
  [243] = {.lex_state = 0},
  [244] = {.lex_state = 0},
  [245] = {.lex_state = 0},
  [246] = {.lex_state = 0},
  [247] = {.lex_state = 0},
  [248] = {.lex_state = 7},
  [249] = {.lex_state = 7},
  [250] = {.lex_state = 0},
  [251] = {.lex_state = 0},
  [252] = {.lex_state = 7},
  [253] = {.lex_state = 7},
  [254] = {.lex_state = 0},
  [255] = {.lex_state = 7},
  [256] = {.lex_state = 0},
  [257] = {.lex_state = 0},
  [258] = {.lex_state = 0},
  [259] = {.lex_state = 0},
  [260] = {.lex_state = 0},
  [261] = {.lex_state = 0},
  [262] = {.lex_state = 0},
  [263] = {.lex_state = 0},
  [264] = {.lex_state = 0},
  [265] = {.lex_state = 0},
  [266] = {.lex_state = 0},
  [267] = {.lex_state = 0},
  [268] = {.lex_state = 75},
  [269] = {.lex_state = 75},
  [270] = {.lex_state = 0},
  [271] = {.lex_state = 0},
  [272] = {.lex_state = 0},
  [273] = {.lex_state = 0},
  [274] = {.lex_state = 0},
  [275] = {.lex_state = 0},
  [276] = {.lex_state = 0},
  [277] = {.lex_state = 18},
  [278] = {.lex_state = 0},
  [279] = {.lex_state = 0},
  [280] = {.lex_state = 10},
  [281] = {.lex_state = 75},
  [282] = {.lex_state = 0},
  [283] = {.lex_state = 0},
  [284] = {.lex_state = 0},
  [285] = {.lex_state = 75},
  [286] = {.lex_state = 75},
  [287] = {.lex_state = 0},
  [288] = {.lex_state = 27},
  [289] = {.lex_state = 0},
  [290] = {.lex_state = 75},
  [291] = {.lex_state = 0},
  [292] = {.lex_state = 27},
  [293] = {.lex_state = 75},
  [294] = {.lex_state = 75},
  [295] = {.lex_state = 75},
  [296] = {.lex_state = 0},
  [297] = {.lex_state = 2},
  [298] = {.lex_state = 75},
  [299] = {.lex_state = 0},
  [300] = {.lex_state = 75},
  [301] = {.lex_state = 75},
  [302] = {.lex_state = 0},
  [303] = {.lex_state = 0},
  [304] = {.lex_state = 75},
  [305] = {.lex_state = 75},
  [306] = {.lex_state = 0},
  [307] = {.lex_state = 0},
  [308] = {.lex_state = 0},
  [309] = {.lex_state = 0},
  [310] = {.lex_state = 27},
  [311] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_ATimport] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_ATmedia] = ACTIONS(1),
    [anon_sym_ATcharset] = ACTIONS(1),
    [anon_sym_ATnamespace] = ACTIONS(1),
    [anon_sym_ATkeyframes] = ACTIONS(1),
    [aux_sym_keyframes_statement_token1] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [sym_from] = ACTIONS(1),
    [sym_to] = ACTIONS(1),
    [anon_sym_ATsupports] = ACTIONS(1),
    [sym_nesting_selector] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_COLON_COLON] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(1),
    [sym_equal] = ACTIONS(1),
    [sym_contains_word_equal] = ACTIONS(1),
    [sym_starts_with_equal] = ACTIONS(1),
    [sym_dash_equal] = ACTIONS(1),
    [sym_contains_equal] = ACTIONS(1),
    [sym_ends_equal] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_important] = ACTIONS(1),
    [anon_sym_LPAREN2] = ACTIONS(1),
    [sym_and] = ACTIONS(1),
    [sym_or] = ACTIONS(1),
    [sym_not] = ACTIONS(1),
    [sym_only] = ACTIONS(1),
    [anon_sym_selector] = ACTIONS(1),
    [aux_sym_color_value_token1] = ACTIONS(1),
    [sym_string_value] = ACTIONS(1),
    [aux_sym_integer_value_token1] = ACTIONS(1),
    [aux_sym_float_value_token1] = ACTIONS(1),
    [sym_minus] = ACTIONS(1),
    [sym_divide] = ACTIONS(1),
    [sym_at_keyword] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [sym_grit_metavariable] = ACTIONS(1),
    [sym__descendant_operator] = ACTIONS(1),
  },
  [1] = {
    [sym_stylesheet] = STATE(307),
    [sym_import_statement] = STATE(63),
    [sym_media_statement] = STATE(63),
    [sym_charset_statement] = STATE(63),
    [sym_namespace_statement] = STATE(63),
    [sym_keyframes_statement] = STATE(63),
    [sym_supports_statement] = STATE(63),
    [sym_at_rule] = STATE(63),
    [sym_rule_set] = STATE(63),
    [sym_selectors] = STATE(276),
    [sym__selector] = STATE(171),
    [sym_universal_selector] = STATE(171),
    [sym_class_selector] = STATE(171),
    [sym_pseudo_class_selector] = STATE(171),
    [sym_pseudo_element_selector] = STATE(171),
    [sym_id_selector] = STATE(171),
    [sym_attribute_selector] = STATE(171),
    [sym_child_selector] = STATE(171),
    [sym_descendant_selector] = STATE(171),
    [sym_sibling_selector] = STATE(171),
    [sym_adjacent_sibling_selector] = STATE(171),
    [sym_declaration] = STATE(63),
    [aux_sym_stylesheet_repeat1] = STATE(9),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_ATimport] = ACTIONS(7),
    [anon_sym_ATmedia] = ACTIONS(9),
    [anon_sym_ATcharset] = ACTIONS(11),
    [anon_sym_ATnamespace] = ACTIONS(13),
    [anon_sym_ATkeyframes] = ACTIONS(15),
    [aux_sym_keyframes_statement_token1] = ACTIONS(15),
    [anon_sym_ATsupports] = ACTIONS(17),
    [sym_nesting_selector] = ACTIONS(19),
    [anon_sym_STAR] = ACTIONS(21),
    [anon_sym_DOT] = ACTIONS(23),
    [anon_sym_COLON] = ACTIONS(25),
    [anon_sym_COLON_COLON] = ACTIONS(27),
    [anon_sym_POUND] = ACTIONS(29),
    [anon_sym_LBRACK] = ACTIONS(31),
    [sym_string_value] = ACTIONS(19),
    [sym_identifier] = ACTIONS(33),
    [sym_at_keyword] = ACTIONS(35),
    [sym_comment] = ACTIONS(3),
    [sym_grit_metavariable] = ACTIONS(19),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 22,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(37), 1,
      anon_sym_ATimport,
    ACTIONS(39), 1,
      anon_sym_ATmedia,
    ACTIONS(41), 1,
      anon_sym_ATcharset,
    ACTIONS(43), 1,
      anon_sym_ATnamespace,
    ACTIONS(47), 1,
      anon_sym_RBRACE,
    ACTIONS(49), 1,
      anon_sym_ATsupports,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(53), 1,
      sym_at_keyword,
    ACTIONS(55), 1,
      sym_grit_metavariable,
    STATE(279), 1,
      sym_selectors,
    STATE(302), 1,
      sym_last_declaration,
    ACTIONS(19), 2,
      sym_nesting_selector,
      sym_string_value,
    ACTIONS(45), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(5), 10,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(171), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [88] = 22,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(37), 1,
      anon_sym_ATimport,
    ACTIONS(39), 1,
      anon_sym_ATmedia,
    ACTIONS(41), 1,
      anon_sym_ATcharset,
    ACTIONS(43), 1,
      anon_sym_ATnamespace,
    ACTIONS(49), 1,
      anon_sym_ATsupports,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(53), 1,
      sym_at_keyword,
    ACTIONS(55), 1,
      sym_grit_metavariable,
    ACTIONS(57), 1,
      anon_sym_RBRACE,
    STATE(279), 1,
      sym_selectors,
    STATE(299), 1,
      sym_last_declaration,
    ACTIONS(19), 2,
      sym_nesting_selector,
      sym_string_value,
    ACTIONS(45), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(10), 10,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(171), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [176] = 22,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(37), 1,
      anon_sym_ATimport,
    ACTIONS(39), 1,
      anon_sym_ATmedia,
    ACTIONS(41), 1,
      anon_sym_ATcharset,
    ACTIONS(43), 1,
      anon_sym_ATnamespace,
    ACTIONS(49), 1,
      anon_sym_ATsupports,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(53), 1,
      sym_at_keyword,
    ACTIONS(55), 1,
      sym_grit_metavariable,
    ACTIONS(59), 1,
      anon_sym_RBRACE,
    STATE(279), 1,
      sym_selectors,
    STATE(282), 1,
      sym_last_declaration,
    ACTIONS(19), 2,
      sym_nesting_selector,
      sym_string_value,
    ACTIONS(45), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(3), 10,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(171), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [264] = 22,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(37), 1,
      anon_sym_ATimport,
    ACTIONS(39), 1,
      anon_sym_ATmedia,
    ACTIONS(41), 1,
      anon_sym_ATcharset,
    ACTIONS(43), 1,
      anon_sym_ATnamespace,
    ACTIONS(49), 1,
      anon_sym_ATsupports,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(53), 1,
      sym_at_keyword,
    ACTIONS(55), 1,
      sym_grit_metavariable,
    ACTIONS(61), 1,
      anon_sym_RBRACE,
    STATE(279), 1,
      sym_selectors,
    STATE(303), 1,
      sym_last_declaration,
    ACTIONS(19), 2,
      sym_nesting_selector,
      sym_string_value,
    ACTIONS(45), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(10), 10,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(171), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [352] = 22,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(37), 1,
      anon_sym_ATimport,
    ACTIONS(39), 1,
      anon_sym_ATmedia,
    ACTIONS(41), 1,
      anon_sym_ATcharset,
    ACTIONS(43), 1,
      anon_sym_ATnamespace,
    ACTIONS(49), 1,
      anon_sym_ATsupports,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(53), 1,
      sym_at_keyword,
    ACTIONS(55), 1,
      sym_grit_metavariable,
    ACTIONS(63), 1,
      anon_sym_RBRACE,
    STATE(279), 1,
      sym_selectors,
    STATE(289), 1,
      sym_last_declaration,
    ACTIONS(19), 2,
      sym_nesting_selector,
      sym_string_value,
    ACTIONS(45), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(10), 10,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(171), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [440] = 22,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(37), 1,
      anon_sym_ATimport,
    ACTIONS(39), 1,
      anon_sym_ATmedia,
    ACTIONS(41), 1,
      anon_sym_ATcharset,
    ACTIONS(43), 1,
      anon_sym_ATnamespace,
    ACTIONS(49), 1,
      anon_sym_ATsupports,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(53), 1,
      sym_at_keyword,
    ACTIONS(55), 1,
      sym_grit_metavariable,
    ACTIONS(65), 1,
      anon_sym_RBRACE,
    STATE(279), 1,
      sym_selectors,
    STATE(283), 1,
      sym_last_declaration,
    ACTIONS(19), 2,
      sym_nesting_selector,
      sym_string_value,
    ACTIONS(45), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    STATE(6), 10,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(171), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [528] = 21,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(67), 1,
      ts_builtin_sym_end,
    ACTIONS(69), 1,
      anon_sym_ATimport,
    ACTIONS(72), 1,
      anon_sym_ATmedia,
    ACTIONS(75), 1,
      anon_sym_ATcharset,
    ACTIONS(78), 1,
      anon_sym_ATnamespace,
    ACTIONS(84), 1,
      anon_sym_ATsupports,
    ACTIONS(90), 1,
      anon_sym_STAR,
    ACTIONS(93), 1,
      anon_sym_DOT,
    ACTIONS(96), 1,
      anon_sym_COLON,
    ACTIONS(99), 1,
      anon_sym_COLON_COLON,
    ACTIONS(102), 1,
      anon_sym_POUND,
    ACTIONS(105), 1,
      anon_sym_LBRACK,
    ACTIONS(108), 1,
      sym_identifier,
    ACTIONS(111), 1,
      sym_at_keyword,
    STATE(8), 1,
      aux_sym_stylesheet_repeat1,
    STATE(276), 1,
      sym_selectors,
    ACTIONS(81), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    ACTIONS(87), 3,
      sym_nesting_selector,
      sym_string_value,
      sym_grit_metavariable,
    STATE(63), 9,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
    STATE(171), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [613] = 21,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_ATimport,
    ACTIONS(9), 1,
      anon_sym_ATmedia,
    ACTIONS(11), 1,
      anon_sym_ATcharset,
    ACTIONS(13), 1,
      anon_sym_ATnamespace,
    ACTIONS(17), 1,
      anon_sym_ATsupports,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(35), 1,
      sym_at_keyword,
    ACTIONS(114), 1,
      ts_builtin_sym_end,
    STATE(8), 1,
      aux_sym_stylesheet_repeat1,
    STATE(276), 1,
      sym_selectors,
    ACTIONS(15), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    ACTIONS(19), 3,
      sym_nesting_selector,
      sym_string_value,
      sym_grit_metavariable,
    STATE(63), 9,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
    STATE(171), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [698] = 21,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(116), 1,
      anon_sym_ATimport,
    ACTIONS(119), 1,
      anon_sym_ATmedia,
    ACTIONS(122), 1,
      anon_sym_ATcharset,
    ACTIONS(125), 1,
      anon_sym_ATnamespace,
    ACTIONS(131), 1,
      anon_sym_RBRACE,
    ACTIONS(133), 1,
      anon_sym_ATsupports,
    ACTIONS(139), 1,
      anon_sym_STAR,
    ACTIONS(142), 1,
      anon_sym_DOT,
    ACTIONS(145), 1,
      anon_sym_COLON,
    ACTIONS(148), 1,
      anon_sym_COLON_COLON,
    ACTIONS(151), 1,
      anon_sym_POUND,
    ACTIONS(154), 1,
      anon_sym_LBRACK,
    ACTIONS(157), 1,
      sym_identifier,
    ACTIONS(160), 1,
      sym_at_keyword,
    ACTIONS(163), 1,
      sym_grit_metavariable,
    STATE(279), 1,
      sym_selectors,
    ACTIONS(128), 2,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
    ACTIONS(136), 2,
      sym_nesting_selector,
      sym_string_value,
    STATE(10), 10,
      sym_import_statement,
      sym_media_statement,
      sym_charset_statement,
      sym_namespace_statement,
      sym_keyframes_statement,
      sym_supports_statement,
      sym_at_rule,
      sym_rule_set,
      sym_declaration,
      aux_sym_block_repeat1,
    STATE(171), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [783] = 18,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(166), 1,
      sym_nesting_selector,
    ACTIONS(168), 1,
      anon_sym_DOT,
    ACTIONS(170), 1,
      anon_sym_POUND,
    ACTIONS(172), 1,
      anon_sym_RPAREN,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(182), 1,
      sym_identifier,
    ACTIONS(184), 1,
      sym_plain_value,
    STATE(35), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(176), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
    STATE(150), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [855] = 18,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(168), 1,
      anon_sym_DOT,
    ACTIONS(170), 1,
      anon_sym_POUND,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(182), 1,
      sym_identifier,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(186), 1,
      sym_nesting_selector,
    ACTIONS(188), 1,
      anon_sym_RPAREN,
    STATE(47), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(176), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
    STATE(152), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [927] = 17,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(168), 1,
      anon_sym_DOT,
    ACTIONS(170), 1,
      anon_sym_POUND,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(182), 1,
      sym_identifier,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(190), 1,
      sym_nesting_selector,
    STATE(100), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(176), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
    STATE(191), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [996] = 18,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(192), 1,
      anon_sym_COMMA,
    ACTIONS(194), 1,
      anon_sym_SEMI,
    ACTIONS(196), 1,
      anon_sym_RBRACE,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(202), 1,
      anon_sym_PLUS,
    ACTIONS(204), 1,
      sym_important,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(212), 1,
      sym_plain_value,
    STATE(29), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(206), 2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(208), 2,
      sym_minus,
      sym_divide,
    STATE(122), 2,
      sym_plus,
      sym_times,
    STATE(97), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [1060] = 17,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(192), 1,
      anon_sym_COMMA,
    ACTIONS(194), 1,
      anon_sym_SEMI,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(202), 1,
      anon_sym_PLUS,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(212), 1,
      sym_plain_value,
    ACTIONS(214), 1,
      sym_important,
    STATE(44), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(206), 2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(208), 2,
      sym_minus,
      sym_divide,
    STATE(122), 2,
      sym_plus,
      sym_times,
    STATE(97), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [1121] = 17,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(192), 1,
      anon_sym_COMMA,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(202), 1,
      anon_sym_PLUS,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(212), 1,
      sym_plain_value,
    ACTIONS(216), 1,
      anon_sym_SEMI,
    ACTIONS(218), 1,
      sym_important,
    STATE(38), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(206), 2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(208), 2,
      sym_minus,
      sym_divide,
    STATE(122), 2,
      sym_plus,
      sym_times,
    STATE(97), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [1182] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(229), 1,
      anon_sym_COLON,
    ACTIONS(226), 4,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
    ACTIONS(222), 6,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
    ACTIONS(224), 6,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
    ACTIONS(220), 8,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      sym_at_keyword,
  [1221] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(238), 1,
      anon_sym_LPAREN,
    STATE(109), 1,
      sym_arguments,
    ACTIONS(234), 3,
      anon_sym_STAR,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(236), 3,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_PLUS,
    ACTIONS(240), 7,
      anon_sym_LPAREN2,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(232), 8,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_RPAREN,
  [1260] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(244), 3,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_PLUS,
    ACTIONS(242), 4,
      anon_sym_STAR,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(246), 6,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(222), 8,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_RPAREN,
  [1293] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(248), 3,
      sym_nesting_selector,
      sym_string_value,
      sym_grit_metavariable,
    STATE(194), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [1336] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(252), 3,
      sym_nesting_selector,
      sym_string_value,
      sym_grit_metavariable,
    STATE(161), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [1379] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(254), 3,
      sym_nesting_selector,
      sym_string_value,
      sym_grit_metavariable,
    STATE(193), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [1422] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(256), 3,
      sym_nesting_selector,
      sym_string_value,
      sym_grit_metavariable,
    STATE(178), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [1465] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(258), 3,
      sym_nesting_selector,
      sym_string_value,
      sym_grit_metavariable,
    STATE(179), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [1508] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(21), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_DOT,
    ACTIONS(25), 1,
      anon_sym_COLON,
    ACTIONS(27), 1,
      anon_sym_COLON_COLON,
    ACTIONS(29), 1,
      anon_sym_POUND,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(250), 1,
      sym_identifier,
    ACTIONS(260), 3,
      sym_nesting_selector,
      sym_string_value,
      sym_grit_metavariable,
    STATE(181), 11,
      sym__selector,
      sym_universal_selector,
      sym_class_selector,
      sym_pseudo_class_selector,
      sym_pseudo_element_selector,
      sym_id_selector,
      sym_attribute_selector,
      sym_child_selector,
      sym_descendant_selector,
      sym_sibling_selector,
      sym_adjacent_sibling_selector,
  [1551] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(264), 1,
      anon_sym_RPAREN,
    STATE(48), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    STATE(240), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(262), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [1599] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(268), 1,
      anon_sym_COMMA,
    ACTIONS(273), 1,
      anon_sym_POUND,
    ACTIONS(276), 1,
      anon_sym_LPAREN2,
    ACTIONS(282), 1,
      aux_sym_integer_value_token1,
    ACTIONS(285), 1,
      aux_sym_float_value_token1,
    ACTIONS(288), 1,
      sym_identifier,
    ACTIONS(291), 1,
      sym_plain_value,
    STATE(27), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(279), 2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(271), 3,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      sym_important,
    STATE(97), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [1645] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(294), 1,
      anon_sym_RPAREN,
    STATE(48), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    STATE(246), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(262), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [1693] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(192), 1,
      anon_sym_COMMA,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(212), 1,
      sym_plain_value,
    ACTIONS(296), 1,
      anon_sym_SEMI,
    ACTIONS(298), 1,
      anon_sym_RBRACE,
    ACTIONS(300), 1,
      sym_important,
    STATE(27), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(206), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(97), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [1743] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(302), 1,
      anon_sym_RPAREN,
    STATE(48), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    STATE(244), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(262), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [1791] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(304), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(306), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [1818] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(310), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(308), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [1845] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(314), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(312), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [1872] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(316), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [1899] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(320), 1,
      anon_sym_COMMA,
    ACTIONS(322), 1,
      anon_sym_RPAREN,
    STATE(48), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    STATE(261), 1,
      aux_sym_pseudo_class_arguments_repeat2,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [1946] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(304), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(306), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [1973] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(326), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(324), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2000] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(192), 1,
      anon_sym_COMMA,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(212), 1,
      sym_plain_value,
    ACTIONS(328), 1,
      anon_sym_SEMI,
    ACTIONS(330), 1,
      sym_important,
    STATE(27), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(206), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(97), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [2047] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(334), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(332), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2074] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(336), 1,
      anon_sym_SEMI,
    ACTIONS(338), 1,
      anon_sym_PLUS,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(350), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    ACTIONS(346), 2,
      sym_minus,
      sym_divide,
    STATE(132), 2,
      sym_plus,
      sym_times,
    STATE(238), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [2119] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(354), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(352), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2146] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(358), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(356), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2173] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(362), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(360), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2200] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(192), 1,
      anon_sym_COMMA,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(212), 1,
      sym_plain_value,
    ACTIONS(296), 1,
      anon_sym_SEMI,
    ACTIONS(364), 1,
      sym_important,
    STATE(27), 1,
      aux_sym_declaration_repeat1,
    ACTIONS(206), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(97), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [2247] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(368), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(366), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2274] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(338), 1,
      anon_sym_PLUS,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(370), 1,
      anon_sym_SEMI,
    ACTIONS(372), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    ACTIONS(346), 2,
      sym_minus,
      sym_divide,
    STATE(132), 2,
      sym_plus,
      sym_times,
    STATE(237), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [2319] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(320), 1,
      anon_sym_COMMA,
    ACTIONS(374), 1,
      anon_sym_RPAREN,
    STATE(48), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    STATE(259), 1,
      aux_sym_pseudo_class_arguments_repeat2,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [2366] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(378), 1,
      anon_sym_POUND,
    ACTIONS(381), 1,
      anon_sym_LPAREN2,
    ACTIONS(387), 1,
      aux_sym_integer_value_token1,
    ACTIONS(390), 1,
      aux_sym_float_value_token1,
    ACTIONS(393), 1,
      sym_identifier,
    ACTIONS(396), 1,
      sym_plain_value,
    STATE(48), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(384), 2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(376), 3,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RPAREN,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [2409] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(401), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(399), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2436] = 4,
    ACTIONS(3), 1,
      sym_comment,
    STATE(122), 2,
      sym_plus,
      sym_times,
    ACTIONS(405), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(403), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [2465] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(409), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(407), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2492] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(411), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(413), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2519] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(417), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(415), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2546] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(421), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(419), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2573] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(425), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(423), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2600] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    STATE(48), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(427), 3,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RPAREN,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [2643] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(431), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(429), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2670] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(435), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(433), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2697] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(439), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(437), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2724] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(411), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(413), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2751] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(431), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(429), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2778] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(435), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(433), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2805] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(443), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(441), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2832] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(238), 1,
      anon_sym_LPAREN,
    STATE(109), 1,
      sym_arguments,
    ACTIONS(240), 8,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(234), 9,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      sym_string_value,
      sym_grit_metavariable,
  [2863] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(445), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(447), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2890] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(451), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(449), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2917] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(439), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(437), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2944] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(425), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(423), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2971] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(421), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(419), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [2998] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(417), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(415), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3025] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(455), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(453), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3052] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(316), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3079] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(445), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(447), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3106] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(401), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(399), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3133] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(314), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(312), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3160] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(457), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(459), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3187] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(334), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(332), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3214] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(326), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(324), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3241] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(354), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(352), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3268] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(358), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(356), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3295] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(362), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(360), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3322] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(368), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(366), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3349] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(457), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(459), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3376] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(409), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(407), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3403] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(463), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(461), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3430] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(463), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(461), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3457] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(451), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(449), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3484] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(455), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(453), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3511] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(465), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(467), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3538] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(471), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(469), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3565] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(475), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(473), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3592] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(475), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(473), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3619] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(471), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(469), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3646] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(465), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(467), 10,
      ts_builtin_sym_end,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3673] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(310), 9,
      anon_sym_ATimport,
      anon_sym_ATmedia,
      anon_sym_ATcharset,
      anon_sym_ATnamespace,
      anon_sym_ATkeyframes,
      aux_sym_keyframes_statement_token1,
      anon_sym_ATsupports,
      anon_sym_COLON,
      sym_at_keyword,
    ACTIONS(308), 10,
      anon_sym_RBRACE,
      sym_nesting_selector,
      anon_sym_STAR,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      sym_string_value,
      sym_identifier,
      sym_grit_metavariable,
  [3700] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(481), 1,
      sym_unit,
    ACTIONS(479), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(477), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [3728] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(202), 1,
      anon_sym_PLUS,
    ACTIONS(208), 2,
      sym_minus,
      sym_divide,
    STATE(122), 2,
      sym_plus,
      sym_times,
    ACTIONS(485), 4,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_identifier,
      sym_plain_value,
    ACTIONS(483), 8,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [3762] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(202), 1,
      anon_sym_PLUS,
    ACTIONS(208), 2,
      sym_minus,
      sym_divide,
    STATE(122), 2,
      sym_plus,
      sym_times,
    ACTIONS(489), 4,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_identifier,
      sym_plain_value,
    ACTIONS(487), 8,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_POUND,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [3796] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(495), 1,
      sym_unit,
    ACTIONS(493), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(491), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [3824] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    STATE(48), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(497), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [3866] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(501), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(499), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [3891] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(202), 1,
      anon_sym_PLUS,
    ACTIONS(208), 2,
      sym_minus,
      sym_divide,
    STATE(122), 2,
      sym_plus,
      sym_times,
    ACTIONS(505), 4,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_identifier,
      sym_plain_value,
    ACTIONS(503), 7,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_POUND,
      anon_sym_RPAREN,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [3924] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(507), 1,
      anon_sym_RPAREN,
    STATE(28), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [3965] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(511), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(509), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [3990] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(513), 1,
      anon_sym_RPAREN,
    STATE(26), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4031] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(517), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(515), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [4056] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(521), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(519), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [4081] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(523), 1,
      anon_sym_RPAREN,
    STATE(30), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4122] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(527), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(525), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [4147] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(531), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(529), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [4172] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(535), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(533), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [4197] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(539), 7,
      anon_sym_PLUS,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_minus,
      sym_divide,
      sym_identifier,
      sym_plain_value,
    ACTIONS(537), 10,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_POUND,
      anon_sym_RPAREN,
      sym_important,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
  [4222] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(541), 1,
      anon_sym_RPAREN,
    STATE(48), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4263] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    STATE(56), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4301] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(184), 1,
      sym_plain_value,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    STATE(113), 1,
      aux_sym_pseudo_class_arguments_repeat1,
    ACTIONS(266), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(102), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4339] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_POUND,
    ACTIONS(545), 1,
      anon_sym_LPAREN2,
    ACTIONS(549), 1,
      aux_sym_integer_value_token1,
    ACTIONS(551), 1,
      aux_sym_float_value_token1,
    ACTIONS(553), 1,
      sym_identifier,
    ACTIONS(555), 1,
      sym_plain_value,
    ACTIONS(547), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(207), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4374] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(557), 1,
      anon_sym_POUND,
    ACTIONS(559), 1,
      anon_sym_LPAREN2,
    ACTIONS(563), 1,
      aux_sym_integer_value_token1,
    ACTIONS(565), 1,
      aux_sym_float_value_token1,
    ACTIONS(567), 1,
      sym_identifier,
    ACTIONS(569), 1,
      sym_plain_value,
    ACTIONS(561), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(40), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4409] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_POUND,
    ACTIONS(545), 1,
      anon_sym_LPAREN2,
    ACTIONS(549), 1,
      aux_sym_integer_value_token1,
    ACTIONS(551), 1,
      aux_sym_float_value_token1,
    ACTIONS(553), 1,
      sym_identifier,
    ACTIONS(573), 1,
      sym_plain_value,
    ACTIONS(571), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(195), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4444] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_POUND,
    ACTIONS(545), 1,
      anon_sym_LPAREN2,
    ACTIONS(549), 1,
      aux_sym_integer_value_token1,
    ACTIONS(551), 1,
      aux_sym_float_value_token1,
    ACTIONS(553), 1,
      sym_identifier,
    ACTIONS(577), 1,
      sym_plain_value,
    ACTIONS(575), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(205), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4479] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(579), 1,
      anon_sym_SEMI,
    ACTIONS(581), 1,
      anon_sym_LBRACE,
    ACTIONS(583), 1,
      sym_grit_metavariable,
    STATE(58), 1,
      sym_block,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(218), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [4516] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(587), 1,
      sym_plain_value,
    ACTIONS(585), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(15), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4551] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(591), 1,
      sym_plain_value,
    ACTIONS(589), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(50), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4586] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(595), 1,
      sym_plain_value,
    ACTIONS(593), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(14), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4621] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(599), 1,
      sym_plain_value,
    ACTIONS(597), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(16), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4656] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(601), 1,
      anon_sym_SEMI,
    ACTIONS(603), 1,
      anon_sym_LBRACE,
    ACTIONS(605), 1,
      sym_grit_metavariable,
    STATE(62), 1,
      sym_block,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(209), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [4693] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_POUND,
    ACTIONS(545), 1,
      anon_sym_LPAREN2,
    ACTIONS(549), 1,
      aux_sym_integer_value_token1,
    ACTIONS(551), 1,
      aux_sym_float_value_token1,
    ACTIONS(553), 1,
      sym_identifier,
    ACTIONS(609), 1,
      sym_plain_value,
    ACTIONS(607), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(219), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4728] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_POUND,
    ACTIONS(545), 1,
      anon_sym_LPAREN2,
    ACTIONS(549), 1,
      aux_sym_integer_value_token1,
    ACTIONS(551), 1,
      aux_sym_float_value_token1,
    ACTIONS(553), 1,
      sym_identifier,
    ACTIONS(613), 1,
      sym_plain_value,
    ACTIONS(611), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(216), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4763] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 1,
      anon_sym_LPAREN2,
    ACTIONS(178), 1,
      aux_sym_integer_value_token1,
    ACTIONS(180), 1,
      aux_sym_float_value_token1,
    ACTIONS(200), 1,
      anon_sym_POUND,
    ACTIONS(210), 1,
      sym_identifier,
    ACTIONS(617), 1,
      sym_plain_value,
    ACTIONS(615), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(98), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4798] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_POUND,
    ACTIONS(545), 1,
      anon_sym_LPAREN2,
    ACTIONS(549), 1,
      aux_sym_integer_value_token1,
    ACTIONS(551), 1,
      aux_sym_float_value_token1,
    ACTIONS(553), 1,
      sym_identifier,
    ACTIONS(621), 1,
      sym_plain_value,
    ACTIONS(619), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(217), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4833] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_POUND,
    ACTIONS(545), 1,
      anon_sym_LPAREN2,
    ACTIONS(549), 1,
      aux_sym_integer_value_token1,
    ACTIONS(551), 1,
      aux_sym_float_value_token1,
    ACTIONS(553), 1,
      sym_identifier,
    ACTIONS(625), 1,
      sym_plain_value,
    ACTIONS(623), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(201), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4868] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(557), 1,
      anon_sym_POUND,
    ACTIONS(559), 1,
      anon_sym_LPAREN2,
    ACTIONS(563), 1,
      aux_sym_integer_value_token1,
    ACTIONS(565), 1,
      aux_sym_float_value_token1,
    ACTIONS(567), 1,
      sym_identifier,
    ACTIONS(629), 1,
      sym_plain_value,
    ACTIONS(627), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(46), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4903] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(557), 1,
      anon_sym_POUND,
    ACTIONS(559), 1,
      anon_sym_LPAREN2,
    ACTIONS(563), 1,
      aux_sym_integer_value_token1,
    ACTIONS(565), 1,
      aux_sym_float_value_token1,
    ACTIONS(567), 1,
      sym_identifier,
    ACTIONS(633), 1,
      sym_plain_value,
    ACTIONS(631), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(139), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4938] = 9,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_POUND,
    ACTIONS(545), 1,
      anon_sym_LPAREN2,
    ACTIONS(549), 1,
      aux_sym_integer_value_token1,
    ACTIONS(551), 1,
      aux_sym_float_value_token1,
    ACTIONS(553), 1,
      sym_identifier,
    ACTIONS(637), 1,
      sym_plain_value,
    ACTIONS(635), 2,
      sym_string_value,
      sym_grit_metavariable,
    STATE(210), 7,
      sym__value,
      sym_parenthesized_value,
      sym_color_value,
      sym_integer_value,
      sym_float_value,
      sym_call_expression,
      sym_binary_expression,
  [4973] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(641), 1,
      anon_sym_COLON,
    ACTIONS(643), 1,
      anon_sym_LPAREN,
    STATE(158), 1,
      sym_pseudo_class_arguments,
    ACTIONS(639), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [4999] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(647), 1,
      anon_sym_COLON,
    ACTIONS(649), 1,
      anon_sym_LPAREN,
    STATE(159), 1,
      sym_pseudo_element_arguments,
    ACTIONS(645), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5025] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(643), 1,
      anon_sym_LPAREN,
    ACTIONS(653), 1,
      anon_sym_COLON,
    STATE(142), 1,
      sym_pseudo_class_arguments,
    ACTIONS(651), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5051] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(649), 1,
      anon_sym_LPAREN,
    ACTIONS(657), 1,
      anon_sym_COLON,
    STATE(144), 1,
      sym_pseudo_element_arguments,
    ACTIONS(655), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5077] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_LPAREN,
    STATE(190), 1,
      sym_arguments,
    ACTIONS(234), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      sym_grit_metavariable,
    ACTIONS(240), 7,
      anon_sym_LPAREN2,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [5102] = 4,
    ACTIONS(3), 1,
      sym_comment,
    STATE(132), 2,
      sym_plus,
      sym_times,
    ACTIONS(403), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(405), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [5125] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(236), 1,
      anon_sym_COLON,
    ACTIONS(232), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5145] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(515), 6,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(517), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [5165] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(663), 1,
      anon_sym_COLON,
    ACTIONS(661), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5185] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(665), 1,
      sym_unit,
    ACTIONS(477), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(479), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [5207] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(669), 1,
      anon_sym_COLON,
    ACTIONS(667), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5227] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(673), 1,
      anon_sym_COLON,
    ACTIONS(671), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5247] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(675), 1,
      sym_unit,
    ACTIONS(491), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(493), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [5269] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(679), 1,
      anon_sym_COLON,
    ACTIONS(677), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5289] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(681), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(233), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5317] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(683), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(250), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5345] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(320), 1,
      anon_sym_COMMA,
    ACTIONS(322), 1,
      anon_sym_RPAREN,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(687), 1,
      anon_sym_COLON,
    ACTIONS(689), 1,
      anon_sym_COLON_COLON,
    ACTIONS(691), 1,
      anon_sym_POUND,
    ACTIONS(693), 1,
      anon_sym_LBRACK,
    ACTIONS(695), 1,
      anon_sym_GT,
    ACTIONS(697), 1,
      anon_sym_TILDE,
    ACTIONS(699), 1,
      anon_sym_PLUS,
    ACTIONS(701), 1,
      sym__descendant_operator,
    STATE(257), 1,
      aux_sym_pseudo_class_arguments_repeat2,
  [5385] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(705), 1,
      anon_sym_COLON,
    ACTIONS(703), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5405] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(320), 1,
      anon_sym_COMMA,
    ACTIONS(374), 1,
      anon_sym_RPAREN,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(687), 1,
      anon_sym_COLON,
    ACTIONS(689), 1,
      anon_sym_COLON_COLON,
    ACTIONS(691), 1,
      anon_sym_POUND,
    ACTIONS(693), 1,
      anon_sym_LBRACK,
    ACTIONS(695), 1,
      anon_sym_GT,
    ACTIONS(697), 1,
      anon_sym_TILDE,
    ACTIONS(699), 1,
      anon_sym_PLUS,
    ACTIONS(701), 1,
      sym__descendant_operator,
    STATE(260), 1,
      aux_sym_pseudo_class_arguments_repeat2,
  [5445] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(707), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(227), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5473] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(709), 1,
      sym_identifier,
    ACTIONS(711), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(258), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5501] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(713), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(225), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5529] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(715), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(228), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5557] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(717), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(232), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5585] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(721), 1,
      anon_sym_COLON,
    ACTIONS(719), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5605] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(725), 1,
      anon_sym_COLON,
    ACTIONS(723), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5625] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(729), 1,
      anon_sym_COLON,
    ACTIONS(727), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5645] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(733), 1,
      anon_sym_COLON,
    ACTIONS(731), 10,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5667] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(737), 1,
      anon_sym_COLON,
    ACTIONS(735), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5687] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(741), 1,
      anon_sym_COLON,
    ACTIONS(739), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5707] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(743), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(251), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5735] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(745), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(254), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5763] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(749), 1,
      anon_sym_COLON,
    ACTIONS(747), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5783] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(753), 1,
      anon_sym_COLON,
    ACTIONS(751), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5803] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(757), 1,
      anon_sym_COLON,
    ACTIONS(755), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5823] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(757), 1,
      anon_sym_COLON,
    ACTIONS(755), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5843] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(761), 1,
      anon_sym_COLON,
    ACTIONS(759), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5863] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(687), 1,
      anon_sym_COLON,
    ACTIONS(689), 1,
      anon_sym_COLON_COLON,
    ACTIONS(691), 1,
      anon_sym_POUND,
    ACTIONS(693), 1,
      anon_sym_LBRACK,
    ACTIONS(695), 1,
      anon_sym_GT,
    ACTIONS(697), 1,
      anon_sym_TILDE,
    ACTIONS(699), 1,
      anon_sym_PLUS,
    ACTIONS(701), 1,
      sym__descendant_operator,
    ACTIONS(763), 1,
      anon_sym_COMMA,
    ACTIONS(765), 1,
      anon_sym_LBRACE,
    STATE(266), 1,
      aux_sym_selectors_repeat1,
  [5903] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(761), 1,
      anon_sym_COLON,
    ACTIONS(759), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5923] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(340), 1,
      anon_sym_LPAREN2,
    ACTIONS(344), 1,
      anon_sym_selector,
    ACTIONS(348), 1,
      sym_identifier,
    ACTIONS(767), 1,
      sym_grit_metavariable,
    ACTIONS(342), 2,
      sym_not,
      sym_only,
    STATE(241), 6,
      sym__query,
      sym_feature_query,
      sym_parenthesized_query,
      sym_binary_query,
      sym_unary_query,
      sym_selector_query,
  [5951] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(771), 1,
      anon_sym_COLON,
    ACTIONS(769), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5971] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(775), 1,
      anon_sym_COLON,
    ACTIONS(773), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [5991] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(779), 1,
      anon_sym_COLON,
    ACTIONS(777), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6011] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(783), 1,
      anon_sym_COLON,
    ACTIONS(781), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6031] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(787), 1,
      anon_sym_COLON,
    ACTIONS(785), 10,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6053] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(791), 1,
      anon_sym_COLON,
    ACTIONS(789), 10,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6075] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(795), 1,
      anon_sym_COLON,
    ACTIONS(793), 11,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6095] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(799), 1,
      anon_sym_COLON,
    ACTIONS(797), 10,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
      anon_sym_RPAREN,
  [6117] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(509), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(511), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [6136] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(801), 1,
      anon_sym_COLON,
    ACTIONS(232), 10,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
  [6155] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(804), 1,
      anon_sym_COLON,
    ACTIONS(232), 10,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
  [6174] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(533), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(535), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [6193] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(806), 1,
      anon_sym_COLON,
    ACTIONS(232), 10,
      sym__descendant_operator,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_DOT,
      anon_sym_COLON_COLON,
      anon_sym_POUND,
      anon_sym_LBRACK,
      anon_sym_GT,
      anon_sym_TILDE,
      anon_sym_PLUS,
  [6212] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(537), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(539), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [6231] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(499), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(501), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [6250] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(519), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(521), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [6269] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(527), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [6288] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(687), 1,
      anon_sym_COLON,
    ACTIONS(689), 1,
      anon_sym_COLON_COLON,
    ACTIONS(691), 1,
      anon_sym_POUND,
    ACTIONS(693), 1,
      anon_sym_LBRACK,
    ACTIONS(695), 1,
      anon_sym_GT,
    ACTIONS(697), 1,
      anon_sym_TILDE,
    ACTIONS(699), 1,
      anon_sym_PLUS,
    ACTIONS(701), 1,
      sym__descendant_operator,
    ACTIONS(497), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [6323] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(529), 5,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_PLUS,
      anon_sym_LPAREN2,
      sym_grit_metavariable,
    ACTIONS(531), 6,
      sym_not,
      sym_only,
      anon_sym_selector,
      sym_minus,
      sym_divide,
      sym_identifier,
  [6342] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(687), 1,
      anon_sym_COLON,
    ACTIONS(689), 1,
      anon_sym_COLON_COLON,
    ACTIONS(691), 1,
      anon_sym_POUND,
    ACTIONS(693), 1,
      anon_sym_LBRACK,
    ACTIONS(695), 1,
      anon_sym_GT,
    ACTIONS(697), 1,
      anon_sym_TILDE,
    ACTIONS(699), 1,
      anon_sym_PLUS,
    ACTIONS(701), 1,
      sym__descendant_operator,
    ACTIONS(809), 2,
      anon_sym_COMMA,
      anon_sym_LBRACE,
  [6377] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(685), 1,
      anon_sym_DOT,
    ACTIONS(687), 1,
      anon_sym_COLON,
    ACTIONS(689), 1,
      anon_sym_COLON_COLON,
    ACTIONS(691), 1,
      anon_sym_POUND,
    ACTIONS(693), 1,
      anon_sym_LBRACK,
    ACTIONS(695), 1,
      anon_sym_GT,
    ACTIONS(697), 1,
      anon_sym_TILDE,
    ACTIONS(699), 1,
      anon_sym_PLUS,
    ACTIONS(701), 1,
      sym__descendant_operator,
    ACTIONS(811), 1,
      anon_sym_RPAREN,
  [6411] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(405), 1,
      sym_divide,
    STATE(118), 2,
      sym_plus,
      sym_times,
    ACTIONS(403), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6430] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(240), 1,
      sym_divide,
    ACTIONS(813), 1,
      anon_sym_LPAREN,
    STATE(214), 1,
      sym_arguments,
    ACTIONS(234), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6451] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(815), 4,
      anon_sym_POUND,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(817), 4,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_identifier,
      sym_plain_value,
  [6467] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(819), 4,
      anon_sym_POUND,
      anon_sym_LPAREN2,
      sym_string_value,
      sym_grit_metavariable,
    ACTIONS(821), 4,
      aux_sym_integer_value_token1,
      aux_sym_float_value_token1,
      sym_identifier,
      sym_plain_value,
  [6483] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(493), 1,
      sym_divide,
    ACTIONS(823), 1,
      sym_unit,
    ACTIONS(491), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6501] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(479), 1,
      sym_divide,
    ACTIONS(825), 1,
      sym_unit,
    ACTIONS(477), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6519] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(338), 1,
      anon_sym_PLUS,
    ACTIONS(827), 1,
      anon_sym_SEMI,
    ACTIONS(829), 1,
      sym_minus,
    ACTIONS(831), 1,
      sym_divide,
    STATE(118), 2,
      sym_plus,
      sym_times,
  [6542] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(517), 1,
      sym_divide,
    ACTIONS(515), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6557] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(535), 1,
      sym_divide,
    ACTIONS(533), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6572] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(833), 1,
      anon_sym_RBRACE,
    ACTIONS(837), 1,
      aux_sym_integer_value_token1,
    STATE(278), 1,
      sym_integer_value,
    ACTIONS(835), 2,
      sym_from,
      sym_to,
    STATE(208), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [6593] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(338), 1,
      anon_sym_PLUS,
    ACTIONS(829), 1,
      sym_minus,
    ACTIONS(831), 1,
      sym_divide,
    ACTIONS(839), 1,
      anon_sym_RPAREN,
    STATE(118), 2,
      sym_plus,
      sym_times,
  [6616] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(843), 1,
      anon_sym_RBRACK,
    ACTIONS(841), 6,
      sym_equal,
      sym_contains_word_equal,
      sym_starts_with_equal,
      sym_dash_equal,
      sym_contains_equal,
      sym_ends_equal,
  [6631] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(338), 1,
      anon_sym_PLUS,
    ACTIONS(829), 1,
      sym_minus,
    ACTIONS(831), 1,
      sym_divide,
    ACTIONS(845), 1,
      anon_sym_RPAREN,
    STATE(118), 2,
      sym_plus,
      sym_times,
  [6654] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(837), 1,
      aux_sym_integer_value_token1,
    ACTIONS(847), 1,
      anon_sym_RBRACE,
    STATE(278), 1,
      sym_integer_value,
    ACTIONS(835), 2,
      sym_from,
      sym_to,
    STATE(222), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [6675] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(603), 1,
      anon_sym_LBRACE,
    ACTIONS(849), 1,
      anon_sym_COMMA,
    ACTIONS(851), 1,
      anon_sym_SEMI,
    STATE(87), 1,
      sym_block,
    STATE(239), 1,
      aux_sym_at_rule_repeat1,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
  [6698] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(338), 1,
      anon_sym_PLUS,
    ACTIONS(829), 1,
      sym_minus,
    ACTIONS(831), 1,
      sym_divide,
    ACTIONS(855), 1,
      anon_sym_SEMI,
    STATE(118), 2,
      sym_plus,
      sym_times,
  [6721] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(511), 1,
      sym_divide,
    ACTIONS(509), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6736] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(531), 1,
      sym_divide,
    ACTIONS(529), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6751] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(521), 1,
      sym_divide,
    ACTIONS(519), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6766] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(527), 1,
      sym_divide,
    ACTIONS(525), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6781] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(859), 1,
      anon_sym_RBRACK,
    ACTIONS(857), 6,
      sym_equal,
      sym_contains_word_equal,
      sym_starts_with_equal,
      sym_dash_equal,
      sym_contains_equal,
      sym_ends_equal,
  [6796] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(338), 1,
      anon_sym_PLUS,
    ACTIONS(829), 1,
      sym_minus,
    ACTIONS(831), 1,
      sym_divide,
    ACTIONS(861), 1,
      anon_sym_RPAREN,
    STATE(118), 2,
      sym_plus,
      sym_times,
  [6819] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(338), 1,
      anon_sym_PLUS,
    ACTIONS(829), 1,
      sym_minus,
    ACTIONS(831), 1,
      sym_divide,
    ACTIONS(863), 1,
      anon_sym_RBRACK,
    STATE(118), 2,
      sym_plus,
      sym_times,
  [6842] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 1,
      anon_sym_LBRACE,
    ACTIONS(849), 1,
      anon_sym_COMMA,
    ACTIONS(865), 1,
      anon_sym_SEMI,
    STATE(66), 1,
      sym_block,
    STATE(236), 1,
      aux_sym_at_rule_repeat1,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
  [6865] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 1,
      anon_sym_STAR,
    ACTIONS(338), 1,
      anon_sym_PLUS,
    ACTIONS(829), 1,
      sym_minus,
    ACTIONS(831), 1,
      sym_divide,
    ACTIONS(867), 1,
      anon_sym_RBRACK,
    STATE(118), 2,
      sym_plus,
      sym_times,
  [6888] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(501), 1,
      sym_divide,
    ACTIONS(499), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6903] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(837), 1,
      aux_sym_integer_value_token1,
    ACTIONS(869), 1,
      anon_sym_RBRACE,
    STATE(278), 1,
      sym_integer_value,
    ACTIONS(835), 2,
      sym_from,
      sym_to,
    STATE(222), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [6924] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(871), 1,
      anon_sym_RBRACE,
    ACTIONS(876), 1,
      aux_sym_integer_value_token1,
    STATE(278), 1,
      sym_integer_value,
    ACTIONS(873), 2,
      sym_from,
      sym_to,
    STATE(222), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [6945] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(837), 1,
      aux_sym_integer_value_token1,
    ACTIONS(879), 1,
      anon_sym_RBRACE,
    STATE(278), 1,
      sym_integer_value,
    ACTIONS(835), 2,
      sym_from,
      sym_to,
    STATE(221), 2,
      sym_keyframe_block,
      aux_sym_keyframe_block_list_repeat1,
  [6966] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(539), 1,
      sym_divide,
    ACTIONS(537), 6,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_PLUS,
      anon_sym_RPAREN,
      sym_minus,
  [6981] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(603), 1,
      anon_sym_LBRACE,
    ACTIONS(881), 1,
      anon_sym_COMMA,
    STATE(76), 1,
      sym_block,
    STATE(245), 1,
      aux_sym_media_statement_repeat1,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
  [7001] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(883), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      sym_and,
      sym_or,
  [7013] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(885), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      sym_and,
      sym_or,
  [7025] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(887), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      sym_and,
      sym_or,
  [7037] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(889), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      sym_and,
      sym_or,
  [7049] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(891), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      sym_and,
      sym_or,
  [7061] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(893), 6,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
      anon_sym_RPAREN,
      sym_and,
      sym_or,
  [7073] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 1,
      anon_sym_LBRACE,
    ACTIONS(881), 1,
      anon_sym_COMMA,
    STATE(83), 1,
      sym_block,
    STATE(243), 1,
      aux_sym_media_statement_repeat1,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
  [7093] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
    ACTIONS(895), 3,
      anon_sym_COMMA,
      anon_sym_SEMI,
      anon_sym_LBRACE,
  [7106] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_LPAREN,
    ACTIONS(897), 1,
      sym_string_value,
    ACTIONS(899), 1,
      sym_identifier,
    STATE(190), 1,
      sym_arguments,
    STATE(309), 1,
      sym_call_expression,
  [7125] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_LPAREN,
    ACTIONS(899), 1,
      sym_identifier,
    ACTIONS(901), 1,
      sym_string_value,
    STATE(190), 1,
      sym_arguments,
    STATE(296), 1,
      sym_call_expression,
  [7144] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 1,
      anon_sym_LBRACE,
    ACTIONS(849), 1,
      anon_sym_COMMA,
    ACTIONS(903), 1,
      anon_sym_SEMI,
    STATE(33), 1,
      sym_block,
    STATE(256), 1,
      aux_sym_at_rule_repeat1,
  [7163] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(905), 1,
      anon_sym_COMMA,
    ACTIONS(907), 1,
      anon_sym_SEMI,
    STATE(264), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
  [7180] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(905), 1,
      anon_sym_COMMA,
    ACTIONS(909), 1,
      anon_sym_SEMI,
    STATE(263), 1,
      aux_sym_import_statement_repeat1,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
  [7197] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(603), 1,
      anon_sym_LBRACE,
    ACTIONS(849), 1,
      anon_sym_COMMA,
    ACTIONS(911), 1,
      anon_sym_SEMI,
    STATE(75), 1,
      sym_block,
    STATE(256), 1,
      aux_sym_at_rule_repeat1,
  [7216] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(913), 1,
      anon_sym_RPAREN,
    STATE(242), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(262), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
  [7230] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
    ACTIONS(915), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
  [7242] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(920), 1,
      anon_sym_RPAREN,
    STATE(242), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(917), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
  [7256] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 1,
      anon_sym_LBRACE,
    ACTIONS(881), 1,
      anon_sym_COMMA,
    STATE(45), 1,
      sym_block,
    STATE(265), 1,
      aux_sym_media_statement_repeat1,
  [7272] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(922), 1,
      anon_sym_RPAREN,
    STATE(242), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(262), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
  [7286] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(603), 1,
      anon_sym_LBRACE,
    ACTIONS(881), 1,
      anon_sym_COMMA,
    STATE(82), 1,
      sym_block,
    STATE(265), 1,
      aux_sym_media_statement_repeat1,
  [7302] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(924), 1,
      anon_sym_RPAREN,
    STATE(242), 1,
      aux_sym_arguments_repeat1,
    ACTIONS(262), 2,
      anon_sym_COMMA,
      anon_sym_SEMI,
  [7316] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(926), 1,
      anon_sym_COLON,
    ACTIONS(891), 3,
      anon_sym_RPAREN,
      sym_and,
      sym_or,
  [7328] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(928), 4,
      anon_sym_RBRACE,
      sym_from,
      sym_to,
      aux_sym_integer_value_token1,
  [7338] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(324), 4,
      anon_sym_RBRACE,
      sym_from,
      sym_to,
      aux_sym_integer_value_token1,
  [7348] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(603), 1,
      anon_sym_LBRACE,
    STATE(92), 1,
      sym_block,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
  [7362] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
    ACTIONS(930), 2,
      anon_sym_COMMA,
      anon_sym_LBRACE,
  [7374] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(306), 4,
      anon_sym_RBRACE,
      sym_from,
      sym_to,
      aux_sym_integer_value_token1,
  [7384] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(467), 4,
      anon_sym_RBRACE,
      sym_from,
      sym_to,
      aux_sym_integer_value_token1,
  [7394] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 1,
      anon_sym_LBRACE,
    STATE(91), 1,
      sym_block,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
  [7408] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(423), 4,
      anon_sym_RBRACE,
      sym_from,
      sym_to,
      aux_sym_integer_value_token1,
  [7418] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(932), 1,
      anon_sym_COMMA,
    STATE(256), 1,
      aux_sym_at_rule_repeat1,
    ACTIONS(935), 2,
      anon_sym_SEMI,
      anon_sym_LBRACE,
  [7432] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(320), 1,
      anon_sym_COMMA,
    ACTIONS(937), 1,
      anon_sym_RPAREN,
    STATE(267), 1,
      aux_sym_pseudo_class_arguments_repeat2,
  [7445] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(939), 1,
      anon_sym_RPAREN,
    ACTIONS(853), 2,
      sym_and,
      sym_or,
  [7456] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(320), 1,
      anon_sym_COMMA,
    ACTIONS(941), 1,
      anon_sym_RPAREN,
    STATE(267), 1,
      aux_sym_pseudo_class_arguments_repeat2,
  [7469] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(320), 1,
      anon_sym_COMMA,
    ACTIONS(943), 1,
      anon_sym_RPAREN,
    STATE(267), 1,
      aux_sym_pseudo_class_arguments_repeat2,
  [7482] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(320), 1,
      anon_sym_COMMA,
    ACTIONS(945), 1,
      anon_sym_RPAREN,
    STATE(267), 1,
      aux_sym_pseudo_class_arguments_repeat2,
  [7495] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(947), 1,
      anon_sym_COMMA,
    ACTIONS(950), 1,
      anon_sym_SEMI,
    STATE(262), 1,
      aux_sym_import_statement_repeat1,
  [7508] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(905), 1,
      anon_sym_COMMA,
    ACTIONS(952), 1,
      anon_sym_SEMI,
    STATE(262), 1,
      aux_sym_import_statement_repeat1,
  [7521] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(905), 1,
      anon_sym_COMMA,
    ACTIONS(954), 1,
      anon_sym_SEMI,
    STATE(262), 1,
      aux_sym_import_statement_repeat1,
  [7534] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(956), 1,
      anon_sym_COMMA,
    ACTIONS(959), 1,
      anon_sym_LBRACE,
    STATE(265), 1,
      aux_sym_media_statement_repeat1,
  [7547] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(763), 1,
      anon_sym_COMMA,
    ACTIONS(961), 1,
      anon_sym_LBRACE,
    STATE(270), 1,
      aux_sym_selectors_repeat1,
  [7560] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(963), 1,
      anon_sym_COMMA,
    ACTIONS(966), 1,
      anon_sym_RPAREN,
    STATE(267), 1,
      aux_sym_pseudo_class_arguments_repeat2,
  [7573] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(968), 1,
      sym_string_value,
    ACTIONS(970), 1,
      sym_identifier,
    STATE(311), 1,
      sym_call_expression,
  [7586] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(972), 1,
      sym_string_value,
    ACTIONS(974), 1,
      sym_identifier,
    STATE(308), 1,
      sym_call_expression,
  [7599] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(976), 1,
      anon_sym_COMMA,
    ACTIONS(979), 1,
      anon_sym_LBRACE,
    STATE(270), 1,
      aux_sym_selectors_repeat1,
  [7612] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_LPAREN,
    STATE(190), 1,
      sym_arguments,
  [7622] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(981), 1,
      anon_sym_LBRACE,
    STATE(90), 1,
      sym_keyframe_block_list,
  [7632] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(983), 1,
      anon_sym_SEMI,
    ACTIONS(985), 1,
      anon_sym_RBRACE,
  [7642] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(987), 1,
      anon_sym_LBRACE,
    STATE(93), 1,
      sym_keyframe_block_list,
  [7652] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(989), 1,
      anon_sym_SEMI,
    ACTIONS(991), 1,
      anon_sym_RBRACE,
  [7662] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 1,
      anon_sym_LBRACE,
    STATE(60), 1,
      sym_block,
  [7672] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(993), 1,
      aux_sym_color_value_token1,
    ACTIONS(995), 1,
      sym_identifier,
  [7682] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(997), 1,
      anon_sym_LBRACE,
    STATE(248), 1,
      sym_block,
  [7692] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(603), 1,
      anon_sym_LBRACE,
    STATE(52), 1,
      sym_block,
  [7702] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(477), 1,
      anon_sym_LBRACE,
    ACTIONS(999), 1,
      sym_unit,
  [7712] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1001), 1,
      sym_identifier,
  [7719] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1003), 1,
      anon_sym_RBRACE,
  [7726] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1005), 1,
      anon_sym_RBRACE,
  [7733] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1007), 1,
      anon_sym_SEMI,
  [7740] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1009), 1,
      sym_identifier,
  [7747] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1011), 1,
      sym_identifier,
  [7754] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(989), 1,
      anon_sym_SEMI,
  [7761] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1013), 1,
      aux_sym_color_value_token1,
  [7768] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1015), 1,
      anon_sym_RBRACE,
  [7775] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1017), 1,
      sym_identifier,
  [7782] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(983), 1,
      anon_sym_SEMI,
  [7789] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1019), 1,
      aux_sym_color_value_token1,
  [7796] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1021), 1,
      sym_identifier,
  [7803] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1023), 1,
      sym_identifier,
  [7810] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1025), 1,
      sym_identifier,
  [7817] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1027), 1,
      anon_sym_SEMI,
  [7824] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1029), 1,
      anon_sym_LPAREN2,
  [7831] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1031), 1,
      sym_identifier,
  [7838] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1033), 1,
      anon_sym_RBRACE,
  [7845] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1035), 1,
      sym_identifier,
  [7852] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1037), 1,
      sym_identifier,
  [7859] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1039), 1,
      anon_sym_RBRACE,
  [7866] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1041), 1,
      anon_sym_RBRACE,
  [7873] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1043), 1,
      sym_identifier,
  [7880] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1045), 1,
      sym_identifier,
  [7887] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1047), 1,
      anon_sym_SEMI,
  [7894] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1049), 1,
      ts_builtin_sym_end,
  [7901] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1051), 1,
      anon_sym_SEMI,
  [7908] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1053), 1,
      anon_sym_SEMI,
  [7915] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1055), 1,
      aux_sym_color_value_token1,
  [7922] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(1057), 1,
      anon_sym_SEMI,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 88,
  [SMALL_STATE(4)] = 176,
  [SMALL_STATE(5)] = 264,
  [SMALL_STATE(6)] = 352,
  [SMALL_STATE(7)] = 440,
  [SMALL_STATE(8)] = 528,
  [SMALL_STATE(9)] = 613,
  [SMALL_STATE(10)] = 698,
  [SMALL_STATE(11)] = 783,
  [SMALL_STATE(12)] = 855,
  [SMALL_STATE(13)] = 927,
  [SMALL_STATE(14)] = 996,
  [SMALL_STATE(15)] = 1060,
  [SMALL_STATE(16)] = 1121,
  [SMALL_STATE(17)] = 1182,
  [SMALL_STATE(18)] = 1221,
  [SMALL_STATE(19)] = 1260,
  [SMALL_STATE(20)] = 1293,
  [SMALL_STATE(21)] = 1336,
  [SMALL_STATE(22)] = 1379,
  [SMALL_STATE(23)] = 1422,
  [SMALL_STATE(24)] = 1465,
  [SMALL_STATE(25)] = 1508,
  [SMALL_STATE(26)] = 1551,
  [SMALL_STATE(27)] = 1599,
  [SMALL_STATE(28)] = 1645,
  [SMALL_STATE(29)] = 1693,
  [SMALL_STATE(30)] = 1743,
  [SMALL_STATE(31)] = 1791,
  [SMALL_STATE(32)] = 1818,
  [SMALL_STATE(33)] = 1845,
  [SMALL_STATE(34)] = 1872,
  [SMALL_STATE(35)] = 1899,
  [SMALL_STATE(36)] = 1946,
  [SMALL_STATE(37)] = 1973,
  [SMALL_STATE(38)] = 2000,
  [SMALL_STATE(39)] = 2047,
  [SMALL_STATE(40)] = 2074,
  [SMALL_STATE(41)] = 2119,
  [SMALL_STATE(42)] = 2146,
  [SMALL_STATE(43)] = 2173,
  [SMALL_STATE(44)] = 2200,
  [SMALL_STATE(45)] = 2247,
  [SMALL_STATE(46)] = 2274,
  [SMALL_STATE(47)] = 2319,
  [SMALL_STATE(48)] = 2366,
  [SMALL_STATE(49)] = 2409,
  [SMALL_STATE(50)] = 2436,
  [SMALL_STATE(51)] = 2465,
  [SMALL_STATE(52)] = 2492,
  [SMALL_STATE(53)] = 2519,
  [SMALL_STATE(54)] = 2546,
  [SMALL_STATE(55)] = 2573,
  [SMALL_STATE(56)] = 2600,
  [SMALL_STATE(57)] = 2643,
  [SMALL_STATE(58)] = 2670,
  [SMALL_STATE(59)] = 2697,
  [SMALL_STATE(60)] = 2724,
  [SMALL_STATE(61)] = 2751,
  [SMALL_STATE(62)] = 2778,
  [SMALL_STATE(63)] = 2805,
  [SMALL_STATE(64)] = 2832,
  [SMALL_STATE(65)] = 2863,
  [SMALL_STATE(66)] = 2890,
  [SMALL_STATE(67)] = 2917,
  [SMALL_STATE(68)] = 2944,
  [SMALL_STATE(69)] = 2971,
  [SMALL_STATE(70)] = 2998,
  [SMALL_STATE(71)] = 3025,
  [SMALL_STATE(72)] = 3052,
  [SMALL_STATE(73)] = 3079,
  [SMALL_STATE(74)] = 3106,
  [SMALL_STATE(75)] = 3133,
  [SMALL_STATE(76)] = 3160,
  [SMALL_STATE(77)] = 3187,
  [SMALL_STATE(78)] = 3214,
  [SMALL_STATE(79)] = 3241,
  [SMALL_STATE(80)] = 3268,
  [SMALL_STATE(81)] = 3295,
  [SMALL_STATE(82)] = 3322,
  [SMALL_STATE(83)] = 3349,
  [SMALL_STATE(84)] = 3376,
  [SMALL_STATE(85)] = 3403,
  [SMALL_STATE(86)] = 3430,
  [SMALL_STATE(87)] = 3457,
  [SMALL_STATE(88)] = 3484,
  [SMALL_STATE(89)] = 3511,
  [SMALL_STATE(90)] = 3538,
  [SMALL_STATE(91)] = 3565,
  [SMALL_STATE(92)] = 3592,
  [SMALL_STATE(93)] = 3619,
  [SMALL_STATE(94)] = 3646,
  [SMALL_STATE(95)] = 3673,
  [SMALL_STATE(96)] = 3700,
  [SMALL_STATE(97)] = 3728,
  [SMALL_STATE(98)] = 3762,
  [SMALL_STATE(99)] = 3796,
  [SMALL_STATE(100)] = 3824,
  [SMALL_STATE(101)] = 3866,
  [SMALL_STATE(102)] = 3891,
  [SMALL_STATE(103)] = 3924,
  [SMALL_STATE(104)] = 3965,
  [SMALL_STATE(105)] = 3990,
  [SMALL_STATE(106)] = 4031,
  [SMALL_STATE(107)] = 4056,
  [SMALL_STATE(108)] = 4081,
  [SMALL_STATE(109)] = 4122,
  [SMALL_STATE(110)] = 4147,
  [SMALL_STATE(111)] = 4172,
  [SMALL_STATE(112)] = 4197,
  [SMALL_STATE(113)] = 4222,
  [SMALL_STATE(114)] = 4263,
  [SMALL_STATE(115)] = 4301,
  [SMALL_STATE(116)] = 4339,
  [SMALL_STATE(117)] = 4374,
  [SMALL_STATE(118)] = 4409,
  [SMALL_STATE(119)] = 4444,
  [SMALL_STATE(120)] = 4479,
  [SMALL_STATE(121)] = 4516,
  [SMALL_STATE(122)] = 4551,
  [SMALL_STATE(123)] = 4586,
  [SMALL_STATE(124)] = 4621,
  [SMALL_STATE(125)] = 4656,
  [SMALL_STATE(126)] = 4693,
  [SMALL_STATE(127)] = 4728,
  [SMALL_STATE(128)] = 4763,
  [SMALL_STATE(129)] = 4798,
  [SMALL_STATE(130)] = 4833,
  [SMALL_STATE(131)] = 4868,
  [SMALL_STATE(132)] = 4903,
  [SMALL_STATE(133)] = 4938,
  [SMALL_STATE(134)] = 4973,
  [SMALL_STATE(135)] = 4999,
  [SMALL_STATE(136)] = 5025,
  [SMALL_STATE(137)] = 5051,
  [SMALL_STATE(138)] = 5077,
  [SMALL_STATE(139)] = 5102,
  [SMALL_STATE(140)] = 5125,
  [SMALL_STATE(141)] = 5145,
  [SMALL_STATE(142)] = 5165,
  [SMALL_STATE(143)] = 5185,
  [SMALL_STATE(144)] = 5207,
  [SMALL_STATE(145)] = 5227,
  [SMALL_STATE(146)] = 5247,
  [SMALL_STATE(147)] = 5269,
  [SMALL_STATE(148)] = 5289,
  [SMALL_STATE(149)] = 5317,
  [SMALL_STATE(150)] = 5345,
  [SMALL_STATE(151)] = 5385,
  [SMALL_STATE(152)] = 5405,
  [SMALL_STATE(153)] = 5445,
  [SMALL_STATE(154)] = 5473,
  [SMALL_STATE(155)] = 5501,
  [SMALL_STATE(156)] = 5529,
  [SMALL_STATE(157)] = 5557,
  [SMALL_STATE(158)] = 5585,
  [SMALL_STATE(159)] = 5605,
  [SMALL_STATE(160)] = 5625,
  [SMALL_STATE(161)] = 5645,
  [SMALL_STATE(162)] = 5667,
  [SMALL_STATE(163)] = 5687,
  [SMALL_STATE(164)] = 5707,
  [SMALL_STATE(165)] = 5735,
  [SMALL_STATE(166)] = 5763,
  [SMALL_STATE(167)] = 5783,
  [SMALL_STATE(168)] = 5803,
  [SMALL_STATE(169)] = 5823,
  [SMALL_STATE(170)] = 5843,
  [SMALL_STATE(171)] = 5863,
  [SMALL_STATE(172)] = 5903,
  [SMALL_STATE(173)] = 5923,
  [SMALL_STATE(174)] = 5951,
  [SMALL_STATE(175)] = 5971,
  [SMALL_STATE(176)] = 5991,
  [SMALL_STATE(177)] = 6011,
  [SMALL_STATE(178)] = 6031,
  [SMALL_STATE(179)] = 6053,
  [SMALL_STATE(180)] = 6075,
  [SMALL_STATE(181)] = 6095,
  [SMALL_STATE(182)] = 6117,
  [SMALL_STATE(183)] = 6136,
  [SMALL_STATE(184)] = 6155,
  [SMALL_STATE(185)] = 6174,
  [SMALL_STATE(186)] = 6193,
  [SMALL_STATE(187)] = 6212,
  [SMALL_STATE(188)] = 6231,
  [SMALL_STATE(189)] = 6250,
  [SMALL_STATE(190)] = 6269,
  [SMALL_STATE(191)] = 6288,
  [SMALL_STATE(192)] = 6323,
  [SMALL_STATE(193)] = 6342,
  [SMALL_STATE(194)] = 6377,
  [SMALL_STATE(195)] = 6411,
  [SMALL_STATE(196)] = 6430,
  [SMALL_STATE(197)] = 6451,
  [SMALL_STATE(198)] = 6467,
  [SMALL_STATE(199)] = 6483,
  [SMALL_STATE(200)] = 6501,
  [SMALL_STATE(201)] = 6519,
  [SMALL_STATE(202)] = 6542,
  [SMALL_STATE(203)] = 6557,
  [SMALL_STATE(204)] = 6572,
  [SMALL_STATE(205)] = 6593,
  [SMALL_STATE(206)] = 6616,
  [SMALL_STATE(207)] = 6631,
  [SMALL_STATE(208)] = 6654,
  [SMALL_STATE(209)] = 6675,
  [SMALL_STATE(210)] = 6698,
  [SMALL_STATE(211)] = 6721,
  [SMALL_STATE(212)] = 6736,
  [SMALL_STATE(213)] = 6751,
  [SMALL_STATE(214)] = 6766,
  [SMALL_STATE(215)] = 6781,
  [SMALL_STATE(216)] = 6796,
  [SMALL_STATE(217)] = 6819,
  [SMALL_STATE(218)] = 6842,
  [SMALL_STATE(219)] = 6865,
  [SMALL_STATE(220)] = 6888,
  [SMALL_STATE(221)] = 6903,
  [SMALL_STATE(222)] = 6924,
  [SMALL_STATE(223)] = 6945,
  [SMALL_STATE(224)] = 6966,
  [SMALL_STATE(225)] = 6981,
  [SMALL_STATE(226)] = 7001,
  [SMALL_STATE(227)] = 7013,
  [SMALL_STATE(228)] = 7025,
  [SMALL_STATE(229)] = 7037,
  [SMALL_STATE(230)] = 7049,
  [SMALL_STATE(231)] = 7061,
  [SMALL_STATE(232)] = 7073,
  [SMALL_STATE(233)] = 7093,
  [SMALL_STATE(234)] = 7106,
  [SMALL_STATE(235)] = 7125,
  [SMALL_STATE(236)] = 7144,
  [SMALL_STATE(237)] = 7163,
  [SMALL_STATE(238)] = 7180,
  [SMALL_STATE(239)] = 7197,
  [SMALL_STATE(240)] = 7216,
  [SMALL_STATE(241)] = 7230,
  [SMALL_STATE(242)] = 7242,
  [SMALL_STATE(243)] = 7256,
  [SMALL_STATE(244)] = 7272,
  [SMALL_STATE(245)] = 7286,
  [SMALL_STATE(246)] = 7302,
  [SMALL_STATE(247)] = 7316,
  [SMALL_STATE(248)] = 7328,
  [SMALL_STATE(249)] = 7338,
  [SMALL_STATE(250)] = 7348,
  [SMALL_STATE(251)] = 7362,
  [SMALL_STATE(252)] = 7374,
  [SMALL_STATE(253)] = 7384,
  [SMALL_STATE(254)] = 7394,
  [SMALL_STATE(255)] = 7408,
  [SMALL_STATE(256)] = 7418,
  [SMALL_STATE(257)] = 7432,
  [SMALL_STATE(258)] = 7445,
  [SMALL_STATE(259)] = 7456,
  [SMALL_STATE(260)] = 7469,
  [SMALL_STATE(261)] = 7482,
  [SMALL_STATE(262)] = 7495,
  [SMALL_STATE(263)] = 7508,
  [SMALL_STATE(264)] = 7521,
  [SMALL_STATE(265)] = 7534,
  [SMALL_STATE(266)] = 7547,
  [SMALL_STATE(267)] = 7560,
  [SMALL_STATE(268)] = 7573,
  [SMALL_STATE(269)] = 7586,
  [SMALL_STATE(270)] = 7599,
  [SMALL_STATE(271)] = 7612,
  [SMALL_STATE(272)] = 7622,
  [SMALL_STATE(273)] = 7632,
  [SMALL_STATE(274)] = 7642,
  [SMALL_STATE(275)] = 7652,
  [SMALL_STATE(276)] = 7662,
  [SMALL_STATE(277)] = 7672,
  [SMALL_STATE(278)] = 7682,
  [SMALL_STATE(279)] = 7692,
  [SMALL_STATE(280)] = 7702,
  [SMALL_STATE(281)] = 7712,
  [SMALL_STATE(282)] = 7719,
  [SMALL_STATE(283)] = 7726,
  [SMALL_STATE(284)] = 7733,
  [SMALL_STATE(285)] = 7740,
  [SMALL_STATE(286)] = 7747,
  [SMALL_STATE(287)] = 7754,
  [SMALL_STATE(288)] = 7761,
  [SMALL_STATE(289)] = 7768,
  [SMALL_STATE(290)] = 7775,
  [SMALL_STATE(291)] = 7782,
  [SMALL_STATE(292)] = 7789,
  [SMALL_STATE(293)] = 7796,
  [SMALL_STATE(294)] = 7803,
  [SMALL_STATE(295)] = 7810,
  [SMALL_STATE(296)] = 7817,
  [SMALL_STATE(297)] = 7824,
  [SMALL_STATE(298)] = 7831,
  [SMALL_STATE(299)] = 7838,
  [SMALL_STATE(300)] = 7845,
  [SMALL_STATE(301)] = 7852,
  [SMALL_STATE(302)] = 7859,
  [SMALL_STATE(303)] = 7866,
  [SMALL_STATE(304)] = 7873,
  [SMALL_STATE(305)] = 7880,
  [SMALL_STATE(306)] = 7887,
  [SMALL_STATE(307)] = 7894,
  [SMALL_STATE(308)] = 7901,
  [SMALL_STATE(309)] = 7908,
  [SMALL_STATE(310)] = 7915,
  [SMALL_STATE(311)] = 7922,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_stylesheet, 0),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(117),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(157),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(130),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(268),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(285),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(165),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(166),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(301),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(300),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(294),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(286),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(281),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(120),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(131),
  [39] = {.entry = {.count = 1, .reusable = false}}, SHIFT(155),
  [41] = {.entry = {.count = 1, .reusable = false}}, SHIFT(133),
  [43] = {.entry = {.count = 1, .reusable = false}}, SHIFT(269),
  [45] = {.entry = {.count = 1, .reusable = false}}, SHIFT(305),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [49] = {.entry = {.count = 1, .reusable = false}}, SHIFT(149),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [53] = {.entry = {.count = 1, .reusable = false}}, SHIFT(125),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(252),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(253),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14),
  [69] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(117),
  [72] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(157),
  [75] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(130),
  [78] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(268),
  [81] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(285),
  [84] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(165),
  [87] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(171),
  [90] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(166),
  [93] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(301),
  [96] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(300),
  [99] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(294),
  [102] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(286),
  [105] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(281),
  [108] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(183),
  [111] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 2, .production_id = 14), SHIFT_REPEAT(120),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_stylesheet, 1, .production_id = 4),
  [116] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(131),
  [119] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(155),
  [122] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(133),
  [125] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(269),
  [128] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(305),
  [131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2),
  [133] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(149),
  [136] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(171),
  [139] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(166),
  [142] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(301),
  [145] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(300),
  [148] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(294),
  [151] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(286),
  [154] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(281),
  [157] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(186),
  [160] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(125),
  [163] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(17),
  [166] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [168] = {.entry = {.count = 1, .reusable = false}}, SHIFT(301),
  [170] = {.entry = {.count = 1, .reusable = true}}, SHIFT(277),
  [172] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [174] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [176] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [178] = {.entry = {.count = 1, .reusable = false}}, SHIFT(96),
  [180] = {.entry = {.count = 1, .reusable = false}}, SHIFT(99),
  [182] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [184] = {.entry = {.count = 1, .reusable = false}}, SHIFT(102),
  [186] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [188] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [194] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [196] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_last_declaration, 3, .production_id = 44),
  [198] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [200] = {.entry = {.count = 1, .reusable = true}}, SHIFT(292),
  [202] = {.entry = {.count = 1, .reusable = false}}, SHIFT(197),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(275),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [208] = {.entry = {.count = 1, .reusable = false}}, SHIFT(122),
  [210] = {.entry = {.count = 1, .reusable = false}}, SHIFT(64),
  [212] = {.entry = {.count = 1, .reusable = false}}, SHIFT(97),
  [214] = {.entry = {.count = 1, .reusable = true}}, SHIFT(287),
  [216] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [218] = {.entry = {.count = 1, .reusable = true}}, SHIFT(306),
  [220] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 1),
  [222] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__selector, 1),
  [224] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 1),
  [226] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__selector, 1), REDUCE(aux_sym_block_repeat1, 1),
  [229] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__selector, 1), REDUCE(aux_sym_block_repeat1, 1),
  [232] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__selector, 1, .production_id = 1),
  [234] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1, .production_id = 5),
  [236] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__selector, 1, .production_id = 1),
  [238] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [240] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1, .production_id = 5),
  [242] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__value, 1),
  [244] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__selector, 1),
  [246] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__value, 1),
  [248] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [250] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [252] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [254] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [256] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [258] = {.entry = {.count = 1, .reusable = true}}, SHIFT(179),
  [260] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [262] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [264] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [266] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [268] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 65), SHIFT_REPEAT(128),
  [271] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 65),
  [273] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 65), SHIFT_REPEAT(292),
  [276] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 65), SHIFT_REPEAT(127),
  [279] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 65), SHIFT_REPEAT(97),
  [282] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 65), SHIFT_REPEAT(96),
  [285] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 65), SHIFT_REPEAT(99),
  [288] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 65), SHIFT_REPEAT(64),
  [291] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 65), SHIFT_REPEAT(97),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [298] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_last_declaration, 4, .production_id = 64),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(273),
  [302] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [304] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3, .production_id = 47),
  [306] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 47),
  [308] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_statement, 3, .production_id = 16),
  [310] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_statement, 3, .production_id = 16),
  [312] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_rule, 4, .production_id = 49),
  [314] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_at_rule, 4, .production_id = 49),
  [316] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyframe_block_list, 3, .production_id = 60),
  [318] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_keyframe_block_list, 3, .production_id = 60),
  [320] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [322] = {.entry = {.count = 1, .reusable = true}}, SHIFT(174),
  [324] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 46),
  [326] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3, .production_id = 46),
  [328] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(284),
  [332] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_rule, 4, .production_id = 48),
  [334] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_at_rule, 4, .production_id = 48),
  [336] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(197),
  [340] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [342] = {.entry = {.count = 1, .reusable = false}}, SHIFT(156),
  [344] = {.entry = {.count = 1, .reusable = false}}, SHIFT(297),
  [346] = {.entry = {.count = 1, .reusable = false}}, SHIFT(132),
  [348] = {.entry = {.count = 1, .reusable = false}}, SHIFT(230),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(238),
  [352] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 44),
  [354] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 4, .production_id = 44),
  [356] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyframe_block_list, 2),
  [358] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_keyframe_block_list, 2),
  [360] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_namespace_statement, 4, .production_id = 43),
  [362] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_namespace_statement, 4, .production_id = 43),
  [364] = {.entry = {.count = 1, .reusable = true}}, SHIFT(291),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_media_statement, 4, .production_id = 41),
  [368] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_media_statement, 4, .production_id = 41),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [372] = {.entry = {.count = 1, .reusable = true}}, SHIFT(237),
  [374] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [376] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2),
  [378] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2), SHIFT_REPEAT(292),
  [381] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2), SHIFT_REPEAT(127),
  [384] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2), SHIFT_REPEAT(102),
  [387] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2), SHIFT_REPEAT(96),
  [390] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2), SHIFT_REPEAT(99),
  [393] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2), SHIFT_REPEAT(64),
  [396] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 2), SHIFT_REPEAT(102),
  [399] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 5, .production_id = 56),
  [401] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_statement, 5, .production_id = 56),
  [403] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expression, 3, .production_id = 36),
  [405] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_expression, 3, .production_id = 36),
  [407] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 4, .production_id = 37),
  [409] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_statement, 4, .production_id = 37),
  [411] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_rule_set, 2, .production_id = 12),
  [413] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule_set, 2, .production_id = 12),
  [415] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 5, .production_id = 63),
  [417] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 5, .production_id = 63),
  [419] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 5, .production_id = 64),
  [421] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 5, .production_id = 64),
  [423] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 66),
  [425] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 4, .production_id = 66),
  [427] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2, .production_id = 54),
  [429] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_rule, 2, .production_id = 10),
  [431] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_at_rule, 2, .production_id = 10),
  [433] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_rule, 2, .production_id = 11),
  [435] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_at_rule, 2, .production_id = 11),
  [437] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 6, .production_id = 71),
  [439] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_declaration, 6, .production_id = 71),
  [441] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_stylesheet_repeat1, 1, .production_id = 2),
  [443] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_stylesheet_repeat1, 1, .production_id = 2),
  [445] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_statement, 3, .production_id = 16),
  [447] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_statement, 3, .production_id = 16),
  [449] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_rule, 3, .production_id = 26),
  [451] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_at_rule, 3, .production_id = 26),
  [453] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_rule, 3, .production_id = 25),
  [455] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_at_rule, 3, .production_id = 25),
  [457] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_media_statement, 3, .production_id = 18),
  [459] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_media_statement, 3, .production_id = 18),
  [461] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_charset_statement, 3, .production_id = 19),
  [463] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_charset_statement, 3, .production_id = 19),
  [465] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [467] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [469] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyframes_statement, 3, .production_id = 20),
  [471] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_keyframes_statement, 3, .production_id = 20),
  [473] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_supports_statement, 3, .production_id = 21),
  [475] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_supports_statement, 3, .production_id = 21),
  [477] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_integer_value, 1),
  [479] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_integer_value, 1),
  [481] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [483] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 1, .production_id = 45),
  [485] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 1, .production_id = 45),
  [487] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 54),
  [489] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_declaration_repeat1, 2, .production_id = 54),
  [491] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float_value, 1),
  [493] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_float_value, 1),
  [495] = {.entry = {.count = 1, .reusable = false}}, SHIFT(107),
  [497] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat2, 2, .production_id = 61),
  [499] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 4, .production_id = 67),
  [501] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_arguments, 4, .production_id = 67),
  [503] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 1),
  [505] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_pseudo_class_arguments_repeat1, 1),
  [507] = {.entry = {.count = 1, .reusable = true}}, SHIFT(187),
  [509] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_color_value, 2),
  [511] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_color_value, 2),
  [513] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [515] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_integer_value, 2),
  [517] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_integer_value, 2),
  [519] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float_value, 2),
  [521] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_float_value, 2),
  [523] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [525] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call_expression, 2, .production_id = 15),
  [527] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call_expression, 2, .production_id = 15),
  [529] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 3, .production_id = 54),
  [531] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_arguments, 3, .production_id = 54),
  [533] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesized_value, 3, .production_id = 16),
  [535] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesized_value, 3, .production_id = 16),
  [537] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 2),
  [539] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_arguments, 2),
  [541] = {.entry = {.count = 1, .reusable = true}}, SHIFT(226),
  [543] = {.entry = {.count = 1, .reusable = true}}, SHIFT(288),
  [545] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [547] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [549] = {.entry = {.count = 1, .reusable = false}}, SHIFT(200),
  [551] = {.entry = {.count = 1, .reusable = false}}, SHIFT(199),
  [553] = {.entry = {.count = 1, .reusable = false}}, SHIFT(196),
  [555] = {.entry = {.count = 1, .reusable = false}}, SHIFT(207),
  [557] = {.entry = {.count = 1, .reusable = true}}, SHIFT(310),
  [559] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [561] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [563] = {.entry = {.count = 1, .reusable = false}}, SHIFT(143),
  [565] = {.entry = {.count = 1, .reusable = false}}, SHIFT(146),
  [567] = {.entry = {.count = 1, .reusable = false}}, SHIFT(138),
  [569] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [573] = {.entry = {.count = 1, .reusable = false}}, SHIFT(195),
  [575] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [577] = {.entry = {.count = 1, .reusable = false}}, SHIFT(205),
  [579] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [581] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [583] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [585] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [587] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [589] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [591] = {.entry = {.count = 1, .reusable = false}}, SHIFT(50),
  [593] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [595] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [597] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [599] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [601] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [603] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [605] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [607] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [609] = {.entry = {.count = 1, .reusable = false}}, SHIFT(219),
  [611] = {.entry = {.count = 1, .reusable = true}}, SHIFT(216),
  [613] = {.entry = {.count = 1, .reusable = false}}, SHIFT(216),
  [615] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [617] = {.entry = {.count = 1, .reusable = false}}, SHIFT(98),
  [619] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [621] = {.entry = {.count = 1, .reusable = false}}, SHIFT(217),
  [623] = {.entry = {.count = 1, .reusable = true}}, SHIFT(201),
  [625] = {.entry = {.count = 1, .reusable = false}}, SHIFT(201),
  [627] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [629] = {.entry = {.count = 1, .reusable = false}}, SHIFT(46),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [633] = {.entry = {.count = 1, .reusable = false}}, SHIFT(139),
  [635] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [637] = {.entry = {.count = 1, .reusable = false}}, SHIFT(210),
  [639] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 3, .production_id = 29),
  [641] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 3, .production_id = 29),
  [643] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [645] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_selector, 3, .production_id = 30),
  [647] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_selector, 3, .production_id = 30),
  [649] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [651] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 2, .production_id = 7),
  [653] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 2, .production_id = 7),
  [655] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_selector, 2, .production_id = 8),
  [657] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_selector, 2, .production_id = 8),
  [659] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [661] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 3, .production_id = 22),
  [663] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 3, .production_id = 22),
  [665] = {.entry = {.count = 1, .reusable = false}}, SHIFT(141),
  [667] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_selector, 3, .production_id = 23),
  [669] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_selector, 3, .production_id = 23),
  [671] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_selector, 3, .production_id = 24),
  [673] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute_selector, 3, .production_id = 24),
  [675] = {.entry = {.count = 1, .reusable = false}}, SHIFT(189),
  [677] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_arguments, 2),
  [679] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_arguments, 2),
  [681] = {.entry = {.count = 1, .reusable = true}}, SHIFT(233),
  [683] = {.entry = {.count = 1, .reusable = true}}, SHIFT(250),
  [685] = {.entry = {.count = 1, .reusable = true}}, SHIFT(304),
  [687] = {.entry = {.count = 1, .reusable = false}}, SHIFT(298),
  [689] = {.entry = {.count = 1, .reusable = true}}, SHIFT(295),
  [691] = {.entry = {.count = 1, .reusable = true}}, SHIFT(293),
  [693] = {.entry = {.count = 1, .reusable = true}}, SHIFT(290),
  [695] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [697] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [699] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [701] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [703] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_arguments, 2),
  [705] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_arguments, 2),
  [707] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [709] = {.entry = {.count = 1, .reusable = false}}, SHIFT(247),
  [711] = {.entry = {.count = 1, .reusable = true}}, SHIFT(258),
  [713] = {.entry = {.count = 1, .reusable = true}}, SHIFT(225),
  [715] = {.entry = {.count = 1, .reusable = true}}, SHIFT(228),
  [717] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [719] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_selector, 4, .production_id = 51),
  [721] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_selector, 4, .production_id = 51),
  [723] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_selector, 4, .production_id = 52),
  [725] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_selector, 4, .production_id = 52),
  [727] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_selector, 4, .production_id = 53),
  [729] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute_selector, 4, .production_id = 53),
  [731] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_descendant_selector, 3, .production_id = 27),
  [733] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_descendant_selector, 3, .production_id = 27),
  [735] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_class_selector, 3, .production_id = 29),
  [737] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_class_selector, 3, .production_id = 29),
  [739] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_id_selector, 3, .production_id = 31),
  [741] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_id_selector, 3, .production_id = 31),
  [743] = {.entry = {.count = 1, .reusable = true}}, SHIFT(251),
  [745] = {.entry = {.count = 1, .reusable = true}}, SHIFT(254),
  [747] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_universal_selector, 1),
  [749] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_universal_selector, 1),
  [751] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_selector, 6, .production_id = 72),
  [753] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute_selector, 6, .production_id = 72),
  [755] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_arguments, 4, .production_id = 69),
  [757] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_arguments, 4, .production_id = 69),
  [759] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_arguments, 4, .production_id = 69),
  [761] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_arguments, 4, .production_id = 69),
  [763] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [765] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 1, .production_id = 3),
  [767] = {.entry = {.count = 1, .reusable = true}}, SHIFT(241),
  [769] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_class_arguments, 3, .production_id = 61),
  [771] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_class_arguments, 3, .production_id = 61),
  [773] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_class_selector, 2, .production_id = 7),
  [775] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_class_selector, 2, .production_id = 7),
  [777] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_id_selector, 2, .production_id = 9),
  [779] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_id_selector, 2, .production_id = 9),
  [781] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pseudo_element_arguments, 3, .production_id = 61),
  [783] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pseudo_element_arguments, 3, .production_id = 61),
  [785] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_child_selector, 3, .production_id = 32),
  [787] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_child_selector, 3, .production_id = 32),
  [789] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sibling_selector, 3, .production_id = 33),
  [791] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sibling_selector, 3, .production_id = 33),
  [793] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute_selector, 5, .production_id = 62),
  [795] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute_selector, 5, .production_id = 62),
  [797] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_adjacent_sibling_selector, 3, .production_id = 34),
  [799] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_adjacent_sibling_selector, 3, .production_id = 34),
  [801] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__selector, 1, .production_id = 1), SHIFT(124),
  [804] = {.entry = {.count = 1, .reusable = false}}, SHIFT(123),
  [806] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__selector, 1, .production_id = 1), SHIFT(121),
  [809] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2, .production_id = 28),
  [811] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [813] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [815] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_plus, 1),
  [817] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_plus, 1),
  [819] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_times, 1),
  [821] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_times, 1),
  [823] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [825] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [827] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [829] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [831] = {.entry = {.count = 1, .reusable = false}}, SHIFT(118),
  [833] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [835] = {.entry = {.count = 1, .reusable = true}}, SHIFT(278),
  [837] = {.entry = {.count = 1, .reusable = true}}, SHIFT(280),
  [839] = {.entry = {.count = 1, .reusable = true}}, SHIFT(185),
  [841] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [843] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [845] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [847] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [849] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [851] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [853] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [855] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [857] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [859] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [861] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [863] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [865] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [867] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [869] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [871] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_keyframe_block_list_repeat1, 2),
  [873] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_keyframe_block_list_repeat1, 2), SHIFT_REPEAT(278),
  [876] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_keyframe_block_list_repeat1, 2), SHIFT_REPEAT(280),
  [879] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [881] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [883] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_feature_query, 5, .production_id = 68),
  [885] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_query, 3, .production_id = 40),
  [887] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_query, 2, .production_id = 17),
  [889] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector_query, 4, .production_id = 58),
  [891] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__query, 1, .production_id = 6),
  [893] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesized_query, 3, .production_id = 38),
  [895] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_at_rule_repeat1, 2, .production_id = 38),
  [897] = {.entry = {.count = 1, .reusable = true}}, SHIFT(309),
  [899] = {.entry = {.count = 1, .reusable = true}}, SHIFT(271),
  [901] = {.entry = {.count = 1, .reusable = true}}, SHIFT(296),
  [903] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [905] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [907] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [909] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [911] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [913] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [915] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_import_statement_repeat1, 2, .production_id = 55),
  [917] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2, .production_id = 65), SHIFT_REPEAT(114),
  [920] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2, .production_id = 65),
  [922] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [924] = {.entry = {.count = 1, .reusable = true}}, SHIFT(188),
  [926] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [928] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyframe_block, 2, .production_id = 59),
  [930] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_media_statement_repeat1, 2, .production_id = 39),
  [932] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_at_rule_repeat1, 2, .production_id = 50), SHIFT_REPEAT(148),
  [935] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_at_rule_repeat1, 2, .production_id = 50),
  [937] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [939] = {.entry = {.count = 1, .reusable = true}}, SHIFT(231),
  [941] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [943] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [945] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [947] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_import_statement_repeat1, 2, .production_id = 57), SHIFT_REPEAT(173),
  [950] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_import_statement_repeat1, 2, .production_id = 57),
  [952] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [954] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [956] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_media_statement_repeat1, 2, .production_id = 42), SHIFT_REPEAT(164),
  [959] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_media_statement_repeat1, 2, .production_id = 42),
  [961] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 2, .production_id = 13),
  [963] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat2, 2, .production_id = 70), SHIFT_REPEAT(13),
  [966] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_pseudo_class_arguments_repeat2, 2, .production_id = 70),
  [968] = {.entry = {.count = 1, .reusable = true}}, SHIFT(311),
  [970] = {.entry = {.count = 1, .reusable = true}}, SHIFT(234),
  [972] = {.entry = {.count = 1, .reusable = true}}, SHIFT(308),
  [974] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [976] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2, .production_id = 35), SHIFT_REPEAT(22),
  [979] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2, .production_id = 35),
  [981] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [983] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [985] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_last_declaration, 5, .production_id = 71),
  [987] = {.entry = {.count = 1, .reusable = true}}, SHIFT(204),
  [989] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [991] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_last_declaration, 4, .production_id = 63),
  [993] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [995] = {.entry = {.count = 1, .reusable = false}}, SHIFT(176),
  [997] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [999] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [1001] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [1003] = {.entry = {.count = 1, .reusable = true}}, SHIFT(249),
  [1005] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [1007] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [1009] = {.entry = {.count = 1, .reusable = true}}, SHIFT(272),
  [1011] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [1013] = {.entry = {.count = 1, .reusable = true}}, SHIFT(211),
  [1015] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [1017] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [1019] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [1021] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [1023] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [1025] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [1027] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [1029] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [1031] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [1033] = {.entry = {.count = 1, .reusable = true}}, SHIFT(255),
  [1035] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [1037] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [1039] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [1041] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [1043] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [1045] = {.entry = {.count = 1, .reusable = true}}, SHIFT(274),
  [1047] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [1049] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [1051] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [1053] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [1055] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [1057] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
};

enum ts_external_scanner_symbol_identifiers {
  ts_external_token__descendant_operator = 0,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token__descendant_operator] = sym__descendant_operator,
};

static const bool ts_external_scanner_states[2][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token__descendant_operator] = true,
  },
};

#ifdef __cplusplus
extern "C" {
#endif
void *tree_sitter_css_external_scanner_create(void);
void tree_sitter_css_external_scanner_destroy(void *);
bool tree_sitter_css_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_css_external_scanner_serialize(void *, char *);
void tree_sitter_css_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_css(void) {
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
      tree_sitter_css_external_scanner_create,
      tree_sitter_css_external_scanner_destroy,
      tree_sitter_css_external_scanner_scan,
      tree_sitter_css_external_scanner_serialize,
      tree_sitter_css_external_scanner_deserialize,
    },
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif

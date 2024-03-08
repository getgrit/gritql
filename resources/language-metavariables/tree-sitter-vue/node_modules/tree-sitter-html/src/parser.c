#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 94
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 41
#define ALIAS_COUNT 0
#define TOKEN_COUNT 25
#define EXTERNAL_TOKEN_COUNT 9
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 4
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_LT_BANG = 1,
  aux_sym_doctype_token1 = 2,
  anon_sym_GT = 3,
  sym__doctype = 4,
  anon_sym_LT = 5,
  anon_sym_SLASH_GT = 6,
  anon_sym_LT_SLASH = 7,
  anon_sym_EQ = 8,
  sym_attribute_name = 9,
  sym_attribute_value = 10,
  sym_entity = 11,
  anon_sym_SQUOTE = 12,
  aux_sym_quoted_attribute_value_token1 = 13,
  anon_sym_DQUOTE = 14,
  aux_sym_quoted_attribute_value_token2 = 15,
  sym_text = 16,
  sym__start_tag_name = 17,
  sym__script_start_tag_name = 18,
  sym__style_start_tag_name = 19,
  sym__end_tag_name = 20,
  sym_erroneous_end_tag_name = 21,
  sym__implicit_end_tag = 22,
  sym_raw_text = 23,
  sym_comment = 24,
  sym_fragment = 25,
  sym_doctype = 26,
  sym__node = 27,
  sym_element = 28,
  sym_script_element = 29,
  sym_style_element = 30,
  sym_start_tag = 31,
  sym_script_start_tag = 32,
  sym_style_start_tag = 33,
  sym_self_closing_tag = 34,
  sym_end_tag = 35,
  sym_erroneous_end_tag = 36,
  sym_attribute = 37,
  sym_quoted_attribute_value = 38,
  aux_sym_fragment_repeat1 = 39,
  aux_sym_start_tag_repeat1 = 40,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_LT_BANG] = "<!",
  [aux_sym_doctype_token1] = "doctype_token1",
  [anon_sym_GT] = ">",
  [sym__doctype] = "doctype",
  [anon_sym_LT] = "<",
  [anon_sym_SLASH_GT] = "/>",
  [anon_sym_LT_SLASH] = "</",
  [anon_sym_EQ] = "=",
  [sym_attribute_name] = "attribute_name",
  [sym_attribute_value] = "attribute_value",
  [sym_entity] = "entity",
  [anon_sym_SQUOTE] = "'",
  [aux_sym_quoted_attribute_value_token1] = "attribute_value",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_quoted_attribute_value_token2] = "attribute_value",
  [sym_text] = "text",
  [sym__start_tag_name] = "tag_name",
  [sym__script_start_tag_name] = "tag_name",
  [sym__style_start_tag_name] = "tag_name",
  [sym__end_tag_name] = "tag_name",
  [sym_erroneous_end_tag_name] = "erroneous_end_tag_name",
  [sym__implicit_end_tag] = "_implicit_end_tag",
  [sym_raw_text] = "raw_text",
  [sym_comment] = "comment",
  [sym_fragment] = "fragment",
  [sym_doctype] = "doctype",
  [sym__node] = "_node",
  [sym_element] = "element",
  [sym_script_element] = "script_element",
  [sym_style_element] = "style_element",
  [sym_start_tag] = "start_tag",
  [sym_script_start_tag] = "start_tag",
  [sym_style_start_tag] = "start_tag",
  [sym_self_closing_tag] = "self_closing_tag",
  [sym_end_tag] = "end_tag",
  [sym_erroneous_end_tag] = "erroneous_end_tag",
  [sym_attribute] = "attribute",
  [sym_quoted_attribute_value] = "quoted_attribute_value",
  [aux_sym_fragment_repeat1] = "fragment_repeat1",
  [aux_sym_start_tag_repeat1] = "start_tag_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_LT_BANG] = anon_sym_LT_BANG,
  [aux_sym_doctype_token1] = aux_sym_doctype_token1,
  [anon_sym_GT] = anon_sym_GT,
  [sym__doctype] = sym__doctype,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_SLASH_GT] = anon_sym_SLASH_GT,
  [anon_sym_LT_SLASH] = anon_sym_LT_SLASH,
  [anon_sym_EQ] = anon_sym_EQ,
  [sym_attribute_name] = sym_attribute_name,
  [sym_attribute_value] = sym_attribute_value,
  [sym_entity] = sym_entity,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [aux_sym_quoted_attribute_value_token1] = sym_attribute_value,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_quoted_attribute_value_token2] = sym_attribute_value,
  [sym_text] = sym_text,
  [sym__start_tag_name] = sym__start_tag_name,
  [sym__script_start_tag_name] = sym__start_tag_name,
  [sym__style_start_tag_name] = sym__start_tag_name,
  [sym__end_tag_name] = sym__start_tag_name,
  [sym_erroneous_end_tag_name] = sym_erroneous_end_tag_name,
  [sym__implicit_end_tag] = sym__implicit_end_tag,
  [sym_raw_text] = sym_raw_text,
  [sym_comment] = sym_comment,
  [sym_fragment] = sym_fragment,
  [sym_doctype] = sym_doctype,
  [sym__node] = sym__node,
  [sym_element] = sym_element,
  [sym_script_element] = sym_script_element,
  [sym_style_element] = sym_style_element,
  [sym_start_tag] = sym_start_tag,
  [sym_script_start_tag] = sym_start_tag,
  [sym_style_start_tag] = sym_start_tag,
  [sym_self_closing_tag] = sym_self_closing_tag,
  [sym_end_tag] = sym_end_tag,
  [sym_erroneous_end_tag] = sym_erroneous_end_tag,
  [sym_attribute] = sym_attribute,
  [sym_quoted_attribute_value] = sym_quoted_attribute_value,
  [aux_sym_fragment_repeat1] = aux_sym_fragment_repeat1,
  [aux_sym_start_tag_repeat1] = aux_sym_start_tag_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_LT_BANG] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_doctype_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [sym__doctype] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [sym_attribute_name] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute_value] = {
    .visible = true,
    .named = true,
  },
  [sym_entity] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_quoted_attribute_value_token1] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_quoted_attribute_value_token2] = {
    .visible = true,
    .named = true,
  },
  [sym_text] = {
    .visible = true,
    .named = true,
  },
  [sym__start_tag_name] = {
    .visible = true,
    .named = true,
  },
  [sym__script_start_tag_name] = {
    .visible = true,
    .named = true,
  },
  [sym__style_start_tag_name] = {
    .visible = true,
    .named = true,
  },
  [sym__end_tag_name] = {
    .visible = true,
    .named = true,
  },
  [sym_erroneous_end_tag_name] = {
    .visible = true,
    .named = true,
  },
  [sym__implicit_end_tag] = {
    .visible = false,
    .named = true,
  },
  [sym_raw_text] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_fragment] = {
    .visible = true,
    .named = true,
  },
  [sym_doctype] = {
    .visible = true,
    .named = true,
  },
  [sym__node] = {
    .visible = false,
    .named = true,
  },
  [sym_element] = {
    .visible = true,
    .named = true,
  },
  [sym_script_element] = {
    .visible = true,
    .named = true,
  },
  [sym_style_element] = {
    .visible = true,
    .named = true,
  },
  [sym_start_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_script_start_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_style_start_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_self_closing_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_end_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_erroneous_end_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute] = {
    .visible = true,
    .named = true,
  },
  [sym_quoted_attribute_value] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_fragment_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_start_tag_repeat1] = {
    .visible = false,
    .named = false,
  },
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
  [4] = 4,
  [5] = 4,
  [6] = 6,
  [7] = 6,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 11,
  [14] = 14,
  [15] = 15,
  [16] = 15,
  [17] = 12,
  [18] = 18,
  [19] = 19,
  [20] = 10,
  [21] = 21,
  [22] = 9,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 18,
  [29] = 19,
  [30] = 27,
  [31] = 14,
  [32] = 21,
  [33] = 23,
  [34] = 25,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 36,
  [39] = 35,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 43,
  [45] = 45,
  [46] = 46,
  [47] = 37,
  [48] = 48,
  [49] = 42,
  [50] = 50,
  [51] = 51,
  [52] = 51,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 54,
  [57] = 55,
  [58] = 53,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 62,
  [67] = 67,
  [68] = 63,
  [69] = 48,
  [70] = 50,
  [71] = 64,
  [72] = 65,
  [73] = 73,
  [74] = 67,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 82,
  [84] = 76,
  [85] = 85,
  [86] = 78,
  [87] = 87,
  [88] = 87,
  [89] = 81,
  [90] = 79,
  [91] = 80,
  [92] = 85,
  [93] = 75,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(57);
      if (lookahead == '"') ADVANCE(73);
      if (lookahead == '&') ADVANCE(3);
      if (lookahead == '\'') ADVANCE(70);
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '<') ADVANCE(63);
      if (lookahead == '=') ADVANCE(66);
      if (lookahead == '>') ADVANCE(61);
      if (lookahead == 'D' ||
          lookahead == 'd') ADVANCE(48);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(73);
      if (lookahead == '\'') ADVANCE(70);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (lookahead != 0 &&
          (lookahead < '<' || '>' < lookahead)) ADVANCE(68);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(73);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(74);
      if (lookahead != 0) ADVANCE(75);
      END_STATE();
    case 3:
      if (lookahead == '#') ADVANCE(51);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 4:
      if (lookahead == '\'') ADVANCE(70);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(71);
      if (lookahead != 0) ADVANCE(72);
      END_STATE();
    case 5:
      if (lookahead == '/') ADVANCE(45);
      if (lookahead == '=') ADVANCE(66);
      if (lookahead == '>') ADVANCE(61);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(5)
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\'' &&
          lookahead != '<') ADVANCE(67);
      END_STATE();
    case 6:
      if (lookahead == ';') ADVANCE(69);
      END_STATE();
    case 7:
      if (lookahead == ';') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(6);
      END_STATE();
    case 8:
      if (lookahead == ';') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(7);
      END_STATE();
    case 9:
      if (lookahead == ';') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(8);
      END_STATE();
    case 10:
      if (lookahead == ';') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(9);
      END_STATE();
    case 11:
      if (lookahead == ';') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(6);
      END_STATE();
    case 12:
      if (lookahead == ';') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(11);
      END_STATE();
    case 13:
      if (lookahead == ';') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(12);
      END_STATE();
    case 14:
      if (lookahead == ';') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(13);
      END_STATE();
    case 15:
      if (lookahead == ';') ADVANCE(69);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(14);
      END_STATE();
    case 16:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(6);
      END_STATE();
    case 17:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(16);
      END_STATE();
    case 18:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(17);
      END_STATE();
    case 19:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(18);
      END_STATE();
    case 20:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 21:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 22:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(21);
      END_STATE();
    case 23:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(22);
      END_STATE();
    case 24:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(23);
      END_STATE();
    case 25:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(24);
      END_STATE();
    case 26:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(25);
      END_STATE();
    case 27:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 28:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(27);
      END_STATE();
    case 29:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(28);
      END_STATE();
    case 30:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 31:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 32:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(31);
      END_STATE();
    case 33:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(32);
      END_STATE();
    case 34:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(33);
      END_STATE();
    case 35:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 36:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(35);
      END_STATE();
    case 37:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(36);
      END_STATE();
    case 38:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(37);
      END_STATE();
    case 39:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 40:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(39);
      END_STATE();
    case 41:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(40);
      END_STATE();
    case 42:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(41);
      END_STATE();
    case 43:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(42);
      END_STATE();
    case 44:
      if (lookahead == ';') ADVANCE(69);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 45:
      if (lookahead == '>') ADVANCE(64);
      END_STATE();
    case 46:
      if (lookahead == 'C' ||
          lookahead == 'c') ADVANCE(50);
      END_STATE();
    case 47:
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(62);
      END_STATE();
    case 48:
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(46);
      END_STATE();
    case 49:
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(47);
      END_STATE();
    case 50:
      if (lookahead == 'T' ||
          lookahead == 't') ADVANCE(52);
      END_STATE();
    case 51:
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(55);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(10);
      END_STATE();
    case 52:
      if (lookahead == 'Y' ||
          lookahead == 'y') ADVANCE(49);
      END_STATE();
    case 53:
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(53);
      if (lookahead != 0 &&
          lookahead != '&' &&
          lookahead != '<' &&
          lookahead != '>') ADVANCE(76);
      END_STATE();
    case 54:
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(59);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(60);
      END_STATE();
    case 55:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(15);
      END_STATE();
    case 56:
      if (eof) ADVANCE(57);
      if (lookahead == '&') ADVANCE(3);
      if (lookahead == '<') ADVANCE(63);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(56)
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(76);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(anon_sym_LT_BANG);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(aux_sym_doctype_token1);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(59);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(60);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(aux_sym_doctype_token1);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(60);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(sym__doctype);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '!') ADVANCE(58);
      if (lookahead == '/') ADVANCE(65);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_SLASH_GT);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_LT_SLASH);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_attribute_name);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != '\'' &&
          lookahead != '/' &&
          (lookahead < '<' || '>' < lookahead)) ADVANCE(67);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_attribute_value);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != '\'' &&
          (lookahead < '<' || '>' < lookahead)) ADVANCE(68);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_entity);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(aux_sym_quoted_attribute_value_token1);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(71);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(72);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(aux_sym_quoted_attribute_value_token1);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(72);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(aux_sym_quoted_attribute_value_token2);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(74);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(75);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(aux_sym_quoted_attribute_value_token2);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(75);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(53);
      if (lookahead != 0 &&
          lookahead != '&' &&
          lookahead != '<' &&
          lookahead != '>') ADVANCE(76);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 56, .external_lex_state = 2},
  [2] = {.lex_state = 56, .external_lex_state = 3},
  [3] = {.lex_state = 56, .external_lex_state = 3},
  [4] = {.lex_state = 56, .external_lex_state = 3},
  [5] = {.lex_state = 56, .external_lex_state = 3},
  [6] = {.lex_state = 56, .external_lex_state = 3},
  [7] = {.lex_state = 56, .external_lex_state = 2},
  [8] = {.lex_state = 56, .external_lex_state = 2},
  [9] = {.lex_state = 56, .external_lex_state = 2},
  [10] = {.lex_state = 56, .external_lex_state = 2},
  [11] = {.lex_state = 56, .external_lex_state = 2},
  [12] = {.lex_state = 56, .external_lex_state = 3},
  [13] = {.lex_state = 56, .external_lex_state = 3},
  [14] = {.lex_state = 56, .external_lex_state = 2},
  [15] = {.lex_state = 56, .external_lex_state = 2},
  [16] = {.lex_state = 56, .external_lex_state = 3},
  [17] = {.lex_state = 56, .external_lex_state = 2},
  [18] = {.lex_state = 56, .external_lex_state = 3},
  [19] = {.lex_state = 56, .external_lex_state = 3},
  [20] = {.lex_state = 56, .external_lex_state = 3},
  [21] = {.lex_state = 56, .external_lex_state = 2},
  [22] = {.lex_state = 56, .external_lex_state = 3},
  [23] = {.lex_state = 56, .external_lex_state = 2},
  [24] = {.lex_state = 56, .external_lex_state = 3},
  [25] = {.lex_state = 56, .external_lex_state = 3},
  [26] = {.lex_state = 56, .external_lex_state = 3},
  [27] = {.lex_state = 56, .external_lex_state = 2},
  [28] = {.lex_state = 56, .external_lex_state = 2},
  [29] = {.lex_state = 56, .external_lex_state = 2},
  [30] = {.lex_state = 56, .external_lex_state = 3},
  [31] = {.lex_state = 56, .external_lex_state = 3},
  [32] = {.lex_state = 56, .external_lex_state = 3},
  [33] = {.lex_state = 56, .external_lex_state = 3},
  [34] = {.lex_state = 56, .external_lex_state = 2},
  [35] = {.lex_state = 5, .external_lex_state = 4},
  [36] = {.lex_state = 5, .external_lex_state = 4},
  [37] = {.lex_state = 5, .external_lex_state = 4},
  [38] = {.lex_state = 5, .external_lex_state = 4},
  [39] = {.lex_state = 5, .external_lex_state = 4},
  [40] = {.lex_state = 5, .external_lex_state = 2},
  [41] = {.lex_state = 5, .external_lex_state = 2},
  [42] = {.lex_state = 5, .external_lex_state = 4},
  [43] = {.lex_state = 1, .external_lex_state = 2},
  [44] = {.lex_state = 1, .external_lex_state = 2},
  [45] = {.lex_state = 5, .external_lex_state = 2},
  [46] = {.lex_state = 5, .external_lex_state = 2},
  [47] = {.lex_state = 5, .external_lex_state = 2},
  [48] = {.lex_state = 5, .external_lex_state = 4},
  [49] = {.lex_state = 5, .external_lex_state = 2},
  [50] = {.lex_state = 5, .external_lex_state = 4},
  [51] = {.lex_state = 0, .external_lex_state = 5},
  [52] = {.lex_state = 0, .external_lex_state = 5},
  [53] = {.lex_state = 5, .external_lex_state = 4},
  [54] = {.lex_state = 0, .external_lex_state = 6},
  [55] = {.lex_state = 0, .external_lex_state = 6},
  [56] = {.lex_state = 0, .external_lex_state = 6},
  [57] = {.lex_state = 0, .external_lex_state = 6},
  [58] = {.lex_state = 5, .external_lex_state = 2},
  [59] = {.lex_state = 0, .external_lex_state = 6},
  [60] = {.lex_state = 0, .external_lex_state = 6},
  [61] = {.lex_state = 0, .external_lex_state = 6},
  [62] = {.lex_state = 0, .external_lex_state = 7},
  [63] = {.lex_state = 0, .external_lex_state = 2},
  [64] = {.lex_state = 2, .external_lex_state = 2},
  [65] = {.lex_state = 0, .external_lex_state = 2},
  [66] = {.lex_state = 0, .external_lex_state = 7},
  [67] = {.lex_state = 4, .external_lex_state = 2},
  [68] = {.lex_state = 0, .external_lex_state = 2},
  [69] = {.lex_state = 5, .external_lex_state = 2},
  [70] = {.lex_state = 5, .external_lex_state = 2},
  [71] = {.lex_state = 2, .external_lex_state = 2},
  [72] = {.lex_state = 0, .external_lex_state = 2},
  [73] = {.lex_state = 0, .external_lex_state = 6},
  [74] = {.lex_state = 4, .external_lex_state = 2},
  [75] = {.lex_state = 0, .external_lex_state = 2},
  [76] = {.lex_state = 0, .external_lex_state = 2},
  [77] = {.lex_state = 0, .external_lex_state = 2},
  [78] = {.lex_state = 0, .external_lex_state = 2},
  [79] = {.lex_state = 54, .external_lex_state = 2},
  [80] = {.lex_state = 0, .external_lex_state = 2},
  [81] = {.lex_state = 0, .external_lex_state = 8},
  [82] = {.lex_state = 0, .external_lex_state = 2},
  [83] = {.lex_state = 0, .external_lex_state = 2},
  [84] = {.lex_state = 0, .external_lex_state = 2},
  [85] = {.lex_state = 0, .external_lex_state = 9},
  [86] = {.lex_state = 0, .external_lex_state = 2},
  [87] = {.lex_state = 0, .external_lex_state = 2},
  [88] = {.lex_state = 0, .external_lex_state = 2},
  [89] = {.lex_state = 0, .external_lex_state = 8},
  [90] = {.lex_state = 54, .external_lex_state = 2},
  [91] = {.lex_state = 0, .external_lex_state = 2},
  [92] = {.lex_state = 0, .external_lex_state = 9},
  [93] = {.lex_state = 0, .external_lex_state = 2},
};

enum {
  ts_external_token__start_tag_name = 0,
  ts_external_token__script_start_tag_name = 1,
  ts_external_token__style_start_tag_name = 2,
  ts_external_token__end_tag_name = 3,
  ts_external_token_erroneous_end_tag_name = 4,
  ts_external_token_SLASH_GT = 5,
  ts_external_token__implicit_end_tag = 6,
  ts_external_token_raw_text = 7,
  ts_external_token_comment = 8,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token__start_tag_name] = sym__start_tag_name,
  [ts_external_token__script_start_tag_name] = sym__script_start_tag_name,
  [ts_external_token__style_start_tag_name] = sym__style_start_tag_name,
  [ts_external_token__end_tag_name] = sym__end_tag_name,
  [ts_external_token_erroneous_end_tag_name] = sym_erroneous_end_tag_name,
  [ts_external_token_SLASH_GT] = anon_sym_SLASH_GT,
  [ts_external_token__implicit_end_tag] = sym__implicit_end_tag,
  [ts_external_token_raw_text] = sym_raw_text,
  [ts_external_token_comment] = sym_comment,
};

static const bool ts_external_scanner_states[10][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token__start_tag_name] = true,
    [ts_external_token__script_start_tag_name] = true,
    [ts_external_token__style_start_tag_name] = true,
    [ts_external_token__end_tag_name] = true,
    [ts_external_token_erroneous_end_tag_name] = true,
    [ts_external_token_SLASH_GT] = true,
    [ts_external_token__implicit_end_tag] = true,
    [ts_external_token_raw_text] = true,
    [ts_external_token_comment] = true,
  },
  [2] = {
    [ts_external_token_comment] = true,
  },
  [3] = {
    [ts_external_token__implicit_end_tag] = true,
    [ts_external_token_comment] = true,
  },
  [4] = {
    [ts_external_token_SLASH_GT] = true,
    [ts_external_token_comment] = true,
  },
  [5] = {
    [ts_external_token__start_tag_name] = true,
    [ts_external_token__script_start_tag_name] = true,
    [ts_external_token__style_start_tag_name] = true,
    [ts_external_token_comment] = true,
  },
  [6] = {
    [ts_external_token_raw_text] = true,
    [ts_external_token_comment] = true,
  },
  [7] = {
    [ts_external_token__end_tag_name] = true,
    [ts_external_token_erroneous_end_tag_name] = true,
    [ts_external_token_comment] = true,
  },
  [8] = {
    [ts_external_token_erroneous_end_tag_name] = true,
    [ts_external_token_comment] = true,
  },
  [9] = {
    [ts_external_token__end_tag_name] = true,
    [ts_external_token_comment] = true,
  },
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_LT_BANG] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [sym__doctype] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_SLASH_GT] = ACTIONS(1),
    [anon_sym_LT_SLASH] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [sym_entity] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [sym__start_tag_name] = ACTIONS(1),
    [sym__script_start_tag_name] = ACTIONS(1),
    [sym__style_start_tag_name] = ACTIONS(1),
    [sym__end_tag_name] = ACTIONS(1),
    [sym_erroneous_end_tag_name] = ACTIONS(1),
    [sym__implicit_end_tag] = ACTIONS(1),
    [sym_raw_text] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_fragment] = STATE(77),
    [sym_doctype] = STATE(8),
    [sym__node] = STATE(8),
    [sym_element] = STATE(8),
    [sym_script_element] = STATE(8),
    [sym_style_element] = STATE(8),
    [sym_start_tag] = STATE(4),
    [sym_script_start_tag] = STATE(54),
    [sym_style_start_tag] = STATE(57),
    [sym_self_closing_tag] = STATE(11),
    [sym_erroneous_end_tag] = STATE(8),
    [aux_sym_fragment_repeat1] = STATE(8),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_LT_BANG] = ACTIONS(7),
    [anon_sym_LT] = ACTIONS(9),
    [anon_sym_LT_SLASH] = ACTIONS(11),
    [sym_entity] = ACTIONS(13),
    [sym_text] = ACTIONS(13),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_LT_BANG,
    ACTIONS(17), 1,
      anon_sym_LT,
    ACTIONS(19), 1,
      anon_sym_LT_SLASH,
    ACTIONS(23), 1,
      sym__implicit_end_tag,
    STATE(5), 1,
      sym_start_tag,
    STATE(9), 1,
      sym_end_tag,
    STATE(13), 1,
      sym_self_closing_tag,
    STATE(55), 1,
      sym_style_start_tag,
    STATE(56), 1,
      sym_script_start_tag,
    ACTIONS(21), 2,
      sym_entity,
      sym_text,
    STATE(6), 7,
      sym_doctype,
      sym__node,
      sym_element,
      sym_script_element,
      sym_style_element,
      sym_erroneous_end_tag,
      aux_sym_fragment_repeat1,
  [44] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_LT_BANG,
    ACTIONS(17), 1,
      anon_sym_LT,
    ACTIONS(25), 1,
      anon_sym_LT_SLASH,
    ACTIONS(27), 1,
      sym__implicit_end_tag,
    STATE(5), 1,
      sym_start_tag,
    STATE(13), 1,
      sym_self_closing_tag,
    STATE(22), 1,
      sym_end_tag,
    STATE(55), 1,
      sym_style_start_tag,
    STATE(56), 1,
      sym_script_start_tag,
    ACTIONS(21), 2,
      sym_entity,
      sym_text,
    STATE(6), 7,
      sym_doctype,
      sym__node,
      sym_element,
      sym_script_element,
      sym_style_element,
      sym_erroneous_end_tag,
      aux_sym_fragment_repeat1,
  [88] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_LT_BANG,
    ACTIONS(17), 1,
      anon_sym_LT,
    ACTIONS(19), 1,
      anon_sym_LT_SLASH,
    ACTIONS(31), 1,
      sym__implicit_end_tag,
    STATE(5), 1,
      sym_start_tag,
    STATE(13), 1,
      sym_self_closing_tag,
    STATE(17), 1,
      sym_end_tag,
    STATE(55), 1,
      sym_style_start_tag,
    STATE(56), 1,
      sym_script_start_tag,
    ACTIONS(29), 2,
      sym_entity,
      sym_text,
    STATE(2), 7,
      sym_doctype,
      sym__node,
      sym_element,
      sym_script_element,
      sym_style_element,
      sym_erroneous_end_tag,
      aux_sym_fragment_repeat1,
  [132] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_LT_BANG,
    ACTIONS(17), 1,
      anon_sym_LT,
    ACTIONS(25), 1,
      anon_sym_LT_SLASH,
    ACTIONS(35), 1,
      sym__implicit_end_tag,
    STATE(5), 1,
      sym_start_tag,
    STATE(12), 1,
      sym_end_tag,
    STATE(13), 1,
      sym_self_closing_tag,
    STATE(55), 1,
      sym_style_start_tag,
    STATE(56), 1,
      sym_script_start_tag,
    ACTIONS(33), 2,
      sym_entity,
      sym_text,
    STATE(3), 7,
      sym_doctype,
      sym__node,
      sym_element,
      sym_script_element,
      sym_style_element,
      sym_erroneous_end_tag,
      aux_sym_fragment_repeat1,
  [176] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 1,
      anon_sym_LT_BANG,
    ACTIONS(40), 1,
      anon_sym_LT,
    ACTIONS(43), 1,
      anon_sym_LT_SLASH,
    ACTIONS(49), 1,
      sym__implicit_end_tag,
    STATE(5), 1,
      sym_start_tag,
    STATE(13), 1,
      sym_self_closing_tag,
    STATE(55), 1,
      sym_style_start_tag,
    STATE(56), 1,
      sym_script_start_tag,
    ACTIONS(46), 2,
      sym_entity,
      sym_text,
    STATE(6), 7,
      sym_doctype,
      sym__node,
      sym_element,
      sym_script_element,
      sym_style_element,
      sym_erroneous_end_tag,
      aux_sym_fragment_repeat1,
  [217] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(49), 1,
      ts_builtin_sym_end,
    ACTIONS(51), 1,
      anon_sym_LT_BANG,
    ACTIONS(54), 1,
      anon_sym_LT,
    ACTIONS(57), 1,
      anon_sym_LT_SLASH,
    STATE(4), 1,
      sym_start_tag,
    STATE(11), 1,
      sym_self_closing_tag,
    STATE(54), 1,
      sym_script_start_tag,
    STATE(57), 1,
      sym_style_start_tag,
    ACTIONS(60), 2,
      sym_entity,
      sym_text,
    STATE(7), 7,
      sym_doctype,
      sym__node,
      sym_element,
      sym_script_element,
      sym_style_element,
      sym_erroneous_end_tag,
      aux_sym_fragment_repeat1,
  [258] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_LT_BANG,
    ACTIONS(9), 1,
      anon_sym_LT,
    ACTIONS(11), 1,
      anon_sym_LT_SLASH,
    ACTIONS(63), 1,
      ts_builtin_sym_end,
    STATE(4), 1,
      sym_start_tag,
    STATE(11), 1,
      sym_self_closing_tag,
    STATE(54), 1,
      sym_script_start_tag,
    STATE(57), 1,
      sym_style_start_tag,
    ACTIONS(65), 2,
      sym_entity,
      sym_text,
    STATE(7), 7,
      sym_doctype,
      sym__node,
      sym_element,
      sym_script_element,
      sym_style_element,
      sym_erroneous_end_tag,
      aux_sym_fragment_repeat1,
  [299] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(69), 1,
      anon_sym_LT,
    ACTIONS(67), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [313] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(73), 1,
      anon_sym_LT,
    ACTIONS(71), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [327] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 1,
      anon_sym_LT,
    ACTIONS(75), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [341] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(81), 1,
      anon_sym_LT,
    ACTIONS(79), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [355] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 1,
      anon_sym_LT,
    ACTIONS(75), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [369] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(85), 1,
      anon_sym_LT,
    ACTIONS(83), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [383] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(89), 1,
      anon_sym_LT,
    ACTIONS(87), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [397] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(89), 1,
      anon_sym_LT,
    ACTIONS(87), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [411] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(81), 1,
      anon_sym_LT,
    ACTIONS(79), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [425] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(93), 1,
      anon_sym_LT,
    ACTIONS(91), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [439] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(97), 1,
      anon_sym_LT,
    ACTIONS(95), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [453] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(73), 1,
      anon_sym_LT,
    ACTIONS(71), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [467] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(101), 1,
      anon_sym_LT,
    ACTIONS(99), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [481] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(69), 1,
      anon_sym_LT,
    ACTIONS(67), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [495] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(105), 1,
      anon_sym_LT,
    ACTIONS(103), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [509] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(109), 1,
      anon_sym_LT,
    ACTIONS(107), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [523] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(113), 1,
      anon_sym_LT,
    ACTIONS(111), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [537] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      anon_sym_LT,
    ACTIONS(115), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [551] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(121), 1,
      anon_sym_LT,
    ACTIONS(119), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [565] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(93), 1,
      anon_sym_LT,
    ACTIONS(91), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [579] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(97), 1,
      anon_sym_LT,
    ACTIONS(95), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [593] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(121), 1,
      anon_sym_LT,
    ACTIONS(119), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [607] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(85), 1,
      anon_sym_LT,
    ACTIONS(83), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [621] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(101), 1,
      anon_sym_LT,
    ACTIONS(99), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [635] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(105), 1,
      anon_sym_LT,
    ACTIONS(103), 5,
      sym__implicit_end_tag,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [649] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(113), 1,
      anon_sym_LT,
    ACTIONS(111), 5,
      ts_builtin_sym_end,
      anon_sym_LT_BANG,
      anon_sym_LT_SLASH,
      sym_entity,
      sym_text,
  [663] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(123), 1,
      anon_sym_GT,
    ACTIONS(125), 1,
      anon_sym_SLASH_GT,
    ACTIONS(127), 1,
      sym_attribute_name,
    STATE(38), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [680] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(127), 1,
      sym_attribute_name,
    ACTIONS(129), 1,
      anon_sym_GT,
    ACTIONS(131), 1,
      anon_sym_SLASH_GT,
    STATE(37), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [697] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(135), 1,
      sym_attribute_name,
    ACTIONS(133), 2,
      anon_sym_GT,
      anon_sym_SLASH_GT,
    STATE(37), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [712] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(127), 1,
      sym_attribute_name,
    ACTIONS(129), 1,
      anon_sym_GT,
    ACTIONS(138), 1,
      anon_sym_SLASH_GT,
    STATE(37), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [729] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(123), 1,
      anon_sym_GT,
    ACTIONS(127), 1,
      sym_attribute_name,
    ACTIONS(140), 1,
      anon_sym_SLASH_GT,
    STATE(36), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [746] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(142), 1,
      anon_sym_GT,
    ACTIONS(144), 1,
      sym_attribute_name,
    STATE(47), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [760] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(144), 1,
      sym_attribute_name,
    ACTIONS(146), 1,
      anon_sym_GT,
    STATE(47), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [774] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(150), 1,
      anon_sym_EQ,
    ACTIONS(148), 3,
      anon_sym_GT,
      anon_sym_SLASH_GT,
      sym_attribute_name,
  [786] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(152), 1,
      sym_attribute_value,
    ACTIONS(154), 1,
      anon_sym_SQUOTE,
    ACTIONS(156), 1,
      anon_sym_DQUOTE,
    STATE(48), 1,
      sym_quoted_attribute_value,
  [802] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(158), 1,
      sym_attribute_value,
    ACTIONS(160), 1,
      anon_sym_SQUOTE,
    ACTIONS(162), 1,
      anon_sym_DQUOTE,
    STATE(69), 1,
      sym_quoted_attribute_value,
  [818] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(144), 1,
      sym_attribute_name,
    ACTIONS(164), 1,
      anon_sym_GT,
    STATE(40), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [832] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(144), 1,
      sym_attribute_name,
    ACTIONS(166), 1,
      anon_sym_GT,
    STATE(41), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [846] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(133), 1,
      anon_sym_GT,
    ACTIONS(168), 1,
      sym_attribute_name,
    STATE(47), 2,
      sym_attribute,
      aux_sym_start_tag_repeat1,
  [860] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 3,
      anon_sym_GT,
      anon_sym_SLASH_GT,
      sym_attribute_name,
  [869] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(173), 1,
      anon_sym_EQ,
    ACTIONS(148), 2,
      anon_sym_GT,
      sym_attribute_name,
  [880] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 3,
      anon_sym_GT,
      anon_sym_SLASH_GT,
      sym_attribute_name,
  [889] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(177), 1,
      sym__start_tag_name,
    ACTIONS(179), 1,
      sym__script_start_tag_name,
    ACTIONS(181), 1,
      sym__style_start_tag_name,
  [902] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(179), 1,
      sym__script_start_tag_name,
    ACTIONS(181), 1,
      sym__style_start_tag_name,
    ACTIONS(183), 1,
      sym__start_tag_name,
  [915] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(185), 3,
      anon_sym_GT,
      anon_sym_SLASH_GT,
      sym_attribute_name,
  [924] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 1,
      anon_sym_LT_SLASH,
    ACTIONS(189), 1,
      sym_raw_text,
    STATE(21), 1,
      sym_end_tag,
  [937] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_LT_SLASH,
    ACTIONS(193), 1,
      sym_raw_text,
    STATE(33), 1,
      sym_end_tag,
  [950] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_LT_SLASH,
    ACTIONS(195), 1,
      sym_raw_text,
    STATE(32), 1,
      sym_end_tag,
  [963] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 1,
      anon_sym_LT_SLASH,
    ACTIONS(197), 1,
      sym_raw_text,
    STATE(23), 1,
      sym_end_tag,
  [976] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(185), 2,
      anon_sym_GT,
      sym_attribute_name,
  [984] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(199), 2,
      sym_raw_text,
      anon_sym_LT_SLASH,
  [992] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 2,
      sym_raw_text,
      anon_sym_LT_SLASH,
  [1000] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(203), 2,
      sym_raw_text,
      anon_sym_LT_SLASH,
  [1008] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(205), 1,
      sym__end_tag_name,
    ACTIONS(207), 1,
      sym_erroneous_end_tag_name,
  [1018] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 1,
      anon_sym_LT_SLASH,
    STATE(29), 1,
      sym_end_tag,
  [1028] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DQUOTE,
    ACTIONS(211), 1,
      aux_sym_quoted_attribute_value_token2,
  [1038] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 1,
      anon_sym_LT_SLASH,
    STATE(10), 1,
      sym_end_tag,
  [1048] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(207), 1,
      sym_erroneous_end_tag_name,
    ACTIONS(213), 1,
      sym__end_tag_name,
  [1058] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_SQUOTE,
    ACTIONS(215), 1,
      aux_sym_quoted_attribute_value_token1,
  [1068] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_LT_SLASH,
    STATE(19), 1,
      sym_end_tag,
  [1078] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 2,
      anon_sym_GT,
      sym_attribute_name,
  [1086] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 2,
      anon_sym_GT,
      sym_attribute_name,
  [1094] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(217), 1,
      anon_sym_DQUOTE,
    ACTIONS(219), 1,
      aux_sym_quoted_attribute_value_token2,
  [1104] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 1,
      anon_sym_LT_SLASH,
    STATE(20), 1,
      sym_end_tag,
  [1114] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(221), 2,
      sym_raw_text,
      anon_sym_LT_SLASH,
  [1122] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(217), 1,
      anon_sym_SQUOTE,
    ACTIONS(223), 1,
      aux_sym_quoted_attribute_value_token1,
  [1132] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(225), 1,
      sym__doctype,
  [1139] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(227), 1,
      anon_sym_GT,
  [1146] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(229), 1,
      ts_builtin_sym_end,
  [1153] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(231), 1,
      anon_sym_SQUOTE,
  [1160] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(233), 1,
      aux_sym_doctype_token1,
  [1167] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(235), 1,
      anon_sym_GT,
  [1174] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(237), 1,
      sym_erroneous_end_tag_name,
  [1181] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(239), 1,
      anon_sym_GT,
  [1188] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(241), 1,
      anon_sym_GT,
  [1195] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 1,
      anon_sym_GT,
  [1202] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 1,
      sym__end_tag_name,
  [1209] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(245), 1,
      anon_sym_SQUOTE,
  [1216] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(245), 1,
      anon_sym_DQUOTE,
  [1223] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(231), 1,
      anon_sym_DQUOTE,
  [1230] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(207), 1,
      sym_erroneous_end_tag_name,
  [1237] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(247), 1,
      aux_sym_doctype_token1,
  [1244] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(249), 1,
      anon_sym_GT,
  [1251] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(205), 1,
      sym__end_tag_name,
  [1258] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(251), 1,
      sym__doctype,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 44,
  [SMALL_STATE(4)] = 88,
  [SMALL_STATE(5)] = 132,
  [SMALL_STATE(6)] = 176,
  [SMALL_STATE(7)] = 217,
  [SMALL_STATE(8)] = 258,
  [SMALL_STATE(9)] = 299,
  [SMALL_STATE(10)] = 313,
  [SMALL_STATE(11)] = 327,
  [SMALL_STATE(12)] = 341,
  [SMALL_STATE(13)] = 355,
  [SMALL_STATE(14)] = 369,
  [SMALL_STATE(15)] = 383,
  [SMALL_STATE(16)] = 397,
  [SMALL_STATE(17)] = 411,
  [SMALL_STATE(18)] = 425,
  [SMALL_STATE(19)] = 439,
  [SMALL_STATE(20)] = 453,
  [SMALL_STATE(21)] = 467,
  [SMALL_STATE(22)] = 481,
  [SMALL_STATE(23)] = 495,
  [SMALL_STATE(24)] = 509,
  [SMALL_STATE(25)] = 523,
  [SMALL_STATE(26)] = 537,
  [SMALL_STATE(27)] = 551,
  [SMALL_STATE(28)] = 565,
  [SMALL_STATE(29)] = 579,
  [SMALL_STATE(30)] = 593,
  [SMALL_STATE(31)] = 607,
  [SMALL_STATE(32)] = 621,
  [SMALL_STATE(33)] = 635,
  [SMALL_STATE(34)] = 649,
  [SMALL_STATE(35)] = 663,
  [SMALL_STATE(36)] = 680,
  [SMALL_STATE(37)] = 697,
  [SMALL_STATE(38)] = 712,
  [SMALL_STATE(39)] = 729,
  [SMALL_STATE(40)] = 746,
  [SMALL_STATE(41)] = 760,
  [SMALL_STATE(42)] = 774,
  [SMALL_STATE(43)] = 786,
  [SMALL_STATE(44)] = 802,
  [SMALL_STATE(45)] = 818,
  [SMALL_STATE(46)] = 832,
  [SMALL_STATE(47)] = 846,
  [SMALL_STATE(48)] = 860,
  [SMALL_STATE(49)] = 869,
  [SMALL_STATE(50)] = 880,
  [SMALL_STATE(51)] = 889,
  [SMALL_STATE(52)] = 902,
  [SMALL_STATE(53)] = 915,
  [SMALL_STATE(54)] = 924,
  [SMALL_STATE(55)] = 937,
  [SMALL_STATE(56)] = 950,
  [SMALL_STATE(57)] = 963,
  [SMALL_STATE(58)] = 976,
  [SMALL_STATE(59)] = 984,
  [SMALL_STATE(60)] = 992,
  [SMALL_STATE(61)] = 1000,
  [SMALL_STATE(62)] = 1008,
  [SMALL_STATE(63)] = 1018,
  [SMALL_STATE(64)] = 1028,
  [SMALL_STATE(65)] = 1038,
  [SMALL_STATE(66)] = 1048,
  [SMALL_STATE(67)] = 1058,
  [SMALL_STATE(68)] = 1068,
  [SMALL_STATE(69)] = 1078,
  [SMALL_STATE(70)] = 1086,
  [SMALL_STATE(71)] = 1094,
  [SMALL_STATE(72)] = 1104,
  [SMALL_STATE(73)] = 1114,
  [SMALL_STATE(74)] = 1122,
  [SMALL_STATE(75)] = 1132,
  [SMALL_STATE(76)] = 1139,
  [SMALL_STATE(77)] = 1146,
  [SMALL_STATE(78)] = 1153,
  [SMALL_STATE(79)] = 1160,
  [SMALL_STATE(80)] = 1167,
  [SMALL_STATE(81)] = 1174,
  [SMALL_STATE(82)] = 1181,
  [SMALL_STATE(83)] = 1188,
  [SMALL_STATE(84)] = 1195,
  [SMALL_STATE(85)] = 1202,
  [SMALL_STATE(86)] = 1209,
  [SMALL_STATE(87)] = 1216,
  [SMALL_STATE(88)] = 1223,
  [SMALL_STATE(89)] = 1230,
  [SMALL_STATE(90)] = 1237,
  [SMALL_STATE(91)] = 1244,
  [SMALL_STATE(92)] = 1251,
  [SMALL_STATE(93)] = 1258,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(52),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(51),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [37] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(93),
  [40] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(51),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(89),
  [46] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(6),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fragment_repeat1, 2),
  [51] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(75),
  [54] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(52),
  [57] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(81),
  [60] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(7),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment, 1),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_element, 3),
  [69] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_element, 3),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_script_element, 3),
  [73] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_script_element, 3),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_element, 1),
  [77] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_element, 1),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_element, 2),
  [81] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_element, 2),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_end_tag, 3),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_end_tag, 3),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_self_closing_tag, 4),
  [89] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_self_closing_tag, 4),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_doctype, 4),
  [93] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_doctype, 4),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_style_element, 3),
  [97] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_style_element, 3),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_script_element, 2),
  [101] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_script_element, 2),
  [103] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_style_element, 2),
  [105] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_style_element, 2),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_start_tag, 4),
  [109] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_start_tag, 4),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_erroneous_end_tag, 3),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_erroneous_end_tag, 3),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_start_tag, 3),
  [117] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_start_tag, 3),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_self_closing_tag, 3),
  [121] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_self_closing_tag, 3),
  [123] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [127] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [129] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_start_tag_repeat1, 2),
  [135] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_start_tag_repeat1, 2), SHIFT_REPEAT(42),
  [138] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [142] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [144] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [146] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [148] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 1),
  [150] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [152] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [154] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [156] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [158] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [160] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [162] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [164] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [166] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [168] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_start_tag_repeat1, 2), SHIFT_REPEAT(49),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 3),
  [173] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_quoted_attribute_value, 2),
  [177] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_quoted_attribute_value, 3),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [195] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [197] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_script_start_tag, 4),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_style_start_tag, 3),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_script_start_tag, 3),
  [205] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [209] = {.entry = {.count = 1, .reusable = false}}, SHIFT(70),
  [211] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [215] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [217] = {.entry = {.count = 1, .reusable = false}}, SHIFT(50),
  [219] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [221] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_style_start_tag, 4),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [225] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [227] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [229] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [233] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [237] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
};

#ifdef __cplusplus
extern "C" {
#endif
void *tree_sitter_html_external_scanner_create(void);
void tree_sitter_html_external_scanner_destroy(void *);
bool tree_sitter_html_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_html_external_scanner_serialize(void *, char *);
void tree_sitter_html_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_html(void) {
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
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .external_scanner = {
      &ts_external_scanner_states[0][0],
      ts_external_scanner_symbol_map,
      tree_sitter_html_external_scanner_create,
      tree_sitter_html_external_scanner_destroy,
      tree_sitter_html_external_scanner_scan,
      tree_sitter_html_external_scanner_serialize,
      tree_sitter_html_external_scanner_deserialize,
    },
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif

#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 4
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 6
#define ALIAS_COUNT 0
#define TOKEN_COUNT 5
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 1
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_null = 1,
  sym_bool = 2,
  sym_int = 3,
  sym_float = 4,
  sym_scalar = 5,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_null] = "null",
  [sym_bool] = "bool",
  [sym_int] = "int",
  [sym_float] = "float",
  [sym_scalar] = "scalar",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_null] = sym_null,
  [sym_bool] = sym_bool,
  [sym_int] = sym_int,
  [sym_float] = sym_float,
  [sym_scalar] = sym_scalar,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_null] = {
    .visible = true,
    .named = true,
  },
  [sym_bool] = {
    .visible = true,
    .named = true,
  },
  [sym_int] = {
    .visible = true,
    .named = true,
  },
  [sym_float] = {
    .visible = true,
    .named = true,
  },
  [sym_scalar] = {
    .visible = true,
    .named = true,
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
  [3] = 3,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(34);
      ADVANCE_MAP(
        '.', 6,
        '0', 37,
        'F', 2,
        'N', 16,
        'T', 13,
        'f', 17,
        'n', 29,
        't', 26,
        '~', 35,
        '+', 1,
        '-', 1,
      );
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(38);
      END_STATE();
    case 1:
      if (lookahead == '.') ADVANCE(7);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(38);
      END_STATE();
    case 2:
      if (lookahead == 'A') ADVANCE(9);
      if (lookahead == 'a') ADVANCE(22);
      END_STATE();
    case 3:
      if (lookahead == 'A') ADVANCE(12);
      if (lookahead == 'a') ADVANCE(12);
      END_STATE();
    case 4:
      if (lookahead == 'E') ADVANCE(36);
      END_STATE();
    case 5:
      if (lookahead == 'F') ADVANCE(41);
      END_STATE();
    case 6:
      if (lookahead == 'I') ADVANCE(11);
      if (lookahead == 'N') ADVANCE(3);
      if (lookahead == 'i') ADVANCE(24);
      if (lookahead == 'n') ADVANCE(18);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(42);
      END_STATE();
    case 7:
      if (lookahead == 'I') ADVANCE(11);
      if (lookahead == 'i') ADVANCE(24);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(42);
      END_STATE();
    case 8:
      if (lookahead == 'L') ADVANCE(35);
      END_STATE();
    case 9:
      if (lookahead == 'L') ADVANCE(14);
      END_STATE();
    case 10:
      if (lookahead == 'L') ADVANCE(8);
      END_STATE();
    case 11:
      if (lookahead == 'N') ADVANCE(5);
      if (lookahead == 'n') ADVANCE(20);
      END_STATE();
    case 12:
      if (lookahead == 'N') ADVANCE(41);
      END_STATE();
    case 13:
      if (lookahead == 'R') ADVANCE(15);
      if (lookahead == 'r') ADVANCE(28);
      END_STATE();
    case 14:
      if (lookahead == 'S') ADVANCE(4);
      END_STATE();
    case 15:
      if (lookahead == 'U') ADVANCE(4);
      END_STATE();
    case 16:
      if (lookahead == 'U') ADVANCE(10);
      if (lookahead == 'u') ADVANCE(23);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(22);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(25);
      END_STATE();
    case 19:
      if (lookahead == 'e') ADVANCE(36);
      END_STATE();
    case 20:
      if (lookahead == 'f') ADVANCE(41);
      END_STATE();
    case 21:
      if (lookahead == 'l') ADVANCE(35);
      END_STATE();
    case 22:
      if (lookahead == 'l') ADVANCE(27);
      END_STATE();
    case 23:
      if (lookahead == 'l') ADVANCE(21);
      END_STATE();
    case 24:
      if (lookahead == 'n') ADVANCE(20);
      END_STATE();
    case 25:
      if (lookahead == 'n') ADVANCE(41);
      END_STATE();
    case 26:
      if (lookahead == 'r') ADVANCE(28);
      END_STATE();
    case 27:
      if (lookahead == 's') ADVANCE(19);
      END_STATE();
    case 28:
      if (lookahead == 'u') ADVANCE(19);
      END_STATE();
    case 29:
      if (lookahead == 'u') ADVANCE(23);
      END_STATE();
    case 30:
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(32);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(43);
      END_STATE();
    case 31:
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(39);
      END_STATE();
    case 32:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(43);
      END_STATE();
    case 33:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(40);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_null);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_bool);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_int);
      if (lookahead == '.') ADVANCE(42);
      if (lookahead == 'o') ADVANCE(31);
      if (lookahead == 'x') ADVANCE(33);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(38);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_int);
      if (lookahead == '.') ADVANCE(42);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(38);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_int);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(39);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_int);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(40);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_float);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_float);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(42);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_float);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(43);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_null] = ACTIONS(1),
    [sym_bool] = ACTIONS(1),
    [sym_int] = ACTIONS(1),
    [sym_float] = ACTIONS(1),
  },
  [1] = {
    [sym_scalar] = STATE(3),
    [sym_null] = ACTIONS(3),
    [sym_bool] = ACTIONS(3),
    [sym_int] = ACTIONS(5),
    [sym_float] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(7), 1,
      ts_builtin_sym_end,
  [4] = 1,
    ACTIONS(9), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 4,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_scalar, 1, 0, 0),
  [9] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_core_schema(void) {
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
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif

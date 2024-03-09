#include <tree_sitter/parser.h>

#include "./tree_sitter_html/scanner.cc"

extern "C" {

void *tree_sitter_vue_external_scanner_create() {
  return new Scanner();
}

void tree_sitter_vue_external_scanner_destroy(void *payload) {
  Scanner *scanner = static_cast<Scanner *>(payload);
  delete scanner;
}

unsigned tree_sitter_vue_external_scanner_serialize(void *payload, char *buffer) {
  Scanner *scanner = static_cast<Scanner *>(payload);
  return scanner->serialize(buffer);
}

void tree_sitter_vue_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
  Scanner *scanner = static_cast<Scanner *>(payload);
  scanner->deserialize(buffer, length);
}

bool tree_sitter_vue_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
  bool is_error_recovery = valid_symbols[START_TAG_NAME] && valid_symbols[RAW_TEXT];
  if (!is_error_recovery) {
    if (lexer->lookahead != '<' && (valid_symbols[TEXT_FRAGMENT] || valid_symbols[INTERPOLATION_TEXT])) {
      bool has_text = false;
      for (;; has_text = true) {
        if (lexer->lookahead == 0) {
          lexer->mark_end(lexer);
          break;
        } else if (lexer->lookahead == '<') {
          lexer->mark_end(lexer);
          lexer->advance(lexer, false);
          if (iswalpha(lexer->lookahead) || lexer->lookahead == '!' || lexer->lookahead == '?' || lexer->lookahead == '/') break;
        } else if (lexer->lookahead == '{') {
          lexer->mark_end(lexer);
          lexer->advance(lexer, false);
          if (lexer->lookahead == '{') break;
        } else if (lexer->lookahead == '}' && valid_symbols[INTERPOLATION_TEXT]) {
          lexer->mark_end(lexer);
          lexer->advance(lexer, false);
          if (lexer->lookahead == '}') {
            lexer->result_symbol = INTERPOLATION_TEXT;
            return has_text;
          }
        } else {
          lexer->advance(lexer, false);
        }
      }
      if (has_text) {
        lexer->result_symbol = TEXT_FRAGMENT;
        return true;
      }
    }
  }
  Scanner *scanner = static_cast<Scanner *>(payload);
  return scanner->scan(lexer, valid_symbols);
}

}

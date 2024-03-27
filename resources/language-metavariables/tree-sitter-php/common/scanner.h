#include "tree_sitter/parser.h"

#include <assert.h>
#include <ctype.h>
#include <string.h>
#include <wctype.h>
#include <wchar.h>

#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define VEC_RESIZE(vec, _cap)                                                                                          \
    void *tmp = realloc((vec).data, (_cap) * sizeof((vec).data[0]));                                                   \
    assert(tmp != NULL);                                                                                               \
    (vec).data = tmp;                                                                                                  \
    assert((vec).data != NULL);                                                                                        \
    (vec).cap = (_cap);

#define VEC_PUSH(vec, el)                                                                                              \
    if ((vec).cap == (vec).len) {                                                                                      \
        VEC_RESIZE((vec), MAX(16, (vec).len * 2));                                                                     \
    }                                                                                                                  \
    (vec).data[(vec).len++] = (el);

#define VEC_POP(vec)                                                                                                   \
    {                                                                                                                  \
        STRING_FREE(VEC_BACK((vec)).word);                                                                             \
        (vec).len--;                                                                                                   \
    }

#define VEC_BACK(vec) ((vec).data[(vec).len - 1])

#define VEC_FREE(vec)                                                                                                  \
    {                                                                                                                  \
        if ((vec).data != NULL)                                                                                        \
            free((vec).data);                                                                                          \
    }

#define VEC_CLEAR(vec)                                                                                                 \
    {                                                                                                                  \
        for (uint32_t i = 0; i < (vec).len; i++) {                                                                     \
            STRING_FREE((vec).data[i].word);                                                                           \
        }                                                                                                              \
        (vec).len = 0;                                                                                                 \
    }

#define STRING_RESIZE(vec, _cap)                                                                                       \
    void *tmp = realloc((vec).data, (_cap + 1) * sizeof((vec).data[0]));                                               \
    assert(tmp != NULL);                                                                                               \
    (vec).data = tmp;                                                                                                  \
    memset((vec).data + (vec).len, 0, ((_cap + 1) - (vec).len) * sizeof((vec).data[0]));                               \
    (vec).cap = (_cap);

#define STRING_GROW(vec, _cap)                                                                                         \
    if ((vec).cap < (_cap)) {                                                                                          \
        STRING_RESIZE((vec), (_cap));                                                                                  \
    }

#define STRING_PUSH(vec, el)                                                                                           \
    if ((vec).cap == (vec).len) {                                                                                      \
        STRING_RESIZE((vec), MAX(16, (vec).len * 2));                                                                  \
    }                                                                                                                  \
    (vec).data[(vec).len++] = (el);

#define STRING_FREE(vec)                                                                                               \
    {                                                                                                                  \
        if ((vec).data != NULL)                                                                                        \
            free((vec).data);                                                                                          \
    }

enum TokenType {
    AUTOMATIC_SEMICOLON,
    ENCAPSED_STRING_CHARS,
    ENCAPSED_STRING_CHARS_AFTER_VARIABLE,
    EXECUTION_STRING_CHARS,
    EXECUTION_STRING_CHARS_AFTER_VARIABLE,
    ENCAPSED_STRING_CHARS_HEREDOC,
    ENCAPSED_STRING_CHARS_AFTER_VARIABLE_HEREDOC,
    EOF_TOKEN,
    HEREDOC_START,
    HEREDOC_END,
    NOWDOC_STRING,
    SENTINEL_ERROR, // Unused token used to indicate error recovery mode
};

typedef struct {
    uint32_t len;
    uint32_t cap;
    wchar_t *data;
} String;

static String string_new() {
    return (String){.cap = 16, .len = 0, .data = calloc(17, sizeof(wchar_t))};
}

static inline bool string_eq(String *self, String *other)  {
    if (self->len != other->len) {
        return false;
    }
    return memcmp(self->data, other->data, self->len * sizeof(self->data[0])) == 0;
}

typedef struct {
    String word;
    bool end_word_indentation_allowed;
} Heredoc;

typedef struct {
    uint32_t len;
    uint32_t cap;
    Heredoc *data;
} HeredocVec;

static HeredocVec vec_new() {
    HeredocVec vec = {0, 0, NULL};
    vec.data = calloc(1, sizeof(Heredoc));
    vec.cap = 1;
    return vec;
}

typedef struct {
    bool has_leading_whitespace;
    HeredocVec open_heredocs;
} Scanner;

typedef enum { Error, End } ScanContentResult;

static inline void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static inline void skip(TSLexer *lexer) { lexer->advance(lexer, true); }

static unsigned serialize(Scanner *scanner, char *buffer) {
    unsigned size = 0;

    buffer[size++] = (char)scanner->open_heredocs.len;
    for (unsigned j = 0; j < scanner->open_heredocs.len; j++) {
        Heredoc *heredoc = &scanner->open_heredocs.data[j];
        unsigned word_bytes = heredoc->word.len * sizeof(heredoc->word.data[0]);
        if (size + 2 + word_bytes >= TREE_SITTER_SERIALIZATION_BUFFER_SIZE) {
            return 0;
        }
        buffer[size++] = (char)heredoc->end_word_indentation_allowed;
        buffer[size++] = (char)heredoc->word.len;
        memcpy(&buffer[size], heredoc->word.data, word_bytes);
        size += word_bytes;
    }

    return size;
}

static void deserialize(Scanner *scanner, const char *buffer, unsigned length) {
    unsigned size = 0;
    scanner->has_leading_whitespace = false;
    VEC_CLEAR(scanner->open_heredocs);

    if (length == 0) {
        return;
    }

    uint8_t open_heredoc_count = buffer[size++];
    for (unsigned j = 0; j < open_heredoc_count; j++) {
        Heredoc heredoc;
        heredoc.end_word_indentation_allowed = buffer[size++];
        heredoc.word = string_new();
        uint8_t word_length = buffer[size++];
        unsigned word_bytes = word_length * sizeof(heredoc.word.data[0]);
        STRING_GROW(heredoc.word, word_length);
        memcpy(heredoc.word.data, buffer + size, word_bytes);
        heredoc.word.len = word_length;
        size += word_bytes;
        VEC_PUSH(scanner->open_heredocs, heredoc);
    }
}

static inline bool scan_whitespace(TSLexer *lexer) {
    for (;;) {
        while (iswspace(lexer->lookahead)) {
            advance(lexer);
        }

        if (lexer->lookahead == '/') {
            advance(lexer);

            if (lexer->lookahead == '/') {
                advance(lexer);
                while (lexer->lookahead != 0 && lexer->lookahead != '\n') {
                    advance(lexer);
                }
            } else {
                return false;
            }
        } else {
            return true;
        }
    }
}

static inline bool is_valid_name_char(TSLexer *lexer) {
    return iswalnum(lexer->lookahead) || lexer->lookahead == '_' || lexer->lookahead >= 0x80;
}

static inline bool is_escapable_sequence(TSLexer *lexer) {
    // Note: remember to also update the escape_sequence rule in the
    // main grammar whenever changing this method
    int32_t letter = lexer->lookahead;

    if (letter == 'n' || letter == 'r' || letter == 't' || letter == 'v' || letter == 'e' || letter == 'f' ||
        letter == '\\' || letter == '$' || letter == '"') {
        return true;
    }

    // Hex
    if (letter == 'x') {
        advance(lexer);
        return iswxdigit(lexer->lookahead);
    }

    // Unicode
    if (letter == 'u') {
        return true; // We handle the case where this is not really an escape
                     // sequence in grammar.js - this is needed to support the
                     // edge case "\u{$a}" in which case "\u" is to be
                     // interpreted as characters and {$a} as a variable
    }

    // Octal
    return iswdigit(lexer->lookahead) && lexer->lookahead >= '0' && lexer->lookahead <= '7';
}

static inline bool scan_nowdoc_string(Scanner *scanner, TSLexer *lexer) {
    bool has_consumed_content = false;
    if (scanner->open_heredocs.len == 0) {
        return false;
    }

    // While PHP requires the nowdoc end tag to be the very first on a new line,
    // there may be an arbitrary amount of whitespace before the closing token
    while (iswspace(lexer->lookahead)) {
        advance(lexer);
        has_consumed_content = true;
    }

    String heredoc_tag = VEC_BACK(scanner->open_heredocs).word;

    bool end_tag_matched = false;

    for (uint32_t i = 0; i < heredoc_tag.len; i++) {
        if (lexer->lookahead != heredoc_tag.data[i]) {
            break;
        }
        advance(lexer);
        has_consumed_content = true;

        end_tag_matched = (i == heredoc_tag.len - 1 && (iswspace(lexer->lookahead) || lexer->lookahead == ';' ||
                                                        lexer->lookahead == ',' || lexer->lookahead == ')'));
    }

    if (end_tag_matched) {
        // There may be an arbitrary amount of white space after the end tag
        while (iswspace(lexer->lookahead) && lexer->lookahead != '\r' && lexer->lookahead != '\n') {
            advance(lexer);
            has_consumed_content = true;
        }

        // Return to allow the end tag parsing if we've encountered an end tag
        // at a valid position
        if (lexer->lookahead == ';' || lexer->lookahead == ',' || lexer->lookahead == ')' || lexer->lookahead == '\n' ||
            lexer->lookahead == '\r') {
            // , and ) is needed to support heredoc in function arguments
            return false;
        }
    }

    for (bool has_content = has_consumed_content;; has_content = true) {
        lexer->mark_end(lexer);

        switch (lexer->lookahead) {
            case '\n':
            case '\r':
                return has_content;
            default:
                if (lexer->eof(lexer)) {
                    return false;
                }
                advance(lexer);
        }
    }

    return false;
}

static bool scan_encapsed_part_string(Scanner *scanner, TSLexer *lexer, bool is_after_variable, bool is_heredoc,
                                      bool is_execution_string) {
    bool has_consumed_content = false;

    if (is_heredoc && scanner->open_heredocs.len > 0) {
        // While PHP requires the heredoc end tag to be the very first on a new
        // line, there may be an arbitrary amount of whitespace before the
        // closing token However, we should not consume \r or \n
        while (iswspace(lexer->lookahead) && lexer->lookahead != '\r' && lexer->lookahead != '\n') {
            advance(lexer);
            has_consumed_content = true;
        }

        String heredoc_tag = VEC_BACK(scanner->open_heredocs).word;

        bool end_tag_matched = false;

        for (uint32_t i = 0; i < heredoc_tag.len; i++) {
            if (lexer->lookahead != heredoc_tag.data[i]) {
                break;
            }
            has_consumed_content = true;
            advance(lexer);

            end_tag_matched = (i == heredoc_tag.len - 1 && (iswspace(lexer->lookahead) || lexer->lookahead == ';' ||
                                                            lexer->lookahead == ',' || lexer->lookahead == ')'));
        }

        if (end_tag_matched) {
            // There may be an arbitrary amount of white space after the end tag
            // However, we should not consume \r or \n
            while (iswspace(lexer->lookahead) && lexer->lookahead != '\r' && lexer->lookahead != '\n') {
                advance(lexer);
                has_consumed_content = true;
            }

            // Return to allow the end tag parsing if we've encountered an end
            // tag at a valid position
            if (lexer->lookahead == ';' || lexer->lookahead == ',' || lexer->lookahead == ')' ||
                lexer->lookahead == '\n' || lexer->lookahead == '\r') {
                // , and ) is needed to support heredoc in function arguments
                return false;
            }
        }
    }

    for (bool has_content = has_consumed_content;; has_content = true) {
        lexer->mark_end(lexer);

        switch (lexer->lookahead) {
            case '"':
                if (!is_heredoc && !is_execution_string) {
                    return has_content;
                }
                advance(lexer);
                break;
            case '`':
                if (is_execution_string) {
                    return has_content;
                }
                advance(lexer);
                break;
            case '\n':
            case '\r':
                if (is_heredoc) {
                    return has_content;
                }
                advance(lexer);
                break;
            case '\\':
                advance(lexer);

                // \{ should not be interpreted as an escape sequence, but both
                // should be consumed as normal characters
                if (lexer->lookahead == '{') {
                    advance(lexer);
                    break;
                }

                if (is_execution_string && lexer->lookahead == '`') {
                    return has_content;
                }

                if (is_heredoc && lexer->lookahead == '\\') {
                    advance(lexer);
                    break;
                }

                if (is_escapable_sequence(lexer)) {
                    return has_content;
                }
                break;
            case '$':
                advance(lexer);

                if (is_valid_name_char(lexer) || lexer->lookahead == '{') {
                    return has_content;
                }
                break;
            case '-':
                if (is_after_variable) {
                    advance(lexer);
                    if (lexer->lookahead == '>') {
                        advance(lexer);
                        if (is_valid_name_char(lexer)) {
                            return has_content;
                        }
                        break;
                    }
                    break;
                }
            case '[':
                if (is_after_variable) {
                    return has_content;
                }
                advance(lexer);
                break;
            case '{':
                advance(lexer);
                if (lexer->lookahead == '$') {
                    return has_content;
                }
                break;
            default:
                if (lexer->eof(lexer)) {
                    return false;
                }
                advance(lexer);
        }

        is_after_variable = false;
    }

    return false;
}

static String scan_heredoc_word(TSLexer *lexer) {
    String result = string_new();

    while (is_valid_name_char(lexer)) {
        STRING_PUSH(result, lexer->lookahead);
        advance(lexer);
    }

    return result;
}

static bool scan(Scanner *scanner, TSLexer *lexer, const bool *valid_symbols) {
    const bool is_error_recovery = valid_symbols[SENTINEL_ERROR];

    if (is_error_recovery) {
        // Consider if we should clear the heredoc list on error
        return false;
    }

    scanner->has_leading_whitespace = false;

    lexer->mark_end(lexer);

    if (valid_symbols[ENCAPSED_STRING_CHARS_AFTER_VARIABLE]) {
        lexer->result_symbol = ENCAPSED_STRING_CHARS_AFTER_VARIABLE;
        return scan_encapsed_part_string(scanner, lexer,
                                         /* is_after_variable */ true,
                                         /* is_heredoc */ false,
                                         /* is_execution_string */ false);
    }

    if (valid_symbols[ENCAPSED_STRING_CHARS]) {
        lexer->result_symbol = ENCAPSED_STRING_CHARS;
        return scan_encapsed_part_string(scanner, lexer,
                                         /* is_after_variable */ false,
                                         /* is_heredoc */ false,
                                         /* is_execution_string */ false);
    }

    if (valid_symbols[EXECUTION_STRING_CHARS_AFTER_VARIABLE]) {
        lexer->result_symbol = EXECUTION_STRING_CHARS_AFTER_VARIABLE;
        return scan_encapsed_part_string(scanner, lexer,
                                         /* is_after_variable */ true,
                                         /* is_heredoc */ false,
                                         /* is_execution_string */ true);
    }

    if (valid_symbols[EXECUTION_STRING_CHARS]) {
        lexer->result_symbol = EXECUTION_STRING_CHARS;
        return scan_encapsed_part_string(scanner, lexer,
                                         /* is_after_variable */ false,
                                         /* is_heredoc */ false,
                                         /* is_execution_string */ true);
    }

    if (valid_symbols[ENCAPSED_STRING_CHARS_AFTER_VARIABLE_HEREDOC]) {
        lexer->result_symbol = ENCAPSED_STRING_CHARS_AFTER_VARIABLE_HEREDOC;
        return scan_encapsed_part_string(scanner, lexer,
                                         /* is_after_variable */ true,
                                         /* is_heredoc */ true,
                                         /* is_execution_string */ false);
    }

    if (valid_symbols[ENCAPSED_STRING_CHARS_HEREDOC]) {
        lexer->result_symbol = ENCAPSED_STRING_CHARS_HEREDOC;
        return scan_encapsed_part_string(scanner, lexer,
                                         /* is_after_variable */ false,
                                         /* is_heredoc */ true,
                                         /* is_execution_string */ false);
    }

    if (valid_symbols[NOWDOC_STRING]) {
        lexer->result_symbol = NOWDOC_STRING;
        return scan_nowdoc_string(scanner, lexer);
    }

    if (valid_symbols[HEREDOC_END]) {
        lexer->result_symbol = HEREDOC_END;
        if (scanner->open_heredocs.len == 0) {
            return false;
        }

        Heredoc heredoc = VEC_BACK(scanner->open_heredocs);

        while (iswspace(lexer->lookahead)) {
            skip(lexer);
        }

        String word = scan_heredoc_word(lexer);
        if (!string_eq(&word, &heredoc.word)) {
            STRING_FREE(word);
            return false;
        }
        STRING_FREE(word);

        lexer->mark_end(lexer);
        VEC_POP(scanner->open_heredocs);
        return true;
    }

    if (!scan_whitespace(lexer)) {
        return false;
    }

    if (valid_symbols[EOF_TOKEN] && lexer->eof(lexer)) {
        lexer->result_symbol = EOF_TOKEN;
        return true;
    }

    if (valid_symbols[HEREDOC_START]) {
        lexer->result_symbol = HEREDOC_START;
        Heredoc heredoc;

        while (iswspace(lexer->lookahead)) {
            skip(lexer);
        }

        heredoc.word = scan_heredoc_word(lexer);
        if (heredoc.word.len == 0) {
            STRING_FREE(heredoc.word);
            return false;
        }
        lexer->mark_end(lexer);

        VEC_PUSH(scanner->open_heredocs, heredoc);
        return true;
    }

    if (valid_symbols[AUTOMATIC_SEMICOLON]) {
        lexer->result_symbol = AUTOMATIC_SEMICOLON;

        if (lexer->lookahead != '?') {
            return false;
        }

        advance(lexer);

        return lexer->lookahead == '>';
    }

    return false;
}

static inline void *external_scanner_create() {
    Scanner *scanner = calloc(1, sizeof(Scanner));
    scanner->open_heredocs = vec_new();
    return scanner;
}

static inline unsigned external_scanner_serialize(void *payload, char *buffer) {
    Scanner *scanner = (Scanner *)payload;
    return serialize(scanner, buffer);
}

static inline void external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
    Scanner *scanner = (Scanner *)payload;
    deserialize(scanner, buffer, length);
}

static inline bool external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
    Scanner *scanner = (Scanner *)payload;
    return scan(scanner, lexer, valid_symbols);
}

static inline void external_scanner_destroy(void *payload) {
    Scanner *scanner = (Scanner *)payload;
    for (size_t i = 0; i < scanner->open_heredocs.len; i++) {
        STRING_FREE(scanner->open_heredocs.data[i].word);
    }
    VEC_FREE(scanner->open_heredocs)
    free(scanner);
}

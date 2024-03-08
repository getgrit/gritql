cp ./node_modules/tree-sitter-html/src/tag.h ./src/tree_sitter_html
sed -e "s/ START_TAG_NAME,/ TEXT_FRAGMENT, INTERPOLATION_TEXT, START_TAG_NAME, TEMPLATE_START_TAG_NAME,/" \
    -e "s/case SCRIPT:/case TEMPLATE: lexer->result_symbol = TEMPLATE_START_TAG_NAME; break; case SCRIPT:/" \
    -e 's/"<\/script"/"<\/SCRIPT"/' \
    -e 's/"<\/style"/"<\/STYLE"/' \
    -e "s/lexer->lookahead == end_deli/towupper(lexer->lookahead) == end_deli/" \
    ./node_modules/tree-sitter-html/src/scanner.cc > ./src/tree_sitter_html/scanner.cc

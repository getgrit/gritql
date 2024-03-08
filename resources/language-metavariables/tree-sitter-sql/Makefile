CC ?= gcc

TS := $(shell which tree-sitter 2>/dev/null)
ifeq (, ${TS})
	TS := $(shell which tree-sitter-cli 2>/dev/null)
endif
TSFLAGS ?=

.PHONY: all
all: compile

.PHONY: clean
clean:
	rm src/grammar.json src/parser.c

.PHONY: generate
generate: src/grammar.json src/parser.c
src/grammar.json src/parser.c: grammar.js queries/highlights.scm queries/indents.scm
	${TS} generate ${TSFLAGS}

.PHONY: regenerate
regenerate: clean generate

.PHONY: test
test: src/grammar.json
	${TS} test

.PHONY: format
format: src/grammar.json
	${TS} test --update

.PHONY: compile
compile: target/parser.so
target/parser.so: src/parser.c src/scanner.c
	${CC} -shared -o target/parser.so -fPIC src/parser.c src/scanner.c -I./src

.PHONY: check_keywords
check_keywords: src/grammar.json queries/highlights.scm scripts/test-keywords.sh
	@bash scripts/test-keywords.sh

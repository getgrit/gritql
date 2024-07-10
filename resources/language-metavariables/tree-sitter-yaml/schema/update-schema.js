#!/usr/bin/env node

// @ts-nocheck

const { writeFileSync } = require("fs");
const { readFile } = require("fs/promises");
const { join } = require("path");

const schema = process.argv[2] ?? "core";

readFile(join(__dirname, schema, "src", "parser.c"), "utf8").then(input => {
  const cases = extractCases(input);
  const enums = ["RS_STR"];
  const content = "switch (sch_stt) " + block([
    "case SCH_STT_FRZ:\n  break;",
    cases
      .map(([key, { content }]) => `${(key === "default" ? "default:" : `case ${key}:`)}\n${indent(content)}`)
      .join("\n  END_STATE();\n")
      .replace(/\s+ADVANCE_MAP\(([^]+?)\);\n/, (_, map) => {
        return map.replace(/'(.)', (\d+),/g, "if (lookahead == '$1') ADVANCE($2);");
      })
      .replace(/ADVANCE\((\d+)\);/g, (_, state) => {
        const stateCase = cases.find(([key]) => key === state);
        if (stateCase) {
          const [, { acceptToken }] = stateCase;
          if (acceptToken) {
            return `{${acceptToken} return ${state};}`;
          }
        }
        return `{*rlt_sch = RS_STR; return ${state};}`;
      })
      .replace("ACCEPT_TOKEN(ts_builtin_sym_end);", "abort();")
      .replace(/ACCEPT_TOKEN\((\w+)\);/g, (_, name) => {
        const newName = "RS_" + convertName(name);
        if (!enums.includes(newName)) {
          enums.push(newName);
        }
        return `*rlt_sch = ${newName};`;
      })
      .replace(/END_STATE\(\);/g, `break;`)
      .replace("return false;", '*rlt_sch = RS_STR;\n  return SCH_STT_FRZ;')
      .replace(/lookahead/g, "cur_chr"),
  ]);
  writeFileSync(
    join(__dirname, "..", "src", `schema.${schema}.c`),
    [
      "#include <stdlib.h>",
      "#define SCH_STT_FRZ -1",
      `typedef enum ${block(enums.map((k) => `${k},`))} ResultSchema;`,
      `static int8_t adv_sch_stt(int8_t sch_stt, int32_t cur_chr, ResultSchema *rlt_sch) ${block([
        content,
        `if (cur_chr != '\\r' && cur_chr != '\\n' && cur_chr != ' ' && cur_chr != 0) *rlt_sch = RS_STR;`,
        "return SCH_STT_FRZ;",
      ])}`,
    ].join("\n\n") + "\n",
  );
});

function extractCases(input) {
  const MAIN_SIGNATURE = "static bool ts_lex(TSLexer *lexer, TSStateId state) {";
  const SWITCH_CASE = "switch (state) {\n";
  const startIndex = input.indexOf(SWITCH_CASE, input.indexOf(MAIN_SIGNATURE)) + SWITCH_CASE.length;
  const endIndex = input.indexOf("}\n}", startIndex);
  const content = input.slice(startIndex, endIndex).replace(/^\s*if \(eof\).+\n/mg, "").trimEnd();
  return dedent(dedent(content)).split("END_STATE();").map(text => {
    const index = text.indexOf(":\n");
    const key = text.slice(0, index).trim().replace(/^case /, "");
    const content = dedent(text.slice(index + 2)).trim();
    const matchAcceptToken = content.match(/^ACCEPT_TOKEN\(\w+\);/);
    const acceptToken = matchAcceptToken && matchAcceptToken[0];
    const hasAcceptTokenOnly = acceptToken && acceptToken.length === content.length;
    return [key, { content, acceptToken, hasAcceptTokenOnly }];
  });
}

function convertName(name) {
  return name.replace("sym_", "").toUpperCase();
}

function block(contents) {
  return `{\n${indent(contents)}\n}`;
}

function lines(contents) {
  return [].concat(contents).join("\n").split("\n");
}

function indent(contents) {
  return lines(contents).map(x => " ".repeat(2) + x).join("\n");
}

function dedent(contents) {
  return lines(contents).map(x => x.replace(/^  /mg, "")).join("\n");
}

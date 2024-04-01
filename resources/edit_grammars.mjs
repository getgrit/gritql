import { exec } from 'child_process';
import fs from 'fs/promises';

///////////////////////////////////////////////////
// Scripting helpers
///////////////////////////////////////////////////

let childProcesses = new Set();
//a tweaked promisify(exec) to keep track of running children
const execPromise = (command) => {
  return new Promise((resolve, reject) => {
    console.log(command);
    const child = exec(command, (error, stdout, stderr) => {
      childProcesses.delete(child);
      if (error) {
        reject(error);
      } else {
        resolve({ stdout, stderr });
      }
    });
    childProcesses.add(child);
  });
};

function cleanExit() {
  console.log(`Killing ${childProcesses.size} running subprocesses`);
  for (let child of childProcesses) {
    child.kill();
  }
  process.exit();
}
process.on('SIGINT', cleanExit); // catch ctrl-c
process.on('SIGTERM', cleanExit); // catch kill
process.on('uncaughtException', (err) => {
  console.error('Uncaught Exception:', err);
  cleanExit();
});

///////////////////////////////////////////////////
// Constants
///////////////////////////////////////////////////
const allLanguages = [
  'c-sharp',
  'css',
  'go',
  'hcl',
  'html',
  'java',
  'javascript',
  'json',
  'markdown',
  'python',
  'ruby',
  'rust',
  'solidity',
  'sql',
  'typescript',
  'yaml',
  'toml',
  'vue',
  'php'
];

// For these languages, copyMvGrammar is optional
// hcl has one but is in a different location than the default
// I'd rather not special-case languages in individual steps
// like this and instead have them define their own build
// steps, but makes the code smaller for now
const languagesWithoutMetaVariables = ['ruby', 'html', 'hcl'];

// assumes that this script is run from marzano/resources directory
const METAVARIABLE_GRAMMARS = `../metavariable-grammars`;
const LANGUAGE_METAVARIABLES = 'language-metavariables';

///////////////////////////////////////////////////
// Build steps
///////////////////////////////////////////////////

//dest is optional and will default to lang. Useful for moving to a slightly different location.
//string interpolation is slightly abused so one can do lang = `markdown/markdown-inline` to pass
//a subfolder.
const copyMvGrammar = async (lang, dest) => {
  if (languagesWithoutMetaVariables.includes(lang)) {
    return;
  }
  await fs.copyFile(
    `${METAVARIABLE_GRAMMARS}/${lang}-metavariable-grammar.js`,
    `tree-sitter-${dest ?? lang}/grammar.js`,
  );
};

const copyMvScanner = async (lang, dest) =>
  fs.copyFile(
    `${METAVARIABLE_GRAMMARS}/${lang}-metavariable-scanner.cc`,
    `tree-sitter-${dest ?? lang}/src/scanner.cc`,
  );

const treeSitterGenerate = async (dir, buildWasm = true) => {
  const andMaybeBuildWasm = buildWasm ? '&& npx tree-sitter build-wasm ' : '';
  await execPromise(
    `cd tree-sitter-${dir} && npx tree-sitter generate ${andMaybeBuildWasm} && echo "Generated grammar for ${dir}"`,
  ).catch((e) => console.log('swallowed error, ', e));
};

const copyNodeTypes = async (lang, dest) =>
  fs.copyFile(
    `tree-sitter-${lang}/src/node-types.json`,
    `../node-types/${dest ?? lang}-node-types.json`,
  );

const copyWasmParser = async (lang, prefix) =>
  fs.rename(
    `${prefix ?? 'tree-sitter-'}${lang}/tree-sitter-${lang}.wasm`,
    `../../crates/wasm-bindings/wasm_parsers/tree-sitter-${lang}.wasm`,
  );

async function rsyncGrammars(language) {
  //If a language is given, only sync that language
  //Otherwise, rm -rf the entire language-metavariables dir and sync it from scratch
  const treeSitterLang = language ? `tree-sitter-${language}` : '.';
  const mvDir = language
    ? `${LANGUAGE_METAVARIABLES}/${treeSitterLang}`
    : LANGUAGE_METAVARIABLES;

  if (languagesWithoutMetaVariables.includes(language)) {
    return;
  }

  await fs.rmdir(mvDir, { recursive: true });
  await fs.mkdir(mvDir, { recursive: true });

  const submodulesDir = language
    ? `language-submodules/${treeSitterLang}`
    : 'language-submodules/.';
  const blobsToExclude = [
    '.git*',
    'tree-sitter-*/example',
    'tree-sitter-*/test',
    'tree-sitter-*/corpus',
  ];
  await execPromise(
    `rsync -r -l ${submodulesDir} language-metavariables --exclude={${blobsToExclude.join(
      ','
    )}}`
  );
}

///////////////////////////////////////////////////
// Language build scripts
///////////////////////////////////////////////////

async function buildSimpleLanguage(log, language) {
  log(`Copying files`);
  await copyMvGrammar(language);
  log(`Running tree-sitter generate`);
  await treeSitterGenerate(language);
  log(`Copying output node types`);
  await copyNodeTypes(language);
  log(`Copying wasm parser`);
  await copyWasmParser(language);
}

async function buildLanguage(language) {
  if (!allLanguages.includes(language)) {
    throw `Unsupported language ${language}`;
  }

  const log = (message, ...args) =>
    console.log(`[${language}] ` + message, ...args);
  log(`Starting`);
  const tsLangDir = `tree-sitter-${language}`;

  if (language == 'toml') {
    await execPromise(`cd ${tsLangDir} && npm install regexp-util && npx tree-sitter generate && cd ..`)
  }
  //Force cargo.toml to use the correct version of tree-sitter
  await execPromise(`for cargo in ${tsLangDir}/[Cc]argo.toml; do
    if [ -f "$cargo" ]; then
      if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' -e 's/tree-sitter = ".*"/tree-sitter = "~0.20"/g' "$cargo";
      else
        sed -i -e 's/tree-sitter = ".*"/tree-sitter = "~0.20"/g' "$cargo";
      fi
    fi;
  done`);

  if (language === 'c-sharp') {
    //skip generating c-sharp, tree-sitter hangs on it
    await copyNodeTypes('c-sharp', 'csharp');
  } else if (language === 'markdown') {
    //markdown has sub-grammars
    await Promise.all([
      copyMvGrammar('markdown-common', 'markdown/common'),
      copyMvGrammar('markdown-block', 'markdown/tree-sitter-markdown'),
      copyMvGrammar('markdown-inline', 'markdown/tree-sitter-markdown-inline'),
    ]);
    await Promise.all([
      treeSitterGenerate('markdown/tree-sitter-markdown'),
      treeSitterGenerate('markdown/tree-sitter-markdown-inline'),
    ]);
    await Promise.all([
      copyNodeTypes('markdown/tree-sitter-markdown', 'markdown-block'),
      copyNodeTypes('markdown/tree-sitter-markdown-inline', `markdown-inline`),
    ]);

    await fs.rename(
      `tree-sitter-markdown/tree-sitter-markdown/tree-sitter-markdown.wasm`,
      `../../crates/wasm-bindings/wasm_parsers/tree-sitter-markdown-block.wasm`,
    );

    await fs.rename(
      `tree-sitter-markdown/tree-sitter-markdown-inline/tree-sitter-markdown_inline.wasm`,
      `../../crates/wasm-bindings/wasm_parsers/tree-sitter-markdown_inline.wasm`,
    );
  } else if (language === 'typescript') {
    // typescript is special
    // we edit its package.json to point to our local version of the js grammar
    log(`Copying  files`);
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS}/typescript-package.json`,
      `${tsLangDir}/package.json`,
    );

    // typescript defines a typescript and tsx grammar, the grammar we care about is in common/define-grammar.js
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS}/typescript-metavariable-define-grammar.js`,
      `${tsLangDir}/common/define-grammar.js`,
    );

    await execPromise(
      `cd ${tsLangDir} && yarn && yarn build && echo "Generated grammar for ${language}"`,
    );
    // await Promise.all([
    //   execPromise(`cd ${tsLangDir}/tsx && npx tree-sitter build-wasm`),
    //   execPromise(`cd ${tsLangDir}/typescript && npx tree-sitter build-wasm`),
    // ]);

    await copyNodeTypes('typescript/typescript', 'typescript');
    await copyNodeTypes('typescript/tsx', 'tsx');

    // await copyWasmParser('typescript', 'tree-sitter-typescript/');
    // await copyWasmParser('tsx', 'tree-sitter-typescript/');
  } else if (language === 'vue') {
    log(`Copying  files`);
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS}/vue-package.json`,
      `${tsLangDir}/package.json`,
    );
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS}/vue-metavariable-grammar.js`,
      `${tsLangDir}/grammar.js`,
    );

    await execPromise(
      `cd ${tsLangDir} && yarn && yarn prepack && npx tree-sitter build-wasm && echo "Generated grammar for ${language}"`,
    );

    await copyNodeTypes(language);
    await copyWasmParser(language);
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS}/cc_build.rs`,
      `${tsLangDir}/bindings/rust/build.rs`,
    );
  } else if (language === 'yaml') {
    await copyMvScanner(language);
    await buildSimpleLanguage(log, language);
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS}/cc_build.rs`,
      `${tsLangDir}/bindings/rust/build.rs`,
    );
  } else if (language === 'hcl') {
    //HCL's mv grammar goes into `make_grammar.js`, not `grammar.js`
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS}/${language}-metavariable-grammar.js`,
      `tree-sitter-${language}/make_grammar.js`,
    );
    await buildSimpleLanguage(log, language);
  } else if (language === 'sql') {
    await copyMvGrammar(language);
    //SQL's wasm build hangs so we skip it
    await treeSitterGenerate(language, false);
    await copyNodeTypes(language);
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS}/c_build.rs`,
      `${tsLangDir}/bindings/rust/build.rs`,
    );
  } else if (language === 'toml') {
    await buildSimpleLanguage(log, language);
  } else if (language === 'php') {
    //php has sub-grammars
    log(`Copying  files`);  
    await Promise.all([
      await fs.copyFile(
        `${METAVARIABLE_GRAMMARS}/${language}-common-metavariable-grammar.js`,
        `tree-sitter-${language}/common/define-grammar.js`,
      ),
      // copyMvGrammar('php_only', 'php/php_only'),
      // copyMvGrammar('php', 'php/php'),
    ]);

    log(`Running tree-sitter generate`);
    await Promise.all([
      treeSitterGenerate('php/php_only'),
      treeSitterGenerate('php/php'),
    ]);

    log(`Copying output node types`);
    await Promise.all([
      copyNodeTypes('php/php_only', 'php_only'),
      copyNodeTypes('php/php', `php`),
    ]);

    log(`Copying wasm parser`);
    await fs.rename(
      `tree-sitter-php/php_only/tree-sitter-php_only.wasm`,
      `../../crates/wasm-bindings/wasm_parsers/tree-sitter-php_only.wasm`,
    );
    await fs.rename(
      `tree-sitter-php/php/tree-sitter-php.wasm`,
      `../../crates/wasm-bindings/wasm_parsers/tree-sitter-php.wasm`,
    );
  }else {
    await buildSimpleLanguage(log, language);
  }

  log(`Done`);
}

async function run() {
  const args = process.argv.slice(2);
  const buildAll = args.length == 0;
  const languagesTobuild = buildAll ? allLanguages : args;

  console.log('Syncing upstream grammars');
  if (buildAll) {
    await rsyncGrammars();
  } else {
    await Promise.all(languagesTobuild.map(rsyncGrammars));
  }
  process.chdir(LANGUAGE_METAVARIABLES);
  await Promise.all(languagesTobuild.map(buildLanguage));
  await execPromise(
    `find . -name "build.rs" -exec sed -i '' -e 's/Wno-unused-parameter/w/g' {} \\;`,
  );
}

run().catch(console.error);

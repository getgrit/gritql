import { exec } from "child_process";
import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";

///////////////////////////////////////////////////
// Scripting helpers
///////////////////////////////////////////////////

let childProcesses = new Set();
//a tweaked promisify(exec) to keep track of running children
const execPromise = (command, cwd) => {
  return new Promise((resolve, reject) => {
    console.log(command);
    const child = exec(command, { cwd }, (error, stdout, stderr) => {
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
process.on("SIGINT", cleanExit); // catch ctrl-c
process.on("SIGTERM", cleanExit); // catch kill
process.on("uncaughtException", (err) => {
  console.error("Uncaught Exception:", err);
  cleanExit();
});

///////////////////////////////////////////////////
// Constants
///////////////////////////////////////////////////
const allLanguages = [
  "c-sharp",
  "css",
  "go",
  "hcl",
  "html",
  "java",
  "javascript",
  "json",
  "markdown",
  "python",
  "ruby",
  "rust",
  "solidity",
  "sql",
  "typescript",
  "yaml",
  "toml",
  "vue",
  "php",
];

// For these languages, copyMvGrammar is optional
// hcl has one but is in a different location than the default
// I'd rather not special-case languages in individual steps
// like this and instead have them define their own build
// steps, but makes the code smaller for now
const languagesWithoutMetaVariables = ["html", "hcl"];

// Set up paths
const resourceDir = path.dirname(fileURLToPath(import.meta.url));
const METAVARIABLE_GRAMMARS_DIR = path.join(
  resourceDir,
  "metavariable-grammars"
);
const LANGUAGE_METAVARIABLES_DIR = path.join(
  resourceDir,
  "language-metavariables"
);

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
  let from = `${METAVARIABLE_GRAMMARS_DIR}/${lang}-metavariable-grammar.js`;
  let to = path.join(
    LANGUAGE_METAVARIABLES_DIR,
    `tree-sitter-${dest ?? lang}/grammar.js`
  );
  await fs.copyFile(from, to);
  console.log(`Copied ${from} to ${to}`);
};

/**
 * Copy in the build.rs file for the language,
 * if an override is needed
 * @param {*} c "c" or "cc
 * @param {*} lang language we are copying from
 * @param {*} dest optional destination
 * @returns
 */
const copyMyBuild = async (c, lang, dest) =>
  fs.copyFile(
    `${METAVARIABLE_GRAMMARS_DIR}/${c}_build.rs`,
    path.join(
      LANGUAGE_METAVARIABLES_DIR,
      `tree-sitter-${dest ?? lang}/bindings/rust/build.rs`
    )
  );

const treeSitterGenerate = async (dir, buildWasm = true) => {
  const andMaybeBuildWasm = buildWasm ? "&& tree-sitter build-wasm " : "";
  await execPromise(
    `tree-sitter generate ${andMaybeBuildWasm} && echo "Generated grammar for ${dir}"`,
    path.join(LANGUAGE_METAVARIABLES_DIR, `tree-sitter-${dir}`)
  ).catch((e) => console.log("swallowed error, ", e));
};

const copyNodeTypes = async (lang, dest) =>
  fs.copyFile(
    path.join(
      LANGUAGE_METAVARIABLES_DIR,
      `tree-sitter-${lang}/src/node-types.json`
    ),
    path.join(resourceDir, `node-types/${dest ?? lang}-node-types.json`)
  );

const copyWasmParser = async (lang, prefix) =>
  fs.rename(
    path.join(
      LANGUAGE_METAVARIABLES_DIR,
      `${prefix ?? "tree-sitter-"}${lang}/tree-sitter-${lang}.wasm`
    ),
    path.join(
      resourceDir,
      `../crates/wasm-bindings/wasm_parsers/tree-sitter-${lang}.wasm`
    )
  );

async function rsyncGrammars(language) {
  //If a language is given, only sync that language
  //Otherwise, rm -rf the entire language-metavariables dir and sync it from scratch
  const treeSitterLang = language ? `tree-sitter-${language}` : ".";
  const mvDir = language
    ? `${LANGUAGE_METAVARIABLES_DIR}/${treeSitterLang}`
    : LANGUAGE_METAVARIABLES_DIR;

  if (languagesWithoutMetaVariables.includes(language)) {
    return;
  }

  await fs.rmdir(mvDir, { recursive: true });
  await fs.mkdir(mvDir, { recursive: true });

  const submodulesDir = path.join(
    resourceDir,
    language
      ? `language-submodules/${treeSitterLang}/.`
      : "language-submodules/."
  );
  const blobsToExclude = [".git*", "**/*/example", "**/*/test", "**/*/corpus"];

  console.log(`Copying ${submodulesDir} to ${mvDir}`);

  await execPromise(
    `rsync -r -l "${submodulesDir}/." "${mvDir}/." --exclude={${blobsToExclude.join(
      ","
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
  const tsLangDir = path.join(
    LANGUAGE_METAVARIABLES_DIR,
    `tree-sitter-${language}`
  );

  if (language == "toml") {
    await execPromise(
      `npm install regexp-util && tree-sitter generate`,
      tsLangDir
    );
  }
  //Force cargo.toml to use the correct version of tree-sitter
  const cargoFiles = ["Cargo.toml", "cargo.toml"];
  let didUpdate = false;
  for (const cargo of cargoFiles) {
    const cargoPath = path.join(tsLangDir, cargo);
    log(`Trying to update ${cargoPath}`);
    try {
      await fs.access(cargoPath);
      let cargoContent = await fs.readFile(cargoPath, "utf8");
      cargoContent = cargoContent.replace(
        /tree-sitter = ".*"/g,
        'tree-sitter = "~0.20"'
      );
      await fs.writeFile(cargoPath, cargoContent);
      log(`Updated ${cargoPath}`);
      didUpdate = true;
      break;
    } catch (e) {
      log(`Could not update ${cargoPath}`);
    }
  }
  if (!didUpdate) {
    throw new Error("Could not find Cargo.toml to update");
  }

  if (language === "c-sharp") {
    //skip generating c-sharp, tree-sitter hangs on it
    await copyNodeTypes("c-sharp", "csharp");
  } else if (language === "markdown") {
    //markdown has sub-grammars
    await Promise.all([
      copyMvGrammar("markdown-common", "markdown/common"),
      copyMvGrammar("markdown-block", "markdown/tree-sitter-markdown"),
      copyMvGrammar("markdown-inline", "markdown/tree-sitter-markdown-inline"),
    ]);
    await Promise.all([
      treeSitterGenerate("markdown/tree-sitter-markdown"),
      treeSitterGenerate("markdown/tree-sitter-markdown-inline"),
    ]);
    await Promise.all([
      copyNodeTypes("markdown/tree-sitter-markdown", "markdown-block"),
      copyNodeTypes("markdown/tree-sitter-markdown-inline", `markdown-inline`),
    ]);

    await fs.rename(
      path.join(
        LANGUAGE_METAVARIABLES_DIR,
        `tree-sitter-markdown/tree-sitter-markdown/tree-sitter-markdown.wasm`
      ),
      path.join(
        resourceDir,
        `../crates/wasm-bindings/wasm_parsers/tree-sitter-markdown-block.wasm`
      )
    );

    await fs.rename(
      path.join(
        LANGUAGE_METAVARIABLES_DIR,
        `tree-sitter-markdown/tree-sitter-markdown-inline/tree-sitter-markdown_inline.wasm`
      ),
      path.join(
        resourceDir,
        `../crates/wasm-bindings/wasm_parsers/tree-sitter-markdown_inline.wasm`
      )
    );
  } else if (language === "typescript") {
    // typescript is special
    // we edit its package.json to point to our local version of the js grammar
    log(`Copying  files`);
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS_DIR}/typescript-package.json`,
      `${tsLangDir}/package.json`
    );

    // typescript defines a typescript and tsx grammar, the grammar we care about is in common/define-grammar.js
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS_DIR}/typescript-metavariable-define-grammar.js`,
      `${tsLangDir}/common/define-grammar.js`
    );

    await execPromise(
      `yarn && yarn build && echo "Generated grammar for ${language}"`,
      tsLangDir
    );
    // await Promise.all([
    //   execPromise(`cd ${tsLangDir}/tsx && tree-sitter build-wasm`),
    //   execPromise(`cd ${tsLangDir}/typescript && tree-sitter build-wasm`),
    // ]);

    await copyNodeTypes("typescript/typescript", "typescript");
    await copyNodeTypes("typescript/tsx", "tsx");

    // await copyWasmParser('typescript', 'tree-sitter-typescript/');
    // await copyWasmParser('tsx', 'tree-sitter-typescript/');
  } else if (language === "vue") {
    log(`Copying  files`);
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS_DIR}/vue-package.json`,
      `${tsLangDir}/package.json`
    );
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS_DIR}/vue-metavariable-grammar.js`,
      `${tsLangDir}/grammar.js`
    );

    await execPromise(
      `yarn && yarn prepack && tree-sitter build-wasm && echo "Generated grammar for ${language}"`,
      tsLangDir
    );

    await copyNodeTypes(language);
    await copyWasmParser(language);
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS_DIR}/cc_build.rs`,
      `${tsLangDir}/bindings/rust/build.rs`
    );
  } else if (language === "yaml") {
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS_DIR}/${language}-metavariable-scanner.c`,
      `${tsLangDir}/src/scanner.c`
    );
    await buildSimpleLanguage(log, language);
    await copyMyBuild("c", language);
  } else if (language === "hcl") {
    //HCL's mv grammar goes into `make_grammar.js`, not `grammar.js`
    await fs.copyFile(
      `${METAVARIABLE_GRAMMARS_DIR}/${language}-metavariable-grammar.js`,
      path.join(
        resourceDir,
        `language-metavariables`,
        `tree-sitter-${language}/make_grammar.js`
      )
    );
    await buildSimpleLanguage(log, language);
  } else if (language === "sql") {
    await copyMvGrammar(language);
    //SQL's wasm build hangs so we skip it
    await treeSitterGenerate(language, false);
    await copyNodeTypes(language);
    await copyMyBuild("c", language);
  } else if (language === "toml") {
    await buildSimpleLanguage(log, language);
    await copyMyBuild("c", language);
  } else if (language === "php") {
    //php has sub-grammars
    log(`Copying  files`);
    await Promise.all([
      await fs.copyFile(
        `${METAVARIABLE_GRAMMARS_DIR}/${language}-common-metavariable-grammar.js`,
        path.join(
          resourceDir,
          `language-metavariables/tree-sitter-${language}/common/grammar.js`
        )
      ),
      // copyMvGrammar('php_only', 'php/php_only'),
      // copyMvGrammar('php', 'php/php'),
    ]);

    log(`Running tree-sitter generate`);
    await Promise.all([
      treeSitterGenerate("php/php_only"),
      treeSitterGenerate("php/php"),
    ]);

    log(`Copying output node types`);
    await Promise.all([
      copyNodeTypes("php/php_only", "php_only"),
      copyNodeTypes("php/php", `php`),
    ]);

    log(`Copying wasm parser`);
    await fs.rename(
      path.join(
        LANGUAGE_METAVARIABLES_DIR,
        `tree-sitter-php/php_only/tree-sitter-php_only.wasm`
      ),
      path.join(
        resourceDir,
        `../crates/wasm-bindings/wasm_parsers/tree-sitter-php_only.wasm`
      )
    );
    await fs.rename(
      path.join(
        LANGUAGE_METAVARIABLES_DIR,
        `tree-sitter-php/php/tree-sitter-php.wasm`
      ),
      path.join(
        resourceDir,
        `../crates/wasm-bindings/wasm_parsers/tree-sitter-php.wasm`
      )
    );
  } else {
    await buildSimpleLanguage(log, language);
  }

  await execPromise(
    `find "${tsLangDir}" -name "build.rs" -exec sed -i '' -e 's/Wno-unused-parameter/w/g' {} \\;`
  );

  log(`Done`);
}

async function run() {
  const args = process.argv.slice(2);
  const buildAll = args.length == 0;
  const languagesTobuild = buildAll ? allLanguages : args;

  console.log("Syncing upstream grammars");
  if (buildAll) {
    await rsyncGrammars();
  } else {
    await Promise.all(languagesTobuild.map(rsyncGrammars));
  }
  await Promise.all(languagesTobuild.map(buildLanguage));
}

run().catch(console.error);

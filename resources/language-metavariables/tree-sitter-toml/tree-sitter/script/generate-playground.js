const assert = require("assert");
const child_process = require("child_process");
const fs = require("fs");
const path = require("path");

module.exports = (outputDirname, { name, example, codeExample, queryExample }) => {
  assert(!fs.existsSync(outputDirname));
  fs.mkdirSync(outputDirname);

  const cwd = process.cwd();
  const fromGrammar = x => path.resolve(cwd, x);
  const fromTreeSitter = x => path.resolve(__dirname, "..", x);
  const fromOutputDir = x => path.resolve(outputDirname, x);

  assert(fs.existsSync(fromGrammar("package.json")));
  const packageJson = require(fromGrammar("package.json"));

  assert(/^tree-sitter-[a-z]+$/.test(packageJson.name));
  const escape_example = x => x.replace(/`/g, "\\`").replace(/<\/script(\s*)>/g, '<\\/script$1>')
  const config = {
    tree_sitter_version: require(fromTreeSitter("cli/npm/package.json")).version,
    grammar_name: name,
    grammar_id: packageJson.name.split("-").pop(),
    grammar_version: packageJson.version,
    grammar_repository: packageJson.repository,
    grammar_code_example: escape_example(codeExample || example),
    grammar_query_example: escape_example(queryExample || ''),
  };
  const config_yml = `
tree_sitter:
  version: ${config.tree_sitter_version}
grammar:
  name: ${config.grammar_name}
  id: ${config.grammar_id}
  version: ${config.grammar_version}
  repository: ${config.grammar_repository}
  example:
    code: ${JSON.stringify(config.grammar_code_example)}
    query: ${JSON.stringify(config.grammar_query_example)}
`;
  const configFilename = "_config.generated.yml";
  fs.writeFileSync(fromTreeSitter(`docs/${configFilename}`), config_yml);

  const execSync = (command, options = {}) =>
    child_process.execSync(command, { ...options, stdio: [0, 1, 2] });

  execSync(
    [
      "docker run",
      "--rm",
      "-v $PWD:$PWD",
      "-w $PWD",
      "ruby",
      `bash -c "bundle install --deployment && bundle exec jekyll build --config _config.yml,${configFilename}"`
    ].join(" "),
    { cwd: fromTreeSitter("docs") }
  );
  execSync("./script/build-wasm", { cwd: fromTreeSitter(".") });
  execSync("cargo build --release", { cwd: fromTreeSitter(".") });
  execSync(`${fromTreeSitter("target/release/tree-sitter")} generate`, {
    cwd: fromGrammar(".")
  });
  execSync(`${fromTreeSitter("target/release/tree-sitter")} build-wasm`, {
    cwd: fromGrammar(".")
  });

  const assetsDir = fromOutputDir('assets');
  fs.mkdirSync(assetsDir)

  const playgroundAssetsDir = path.resolve(assetsDir, `tree-sitter-playground-${config.tree_sitter_version}`);
  fs.mkdirSync(playgroundAssetsDir);
  execSync(`cp ${fromTreeSitter('LICENSE')} ${
    fromTreeSitter('docs/_site/assets/css/style.css')} ${
    fromTreeSitter('docs/_site/assets/js/playground.js')} ${
    playgroundAssetsDir}`);

  const webAssetsDir = path.resolve(assetsDir, `web-tree-sitter-${config.tree_sitter_version}`);
  fs.mkdirSync(webAssetsDir);
  execSync(`cp ${fromTreeSitter('LICENSE')} ${
    fromTreeSitter("lib/binding_web/tree-sitter.js")} ${
    fromTreeSitter("lib/binding_web/tree-sitter.wasm")} ${
    webAssetsDir}`);

  const GrammarAssetsDir = path.resolve(assetsDir, `tree-sitter-${config.grammar_id}-${config.grammar_version}`);
  fs.mkdirSync(GrammarAssetsDir);
  execSync(`cp ${fromGrammar(`tree-sitter-${config.grammar_id}.wasm`)} ${
    GrammarAssetsDir}`);

  execSync(
    `cp ${fromTreeSitter("docs/_site/playground.html")} ${
      fromOutputDir("index.html")}`
  );
};

const { AsyncLocalStorage } = require('node:async_hooks');
const { QueryBuilder } = require('../__generated__/bridge.darwin-arm64.node');

const testStorage = new AsyncLocalStorage();
// testStorage.enterWith(9);

// testStorage.disable();

// testStorage.exit(() => {
const query = new QueryBuilder(`js"console.log($msg)"`);
query.filter((err, log) => {
  console.log('filter was called', err, log);
  log.insertAfter('\n<<inserted>>');
  return true;
});

query
  .applyToFile({
    path: 'test.js',
    content: `console.log("hello")`,
  })
  .then((result) => {
    console.log(result);
    process.exit(0);
  });
// });

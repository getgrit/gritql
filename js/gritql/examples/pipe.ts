import { QueryBuilder } from '../__generated__/index.js';

/// The first query is just filtering files
const query = new QueryBuilder(`file()`);

// NOTE: THIS CAUSES THE PIPE TO BREAK
// query.filter((err, arg) => {
//   console.log('file hit in layer 1', arg);
//   return true;
// });

/// The second query actually looks for foo_bar
const query2 = new QueryBuilder(`js"function $_($_) { $_ }"`);

query2.filter((node) => {
  console.log('file hit in layer 2', node);
  return true;
});

/// We pipe the first query into the second
query.pipe(query2);

/// Then run the first query and it will run the second query as well
const files = await query.run({ targetPaths: [process.cwd()], stepId: 'test' });
const total = files.length;
console.log(`Hello from search.ts!, we found ${total} files`);

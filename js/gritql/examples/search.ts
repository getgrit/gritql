import { QueryBuilder } from '../__generated__/index.js';

const query = new QueryBuilder(`js"console.log($_)" as $match where {
    $msg = $match,
    $something = length($msg),
    $something <: true
}`);

query.filter((node) => {
  console.log('got my first arg!', node.text());
  const length = arg.length;
  const isOdd = length % 2 === 1;
  return isOdd;
});

query.filter((node) => {
  console.log('got my second arg!', node.text());
  return arg.length > 19;
});

const files = await query.run({ targetPaths: [process.cwd()], stepId: 'test' });
const total = files.length;
console.log(`Hello from search.ts!, we found ${total} files`);

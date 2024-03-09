import { stdlib } from '@getgrit/api';

export async function execute() {
  await stdlib.apply({ query: '`sum($a, $b)` => `sum($b, $a)`' }, {});
  return { success: true, message: 'Migration complete' };
}

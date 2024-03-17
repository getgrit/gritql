import * as grit from '@getgrit/api';
import * as sdk from '@getgrit/workflows-sdk';

import { magicNumber } from './other_workflow';

export default await sdk.defineWorkflow({
  name: 'test_workflow',
  run: async () => {
    // // console.log('internal is', internal.writeLog);
    console.log('done');
    console.log('The magic number is ', magicNumber, import.meta.main);
    return {
      success: true,
    };
  },
});

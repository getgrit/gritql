import * as grit from '@getgrit/api';
import * as sdk from '@getgrit/workflows-sdk';

export const magicNumber = 9;

export default await sdk.defineWorkflow({
  name: 'other_workflow',
  run: async () => {
    // // console.log('internal is', internal.writeLog);
    console.log('The OTHER function was invoked', import.meta.main);
    return {
      success: true,
    };
  },
});

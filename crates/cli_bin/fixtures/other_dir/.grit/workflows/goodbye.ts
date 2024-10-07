import type { JSONSchema7 } from 'json-schema';

import * as sdk from '@getgrit/workflows-sdk';
import * as grit from '@getgrit/api';

export default sdk.defineWorkflow({
  run: async (options) => {
    return {
      success: false,
      message: 'Goodbye, world!',
    };
  },
});

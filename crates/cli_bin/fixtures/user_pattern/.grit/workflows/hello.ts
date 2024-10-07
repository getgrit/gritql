import * as sdk from '@getgrit/workflows-sdk';
import * as grit from '@getgrit/api';

export default sdk.defineWorkflow({
  run: async (options) => {
    grit.logging.info('Running hello workflow', options);

    return {
      success: true,
      outcome: 'success',
      message: `This workflow was processed successfully!`,
    };
  },
});

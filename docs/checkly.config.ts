import { defineConfig } from 'checkly';
import { Frequency } from 'checkly/constructs';

export default defineConfig({
  projectName: 'docs',
  logicalId: 'docs-e2e',
  repoUrl: 'https://github.com/getgrit/rewriter',
  checks: {
    activated: true,
    muted: false,
    runtimeId: '2022.10',
    locations: ['us-east-1', 'us-west-1'],
    tags: ['docs', 'e2e'],
    alertChannels: [],
    checkMatch: '**/__checks__/*.check.ts',
    ignoreDirectoriesMatch: [],
    browserChecks: {
      frequency: Frequency.EVERY_1H,
      testMatch: '**/__checks__/**/*.spec.ts',
    },
  },
  cli: {
    runLocation: 'us-east-1',
  },
});

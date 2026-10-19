import * as grit from '@getgrit/api';
import * as sdk from '@getgrit/workflows-sdk';

import { QueryBuilder } from '@getgrit/bridge';

/**
 * Rewrite a Jest setup file to be usable as a --preload file
 */
async function transformTests(targetPath: string) {
  grit.logging.debug(`Transforming Jest test files at ${targetPath}`);
  const pattern = `or {
        js"describe" where { add_import(js"describe", js"'bun:test'")},
        js"it" where { add_import(js"it", js"'bun:test'")},
        js"test($_)" where { add_import(js"test", js"'bun:test'")},
        js"expect($_)" where { add_import(js"expect", js"'bun:test'")},
}`;
const query = new QueryBuilder(pattern);

  const instanceCount = await query.run([targetPath]);
  grit.logging.info(`Transformed Jest setup file at ${targetPath}, with ${instanceCount} changes`);

  return targetPath;
}

/**
 * Rewrite a Jest setup file to be usable as a --preload file
 */
async function transformSetupFile(targetPath: string) {
  grit.logging.debug(`Transforming Jest setup file at ${targetPath}`);
  const pattern = `js"$func($contents)" where {
    $func <: or {
        js"beforeAll" => js"beforeEachFile" where { add_import(js"beforeEachFile", js"'bun:test'")},
        js"afterAll" => js"afterEachFile" where { add_import(js"afterEachFile", js"'bun:test'")},
    }
}`;
const query = new QueryBuilder(pattern);

  const instanceCount = await query.run([targetPath]);
  grit.logging.info(`Transformed Jest setup file at ${targetPath}, with ${instanceCount} changes`);

  return targetPath;
}

/**
 * Fix the provided bun config
 */
async function fixBunConfig(targetPath: string, props: { [key: string]: any } = {}) {
  grit.logging.debug(`Fixing Bun config at ${targetPath}`);
  const pattern = `
language toml

file($body) where {
    // $props = {preload: \`"foo.ts"\`},
    $props = {${Object.entries(props).map(([key, value]) => `${key}: \`${value}\``).join(', ')}},
    if ($body <: contains \`[test]
$existing\`) {
        $props <: some bubble($existing) [$key, $value] where {
            or {
                $existing <: contains \`$key = $_\` => \`$key: $value\`,
                $existing += \`\n$key = $value\`
            }
        }
    } else {
        $body += \`\n[test]\n\`,
        $props <: some bubble($body) [$key, $value] where {
            $body += \`$key = $value\n\`
        }
    }
}`;

grit.logging.debug(`Pattern: ${pattern}`);

    const query = new QueryBuilder(pattern);

    const instanceCount = await query.run([targetPath]);
    grit.logging.info(`Fixed Bun config at ${targetPath}, with ${instanceCount} changes`);

    return targetPath;
}

export default await sdk.defineWorkflow({
  name: 'jest_to_bun',

  run: async (options) => {
    const targetDir = process.cwd();

    const configRoot = targetDir;

    // Look for a jest.config.json file
    const targetFile = 'jest.config.js';
    const config = await import(`${targetDir}/${targetFile}`);

    grit.logging.debug(`Successfully loaded config`, config);

    // Props we want to set up
    const props: { [key: string]: string } = {};

    await transformTests(`${configRoot}/src`);

    // Convert from setupFilesAfterEnv -> preload with beforeEachFile
    if (config.setupFilesAfterEnv) {
      const preloads = [];
      for (const setupFile of config.setupFilesAfterEnv) {
        const actualFile = `${configRoot}/${setupFile}`;
        const fixed = await transformSetupFile(actualFile);
        grit.logging.debug(`Adding ${fixed} to preload`);
        preloads.push(setupFile);
      }
      props.preload = JSON.stringify(preloads);
    }

    if (config.bail) {
      props.bail = config.bail;
    }

    if (config.collectCoverage) {
      props.coverage = config.collectCoverage;
    }

    if (config.coverageThreshold?.global) {
      let parts = [];
      if (config.coverageThreshold.global.lines) {
        parts.push(`line = ${config.coverageThreshold.global.lines / 100}`);
      }
      if (config.coverageThreshold.global.functions) {
        parts.push(`function = ${config.coverageThreshold.global.functions / 100}`);
      }
      if (config.coverageThreshold.global.statements) {
        parts.push(`statement = ${config.coverageThreshold.global.statements / 100}`);
      }
      props.coverageThreshold = `{${parts.join(', ')}}`;
    }

    // Apply the bunfig
    const bunConfig = `${configRoot}/Bunfig.toml`;
    await fixBunConfig(bunConfig, props);

    return {
      success: true,
      message: 'Jest to Bun',
    }
  },
});

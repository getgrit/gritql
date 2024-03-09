/**
 * This is just a big file from our codebase.
 */

import { MockCheckpointer, TaskTree, injectContext } from '@getgrit/sdk';
import {
  extractPath,
  getFlexiblePaths,
  injectOriginalFilePath,
  isFileAction,
  makeBranchRef,
} from '@getgrit/universal';
const {
  GRIT_APP_BOT_AUTHOR,
  MarzanoEngine,
  Repo,
  addAllToStaging,
  applyFileAction,
  commitWithAuthor,
  getAuthorsSince,
  push,
} = require('@getgrit/sdk');
import { mkdtemp } from 'fs-extra';
import { groupBy, isEqual, isString, maxBy, orderBy } from 'lodash';
import minimatch from 'minimatch';
import path from 'path';
import { inspect } from 'util';
import { v4 } from 'uuid';
import { getConfigValue } from '../../lib/config';
import { client } from '../../lib/db';
import logger from '../../lib/logger';
import { makeBranchSmartRef } from '../../models/gitref';
import { ProjectExecution } from '../../models/migrations/project-execution';
import { TrackedPullRequest, getMigrationName } from '../../models/migrations/tracked';
import { analytics } from '../analytics';
import { EngineService } from '../engine';
import { git } from '../git';
import { MockPatternResolver } from '../patterns';
import { getQueueName, getTemporalConfig } from './env';
import { ProvoloneInteractor } from './workflow_interactor';
import { hub } from '../../providers/github';
import { packageContext } from './context';

export class WorkflowService {
  async getClient() {
    if (this.client) return this.client;
    const { crt, key, address, namespace } = getTemporalConfig('client');

    return this.client;
  }

  injectContext(topDir, engine, tree) {
    const context = {
      topDir,
      engineUrl: engine.ENGINE_URL,
    };
    // TODO: make this much safer
    injectContext({ context, tree });
  }

  async watch(runId, emitter) {
    const [execution, items] = await Promise.all([
      ProjectExecution.model.findFirst({
        where: {
          id: runId,
        },
      }),
      client.workflow_work_item.findMany({
        where: {
          execution_id: runId,
        },
      }),
    ]);
    const orderedItems = orderBy(items, 'created_at');
    if (!execution) {
      throw new Error(`Execution ${runId} not found`);
    }
    const { commit, status, migration_id } = execution;
    for (const item of orderedItems) {
      const result = item.result;
      if (!isFileAction(result)) {
        continue;
      }
      emitter.emit('workflow-result', {
        runId,
        filePath: extractPath(result),
        originalContent: item.original_content,
        result,
      });
    }
    return { commit, status, migration_id };
  }

  /**
   * Run a "preview" of a workflow on a fake project, for landing page
   */
  async previewSynthetic(user, execution, richFiles, migration, emitter, traceData) {
    return this.executePackagedWorkflow({
      eventName: 'synthetic_workflow_preview',
      user,
      richFiles,
      execution,
      migration,
      emitter,
      traceData,
    });
  }

  /**
   * Run a workflow remotely via Temporal (the primary way going forward)
   * @param options
   * @returns
   */
  async startPackagedWorkflow(options) {
    const client = await this.getClient();
    const packaged = await packageContext(options);
    const migrationName = await getMigrationName(options.migration);
    const handle = await client.start('run_packaged_workflow', {
      workflowId: options.execution.id,
      args: [
        migrationName,
        packaged,
        { simple: options.eventName === 'synthetic_workflow_preview' },
      ],
      taskQueue: getQueueName(),
    });
    return handle;
  }

  async executePackagedWorkflow(options) {
    const workflow = await this.startPackagedWorkflow(options);
    const result = await workflow.result();
    return result;
  }

  /** Run a scan workflow */
  async scan(user, execution, project, ref, migration) {
    if ((await execution.getKind()) !== 'SCAN') {
      throw new Error(`Attempting to scan a non-scan execution`);
    }
    return this.executePackagedWorkflow({
      eventName: 'workflow_scan',
      user,
      execution,
      project,
      ref,
      migration,
      scanOnly: true,
    });
  }

  async preview(user, execution, project, ref, migration, emitter, globs, traceData) {
    return this.executePackagedWorkflow({
      eventName: 'workflow_preview',
      user,
      execution,
      project,
      ref,
      migration,
      emitter,
      globs,
      traceData,
    });
  }

  /**
   * Triggers an operational workflow through the queue and returns the workflow handle.
   */
  async triggerTraditionalWorkflow(name, input) {
    const client = await this.getClient();
    const workflowId = v4();
    logger.debug('workflow started', { workflowId, name, input });
    const result = await client.start('run_traditional_workflow', {
      workflowId,
      args: [name, input],
      taskQueue: getQueueName(),
    });
    return result;
  }

  /**
   * Runs a workflow through the queue and returns a promise that resolves with the result.
   */
  async runTraditionalWorkflow(name, input) {
    const handle = await this.triggerTraditionalWorkflow(name, input);
    const result = await handle.result();
    return result;
  }

  /**
   * Runs a workflow directly, without going through the worker queue.
   */
  async runDirectWorkflow(migrationName, input) {
    analytics.trackForService('workflows', 'workflow_started', {
      migrationName,
      ...input,
    });
    const migrationPath = path.join(__dirname, `../../workflows/${migrationName}`);
    const migration = require(migrationPath);

    const engine = (await EngineService.getDefault()).engine;

    const topDir = await mkdtemp('/tmp/grit-workflow-');
    const tree = new TaskTree(
      {
        topDir,
        continueOnFailure: true,
        OPENAI_API_KEY: getConfigValue('OPENAI_API_KEY'),
        mode: 'regular',
      },
      {},
      new ProvoloneInteractor(migration),
      new MockCheckpointer(),
      {
        scala: {
          start: async () => {},
          engine: async () => {
            return engine;
          },
          resolver: async () => {
            return new MockPatternResolver();
          },
        },
        marzano: {
          start: async () => {},
          engine: async () => {
            return new MarzanoEngine();
          },
          resolver: async () => {
            return new MockPatternResolver();
          },
        },
      },
    );
    this.injectContext(topDir, engine, tree);

    let res;
    try {
      res = await migration.migrate(input);
      analytics.trackForService('workflows', 'workflow_finished', {
        migrationName,
        ...input,
      });
    } catch (e) {
      console.error(e);
      logger.error(`Failed to run migration ${migrationName} with input ${Object.keys(input)}`, {
        e,
        error: inspect(e),
      });
      analytics.trackForService('workflows', 'workflow_failed', {
        migrationName,
        ...input,
      });
      res = false;
    }

    return res;
  }

  async applyStoredExecution(execution, worktree, globs) {
    const messages = (await execution.getWorkItems()).map((r) => ({
      ...r.result,
      originalPath: r.original_path ?? undefined,
      createdAt: r.created_at,
    }));

    const messagesByFile = groupBy(messages, 'originalPath');

    const filterGlobs = minimatch.filter(`{${getFlexiblePaths(globs).join(',')}}`, {
      matchBase: true,
    });

    const lastMessageByFile = {};
    Object.entries(messagesByFile).forEach(([file, actions]) => {
      const lastAction = maxBy(actions, (action) => action.createdAt);
      if (lastAction && filterGlobs(file)) {
        lastMessageByFile[file] = lastAction;
      }
    });

    const applied = (
      await Promise.all(
        Object.keys(lastMessageByFile).map(async (file) => {
          const action = injectOriginalFilePath(lastMessageByFile[file], file);
          try {
            const applied = await applyFileAction(action, worktree);
            return applied;
          } catch (e) {
            logger.error(`Error applying action ${action.__typename} to ${file}: ${inspect(e)}`, {
              execution,
              action,
              file,
              e,
            });
          }
        }),
      )
    ).filter(isString);

    logger.info(`Applied pattern to ${applied.length} original files`, {
      applied,
    });
  }

  async createPullRequest(execution, baseBranch, globs) {
    const branchName = v4();
    const targetRef = makeBranchSmartRef(branchName);
    const project = await execution.getProject();
    const repo = new Repo(project.repo);
    const baseCommit = await execution.getCommit();
    const worktree = await git.checkoutNew(repo, targetRef, baseCommit);

    await this.applyStoredExecution(execution, worktree, globs);

    await addAllToStaging(worktree);
    const author = GRIT_APP_BOT_AUTHOR;
    logger.info('Committing migration execution results', {
      execution,
      worktree,
    });

    await commitWithAuthor(author, `[bot] migrate files`, worktree);
    await push(worktree, targetRef.name, logger);
    const pr = await hub.createPullRequest(
      repo,
      baseBranch.name,
      branchName,
      'Apply autogenerated pattern',
    );

    const tracked = TrackedPullRequest.fromParent(pr);
    tracked.upsert(tracked.data, execution);
    return pr;
  }

  async updatePullRequest(execution, globs, pullRequest) {
    const data = await pullRequest.getData();
    const branchName = data.head.ref;
    const prBranch = makeBranchRef(branchName);
    const project = await execution.getProject();
    const repo = new Repo(project.repo);
    const runnerBranch = makeBranchRef(v4());
    const worktree = await git.checkoutNew(repo, runnerBranch, prBranch);

    const prBaseBranch = makeBranchRef(data.base.ref);

    const lastAuthor = await getAuthorsSince(worktree, prBaseBranch, logger);
    if (lastAuthor.find((author) => !isEqual(author, GRIT_APP_BOT_AUTHOR))) {
      throw new Error(
        'Your PR was not updated because there have been changes since the last Grit commit',
      );
    }

    await this.applyStoredExecution(execution, worktree, globs);

    await addAllToStaging(worktree);
    const author = { name: 'grit-app[bot]' };
    logger.info('Committing migration execution results', {
      execution,
      worktree,
    });

    try {
      await commitWithAuthor(author, `[bot] migrate files`, worktree);
      await push(worktree, prBranch.name, logger);
    } catch (e) {
      console.log(`e here is ${inspect(e)} and stderr is ${e.stderr}`);
      if (e.stdout?.toString()?.includes('nothing to commit')) {
        logger.info('No changes to commit, skipping pull request update');
      } else {
        throw e;
      }
    }

    await pullRequest.upsert(undefined, execution);
    return pullRequest;
  }
}

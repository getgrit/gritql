import type { ChildProcessWithoutNullStreams } from 'child_process';
import { spawn } from 'child_process';
import type { stdlib } from '@getgrit/api';

export function wrap_exec({
  callback,
  cp,
  passOnNonZeroExitCode,
}: {
  callback?: (write: (str: string) => void, end: () => void) => void;
  cp: ChildProcessWithoutNullStreams;
  passOnNonZeroExitCode: boolean | undefined;
}): Promise<stdlib.ShResult> {
  return new Promise<stdlib.ShResult>((resolve) => {
    const allout = [] as string[];
    const stdout = [] as string[];
    const stderr = [] as string[];
    const write = (str: string): void => {
      process.stdin.cork();
      process.stdin.write(str);
      process.stdin.uncork();
    };

    const end = (): void => {
      process.stdin.end();
    };

    if (callback) {
      callback(write, end);
    }

    cp.stdout.on('data', (data: string) => {
      stdout.push(data);
      allout.push(data);
    });
    cp.stderr.on('data', (data: string) => {
      stderr.push(data);
      allout.push(data);
    });
    cp.on('error', (e) => {
      resolve({
        __typename: 'ShResult',
        kind: 'direct',
        success: false,
        code: -1,
        allout: allout.join(''),
        stdout: stdout.join(''),
        stderr: stderr.join(''),
        message: e.message,
      });
    });
    cp.on('close', (code: number) => {
      // Trailing info is more valuable than truncated.
      const message = stderr.join('').slice(-2000);
      resolve({
        __typename: 'ShResult',
        kind: 'direct',
        success: passOnNonZeroExitCode ? true : code === 0,
        code: code || 0,
        allout: allout.join(''),
        stdout: stdout.join(''),
        stderr: stderr.join(''),
        message,
      });
    });
  });
}

export function baseSh(
  cmd: string,
  callback: (write: (str: string) => void, end: () => void) => void = () => {},
  cwd: string | URL | undefined = undefined,
  passOnNonZeroExitCode: boolean | undefined = undefined,
  env: NodeJS.ProcessEnv | undefined = undefined,
  detached?: boolean,
): Promise<stdlib.ShResult> {
  const cp = spawn(cmd, { shell: true, detached, cwd, env: { ...process.env, ...(env ?? {}) } });
  const interruptCP = function () {
    cp.emit('SIGINT');
  };

  try {
    const promise = wrap_exec({ callback, cp, passOnNonZeroExitCode });
    if (!detached) {
      return promise;
    }
  } finally {
    process.removeListener('SIGINT', interruptCP);
  }
  return Promise.resolve({
    __typename: 'ShResult',
    kind: 'detached',
    allout: 'Result is detached',
    stderr: '',
    stdout: 'Result is detached',
    code: 0,
    success: true,
  });
}
/**
 * The most basic sh. Sends output to console if anything goes wrong, that is, if status code != 0
 * @param cmd the command to run
 * @param callback the callback to run
 * @returns All output from the command.
 */

export const simpleSh = async (
  cmd: string,
  callback: (write: (str: string) => void, end: () => void) => void = () => {},
  cwd: string | URL | undefined = undefined,
): Promise<string> => {
  const res = await baseSh(cmd, callback, cwd);
  if (res.code !== 0) {
    console.log(res.stdout);
    console.error(res.stderr);
    throw new Error(res.stderr);
  }
  return res.allout;
};

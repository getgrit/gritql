# Worker code

Next.js has built-in support for including worker code. Unfortunately, this does not traverse module boundaries effectively.

Therefore, we have a very hacky solution of writing the code for the workers here (in apps/docs/src/workers).

It is then copied into `apps/web` via a generate command.

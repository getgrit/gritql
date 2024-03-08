/// We don't want to block the main LSP thread with CPU-intensive work
/// Therefore, we have a dedicated executor that runs on a separate Tokio runtime.
/// This executor is used for applying patterns, which can be CPU-intensive.
///
/// Discussion of this problem:
///   - https://thenewstack.io/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/
///   - https://gist.github.com/alamb/bd0e086448ef9b438aeebd6f550e23ed?utm_source=thenewstack&utm_medium=website&utm_content=inline-mention&utm_campaign=platform
///
/// This implementation spawns a *new* thread for each task.
/// This has somewhat worse performance than keeping a dedicated pool, but it's much more conceptually simple.
///
/// The main purpose of this wrapper is to aggregate all of the CPU intensive work into one place.
/// This makes it easier to add a dedicated thread pool in the future.
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct IntenseExecutor {}

impl IntenseExecutor {
    pub fn new() -> Self {
        Self {}
    }

    /// Spawn a new task on the executor.

    pub fn spawn<F, R>(&self, f: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        tokio::task::spawn_blocking(f)
    }
}

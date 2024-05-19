/// The Error number
#[derive(Debug)]
pub enum ExecutorErr {
    /// Executor not initialized
    NotInit,
    /// RunQueue is empty
    RunQueueEmpty,
}
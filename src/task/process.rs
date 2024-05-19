use core::future::Future;

use crate::state::State;

/// Pid
pub struct PidHandle(usize);

pub trait Process: Future<Output = i32> + 'static + Send + Sync {
    fn pid(&self) -> usize;

    fn status(&self) -> State;

}
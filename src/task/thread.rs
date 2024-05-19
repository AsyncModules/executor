use core::future::Future;

use crate::state::State;

pub struct TidHandle(usize);

pub trait Thread: Future<Output = i32> + 'static + Send + Sync {
    fn tid(&self) -> usize;

    fn state(&self) -> State;
}
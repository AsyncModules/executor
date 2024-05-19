use crate::{TaskRef, BaseScheduler};
use alloc::boxed::Box;
use crossbeam::atomic::AtomicCell;

pub struct Executor {
    ready_queue: AtomicCell<Box<dyn BaseScheduler<SchedItem = TaskRef>>>
}

impl Executor {
    pub const fn new(ready_queue: AtomicCell<Box<dyn BaseScheduler<SchedItem = TaskRef>>>) -> Self {
        Self {
            ready_queue
        }
    }

    pub fn wake_task(&self, task_ref: TaskRef) {
        unsafe { &mut *self.ready_queue.as_ptr() }.add_task(task_ref);
    }
}
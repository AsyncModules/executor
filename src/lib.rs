#![cfg_attr(not(test), no_std)]
#![feature(waker_getters)]
#![feature(const_nonnull_new)]
#![feature(const_option)]
#![feature(extern_types)]

mod error;
mod executor;
mod interface;
mod state;
mod task;
mod timer_queue;
mod util;
mod wait_queue;
mod waker;

extern crate alloc;

pub use interface::BaseScheduler;
pub use task::{Task, TaskRef};
use error::ExecutorErr;
use executor::Executor;
use util::SyncSendUnsafeCell;
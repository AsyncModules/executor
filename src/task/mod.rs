//! Task Control Block.
//!

mod process;
mod thread;

use alloc::{boxed::Box, sync::Arc};
use core::{
    future::Future,
    pin::Pin,
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
    task::{Context, Poll},
};
use crossbeam::atomic::AtomicCell;
use crate::Executor;

use crate::state::State;
use crate::waker::from_task;

pub trait ProcessInfo {

}

pub trait ThreadInfo {

}

pub trait CoroutineInfo {

}

/// The `Task` is stored in heap by using `Arc`.
#[repr(C)]
pub struct Task {
    pub state: State,
    ///
    pub executor: &'static Executor,
    ///
    pub process_info: AtomicCell<Box<dyn ProcessInfo>>,
    ///
    pub thread_info: AtomicCell<Box<dyn ThreadInfo>>,
    ///
    pub coroutine_info: AtomicCell<Box<dyn ThreadInfo>>,
    ///
    pub fut: AtomicCell<Pin<Box<dyn Future<Output = i32> + 'static + Send + Sync>>>,
}

impl Task {

    ///
    fn from_ref(task_ref: TaskRef) -> Arc<Self> {
        let raw_ptr = task_ref.as_ptr();
        unsafe { Arc::from_raw(raw_ptr) }
    }
}
/// This is essentially a `&'static Task`.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskRef {
    ptr: NonNull<Task>,
}

unsafe impl Send for TaskRef {}
unsafe impl Sync for TaskRef {}

impl TaskRef {
    /// Create a virtual task
    pub const unsafe fn virt_task(ptr: usize) -> Self {
        Self {
            ptr: NonNull::new(ptr as *mut Task).unwrap()
        }
    }

    /// Safety: The pointer must have been obtained with `Task::as_ptr`
    pub(crate) unsafe fn from_ptr(ptr: *const Task) -> Self {
        Self {
            ptr: NonNull::new(ptr as *mut Task).unwrap(),
        }
    }

    /// The returned pointer
    pub fn as_ptr(&self) -> *const Task {
        self.ptr.as_ptr()
    }

    /// Get TaskRef from Context
    pub fn from_cx(cx: &mut Context) -> Self {
        let ptr = cx.waker().as_raw().data() as _;
        unsafe { Self::virt_task(ptr) }
    }

    ///
    #[inline(always)]
    pub fn poll(self) -> Poll<i32> {
        unsafe {
            let waker = from_task(self.clone());
            let mut cx = Context::from_waker(&waker);
            let task = Task::from_ref(self);
            let future = &mut *task.fut.as_ptr();
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(res) => Poll::Ready(res),
                Poll::Pending => {
                    task.as_ref();
                    Poll::Pending
                },
            }
        }
    }
}

/// Wake a task by `TaskRef`.
///
/// You can obtain a `TaskRef` from a `Waker` using [`task_from_waker`].
#[inline(always)]
pub fn wake_task(task_ref: TaskRef) {
    unsafe {
        let task_ptr = task_ref.as_ptr();
        if (*task_ptr).state.ready_enqueue() {
            (*task_ptr).executor.wake_task(task_ref)
        }
    }
}


#[test]
fn future_downcast() {
    pub struct TestFuture(i32);

    impl Future for TestFuture {
        type Output = i32;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(self.0)
        }
    }

    impl TestFuture {
        fn new(val: i32) -> Self {
            Self(val)
        }

        fn test_a(&self) {
            println!("test {}", self.0);
        }
    }

    let fut = TestFuture::new(2893);
    let a = Box::new(fut);
    a.test_a();
    
}

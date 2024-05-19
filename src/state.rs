use core::{fmt::Display, sync::atomic::{AtomicU32, Ordering}};

/// Task is spawned (create a task on the heap but not in queue)
pub(crate) const STATE_SPAWNED: u32 = 1 << 0;
/// Task is in the executor ready queue
pub(crate) const STATE_READY_QUEUED: u32 = 1 << 1;
/// Task is in the executor wait queue
pub(crate) const STATE_WAIT_QUEUED: u32 = 1 << 2;
/// Task is in the executor timer queue
pub(crate) const STATE_TIMER_QUEUED: u32 = 1 << 3;

pub(crate) struct State {
    state: AtomicU32,
}

impl State {
    pub const fn new() -> State {
        Self {
            state: AtomicU32::new(STATE_SPAWNED),
        }
    }

    /// Mark the task as ready-queued if it's spawned and isn't already ready-queued. Return true on success.
    #[inline(always)]
    pub fn ready_enqueue(&self) -> bool {
        let old_state = self.state.fetch_or(STATE_READY_QUEUED, Ordering::AcqRel);
        old_state & STATE_READY_QUEUED == 0
    }

    /// Unmark the task as ready-queued. Return whether the task is spawned.
    #[inline(always)]
    pub fn ready_dequeue(&self) -> bool {
        let state = self.state.fetch_and(!STATE_READY_QUEUED, Ordering::AcqRel);
        state & STATE_SPAWNED != 0
    }

    /// Mark the task as timer-queued. Return whether it was newly queued (i.e. not queued before)
    #[inline(always)]
    pub fn timer_enqueue(&self) -> bool {
        let old_state = self.state.fetch_or(STATE_TIMER_QUEUED, Ordering::AcqRel);
        old_state & STATE_TIMER_QUEUED == 0
    }

    /// Unmark the task as timer-queued.
    #[inline(always)]
    pub fn timer_dequeue(&self) {
        self.state.fetch_and(!STATE_TIMER_QUEUED, Ordering::AcqRel);
    }

    /// Mark the task as wait-queued. Return whether it was newly queued (i.e. not queued before)
    #[inline(always)]
    pub fn wait_enqueue(&self) -> bool {
        let old_state = self.state.fetch_or(STATE_WAIT_QUEUED, Ordering::AcqRel);
        old_state & STATE_WAIT_QUEUED == 0
    }

    /// Unmark the task as wait-queued.
    #[inline(always)]
    pub fn wait_dequeue(&self) {
        self.state.fetch_and(!STATE_WAIT_QUEUED, Ordering::AcqRel);
    }
}

impl Display for State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let state = self.state.load(Ordering::Relaxed);
        let str_state = if state & STATE_READY_QUEUED != 0 {
            "ReadyQueued"   
        } else if state & STATE_WAIT_QUEUED != 0 {
            "WaitQueued"
        } else if state & STATE_TIMER_QUEUED != 0 {
            "TimerQueued"
        } else {
            "Spawned"
        };
        write!(f, "{}", str_state)
    }
}

#[test]
fn test_state_fmt() {
    let state = State::new();
    println!("task state: {}", state);

    state.ready_enqueue();
    println!("task state: {}", state);

    state.ready_dequeue();
    println!("task state: {}", state);

    state.wait_enqueue();
    println!("task state: {}", state);

    state.wait_dequeue();
    println!("task state: {}", state);

    state.timer_enqueue();
    println!("task state: {}", state);

    state.timer_dequeue();
    println!("task state: {}", state);

    println!("task state: {}", state);

}
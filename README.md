# executor

Target: Provides an out-of-the-box executor that can be used in multiple operating systems.

## Submodules

### Executor

### Task & TaskRef

The `Task` structure is the concept of `Task Control Block(TCB)`. It must play the same role as the `TCB` in the traditional operating system. So it must implement some traits to provide informations when the task is running.

All information is obtained through traits. Different information is obtained through different traits.

所有信息都通过 trait 获得，不同的接口获得不同的信息，不同的任务在不同的环境只能使用固定的 trait。

需要类型转换，根 trait 是 Future<Output = i32> + 'static + Send + Sync；

进程的 trait 是 Process。

需要从 future trait 转化成 Process + Future trait。

在process trait中需要定义好内存管理、进程管理、文件管理、进程同步互斥等接口。

### Queue

- [ ] ReadyQueue
- [ ] TimerQueue
- [ ] WaitQueue

All queues are used to store the `TaskRef`. The `ReadyQueue`  depends on the Scheduler. The `TimerQueue` and `WaitQueue` also depends on the Queue.

It can be used

### State

用原子操作来判断任务状态，但这个状态只能在特定的特权级下来使用


	普通的协程在运行时，通过cx可以获得 TaskRef，从而在 Task 中获得具体的信息。但这种方式只能获得普通的，直接在任务控制块中定义的信息，对于通过系统调用获取的信息，需要进入到内核，在内核中通过上述的方法获取信息，再返回到用户态。


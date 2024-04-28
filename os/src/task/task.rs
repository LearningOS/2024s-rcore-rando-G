//! Types related to task management

use super::TaskContext;
use crate::task::MAX_SYSCALL_NUM;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
        //lab1 add

    ///任务的开始时间
    pub start_time:usize,
    
    ///syscall数组
    pub syscall_times:[u32;MAX_SYSCALL_NUM]
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

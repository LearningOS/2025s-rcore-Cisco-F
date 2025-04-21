//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// record syscall times
    pub syscall_times: [usize; MAX_SYSCALL_NUM],
}

impl TaskControlBlock {
    /// update
    pub fn record_syscall(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
        if syscall_id == 410 {
            debug!("syscall {} increased! now is {}", syscall_id, self.syscall_times[syscall_id]);
        }
    }
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

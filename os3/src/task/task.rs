//! Types related to task management

use super::TaskContext;
use crate::task::MAX_SYSCALL_NUM;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub start_time: usize
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

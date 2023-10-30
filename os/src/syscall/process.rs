//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    syscall::syscall_id_from_dense,
    task::{exit_current_and_run_next, get_task_info, suspend_current_and_run_next, TaskStatus},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// get [TaskInfo] of the task
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    if let Some((status, dense_syscall_times, time)) = get_task_info() {
        let mut syscall_times = [0; MAX_SYSCALL_NUM];
        for (i, n) in dense_syscall_times.iter().enumerate() {
            syscall_times[syscall_id_from_dense(i)] = *n;
        }
        unsafe {
            *ti = TaskInfo {
                status,
                syscall_times,
                time,
            }
        }
        0
    } else {
        -1
    }
}

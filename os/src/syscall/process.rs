//! Process management syscalls
use crate::config::MAX_SYSCALL_NUM;
use crate::mm::translated_byte_buffer;
use crate::task::{
    change_program_brk, current_user_token, exit_current_and_run_next, get_task_info, mmap, munmap,
    suspend_current_and_run_next, TaskStatus,
};
use crate::timer::get_time_us;

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
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// current time in the format of [TimeVal]
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    set_val_in_user_memory(ts, &time_val)
}

/// get [TaskInfo] of current task
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    if let Some((status, syscall_times, time)) = get_task_info() {
        let task_info = TaskInfo {
            status,
            syscall_times,
            time,
        };
        set_val_in_user_memory(ti, &task_info)
    } else {
        -1
    }
}

/// map files or devices into memory
///
/// This is a simplified version which only allocate memory
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    mmap(start, len, port)
}

/// unmap files or devices into memory
///
/// This is a simplified version which only deallocate memory
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    munmap(start, len)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

fn set_val_in_user_memory<T: Sized>(ptr: *mut T, val: &T) -> isize {
    let len = core::mem::size_of::<T>();
    let buffers = translated_byte_buffer(current_user_token(), ptr as *const u8, len);
    let value_bytes = unsafe { any_as_u8_slice(val) };
    let mut start = 0;
    for buffer in buffers {
        let buffer_size = buffer.len();
        let end = start + buffer_size;
        buffer.copy_from_slice(&value_bytes[start..end]);
        start = end;
    }
    if start == len {
        0
    } else {
        -1
    }
}

//! Process management syscalls
use core::slice;

use crate::{config::CLOCK_FREQ, mm::translated_byte_buffer, task::{change_program_brk, current_user_token, exit_current_and_run_next, read_byte, suspend_current_and_run_next, task_info, write_byte}, timer::get_time};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let tick = get_time();
    let sec = tick / CLOCK_FREQ;
    let usec = (tick % CLOCK_FREQ) * 1_000_000 / CLOCK_FREQ;
    let time = TimeVal { sec, usec };

    let token = current_user_token();
    let mut buffers = translated_byte_buffer(token, ts as *const u8, core::mem::size_of::<TimeVal>());
    let src = unsafe {
        slice::from_raw_parts(&time as *const _ as *const u8, core::mem::size_of::<TimeVal>())
    };
    buffers[0].copy_from_slice(src);

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    let result = match trace_request {
        0 => read_byte(id) as isize,
        1 => write_byte(id, data) as isize,
        2 => task_info(id) as isize,
        _ => -1,
    };
    result
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
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

//! File and filesystem-related syscalls
use crate::fs::{link, open_file, unlink, OpenFlags, Stat};
use crate::mm::{translated_byte_buffer, translated_str, UserBuffer, VirtAddr};
use crate::task::{current_task, current_user_token, va_to_pa};

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_write", current_task().unwrap().pid.0);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_read", current_task().unwrap().pid.0);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        trace!("kernel: sys_read .. file.read");
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    trace!("kernel:pid[{}] sys_open", current_task().unwrap().pid.0);
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = task.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    trace!("kernel:pid[{}] sys_close", current_task().unwrap().pid.0);
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_fstat(fd: usize, st: *mut Stat) -> isize {
    trace!(
        "kernel:pid[{}] sys_fstat",
        current_task().unwrap().pid.0
    );

    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if let Some(inode) = &inner.fd_table[fd] {
        let stat = inode.fstat();
        drop(inner);

        let va = VirtAddr::from(st as usize);
        let pa = va_to_pa(va);
        match pa {
            Some(pa) => {
                let pa = pa.0 as *mut Stat;
                unsafe {
                    (*pa).ino = stat.0;
                    (*pa).mode = stat.1;
                    (*pa).nlink = stat.2;
                }
                0
            },
            None => {
                error!("sys_fstat, convert va to pa failed!");
                -1
            }
        }
    } else {
        -1
    }
}

pub fn sys_linkat(old_name: *const u8, new_name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_linkat NOT IMPLEMENTED",
        current_task().unwrap().pid.0
    );
    let old_name = va_to_pa(VirtAddr::from(old_name as usize));
    let new_name = va_to_pa(VirtAddr::from(new_name as usize));
    if let (Some(old_name), Some(new_name)) = (old_name, new_name) {
        let old_name = old_name.0 as *const u8;
        let new_name = new_name.0 as *const u8;
        link(old_name, new_name)
    } else {
        error!("kernel: sys_linkat failed!");
        -1
    }
}

pub fn sys_unlinkat(name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_unlinkat",
        current_task().unwrap().pid.0
    );
    let pa = va_to_pa(VirtAddr::from(name as usize));
    if let Some(pa) = pa {
        unlink(pa.0 as *const u8)
    } else {
        error!("kernel: sys_unlinkat failed!");
        -1
    }
}

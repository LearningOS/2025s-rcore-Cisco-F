//! utils
use alloc::vec::Vec;

/// get a usize from a byte buffer with length of size_of::<usize>()
pub fn from_translated_byte_buffer(value: Vec<&mut [u8]>) -> usize {
    let mut result: usize = 0;
    let mut pos = 0;
    for slice in value.iter() {
        for byte in slice.iter() {
            result |= (*byte as usize) << (8 * pos);
            pos += 1;
        }
    }
    debug!("ptr is {}", result);
    result
}
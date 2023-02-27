#![no_std]
#![no_main]

extern crate rt;

use core::ptr;

static HELLO_WORLD: &[u8] = b"Hello, world from Rust!\n";
static mut DATA: u32 = 10;

#[no_mangle]
pub fn main() {
    // Demonstrate that the static data has in fact been initialised as expected.
    let count = unsafe { DATA };

    let tty_status = 0x8000 as *mut u8;
    let tty_data = 0x8001 as *mut u8;

    for _ in 1..=count {
        for &byte in HELLO_WORLD.iter() {
            unsafe {
                while ptr::read_volatile(tty_status) & 0x01 == 0 {}
                ptr::write_volatile(tty_data, byte);
            }
        }
    }
}

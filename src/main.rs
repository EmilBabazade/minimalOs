#![no_std]
#![no_main]

use core::panic::PanicInfo;

// this function is called during panic
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

// starting point
static NICE: &[u8] = b"Badamdar yek";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in NICE.iter().enumerate() {
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    
    loop {}
}
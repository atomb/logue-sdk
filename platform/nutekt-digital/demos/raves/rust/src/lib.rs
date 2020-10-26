#![no_std]
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rmul(x: f32, y: f32) -> f32 {
    return x * y
}

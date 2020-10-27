#![no_std]
use core::f32;
use core::panic::PanicInfo;
use micromath::F32Ext;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/*
#[repr(C)]
struct RavesState {
    wave0: *const f32;
    wave1: *const f32;
    subwave: *const f32;
    phi0: f32,
    phi1: f32,
    phisub: f32,
    w00: f32,
    w01: f32,
    w0sub: f32,
    lfo: f32,
    lfoz: f32,
    dither: f32,
    bitres: f32,
    bitresrcp: f32,
    imperfection: f32,
    flags: u8,
}

#[repr(C)]
struct RavesParams {
    submix: f32,
    ringmix: f32,
    bitcrush: f32,
    shape: f32,
    shiftshape: f32,
    wave0: u8,
    wave1: u8,
    subwave: u8,
    padding: u8,
}
*/

#[no_mangle]
pub extern "C" fn r_mul_round(sig: f32, bitres: f32, bitresrcp: f32) -> f32 {
    return (sig * bitres).round() * bitresrcp;
}

#![no_std]
use core::mem::transmute;
use core::panic::PanicInfo;

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
*/

fn f32_copysign(x: f32, y: f32) -> f32 {
    unsafe {
        let xi = transmute::<f32, u32>(x);
        let yi = transmute::<f32, u32>(y);
        let zi = (xi & 0x7fffffff) | (yi & 0x80000000);
        return transmute::<u32, f32>(zi);
    }
}

fn f32_round(x: f32) -> f32 {
    return ((x + f32_copysign(0.5, x)) as i32) as f32;
}

#[no_mangle]
pub extern "C" fn r_mul_round(sig: f32, bitres: f32, bitresrcp: f32) -> f32 {
    return f32_round(sig * bitres) * bitresrcp;
}

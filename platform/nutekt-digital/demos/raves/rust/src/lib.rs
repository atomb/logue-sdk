#![no_std]
use core::f32;
use core::panic::PanicInfo;
use core::ptr;
use micromath::F32Ext;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//const SAMPLERATE_RECIP: f32 = 2.08333333333333e-005f32;
const SR440: f32            = 9.16666666666666e-003f32;
const SR220: f32            = 4.58333333333333e-003f32;

const k_waves_a_cnt : u16 = 16;
const k_waves_b_cnt : u16 = 16;
const k_waves_c_cnt : u16 = 14;
const k_waves_d_cnt : u16 = 13;
const k_waves_e_cnt : u16 = 15;
const k_waves_f_cnt : u16 = 16;

extern "C" {
    static wavesA: *const f32;
    static wavesD: *const f32;
}

#[repr(C)]
pub struct UserOscParams {
    shape_lfo: i32,
    pitch: u16,
    cutoff: u16,
    resonance: u16,
    reserved0: [u16; 3],
}

#[repr(u8)]
pub enum RavesFlags {
    None     = 0,
    Wave0    = 1 << 1,
    Wave1    = 1 << 2,
    SubWave  = 1 << 3,
    RingMix  = 1 << 4,
    BitCrush = 1 << 5,
    Reset    = 1 << 6,
}

#[repr(u16)]
pub enum UserOscParamId {
    Id1 = 0,
    Id2,
    Id3,
    Id4,
    Id5,
    Id6,
    Shape,
    ShiftShape,
}

#[repr(C)]
pub struct RavesState {
    wave0: *const f32,
    wave1: *const f32,
    subwave: *const f32,
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

impl RavesState {
    pub const fn new() -> Self {
        RavesState {
            wave0: ptr::null(),
            wave1: ptr::null(),
            subwave: ptr::null(),
            phi0: 0.0,
            phi1: 0.0,
            phisub: 0.0,
            w00: SR440,
            w01: SR440,
            w0sub: SR220,
            lfo: 0.0,
            lfoz: 0.0,
            dither: 0.0,
            bitres: 1.0,
            bitresrcp: 1.0,
            imperfection: 0.0,
            flags: RavesFlags::None as u8,
        }
    }
}

#[repr(C)]
pub struct RavesParams {
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

impl RavesParams {
    pub const fn new() -> Self {
        RavesParams {
            submix: 0.05,
            ringmix: 0.0,
            bitcrush: 0.0,
            shape: 0.0,
            shiftshape: 0.0,
            wave0: 0,
            wave1: 0,
            subwave: 0,
            padding: 0,
        }
    }
}

#[repr(C)]
pub struct Raves {
    state: RavesState,
    params: RavesParams,
}

impl Raves {
    pub const fn new() -> Self {
        Raves {
            params: RavesParams::new(),
            state: RavesState::new()
        }
    }
}

static mut S_RAVES : Raves = Raves::new();

pub fn clip01f(x: f32) -> f32 {
    if x > 1.0 { 1.0 } else if x < 0.0 { 0.0 } else { x }
}

pub fn param_val_to_f32(x: u16) -> f32 {
    x as f32 * 9.77517106549365e-004f32
}

#[no_mangle]
pub extern "C" fn r_mul_round(sig: f32, bitres: f32, bitresrcp: f32) -> f32 {
    return (sig * bitres).round() * bitresrcp;
}

#[no_mangle]
pub extern "C" fn r_osc_init(_raves: &mut Raves, _platform: u32, _api: u32) {
    /*
    unsafe {
        S_RAVES.state.wave0 = wavesA;
        S_RAVES.state.wave1 = wavesD;
        S_RAVES.state.subwave = wavesA;
    }
    */
}

#[no_mangle]
pub extern "C" fn r_osc_noteon(raves: &mut Raves, params: &UserOscParams) {
    raves.state.flags |= RavesFlags::Reset as u8;
}

#[no_mangle]
pub extern "C" fn r_osc_noteoff(raves: &mut Raves, params: &UserOscParams) {
}

#[no_mangle]
pub extern "C" fn r_osc_param(raves: &mut Raves, index: UserOscParamId, value: u16) {
    let p : &mut RavesParams = &mut raves.params;
    let s : &mut RavesState = &mut raves.state;

    match index {
        UserOscParamId::Id1 => {
            let cnt : u16 = k_waves_a_cnt + k_waves_b_cnt + k_waves_c_cnt;
            p.wave1 = (value % cnt) as u8;
            s.flags |= RavesFlags::Wave0 as u8;
        },
        UserOscParamId::Id2 => {
            let cnt : u16 = k_waves_d_cnt + k_waves_e_cnt + k_waves_f_cnt;
            p.wave1 = (value % cnt) as u8;
            s.flags |= RavesFlags::Wave1 as u8;
        },
        UserOscParamId::Id3 => {
            p.subwave = (value % k_waves_a_cnt) as u8;
            s.flags |= RavesFlags::SubWave as u8;
        },
        UserOscParamId::Id4 => {
            p.submix = clip01f(0.05 + (value as f32 * 0.01 * 0.90));
        },
        UserOscParamId::Id5 => {
            p.ringmix = clip01f(value as f32 * 0.01);
        },
        UserOscParamId::Id6 => {
            p.bitcrush = clip01f(value as f32 * 0.01);
            s.flags |= RavesFlags::BitCrush as u8;
        },
        UserOscParamId::Shape => {
            p.shape = param_val_to_f32(value);
        },
        UserOscParamId::ShiftShape => {
            p.shiftshape = 1.0 + param_val_to_f32(value);
        },
    }
}

/*
#[no_mangle]
pub extern "C" fn r_noteon(_params: *const UserOscParams) {
    let s: &mut RavesState = unsafe { &mut S_RAVES.state };
    let p: &mut RavesParams = unsafe { &mut S_RAVES.params };
}

*/

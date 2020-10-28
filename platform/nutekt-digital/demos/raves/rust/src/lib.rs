#![no_std]

use panic_halt as _;
use core::f32;
use core::ptr;
use micromath::F32Ext;

pub mod biquad;
pub mod mathutil;
pub mod nts1;

use mathutil::*;
use nts1::*;

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

    pub fn reset(&mut self) {
        self.phi0 = 0.0;
        self.phi1 = 0.0;
        self.phisub = 0.0;
        self.lfo = self.lfoz;
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
    prelpf: biquad::BiQuad,
    postlpf: biquad::BiQuad,
}

impl Raves {
    pub const fn new() -> Self {
        Raves {
            params: RavesParams::new(),
            state: RavesState::new(),
            prelpf: biquad::BiQuad::new(),
            postlpf: biquad::BiQuad::new(),
        }
    }

    pub fn update_pitch(&mut self, w0: f32) {
        let w0new = w0 + self.state.imperfection;
        let drift = self.params.shiftshape;
        self.state.w00 = w0new;
        // Alt osc with slight drift (0.25Hz@48KHz)
        self.state.w01 = w0new + drift * 5.20833333333333e-006;
        // Sub one octave and a phase drift (0.15Hz@48KHz)
        self.state.w0sub = 0.5 * w0new + drift * 3.125e-006;
    }

    pub fn update_waves(&mut self, flags: u8) {
    }
}

//static mut S_RAVES : Raves = Raves::new();

#[no_mangle]
pub extern "C" fn r_osc_w0f_for_note(note: u8, modulation: u8) -> f32{
    let f0 = osc_notehzf(note);
    let f1 = osc_notehzf(note + 1);
    let f = clipmaxf(linintf(modulation as f32 * K_NOTE_MOD_FSCALE, f0, f1), K_NOTE_MAX_HZ);
    return f * SAMPLERATE_RECIP;
}

#[no_mangle]
pub unsafe extern "C" fn r_osc_white() -> f32{
    _osc_white()
}

#[no_mangle]
pub extern "C" fn r_mul_round(sig: f32, bitres: f32, bitresrcp: f32) -> f32 {
    return (sig * bitres).round() * bitresrcp;
}

#[no_mangle]
pub extern "C" fn r_update_pitch(raves: &mut Raves, w0: f32) {
    raves.update_pitch(w0);
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

pub extern "C" fn r_osc_cycle(raves: &mut Raves, params: &UserOscParams,
                              yn: *mut i32, frames: u32) {

    // Set pitch based on input parameters, waves based on flags
    let phi = (params.pitch >> 8) as u8;
    let plo = (params.pitch & 0xFF) as u8;
    let flags = raves.state.flags;
    raves.update_pitch(r_osc_w0f_for_note(phi, plo));
    raves.update_waves(flags);

    let p : &RavesParams = &raves.params;

    // Preliminary state update
    {
        let sm : &mut RavesState = &mut raves.state;

        if (flags as u8) & (RavesFlags::Reset as u8) != 0 {
            sm.reset();
        }

        if (flags as u8) & (RavesFlags::BitCrush as u8) != 0 {
            sm.dither = p.bitcrush * 2e-008;
            sm.bitres = osc_bitresf(p.bitcrush);
            sm.bitresrcp = 1.0 / sm.bitres;
        }

        sm.lfo = q31_to_f32(params.shape_lfo);
        sm.flags = RavesFlags::None as u8;
    }

    // Pure signal calculation
    let s : &RavesState = &raves.state;
    let mut phi0 = s.phi0;
    let mut phi1 = s.phi1;
    let mut phisub = s.phisub;
    let mut lfoz = s.lfoz;

    let lfo_inc = (s.lfo - lfoz) / frames as f32;

    //let ditheramt = p.bitcrush * 2e-008;

    //let bitres = osc_bitresf(p.bitcrush);
    //let bitres_recip = 1.0 / bitres;

    let submix = p.submix;
    let ringmix = p.ringmix;

    let prelpf = &mut raves.prelpf;
    let postlpf = &mut raves.postlpf;

    for i in 0..frames {
        let wavemix = clipminmaxf(0.005, p.shape + lfoz, 0.995);
        let mut sig = (1.0 - wavemix); // TODO: * osc_wave_scanf(s.wave0, phi0);

        sig += wavemix; // TODO: * osc_wave_scanf(s.wave1, phi1);

        let subsig = 0.0; // TODO: osc_wave_scanf(s.subwave, phisub);

        sig = (1.0 - submix) * sig + submix * subsig;
        sig = (1.0 - ringmix) * sig + ringmix * (subsig * sig);
        sig = clip1m1f(sig);

        sig = prelpf.process_fo(sig);
        sig += s.dither * unsafe { _osc_white() };
        sig = (sig * s.bitres).round() * s.bitresrcp;
        sig = postlpf.process_fo(sig);
        sig = osc_softclipf(0.125, sig);

        // TODO: yn[i] = f32_to_q31(sig);

        phi0 += s.w00;
        phi0 -= (phi0 as u32) as f32;
        phi1 += s.w01;
        phi1 -= (phi1 as u32) as f32;
        phisub += s.w0sub;
        phisub -= (phisub as u32) as f32;
        lfoz += lfo_inc;
    }

    // Final state update
    {
        let sm : &mut RavesState = &mut raves.state;
        sm.phi0 = phi0;
        sm.phi1 = phi1;
        sm.phisub = phisub;
        sm.lfoz = lfoz;
    }
}

#[no_mangle]
pub extern "C" fn r_osc_noteon(raves: &mut Raves, _params: &UserOscParams) {
    raves.state.flags |= RavesFlags::Reset as u8;
}

#[no_mangle]
pub extern "C" fn r_osc_noteoff(_raves: &mut Raves, _params: &UserOscParams) {
}

#[no_mangle]
pub extern "C" fn r_osc_param(raves: &mut Raves, index: UserOscParamId, value: u16) {
    let p : &mut RavesParams = &mut raves.params;
    let s : &mut RavesState = &mut raves.state;

    match index {
        UserOscParamId::Id1 => {
            let cnt : usize = K_WAVES_A_CNT + K_WAVES_B_CNT + K_WAVES_C_CNT;
            p.wave1 = (value % cnt as u16) as u8;
            s.flags |= RavesFlags::Wave0 as u8;
        },
        UserOscParamId::Id2 => {
            let cnt : usize = K_WAVES_D_CNT + K_WAVES_E_CNT + K_WAVES_F_CNT;
            p.wave1 = (value % cnt as u16) as u8;
            s.flags |= RavesFlags::Wave1 as u8;
        },
        UserOscParamId::Id3 => {
            p.subwave = (value % K_WAVES_A_CNT as u16) as u8;
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

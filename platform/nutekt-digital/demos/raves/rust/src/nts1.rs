use crate::mathutil::*;

/// The inverse of the 48,000 Hz sample rate used in the NTS-1.
pub const K_SAMPLERATE_RECIP: f32 = 2.08333333333333e-005f32;

/// SAMPLERATE_RECIP multiplied by 440.0, since FP math isn't allowed in const fns.
pub const K_SR440: f32            = 9.16666666666666e-003f32;

/// SAMPLERATE_RECIP multiplied by 220.0, since FP math isn't allowed in const fns.
pub const K_SR220: f32            = 4.58333333333333e-003f32;

pub const K_MIDI_TO_HZ_SIZE: usize = 152;
pub const K_NOTE_MOD_FSCALE: f32 = 0.00392156862745098f32;
pub const K_NOTE_MAX_HZ: f32 = 23679.643054f32;

pub const K_BITRES_SIZE_EXP : usize = 7;
pub const K_BITRES_SIZE : usize = 1 << K_BITRES_SIZE_EXP;
pub const K_BITRES_MASK : usize = K_BITRES_SIZE - 1;
pub const K_BITRES_LUT_SIZE : usize = K_BITRES_SIZE + 1;

pub const K_TANPI_SIZE_EXP: usize = 8;
pub const K_TANPI_SIZE: usize = 1 << K_TANPI_SIZE_EXP;
pub const K_TANPI_MASK: usize = K_TANPI_SIZE - 1;
pub const K_TANPI_RANGE_RECIP: f32 = 2.04081632653061; // 1/0.49
pub const K_TANPI_LUT_SIZE: usize = K_TANPI_SIZE + 1;

pub const K_WAVES_SIZE_EXP : usize = 7;
pub const K_WAVES_SIZE : usize = 1 << K_WAVES_SIZE_EXP;
pub const K_WAVES_U32_SHIFT : usize = 24;
pub const K_WAVES_FRRECIP : f32 = 5.96046447753906e-008;
pub const K_WAVES_MASK : usize = K_WAVES_SIZE - 1;
pub const K_WAVES_LUT_SIZE : usize = K_WAVES_SIZE + 1;

pub const K_WAVES_A_CNT : usize = 16;
pub const K_WAVES_B_CNT : usize = 16;
pub const K_WAVES_C_CNT : usize = 14;
pub const K_WAVES_D_CNT : usize = 13;
pub const K_WAVES_E_CNT : usize = 15;
pub const K_WAVES_F_CNT : usize = 16;

pub mod userosc;

pub type WaveLUT = [f32; K_WAVES_LUT_SIZE];

// The external symbols exported by the NTS-1 firmware.
extern "C" {

    /// Current platform
    pub static k_osc_api_platform: u32;

    /// Current API version
    pub static k_osc_api_version: u32;

    /// Get a MCU-specific "unique" hash.
    fn _osc_mcu_hash() -> u32;

    static wavesA: [*const WaveLUT; K_WAVES_A_CNT];
    static wavesB: [*const WaveLUT; K_WAVES_B_CNT];
    static wavesC: [*const WaveLUT; K_WAVES_C_CNT];
    static wavesD: [*const WaveLUT; K_WAVES_D_CNT];
    static wavesE: [*const WaveLUT; K_WAVES_E_CNT];
    static wavesF: [*const WaveLUT; K_WAVES_F_CNT];
    static midi_to_hz_lut_f: [f32; K_MIDI_TO_HZ_SIZE];
    static bitres_lut_f: [f32; K_BITRES_LUT_SIZE];
    static tanpi_lut_f: [f32; K_TANPI_LUT_SIZE];
    fn _osc_white() -> f32;
}

/// Gaussian white noise. Returns a value in [-1.0, 1.0].
pub fn osc_white() -> f32 {
    unsafe { _osc_white() }
}

/// Convert 10-bit parameter value to f32
pub fn param_val_to_f32(x: u16) -> f32 {
    x as f32 * 9.77517106549365e-004f32
}

pub fn ptr_as_ref<T>(p: *const T) -> &'static T {
    unsafe { p.as_ref().unwrap() }
}

pub fn get_waves_a_elt(idx: usize) -> *const WaveLUT {
    unsafe { wavesA[idx] }
}

pub fn get_waves_b_elt(idx: usize) -> *const WaveLUT {
    unsafe { wavesB[idx] }
}

pub fn get_waves_c_elt(idx: usize) -> *const WaveLUT {
    unsafe { wavesC[idx] }
}

pub fn get_waves_d_elt(idx: usize) -> *const WaveLUT {
    unsafe { wavesD[idx] }
}

pub fn get_waves_e_elt(idx: usize) -> *const WaveLUT {
    unsafe { wavesE[idx] }
}

pub fn get_waves_f_elt(idx: usize) -> *const WaveLUT {
    unsafe { wavesF[idx] }
}

pub fn osc_wave_scanf(w: &WaveLUT, x: f32) -> f32{
    let p = x - (x as u32) as f32;
    let x0f = p * K_WAVES_SIZE as f32;
    let x0 = x0f as usize & K_WAVES_MASK;
    let x1 = (x0 + 1) & K_WAVES_MASK;
    return linintf(x0f - (x0f as u32) as f32, w[x0], w[x1]);
}

pub fn osc_softclipf(c: f32, x: f32) -> f32 {
    let x = clip1m1f(x);
    return x - c * (x*x*x);
}

pub fn osc_bitresf(x: f32) -> f32 {
    let xf = x * K_BITRES_SIZE as f32;
    let xi = xf as usize;
    let y0 = unsafe { bitres_lut_f[xi] };
    let y1 = unsafe { bitres_lut_f[xi + 1] };
    return linintf(xf - xi as f32, y0, y1);
}

pub fn osc_tanpif(x: f32) -> f32 {
    let idxf = x * K_TANPI_RANGE_RECIP * K_TANPI_SIZE as f32;
    let idx = idxf as usize;
    let y0 = unsafe { tanpi_lut_f[idx] };
    let y1 = unsafe { tanpi_lut_f[idx + 1] };
    return linintf(idxf - idx as f32, y0, y1);
}

pub fn osc_w0f_for_note(note: u8, modulation: u8) -> f32{
    let f0 = osc_notehzf(note);
    let f1 = osc_notehzf(note + 1);
    let f = clipmaxf(linintf(modulation as f32 * K_NOTE_MOD_FSCALE, f0, f1), K_NOTE_MAX_HZ);
    return f * K_SAMPLERATE_RECIP;
}

/// Get Hertz value for `note`, which should be in the range [0-151].
/// Larger values will be clipped to 151.
pub fn osc_notehzf(note: u8) -> f32 {
    let idx = clipmaxnote(note, K_MIDI_TO_HZ_SIZE - 1);
    return unsafe { midi_to_hz_lut_f[idx] };
}

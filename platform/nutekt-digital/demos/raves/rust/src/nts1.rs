use crate::mathutil::*;

pub const SAMPLERATE_RECIP: f32 = 2.08333333333333e-005f32;
pub const SR440: f32            = 9.16666666666666e-003f32;
pub const SR220: f32            = 4.58333333333333e-003f32;

pub const K_BITRES_SIZE_EXP : usize = 7;
pub const K_BITRES_SIZE : usize = 1 << K_BITRES_SIZE_EXP;
pub const K_BITRES_MASK : usize = K_BITRES_SIZE - 1;
pub const K_BITRES_LUT_SIZE : usize = K_BITRES_SIZE + 1;

pub const K_TANPI_SIZE_EXP: usize = 8;
pub const K_TANPI_SIZE: usize = 1 << K_TANPI_SIZE_EXP;
pub const K_TANPI_MASK: usize = K_TANPI_SIZE - 1;
pub const K_TANPI_RANGE_RECIP: f32 = 2.04081632653061; // 1/0.49
pub const K_TANPI_LUT_SIZE: usize = K_TANPI_SIZE + 1;

pub const K_MIDI_TO_HZ_SIZE: usize = 152;
pub const K_NOTE_MOD_FSCALE: f32 = 0.00392156862745098f32;
pub const K_NOTE_MAX_HZ: f32 = 23679.643054f32;

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

#[repr(C)]
pub struct UserOscParams {
    pub shape_lfo: i32,
    pub pitch: u16,
    pub cutoff: u16,
    pub resonance: u16,
    pub reserved0: [u16; 3],
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

pub type WaveLUT = [f32; K_WAVES_LUT_SIZE];

extern "C" {
    pub static wavesA: [*const WaveLUT; K_WAVES_A_CNT];
    pub static wavesB: [*const WaveLUT; K_WAVES_B_CNT];
    pub static wavesC: [*const WaveLUT; K_WAVES_C_CNT];
    pub static wavesD: [*const WaveLUT; K_WAVES_D_CNT];
    pub static wavesE: [*const WaveLUT; K_WAVES_E_CNT];
    pub static wavesF: [*const WaveLUT; K_WAVES_F_CNT];
    pub static midi_to_hz_lut_f: [f32; K_MIDI_TO_HZ_SIZE];
    pub static bitres_lut_f: [f32; K_BITRES_LUT_SIZE];
    pub static tanpi_lut_f: [f32; K_TANPI_LUT_SIZE];
    pub fn _osc_white() -> f32;
}

pub fn param_val_to_f32(x: u16) -> f32 {
    x as f32 * 9.77517106549365e-004f32
}

pub fn ptr_as_ref<T>(p: *const T) -> &'static T {
    unsafe { p.as_ref().unwrap() }
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

pub fn r_osc_w0f_for_note(note: u8, modulation: u8) -> f32{
    let f0 = osc_notehzf(note);
    let f1 = osc_notehzf(note + 1);
    let f = clipmaxf(linintf(modulation as f32 * K_NOTE_MOD_FSCALE, f0, f1), K_NOTE_MAX_HZ);
    return f * SAMPLERATE_RECIP;
}

pub fn osc_notehzf(note: u8) -> f32 {
    let idx = clipmaxnote(note, K_MIDI_TO_HZ_SIZE - 1);
    return unsafe { midi_to_hz_lut_f[idx] };
}

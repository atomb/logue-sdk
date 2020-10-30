use crate::mathutil::*;

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

pub mod clipsat;
pub mod platform;
pub mod random;
pub mod userosc;
pub mod wavebank;

use platform::*;

extern "C" {
    static midi_to_hz_lut_f: [f32; K_MIDI_TO_HZ_SIZE];
    static bitres_lut_f: [f32; K_BITRES_LUT_SIZE];
    static tanpi_lut_f: [f32; K_TANPI_LUT_SIZE];
}

/// Convert 10-bit parameter value to f32
pub fn param_val_to_f32(x: u16) -> f32 {
    x as f32 * 9.77517106549365e-004f32
}

pub fn osc_bitresf(x: f32) -> f32 {
    let xf = x * K_BITRES_SIZE as f32;
    let xi = xf as usize;
    let y0 = unsafe { *bitres_lut_f.get_unchecked(xi) };
    let y1 = unsafe { *bitres_lut_f.get_unchecked(xi + 1) };
    return linintf(xf - xi as f32, y0, y1);
}

pub fn osc_tanpif(x: f32) -> f32 {
    let idxf = x * K_TANPI_RANGE_RECIP * K_TANPI_SIZE as f32;
    let idx = idxf as usize;
    let y0 = unsafe { *tanpi_lut_f.get_unchecked(idx) };
    let y1 = unsafe { *tanpi_lut_f.get_unchecked(idx + 1) };
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
    return unsafe { *midi_to_hz_lut_f.get_unchecked(idx) };
}

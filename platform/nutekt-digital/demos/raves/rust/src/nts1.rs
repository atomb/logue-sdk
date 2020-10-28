use crate::mathutil::*;

pub const SAMPLERATE_RECIP: f32 = 2.08333333333333e-005f32;
pub const SR440: f32            = 9.16666666666666e-003f32;
pub const SR220: f32            = 4.58333333333333e-003f32;

pub const k_waves_a_cnt : usize = 16;
pub const k_waves_b_cnt : usize = 16;
pub const k_waves_c_cnt : usize = 14;
pub const k_waves_d_cnt : usize = 13;
pub const k_waves_e_cnt : usize = 15;
pub const k_waves_f_cnt : usize = 16;
pub const k_midi_to_hz_size: usize = 152;
pub const k_note_mod_fscale: f32 = 0.00392156862745098f32;
pub const k_note_max_hz: f32 = 23679.643054f32;

#[repr(C)]
pub struct UserOscParams {
    shape_lfo: i32,
    pitch: u16,
    cutoff: u16,
    resonance: u16,
    reserved0: [u16; 3],
}

extern "C" {
    pub static wavesA: [*const f32; k_waves_a_cnt];
    pub static wavesD: [*const f32; k_waves_d_cnt];
    pub static midi_to_hz_lut_f: [f32; k_midi_to_hz_size];
}

pub fn param_val_to_f32(x: u16) -> f32 {
    x as f32 * 9.77517106549365e-004f32
}

pub fn osc_notehzf(note: u8) -> f32 {
    unsafe {
        return midi_to_hz_lut_f[clipmaxnote(note, k_midi_to_hz_size - 1)];
    }
}

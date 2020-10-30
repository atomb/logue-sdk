extern "C" {
    /// Current platform
    pub static k_osc_api_platform: u32;

    /// Current API version
    pub static k_osc_api_version: u32;

    /// Get a MCU-specific "unique" hash.
    fn _osc_mcu_hash() -> u32;
}

/// The inverse of the 48,000 Hz sample rate used in the NTS-1.
pub const K_SAMPLERATE_RECIP: f32 = 2.08333333333333e-005;

/// SAMPLERATE_RECIP multiplied by 440.0, since FP math isn't allowed in const fns.
pub const K_SR440: f32            = 9.16666666666666e-003;

/// SAMPLERATE_RECIP multiplied by 220.0, since FP math isn't allowed in const fns.
pub const K_SR220: f32            = 4.58333333333333e-003;

pub fn osc_mcu_hash() -> u32 {
    unsafe { _osc_mcu_hash() }
}

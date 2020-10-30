extern "C" {
    fn _osc_rand() -> u32;
    fn _osc_white() -> f32;
}

/// Returns a random integer in [0, u32::MAX]. Generated with
/// Park-Miller-Carta.
pub fn osc_rand() -> u32 {
    unsafe { _osc_rand() }
}

/// Gaussian white noise. Returns a value in [-1.0, 1.0].
pub fn osc_white() -> f32 {
    unsafe { _osc_white() }
}


const Q31_TO_F32_C : f32 = 4.65661287307739e-010f32;

pub fn clip01f(x: f32) -> f32 {
    if x > 1.0 { 1.0 } else if x < 0.0 { 0.0 } else { x }
}

pub fn clip1m1f(x: f32) -> f32 {
    if x > 1.0 { 1.0 } else if x < -1.0 { -1.0 } else { x }
}

pub fn clipminmaxf(lo: f32, x: f32, hi: f32) -> f32 {
    if x >= hi { hi } else if x <= lo { lo } else { x }
}

pub fn clipmaxf(x: f32, m: f32) -> f32 {
    if x >= m { m } else { x }
}

pub fn clipmaxu32(x: u32, m: u32) -> u32 {
    if x >= m { m } else { x }
}

pub fn clipmaxnote(note: u8, m: usize) -> usize {
    let unote = note as usize;
    if unote >= m { m } else { unote }
}

pub fn linintf(fr: f32, x0: f32, x1: f32) -> f32 {
    x0 + fr * (x1 - x0)
}

pub fn q31_to_f32(x: i32) -> f32 {
    x as f32 * Q31_TO_F32_C
}

pub fn f32_to_q31(x: f32) -> i32 {
    (x * 0x7FFFFFFF as f32) as i32
}

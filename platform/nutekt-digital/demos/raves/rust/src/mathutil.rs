pub fn clip01f(x: f32) -> f32 {
    if x > 1.0 { 1.0 } else if x < 0.0 { 0.0 } else { x }
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
    return x0 + fr * (x1 - x0);
}

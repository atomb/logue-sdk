use crate::mathutil::clip1m1f;

pub fn osc_softclipf(c: f32, x: f32) -> f32 {
    let x = clip1m1f(x);
    return x - c * (x*x*x);
}

pub struct Coeffs {
    ff0: f32,
    ff1: f32,
    ff2: f32,
    fb1: f32,
    fb2: f32,
}

impl Coeffs {
    pub const fn new() -> Self {
        Coeffs {
            ff0: 0.0,
            ff1: 0.0,
            ff2: 0.0,
            fb1: 0.0,
            fb2: 0.0,
        }
    }
}

pub struct BiQuad {
    coeffs: Coeffs,
    z1: f32,
    z2: f32,
}

impl BiQuad {
    pub const fn new() -> Self {
        BiQuad {
            coeffs: Coeffs::new(),
            z1: 0.0,
            z2: 0.0,
        }
    }
}

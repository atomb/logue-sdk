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

    pub fn set_pole_lp(&mut self, pole: f32) {
        self.ff0 = 1.0 - pole;
        self.ff1 = 0.0;
        self.ff2 = 0.0;
        self.fb1 = -pole;
        self.fb2 = 0.0;
    }

    pub fn set_folp(&mut self, k: f32) {
        let kp1 = k + 1.0;
        let km1 = k - 1.0;
        self.ff0 = k / kp1;
        self.ff1 = k / kp1;
        self.ff2 = 0.0;
        self.fb1 = km1 / kp1;
        self.fb2 = 0.0;
    }
}

pub struct BiQuad {
    pub coeffs: Coeffs,
    z1: f32,
    // z2: f32, // Included in C++ but unused in the code ported so far
}

impl BiQuad {
    pub const fn new() -> Self {
        BiQuad {
            coeffs: Coeffs::new(),
            z1: 0.0,
            //z2: 0.0,
        }
    }

    pub fn process_fo(&mut self, xn: f32) -> f32 {
        let acc = self.coeffs.ff0 * xn + self.z1;
        self.z1 = self.coeffs.ff1 * xn;
        self.z1 -= self.coeffs.fb1 * acc;
        return acc;

    }
}

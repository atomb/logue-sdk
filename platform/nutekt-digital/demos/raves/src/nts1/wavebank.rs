use crate::mathutil::linintf;

pub type WaveLUT = [f32; K_WAVES_LUT_SIZE];

extern "C" {
    static wavesA: [*const WaveLUT; K_WAVES_A_CNT];
    static wavesB: [*const WaveLUT; K_WAVES_B_CNT];
    static wavesC: [*const WaveLUT; K_WAVES_C_CNT];
    static wavesD: [*const WaveLUT; K_WAVES_D_CNT];
    static wavesE: [*const WaveLUT; K_WAVES_E_CNT];
    static wavesF: [*const WaveLUT; K_WAVES_F_CNT];
}

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

pub fn wave_table_ref(p: *const WaveLUT) -> &'static WaveLUT {
    unsafe { &*p }
}

pub fn get_waves_a_elt(idx: usize) -> *const WaveLUT {
    unsafe { *wavesA.get_unchecked(idx) }
}

pub fn get_waves_b_elt(idx: usize) -> *const WaveLUT {
    unsafe { *wavesB.get_unchecked(idx) }
}

pub fn get_waves_c_elt(idx: usize) -> *const WaveLUT {
    unsafe { *wavesC.get_unchecked(idx) }
}

pub fn get_waves_d_elt(idx: usize) -> *const WaveLUT {
    unsafe { *wavesD.get_unchecked(idx) }
}

pub fn get_waves_e_elt(idx: usize) -> *const WaveLUT {
    unsafe { *wavesE.get_unchecked(idx) }
}

pub fn get_waves_f_elt(idx: usize) -> *const WaveLUT {
    unsafe { *wavesF.get_unchecked(idx) }
}

pub fn osc_wave_scanf(w: &WaveLUT, x: f32) -> f32 {
    let p = x - (x as u32) as f32;
    let x0f = p * K_WAVES_SIZE as f32;
    let x0 = x0f as usize & K_WAVES_MASK;
    let x1 = (x0 + 1) & K_WAVES_MASK;
    return linintf(x0f - (x0f as u32) as f32, w[x0], w[x1]);
}

pub fn osc_wave_scanuf(w: &WaveLUT, x: u32) -> f32 {
    let xu = x as usize;
    let x0 = xu >> K_WAVES_U32_SHIFT;
    let x1 = (x0 + 1) & K_WAVES_MASK;
    let fr = K_WAVES_FRRECIP * ((x & ((1 << K_WAVES_U32_SHIFT) - 1)) as f32);
    return linintf(fr, w[x0], w[x1]);
}

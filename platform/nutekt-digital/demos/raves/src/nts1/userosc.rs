use core::mem;

#[repr(C)]
pub struct UserOscParams {
    /// Value of LFO implicitly applied to shape parameter.
    pub shape_lfo: i32,
    /// Current pitch. High byte: note number. Low byte: fine (0-255).
    pub pitch: u16,
    /// Current cutoff value (0x0000-0x1fff).
    pub cutoff: u16,
    /// Current resonance value (0x0000-0x1fff)
    pub resonance: u16,
    pub reserved0: [u16; 3],
}

#[repr(u16)]
pub enum UserOscParamId {
    /// Edit parameter 1
    Id1 = 0,
    /// Edit parameter 2
    Id2,
    /// Edit parameter 3
    Id3,
    /// Edit parameter 4
    Id4,
    /// Edit parameter 5
    Id5,
    /// Edit parameter 6
    Id6,
    /// Shape parameter
    Shape,
    ///  Alternative Shape parameter: generally available via a shift function
    ShiftShape,
}

/// Convert 10-bit parameter value to f32
pub fn param_val_to_f32(x: u16) -> f32 {
    x as f32 * 9.77517106549365e-004f32
}

type InitCallback = unsafe extern "C" fn(platform: u32, api: u32);
type CycleCallback = unsafe extern "C" fn(params: &UserOscParams, yn: *mut i32, frames: u32);
type OnCallback = unsafe extern "C" fn(params: &UserOscParams);
type OffCallback = unsafe extern "C" fn(params: &UserOscParams);
type MuteCallback = unsafe extern "C" fn(params: &UserOscParams);
type ValueCallback = unsafe extern "C" fn(value: u16);
type ParamCallback = unsafe extern "C" fn(index: UserOscParamId, value: u16);
type DummyCallback = unsafe extern "C" fn();

pub const DEFAULT_RESERVED0: [u8; 7] = [0; 7];
pub const DEFAULT_RESERVED1: [u8; 5*mem::size_of::<DummyCallback>()] =
    [0; 5*mem::size_of::<DummyCallback>()];

#[repr(C)]
#[repr(packed)]
pub struct UserOscHookTable {
    pub magic: [u8; 4],
    pub api: u32,
    pub platform: u8,
    pub reserved0: [u8; 7],

    /// Initialization callback. Must be implemented by your custom
    /// oscillator.
    ///
    /// The `platform` parameter is the current target platform/module.
    ///
    /// The `api` parameter is the current API version.
    pub func_entry: InitCallback,

    /// Rendering callback. Must be implemented by your custom oscillator.
    ///
    /// The `params` parameter contains the current realtime parameter
    /// state.
    ///
    /// The `yn` parameter points to the output buffer (1 sample per
    /// frame).
    ///
    /// The `frames` parameter holds the size of the output buffer.
    ///
    /// The implementation must support at least up to 64 frames.
    /// Optimize it for powers of two.
    pub func_cycle: CycleCallback,

    /// Note on callback. Must be implemented by your custom oscillator.
    ///
    /// The `params` parameter contains the current realtime parameter
    /// state.
    pub func_on: OnCallback,

    /// Note off callback. Must be implemented by your custom
    /// oscillator.
    ///
    /// The `params` parameter contains the current realtime parameter
    /// state.
    pub func_off: OffCallback,

    pub func_mute: MuteCallback,

    pub func_value: ValueCallback,

    /// Parameter change callback. Must be implemented by your custom
    /// oscillator.
    ///
    /// The parameter `index` contains the parameter ID (as in
    /// `UserOscParamId`) and `value` contains the parameter value.
    ///
    /// Resolution is 10 bits for shape and shift-shape, 0-200 for
    /// bipolar percentage paramters (0% at 100, -100% at 0) and 0-11
    /// for unipolar percentage parameters and typeless parameters.
    pub func_param: ParamCallback,

    //pub reserved1: [DummyCallback; 5],
    // Use bytes for the following so it can be zeroed
    pub reserved1: [u8; 5*mem::size_of::<DummyCallback>()],
}

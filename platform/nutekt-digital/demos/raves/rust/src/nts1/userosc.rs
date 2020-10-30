use core::mem;

#[repr(C)]
pub struct UserOscParams {
    pub shape_lfo: i32,
    pub pitch: u16,
    pub cutoff: u16,
    pub resonance: u16,
    pub reserved0: [u16; 3],
}

#[repr(u16)]
pub enum UserOscParamId {
    Id1 = 0,
    Id2,
    Id3,
    Id4,
    Id5,
    Id6,
    Shape,
    ShiftShape,
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
    pub func_entry: InitCallback,
    pub func_cycle: CycleCallback,
    pub func_on: OnCallback,
    pub func_off: OffCallback,
    pub func_mute: MuteCallback,
    pub func_value: ValueCallback,
    pub func_param: ParamCallback,
    //pub reserved1: [DummyCallback; 5],
    // Use bytes for the following so it can be zeroed
    pub reserved1: [u8; 5*mem::size_of::<DummyCallback>()],
}

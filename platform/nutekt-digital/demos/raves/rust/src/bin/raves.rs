#![no_main]
#![no_std]
use core::slice;
use raves::*;
use raves::nts1::userosc::*;

static K_USER_TARGET_NUTEKTDIGITAL: u32 = 3<<8;
static USER_TARGET_PLATFORM: u32 = K_USER_TARGET_NUTEKTDIGITAL;
static K_USER_API_1_1_0 : u32 = (1<<16) | (1<<8) | (0);
static USER_API_VERSION : u32 = K_USER_API_1_1_0;

/// Global Raves state. Safe to access from the functions below because
/// they can never be called concurrently.
#[used]
static mut S_RAVES : Raves = Raves::new();

#[used]
#[no_mangle]
#[link_section = ".hooks"]
static s_hook_table: UserOscHookTable =
  UserOscHookTable {
      magic: ['U' as u8, 'O' as u8, 'S' as u8, 'C' as u8],
      api: USER_API_VERSION,
      platform: (USER_TARGET_PLATFORM>>8) as u8,
      reserved0: [0; 7],
      func_entry: _hook_init,
      func_cycle: _hook_cycle,
      func_on: _hook_on,
      func_off: _hook_off,
      func_mute: _hook_mute,
      func_value: _hook_value,
      func_param: _hook_param,
      reserved1: [0; 5],
  };

#[no_mangle]
unsafe extern "C" fn _hook_init(platform: u32, api: u32) {
    osc_init(&mut S_RAVES, platform, api);
}

#[no_mangle]
unsafe extern "C" fn _hook_cycle(params: &UserOscParams, yn: *mut i32, frames: u32) {
    osc_cycle(&mut S_RAVES, params, slice::from_raw_parts_mut(yn, frames as usize));
}

#[no_mangle]
unsafe extern "C" fn _hook_on(params: &UserOscParams) {
    osc_noteon(&mut S_RAVES, params);
}

#[no_mangle]
unsafe extern "C" fn _hook_off(_params: &UserOscParams) {
}

#[no_mangle]
unsafe extern "C" fn _hook_mute(_params: &UserOscParams) {
}

#[no_mangle]
unsafe extern "C" fn _hook_value(_value: u16) {
}

#[no_mangle]
unsafe extern "C" fn _hook_param(index: UserOscParamId, value: u16) {
    osc_param(&mut S_RAVES, index, value);
}

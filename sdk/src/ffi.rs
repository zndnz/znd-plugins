//! `sdk/znd_plugin.h`（znd 本体）と同じ並びの構造体。ABI 3。

use std::ffi::{c_char, c_void};

pub const ABI: u32 = 3;

pub const PARAM_FLOAT: u32 = 0;
pub const PARAM_BOOL: u32 = 1;
pub const PARAM_CHOICE: u32 = 2;
pub const PARAM_COLOR: u32 = 3;
pub const PARAM_POINT: u32 = 4;
pub const PARAM_TEXT: u32 = 5;
pub const PARAM_LAYER: u32 = 6;

pub const PARAM_PIXELS: u32 = 1;

pub const EFFECT_TIME_DEPENDENT: u32 = 1;
pub const EFFECT_THREAD_SAFE: u32 = 2;
pub const EFFECT_AUDIO: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ParamDesc {
    pub id: *const c_char,
    pub label: *const c_char,
    pub kind: u32,
    pub flags: u32,
    pub def: [f64; 4],
    pub min: f64,
    pub max: f64,
    pub choices: *const c_char,
}

#[repr(C)]
pub struct EffectDesc {
    pub id: *const c_char,
    pub name: *const c_char,
    pub category: *const c_char,
    pub flags: u32,
    pub param_count: u32,
    pub params: *const ParamDesc,
}

#[repr(C)]
pub struct PluginDesc {
    pub abi_version: u32,
    pub vendor: *const c_char,
    pub version: *const c_char,
    pub license: *const c_char,
    pub effect_count: u32,
    pub effects: *const EffectDesc,
}

#[repr(C)]
pub struct Image {
    pub rgba: *mut f32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct Host {
    pub ctx: *mut c_void,
    pub frame_at: unsafe extern "C" fn(*mut c_void, f64, *mut Image) -> i32,
    pub layer_image: unsafe extern "C" fn(*mut c_void, u32, *mut Image) -> i32,
    pub audio: unsafe extern "C" fn(*mut c_void, u32, *mut f32, u32) -> i32,
    pub apply_effect: unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char, *mut Image) -> i32,
    pub set_output: unsafe extern "C" fn(*mut c_void, u32, u32, i32, i32, *mut Image) -> i32,
}

#[repr(C)]
pub struct RenderArgs {
    pub rgba: *mut f32,
    pub width: u32,
    pub height: u32,
    pub time: f64,
    pub fps: f64,
    pub scale: f64,
    pub seed: u32,
    pub param_count: u32,
    pub values: *const f64,
    pub strings: *const *const c_char,
    pub layer_time: f64,
    pub layer_duration: f64,
    pub host: *const Host,
}

#[repr(C)]
pub struct AudioArgs {
    pub samples: *mut f32,
    pub frames: u32,
    pub sample_rate: u32,
    pub start: i64,
    pub time: f64,
    pub instance: u64,
    pub seed: u32,
    pub param_count: u32,
    pub values: *const f64,
    pub strings: *const *const c_char,
}

/// 静的な定義（生ポインタを含む）をスレッド間で共有するための包み。中身は読み取り専用。
pub struct Shared<T>(pub T);
unsafe impl<T> Sync for Shared<T> {}

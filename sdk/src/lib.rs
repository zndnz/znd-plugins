//! znd ネイティブプラグイン（ABI 3）を Rust で書くための SDK。
//!
//! ```ignore
//! use znd_plugin_sdk::*;
//!
//! fn render(f: &mut Frame) -> Result<(), &'static str> {
//!     let gain = f.f(0) as f32;
//!     f.rgba.chunks_exact_mut(4).for_each(|p| { p[0] *= gain; p[1] *= gain; p[2] *= gain; });
//!     Ok(())
//! }
//!
//! plugin! {
//!     vendor: "作者", version: "1.0", license: "MIT",
//!     effects: [
//!         { id: "com.example.gain", name: "明るさ", category: "例", flags: THREAD_SAFE,
//!           params: [float(c"gain", c"倍率", 1.0, 0.0, 4.0)], render: render },
//!     ]
//! }
//! ```
//!
//! 画像はすべて RGBA・リニアライト・プリマルチプライドの f32。座標は左上原点のバッファ画素。

pub mod audio;
pub mod aul;
pub mod color;
pub mod draw;
pub mod ffi;
pub mod image;
pub mod math;
pub mod noise;

pub use crate::image::Image;
pub use ffi::{EFFECT_AUDIO as AUDIO, EFFECT_THREAD_SAFE as THREAD_SAFE, EFFECT_TIME_DEPENDENT as TIME_DEPENDENT};

use ffi::*;
use std::ffi::{c_char, CStr, CString};

pub type FxResult = Result<(), &'static str>;

// ---------- パラメータの定義 ----------

const fn param(id: &'static CStr, label: &'static CStr, kind: u32, flags: u32, def: [f64; 4], min: f64, max: f64) -> ParamDesc {
    ParamDesc { id: id.as_ptr(), label: label.as_ptr(), kind, flags, def, min, max, choices: std::ptr::null() }
}

/// 数値。
pub const fn float(id: &'static CStr, label: &'static CStr, def: f64, min: f64, max: f64) -> ParamDesc {
    param(id, label, PARAM_FLOAT, 0, [def, 0.0, 0.0, 0.0], min, max)
}

/// px 単位の数値（プレビュー解像度に合わせてホストが縮尺を掛けて渡す）。
pub const fn pixels(id: &'static CStr, label: &'static CStr, def: f64, min: f64, max: f64) -> ParamDesc {
    param(id, label, PARAM_FLOAT, PARAM_PIXELS, [def, 0.0, 0.0, 0.0], min, max)
}

pub const fn boolean(id: &'static CStr, label: &'static CStr, def: bool) -> ParamDesc {
    param(id, label, PARAM_BOOL, 0, [def as u8 as f64, 0.0, 0.0, 0.0], 0.0, 0.0)
}

/// 選択肢。`choices` は '|' 区切り。
pub const fn choice(id: &'static CStr, label: &'static CStr, def: u32, choices: &'static CStr) -> ParamDesc {
    let mut p = param(id, label, PARAM_CHOICE, 0, [def as f64, 0.0, 0.0, 0.0], 0.0, 0.0);
    p.choices = choices.as_ptr();
    p
}

/// 色。既定値は sRGB の表示値（0〜1）。描画時はリニアライトのストレートアルファで渡る。
pub const fn color(id: &'static CStr, label: &'static CStr, rgba: [f64; 4]) -> ParamDesc {
    param(id, label, PARAM_COLOR, 0, rgba, 0.0, 0.0)
}

/// 0xRRGGBB の色。
pub const fn color_hex(id: &'static CStr, label: &'static CStr, hex: u32) -> ParamDesc {
    let r = ((hex >> 16) & 255) as f64 / 255.0;
    let g = ((hex >> 8) & 255) as f64 / 255.0;
    let b = (hex & 255) as f64 / 255.0;
    color(id, label, [r, g, b, 1.0])
}

/// 位置。既定値はレイヤーの幅・高さに対する割合。描画時はバッファ画素座標で渡る。
pub const fn point(id: &'static CStr, label: &'static CStr, fx: f64, fy: f64) -> ParamDesc {
    param(id, label, PARAM_POINT, 0, [fx, fy, 0.0, 0.0], 0.0, 0.0)
}

pub const fn text(id: &'static CStr, label: &'static CStr) -> ParamDesc {
    param(id, label, PARAM_TEXT, 0, [0.0; 4], 0.0, 0.0)
}

pub const fn layer(id: &'static CStr, label: &'static CStr) -> ParamDesc {
    param(id, label, PARAM_LAYER, 0, [0.0; 4], 0.0, 0.0)
}

// ---------- 描画 ----------

/// 1 回の描画に渡されるもの。
pub struct Frame<'a> {
    args: &'a RenderArgs,
    pub width: usize,
    pub height: usize,
    /// 入力画像（その場で書き換えてよい）。
    pub rgba: &'a mut [f32],
    output: Option<(Image, i32, i32)>,
}

unsafe fn cstr<'a>(p: *const c_char) -> &'a str {
    if p.is_null() {
        ""
    } else {
        CStr::from_ptr(p).to_str().unwrap_or("")
    }
}

impl<'a> Frame<'a> {
    /// # Safety
    /// `args` はホストが渡した有効な構造体であること。
    pub unsafe fn from_raw(args: &'a RenderArgs) -> Self {
        let n = args.width as usize * args.height as usize * 4;
        Frame {
            args,
            width: args.width as usize,
            height: args.height as usize,
            rgba: std::slice::from_raw_parts_mut(args.rgba, n),
            output: None,
        }
    }

    fn v(&self, i: usize) -> &[f64] {
        if i >= self.args.param_count as usize {
            return &[0.0; 4];
        }
        unsafe { std::slice::from_raw_parts(self.args.values.add(i * 4), 4) }
    }

    /// i 番目の数値パラメータ。
    pub fn f(&self, i: usize) -> f64 {
        self.v(i)[0]
    }
    pub fn b(&self, i: usize) -> bool {
        self.v(i)[0] != 0.0
    }
    /// 選択肢の番号。
    pub fn i(&self, i: usize) -> usize {
        self.v(i)[0].max(0.0) as usize
    }
    /// 色（リニアライト・ストレートアルファ）。
    pub fn color(&self, i: usize) -> [f32; 4] {
        let v = self.v(i);
        [v[0] as f32, v[1] as f32, v[2] as f32, v[3] as f32]
    }
    /// 位置（バッファ画素座標）。
    pub fn point(&self, i: usize) -> [f64; 2] {
        let v = self.v(i);
        [v[0], v[1]]
    }
    pub fn text(&self, i: usize) -> &str {
        if self.args.strings.is_null() || i >= self.args.param_count as usize {
            return "";
        }
        unsafe { cstr(*self.args.strings.add(i)) }
    }
    pub fn has_layer(&self, i: usize) -> bool {
        self.v(i)[0] != 0.0
    }

    /// コンポの時刻（秒）。
    pub fn time(&self) -> f64 {
        self.args.time
    }
    /// レイヤーのイン点からの経過秒（AviUtl の obj.time）。
    pub fn layer_time(&self) -> f64 {
        self.args.layer_time
    }
    /// レイヤーの長さ（秒。AviUtl の obj.totaltime）。
    pub fn duration(&self) -> f64 {
        self.args.layer_duration
    }
    pub fn fps(&self) -> f64 {
        self.args.fps
    }
    /// レイヤーのイン点からのフレーム番号（AviUtl の obj.frame）。
    pub fn frame(&self) -> i64 {
        (self.args.layer_time * self.args.fps + 1e-6).floor() as i64
    }
    /// レイヤーの長さ（フレーム数。AviUtl の obj.totalframe）。
    pub fn total_frames(&self) -> i64 {
        (self.args.layer_duration * self.args.fps).round().max(1.0) as i64
    }
    /// バッファ画素 / レイヤー座標 1 単位。px 単位でない長さ（ParamDesc::float）を画素にするときに掛ける。
    pub fn scale(&self) -> f64 {
        self.args.scale
    }
    pub fn seed(&self) -> u32 {
        self.args.seed
    }

    fn host(&self) -> Option<&Host> {
        unsafe { self.args.host.as_ref() }
    }

    /// 入力画像の複製。
    pub fn input(&self) -> Image {
        Image { w: self.width, h: self.height, data: self.rgba.to_vec() }
    }

    /// 結果を置く。大きさが入力と同じなら左上を (ox, oy) にずらせないので、`output` を使うこと。
    pub fn set(&mut self, img: Image) {
        if img.w == self.width && img.h == self.height {
            self.rgba.copy_from_slice(&img.data);
            self.output = None;
        } else {
            self.output(img, 0, 0);
        }
    }

    /// 大きさの違う結果を置く。(ox, oy) は結果の左上の位置を入力の画素座標で表したもの。
    pub fn output(&mut self, img: Image, ox: i32, oy: i32) {
        self.output = Some((img, ox, oy));
    }

    /// このレイヤーの（エフェクト前の）絵を別の時刻で取得する。
    pub fn frame_at(&self, time: f64) -> Option<Image> {
        let h = self.host()?;
        let mut out = ffi::Image { rgba: std::ptr::null_mut(), width: 0, height: 0 };
        unsafe {
            if (h.frame_at)(h.ctx, time, &mut out) != 0 || out.rgba.is_null() {
                return None;
            }
            Some(Image::from_raw(&out))
        }
    }

    /// LAYER パラメータで選ばれたレイヤーの絵（このバッファと同じ大きさ）。
    pub fn layer_image(&self, param: usize) -> Option<Image> {
        let h = self.host()?;
        let mut out = ffi::Image { rgba: std::ptr::null_mut(), width: 0, height: 0 };
        unsafe {
            if (h.layer_image)(h.ctx, param as u32, &mut out) != 0 || out.rgba.is_null() {
                return None;
            }
            Some(Image::from_raw(&out))
        }
    }

    /// LAYER パラメータで選ばれたレイヤーの音声（モノラル 48kHz、現在時刻を中心に len サンプル）。
    pub fn audio(&self, param: usize, len: usize) -> Option<Vec<f32>> {
        let h = self.host()?;
        let mut out = vec![0.0f32; len];
        unsafe {
            if (h.audio)(h.ctx, param as u32, out.as_mut_ptr(), len as u32) != 0 {
                return None;
            }
        }
        Some(out)
    }

    /// 組み込みエフェクトを掛ける（`kind` は "gaussian_blur" など、`params` は JSON）。
    pub fn builtin(&self, kind: &str, params: &str, img: &mut Image) -> bool {
        let Some(h) = self.host() else { return false };
        let (Ok(k), Ok(p)) = (CString::new(kind), CString::new(params)) else { return false };
        let mut io = ffi::Image { rgba: img.data.as_mut_ptr(), width: img.w as u32, height: img.h as u32 };
        let before = io.rgba;
        unsafe {
            if (h.apply_effect)(h.ctx, k.as_ptr(), p.as_ptr(), &mut io) != 0 {
                return false;
            }
            if io.rgba != before {
                *img = Image::from_raw(&io);
            }
        }
        true
    }

    fn finish(&mut self) -> FxResult {
        let Some((img, ox, oy)) = self.output.take() else { return Ok(()) };
        if img.w == 0 || img.h == 0 {
            return Err("出力が空です");
        }
        let h = self.host().ok_or("ホスト関数がありません")?;
        let mut out = ffi::Image { rgba: std::ptr::null_mut(), width: 0, height: 0 };
        unsafe {
            if (h.set_output)(h.ctx, img.w as u32, img.h as u32, ox, oy, &mut out) != 0 || out.rgba.is_null() {
                return Err("出力の大きさを変えられません");
            }
            std::slice::from_raw_parts_mut(out.rgba, img.w * img.h * 4).copy_from_slice(&img.data);
        }
        Ok(())
    }
}

// ---------- 音声 ----------

/// 音声エフェクトに渡される 1 ブロック。
pub struct Audio<'a> {
    args: &'a AudioArgs,
    /// ステレオのインターリーブ（L, R, ...）。
    pub samples: &'a mut [f32],
}

impl<'a> Audio<'a> {
    /// # Safety
    /// `args` はホストが渡した有効な構造体であること。
    pub unsafe fn from_raw(args: &'a AudioArgs) -> Self {
        Audio { args, samples: std::slice::from_raw_parts_mut(args.samples, args.frames as usize * 2) }
    }
    fn v(&self, i: usize) -> &[f64] {
        if i >= self.args.param_count as usize {
            return &[0.0; 4];
        }
        unsafe { std::slice::from_raw_parts(self.args.values.add(i * 4), 4) }
    }
    pub fn f(&self, i: usize) -> f64 {
        self.v(i)[0]
    }
    pub fn b(&self, i: usize) -> bool {
        self.v(i)[0] != 0.0
    }
    pub fn i(&self, i: usize) -> usize {
        self.v(i)[0].max(0.0) as usize
    }
    pub fn text(&self, i: usize) -> &str {
        if self.args.strings.is_null() || i >= self.args.param_count as usize {
            return "";
        }
        unsafe { cstr(*self.args.strings.add(i)) }
    }
    pub fn frames(&self) -> usize {
        self.args.frames as usize
    }
    pub fn sample_rate(&self) -> f64 {
        self.args.sample_rate as f64
    }
    /// samples[0] の、レイヤーのイン点からのサンプル位置。
    pub fn start(&self) -> i64 {
        self.args.start
    }
    /// samples[0] のコンポ時刻（秒）。
    pub fn time(&self) -> f64 {
        self.args.time
    }
    pub fn instance(&self) -> u64 {
        self.args.instance
    }
    pub fn seed(&self) -> u32 {
        self.args.seed
    }
}

/// 状態を持つ音声処理のための入れ物。インスタンス（レイヤー上のエフェクト）ごとに状態を持ち、
/// ブロックが前回の続きでなければ（シーク・ループ）作り直す。`static` に置ける。
pub struct AudioState<T> {
    map: std::sync::Mutex<Option<std::collections::HashMap<u64, (i64, T)>>>,
}

impl<T> Default for AudioState<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AudioState<T> {
    pub const fn new() -> Self {
        AudioState { map: std::sync::Mutex::new(None) }
    }

    /// このブロックで使う状態を渡す。
    pub fn with<R>(&self, a: &Audio, init: impl Fn() -> T, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.map.lock().unwrap_or_else(|e| e.into_inner());
        let map = guard.get_or_insert_with(Default::default);
        if map.len() > 256 {
            map.clear();
        }
        let entry = map.entry(a.instance()).or_insert_with(|| (a.start(), init()));
        if entry.0 != a.start() {
            entry.1 = init();
        }
        let r = f(&mut entry.1);
        entry.0 = a.start() + a.frames() as i64;
        r
    }
}

// ---------- 登録 ----------

pub enum Handler {
    Video(fn(&mut Frame) -> FxResult),
    Audio(fn(&mut Audio) -> FxResult),
}

/// # Safety
/// ホストから呼ばれる。
pub unsafe fn dispatch_render(handlers: &[Handler], effect: u32, args: *const RenderArgs) -> i32 {
    let (Some(Handler::Video(f)), Some(args)) = (handlers.get(effect as usize), args.as_ref()) else { return 1 };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut frame = Frame::from_raw(args);
        f(&mut frame).and_then(|_| frame.finish())
    }));
    matches!(r, Ok(Ok(()))) as i32 ^ 1
}

/// # Safety
/// ホストから呼ばれる。
pub unsafe fn dispatch_audio(handlers: &[Handler], effect: u32, args: *const AudioArgs) -> i32 {
    let (Some(Handler::Audio(f)), Some(args)) = (handlers.get(effect as usize), args.as_ref()) else { return 1 };
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut Audio::from_raw(args))));
    matches!(r, Ok(Ok(()))) as i32 ^ 1
}

#[doc(hidden)]
#[macro_export]
macro_rules! __handler {
    (render, $f:path) => {
        $crate::Handler::Video($f)
    };
    (audio, $f:path) => {
        $crate::Handler::Audio($f)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __kind_flags {
    (render) => {
        0
    };
    (audio) => {
        $crate::ffi::EFFECT_AUDIO
    };
}

/// プラグインの定義と、ホストが呼ぶ関数（znd_plugin_describe / render / process_audio）を作る。
#[macro_export]
macro_rules! plugin {
    (
        vendor: $vendor:literal, version: $version:literal, license: $license:literal,
        effects: [ $( {
            id: $id:literal, name: $name:literal, category: $cat:literal, flags: $flags:expr,
            params: [ $( $p:expr ),* $(,)? ], $kind:ident : $f:path $(,)?
        } ),+ $(,)? ]
    ) => {
        static __ZND_EFFECTS: $crate::ffi::Shared<&[$crate::ffi::EffectDesc]> = $crate::ffi::Shared(&[ $(
            $crate::ffi::EffectDesc {
                id: concat!($id, "\0").as_ptr() as *const ::std::ffi::c_char,
                name: concat!($name, "\0").as_ptr() as *const ::std::ffi::c_char,
                category: concat!($cat, "\0").as_ptr() as *const ::std::ffi::c_char,
                flags: ($flags) | $crate::__kind_flags!($kind),
                param_count: { const P: &[$crate::ffi::ParamDesc] = &[$($p),*]; P.len() as u32 },
                params: { const P: &[$crate::ffi::ParamDesc] = &[$($p),*]; P.as_ptr() },
            }
        ),+ ]);
        static __ZND_DESC: $crate::ffi::Shared<$crate::ffi::PluginDesc> = $crate::ffi::Shared($crate::ffi::PluginDesc {
            abi_version: $crate::ffi::ABI,
            vendor: concat!($vendor, "\0").as_ptr() as *const ::std::ffi::c_char,
            version: concat!($version, "\0").as_ptr() as *const ::std::ffi::c_char,
            license: concat!($license, "\0").as_ptr() as *const ::std::ffi::c_char,
            effect_count: { const N: &[&str] = &[$($id),+]; N.len() as u32 },
            effects: __ZND_EFFECTS.0.as_ptr(),
        });
        static __ZND_HANDLERS: &[$crate::Handler] = &[ $( $crate::__handler!($kind, $f) ),+ ];

        #[no_mangle]
        pub extern "C" fn znd_plugin_describe() -> *const $crate::ffi::PluginDesc {
            &__ZND_DESC.0
        }

        /// # Safety
        /// ホストから呼ばれる。
        #[no_mangle]
        pub unsafe extern "C" fn znd_plugin_render(effect: u32, args: *const $crate::ffi::RenderArgs) -> i32 {
            $crate::dispatch_render(__ZND_HANDLERS, effect, args)
        }

        /// # Safety
        /// ホストから呼ばれる。
        #[no_mangle]
        pub unsafe extern "C" fn znd_plugin_process_audio(effect: u32, args: *const $crate::ffi::AudioArgs) -> i32 {
            $crate::dispatch_audio(__ZND_HANDLERS, effect, args)
        }
    };
}

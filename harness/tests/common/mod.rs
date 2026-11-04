//! プラグインごとのテストで使う道具。
#![allow(dead_code)]

use std::path::{Path, PathBuf};
pub use znd_core::effect::EffectInstance;
use znd_core::*;
use znd_render::{FrameBuf, NoFootage, RenderOptions, Renderer};

pub fn plugins_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../plugins").canonicalize().unwrap()
}

/// plugins/ のすべてを読み込む（1 回だけ）。
pub fn load() {
    static ONCE: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| {
        let report = znd_render::plugins::scan(&[plugins_dir()]);
        assert!(report.errors.is_empty(), "{:#?}", report.errors);
    });
}

pub fn fx(kind: &str, w: u32, h: u32) -> EffectInstance {
    EffectInstance::new(kind, w as f64, h as f64).unwrap_or_else(|| panic!("{kind} がありません"))
}

/// 白い w×h の平面をコンポ（cw×ch）の中央に置き、エフェクトを掛けたプロジェクト。
pub fn solid_project(w: u32, h: u32, cw: u32, ch: u32, effects: Vec<EffectInstance>) -> Project {
    colored_project(w, h, cw, ch, [1.0, 1.0, 1.0], effects)
}

pub fn colored_project(w: u32, h: u32, cw: u32, ch: u32, color: [f32; 3], effects: Vec<EffectInstance>) -> Project {
    let dur = Time::from_seconds(4.0);
    let mut l = Layer::new("a", "a", LayerKind::Solid { width: w, height: h, color }, Transform::centered(w as f64, h as f64, cw as f64, ch as f64), dur);
    l.effects = effects;
    let mut p = Project::new("t");
    p.compositions.push(Composition { layers: vec![l], ..Composition::new("main", "main", cw, ch, FrameRate::FPS_30, dur) });
    p
}

/// 背景なし（透明）で描く。
pub fn render(p: &Project, seconds: f64) -> FrameBuf {
    render_opts(p, seconds, RenderOptions { downsample: 1, background: false, guides: false })
}

pub fn render_opts(p: &Project, seconds: f64, opts: RenderOptions) -> FrameBuf {
    Renderer::new(p, &NoFootage).render_frame(&CompId::new("main"), Time::from_seconds(seconds), opts).unwrap()
}

pub fn close(a: f32, b: f32, tol: f32, what: &str) {
    assert!((a - b).abs() <= tol, "{what}: {a} != {b}");
}

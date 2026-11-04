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

/// パネルの onEvent を呼んだ結果のプロジェクト（変更が無ければ元のまま）。
pub fn panel_event(p: &Project, package: &str, panel: &str, id: &str, value: serde_json::Value, ctx: serde_json::Value) -> Project {
    load();
    let pan = znd_render::plugins::panels().into_iter().find(|x| x.key == format!("{package}/{panel}")).expect("パネルがありません");
    let wrapper = format!(
        "{}\n;function __event(id, value, state) {{ panel(state); if (typeof onEvent === 'function') onEvent(id, value, state); return state; }}",
        pan.source
    );
    let run = znd_script::run_plugin(p, &wrapper, Some(("__event", &[serde_json::json!(id), value, serde_json::json!({})])), &ctx).unwrap();
    run.project.unwrap_or_else(|| p.clone())
}

/// コマンドを実行した結果のプロジェクト。
pub fn run_command(p: &Project, key: &str, ctx: serde_json::Value) -> (Project, Vec<serde_json::Value>) {
    load();
    let cmd = znd_render::plugins::commands().into_iter().find(|c| c.key == key).expect("コマンドがありません");
    let call = cmd.function.as_deref().map(|f| (f, &[][..]));
    let run = znd_script::run_plugin(p, &cmd.source, call, &ctx).unwrap();
    (run.project.unwrap_or_else(|| p.clone()), run.actions)
}

pub fn context(comp: &str, layers: &[&str]) -> serde_json::Value {
    serde_json::json!({ "comp": comp, "layer": layers.first(), "layers": layers, "frame": 0, "time": 0.0, "fps": 30.0, "plugin": { "id": "test", "dir": "" } })
}

/// エクスプレッション込みで描く。
pub fn render_expr(p: &Project, seconds: f64) -> FrameBuf {
    let engine = znd_script::ExprEngine::new();
    Renderer::new(p, &NoFootage)
        .with_resolver(&engine)
        .render_frame(&CompId::new("main"), Time::from_seconds(seconds), RenderOptions { downsample: 1, background: false, guides: false })
        .unwrap()
}

/// 白い文字のテキストレイヤー（id "t"）をコンポ（cw×ch）の中央に置いたプロジェクト。
pub fn text_project(text: &str, size: f64, cw: u32, ch: u32) -> Project {
    let dur = Time::from_seconds(4.0);
    let doc = znd_core::text::TextDocument::new(text, size);
    let l = Layer::new(
        "t",
        "t",
        LayerKind::Text { document: doc, animators: Vec::new(), path: None, per_char_3d: false },
        Transform::centered(0.0, 0.0, cw as f64, ch as f64),
        dur,
    );
    let mut p = Project::new("t");
    p.compositions.push(Composition { layers: vec![l], ..Composition::new("main", "main", cw, ch, FrameRate::FPS_30, dur) });
    p
}

/// 不透明度の合計（描かれた量の目安）。
pub fn coverage(buf: &FrameBuf) -> f32 {
    buf.data.chunks_exact(4).map(|p| p[3]).sum()
}

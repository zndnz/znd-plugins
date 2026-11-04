//! plugins/ のすべてのパッケージを znd に読み込み、
//! - 安全性スキャンで止められず、警告なく読み込めること
//! - すべてのエフェクトが既定値で描画でき、結果が有限であること（時刻を変えて数回）
//! - 音声エフェクトが無音・音ありの入力で有限の値を返すこと
//! - スクリプト（コマンド・パネル）が構文エラーなく評価できること
//! を確かめる。先に `tools/build.sh debug` でライブラリを置いておくこと（tools/check.sh が両方行う）。

use std::path::{Path, PathBuf};
use znd_core::effect::{effect_def, EffectInstance};
use znd_core::*;
use znd_render::plugins::{self, PackageStatus};
use znd_render::{NoFootage, RenderOptions, Renderer};

fn plugins_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../plugins").canonicalize().unwrap()
}

fn package_dirs() -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(plugins_dir())
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("plugin.json").is_file())
        .collect();
    v.sort();
    v
}

fn load_all() -> Vec<std::sync::Arc<plugins::Package>> {
    static ONCE: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| {
        let report = plugins::scan(&[plugins_dir()]);
        assert!(report.errors.is_empty(), "読み込みエラー: {:#?}", report.errors);
    });
    plugins::packages().into_iter().filter(|p| p.dir.starts_with(plugins_dir())).collect()
}

/// 色の付いた絵（グラデーションの平面＋文字の代わりの図形）を 1 枚持つコンポ。`fx` を掛ける。
fn test_project(fx: EffectInstance) -> Project {
    let dur = Time::from_seconds(4.0);
    let mut bg = Layer::new(
        "bg",
        "bg",
        LayerKind::Solid { width: 96, height: 64, color: [0.8, 0.3, 0.2] },
        Transform::centered(96.0, 64.0, 128.0, 96.0),
        dur,
    );
    bg.effects.push(EffectInstance::new("gradient_ramp", 96.0, 64.0).unwrap());
    bg.effects.push(fx);
    let mut p = Project::new("harness");
    p.compositions.push(Composition { layers: vec![bg], ..Composition::new("main", "main", 128, 96, FrameRate::FPS_30, dur) });
    p
}

#[test]
fn every_package_loads_without_warnings() {
    let dirs = package_dirs();
    let pkgs = load_all();
    for d in &dirs {
        let p = pkgs.iter().find(|p| p.dir.canonicalize().ok().as_deref() == Some(d.as_path()));
        let Some(p) = p else { panic!("{} が読み込まれていません", d.display()) };
        assert_eq!(p.status, PackageStatus::Loaded, "{}: {:?} {:#?}", d.display(), p.status, p.report.findings);
        assert!(p.warnings.is_empty(), "{}: {:?}", d.display(), p.warnings);
        assert!(!p.report.level.blocks(), "{}: 危険度 {:?} {:#?}", d.display(), p.report.level, p.report.findings);
        assert!(!p.manifest.license.is_empty(), "{}: license が空です", d.display());
        assert!(d.join("LICENSE").is_file() || d.join("LICENSE.txt").is_file() || d.join("LICENSE.md").is_file(), "{}: 元のライセンス文がありません", d.display());
    }
}

#[test]
fn every_effect_renders_finite_images() {
    load_all();
    let ours: Vec<_> = plugins::loaded().into_iter().filter(|i| i.path.starts_with(plugins_dir())).collect();
    for info in ours {
        let def = effect_def(&info.kind).unwrap();
        let fx = EffectInstance::new(&info.kind, 96.0, 64.0).unwrap();
        let p = test_project(fx);
        let r = Renderer::new(&p, &NoFootage);
        for t in [0.0, 0.5, 2.0] {
            for ds in [1, 2] {
                let buf = r
                    .render_frame(&CompId::new("main"), Time::from_seconds(t), RenderOptions { downsample: ds, ..RenderOptions::EXPORT })
                    .unwrap_or_else(|e| panic!("{}: {e}", info.kind));
                assert!(buf.data.iter().all(|v| v.is_finite()), "{} ({}) t={t}: 有限でない値", info.kind, def.label);
            }
        }
        if let Some(proc) = znd_core::effect::audio_processor(&info.kind) {
            let fx = EffectInstance::new(&info.kind, 96.0, 64.0).unwrap();
            for start in [0i64, 48000] {
                let mut s: Vec<f32> = (0..512).map(|i| ((i as f32) * 0.05).sin() * 0.5).collect();
                proc.process(
                    &fx.eval(Time::ZERO),
                    znd_core::effect::AudioBlock { samples: &mut s, sample_rate: 48000, start, time: Time::ZERO, instance: 7, seed: 1 },
                );
                assert!(s.iter().all(|v| v.is_finite()), "{}: 音声に有限でない値", info.kind);
            }
        }
    }
}

#[test]
fn every_script_evaluates() {
    let project = znd_core::demo::demo_project();
    let comp = project.compositions[0].id.0.clone();
    let layer = project.compositions[0].layers[0].id.0.clone();
    let ctx = serde_json::json!({ "comp": comp, "layer": layer, "layers": [layer], "frame": 0, "time": 0.0, "fps": 30.0,
        "plugin": { "id": "test", "dir": "" } });
    for p in load_all() {
        for c in &p.commands {
            // 関数を呼ばずにファイル全体を評価する（構文と最上位の処理の確認）
            znd_script::run_plugin(&project, &c.source, None, &ctx).unwrap_or_else(|e| panic!("{}: {e}", c.key));
            if let Some(f) = &c.function {
                assert!(c.source.contains(&format!("function {f}")), "{}: 関数 {f} がありません", c.key);
            }
        }
        for panel in &p.panels {
            let wrapper = format!("{}\n;function __panel(state) {{ return panel(state); }}", panel.source);
            let run = znd_script::run_plugin(&project, &wrapper, Some(("__panel", &[serde_json::json!({})])), &ctx)
                .unwrap_or_else(|e| panic!("{}: {e}", panel.key));
            assert!(run.ret.is_array(), "{}: panel() が配列を返しません", panel.key);
        }
    }
}

mod common;
use common::*;
use znd_core::*;

#[test]
fn timer_counts_only_running_sections() {
    let mut p = text_project("0", 30.0, 300, 80);
    // 0〜1 秒: 走行前、1〜2 秒: 走行中、2〜3 秒: 中断中、3 秒〜: 走行中
    for s in [1.0, 2.0, 3.0] {
        p.compositions[0].layers[0].markers.push(Marker { time: Time::from_seconds(s), duration: Time::ZERO, comment: String::new() });
    }
    let p = panel_event(&p, "SLDC.SeiSa", "seisa", "apply", serde_json::Value::Null, context("main", &["t"]));
    let engine = znd_script::ExprEngine::new();
    let text_at = |s: f64| {
        let c = p.comp(&CompId::new("main")).unwrap();
        let r = znd_render::Resolver::resolve(&engine, &p, c, Time::from_seconds(s)).unwrap().unwrap();
        match &r.layers[0].kind {
            LayerKind::Text { document, .. } => document.text.clone(),
            _ => unreachable!(),
        }
    };
    assert_eq!(text_at(0.5), "0.000");
    assert_eq!(text_at(1.5), "0.500");
    assert_eq!(text_at(2.5), "1.000");
    assert_eq!(text_at(3.5), "1.500");
}

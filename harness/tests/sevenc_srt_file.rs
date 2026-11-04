mod common;
use common::*;
use znd_core::*;

const SRT: &str = "1\r\n00:00:00,500 --> 00:00:01,000\r\nこんにちは\r\n\r\n2\r\n00:00:01,500 --> 00:00:02,000\r\n二行目\r\nです\r\n";

#[test]
fn imports_and_exports_subtitles() {
    load();
    let p = text_project("ひな形", 40.0, 300, 200);
    let cmd = znd_render::plugins::commands().into_iter().find(|c| c.key == "sevenc-nanashi.aviutl2-rs.rusty_srt_file/import").unwrap();
    let args = [serde_json::json!(SRT), serde_json::json!("a.srt"), serde_json::json!({ "comp": "main", "layer": "t" })];
    let run = znd_script::run_plugin(&p, &cmd.source, Some(("onSrt", &args)), &context("main", &["t"])).unwrap();
    let p = run.project.unwrap();
    let ls = &p.compositions[0].layers;
    assert_eq!(ls.len(), 2, "ひな形は消えて字幕が 2 つ");
    let LayerKind::Text { document, .. } = &ls[1].kind else { unreachable!() };
    assert_eq!(document.text, "二行目\nです");
    assert!((ls[0].in_point.seconds() - 0.5).abs() < 1e-6 && (ls[0].out_point.seconds() - 1.0).abs() < 1e-6);
    // 書き出し
    let (_, actions) = run_command(&p, "sevenc-nanashi.aviutl2-rs.rusty_srt_file/export", context("main", &[]));
    let s = actions.iter().find_map(|a| a.get("content").and_then(|c| c.as_str()).map(String::from)).expect("保存の動作");
    assert!(s.contains("00:00:00,500 --> 00:00:01,000\nこんにちは"), "{s}");
    assert!(s.contains("2\n00:00:01,500 --> 00:00:02,000\n二行目\nです"), "{s}");
}

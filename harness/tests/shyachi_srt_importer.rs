mod common;
use common::*;
use znd_core::*;

#[test]
fn imports_subtitles_with_settings() {
    load();
    let p = solid_project(10, 10, 640, 360, vec![]);
    let pan = znd_render::plugins::panels().into_iter().find(|x| x.key == "shyachi.SrtImporter/srt_importer").unwrap();
    let srt = "1\r\n00:00:00,500 --> 00:00:01,000\r\nA\r\n\r\n2\r\n00:00:01,500 --> 00:00:02,000\r\nB\r\n";
    let args = [serde_json::json!(srt), serde_json::json!("a.srt"), serde_json::json!({ "comp": "main", "time": 1.0, "s": { "y": 100, "size": 30, "outline": true } })];
    let run = znd_script::run_plugin(&p, &pan.source, Some(("onSrt", &args)), &context("main", &[])).unwrap();
    let p = run.project.unwrap();
    let ls = &p.compositions[0].layers;
    assert_eq!(ls.len(), 3);
    let LayerKind::Text { document, .. } = &ls[0].kind else { panic!() };
    assert_eq!((document.text.as_str(), document.size), ("A", 30.0));
    assert!(document.stroke.is_some());
    assert!((ls[0].in_point.seconds() - 1.5).abs() < 1e-6, "再生ヘッドの 1 秒から");
    assert!((ls[0].transform.position.value[1] - 280.0).abs() < 1e-6);
    assert!(p.validate().is_empty());
}

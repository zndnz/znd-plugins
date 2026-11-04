mod common;
use common::*;
use znd_core::*;

#[test]
fn makes_text_from_audio_file_name() {
    let mut p = solid_project(10, 10, 100, 100, vec![]);
    let fid = FootageId::new("snd");
    let mut item = p.compositions[0].layers[0].clone();
    item.id = LayerId::new("v");
    item.name = "v".into();
    item.kind = LayerKind::Audio { footage: fid.clone() };
    item.in_point = Time::from_seconds(1.0);
    item.out_point = Time::from_seconds(2.5);
    p.compositions[0].layers.push(item);
    let mut f: FootageItem = serde_json::from_value(serde_json::json!({ "id": "snd", "name": "a", "path": "/voice/zunda_こんにちは.wav", "width": 0, "height": 0 })).unwrap();
    f.duration = Some(Time::from_seconds(1.5));
    p.footage.push(f);
    let (p, _) = run_command(&p, "KuroNeko.audio2obj/make_id", context("main", &["v"]));
    let ls = &p.compositions[0].layers;
    let t = ls.iter().find(|l| matches!(l.kind, LayerKind::Text { .. })).expect("テキストができる");
    let LayerKind::Text { document, .. } = &t.kind else { unreachable!() };
    assert_eq!(document.text, "こんにちは");
    assert_eq!((t.in_point.seconds(), t.out_point.seconds()), (1.0, 2.5));
}

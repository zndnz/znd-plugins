mod common;
use common::*;
use znd_core::*;

#[test]
fn splits_into_one_layer_per_character() {
    load();
    let p = text_project("AB C", 40.0, 300, 200);
    let whole = render_expr(&p, 0.0);
    let (p2, _) = run_command(&p, "Natadecoco.TextSplitter/split", context("main", &["t"]));
    let ls = &p2.compositions[0].layers;
    assert_eq!(ls.len(), 3, "空白を除いた 3 文字");
    assert!(ls.iter().all(|l| matches!(l.kind, LayerKind::Text { .. })));
    assert!(p2.validate().is_empty());
    // 3 つ重ねた絵は元と同じ
    let split = render_expr(&p2, 0.0);
    let diff = (0..200).flat_map(|y| (0..300).map(move |x| (x, y))).map(|(x, y)| (whole.pixel(x, y)[3] - split.pixel(x, y)[3]).abs()).fold(0.0f32, f32::max);
    assert!(diff < 0.02, "{diff}");
}

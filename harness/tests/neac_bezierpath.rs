mod common;
use common::*;
use znd_core::*;

#[test]
fn layers_follow_the_curve() {
    let mut p = solid_project(8, 8, 200, 200, vec![]);
    for k in 1..3 {
        let mut l = p.compositions[0].layers[0].clone();
        l.id = LayerId::new(&format!("a{k}"));
        l.name = format!("a{k}");
        p.compositions[0].layers.push(l);
    }
    let (p, _) = run_command(&p, "NeAc.BezierPath/arrange", context("main", &["a", "a1", "a2"]));
    assert!(p.validate().is_empty(), "{:?}", p.validate());
    // ガイドを消して、並んだ 3 つだけを見る
    let mut q = p.clone();
    let fx = &mut q.compositions[0].layers[0].effects[0];
    fx.params.insert("guide".into(), znd_core::effect::ParamValue::Bool(false));
    // オフセット 0 では最後の 1 つだけが曲線の始点に乗る（元と同じ）。100 にすると 3 つとも曲線の上に並ぶ
    fx.params.insert("offset".into(), znd_core::effect::ParamValue::Scalar(znd_core::property::Property::constant(100.0)));
    let buf = render_expr(&q, 0.0);
    let drawn = buf.data.chunks_exact(4).filter(|px| px[3] > 0.5).count();
    assert!(drawn >= 3 * 40, "3 つとも描かれている: {drawn}");
}

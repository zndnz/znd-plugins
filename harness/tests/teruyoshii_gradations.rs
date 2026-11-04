mod common;
use common::*;
use znd_core::effect::ParamValue;
use znd_core::*;

/// 「グラデ曲線」に 2 つのキーフレームを打つ。
fn with_curve(fx: EffectInstance, a: f64, b: f64) -> EffectInstance {
    let mut p = Property::constant(a);
    p.keyframes = vec![Keyframe::linear(Time::ZERO, a), Keyframe::linear(Time::from_seconds(4.0), b)];
    let mut fx = fx.set_b("use_curve", true);
    fx.params.insert("curve".into(), ParamValue::Scalar(p));
    fx
}

#[test]
fn noise_gradation_follows_keyframed_curve() {
    load();
    let base = fx("plugin:teruyoshii.noiseGradation", 40, 40).set_f("width", 400.0);
    // 曲線がずっと 100 なら全部終了色（青）、ずっと 0 なら全部開始色（赤）
    for (v, blue) in [(100.0, true), (0.0, false)] {
        let buf = render(&solid_project(40, 40, 100, 100, vec![with_curve(base.clone(), v, v)]), 1.0);
        for (x, y) in [(35, 35), (50, 50), (65, 62)] {
            let p = buf.pixel(x, y);
            assert_eq!(p[2] > p[0], blue, "曲線 {v}: {p:?}");
        }
    }
    // 曲線なし（直線）なら両方の色が混ざる
    let buf = render(&solid_project(40, 40, 100, 100, vec![base]), 1.0);
    let blues = (31..69).flat_map(|y| (31..69).map(move |x| (x, y))).filter(|&(x, y)| buf.pixel(x, y)[2] > buf.pixel(x, y)[0]).count();
    assert!(blues > 200 && blues < 38 * 38 - 200, "{blues}");
}

#[test]
fn step_gradation_and_four_corners_render() {
    load();
    let buf = render(&solid_project(40, 40, 100, 100, vec![with_curve(fx("plugin:teruyoshii.stepGradation", 40, 40), 0.0, 100.0)]), 2.0);
    assert!(buf.pixel(50, 50)[3] > 0.99);
    // 四隅: 100x100 のコンポで余白 20 → 左上の隅は (20,20) あたり
    let buf = render(&solid_project(20, 20, 100, 100, vec![fx("plugin:teruyoshii.4corner", 20, 20).set_f("mx", 20.0).set_f("my", 20.0)]), 0.0);
    assert!(buf.pixel(20, 20)[3] > 0.99 && buf.pixel(80, 80)[3] > 0.99, "四隅に置かれる");
    assert!(buf.pixel(50, 50)[3] < 0.01, "中心には無い");
}

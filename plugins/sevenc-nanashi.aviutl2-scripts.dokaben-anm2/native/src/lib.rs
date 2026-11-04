//! ドカベン（Nanashi.）の移植: 絵が手前に倒れた状態から起き上がる（また倒れる）、ドカベンの文字の動き。
//! 縦に分けた帯を台形に変形して描く（式は元と同じ）。

use std::f64::consts::PI;
use znd_plugin_sdk::draw::textured_quad;
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let speed = f.f(0);
    let auto_fall = f.b(1);
    let upright = f.f(2);
    let yo = f.f(3) * f.scale();
    let drop = f.b(4);
    let fps = f.fps().max(1.0);
    let mut t = if speed == 0.0 { PI / 2.0 } else { f.layer_time() * speed / 100.0 };
    if drop {
        t -= t.rem_euclid(0.12);
    }
    if auto_fall {
        t = t.rem_euclid(PI + upright / fps);
    } else {
        t = t.min(PI / 2.0);
    }
    if t > PI / 2.0 {
        t = (PI / 2.0).min(PI + upright / fps - t);
    }
    let (s, c) = (0.5 - t.sin(), 0.5 - t.cos() / 8.0);
    let src = f.input();
    let (w, h) = (src.w as f64, src.h as f64);
    let n = ((w / 32.0).floor() as usize).clamp(1, 32);
    let mut out = Image::new(src.w, src.h);
    let o = V2::new(w / 2.0, h / 2.0);
    for i in 0..n {
        let (l, r) = (i as f64 / n as f64, (i + 1) as f64 / n as f64);
        let top = (h - yo) * s - yo / 2.0;
        let bottom = h / 2.0 - yo * (0.5 + s);
        let d = [
            V2::new((l * 2.0 - 1.0) * w * c, top),
            V2::new((r * 2.0 - 1.0) * w * c, top),
            V2::new((r - 0.5) * w, bottom),
            V2::new((l - 0.5) * w, bottom),
        ]
        .map(|p| p.add(o));
        textured_quad(&mut out, &src, d, [V2::new(l * w, 0.0), V2::new(r * w, 0.0), V2::new(r * w, h), V2::new(l * w, h)], 1.0);
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-scripts.dokaben-anm2", name: "ドカベン", category: "アニメーション", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"speed", c"再生速度", 100.0, 0.0, 800.0),
                boolean(c"auto_fall", c"自動で倒れる", false),
                float(c"upright", c"直立時間", -1.0, 0.0, 800.0),
                float(c"y_offset", c"Yオフセット", 0.0, 0.0, 100.0),
                boolean(c"drop_frame", c"コマ落ち", true),
            ],
            render: render,
        },
    ]
}

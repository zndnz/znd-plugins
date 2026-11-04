//! OpticsCompensation_s（Sagyou）の移植: レンズの樽型・糸巻き型の歪み（After Effects の「光学補正」に相当）。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let amount = f.f(0) / 100.0;
    if amount == 0.0 {
        return Ok(());
    }
    let barrel = amount > 0.0;
    let a = amount.abs().min(0.999);
    // 焦点距離はレイヤー座標で 500px を基準にする（プレビュー解像度に合わせて縮める）
    let focal = if a > 1e-4 { 500.0 / (0.5 * a * std::f64::consts::PI).tan() * f.scale() } else { 1e6 };
    let c = f.point(1);
    let aa = f.b(2) && barrel;
    let transparent = f.b(3);
    let src = f.input();
    let calc = |px: f64, py: f64| -> (f32, f32) {
        let (rx, ry) = (px - c[0], py - c[1]);
        let d = rx.hypot(ry);
        if d < 1e-4 {
            return (px as f32, py as f32);
        }
        let k = if barrel {
            focal * (d / focal).clamp(-1.570796, 1.570796).tan()
        } else {
            focal * (d / focal).atan()
        };
        ((rx / d * k + c[0]) as f32, (ry / d * k + c[1]) as f32)
    };
    let get = |p: (f32, f32)| if transparent { src.sample(p.0, p.1) } else { src.sample_clamped(p.0, p.1) };
    let mut out = Image::new(src.w, src.h);
    out.map(|x, y, _| {
        let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
        if aa {
            let mut acc = [0.0f32; 4];
            for (dx, dy) in [(-0.25, -0.25), (0.25, -0.25), (-0.25, 0.25), (0.25, 0.25)] {
                let s = get(calc(px + dx, py + dy));
                for k in 0..4 {
                    acc[k] += s[k] * 0.25;
                }
            }
            acc
        } else {
            get(calc(px, py))
        }
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "Sagyou（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "Sagyou.OpticsCompensation_s", name: "光学補正（OpticsCompensation_s）", category: "ディストーション", flags: THREAD_SAFE,
            params: [
                float(c"amount", c"強さ", 0.0, -100.0, 100.0),
                point(c"center", c"中心", 0.5, 0.5),
                boolean(c"use_aa", c"AA（樽型時）", true),
                boolean(c"transparent", c"範囲外を透明にする", true),
            ],
            render: render,
        },
    ]
}

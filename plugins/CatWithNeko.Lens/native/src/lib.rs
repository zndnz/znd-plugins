//! Lens（こんにゃくねこ）の移植: 中心からの距離のべき乗で歪ませるレンズディストーション。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let d = f.f(0);
    let off = [f.f(1) * f.scale(), f.f(2) * f.scale()];
    let p = f.f(3);
    let lim = f.f(4);
    let (w, h) = (f.width as f64, f.height as f64);
    let src = f.input();
    let zoom = if d < lim { 1.0 + (-d + lim).max(0.0).powf(1.4) } else { 1.0 };
    let mut out = Image::new(f.width, f.height);
    for y in 0..f.height {
        for x in 0..f.width {
            let c = [x as f64 + 0.5 - w / 2.0 - off[0], y as f64 + 0.5 - h / 2.0 - off[1]];
            let r = (c[0] / w).hypot(c[1] / h);
            let denom = (1.0 - r.powf(p) * d.max(lim) * 10.0).max(0.001);
            let tx = c[0] / denom / zoom + w / 2.0 + off[0];
            let ty = c[1] / denom / zoom + h / 2.0 + off[1];
            out.set(x, y, src.get(tx.floor() as isize, ty.floor() as isize));
        }
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Lens", name: "Lens", category: "変形", flags: THREAD_SAFE,
            params: [
                float(c"d", c"Distortion", 0.0, -10.0, 10.0),
                float(c"cx", c"X", 0.0, -4000.0, 4000.0),
                float(c"cy", c"Y", 0.0, -4000.0, 4000.0),
                float(c"pow", c"Pow", 2.0, 0.1, 10.0),
                float(c"lim", c"Limit", -0.5, -10.0, 0.0),
            ],
            render: render,
        },
    ]
}

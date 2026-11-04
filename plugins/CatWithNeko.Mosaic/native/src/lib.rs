//! Mosaic_C（こんにゃくねこ）の移植: 中心から升目に区切り、升目の中心の色で塗るシャープなモザイク。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let size = f.f(0).max(1.0) * f.scale();
    let (sx, sy) = ((size * f.f(1)).max(1e-3), (size * f.f(2)).max(1e-3));
    let (w, h) = (f.width as f64, f.height as f64);
    let src = f.input();
    let mut out = Image::new(f.width, f.height);
    for y in 0..f.height {
        let py = y as f64 + 0.5;
        let cy = (((py - h / 2.0) / sy).floor() * sy + h / 2.0 + sy / 2.0).clamp(sy / 2.0, (h - sy / 2.0).max(sy / 2.0));
        for x in 0..f.width {
            let px = x as f64 + 0.5;
            let cx = (((px - w / 2.0) / sx).floor() * sx + w / 2.0 + sx / 2.0).clamp(sx / 2.0, (w - sx / 2.0).max(sx / 2.0));
            out.set(x, y, src.sample_clamped(cx as f32, cy as f32));
        }
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Mosaic", name: "Mosaic_C", category: "加工", flags: THREAD_SAFE,
            params: [
                float(c"size", c"Size", 1.0, 1.0, 4000.0),
                float(c"size_x", c"sizeX", 1.0, 0.001, 1.0),
                float(c"size_y", c"sizeY", 1.0, 0.001, 1.0),
            ],
            render: render,
        },
    ]
}

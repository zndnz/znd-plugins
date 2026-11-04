//! Chromatic Aberration（こんにゃくねこ）の移植: 1 色を残し、残りの 2 色を逆向きにずらす簡易色収差。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let off = [f.f(0) * sc, f.f(1) * sc];
    let center = [f.f(2) * sc, f.f(3) * sc];
    let zoom = (f.f(4) / 100.0).max(1e-6);
    let channel = f.i(5).min(2);
    let bi = f.b(6);
    let (w, h) = (f.width as f64, f.height as f64);
    let src = f.input();
    let mut out = Image::new(f.width, f.height);
    for y in 0..f.height {
        for x in 0..f.width {
            let uv = [x as f64 + 0.5 - w / 2.0, y as f64 + 0.5 - h / 2.0];
            let r1 = [0, 1].map(|k| (uv[k] - center[k]) / zoom + center[k]);
            let r2 = [0, 1].map(|k| if bi { (uv[k] - center[k]) * zoom + center[k] } else { (uv[k] - center[k]) / ((zoom - 1.0) / 2.0 + 1.0) + center[k] });
            let col = src.px(x, y);
            let cp = src.get((r1[0] + w / 2.0 - off[0]).floor() as isize, (r1[1] + h / 2.0 - off[1]).floor() as isize);
            let cm = src.get((r2[0] + w / 2.0 + off[0]).floor() as isize, (r2[1] + h / 2.0 + off[1]).floor() as isize);
            let rgb = match channel {
                0 => [col[0], cp[1], cm[2]],
                1 => [cp[0], col[1], cm[2]],
                _ => [cp[0], cm[1], col[2]],
            };
            let a = col[3];
            out.set(x, y, [rgb[0].min(a), rgb[1].min(a), rgb[2].min(a), a]);
        }
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.ChromaticAberration", name: "Chromatic Aberration", category: "加工", flags: THREAD_SAFE,
            params: [
                float(c"x", c"X", 0.0, -4000.0, 4000.0),
                float(c"y", c"Y", 0.0, -4000.0, 4000.0),
                float(c"cx", c"CenterX", 0.0, -4000.0, 4000.0),
                float(c"cy", c"CenterY", 0.0, -4000.0, 4000.0),
                float(c"zoom", c"Zoom", 100.0, 0.0, 200.0),
                choice(c"channel", c"Channel", 1, c"R|G|B"),
                boolean(c"bi", c"Bidrectional", false),
            ],
            render: render,
        },
    ]
}

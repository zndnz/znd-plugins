//! ポイントズーム（azurite）の移植: 指定した点を中心に拡大・縮小する。領域外は指定色・引き伸ばし・ループ・ミラー。

use znd_plugin_sdk::*;

fn mirror(v: f32, n: f32) -> f32 {
    let p = v.rem_euclid(2.0 * n);
    if p < n {
        p
    } else {
        2.0 * n - p
    }
}

fn render(f: &mut Frame) -> FxResult {
    let mode = f.i(3);
    let mut zoom = (f.f(0) / 100.0).clamp(1e-7, 100.0);
    if mode >= 2 {
        zoom = zoom.max(1.0);
    }
    let nearest = f.b(1);
    let target = f.point(2);
    let bg_color = color::premul(f.color(4), 1.0);
    let a = (1.0 - f.f(5) / 100.0).clamp(0.0, 1.0) as f32;
    let src = f.input();
    let (w, h) = (src.w as f32, src.h as f32);
    let (cx, cy) = (target[0] as f32, target[1] as f32);
    let z = zoom as f32;
    let mut out = Image::new(src.w, src.h);
    out.map(|x, y, _| {
        let (sx, sy) = (cx + (x as f32 + 0.5 - cx) / z, cy + (y as f32 + 0.5 - cy) / z);
        let top = if nearest { src.get(sx.floor() as isize, sy.floor() as isize) } else { src.sample(sx, sy) };
        let bg = match mode {
            0 => bg_color.map(|v| v * (1.0 - top[3])),
            1 => src.sample_clamped(sx, sy),
            2 => src.sample_wrapped(sx, sy),
            _ => src.sample(mirror(sx, w).clamp(0.5, w - 0.5), mirror(sy, h).clamp(0.5, h - 0.5)),
        };
        let mut o = [0.0; 4];
        for k in 0..4 {
            o[k] = top[k] + bg[k] * a * (1.0 - top[3]);
        }
        o
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "azurite（znd 移植: znd-plugins）", version: "2.0.1", license: "MIT",
    effects: [
        {
            id: "azurite.PointZoom", name: "ポイントズーム", category: "ディストーション", flags: THREAD_SAFE,
            params: [
                float(c"zoom", c"拡大率", 100.0, 0.0, 10000.0),
                boolean(c"no_interp", c"補間なし", false),
                point(c"target", c"目標座標", 0.5, 0.5),
                choice(c"sampler", c"領域外", 0, c"指定色|引き伸ばす|ループ|ミラー"),
                color_hex(c"color", c"色", 0xffffff),
                float(c"transparency", c"透明度", 0.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}

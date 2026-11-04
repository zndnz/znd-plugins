//! Halftone（こんにゃくねこ）の移植: 升目ごとに、暗いほど大きい円を描くなんちゃってハーフトーン。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let size = (f.f(0).max(2.0) * f.scale()).max(1.0);
    let st = f.f(1);
    let blur = f.f(2) * f.scale();
    let ca = f.color(3);
    let cb = f.color(4);
    let blend = f.b(5);
    let mode = f.f(6).round().clamp(0.0, 9.0) as usize;
    let mix = f.f(7) as f32;
    let (w, h) = (f.width, f.height);
    let org = f.input();
    let mut src = org.clone();
    if blur > 0.0 {
        aul::blur(&mut src, blur as f32);
    }
    let mut out = Image::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
            let cx = ((px / size).floor() * size + size / 2.0).clamp(0.0, w as f64 - 1.0);
            let cy = ((py / size).floor() * size + size / 2.0).clamp(0.0, h as f64 - 1.0);
            let d = (px - cx).hypot(py - cy);
            let s = color::to_display(src.px(cx as usize, cy as usize));
            let lum = 0.2126 * s[0] as f64 + 0.7152 * s[1] as f64 + 0.0722 * s[2] as f64;
            let r = (1.0 - lum) * (size / 2.0) * st;
            let c = if d > r { ca } else { cb };
            out.set(x, y, color::premul(c, org.px(x, y)[3]));
        }
    }
    if blend {
        let mut base = org;
        aul::blend(&mut base, &out, mode, mix);
        out = base;
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Halftone", name: "Halftone", category: "色調整", flags: THREAD_SAFE,
            params: [
                float(c"size", c"Size", 10.0, 2.0, 100.0),
                float(c"scale", c"Scale", 1.0, 0.0, 5.0),
                float(c"blur", c"Blur", 0.0, 0.0, 100.0),
                color_hex(c"ca", c"Color1", 0xffffff),
                color_hex(c"cb", c"Color2", 0x000000),
                boolean(c"blend", c"Blend", true),
                float(c"mode", c"BlendMode", 0.0, 0.0, 9.0),
                float(c"mix", c"Mix", 1.0, 0.0, 1.0),
            ],
            render: render,
        },
    ]
}

//! SelfDisplace（こんにゃくねこ）の移植: 自分自身の明るさを地図にして、色ごとにずらす・回す・拡大するディスプレイスメント。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let off = [f.f(0) * sc, f.f(1) * sc];
    let rad = f.f(2).to_radians();
    let zoom = f.f(3) / 100.0;
    let zoom2 = [1.0 + f.f(4) * (zoom - 1.0), 1.0 + (zoom - 1.0) * f.f(5)];
    let blur = f.f(6) * sc;
    let rel = [f.f(7), f.f(8), f.f(9), 1.0];
    let wts = match f.i(10) {
        1 => [0.2126, 0.7152, 0.0722],
        2 => [0.2627, 0.6780, 0.0593],
        3 => [0.3333, 0.3333, 0.3333],
        _ => [0.299, 0.587, 0.114],
    };
    let inv = f.b(11);
    let bi = f.b(12);
    let org = f.input();
    let mut map = org.clone();
    if blur > 0.0 {
        aul::blur(&mut map, blur as f32);
    }
    let (w, h) = (org.w as f64, org.h as f64);
    let aspect = w / h;
    let disp = |uv: [f64; 2], d: f64| -> [f32; 2] {
        let mut c = [uv[0] - 0.5, uv[1] - 0.5];
        c[0] *= aspect;
        let (s, co) = (-rad * d).sin_cos();
        let mut r = [c[0] * co - c[1] * s, c[0] * s + c[1] * co];
        r[0] /= aspect;
        let z = [r[0] / (1.0 + (zoom2[0] - 1.0) * d), r[1] / (1.0 + (zoom2[1] - 1.0) * d)];
        let fu = [z[0] + off[0] * d / w + 0.5, z[1] + off[1] * d / h + 0.5];
        [(fu[0] * w) as f32, (fu[1] * h) as f32]
    };
    let mut out = Image::new(org.w, org.h);
    for y in 0..org.h {
        for x in 0..org.w {
            let m = color::to_display(map.px(x, y));
            let mut lum = (m[0] as f64 * wts[0] + m[1] as f64 * wts[1] + m[2] as f64 * wts[2]) * m[3] as f64;
            if inv {
                // 元の「反転（輝度反転）」は色だけを反転する
                lum = (1.0 - m[0] as f64) * wts[0] + (1.0 - m[1] as f64) * wts[1] + (1.0 - m[2] as f64) * wts[2];
                lum *= m[3] as f64;
            }
            let d = if bi { lum * 2.0 - 1.0 } else { lum };
            let uv = [(x as f64 + 0.5) / w, (y as f64 + 0.5) / h];
            let mut o = [0.0f32; 4];
            for k in 0..4 {
                let p = disp(uv, rel[k] * d);
                o[k] = org.sample_clamped(p[0], p[1])[k];
            }
            let a = o[3];
            out.set(x, y, [o[0].min(a), o[1].min(a), o[2].min(a), a]);
        }
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.SelfDisplace", name: "SelfDisplace", category: "変形", flags: THREAD_SAFE,
            params: [
                float(c"amt_x", c"Off X", 0.0, -4000.0, 4000.0),
                float(c"amt_y", c"Off Y", 0.0, -4000.0, 4000.0),
                float(c"rot", c"Rotate", 0.0, -4000.0, 4000.0),
                float(c"zoom", c"Zoom", 100.0, 0.0, 4000.0),
                float(c"zoom_x", c"Zoom X", 1.0, 0.0, 1.0),
                float(c"zoom_y", c"Zoom Y", 1.0, 0.0, 1.0),
                float(c"blur", c"Blur", 15.0, 0.0, 1000.0),
                float(c"rel_r", c"Rel R", 0.5, 0.0, 1.0),
                float(c"rel_g", c"Rel G", 0.75, 0.0, 1.0),
                float(c"rel_b", c"Rel B", 1.0, 0.0, 1.0),
                choice(c"mode", c"Mode", 0, c"Luminance (BT.601)|Luminance (BT.709)|Luminance (BT.2020)|Mean"),
                boolean(c"inv", c"Invert", false),
                boolean(c"bi", c"Bidirectional", false),
            ],
            render: render,
        },
    ]
}

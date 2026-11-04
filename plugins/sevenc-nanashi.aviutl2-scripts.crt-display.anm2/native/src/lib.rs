//! CRTディスプレイ風（Nanashi.）の移植: 走査線の明滅・RGB の縦縞・横のにじみで、ブラウン管の画面のようにする。
//! 計算は元のシェーダーと同じ（色は表示値。結果は不透明）。

use znd_plugin_sdk::*;

fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let hblur = f.f(0);
    let rgb_eff = f.f(1);
    let ps = (f.f(2) * sc).max(1.0);
    let decay = f.f(3) * 0.3;
    let slow = f.f(4).max(0.1);
    let progressive = f.b(5);
    let t = f.time();
    let (w, h) = (f.width, f.height);
    let src = f.input();
    let px = |x: f64, y: f64| -> [f64; 3] {
        let p = color::to_display(src.get(x.floor() as isize, y.floor() as isize));
        // 元のシェーダーは乗算済みのまま rgb を読む
        [p[0] as f64 * p[3] as f64, p[1] as f64 * p[3] as f64, p[2] as f64 * p[3] as f64]
    };
    let dot = ps / 6.0;
    let mut out = Image::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let (lx, ly) = (x as f64 + 0.5, y as f64 + 0.5);
            let pc = [(lx / ps).floor(), (ly / ps).floor()];
            let pp = [(pc[0] + 0.5) * ps, (pc[1] + 0.5) * ps];
            let mut o = if hblur > 0.0 {
                let wts = [0.3, 0.2, 0.1, 0.05, 0.01];
                let mut s = px(pp[0], pp[1]).map(|v| v * wts[0]);
                for (i, wt) in wts.iter().enumerate().skip(1) {
                    let d = i as f64 * ps * hblur;
                    let (a, b) = (px(pp[0] + d, pp[1]), px(pp[0] - d, pp[1]));
                    for k in 0..3 {
                        s[k] += (a[k] + b[k]) * wt;
                    }
                }
                s
            } else {
                px(lx, ly)
            };
            if rgb_eff > 0.0 {
                let n = (lx / dot).floor() % 6.0 % 3.0;
                let r = 1.0 - (rgb_eff / 100.0).clamp(0.0, 1.0);
                let m = if n < 1.0 { [1.0, r, r] } else if n < 2.0 { [r, 1.0, r] } else { [r, r, 1.0] };
                for k in 0..3 {
                    o[k] *= m[k];
                }
            }
            if decay > 0.0 {
                let mut dur = 1.0 / 60.0;
                let all_lines = (h as f64 / ps).ceil();
                let all_px = (w as f64 / ps).floor().max(1.0);
                let line_t = dur / all_lines;
                let px_t = line_t / all_px;
                dur += line_t;
                let order = if progressive { pc[1] } else {
                    let no = (pc[1] / 2.0).floor();
                    if pc[1] % 2.0 < 0.5 { no } else { (all_lines / 2.0).ceil() + no }
                };
                let mut end = (t / slow) % dur - order * line_t;
                if end < 0.0 {
                    end += dur;
                }
                let pend = end - pc[0] * px_t;
                let remap = line_t + (dur - line_t) * decay.clamp(0.0, 1.0);
                let bright = if pend < 0.0 { 0.0 } else { (1.0 - smoothstep(0.0, remap, pend)).powi(4) };
                let long = 10.0 * (1.0 - smoothstep(2.0, 3.0, decay));
                let k = 1.0 - (1.0 - bright) * long * 0.1;
                o = o.map(|v| v * k);
                let flash = px_t * 5.0;
                if (0.0..flash).contains(&pend) {
                    let fa = (1.0 - pend / flash).powi(2);
                    o = o.map(|v| v + (1.0 - v) * fa);
                }
            }
            if ps > 1.0 {
                let on_x = lx % dot < ps / 30.0;
                let on_y = ly % (ps / 8.0) < ps / 200.0;
                let bit = (ly / ps * 8.0).floor() % 8.0;
                let k = if bit == 0.0 || bit == 7.0 { 0.4 } else if bit == 1.0 || bit == 6.0 { 0.7 } else { 1.0 };
                o = o.map(|v| v * k);
                if on_x || on_y {
                    let s = smoothstep(1.0, 8.0, ps);
                    o = o.map(|v| v * (1.0 - s));
                }
            }
            out.set(x, y, color::from_display([o[0] as f32, o[1] as f32, o[2] as f32, 1.0].map(|v| v.clamp(0.0, 1.0))));
        }
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-scripts.crt-display.anm2", name: "CRTディスプレイ風", category: "加工", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"horizontal_blur", c"水平ブラー", 0.5, 0.0, 1.0),
                float(c"rgb_percent", c"RGB%", 100.0, 0.0, 100.0),
                float(c"pixel_size", c"ピクセルサイズ", 1.5, 1.0, 50.0),
                float(c"decay", c"減衰", 9.0, 0.01, 10.0),
                float(c"slowmotion", c"スローモーション", 1.0, 0.1, 1000.0),
                boolean(c"use_progressive", c"プログレッシブ", false),
            ],
            render: render,
        },
    ]
}

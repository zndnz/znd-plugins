//! Open Glow（絵に描いた虹）の移植: 縮小ピラミッドを重ねて、距離のべき乗で減衰する光を作るグロー。
//! 手順は元と同じ（前処理 → 1/2 縮小をくり返す → テントで拡大しながら重みを付けて足す → 合成）。

use znd_plugin_sdk::*;

/// 乗算済みの表示値のまま扱う 4 チャンネルの浮動小数の画像（元のバッファと同じ意味）。
type Buf = Image;

fn down(src: &Buf) -> Buf {
    let (w, h) = (src.w.div_ceil(2).max(1), src.h.div_ceil(2).max(1));
    // 13 タップ（Jimenez）。座標は元の画素単位
    let taps: [(f32, f32, f32); 13] = [
        (0.125, 0.0, 0.0), (0.125, -1.0, -1.0), (0.125, 1.0, -1.0), (0.125, -1.0, 1.0), (0.125, 1.0, 1.0),
        (0.0625, 0.0, -2.0), (0.0625, -2.0, 0.0), (0.0625, 2.0, 0.0), (0.0625, 0.0, 2.0),
        (0.03125, -2.0, -2.0), (0.03125, 2.0, -2.0), (0.03125, -2.0, 2.0), (0.03125, 2.0, 2.0),
    ];
    let (sx, sy) = (src.w as f32 / w as f32, src.h as f32 / h as f32);
    Image::generate(w, h, |x, y| {
        let (cx, cy) = ((x as f32 + 0.5) * sx, (y as f32 + 0.5) * sy);
        let mut s = [0.0f32; 4];
        for &(k, dx, dy) in &taps {
            let p = src.sample(cx + dx, cy + dy);
            for c in 0..4 {
                s[c] += k * p[c];
            }
        }
        s
    })
}

fn up_add(low: &Buf, cur: &mut Buf, wl: f32) {
    let (sx, sy) = (low.w as f32 / cur.w as f32, low.h as f32 / cur.h as f32);
    let taps: [(f32, f32, f32); 9] = [
        (4.0, 0.0, 0.0), (2.0, -1.0, 0.0), (2.0, 1.0, 0.0), (2.0, 0.0, -1.0), (2.0, 0.0, 1.0),
        (1.0, -1.0, -1.0), (1.0, 1.0, -1.0), (1.0, -1.0, 1.0), (1.0, 1.0, 1.0),
    ];
    for y in 0..cur.h {
        for x in 0..cur.w {
            let (cx, cy) = ((x as f32 + 0.5) * sx, (y as f32 + 0.5) * sy);
            let mut s = [0.0f32; 4];
            for &(k, dx, dy) in &taps {
                let p = low.sample(cx + dx, cy + dy);
                for c in 0..4 {
                    s[c] += k * p[c];
                }
            }
            let i = cur.idx(x, y);
            for c in 0..4 {
                cur.data[i + c] += wl * s[c] / 16.0;
            }
        }
    }
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let strength = f.f(0) / 100.0;
    let r = f.f(1).max(4.0) * sc;
    let falloff = f.f(2);
    let thr = f.f(3);
    let gam = f.f(4).max(0.01) as f32;
    let mode = f.i(5);
    let tint = if f.b(6) {
        let c = f.color(7);
        [0, 1, 2].map(|i| color::linear_to_srgb(c[i]).powf(gam))
    } else {
        [1.0; 3]
    };
    let aspect = f.f(8);
    let angle = f.f(9);
    let fixsize = f.b(10);
    if strength <= 0.0 || f.width == 0 || f.height == 0 {
        return Ok(());
    }
    let a = 1.0 + aspect.abs().min(10.0);
    let rg = r / a.sqrt();
    let th = if a > 1.0 { angle.to_radians() } else { 0.0 };
    let (co, si) = (th.cos(), th.sin());
    let (sx, sy) = if a > 1.0 { if aspect >= 0.0 { (a, 1.0) } else { (1.0, a) } } else { (1.0, 1.0) };
    // 広がる分だけ周りを広げる
    let (padx, pady) = if fixsize {
        (0, 0)
    } else {
        let (rx, ry) = (1.25 * rg * sx, 1.25 * rg * sy);
        (((rx * co).powi(2) + (ry * si).powi(2)).sqrt().min(4000.0) as usize, ((rx * si).powi(2) + (ry * co).powi(2)).sqrt().min(4000.0) as usize)
    };
    let org = f.input().padded(padx, pady, padx, pady);
    let (w, h) = (org.w as f64, org.h as f64);
    // 元のバッファの意味（乗算済みの表示値）に直す
    let src = Image::generate(org.w, org.h, |x, y| {
        let d = color::to_display(org.px(x, y));
        [d[0] * d[3], d[1] * d[3], d[2] * d[3], d[3]]
    });
    // グロー空間（回転して短い軸を縮める）
    let gw = ((w * co.abs() + h * si.abs()) / sx).ceil().max(1.0) as usize;
    let gh = ((w * si.abs() + h * co.abs()) / sy).ceil().max(1.0) as usize;
    let (m00, m01, m10, m11) = (co * sx, -si * sy, si * sx, co * sy);
    let (i00, i01, i10, i11) = (co / sx, si / sx, -si / sy, co / sy);
    let t_lin = (thr / 100.0).powf(gam as f64) as f32;
    let knee = 0.5 * t_lin;
    let base = Image::generate(gw, gh, |x, y| {
        let q = [x as f64 + 0.5 - gw as f64 / 2.0, y as f64 + 0.5 - gh as f64 / 2.0];
        let p = [m00 * q[0] + m01 * q[1] + w / 2.0, m10 * q[0] + m11 * q[1] + h / 2.0];
        let c = src.sample(p[0] as f32, p[1] as f32);
        let acc = [c[0].max(0.0).powf(gam), c[1].max(0.0).powf(gam), c[2].max(0.0).powf(gam), c[3]];
        let br = acc[0].max(acc[1]).max(acc[2]);
        let soft = (br - t_lin + knee).clamp(0.0, 2.0 * knee);
        let soft = soft * soft / (4.0 * knee + 1e-5);
        let k = soft.max(br - t_lin) / br.max(1e-5);
        [acc[0] * k * tint[0], acc[1] * k * tint[1], acc[2] * k * tint[2], acc[3] * k]
    });
    // ピラミッド
    let q = 2f64.powf(2.0 - falloff);
    let nf = rg.log2().max(0.05);
    let n_full = (nf.floor() as usize).min(12);
    let frac = nf - n_full as f64;
    let mut levels = vec![base];
    let want = (n_full + usize::from(frac > 0.0)).min(12);
    while levels.len() <= want {
        let last = levels.last().unwrap();
        if last.w.max(last.h) <= 2 {
            break;
        }
        let d = down(last);
        levels.push(d);
    }
    let m = levels.len() - 1;
    if m < 1 {
        return Ok(());
    }
    let partial = frac > 0.0 && m == n_full + 1;
    let (mut total, mut coeff) = (1.0, 1.0);
    for k in 1..=m {
        coeff *= q;
        total += if partial && k == m { coeff * frac } else { coeff };
    }
    for k in (0..m).rev() {
        let wl = if partial && k == m - 1 { q * frac } else { q } as f32;
        let low = levels.pop().unwrap();
        up_add(&low, &mut levels[k], wl);
    }
    let glow = levels.pop().unwrap();
    let amp = (strength / total) as f32;
    let inv = 1.0 / gam;
    let mut out = Image::new(org.w, org.h);
    for y in 0..org.h {
        for x in 0..org.w {
            let p = [x as f64 + 0.5 - w / 2.0, y as f64 + 0.5 - h / 2.0];
            let gx = i00 * p[0] + i01 * p[1] + gw as f64 / 2.0;
            let gy = i10 * p[0] + i11 * p[1] + gh as f64 / 2.0;
            let g = glow.sample(gx as f32, gy as f32);
            let gl = [0, 1, 2].map(|c| (amp * g[c]).max(0.0));
            let ge = gl.map(|v| v.powf(inv));
            let ah = ge[0].max(ge[1]).max(ge[2]).clamp(0.0, 1.0);
            let c = src.px(x, y);
            let (rgb, ao) = match mode {
                2 => (ge.map(|v| v.min(1.0)), ah),
                1 => ([0, 1, 2].map(|k| c[k] + ge[k].min(1.0) * (1.0 - c[k].min(1.0))), c[3] + (1.0 - c[3]) * ah),
                _ => ([0, 1, 2].map(|k| (c[k].max(0.0).powf(gam) + gl[k]).powf(inv)), c[3] + (1.0 - c[3]) * ah),
            };
            let o = if ao > 1e-6 { color::from_display([(rgb[0] / ao).min(1.0), (rgb[1] / ao).min(1.0), (rgb[2] / ao).min(1.0), ao.min(1.0)]) } else { [0.0; 4] };
            out.set(x, y, o);
        }
    }
    f.output(out, -(padx as i32), -(pady as i32));
    Ok(())
}

plugin! {
    vendor: "絵に描いた虹（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "IZUisMe.OpenGlow", name: "Open Glow", category: "光効果", flags: THREAD_SAFE,
            params: [
                float(c"strength", c"強さ", 100.0, 0.0, 1000.0),
                float(c"radius", c"拡散", 300.0, 4.0, 2000.0),
                float(c"falloff", c"減衰", 2.0, 0.5, 4.0),
                float(c"threshold", c"しきい値", 0.0, 0.0, 100.0),
                float(c"gamma", c"ガンマ", 2.2, 1.0, 3.0),
                choice(c"blend_mode", c"合成モード", 0, c"リニア加算|スクリーン|グローのみ"),
                boolean(c"use_tint", c"光色を使う", false),
                color_hex(c"tint", c"光色", 0xffffff),
                float(c"aspect", c"縦横比", 0.0, -10.0, 10.0),
                float(c"angle", c"角度", 0.0, -360.0, 360.0),
                boolean(c"fixsize", c"サイズ固定", false),
            ],
            render: render,
        },
    ]
}

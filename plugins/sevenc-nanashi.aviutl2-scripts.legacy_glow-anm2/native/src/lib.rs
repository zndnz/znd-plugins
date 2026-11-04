//! 旧発光（Nanashi.）の移植: AviUtl（無印）の「発光」の再現。明るさのしきい値を超えた分を取り出し、
//! 半径を変えた 6 回のボックスぼかしを重ねて足す。計算（YC 色空間・4096 段の量子化・拡散速度の対数圧縮）は元と同じ。

use znd_plugin_sdk::*;

type Yc = [f64; 3];

fn to_yc(c: [f64; 3]) -> Yc {
    let y = c[0] * 0.299 + c[1] * 0.587 + c[2] * 0.114;
    [y, (c[2] - y) / 1.772, (c[0] - y) / 1.402]
}
fn to_rgb(c: Yc) -> [f64; 3] {
    [c[0] + 1.402 * c[2], c[0] - 0.344136 * c[1] - 0.714136 * c[2], c[0] + 1.772 * c[1]]
}

struct G {
    exponent: f64,
}
impl G {
    fn decode(&self, mut c: Yc) -> Yc {
        if self.exponent > 0.0 {
            c[0] = (c[0] * self.exponent).exp2() - 1.0;
        }
        c
    }
    fn encode(&self, mut c: Yc) -> Yc {
        c[0] = c[0].max(0.0);
        if self.exponent > 0.0 {
            c[0] = (1.0 + c[0]).log2() / self.exponent;
        }
        c
    }
}

/// 1 方向のボックスぼかし（範囲外は 0、割る数は 2r+1 のまま。元と同じ）。
fn blur_line(g: &G, src: &[Yc], r: usize, out: &mut [Yc]) {
    let n = src.len();
    let mut sum = [0.0; 3];
    for v in src.iter().take(r.min(n.saturating_sub(1)) + 1) {
        let d = g.decode(*v);
        for k in 0..3 {
            sum[k] += d[k];
        }
    }
    let div = (2 * r + 1) as f64;
    for p in 0..n {
        let mut v = sum.map(|s| s / div);
        if g.exponent == 0.0 {
            v = v.map(|x| (x * 4096.0).trunc() / 4096.0);
        } else {
            v[1] = (v[1] * 256.0).round() / 256.0;
            v[2] = (v[2] * 256.0).round() / 256.0;
        }
        out[p] = g.encode(v);
        if p >= r {
            let d = g.decode(src[p - r]);
            for k in 0..3 {
                sum[k] -= d[k];
            }
        }
        if p + r + 1 < n {
            let d = g.decode(src[p + r + 1]);
            for k in 0..3 {
                sum[k] += d[k];
            }
        }
    }
}

fn render(f: &mut Frame) -> FxResult {
    let strength = f.f(0) / 100.0;
    let diffusion = f.f(1).max(10.0) * f.scale();
    let threshold = f.f(2) / 100.0;
    let speed = f.f(3);
    let light = if f.b(4) { Some(f.color(5)) } else { None };
    let fixed = f.b(6);
    if strength <= 0.0 || f.width == 0 || f.height == 0 {
        return Ok(());
    }
    let radii: Vec<usize> = (0..6).map(|i| (2.0 * (diffusion / 2.0).powf(i as f64 / 5.0) + 0.5).floor() as usize).collect();
    let pad = if fixed { 0 } else { radii.iter().sum::<usize>() };
    let src = f.input().padded(pad, pad, pad, pad);
    let (w, h) = (src.w, src.h);
    let g = G { exponent: 256.0 * (1.0 + speed / 1000.0).ln() / 2f64.ln() };
    // 乗算済みの表示値
    let disp: Vec<[f64; 4]> = src.data.chunks_exact(4).map(|p| {
        let d = color::to_display([p[0], p[1], p[2], p[3]]);
        let a = d[3] as f64;
        [d[0] as f64 * a, d[1] as f64 * a, d[2] as f64 * a, a]
    }).collect();
    let lc = light.map(|c| to_yc([0, 1, 2].map(|i| color::linear_to_srgb(c[i]) as f64)));
    let s = (strength * 4096.0).floor();
    let q = |v: f64| (v * 4096.0).floor() / 4096.0;
    let mut seed: Vec<Yc> = disp.iter().map(|p| {
        if p[3] <= 0.0 {
            return [0.0; 3];
        }
        let yc = to_yc([p[0], p[1], p[2]]);
        let diff = (yc[0] * 4096.0).floor() - (threshold * 4096.0).floor();
        let glow = if diff < 0.0 { 0.0 } else { (((diff * s.min(4096.0) / 4096.0).floor() + (s - 4096.0).max(0.0)) / 4096.0).clamp(0.0, 1.0) };
        let mut l = match lc {
            Some(c) => c.map(|v| v * glow),
            None => [glow, yc[1] / p[3] * glow, yc[2] / p[3] * glow],
        };
        l = l.map(q);
        if g.exponent > 0.0 {
            l[1] = ((l[1] * 256.0 + 0.5).floor()).clamp(-128.0, 127.0) / 256.0;
            l[2] = ((l[2] * 256.0 + 0.5).floor()).clamp(-128.0, 127.0) / 256.0;
        }
        l
    }).collect();
    let mut total = vec![[0.0; 3]; w * h];
    let mut tmp = vec![[0.0; 3]; w * h];
    let (mut col, mut colo) = (vec![[0.0; 3]; h], vec![[0.0; 3]; h]);
    for &r0 in &radii {
        // 縦（seed → tmp）
        let r = r0.min((h / 2).saturating_sub(1));
        for x in 0..w {
            for y in 0..h {
                col[y] = seed[y * w + x];
            }
            blur_line(&g, &col, r, &mut colo);
            for y in 0..h {
                tmp[y * w + x] = colo[y];
            }
        }
        // 横（tmp → seed）
        let r = r0.min((w / 2).saturating_sub(1));
        for y in 0..h {
            let row = tmp[y * w..(y + 1) * w].to_vec();
            blur_line(&g, &row, r, &mut seed[y * w..(y + 1) * w]);
        }
        for i in 0..w * h {
            let mut t = g.decode(total[i]);
            let d = g.decode(seed[i]);
            for k in 0..3 {
                t[k] += d[k];
            }
            if g.exponent == 0.0 {
                if t[0] > 2.0 {
                    let k = ((4.0 - t[0]) / 2.0).clamp(0.0, 1.0);
                    t[1] *= k;
                    t[2] *= k;
                    t[0] = 2.0;
                }
            } else {
                t[1] = t[1].clamp(-0.5, 127.0 / 256.0);
                t[2] = t[2].clamp(-0.5, 127.0 / 256.0);
            }
            total[i] = g.encode(t);
        }
    }
    let mut out = Image::new(w, h);
    for i in 0..w * h {
        let p = disp[i];
        let gl = total[i];
        let base = to_yc([p[0], p[1], p[2]]);
        let rgb = to_rgb([gl[0] + base[0], gl[1] + base[1], gl[2] + base[2]]);
        let a = (p[3] + gl[0]).clamp(0.0, 1.0);
        let o = if a > 1e-6 { color::from_display([0, 1, 2].map(|k| (rgb[k] / a).clamp(0.0, 1.0) as f32).into_iter().chain([a as f32]).collect::<Vec<_>>().try_into().unwrap()) } else { [0.0; 4] };
        out.data[i * 4..i * 4 + 4].copy_from_slice(&o);
    }
    f.output(out, -(pad as i32), -(pad as i32));
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-scripts.legacy_glow-anm2", name: "旧発光", category: "光効果", flags: THREAD_SAFE,
            params: [
                float(c"strength", c"強さ", 100.0, 0.0, 200.0),
                float(c"diffusion", c"拡散", 250.0, 10.0, 800.0),
                float(c"threshold", c"しきい値", 80.0, 0.0, 200.0),
                float(c"speed", c"拡散速度", 0.0, 0.0, 60.0),
                boolean(c"use_color", c"光色を使う", false),
                color_hex(c"light_color", c"光色", 0xffffff),
                boolean(c"fixed_size", c"サイズ固定", false),
            ],
            render: render,
        },
    ]
}

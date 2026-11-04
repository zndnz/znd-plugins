//! JPEGノイズ（作業だるい）の移植: 8×8 の DCT・量子化・逆 DCT で JPEG のブロックノイズと色のにじみを出す。
//! 計算は元のシェーダーと同じ（YUV の作り方・量子化表・補間式も元のまま）。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

const ZIGZAG: [u32; 64] = [
    0, 1, 5, 6, 14, 15, 27, 28, 2, 4, 7, 13, 16, 26, 29, 42, 3, 8, 12, 17, 25, 30, 41, 43, 9, 11, 18, 24, 31, 40, 44, 53,
    10, 19, 23, 32, 39, 45, 52, 54, 20, 22, 33, 38, 46, 51, 55, 60, 21, 34, 37, 47, 50, 56, 59, 61, 35, 36, 48, 49, 57, 58, 62, 63,
];
const QY: [f64; 64] = [
    16., 11., 10., 16., 24., 40., 51., 61., 12., 12., 14., 19., 26., 58., 60., 55., 14., 13., 16., 24., 40., 57., 69., 55.,
    14., 17., 22., 29., 51., 87., 80., 62., 18., 22., 37., 56., 68., 109., 103., 77., 24., 35., 55., 64., 81., 104., 113., 92.,
    49., 64., 78., 87., 103., 121., 120., 101., 72., 92., 95., 98., 112., 100., 103., 99.,
];
const QUV: [f64; 64] = {
    let mut t = [99.0; 64];
    let head: [[f64; 4]; 4] = [[17., 18., 24., 47.], [18., 21., 26., 66.], [24., 26., 56., 99.], [47., 66., 99., 99.]];
    let mut y = 0;
    while y < 4 {
        let mut x = 0;
        while x < 4 {
            t[y * 8 + x] = head[y][x];
            x += 1;
        }
        y += 1;
    }
    t
};

fn render(f: &mut Frame) -> FxResult {
    let q = f.f(0);
    let max_f = f.f(1).round().clamp(1.0, 64.0) as u32;
    let comp_alpha = f.b(2);
    let scale = f.f(3).round().max(1.0) as i64;
    let (shx, shy) = (f.f(4), f.f(5));
    let src = f.input();
    let (w, h) = (src.w as i64, src.h as i64);
    if w == 0 || h == 0 {
        return Ok(());
    }
    let mut cos = [[0.0f64; 8]; 8];
    for (u, row) in cos.iter_mut().enumerate() {
        for (x, v) in row.iter_mut().enumerate() {
            *v = ((2 * x + 1) as f64 * u as f64 * PI / 16.0).cos();
        }
    }
    let bpx = 8 * scale;
    let (bx, by) = ((w + bpx - 1) / bpx + 2, (h + bpx - 1) / bpx + 2);
    let ox = (((bx * bpx - w) as f64) * 0.5 - shx).floor() as i64;
    let oy = (((by * bpx - h) as f64) * 0.5 - shy).floor() as i64;
    let half = scale >> 1;
    let mut out = Image::new(src.w, src.h);
    let mut d = [[[0.0f64; 4]; 8]; 8];
    let mut orig_a = [[0.0f64; 8]; 8];
    for gy in 0..by {
        for gx in 0..bx {
            for ly in 0..8 {
                for lx in 0..8 {
                    let sx = ((gx * 8 + lx as i64) * scale - ox + half).clamp(0, w - 1);
                    let sy = ((gy * 8 + ly as i64) * scale - oy + half).clamp(0, h - 1);
                    let c = color::to_display(src.px(sx as usize, sy as usize));
                    let (r, g, b) = (c[0] as f64 * 255.0, c[1] as f64 * 255.0, c[2] as f64 * 255.0);
                    let a = c[3] as f64 * 255.0;
                    d[ly][lx] = [
                        0.299 * r + 0.587 * g + 0.114 * b,
                        (0.5 * r - 0.419 * g - 0.081 * b + 128.0) * 0.5,
                        (-0.169 * r - 0.332 * g + 0.5 * b + 128.0) * 0.5,
                        a,
                    ];
                    orig_a[ly][lx] = a;
                }
            }
            // 横の FDCT
            let mut t = [[[0.0f64; 4]; 8]; 8];
            for ly in 0..8 {
                for lx in 0..8 {
                    let mut s = [0.0; 4];
                    for tx in 0..8 {
                        let c = cos[lx][tx];
                        let p = d[ly][tx];
                        s[0] += p[0] * c;
                        s[1] += (p[1] - 64.0) * c;
                        s[2] += (p[2] - 64.0) * c;
                        s[3] += p[3] * c;
                    }
                    t[ly][lx] = s;
                }
            }
            // 縦の FDCT と量子化
            for ly in 0..8 {
                for lx in 0..8 {
                    let idx = lx + ly * 8;
                    if ZIGZAG[idx] >= max_f {
                        d[ly][lx] = [0.0; 4];
                        continue;
                    }
                    let mut s = [0.0; 4];
                    for ty in 0..8 {
                        for k in 0..4 {
                            s[k] += t[ty][lx][k] * cos[ly][ty];
                        }
                    }
                    let cu = if lx == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
                    let cv = if ly == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
                    let fac = 0.25 * cu * cv;
                    let sy = (0.02 * (1.0 - QY[idx]) * q + 2.0 * QY[idx] - 1.0).max(1.0);
                    let suv = (0.02 * (1.0 - QUV[idx]) * q + 2.0 * QUV[idx] - 1.0).max(1.0);
                    let step = [sy, suv, suv, sy];
                    for k in 0..4 {
                        d[ly][lx][k] = (s[k] * fac / step[k]).round() * step[k];
                    }
                }
            }
            // 縦の IDCT
            for ly in 0..8 {
                for lx in 0..8 {
                    let mut s = [0.0; 4];
                    for v in 0..8 {
                        let cv = if v == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
                        for k in 0..4 {
                            s[k] += d[v][lx][k] * cv * cos[v][ly];
                        }
                    }
                    t[ly][lx] = s;
                }
            }
            // 横の IDCT と書き出し
            for ly in 0..8 {
                for lx in 0..8 {
                    let mut o = [0.0; 4];
                    for u in 0..8 {
                        let cu = if u == 0 { std::f64::consts::FRAC_1_SQRT_2 } else { 1.0 };
                        for k in 0..4 {
                            o[k] += t[ly][u][k] * cu * cos[u][lx];
                        }
                    }
                    let y = o[0] * 0.25;
                    let u = 2.0 * (o[1] * 0.25 + 64.0) - 128.0;
                    let v = 2.0 * (o[2] * 0.25 + 64.0) - 128.0;
                    let a = if comp_alpha { o[3] * 0.25 } else { orig_a[ly][lx] };
                    let rgb = [y + 1.402 * u, y - 0.714 * u - 0.344 * v, y + 1.772 * v].map(|c| (c / 255.0).clamp(0.0, 1.0) as f32);
                    let a = (a / 255.0).clamp(0.0, 1.0) as f32;
                    let px = color::from_display([rgb[0], rgb[1], rgb[2], a]);
                    let bxo = (gx * 8 + lx as i64) * scale - ox;
                    let byo = (gy * 8 + ly as i64) * scale - oy;
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let (wx, wy) = (bxo + dx, byo + dy);
                            if wx >= 0 && wy >= 0 && wx < w && wy < h {
                                out.set(wx as usize, wy as usize, px);
                            }
                        }
                    }
                }
            }
        }
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "作業だるい（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "Sagyou.JpegNoise", name: "JPEGノイズ", category: "加工", flags: THREAD_SAFE,
            params: [
                float(c"quality", c"品質", 50.0, 0.0, 100.0),
                float(c"interlace", c"インターレース", 64.0, 1.0, 64.0),
                boolean(c"compress_alpha", c"透明度も圧縮", false),
                float(c"block_scale", c"サイズ倍率", 1.0, 1.0, 16.0),
                float(c"shift_x", c"Xズレ", 0.0, -32.0, 32.0),
                float(c"shift_y", c"Yズレ", 0.0, -32.0, 32.0),
            ],
            render: render,
        },
    ]
}

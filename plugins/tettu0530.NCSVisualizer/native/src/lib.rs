//! NCSVisualizer（Tettu0530）の移植: 音に反応して膨らむ、点でできた球（NCS のビジュアライザー風）。
//! 格子の点をパーリンノイズで揺らし、半球に投影して、奥行きに応じた遠近で円を描く。
//! コンポの大きさの平面に掛け、「音声レイヤー」に音のレイヤーを選ぶ。

use znd_plugin_sdk::audio::{band, spectrum};
use znd_plugin_sdk::draw::{circle, fill, paint};
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

const PERM: [u8; 256] = [151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180];
const GX: [f32; 16] = [1., -1., 1., -1., 1., -1., 1., -1., 0., 0., 0., 0., 1., 0., -1., 0.];
const GY: [f32; 16] = [1., 1., -1., -1., 0., 0., 0., 0., 1., -1., 1., -1., 1., -1., 1., -1.];
const GZ: [f32; 16] = [0., 0., 0., 0., 1., 1., -1., -1., 1., 1., -1., -1., 0., 1., 0., -1.];

fn p(i: usize) -> usize {
    PERM[i & 255] as usize
}

/// Ken Perlin の improved noise（元のスクリプトと同じ表と勾配。1.4 倍）。
fn perlin(x: f32, y: f32, z: f32) -> f32 {
    let (xf, yf, zf) = (x.floor(), y.floor(), z.floor());
    let (xi, yi, zi) = ((xf as i64).rem_euclid(256) as usize, (yf as i64).rem_euclid(256) as usize, (zf as i64).rem_euclid(256) as usize);
    let (x, y, z) = (x - xf, y - yf, z - zf);
    let fade = |t: f32| t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    let (u, v, w) = (fade(x), fade(y), fade(z));
    let a = p(xi) + yi;
    let b = p(xi + 1) + yi;
    let (aa, ab, ba, bb) = (p(a) + zi, p(a + 1) + zi, p(b) + zi, p(b + 1) + zi);
    let g = |h: usize, x: f32, y: f32, z: f32| {
        let h = p(h) % 16;
        GX[h] * x + GY[h] * y + GZ[h] * z
    };
    let l = |a: f32, b: f32, t: f32| a + t * (b - a);
    let m1 = l(l(g(aa, x, y, z), g(ba, x - 1.0, y, z), u), l(g(ab, x, y - 1.0, z), g(bb, x - 1.0, y - 1.0, z), u), v);
    let m2 = l(
        l(g(aa + 1, x, y, z - 1.0), g(ba + 1, x - 1.0, y, z - 1.0), u),
        l(g(ab + 1, x, y - 1.0, z - 1.0), g(bb + 1, x - 1.0, y - 1.0, z - 1.0), u),
        v,
    );
    l(m1, m2, w) * 1.4
}

/// 帯域の強さ（元の obj.getaudio の fourier に合わせた目盛り）。
fn strength(samples: &[f32], band_no: usize) -> f32 {
    let spec = spectrum(samples);
    let (lo, hi) = match band_no {
        1 => (0.0, 150.0),
        2 => (150.0, 2000.0),
        3 => (2000.0, 24000.0),
        _ => (0.0, 24000.0),
    };
    band(&spec, 48000.0, lo, hi) * 32768.0
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale() as f32;
    let q = |i: usize| f.f(i) as f32;
    let (sphere, res, point) = (q(0), q(1).round().max(2.0) as usize, q(2));
    let col = f.color(3);
    let (spherical, pf) = (f.b(4), q(5));
    let (noise_on, amp0, period, nz) = (f.b(6), q(7), q(8), q(9));
    let (fx, fy, fz) = (q(10), q(11), q(12));
    let band_no = f.i(14);
    let (r_mul, n_mul) = (q(15), q(16));
    let (use_ar, smooth, attack, release) = (f.b(17), q(18), q(19), q(20));
    // 音: 元は前のフレームからの追従（状態を持つ）。ここでは過去 0.5 秒の窓を順に追従させて、状態を持たずに同じ動きを作る
    let mut level = 0.0f32;
    if f.has_layer(13) {
        let fps = f.fps().max(1.0) as f32;
        let hop = (48000.0 / fps) as usize;
        let steps = 15usize;
        if let Some(buf) = f.audio(13, hop * steps + 2048) {
            for k in 0..=steps {
                let end = buf.len() - (steps - k) * hop;
                let win = &buf[end.saturating_sub(2048)..end];
                let a = strength(win, band_no);
                let t = if use_ar { if a > level { attack } else { release } } else { smooth } * 0.04;
                level += t.min(1.0) * (a - level);
            }
        }
    }
    let frame = f.frame() as f32;
    let amp = amp0 + level * n_mul * 0.2;
    let radius = sphere * 0.5 + level * r_mul * 0.1;
    let freq = period * 0.001;
    let (ox, oy, oz) = (frame * fx * 0.1, frame * fy * 0.1, nz + frame * fz * 0.1);
    let gap = 800.0 / (res - 1) as f32;
    let (w, h) = (f.width, f.height);
    let (cx, cy) = (w as f64 / 2.0, h as f64 / 2.0);
    let mut img = Image::new(w, h);
    let paint_c = color::premul(col, 1.0);
    // 奥にある点から描く（重なりを正しく）
    let mut pts = Vec::with_capacity(res * res);
    for xi in 0..res {
        for yi in 0..res {
            let (mut x, mut y, mut z) = (xi as f32 * gap - 400.0, yi as f32 * gap - 400.0, 0.0f32);
            if noise_on {
                let (gx, gy) = (xi as f32 * freq, yi as f32 * freq);
                x += amp * perlin(gx + ox, gy, freq * oz + 100.0);
                y += amp * perlin(gx + 100.0, gy + oy, freq * oz);
                z += amp * perlin(gx, gy + 100.0, freq * oz);
            }
            if spherical && pf > 0.0 {
                let len = (x * x + y * y + z * z).sqrt();
                if len > 0.0 {
                    let k = radius / len;
                    let (sx, sy, sz) = (x * k, y * k, (z * k).abs());
                    x += pf * (sx - x);
                    y += pf * (sy - y);
                    z += pf * (sz - z);
                }
            }
            pts.push((x, y, z));
        }
    }
    pts.sort_by(|a, b| b.2.total_cmp(&a.2));
    for (x, y, z) in pts {
        // AviUtl の既定のカメラ（距離 1024）の遠近
        let k = 1024.0 / (1024.0 + z).max(1.0);
        let r = (point * 0.5 * k * s).max(0.3) as f64;
        let c = V2::new(cx + (x * k * s) as f64, cy + (y * k * s) as f64);
        if c.x < -r || c.y < -r || c.x > w as f64 + r || c.y > h as f64 + r {
            continue;
        }
        // 小さな円だけを描くので、周りの小さな範囲で塗る
        let (x0, y0) = ((c.x - r - 1.0).floor().max(0.0) as usize, (c.y - r - 1.0).floor().max(0.0) as usize);
        let (x1, y1) = (((c.x + r + 1.0).ceil() as usize).min(w), ((c.y + r + 1.0).ceil() as usize).min(h));
        if x1 <= x0 || y1 <= y0 {
            continue;
        }
        let (bw, bh) = (x1 - x0, y1 - y0);
        let cov = fill(bw, bh, &[circle(V2::new(c.x - x0 as f64, c.y - y0 as f64), r)]);
        let mut tile = Image::new(bw, bh);
        paint(&mut tile, &cov, paint_c);
        img.draw(&tile, x0 as isize, y0 as isize, 1.0);
    }
    f.set(img);
    Ok(())
}

plugin! {
    vendor: "Tettu0530（znd 移植: znd-plugins）", version: "2.1", license: "MIT",
    effects: [
        {
            id: "tettu0530.NCSVisualizer", name: "NCSVisualizer", category: "描画", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"sphere_size", c"球サイズ", 600.0, 0.0, 2160.0),
                float(c"resolution", c"メッシュ分割数", 64.0, 2.0, 300.0),
                float(c"pointer_size", c"点の大きさ", 6.0, 0.0, 50.0),
                color_hex(c"color", c"色", 0xffffff),
                boolean(c"spherical", c"球変換", true),
                float(c"projection", c"球体化の強さ", 1.0, 0.0, 1.0),
                boolean(c"noise", c"ノイズ", true),
                float(c"noise_height", c"ノイズ変位高さ", 64.0, 0.0, 1000.0),
                float(c"noise_period", c"ノイズ変位周期", 16.0, 0.0, 200.0),
                float(c"noise_z", c"ノイズZ", 0.0, -10000.0, 10000.0),
                float(c"flow_x", c"ノイズの流れX", -0.2, -50.0, 50.0),
                float(c"flow_y", c"ノイズの流れY", -0.4, -50.0, 50.0),
                float(c"flow_z", c"ノイズの流れZ", 0.0, -50.0, 50.0),
                layer(c"audio", c"音声レイヤー"),
                choice(c"band", c"反応帯域", 0, c"全体|低音|中音|高音"),
                float(c"radius_mul", c"球サイズ感度", 0.3, 0.0, 20.0),
                float(c"noise_mul", c"球ノイズ感度", 0.2, 0.0, 20.0),
                boolean(c"attack_release", c"アタック/リリースを分ける", false),
                float(c"smoothness", c"動きの滑らかさ", 20.0, 1.0, 100.0),
                float(c"attack", c"アタック", 60.0, 1.0, 100.0),
                float(c"release", c"リリース", 15.0, 1.0, 100.0),
            ],
            render: render,
        },
    ]
}

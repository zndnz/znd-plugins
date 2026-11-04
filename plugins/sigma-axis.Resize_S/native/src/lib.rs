//! Resize_S（σ軸）の移植: アルゴリズムを選べるリサイズσと、矩形に収める／覆うボックスリサイズσ。
//! 横 → 縦の順に 1 次元の拡大縮小を掛ける（元のシェーダーと同じ計算）。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

#[derive(Clone, Copy, PartialEq)]
enum Up {
    Nearest,
    Bilinear,
    Mitchell,
    CatmullRom,
    Lanczos2,
    Lanczos3,
}
#[derive(Clone, Copy, PartialEq)]
enum Dn {
    Nearest,
    Average,
    Bilinear,
    Hamming,
    Lanczos2,
    Lanczos3,
}

fn up_of(i: usize) -> Up {
    [Up::Nearest, Up::Bilinear, Up::Mitchell, Up::CatmullRom, Up::Lanczos2, Up::Lanczos3][i.min(5)]
}
fn dn_of(i: usize) -> Dn {
    [Dn::Nearest, Dn::Average, Dn::Bilinear, Dn::Hamming, Dn::Lanczos2, Dn::Lanczos3][i.min(5)]
}

fn lanczos(x: f64, a: f64) -> f64 {
    if x.abs() < 1.0 / 256.0 {
        return 1.0 - (PI * PI * (1.0 + 1.0 / (a * a)) / 6.0) * x * x;
    }
    if x.abs() >= a {
        return 0.0;
    }
    (PI / a * x).sin() * (PI * x).sin() / (PI * PI / a * x * x)
}
fn cubic(x: f64, b: f64, c: f64) -> f64 {
    let x = x.abs();
    if x < 1.0 {
        ((12.0 - 9.0 * b - 6.0 * c) * x * x * x + (-18.0 + 12.0 * b + 6.0 * c) * x * x + (6.0 - 2.0 * b)) / 6.0
    } else if x < 2.0 {
        ((-b - 6.0 * c) * x * x * x + (6.0 * b + 30.0 * c) * x * x + (-12.0 * b - 48.0 * c) * x + (8.0 * b + 24.0 * c)) / 6.0
    } else {
        0.0
    }
}
fn tent(x: f64) -> f64 {
    (1.0 - x.abs()).max(0.0)
}

/// 出力の画素ごとの（入力の位置, 重み）の表。
fn weights(n: usize, m: usize, up: Up, dn: Dn) -> Vec<Vec<(usize, f32)>> {
    let zoom = m as f64 / n as f64;
    let inv = 1.0 / zoom;
    let clamp = |x: i64| x.clamp(0, n as i64 - 1) as usize;
    let norm = |v: Vec<(usize, f64)>| {
        let s: f64 = v.iter().map(|p| p.1).sum();
        let s = if s.abs() < 1e-12 { 1.0 } else { s };
        v.into_iter().map(|(i, w)| (i, (w / s) as f32)).collect::<Vec<_>>()
    };
    (0..m)
        .map(|i| {
            let c = i as f64 + 0.5;
            if m == n {
                return vec![(i, 1.0)];
            }
            let nearest = vec![(clamp((c * inv).floor() as i64), 1.0)];
            if m > n {
                let (k, s): (fn(f64) -> f64, i64) = match up {
                    Up::Nearest => return nearest,
                    Up::Bilinear => (tent, 1),
                    Up::Mitchell => (|x| cubic(x, 1.0 / 3.0, 1.0 / 3.0), 2),
                    Up::CatmullRom => (|x| cubic(x, 0.0, 0.5), 2),
                    Up::Lanczos2 => (|x| lanczos(x, 2.0), 2),
                    Up::Lanczos3 => (|x| lanczos(x, 3.0), 3),
                };
                let f = c * inv - 0.5;
                let fi = f.floor() as i64;
                let fr = f - fi as f64;
                norm(((1 - s)..=s).map(|d| (clamp(fi + d), k(fr - d as f64))).collect())
            } else {
                let (k, s): (fn(f64) -> f64, f64) = match dn {
                    Dn::Nearest => return nearest,
                    Dn::Average => {
                        let (a, b) = (inv * (c - 0.5), inv * (c + 0.5));
                        let (ai, bi) = (a.floor() as i64, b.floor() as i64);
                        let mut v = vec![(clamp(ai), (ai + 1) as f64 - a)];
                        for x in ai + 1..bi {
                            v.push((clamp(x), 1.0));
                        }
                        v.push((clamp(bi), b - bi as f64));
                        if ai == bi {
                            v = vec![(clamp(ai), b - a)];
                        }
                        return norm(v);
                    }
                    Dn::Bilinear => (tent, 1.0),
                    Dn::Hamming => (|x| if x.abs() < 1.0 { 25.0 / 46.0 + (21.0 / 46.0) * (PI * x).cos() } else { 0.0 }, 1.0),
                    Dn::Lanczos2 => (|x| lanczos(x, 2.0), 2.0),
                    Dn::Lanczos3 => (|x| lanczos(x, 3.0), 3.0),
                };
                let a = (inv * (c - s) + 0.5).floor() as i64;
                let b = (inv * (c + s) + 0.5).floor() as i64;
                norm((a..b).map(|x| (clamp(x), k(zoom * (x as f64 + 0.5) - c))).collect())
            }
        })
        .collect()
}

/// w×h を W×H に拡大縮小する。
fn resize(src: &Image, nw: usize, nh: usize, up: Up, dn: Dn) -> Image {
    let (w, h) = (src.w, src.h);
    let wx = weights(w, nw, up, dn);
    let mut tmp = Image::new(nw, h);
    for y in 0..h {
        for (x, ws) in wx.iter().enumerate() {
            let mut acc = [0.0f32; 4];
            for &(sx, k) in ws {
                let p = src.px(sx, y);
                for c in 0..4 {
                    acc[c] += p[c] * k;
                }
            }
            tmp.set(x, y, acc);
        }
    }
    let wy = weights(h, nh, up, dn);
    let mut out = Image::new(nw, nh);
    for (y, ws) in wy.iter().enumerate() {
        for x in 0..nw {
            let mut acc = [0.0f32; 4];
            for &(sy, k) in ws {
                let p = tmp.px(x, sy);
                for c in 0..4 {
                    acc[c] += p[c] * k;
                }
            }
            // 負のローブではみ出た分を、乗算済みアルファとして正しい範囲に収める。
            let a = acc[3].clamp(0.0, 1.0);
            out.set(x, y, [acc[0].clamp(0.0, a), acc[1].clamp(0.0, a), acc[2].clamp(0.0, a), a]);
        }
    }
    out
}

const MAX: usize = 16384;

fn render_resize(f: &mut Frame) -> FxResult {
    let zoom = f.f(0).max(0.0) / 100.0;
    let absolute = f.b(3);
    let (w, h) = (f.width, f.height);
    let (sx, sy) = if absolute {
        (f.f(1).max(0.0).round() * f.scale(), f.f(2).max(0.0).round() * f.scale())
    } else {
        (f.f(1).max(0.0) / 100.0 * w as f64, f.f(2).max(0.0) / 100.0 * h as f64)
    };
    let nw = ((zoom * sx).round() as usize).min(MAX);
    let nh = ((zoom * sy).round() as usize).min(MAX);
    if nw == w && nh == h {
        return Ok(());
    }
    if nw == 0 || nh == 0 {
        f.output(Image::new(1, 1), 0, 0);
        return Ok(());
    }
    let img = resize(&f.input(), nw, nh, up_of(f.i(4)), dn_of(f.i(5)));
    let ox = ((w as f64 - nw as f64) / 2.0).round() as i32;
    let oy = ((h as f64 - nh as f64) / 2.0).round() as i32;
    f.output(img, ox, oy);
    Ok(())
}

fn render_box(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let bw = (f.f(0).max(0.0) * sc).round();
    let bh = (f.f(1).max(0.0) * sc).round();
    let cover = f.i(2) == 1;
    let dir = f.i(3);
    let ax = (f.f(6) / 100.0).clamp(-1.0, 1.0);
    let ay = (f.f(7) / 100.0).clamp(-1.0, 1.0);
    let crop_pad = f.b(8);
    let (w, h) = (f.width as f64, f.height as f64);
    if bw <= 0.0 || bh <= 0.0 {
        f.output(Image::new(1, 1), 0, 0);
        return Ok(());
    }
    // 内接最大なら小さい方、外接最小なら大きい方の倍率に合わせる。
    let (mut nw, mut nh) = if (!cover) == (bw * h <= bh * w) { (bw, (bw / w * h).round()) } else { ((bh / h * w).round(), bh) };
    if (dir == 2 && nw >= w && nh >= h) || (dir == 1 && nw <= w && nh <= h) {
        nw = w;
        nh = h;
    }
    let (nw, nh) = ((nw as usize).clamp(1, MAX), (nh as usize).clamp(1, MAX));
    let src = f.input();
    let img = if nw == f.width && nh == f.height { src } else { resize(&src, nw, nh, up_of(f.i(4)), dn_of(f.i(5))) };
    if !crop_pad {
        let ox = ((w - nw as f64) / 2.0).round() as i32;
        let oy = ((h - nh as f64) / 2.0).round() as i32;
        f.output(img, ox, oy);
        return Ok(());
    }
    // 箱の大きさに余白を足す／はみ出た分を切る（揃えで配分を決める）。
    let (cw, ch) = (bw as usize, bh as usize);
    let px = ((cw as f64 - nw as f64) * (1.0 + ax) / 2.0).round() as isize;
    let py = ((ch as f64 - nh as f64) * (1.0 + ay) / 2.0).round() as isize;
    let mut out = Image::new(cw, ch);
    out.draw(&img, px, py, 1.0);
    let ox = ((w - cw as f64) / 2.0).round() as i32;
    let oy = ((h - ch as f64) / 2.0).round() as i32;
    f.output(out, ox, oy);
    Ok(())
}

const UP: &std::ffi::CStr = c"最近傍法|双線形|Mitchell-Netravali|Catmull-Rom|Lanczos2|Lanczos3";
const DN: &std::ffi::CStr = c"最近傍法|単純平均|双線形|Hamming|Lanczos2|Lanczos3";

plugin! {
    vendor: "σ軸（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sigma-axis.Resize_S", name: "リサイズσ", category: "変形", flags: THREAD_SAFE,
            params: [
                float(c"zoom", c"拡大率", 100.0, 0.0, 5000.0),
                float(c"sz_x", c"X", 100.0, 0.0, 5000.0),
                float(c"sz_y", c"Y", 100.0, 0.0, 5000.0),
                boolean(c"absolute", c"ピクセル数でサイズ指定", false),
                choice(c"upscale", c"拡大方法", 5, UP),
                choice(c"downscale", c"縮小方法", 5, DN),
            ],
            render: render_resize,
        },
        {
            id: "sigma-axis.Resize_S.box", name: "ボックスリサイズσ", category: "変形", flags: THREAD_SAFE,
            params: [
                float(c"width", c"X", 256.0, 0.0, 4000.0),
                float(c"height", c"Y", 256.0, 0.0, 4000.0),
                choice(c"mode", c"モード", 0, c"内接最大|外接最小"),
                choice(c"dir", c"拡大縮小", 0, c"拡縮両方|拡大のみ|縮小のみ"),
                choice(c"upscale", c"拡大方法", 5, UP),
                choice(c"downscale", c"縮小方法", 5, DN),
                float(c"align_x", c"水平揃え", 0.0, -100.0, 100.0),
                float(c"align_y", c"垂直揃え", 0.0, -100.0, 100.0),
                boolean(c"crop_pad", c"余白/クリッピング", true),
            ],
            render: render_box,
        },
    ]
}

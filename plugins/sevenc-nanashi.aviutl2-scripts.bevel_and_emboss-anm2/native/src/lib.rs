//! ベベルとエンボス（Nanashi.、Mr-Ojii の Bevel_And_Emboss_M と gometh 氏の元スクリプトがもと）の移植:
//! 輪郭からの距離（ジャンプフラッディングで求める）の傾きで、光の当たる側と影の側を作って重ねる。
//! 手順と式は元のシェーダーと同じ（色は表示値）。

use znd_plugin_sdk::*;

type Off = Option<[f64; 2]>;

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let thr = (f.f(0) / 255.0) as f32;
    let width = f.f(1) * sc;
    let angle = f.f(2).to_radians();
    let elev = f.f(3).to_radians();
    let shape = f.i(4);
    let output = f.i(5);
    let blur = f.f(6) * sc;
    let preblur = f.f(7) * sc;
    let disp = |c: [f32; 4]| [0, 1, 2].map(|i| color::linear_to_srgb(c[i]) as f64);
    let (hcol, hblend, hop) = (disp(f.color(8)), f.i(9), f.f(10) / 100.0);
    let (scol, sblend, sop) = (disp(f.color(11)), f.i(12), f.f(13) / 100.0);
    let bg = disp(f.color(14));
    if width <= 0.0 || f.width == 0 || f.height == 0 {
        return Ok(());
    }
    let bw = if shape >= 2 { width / 2.0 } else { width };
    let margin = (preblur + 1.0 + if shape == 1 { 0.0 } else { bw } + blur).ceil() as usize;
    let org = f.input().padded(margin, margin, margin, margin);
    let (w, h) = (org.w, org.h);
    let mut contour = org.clone();
    if preblur > 0.0 {
        aul::blur(&mut contour, preblur as f32);
    }
    let ca: Vec<f32> = contour.data.chunks_exact(4).map(|p| p[3]).collect();
    let a_at = |x: isize, y: isize| if x < 0 || y < 0 || x >= w as isize || y >= h as isize { 0.0 } else { ca[y as usize * w + x as usize] };
    // 輪郭の種（輪郭の画素から、しきい値の位置へのずれ）
    let mut seeds: Vec<Off> = vec![None; w * h];
    for y in 0..h as isize {
        for x in 0..w as isize {
            let a = a_at(x, y);
            let (l, r, u, d) = (a_at(x - 1, y), a_at(x + 1, y), a_at(x, y - 1), a_at(x, y + 1));
            if a <= thr || l.min(r).min(u.min(d)) > thr {
                continue;
            }
            let mut dx = 0.0;
            let mut dy = 0.0;
            if r <= thr {
                dx = ((a - thr) / (a - r)) as f64;
            } else if l <= thr {
                dx = -((a - thr) / (a - l)) as f64;
            }
            if d <= thr {
                dy = ((a - thr) / (a - d)) as f64;
            } else if u <= thr {
                dy = -((a - thr) / (a - u)) as f64;
            }
            let s = dx.abs() + dy.abs();
            // 元は 0 のずれを「種なし」と同じに扱うので、ずれ 0 は小さな値にしておく
            let o = if s > 0.0 { [dx * dx.abs() / s, dy * dy.abs() / s] } else { [0.0, 0.0] };
            seeds[y as usize * w + x as usize] = Some(if o == [0.0, 0.0] { [1e-6, 0.0] } else { o });
        }
    }
    // ジャンプフラッディング
    let limit = (bw + 2.0) * (bw + 2.0);
    let jump = |src: &Vec<Off>, step: isize| -> Vec<Off> {
        let mut out = vec![None; w * h];
        for y in 0..h as isize {
            for x in 0..w as isize {
                let mut best = limit;
                let mut b = None;
                for jy in -1..=1isize {
                    for jx in -1..=1isize {
                        let (qx, qy) = (x + jx * step, y + jy * step);
                        if qx < 0 || qy < 0 || qx >= w as isize || qy >= h as isize {
                            continue;
                        }
                        if let Some(o) = src[qy as usize * w + qx as usize] {
                            let off = [o[0] + (jx * step) as f64, o[1] + (jy * step) as f64];
                            let d2 = off[0] * off[0] + off[1] * off[1];
                            if d2 < best {
                                best = d2;
                                b = Some(off);
                            }
                        }
                    }
                }
                out[y as usize * w + x as usize] = b;
            }
        }
        out
    };
    let mut step = 1isize;
    while (step as f64) < bw + 2.0 {
        step *= 2;
    }
    let mut cur = seeds;
    while step >= 1 {
        cur = jump(&cur, step);
        step /= 2;
    }
    cur = jump(&cur, 1);
    let sd = |x: isize, y: isize| -> f64 {
        let (cx, cy) = (x.clamp(0, w as isize - 1) as usize, y.clamp(0, h as isize - 1) as usize);
        let d = cur[cy * w + cx].map(|o| o[0].hypot(o[1])).unwrap_or(bw + 2.0);
        if ca[cy * w + cx] > thr { -d } else { d }
    };
    let (lx, ly) = (angle.cos(), -angle.sin());
    let strength = elev.cos();
    // field: [光, 影, 帯, 内側]
    let mut field = vec![[0.0f64; 4]; w * h];
    for y in 0..h as isize {
        for x in 0..w as isize {
            if cur[y as usize * w + x as usize].is_none() {
                continue;
            }
            let d = sd(x, y);
            let g = [sd(x + 1, y) - sd(x - 1, y), sd(x, y + 1) - sd(x, y - 1)];
            let gl = g[0].hypot(g[1]).max(2.0);
            let gray = (g[0] / gl * lx + g[1] / gl * ly) * strength;
            field[y as usize * w + x as usize] = [gray.max(0.0), (-gray).max(0.0), (bw - d.abs()).clamp(0.0, 1.0), if d < 0.0 { 1.0 } else { 0.0 }];
        }
    }
    let oa: Vec<f64> = org.data.chunks_exact(4).map(|p| p[3] as f64).collect();
    let make = |col: [f64; 3], is_shadow: bool, op: f64| -> Image {
        let mut img = Image::new(w, h);
        for i in 0..w * h {
            let fl = field[i];
            let mut gray = fl[0] - fl[1];
            if shape == 3 {
                gray *= oa[i] * 2.0 - 1.0;
            }
            let mut a = if is_shadow { -gray } else { gray }.max(0.0);
            if output == 0 {
                if shape == 1 {
                    a *= fl[2];
                } else if shape >= 2 {
                    a *= if fl[2] > 0.0 { if fl[3] > 0.0 { 1.0 } else { fl[2] } } else { 0.0 };
                }
            } else {
                a *= fl[2];
                if shape == 0 {
                    a = a.min(1.0 - oa[i]);
                }
                if shape == 1 {
                    a = a.min(oa[i]);
                }
            }
            a = (a * op).clamp(0.0, 1.0);
            let p = color::from_display([col[0] as f32, col[1] as f32, col[2] as f32, a as f32]);
            img.data[i * 4..i * 4 + 4].copy_from_slice(&p);
        }
        if blur > 0.0 {
            aul::blur(&mut img, blur as f32);
        }
        img
    };
    let hl = if output != 3 { Some(make(hcol, false, hop)) } else { None };
    let sh = if output != 2 { Some(make(scol, true, sop)) } else { None };
    let result = match output {
        2 => hl.unwrap(),
        3 => sh.unwrap(),
        1 => {
            // 直接描画: 元の絵の上に、ハイライトと影をそれぞれの合成モードで重ねる
            let mut base = org.clone();
            aul::blend(&mut base, hl.as_ref().unwrap(), hblend, 1.0);
            aul::blend(&mut base, sh.as_ref().unwrap(), sblend, 1.0);
            base
        }
        _ => {
            let bgl = color::from_display([bg[0] as f32, bg[1] as f32, bg[2] as f32, 1.0]);
            let mut base = Image::generate(w, h, |x, y| {
                let o = org.px(x, y);
                match shape {
                    0 => bgl,
                    1 => if o[3] > 0.0 { [o[0] / o[3], o[1] / o[3], o[2] / o[3], 1.0] } else { [0.0, 0.0, 0.0, 1.0] },
                    _ => {
                        let k = 1.0 - o[3];
                        [o[0] + bgl[0] * k, o[1] + bgl[1] * k, o[2] + bgl[2] * k, 1.0]
                    }
                }
            });
            aul::blend(&mut base, hl.as_ref().unwrap(), hblend, 1.0);
            aul::blend(&mut base, sh.as_ref().unwrap(), sblend, 1.0);
            Image::generate(w, h, |x, y| {
                let i = y * w + x;
                let c = base.px(x, y);
                let o = org.px(x, y);
                let a = if shape == 1 { o[3] } else { o[3].max(field[i][2] as f32) };
                let c = if shape == 0 { [o[0] + c[0] * (1.0 - o[3]), o[1] + c[1] * (1.0 - o[3]), o[2] + c[2] * (1.0 - o[3])] } else { [c[0], c[1], c[2]] };
                [c[0] * a, c[1] * a, c[2] * a, a]
            })
        }
    };
    f.output(result, -(margin as i32), -(margin as i32));
    Ok(())
}

const BLENDS: &std::ffi::CStr = c"通常|加算|減算|乗算|スクリーン|オーバーレイ|比較（明）|比較（暗）|輝度|色差|陰影|明暗|差分";

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-scripts.bevel_and_emboss-anm2", name: "ベベルとエンボス", category: "加工", flags: THREAD_SAFE,
            params: [
                float(c"threshold", c"α閾値", 206.0, 1.0, 254.0),
                float(c"width", c"幅", 20.0, 0.0, 150.0),
                float(c"angle", c"角度", 60.0, -360.0, 360.0),
                float(c"elevation", c"高度", 0.0, 0.0, 90.0),
                choice(c"shape", c"形状", 1, c"ベベル（外側）|ベベル（内側）|エンボス|ピローエンボス"),
                choice(c"output", c"出力", 0, c"通常合成|直接描画|ハイライトのみ|シャドウのみ"),
                float(c"blur", c"ぼかし", 0.0, 0.0, 100.0),
                float(c"preblur", c"参照輪郭ぼかし", 2.0, 0.0, 100.0),
                color_hex(c"highlight_color", c"ハイライト::色", 0xffffff),
                choice(c"highlight_blend", c"ハイライト::合成モード", 4, BLENDS),
                float(c"highlight_opacity", c"ハイライト::不透明度", 100.0, 0.0, 100.0),
                color_hex(c"shadow_color", c"シャドウ::色", 0x000000),
                choice(c"shadow_blend", c"シャドウ::合成モード", 3, BLENDS),
                float(c"shadow_opacity", c"シャドウ::不透明度", 100.0, 0.0, 100.0),
                color_hex(c"background_color", c"背景色", 0x808080),
            ],
            render: render,
        },
    ]
}

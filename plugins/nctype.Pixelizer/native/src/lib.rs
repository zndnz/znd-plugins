//! Pixelizer（nctype）の移植: ドット絵風にする（ブロック化・減色・階調・ディザ・パレット・輪郭）。

use znd_plugin_sdk::color::{from_display, to_display};
use znd_plugin_sdk::*;

fn luma(c: [f32; 3]) -> f32 {
    c[0] * 0.299 + c[1] * 0.587 + c[2] * 0.114
}

const BAYER4: [f32; 16] = [0., 8., 2., 10., 12., 4., 14., 6., 3., 11., 1., 9., 15., 7., 13., 5.];
const GB: [[f32; 3]; 4] = [[15., 56., 15.], [48., 98., 48.], [139., 172., 15.], [155., 188., 15.]];

fn render(f: &mut Frame) -> FxResult {
    let palette = f.i(0);
    let edge_style = f.i(1);
    let ps = (f.f(2) * f.scale()).round().max(1.0);
    let levels = f.f(3).max(2.0) as f32;
    let tone = f.f(4).max(0.0) as f32;
    let dmode = f.i(5);
    let dstr = (f.f(6) / 100.0).clamp(0.0, 1.0) as f32;
    let estr = (f.f(7) / 100.0).clamp(0.0, 1.0) as f32;
    let protect = f.b(8);
    let ath = (f.f(9) / 255.0).clamp(0.0, 1.0) as f32;
    let src = f.input();
    let (w, h) = (src.w as f64, src.h as f64);
    let straight = |p: [f32; 4]| -> [f32; 3] {
        if p[3] <= 1e-5 {
            return [0.0; 3];
        }
        let d = to_display(p);
        [d[0].clamp(0.0, 1.0), d[1].clamp(0.0, 1.0), d[2].clamp(0.0, 1.0)]
    };
    let at = |x: f64, y: f64| src.sample_clamped(x.clamp(0.5, w - 0.5) as f32, y.clamp(0.5, h - 0.5) as f32);
    let mut out = Image::new(src.w, src.h);
    out.map(|x, y, _| {
        let (bx, by) = ((x as f64 / ps).floor(), (y as f64 / ps).floor());
        let (sx, sy) = ((bx * ps + ps * 0.5).min(w - 0.5), (by * ps + ps * 0.5).min(h - 0.5));
        let p = at(sx, sy);
        if p[3] <= 1e-5 || (protect && p[3] <= ath) {
            return [0.0; 4];
        }
        let mut rgb = straight(p);
        if tone >= 2.0 {
            let l = luma(rgb);
            if l > 1e-5 {
                let d = tone - 1.0;
                let q = (l.clamp(0.0, 1.0) * d + 0.5).floor() / d;
                rgb = rgb.map(|v| (v * q / l).clamp(0.0, 1.0));
            }
        }
        if dmode > 0 && dstr > 0.0 {
            let (ix, iy) = (bx.rem_euclid(4.0) as usize, by.rem_euclid(4.0) as usize);
            let dv = if dmode == 1 {
                [0.0, 0.5, 0.75, 0.25][(by.rem_euclid(2.0) as usize) * 2 + bx.rem_euclid(2.0) as usize] - 0.375
            } else {
                BAYER4[iy * 4 + ix] / 16.0 - 0.46875
            };
            let amp = dstr / (levels - 1.0).max(1.0);
            rgb = rgb.map(|v| (v + dv * amp).clamp(0.0, 1.0));
        }
        rgb = match palette {
            1 => {
                let best = GB.iter().map(|c| c.map(|v| v / 255.0)).min_by(|a, b| {
                    let d = |c: &[f32; 3]| (0..3).map(|k| (rgb[k] - c[k]).powi(2)).sum::<f32>();
                    d(a).total_cmp(&d(b))
                });
                best.unwrap()
            }
            2 => [(rgb[0] * 31.0 + 0.5).floor() / 31.0, (rgb[1] * 63.0 + 0.5).floor() / 63.0, (rgb[2] * 31.0 + 0.5).floor() / 31.0],
            3 => {
                let q = (luma(rgb).clamp(0.0, 1.0) * 3.0 + 0.5).floor() / 3.0;
                [q; 3]
            }
            _ => rgb.map(|v| (v * (levels - 1.0) + 0.5).floor() / (levels - 1.0)),
        };
        if estr > 0.0 {
            let al = |dx: f64, dy: f64| at(sx + dx, sy + dy)[3];
            let lu = |dx: f64, dy: f64| { let q = at(sx + dx, sy + dy); if q[3] <= 1e-5 { 0.0 } else { luma(straight(q)) } };
            let (a0, a1, a2, a3, a4) = (p[3], al(-ps, 0.0), al(ps, 0.0), al(0.0, -ps), al(0.0, ps));
            let ld = (lu(-ps, 0.0) - lu(ps, 0.0)).abs() + (lu(0.0, -ps) - lu(0.0, ps)).abs();
            let ad = (a1 - a2).abs() + (a3 - a4).abs() + (a0 - a1).abs() + (a0 - a2).abs() + (a0 - a3).abs() + (a0 - a4).abs();
            let e = (ld * 1.2 + ad).clamp(0.0, 1.0);
            if edge_style == 1 {
                let m = ((e - 0.08) * 2.2).clamp(0.0, 1.0) * estr;
                rgb = rgb.map(|v| v + (v * 0.08 - v) * m);
            } else {
                let k = (1.0 - e * estr * 0.7).clamp(0.0, 1.0);
                rgb = rgb.map(|v| v * k);
            }
        }
        from_display([rgb[0], rgb[1], rgb[2], p[3]])
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "nctype（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "nctype.Pixelizer", name: "Pixelizer（ドット絵）", category: "スタイライズ", flags: THREAD_SAFE,
            params: [
                choice(c"palette", c"パレットモード", 0, c"デフォルト|GameBoy|16-bit|4階調グレースケール"),
                choice(c"edge_style", c"エッジスタイル", 0, c"ソフト|エッジ強調"),
                float(c"pixel_size", c"ピクセルサイズ", 4.0, 1.0, 64.0),
                float(c"color_levels", c"色数", 8.0, 2.0, 32.0),
                float(c"tone_steps", c"階調数（0 で無効）", 4.0, 0.0, 8.0),
                choice(c"dither", c"ディザリング", 0, c"なし|2x2|4x4"),
                float(c"dither_strength", c"ディザリング強度", 35.0, 0.0, 100.0),
                float(c"edge_strength", c"エッジ強度", 20.0, 0.0, 100.0),
                boolean(c"alpha_protect", c"透明エッジ保護", true),
                float(c"alpha_threshold", c"透過しきい値", 8.0, 0.0, 255.0),
            ],
            render: render,
        },
    ]
}

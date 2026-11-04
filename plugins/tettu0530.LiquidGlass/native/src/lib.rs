//! Liquid Glass（Tettu0530）の移植: オブジェクトの形のガラス越しに、背景が屈折して見える（Apple の Liquid Glass 風）。
//! 形の縁を箱型ぼかしして距離場を作り、ベゼル（縁の丸み）の傾きで背景をずらす。色収差・ハイライト・すりガラス・ガラスの色。

use znd_plugin_sdk::color::{from_display, to_display};
use znd_plugin_sdk::*;

fn convex(x: f64) -> f64 {
    (1.0 - (1.0 - x).powi(2)).clamp(0.0, 1.0).sqrt()
}
fn i_func(x: f64) -> f64 {
    1.0 - convex(x.clamp(0.0, 1.0))
}
fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn render(f: &mut Frame) -> FxResult {
    let Some(bg) = f.layer_image(0) else { return Err("背景のレイヤーを選んでください") };
    let q = |i: usize| f.f(i);
    let (bezel, thick, distort, aberr) = (q(1) / 1000.0, q(2) / 1000.0, q(3) / 1000.0, q(4) / 1000.0);
    let tint = to_display(color::premul(f.color(5), 1.0));
    let density = q(6) / 100.0;
    let (spec_ang, spec) = (q(7).to_radians(), q(8) / 100.0);
    let (frost, frost_blur, frost_start, frost_edge) = (f.b(9), q(10) / 1000.0, q(11) / 100.0, q(12) / 100.0);
    let (w, h) = (f.width, f.height);
    let (wf, hf) = (w as f64, h as f64);
    let aspect = wf / hf;
    // 距離場: 形のアルファを、ベゼル幅（画面の高さに対する割合）で箱型にぼかしたもの
    let mask = Image { w, h, data: f.rgba.chunks_exact(4).flat_map(|p| [0.0, 0.0, 0.0, p[3]]).collect() };
    let mut field = mask.clone();
    field.box_blur((bezel * hf).round().max(0.0) as usize);
    let disp_bg: Vec<[f64; 4]> = bg.data.chunks_exact(4).map(|p| { let d = to_display([p[0], p[1], p[2], p[3]]); [(d[0] * d[3]) as f64, (d[1] * d[3]) as f64, (d[2] * d[3]) as f64, d[3] as f64] }).collect();
    let bg_disp = Image { w: bg.w, h: bg.h, data: disp_bg.iter().flat_map(|c| c.map(|v| v as f32)).collect() };
    let mut frosted = bg_disp.clone();
    if frost && frost_blur > 0.0 {
        frosted.gaussian_blur((frost_blur * hf * 0.4) as f32);
    }
    let fa = |u: f64, v: f64| field.sample_clamped((u.clamp(0.0, 1.0) * wf) as f32, (v.clamp(0.0, 1.0) * hf) as f32)[3] as f64;
    let sbg = |img: &Image, u: f64, v: f64| img.sample_clamped((u.clamp(0.0, 1.0) * wf) as f32, (v.clamp(0.0, 1.0) * hf) as f32);
    let bezel_w = bezel.max(1e-5);
    let (lx, ly) = (spec_ang.cos(), -spec_ang.sin());
    let mut out = Image::new(w, h);
    out.map(|x, y, _| {
        let (u, v) = ((x as f64 + 0.5) / wf, (y as f64 + 0.5) / hf);
        let mask_a = mask.px(x, y)[3] as f64;
        if mask_a <= 0.0 {
            return [0.0; 4];
        }
        let xx = ((fa(u, v) - 0.5) * 2.0).clamp(0.0, 1.0);
        let height = i_func(xx);
        let delta = 0.002;
        let slope = (i_func((xx * bezel_w + delta) / bezel_w) - height) / delta;
        let e = (bezel_w * 0.08).max(1.0 / hf);
        let (gx, gy) = (fa(u + e, v) - fa(u - e, v), fa(u, v + e) - fa(u, v - e));
        let gl = gx.hypot(gy);
        let dir = if gl > 1e-6 { (-gx / gl, -gy / gl) } else { (0.0, 0.0) };
        let gnorm = (gl / (e / bezel_w).max(1e-6)).clamp(0.0, 1.0);
        let total = (height * slope * thick * distort * gnorm).clamp(-0.5, 0.5);
        let d = (dir.0 * total / aspect, dir.1 * total);
        let cr = sbg(&bg_disp, u + d.0 * (1.0 - aberr), v + d.1 * (1.0 - aberr));
        let cg = sbg(&bg_disp, u + d.0, v + d.1);
        let cb = sbg(&bg_disp, u + d.0 * (1.0 + aberr), v + d.1 * (1.0 + aberr));
        let mut col = [cr[0] as f64, cg[1] as f64, cb[2] as f64];
        if frost && frost_blur > 0.0 {
            let b = sbg(&frosted, u + d.0, v + d.1);
            let m = smoothstep(frost_start, frost_start + frost_edge.max(0.02), xx);
            for k in 0..3 {
                col[k] += (b[k] as f64 - col[k]) * m;
            }
        }
        let orig_a = sbg(&bg_disp, u, v)[3] as f64;
        if density > 0.0 {
            let path = 1.0 - height;
            for k in 0..3 {
                col[k] *= (-(1.0 - tint[k] as f64) * density * 3.5 * path).exp();
            }
        }
        let uu = 1.0 - xx;
        let dz = (uu / (1.0 - uu * uu).max(1e-4).sqrt()).min(8.0);
        let n = (dz * dir.0, dz * dir.1, 1.0);
        let nz = 1.0 / (n.0 * n.0 + n.1 * n.1 + 1.0).sqrt();
        let fres = (1.0 - nz.clamp(0.0, 1.0)).powi(5);
        let nl = dir.0 * lx + dir.1 * ly;
        let lobe = nl.clamp(0.0, 1.0).powi(2) + (-nl).clamp(0.0, 1.0).powi(2) * 0.5;
        for k in 0..3 {
            col[k] += fres * lobe * spec;
        }
        let a = (mask_a * orig_a) as f32;
        // col はプリマルチプライドの表示値（背景が不透明なら表示値そのもの）
        let st = |c: f64| if orig_a > 0.0 { (c / orig_a).clamp(0.0, 1.0) as f32 } else { 0.0 };
        from_display([st(col[0]), st(col[1]), st(col[2]), a])
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "Tettu0530（znd 移植: znd-plugins）", version: "3.1.2", license: "MIT",
    effects: [
        {
            id: "tettu0530.LiquidGlass", name: "Liquid Glass", category: "スタイライズ", flags: THREAD_SAFE,
            params: [
                layer(c"background", c"背景レイヤー"),
                float(c"bezel", c"ベゼル幅", 70.0, 0.0, 100.0),
                float(c"thickness", c"厚み", 20.0, 0.0, 50.0),
                float(c"distort", c"歪み", 350.0, 0.0, 500.0),
                float(c"aberr", c"色収差", 100.0, 0.0, 100.0),
                color_hex(c"tint", c"色", 0xffffff),
                float(c"density", c"濃度", 0.0, 0.0, 100.0),
                float(c"spec_angle", c"ハイライトの角度", 123.0, -180.0, 180.0),
                float(c"spec", c"ハイライトの強さ", 100.0, 0.0, 200.0),
                boolean(c"frost", c"すりガラス", false),
                float(c"frost_blur", c"ぼかし量", 30.0, 0.0, 100.0),
                float(c"frost_start", c"ぼかし開始", 0.0, 0.0, 100.0),
                float(c"frost_edge", c"ぼかし遷移", 50.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}

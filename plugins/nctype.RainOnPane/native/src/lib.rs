//! RainOnPane（nctype）の移植: 窓ガラスに付いた雨粒と、流れ落ちる雨筋。屈折・曇り・すりガラス・四隅の変形。
//! 調整レイヤーに掛けて、下の絵をガラス越しに見せる。計算は元のシェーダーと同じ（座標はレイヤー座標の px）。

use znd_plugin_sdk::*;

type V = (f64, f64);

fn fract(v: f64) -> f64 {
    v - v.floor()
}
fn smooth(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
fn step(e: f64, x: f64) -> f64 {
    if x >= e {
        1.0
    } else {
        0.0
    }
}
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
fn norm(v: V) -> V {
    let l = v.0.hypot(v.1).max(1e-12);
    (v.0 / l, v.1 / l)
}

struct C {
    seed: f64,
    time: f64,
    natural: f64,
    rnd: f64,
}

impl C {
    fn hash12(&self, x: f64, y: f64) -> f64 {
        let (mut a, mut b, mut c) = (fract(x * 0.1031), fract(y * 0.1031), fract(x * 0.1031));
        let k = self.seed * 0.001 + 33.33;
        let d = a * (b + k) + b * (c + k) + c * (a + k);
        a += d;
        b += d;
        c += d;
        fract((a + b) * c)
    }
    fn hash22(&self, x: f64, y: f64) -> V {
        let (mut a, mut b, mut c) = (fract(x * 0.1031), fract(y * 0.1030), fract(x * 0.0973));
        let k = self.seed * 0.001 + 33.33;
        let d = a * (b + k) + b * (c + k) + c * (a + k);
        a += d;
        b += d;
        c += d;
        (fract((a + a) * c), fract((b + c) * b))
    }
    fn hash1(&self, n: f64) -> f64 {
        fract((n * 12.9898 + self.seed * 78.233).sin() * 43758.5453)
    }
    /// (被覆, 縁, 法線)
    fn drops(&self, px: V, cell: f64, rs: f64, density: f64, rate: f64, ls: f64) -> (f64, f64, V) {
        let (cx, cy) = ((px.0 / cell).floor(), (px.1 / cell).floor());
        let (mut bm, mut br, mut bn) = (0.0, 0.0, (0.0, 0.0));
        for oy in -1..=1 {
            for ox in -1..=1 {
                let g = (cx + ox as f64, cy + oy as f64);
                let enabled = step(self.hash12(g.0 + ls, g.1 + ls), density) * step(0.0001, density);
                let raw = self.time * rate * lerp(0.72, 1.33, self.hash12(g.0 + ls + 19.0, g.1 + ls + 19.0)) + self.hash12(g.0 + ls + 7.0, g.1 + ls + 7.0);
                let (epoch, life) = (raw.floor(), fract(raw));
                let key = (g.0 + ls + epoch * 37.17, g.1 + ls + epoch * 91.73);
                let j = self.hash22(key.0 + 2.0, key.1 + 2.0);
                let mut sz = self.hash12(key.0 + 11.0, key.1 + 11.0);
                sz *= sz;
                let radius = (rs * lerp(0.22, 1.72, sz)).max(1.25);
                let center = ((g.0 + 0.12 + j.0 * 0.76) * cell, (g.1 + 0.12 + j.1 * 0.76) * cell);
                let shape = (self.hash12(key.0 + 23.0, key.1 + 23.0) - 0.5) * self.rnd;
                let asp = (1.0 + shape * 0.32, 1.0 - shape * 0.16);
                let q = ((px.0 - center.0) / (radius * asp.0).max(0.5), (px.1 - center.1) / (radius * asp.1).max(0.5));
                let d = q.0.hypot(q.1).min((q.0 / 0.88).hypot((q.1 - 0.18) / 1.08));
                let aa = 1.35 / radius.max(1.5);
                let vis = smooth(0.0, 0.055, life) * (1.0 - smooth(0.88, 1.0, life));
                let m = (1.0 - smooth(1.0 - aa, 1.0 + aa, d)) * vis * enabled;
                let rim = smooth(0.68, 0.96, d) * (1.0 - smooth(0.98, 1.0 + aa, d)) * m;
                if m > bm {
                    bm = m;
                    br = rim;
                    bn = norm((q.0 / asp.0 + 0.0001, q.1 / asp.1 + 0.0001));
                }
            }
        }
        (bm, br, bn)
    }
}

fn runner_path(y: f64, key: f64, width: f64, natural: f64) -> f64 {
    ((y * 0.018 + key * 1.71).sin() * 0.46 + (y * 0.047 - key * 2.13).sin() * 0.19 + (y * 0.0065 + key * 0.37).sin() * 0.35) * width * 0.72 * natural
}
fn width_noise(y: f64, key: f64) -> f64 {
    1.0 + 0.24 * (y * 0.053 + key).sin() + 0.11 * (y * 0.137 - key * 2.0).sin()
}
fn retained(age: f64, hold: f64, rec: f64) -> f64 {
    1.0 - smooth(hold, hold + rec.max(0.01), age)
}

/// 4 点（左上・右上・右下・左下）への射影変換 [a b c; d e f; g h i]
fn homography(p: [V; 4]) -> Option<[f64; 9]> {
    let [(x0, y0), (x1, y1), (x2, y2), (x3, y3)] = p;
    let (dx1, dx2, dy1, dy2) = (x1 - x2, x3 - x2, y1 - y2, y3 - y2);
    let (dx3, dy3) = (x0 - x1 + x2 - x3, y0 - y1 + y2 - y3);
    let den = dx1 * dy2 - dx2 * dy1;
    if den.abs() < 1e-6 {
        return None;
    }
    let g = (dx3 * dy2 - dx2 * dy3) / den;
    let k = (dx1 * dy3 - dx3 * dy1) / den;
    Some([x1 - x0 + g * x1, x3 - x0 + k * x3, x0, y1 - y0 + g * y1, y3 - y0 + k * y3, y0, g, k, 1.0])
}
fn inverse(m: [f64; 9]) -> Option<[f64; 9]> {
    let [a, b, c, d, e, f, g, k, i] = m;
    let det = a * (e * i - f * k) - b * (d * i - f * g) + c * (d * k - e * g);
    if det.abs() < 1e-6 {
        return None;
    }
    let r = 1.0 / det;
    Some([(e * i - f * k) * r, (c * k - b * i) * r, (b * f - c * e) * r, (f * g - d * i) * r, (a * i - c * g) * r, (c * d - a * f) * r, (d * k - e * g) * r, (b * g - a * k) * r, (a * e - b * d) * r])
}
fn apply(m: &[f64; 9], p: V) -> V {
    let z = m[6] * p.0 + m[7] * p.1 + m[8];
    let z = if z.abs() < 1e-5 { 1e-5 } else { z };
    ((m[0] * p.0 + m[1] * p.1 + m[2]) / z, (m[3] * p.0 + m[4] * p.1 + m[5]) / z)
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let q = |i: usize| f.f(i);
    let (w, h) = (f.width, f.height);
    let canvas = (w as f64 / s, h as f64 / s);
    let src = f.input();
    let sample = |img: &Image, u: f64, v: f64| img.sample_clamped((u.clamp(0.0, 1.0) * w as f64) as f32, (v.clamp(0.0, 1.0) * h as f64) as f32);
    // 曇り: 下の絵をぼかしたもの
    let (vblur, vstr) = (q(23), q(24));
    let vapor_radius = vblur * (vstr / 100.0).max(1.0);
    let vapor = if vblur > 0.0 && vstr > 0.0 {
        let mut v = src.clone();
        v.gaussian_blur((vapor_radius / 6.0 * 1.6 * s) as f32);
        Some(v)
    } else {
        None
    };
    let c = C { seed: q(20), time: f.layer_time(), natural: (q(18) / 125.0).clamp(0.0, 1.6), rnd: (q(18) / 125.0).clamp(0.0, 1.6) };
    let transform = f.b(25);
    let corners = [f.point(27), f.point(28), f.point(29), f.point(30)].map(|p| (p[0] / s, p[1] / s));
    let (fwd, inv) = if transform {
        match homography(corners).and_then(|m| inverse(m).map(|i| (m, i))) {
            Some(x) => x,
            None => return Ok(()),
        }
    } else {
        let full = [(0.0, 0.0), (canvas.0, 0.0), (canvas.0, canvas.1), (0.0, canvas.1)];
        let m = homography(full).unwrap();
        (m, inverse(m).unwrap())
    };
    let feather = q(26) / canvas.0.min(canvas.1).max(1.0);
    let density = if q(0) <= 165.0 { q(0) / 220.0 } else if q(0) <= 300.0 { lerp(0.75, 1.0, (q(0) - 165.0) / 135.0) } else { lerp(1.0, 1.25, (q(0) - 300.0) / 200.0) };
    let micro_cell = lerp(38.0, 13.0, density);
    let (micro_r, att_cell, att_r) = (q(1) * 0.026, micro_cell * 3.35, q(1) * 0.105);
    let rate = q(2) * 0.000203;
    let prob = if q(4) <= 42.0 { q(4) / 70.0 } else { lerp(0.6, 1.0, (q(4) - 42.0) / 58.0) };
    let motion = step(0.001, q(7));
    let mut out = Image::new(w, h);
    out.map(|x, y, _| {
        let screen = ((x as f64 + 0.5) / s, (y as f64 + 0.5) / s);
        let uv = (screen.0 / canvas.0, screen.1 / canvas.1);
        let t_uv = apply(&inv, screen);
        let pane_uv = if transform { t_uv } else { uv };
        let edge_d = pane_uv.0.min(pane_uv.1).min(1.0 - pane_uv.0).min(1.0 - pane_uv.1);
        let pane_mask = if !transform { 1.0 } else if feather > 1e-5 { smooth(0.0, feather, edge_d) } else { step(0.0, edge_d) };
        let px = (pane_uv.0 * canvas.0, pane_uv.1 * canvas.1);
        let m1 = c.drops(px, micro_cell, micro_r, density * 0.92, rate * 0.42, 13.0);
        let m2 = c.drops(px, att_cell, att_r, density * 0.7, rate, 417.0);
        let use2 = m1.0 <= m2.0;
        let mut drop_mask = m1.0.max(m2.0);
        let mut drop_rim = if use2 { m2.1 } else { m1.1 };
        let dn = norm(if use2 { (m2.2 .0 + 1e-4, m2.2 .1 + 1e-4) } else { (m1.2 .0 + 1e-4, m1.2 .1 + 1e-4) });
        let (mut rmask, mut redge, mut vclear, mut dclear, mut rn) = (0.0f64, 0.0, 0.0f64, 0.0f64, (0.0, 0.0));
        for i in 0..14 {
            let id = 1000.0 + i as f64;
            let speed = lerp(0.58, 1.55, c.hash1(id + 8.0));
            let travel_n = canvas.1 + q(5) + 500.0;
            let raw = c.hash1(id + 4.0) + c.time * (45.0 + q(7) * 3.1) * speed * motion / travel_n.max(1.0);
            let (epoch, phase) = (raw.floor(), fract(raw));
            let key = id + epoch * 173.317;
            let enabled = step(c.hash1(key + 90.0), prob) * step(0.0001, prob) * motion;
            let mut lr = c.hash1(key + 2.0);
            lr *= lr;
            let len = (q(5) * lerp(0.12, 2.65, lr)).max(30.0);
            let half = len * 0.5;
            let bw = (q(6) * lerp(0.45, 1.9, c.hash1(key + 5.0))).max(2.0);
            let rx = lerp(-0.08, 1.08, c.hash1(key + 3.0)) * canvas.0;
            let center_y = phase * (canvas.1 + len + 500.0) - half - 250.0;
            let path = runner_path(px.1, key, bw, c.natural);
            let ly = px.1 - center_y;
            let cy = ly.clamp(-half, half);
            let along = ((cy + half) / len.max(1.0)).clamp(0.0, 1.0);
            let taper = lerp(0.16, 1.0, smooth(0.0, 0.8, along));
            let lw = (bw * taper * width_noise(cy, key)).max(1.5);
            let (dx, dy) = ((px.0 - rx - path) / lw, (ly - cy) / bw.max(1.0));
            let dist = dx.hypot(dy);
            let aa = 1.4 / lw.max(2.0);
            let m = (1.0 - smooth(1.0 - aa, 1.0 + aa, dist)) * enabled;
            if m > rmask {
                rmask = m;
                redge = smooth(0.54, 0.96, dx.abs()) * m;
                rn = norm((dx + 1e-4, dy * 0.12 + 1e-4));
            }
            let head = center_y + half;
            let clear_scale = q(10) / 100.0;
            let cap = (q(13) / 100.0).clamp(0.0, 1.0);
            let lane = |x_: f64, path_: f64, tw: f64| {
                let vw = tw * clear_scale;
                let dw = vw * (1.0 + cap * 0.35);
                let fe = 3.0 + tw * cap;
                let d = (px.0 - x_ - path_).abs();
                (1.0 - smooth(vw, vw + fe, d), 1.0 - smooth(dw, dw + fe, d))
            };
            let tw = (bw * width_noise(px.1, key)).max(1.5);
            let (vl, dl) = lane(rx, path, tw);
            let pf = (tw * 1.6).max(8.0);
            let passed = smooth(-pf, pf, px.1) * (1.0 - smooth(head - pf, head + pf, px.1));
            vclear = vclear.max(vl * passed * enabled * (q(11) / 100.0).clamp(0.0, 1.0));
            dclear = dclear.max(dl * passed * enabled * (q(12) / 100.0).clamp(0.0, 1.0));
            let cycle = travel_n / ((45.0 + q(7) * 3.1) * speed).max(0.01);
            for hist in 1..=12 {
                let hk = id + (epoch - hist as f64) * 173.317;
                let he = step(c.hash1(hk + 90.0), prob) * step(0.0001, prob) * motion;
                if he == 0.0 {
                    continue;
                }
                let age = (phase + hist as f64 - 1.0) * cycle;
                let win = 1.0 - smooth(cycle * 11.0, cycle * 12.0, age);
                let vm = retained(age, q(14), q(15)) * win;
                let dm = retained(age, q(14), q(16)) * win;
                let hw = (q(6) * lerp(0.45, 1.9, c.hash1(hk + 5.0))).max(2.0);
                let hx = lerp(-0.08, 1.08, c.hash1(hk + 3.0)) * canvas.0;
                let hp = runner_path(px.1, hk, hw, c.natural);
                let (hv, hd) = lane(hx, hp, (hw * width_noise(px.1, hk)).max(1.5));
                vclear = vclear.max(hv * he * vm * (q(11) / 100.0).clamp(0.0, 1.0));
                dclear = dclear.max(hd * he * dm * (q(12) / 100.0).clamp(0.0, 1.0));
            }
        }
        let remain = 1.0 - dclear.clamp(0.0, 1.0);
        drop_mask *= remain * pane_mask;
        drop_rim *= remain * pane_mask;
        rmask *= pane_mask;
        redge *= pane_mask;
        let opacity = (q(19) / 100.0).clamp(0.0, 1.0);
        let wet = (q(17) / 100.0).clamp(0.0, 2.0);
        let mut off = (dn.0 * drop_mask * q(3) * 0.095 * wet + rn.0 * rmask * q(8) * 0.22 * wet, dn.1 * drop_mask * q(3) * 0.095 * wet + rn.1 * rmask * q(8) * 0.22 * wet);
        let film = (px.1 * 0.009 + (px.0 * 0.006 + c.seed).sin() * 2.0).sin();
        off.0 += film * wet * 1.15 * (1.0 - rmask) * drop_mask;
        let disp_pane = (((px.0 + off.0) / canvas.0).clamp(0.0, 1.0), ((px.1 + off.1) / canvas.1).clamp(0.0, 1.0));
        let disp_screen = if transform { apply(&fwd, (disp_pane.0, disp_pane.1)) } else { (screen.0 + off.0, screen.1 + off.1) };
        let duv = ((disp_screen.0 / canvas.0).clamp(0.0, 1.0), (disp_screen.1 / canvas.1).clamp(0.0, 1.0));
        let base = sample(&src, uv.0, uv.1);
        let frost_r = q(21).max(0.0) * (q(22) / 100.0).max(1.0);
        let frost_mix = (q(22) / 100.0).clamp(0.0, 1.0) * step(0.01, frost_r) * pane_mask;
        let mut dry = base;
        if frost_mix > 1e-4 {
            let (fx, fy) = (frost_r / canvas.0, frost_r / canvas.1);
            let g = |a: f64, b: f64| sample(&src, uv.0 + a, uv.1 + b);
            let (ax, bx, cx, dx2) = (g(fx, 0.0), g(-fx, 0.0), g(0.0, fy), g(0.0, -fy));
            let (e, f2, gg, hh) = (g(fx, fy), g(fx, -fy), g(-fx, fy), g(-fx, -fy));
            let fr: [f32; 4] = std::array::from_fn(|k| base[k] * 0.12 + (ax[k] + bx[k] + cx[k] + dx2[k]) * 0.12 + (e[k] + f2[k] + gg[k] + hh[k]) * 0.1);
            dry = std::array::from_fn(|k| base[k] + (fr[k] - base[k]) * frost_mix as f32);
        }
        if let Some(v) = &vapor {
            let vm = ((vstr / 100.0).clamp(0.0, 1.0) * (1.0 - vclear.clamp(0.0, 1.0)) * pane_mask) as f32;
            if vm > 1e-4 {
                let vv = sample(v, uv.0, uv.1);
                dry = std::array::from_fn(|k| dry[k] + (vv[k] - dry[k]) * vm);
            }
        }
        let refr = sample(&src, duv.0, duv.1);
        let cover = drop_mask.max(rmask).clamp(0.0, 1.0);
        let bp = q(9) * 0.12 * cover;
        let mut water = refr;
        if bp > 1e-4 {
            let (bx, by) = (bp / canvas.0, bp / canvas.1);
            let g = |a: f64, b: f64| sample(&src, duv.0 + a, duv.1 + b);
            let (a1, a2, a3, a4) = (g(bx, 0.0), g(-bx, 0.0), g(0.0, by), g(0.0, -by));
            water = std::array::from_fn(|k| refr[k] * 0.44 + (a1[k] + a2[k] + a3[k] + a4[k]) * 0.14);
        }
        let vis = wet.clamp(0.0, 1.0);
        let wm = (cover * opacity * vis).clamp(0.0, 1.0) as f32;
        let depth = q(31) / 100.0;
        let mut rgb: [f32; 3] = std::array::from_fn(|k| dry[k] + (water[k] - dry[k]) * wm);
        let dark = (1.0 - drop_rim * depth * opacity * vis * 0.24) * (1.0 - redge * depth * opacity * vis * 0.12);
        let light = drop_mask * dn.1.clamp(0.0, 1.0) * 0.035 * depth * opacity * vis;
        for v in &mut rgb {
            // 明るさの加算は表示値で行う元の式に合わせ、アルファに比例させる
            *v = (*v * dark as f32 + light as f32 * base[3]).max(0.0);
        }
        [rgb[0], rgb[1], rgb[2], base[3]]
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "nctype（znd 移植: znd-plugins）", version: "1.1.0", license: "MIT",
    effects: [
        {
            id: "nctype.RainOnPane", name: "窓の雨（RainOnPane）", category: "スタイライズ", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"drop_density", c"雨粒の密度", 165.0, 0.0, 500.0),
                float(c"drop_size", c"雨粒のサイズ", 100.0, 10.0, 300.0),
                float(c"drop_motion", c"雨粒の更新速度", 35.0, 0.0, 200.0),
                float(c"drop_refraction", c"雨粒屈折", 38.0, 0.0, 150.0),
                float(c"runner_count", c"流れの数", 42.0, 0.0, 100.0),
                float(c"runner_length", c"流れの長さ", 900.0, 100.0, 2400.0),
                float(c"runner_width", c"流れの太さ", 10.0, 2.0, 150.0),
                float(c"runner_speed", c"流速", 105.0, 0.0, 500.0),
                float(c"runner_refraction", c"流れ屈折", 165.0, 0.0, 300.0),
                float(c"softness", c"水のボケ", 28.0, 0.0, 100.0),
                float(c"clear_width", c"雨筋幅", 115.0, 50.0, 400.0),
                float(c"vapor_removal", c"曇り除去", 80.0, 0.0, 100.0),
                float(c"drop_removal", c"雨粒除去", 100.0, 0.0, 100.0),
                float(c"edge_capture", c"縁の巻込み", 15.0, 0.0, 100.0),
                float(c"trail_hold", c"雨筋保持時間", 8.0, 0.0, 30.0),
                float(c"vapor_recovery", c"曇りの回復", 18.0, 0.1, 60.0),
                float(c"drop_regen", c"雨粒の再付着", 24.0, 0.1, 120.0),
                float(c"wetness", c"濡れ", 100.0, 0.0, 200.0),
                float(c"randomness", c"ランダム", 125.0, 0.0, 200.0),
                float(c"opacity", c"透明度", 100.0, 0.0, 100.0),
                float(c"seed", c"シード", 2846.0, 0.0, 9999.0),
                float(c"frost_blur", c"すりガラスぼかし", 0.0, 0.0, 80.0),
                float(c"frost_strength", c"すりガラス強度", 0.0, 0.0, 200.0),
                float(c"vapor_blur", c"曇りぼかし", 0.0, 0.0, 120.0),
                float(c"vapor_strength", c"曇り強度", 0.0, 0.0, 200.0),
                boolean(c"transform", c"四隅変形", false),
                float(c"feather", c"境界ぼかし", 4.0, 0.0, 100.0),
                point(c"tl", c"左上", 0.0, 0.0),
                point(c"tr", c"右上", 1.0, 0.0),
                point(c"br", c"右下", 1.0, 1.0),
                point(c"bl", c"左下", 0.0, 1.0),
                float(c"depth", c"立体感", 115.0, 0.0, 200.0),
            ],
            render: render,
        },
    ]
}

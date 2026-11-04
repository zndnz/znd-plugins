//! SomethingLikeGlitch（nctype）の移植: ブロック・ライン状の断片をずらすティアリング、カラーブロック、色ズレのグリッチ。
//! 周期・ランダムのバースト、エッジだけに掛けるモードも元と同じ。計算は元のシェーダー（プリマルチプライドの表示値）と同じ。

use znd_plugin_sdk::color::{from_display, to_display};
use znd_plugin_sdk::*;

struct P {
    w: f32,
    h: f32,
    strength: f32,
    time: f32,
    disp: f32,
    coverage: f32,
    block: f32,
    stretch: f32,
    step_mode: bool,
    seed: u32,
    angle: f32,
    gate: f32,
    flash_ratio: f32,
    flash_on: bool,
    colors: [[f32; 3]; 3],
    missing: bool,
    line_ratio: f32,
    second: f32,
    edge: usize,
}

fn scramble(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846ca68b);
    x ^ (x >> 16)
}

impl P {
    fn rnd(&self, c: (i32, i32), tick: i32, stream: u32) -> f32 {
        let h = scramble(c.0 as u32 ^ scramble(c.1 as u32));
        let h = scramble(h ^ scramble(tick as u32) ^ scramble(self.seed.wrapping_add(stream)));
        (h >> 8) as f32 / 16777216.0
    }
    fn shape_rnd(&self, c: (i32, i32), slot: i32, layer: i32, ch: u32) -> f32 {
        let stream = (1001 + layer as u32 * 1009).wrapping_add(((slot + 1) as u32).wrapping_mul(43)).wrapping_add(ch);
        if self.step_mode {
            self.rnd(c, self.time.floor() as i32, stream)
        } else {
            let tick = self.time.floor() as i32;
            let f = self.time - self.time.floor();
            let f = f * f * (3.0 - 2.0 * f);
            let (a, b) = (self.rnd(c, tick, stream), self.rnd(c, tick + 1, stream));
            a + (b - a) * f
        }
    }
    fn unit(&self, layer: i32) -> f32 {
        (self.w.min(self.h) * self.block * if layer == 0 { 1.0 } else { 0.55 }).max(1.0)
    }
    fn pitch(&self, layer: i32) -> (f32, f32) {
        let u = self.unit(layer);
        (u * self.stretch.max(1.0), u * 2.0)
    }
    fn amplitude(&self, layer: i32) -> f32 {
        self.w.min(self.h) * self.disp * self.strength * self.gate * if layer == 0 { 1.0 } else { self.second }
    }
    fn line_slots(&self) -> i32 {
        if self.line_ratio <= 0.0 {
            0
        } else if self.line_ratio >= 1.0 {
            8
        } else {
            ((self.line_ratio * 8.0 + 0.5).floor() as i32).clamp(1, 6)
        }
    }
    fn enabled(&self, c: (i32, i32), layer: i32) -> bool {
        self.coverage >= 1.0 || self.shape_rnd(c, -1, layer, 0) < self.coverage
    }
    fn bounds(&self, c: (i32, i32), slot: i32, layer: i32) -> [f32; 4] {
        let u = self.unit(layer);
        let p = self.pitch(layer);
        let jx = self.shape_rnd(c, slot, layer, 1) - 0.5;
        let jy = self.shape_rnd(c, slot, layer, 2) - 0.5;
        let cx = self.w * 0.5 + (c.0 as f32 + jx) * p.0;
        let cy = self.h * 0.5 + (c.1 as f32 + jy) * p.1;
        let a = self.shape_rnd(c, slot, layer, 3);
        let b = self.shape_rnd(c, slot, layer, 4);
        let kind = if slot >= 8 - self.line_slots() { 2 } else { slot & 1 };
        let (sw, sh) = match kind {
            0 => {
                let s = (u * (0.45 + 0.55 * a)).max(1.0);
                (s, s)
            }
            1 => {
                let w = (p.0 * (0.3 + 0.7 * a)).max(1.0);
                (w, w.min((u * (0.18 + 0.52 * b)).max(1.0)))
            }
            _ => ((p.0 * (0.4 + 0.6 * a)).max(1.0), ((u * 0.04).min(2.0) * (0.5 + 0.5 * b)).max(1.0)),
        };
        [cx - sw * 0.5, cy - sh * 0.5, cx + sw * 0.5, cy + sh * 0.5]
    }
    fn shift(&self, c: (i32, i32), slot: i32, layer: i32) -> f32 {
        (self.shape_rnd(c, slot, layer, 5) * 2.0 - 1.0) * self.amplitude(layer)
    }
    fn region(&self, x: f32, y: f32, layer: i32) -> (f32, f32) {
        let p = self.pitch(layer);
        let ax = ((x - self.w * 0.5) / p.0).floor() as i32;
        let ay = ((y - self.h * 0.5) / p.1).floor() as i32;
        let (mut amount, mut active) = (0.0f32, 0.0f32);
        for cy in ay - 1..=ay + 1 {
            for cx in ax - 1..=ax + 1 {
                if !self.enabled((cx, cy), layer) {
                    continue;
                }
                for slot in 0..8 {
                    if inside(x, y, self.bounds((cx, cy), slot, layer)) {
                        amount = amount.max((self.shape_rnd((cx, cy), slot, layer, 5) * 2.0 - 1.0).abs());
                        active = 1.0;
                    }
                }
            }
        }
        (amount, active)
    }
}

fn inside(x: f32, y: f32, b: [f32; 4]) -> bool {
    x >= b[0] && y >= b[1] && x < b[2] && y < b[3]
}

struct Src<'a> {
    img: &'a Image,
    core: Option<&'a [bool]>,
    edge: usize,
}

impl Src<'_> {
    fn sample(&self, x: f32, y: f32) -> [f32; 4] {
        match self.core {
            None => match self.edge {
                1 => self.img.sample_clamped(x, y),
                2 => self.img.sample_wrapped(x, y),
                _ => self.img.sample(x, y),
            },
            Some(core) => {
                // エッジのみ: 中身（芯）の画素は使わない
                let (w, h) = (self.img.w as isize, self.img.h as isize);
                let t = |px: isize, py: isize| -> [f32; 4] {
                    if px < 0 || py < 0 || px >= w || py >= h || core[(py * w + px) as usize] {
                        [0.0; 4]
                    } else {
                        self.img.px(px as usize, py as usize)
                    }
                };
                let (u, v) = (x - 0.5, y - 0.5);
                let (qx, qy) = (u.floor() as isize, v.floor() as isize);
                let (fx, fy) = (u - u.floor(), v - v.floor());
                let (a, b, c, d) = (t(qx, qy), t(qx + 1, qy), t(qx, qy + 1), t(qx + 1, qy + 1));
                std::array::from_fn(|k| (a[k] + (b[k] - a[k]) * fx) * (1.0 - fy) + (c[k] + (d[k] - c[k]) * fx) * fy)
            }
        }
    }
}

fn tear(p: &P, src: &Image, core: Option<&[bool]>, orig: &Image, layer: i32) -> Image {
    let s = Src { img: src, core, edge: p.edge };
    let mut out = Image::new(src.w, src.h);
    let (dx, dy) = (p.angle.cos(), p.angle.sin());
    let pitch = p.pitch(layer);
    let amp = p.amplitude(layer);
    out.map(|x, y, _| {
        if let Some(c) = core {
            if c[y * src.w + x] {
                return orig.px(x, y);
            }
        }
        let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
        let original = s.sample(px, py);
        if amp <= 1e-5 || p.coverage <= 0.0 {
            return original;
        }
        let (rx, ry) = (pitch.0 + dx.abs() * amp, pitch.1 + dy.abs() * amp);
        let (bx, by) = (((px - p.w * 0.5 - rx) / pitch.0).ceil() as i32, ((py - p.h * 0.5 - ry) / pitch.1).ceil() as i32);
        let (ex, ey) = (((px - p.w * 0.5 + rx) / pitch.0).floor() as i32, ((py - p.h * 0.5 + ry) / pitch.1).floor() as i32);
        let mut hole = false;
        let mut frag = [0.0f32; 4];
        for cy in by..=ey {
            for cx in bx..=ex {
                let c = (cx, cy);
                if !p.enabled(c, layer) {
                    continue;
                }
                for slot in 0..8 {
                    let d = p.shift(c, slot, layer);
                    if d.abs() <= 1e-5 {
                        continue;
                    }
                    let b = p.bounds(c, slot, layer);
                    if p.missing && inside(px, py, b) {
                        hole = true;
                    }
                    let (sx, sy) = (px - dx * d, py - dy * d);
                    if !inside(sx, sy, b) {
                        continue;
                    }
                    if p.missing && (sx < 0.0 || sy < 0.0 || sx >= p.w || sy >= p.h) {
                        continue;
                    }
                    let mut fr = s.sample(sx, sy);
                    let tick = p.time.floor() as i32;
                    let stream = 700 + (slot as u32 + 1) * 37 + layer as u32 * 409;
                    if p.flash_on && p.rnd(c, tick, stream) < p.flash_ratio {
                        let ch = p.rnd(c, tick, stream + 1709);
                        let col = p.colors[if ch < 1.0 / 3.0 { 0 } else if ch < 2.0 / 3.0 { 1 } else { 2 }];
                        for k in 0..3 {
                            fr[k] = col[k] * fr[3];
                        }
                    }
                    for k in 0..4 {
                        frag[k] = fr[k] + frag[k] * (1.0 - fr[3]);
                    }
                }
            }
        }
        let base = if hole { [0.0; 4] } else { original };
        std::array::from_fn(|k| frag[k] + base[k] * (1.0 - frag[3]))
    });
    out
}

fn park_miller(index: i64, stream: i64, seed: i64) -> f64 {
    let mut n = ((index % 2147483646) + seed * 131 + stream * 8191).rem_euclid(2147483646) + 1;
    n = (n * 16807) % 2147483647;
    n = (n * 16807) % 2147483647;
    n as f64 / 2147483647.0
}

fn render(f: &mut Frame) -> FxResult {
    let q = |i: usize| f.f(i);
    let strength = q(0) / 100.0;
    let contour_only = f.b(8);
    if strength <= 0.0 || (contour_only && q(9) <= 0.0) {
        return Ok(());
    }
    let t = f.frame() as f64 / f.fps().max(0.001);
    let seed = q(15).floor() as i64;
    // バースト（周期・ランダム）
    let rhythm = f.i(3);
    let mut gate = 1.0;
    if rhythm != 0 {
        let (gap, dur, jit) = (q(4).max(0.02), q(5).max(0.01), q(6) / 100.0);
        gate = 0.0;
        if rhythm == 1 || jit <= 0.0 {
            let period = dur + gap;
            if t - (t / period).floor() * period < dur {
                gate = 1.0;
            }
        } else {
            let (mut start, mut ev) = (0.0, 0i64);
            while start <= t && ev < 1_000_000 {
                let d = dur * (1.0 + (park_miller(ev, 2, seed) - 0.5) * jit);
                let g = gap * (1.0 + (park_miller(ev, 1, seed) - 0.5) * jit);
                if t < start + d {
                    gate = 1.0;
                    break;
                }
                if t < start + d + g {
                    break;
                }
                start += d + g;
                ev += 1;
            }
        }
    }
    if gate == 0.0 {
        return Ok(());
    }
    let dc = |i: usize| { let d = to_display(color::premul(f.color(i), 1.0)); [d[0], d[1], d[2]] };
    let (w, h) = (f.width, f.height);
    let p = P {
        w: w as f32,
        h: h as f32,
        strength: strength as f32,
        time: (t * q(14)) as f32,
        disp: (q(10) / 100.0) as f32,
        coverage: (q(11) / 100.0) as f32,
        block: (q(12) / 100.0) as f32,
        stretch: q(13) as f32,
        step_mode: f.i(2) == 1,
        seed: seed as u32,
        angle: q(26).to_radians() as f32,
        gate: gate as f32,
        flash_ratio: (q(17) / 100.0) as f32,
        flash_on: f.b(16),
        colors: [dc(18), dc(19), dc(20)],
        missing: f.b(7),
        line_ratio: (q(27) / 100.0) as f32,
        second: (q(1) / 100.0) as f32,
        edge: f.i(28),
    };
    // 表示値（プリマルチプライド）にして計算する
    let input = f.input();
    let to_d = |img: &Image| {
        let mut o = img.clone();
        o.data.chunks_exact_mut(4).for_each(|px| {
            let d = to_display([px[0], px[1], px[2], px[3]]);
            px.copy_from_slice(&[d[0] * d[3], d[1] * d[3], d[2] * d[3], d[3]]);
        });
        o
    };
    let orig = to_d(&input);
    // エッジのみ: 内側（アルファの芯を幅だけ削ったもの、マス目で広げる）は元の絵のまま守る
    let core: Option<Vec<bool>> = contour_only.then(|| {
        let thr = (q(25) / 100.0) as f32;
        let seedm: Vec<f32> = orig.data.chunks_exact(4).map(|px| if px[3] > thr { 1.0 } else { 0.0 }).collect();
        let r = ((w.min(h) as f64 * q(9) / 100.0 + 0.5).floor().max(1.0)) as f32;
        let mut c = znd_plugin_sdk::image::erode_alpha(&seedm, w, h, r * std::f32::consts::SQRT_2);
        let grid = ((w.min(h) as f64 * q(12) / 200.0).min(r as f64 / 2.0) + 0.5).floor().max(1.0) as usize;
        if grid > 1 {
            let (ox, oy) = (w / 2, h / 2);
            let mut c2 = c.clone();
            for y in 0..h {
                for x in 0..w {
                    let st = ((x as isize - ox as isize).div_euclid(grid as isize) * grid as isize + ox as isize).max(0) as usize;
                    c2[y * w + x] = (st..(st + grid).min(w)).map(|i| c[y * w + i]).fold(0.0, f32::max);
                }
            }
            for y in 0..h {
                for x in 0..w {
                    let st = ((y as isize - oy as isize).div_euclid(grid as isize) * grid as isize + oy as isize).max(0) as usize;
                    c[y * w + x] = (st..(st + grid).min(h)).map(|i| c2[i * w + x]).fold(0.0, f32::max);
                }
            }
        }
        c.iter().map(|v| *v > 0.5).collect()
    });
    let core_ref = core.as_deref();
    let mut img = orig.clone();
    if p.disp > 0.0 && p.coverage > 0.0 {
        img = tear(&p, &img, core_ref, &orig, 0);
        if p.second > 0.0 {
            img = tear(&p, &img, core_ref, &orig, 1);
        }
    }
    let rgb_off = q(21) / 100.0;
    if rgb_off > 0.0 {
        let follow = f.b(22);
        let ang = if follow { p.angle } else { q(23).to_radians() as f32 };
        let rgb_type = f.i(24);
        let (ca, cb) = (dc(29), dc(30));
        let edge = if p.missing && p.disp > 0.0 && p.coverage > 0.0 { 0 } else { p.edge };
        let s = Src { img: &img, core: core_ref, edge };
        let short = p.w.min(p.h);
        let mut out = Image::new(w, h);
        out.map(|x, y, _| {
            if let Some(c) = core_ref {
                if c[y * w + x] {
                    return orig.px(x, y);
                }
            }
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let mut c = s.sample(px, py);
            let field = {
                let a = p.region(px, py, 0);
                if p.second > 0.0 {
                    let b = p.region(px, py, 1);
                    (a.0.max(b.0), a.1.max(b.1))
                } else {
                    a
                }
            };
            let fol = if follow { field.1 * (0.3 + 0.7 * field.0.abs()) } else { 1.0 };
            let k = short * rgb_off as f32 * p.strength * p.gate * fol;
            let (ox, oy) = (ang.cos() * k, ang.sin() * k);
            let pos = s.sample(px - ox, py - oy);
            let neg = s.sample(px + ox, py + oy);
            match rgb_type {
                0 => c = [pos[0], c[1], neg[2], c[3]],
                1 => c = [pos[0], neg[1], neg[2], c[3]],
                2 => c = [neg[0], pos[1], neg[2], c[3]],
                _ => {
                    for i in 0..3 {
                        let den = (ca[i] + cb[i]).max(1.0);
                        let (wa, wb) = (ca[i] / den, cb[i] / den);
                        c[i] = c[i] * (1.0 - wa - wb) + pos[i] * wa + neg[i] * wb;
                    }
                }
            }
            c[3] = c[3].max(pos[3]).max(neg[3]);
            c
        });
        img = out;
    }
    img.data.chunks_exact_mut(4).for_each(|px| {
        let a = px[3];
        let o = if a <= 0.0 { [0.0; 4] } else { from_display([(px[0] / a).clamp(0.0, 1.0), (px[1] / a).clamp(0.0, 1.0), (px[2] / a).clamp(0.0, 1.0), a.min(1.0)]) };
        px.copy_from_slice(&o);
    });
    f.set(img);
    Ok(())
}

plugin! {
    vendor: "nctype（znd 移植: znd-plugins）", version: "1.0.2", license: "MIT",
    effects: [
        {
            id: "nctype.SomethingLikeGlitch", name: "SomethingLikeGlitch", category: "スタイライズ", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"strength", c"強さ", 65.0, 0.0, 100.0),
                float(c"second_strength", c"第2層の強さ", 60.0, 0.0, 100.0),
                choice(c"pattern", c"ティアリングモード", 0, c"滑らか|ステップ"),
                choice(c"rhythm", c"タイミングモード", 0, c"連続|周期|ランダム"),
                float(c"burst_interval", c"間隔（秒）", 1.0, 0.02, 30.0),
                float(c"burst_duration", c"持続時間（秒）", 0.18, 0.01, 30.0),
                float(c"burst_randomness", c"ランダム度（%）", 60.0, 0.0, 100.0),
                boolean(c"tear_missing", c"ティア元を空ける", true),
                boolean(c"contour_only", c"エッジのみ", false),
                float(c"contour_width", c"エッジ幅（%）", 2.0, 0.0, 25.0),
                float(c"displacement", c"ズレ（%）", 8.0, 0.0, 100.0),
                float(c"coverage", c"カバー率（%）", 35.0, 0.0, 100.0),
                float(c"block_size", c"ブロックサイズ（%）", 5.0, 0.1, 100.0),
                float(c"stretch", c"横伸び", 8.0, 1.0, 40.0),
                float(c"evolution", c"変化速度", 8.0, 0.0, 60.0),
                float(c"seed", c"乱数シード", 1.0, 0.0, 99999.0),
                boolean(c"flash_enabled", c"カラーブロック", true),
                float(c"flash_strength", c"カラーブロックのカバー率（%）", 20.0, 0.0, 100.0),
                color_hex(c"flash_color", c"カラーブロックの色1", 0x36ebf2),
                color_hex(c"flash_color2", c"カラーブロックの色2", 0xff3864),
                color_hex(c"flash_color3", c"カラーブロックの色3", 0xffffff),
                float(c"rgb_offset", c"色ズレ（%）", 0.45, 0.0, 10.0),
                boolean(c"rgb_follow", c"色ズレをティアリングに連動", true),
                float(c"rgb_angle", c"色ズレの向き（°）", 0.0, -180.0, 180.0),
                choice(c"rgb_type", c"色ズレのタイプ", 3, c"赤・青|赤・シアン|緑・マゼンタ|カスタム2色"),
                float(c"alpha_threshold", c"アルファ閾値（%）", 1.0, 0.0, 100.0),
                float(c"tear_angle", c"ティアリング方向（°）", 0.0, -180.0, 180.0),
                float(c"line_ratio", c"ライン割合（%）", 25.0, 0.0, 100.0),
                choice(c"edge", c"端の処理", 0, c"透明|端を延長|ループ"),
                color_hex(c"rgb_color1", c"色ズレの色1", 0xff0000),
                color_hex(c"rgb_color2", c"色ズレの色2", 0x00ffff),
            ],
            render: render,
        },
    ]
}

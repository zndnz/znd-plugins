//! 波形ライン（arcana）の移植: レイヤーのマスクのパスを線として描き、音声波形で揺らす。
//! 元はプレビュー上でマウスで線を描く仕組みだが、znd ではマスクのパスを線として使う。
//! 線の作り方（描く割合・破線・丸い端・手描きのブレ・波形の形）は元のスクリプトと同じ。

use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

const HOLD: i64 = 3; // ブレを何フレームごとに変えるか

/// 一定間隔に並べ直した折れ線。
fn resample(pts: &[V2], closed: bool, step: f64) -> Vec<V2> {
    let mut src = pts.to_vec();
    if closed && src.len() > 1 {
        src.push(src[0]);
    }
    let mut out = vec![src[0]];
    let mut carry = 0.0;
    for w in src.windows(2) {
        let (a, b) = (w[0], w[1]);
        let d = b.sub(a).len();
        if d < 1e-9 {
            continue;
        }
        let mut s = step - carry;
        while s <= d {
            out.push(a.lerp(b, s / d));
            s += step;
        }
        carry = d - (s - step);
    }
    if let Some(last) = src.last() {
        if out.last().map(|p| p.sub(*last).len() > 1e-6).unwrap_or(true) {
            out.push(*last);
        }
    }
    out
}

fn normals(p: &[V2]) -> Vec<V2> {
    let n = p.len();
    (0..n)
        .map(|k| {
            let d = p[(k + 1).min(n - 1)].sub(p[k.saturating_sub(1)]);
            let l = d.len();
            if l < 1e-6 {
                V2::new(0.0, 1.0)
            } else {
                V2::new(-d.y / l, d.x / l)
            }
        })
        .collect()
}

struct Wave {
    wav: Vec<f32>,
    scale: f64,
    freq: f64,
    square: bool,
}

impl Wave {
    /// 線上の位置 u（0～1）での揺れ（-1～1）。
    fn level(&self, u: f64) -> f64 {
        let nw = self.wav.len();
        if nw < 2 || self.scale == 0.0 {
            return 0.0;
        }
        let q = (u * self.freq).rem_euclid(1.0);
        if self.square {
            const SEG: f64 = 12.0; // 1 周期あたりの尖りの数
            let t2 = q * SEG;
            let i2 = t2.floor();
            let f2 = t2 - i2;
            let j = ((i2 / SEG) * (nw - 1) as f64).round() as usize;
            let h = self.wav[j.min(nw - 1)] as f64 * self.scale;
            let tri = if f2 < 0.5 { f2 * 2.0 } else { 2.0 - f2 * 2.0 };
            return h * tri;
        }
        let x = q * (nw - 1) as f64;
        let i = (x.floor() as usize).min(nw - 1);
        let f = x - i as f64;
        let a = self.wav[i] as f64;
        let b = *self.wav.get(i + 1).unwrap_or(&self.wav[i]) as f64;
        (a + (b - a) * f) * self.scale
    }
}

/// 太さを持つ線分（端は丸い）を、色の位置 t0→t1 で描く。重なりは濃い方を残す。
struct Canvas {
    w: usize,
    h: usize,
    cov: Vec<f32>,
    t: Vec<f32>,
}

impl Canvas {
    fn capsule(&mut self, a: V2, b: V2, r: f64, t0: f64, t1: f64) {
        let x0 = (a.x.min(b.x) - r - 1.0).floor().max(0.0) as usize;
        let y0 = (a.y.min(b.y) - r - 1.0).floor().max(0.0) as usize;
        let x1 = ((a.x.max(b.x) + r + 1.0).ceil().max(0.0) as usize).min(self.w);
        let y1 = ((a.y.max(b.y) + r + 1.0).ceil().max(0.0) as usize).min(self.h);
        let ab = b.sub(a);
        let l2 = ab.dot(ab);
        for y in y0..y1 {
            for x in x0..x1 {
                let p = V2::new(x as f64 + 0.5, y as f64 + 0.5);
                let s = if l2 < 1e-12 { 0.0 } else { (p.sub(a).dot(ab) / l2).clamp(0.0, 1.0) };
                let d = p.sub(a.add(ab.mul(s))).len();
                let c = (r + 0.5 - d).clamp(0.0, 1.0) as f32;
                let i = y * self.w + x;
                if c > self.cov[i] {
                    self.cov[i] = c;
                    self.t[i] = (t0 + (t1 - t0) * s) as f32;
                }
            }
        }
    }
}

fn jrand(seed: u32, i: u32, frame: i64) -> f64 {
    noise::rand01(seed, i, frame as u32) as f64 * 2.0 - 1.0
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let wave_mode = f.i(0) == 1;
    let thick = f.f(1) * sc;
    let col = f.color(2);
    let col2 = if f.b(3) { f.color(4) } else { col };
    let smooth = f.f(5).round().max(0.0) as usize;
    let grow = f.f(6).clamp(0.0, 100.0) / 100.0;
    let dash = f.f(7) * sc;
    let gap = f.f(8) * sc;
    let amp = f.f(9) * sc;
    let freq = f.f(10);
    let square = f.i(11) == 1;
    let jit = f.f(12) * sc;
    let keep = f.b(14);

    let mut wave = Wave { wav: Vec::new(), scale: 0.0, freq, square };
    if wave_mode && f.has_layer(13) {
        if let Some(mut wav) = f.audio(13, 2048) {
            if smooth > 0 && wav.len() > 2 {
                let n = wav.len();
                wav = (0..n)
                    .map(|i| {
                        let (a, b) = (i.saturating_sub(smooth), (i + smooth).min(n - 1));
                        wav[a..=b].iter().sum::<f32>() / (b - a + 1) as f32
                    })
                    .collect();
            }
            let mx = wav.iter().fold(0.0f32, |m, v| m.max(v.abs()));
            if mx > 100.0 / 32768.0 {
                wave.scale = 1.0 / mx as f64; // ほぼ無音なら揺らさない
            }
            wave.wav = wav;
        }
    }
    let jfr = if wave.scale > 0.0 { f.frame().div_euclid(HOLD) } else { 0 };

    // 線を作る（バッファ座標）。
    let mut lines: Vec<(Vec<V2>, bool)> = Vec::new();
    for (si, (pts, closed)) in f.masks().into_iter().enumerate() {
        if pts.len() < 2 {
            continue;
        }
        let mut p = resample(&pts, closed, 2.0 * sc.max(0.25));
        let n = p.len();
        if n < 2 {
            continue;
        }
        if jit > 0.0 && wave_mode {
            let step = (n / 8).max(4); // 線を 8 節ぶんに分けた、ゆるいうねり
            let nn = n / step + 2;
            let aw: Vec<f64> = (0..nn).map(|i| jrand(f.seed(), (si * 1000 + 400 + i) as u32, jfr) * jit).collect();
            let nm = normals(&p);
            for k in 0..n {
                let u = k as f64 / step as f64;
                let i = u.floor() as usize;
                let w = aw[i] + (aw[i + 1] - aw[i]) * (u - i as f64);
                p[k] = p[k].add(nm[k].mul(w));
            }
        }
        if wave_mode {
            let nm = normals(&p);
            let base = wave.level(0.0);
            p = (0..n)
                .map(|k| {
                    let u = k as f64 / (n - 1) as f64;
                    let mut o = wave.level(u);
                    if closed {
                        let e = u.min(1.0 - u) * 8.0;
                        if e < 1.0 {
                            o = o * e + base * (1.0 - e);
                        }
                    }
                    p[k].add(nm[k].mul(o * amp))
                })
                .collect();
        }
        lines.push((p, closed));
    }
    if lines.is_empty() {
        if !keep {
            f.rgba.fill(0.0);
        }
        return Ok(());
    }

    // 描く範囲。
    let r = thick / 2.0;
    let (mut lo, mut hi) = (V2::new(0.0, 0.0), V2::new(f.width as f64, f.height as f64));
    for (p, _) in &lines {
        for q in p {
            lo = V2::new(lo.x.min(q.x - r - 2.0), lo.y.min(q.y - r - 2.0));
            hi = V2::new(hi.x.max(q.x + r + 2.0), hi.y.max(q.y + r + 2.0));
        }
    }
    let (ox, oy) = (lo.x.floor() as i32, lo.y.floor() as i32);
    let w = (hi.x.ceil() as i32 - ox).clamp(1, 16384) as usize;
    let h = (hi.y.ceil() as i32 - oy).clamp(1, 16384) as usize;
    let off = V2::new(ox as f64, oy as f64);
    let mut cv = Canvas { w, h, cov: vec![0.0; w * h], t: vec![0.0; w * h] };

    for (p, _) in &lines {
        let p: Vec<V2> = p.iter().map(|q| q.sub(off)).collect();
        let mut sl = vec![0.0; p.len()];
        for k in 1..p.len() {
            sl[k] = sl[k - 1] + p[k].sub(p[k - 1]).len();
        }
        let total = *sl.last().unwrap();
        if total <= 1e-9 {
            continue;
        }
        let visend = total * grow;
        let (mut dz, mut gz) = (dash, gap);
        let dashed = dz > 0.0 && gz > 0.0 && dz + gz > 0.01;
        if dashed && total / (dz + gz) > 1000.0 {
            let k = total / (dz + gz) / 1000.0;
            dz *= k;
            gz *= k;
        }
        let period = dz + gz;
        for k in 0..p.len() - 1 {
            let (s0, s1) = (sl[k], sl[k + 1]);
            let len = s1 - s0;
            if len <= 1e-9 || s0 >= visend {
                continue;
            }
            let at = |s: f64| (p[k].lerp(p[k + 1], (s - s0) / len), s / total);
            let mut piece = |a: f64, b: f64| {
                if b - a > 1e-4 {
                    let ((pa, ta), (pb, tb)) = (at(a), at(b));
                    cv.capsule(pa, pb, r, ta, tb);
                }
            };
            if !dashed {
                piece(s0, s1.min(visend));
            } else {
                let i0 = (s0 / period).floor() as i64;
                let i1 = (s1 / period).floor() as i64;
                for i in i0..=i1 {
                    let ds = i as f64 * period;
                    piece(s0.max(ds), s1.min(ds + dz).min(visend));
                }
            }
        }
    }

    let mut img = Image::new(w, h);
    for i in 0..w * h {
        let c = cv.cov[i];
        if c <= 0.0 {
            continue;
        }
        let t = cv.t[i];
        let a = c * (col[3] + (col2[3] - col[3]) * t);
        for ch in 0..3 {
            img.data[i * 4 + ch] = (col[ch] + (col2[ch] - col[ch]) * t) * a;
        }
        img.data[i * 4 + 3] = a;
    }
    if keep {
        let src = f.input();
        let mut base = Image::new(w, h);
        base.draw(&src, -ox as isize, -oy as isize, 1.0);
        base.draw(&img, 0, 0, 1.0);
        img = base;
    }
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "arcana（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "ar-ca-na.waveform-line", name: "波形ライン", category: "描画", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                choice(c"mode", c"モード", 0, c"線を描く|波形にする"),
                float(c"thick", c"太さ", 4.0, 0.01, 20.0),
                color_hex(c"col", c"色", 0xffffff),
                boolean(c"grad", c"グラデーション", false),
                color_hex(c"col2", c"色2(グラデ)", 0xffffff),
                float(c"smooth", c"波形をならす", 0.0, 0.0, 50.0),
                float(c"grow", c"描く割合", 100.0, 0.0, 100.0),
                float(c"dash", c"破線の長さ", 0.0, 0.0, 200.0),
                float(c"gap", c"破線のすき間", 10.0, 0.0, 100.0),
                float(c"amp", c"波形の振幅", 60.0, 0.0, 150.0),
                float(c"freq", c"波の回数", 1.0, 0.05, 30.0),
                choice(c"wshape", c"波形の形", 0, c"なめらか|四角"),
                float(c"jit", c"手描きのブレ", 0.0, 0.0, 200.0),
                layer(c"audio", c"音声レイヤー"),
                boolean(c"keep", c"元の絵を残す", false),
            ],
            render: render,
        },
    ]
}

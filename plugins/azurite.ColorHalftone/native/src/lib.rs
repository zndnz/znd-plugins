//! ColorHalftone（azurite）の移植: 3 色の網点に分解するカラーハーフトーン。
//! 計算は元のシェーダーと同じ（網点の形は Inigo Quilez の 2D SDF、MIT）。

use znd_plugin_sdk::*;

type V = [f64; 2];

fn len(p: V) -> f64 {
    p[0].hypot(p[1])
}
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}
fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}
fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

// ---------- 2D SDF（Inigo Quilez, MIT） ----------

fn sd_circle(p: V, r: f64) -> f64 {
    len(p) - r
}
fn sd_square(p: V, s: V) -> f64 {
    let d = [p[0].abs() - s[0], p[1].abs() - s[1]];
    len([d[0].max(0.0), d[1].max(0.0)]) + d[0].max(d[1]).min(0.0)
}
fn sd_triangle(p: V, r: f64) -> f64 {
    let k = 3f64.sqrt();
    let mut p = [p[0].abs() - r, p[1] + r / k];
    if p[0] + k * p[1] > 0.0 {
        p = [(p[0] - k * p[1]) / 2.0, (-k * p[0] - p[1]) / 2.0];
    }
    p[0] -= p[0].clamp(-2.0 * r, 0.0);
    -len(p) * sign(p[1])
}
fn sd_pentagon(p: V, r: f64) -> f64 {
    let k = [0.809016994, 0.587785252, 0.726542528];
    let mut p = [p[0].abs(), p[1]];
    let a = [-k[0], k[1]];
    let m = 2.0 * dot(a, p).min(0.0);
    p = [p[0] - m * a[0], p[1] - m * a[1]];
    let b = [k[0], k[1]];
    let m = 2.0 * dot(b, p).min(0.0);
    p = [p[0] - m * b[0], p[1] - m * b[1]];
    p = [p[0] - p[0].clamp(-r * k[2], r * k[2]), p[1] - r];
    len(p) * sign(p[1])
}
fn sd_hexagon(p: V, r: f64) -> f64 {
    let k = [-0.866025404, 0.5, 0.577350269];
    let mut p = [p[0].abs(), p[1].abs()];
    let m = 2.0 * dot([k[0], k[1]], p).min(0.0);
    p = [p[0] - m * k[0], p[1] - m * k[1]];
    p = [p[0] - p[0].clamp(-k[2] * r, k[2] * r), p[1] - r];
    len(p) * sign(p[1])
}
fn sd_pentagram(p: V, r: f64) -> f64 {
    let s5 = 5f64.sqrt();
    let (k1x, k2x) = ((s5 + 1.0) / 4.0, (s5 - 1.0) / 4.0);
    let (k1y, k2y) = ((10.0 - 2.0 * s5).sqrt() / 4.0, (10.0 + 2.0 * s5).sqrt() / 4.0);
    let k1z = (5.0 - 2.0 * s5).sqrt();
    let v1 = [k1x, -k1y];
    let v2 = [-k1x, -k1y];
    let v3 = [k2x, -k2y];
    let mut p = [p[0].abs(), p[1]];
    let m = 2.0 * dot(v1, p).max(0.0);
    p = [p[0] - m * v1[0], p[1] - m * v1[1]];
    let m = 2.0 * dot(v2, p).max(0.0);
    p = [p[0] - m * v2[0], p[1] - m * v2[1]];
    p[0] = p[0].abs();
    p[1] -= r;
    let t = dot(p, v3).clamp(0.0, k1z * r);
    len([p[0] - v3[0] * t, p[1] - v3[1] * t]) * sign(p[1] * v3[0] - p[0] * v3[1])
}
fn sd_heart(p: V) -> f64 {
    let p = [p[0].abs(), p[1]];
    let d2 = |v: V| dot(v, v);
    if p[1] + p[0] > 1.0 {
        return d2([p[0] - 0.25, p[1] - 0.75]).sqrt() - 2f64.sqrt() / 4.0;
    }
    let m = 0.5 * (p[0] + p[1]).max(0.0);
    d2([p[0], p[1] - 1.0]).min(d2([p[0] - m, p[1] - m])).sqrt() * sign(p[0] - p[1])
}

fn smin(a: f64, b: f64, k: f64) -> f64 {
    if k <= 0.0 {
        return a.min(b);
    }
    let h = (0.5 + 0.5 * (b - a) / k).clamp(0.0, 1.0);
    lerp(b, a, h) - k * h * (1.0 - h)
}

/// 回転行列（元の rotate() と同じ並び）。
fn rot(deg: f64) -> [f64; 4] {
    let r = deg.to_radians();
    [r.cos(), -r.sin(), r.sin(), r.cos()]
}
fn mul(m: [f64; 4], v: V) -> V {
    [m[0] * v[0] + m[1] * v[1], m[2] * v[0] + m[3] * v[1]]
}
fn mul_t(m: [f64; 4], v: V) -> V {
    [m[0] * v[0] + m[2] * v[1], m[1] * v[0] + m[3] * v[1]]
}

struct Ch {
    dot_size: f64,
    shift: V,
    screen: [f64; 4],
    dot: [f64; 4],
    col: [f64; 3],
    visible: bool,
}

struct Ctx<'a> {
    src: &'a Image,
    w: f64,
    aspect: f64,
    screen: f64,
    shape: usize,
    offset: bool,
    fusion: f64,
    additive: bool,
    bg: [f64; 4],
    aa: f64,
}

impl Ctx<'_> {
    /// 中心からの uv（横幅で正規化）にある、入力の色（背景色の上に置いた、まっすぐな RGB）。
    fn tex(&self, cu: V) -> [f64; 3] {
        let (w, h) = (self.src.w as f64, self.src.h as f64);
        let u = ((cu[0] + 0.5) * w) as f32;
        let v = ((cu[1] * self.aspect + 0.5) * h) as f32;
        let c = self.src.sample_clamped(u, v);
        let a = c[3] as f64;
        let s = |i: usize| if a > 1e-6 { c[i] as f64 / a } else { 0.0 };
        [lerp(self.bg[0], s(0), a), lerp(self.bg[1], s(1), a), lerp(self.bg[2], s(2), a)]
    }

    fn halftone(&self, p: V, channel: usize, ch: &Ch) -> [f64; 4] {
        let s = 1.0;
        let tone = [ch.shift[0] / self.w * self.screen, ch.shift[1] / self.w * self.screen];
        let st = mul(ch.screen, [p[0] - tone[0], p[1] - tone[1]]);
        let id = [(st[0] / s).round(), (st[1] / s).round()];
        let mut min_d = 1e20;
        let mut shape_size = 0.0;
        let range = ((ch.dot_size * 1.5 + self.fusion * 0.5).ceil() as i32).clamp(1, 3);
        for y in -range..=range {
            for x in -range..=range {
                let rid = [id[0] + x as f64, id[1] + y as f64];
                let mut pos = [rid[0] * s, rid[1] * s];
                // 行が奇数なら X を 0.5 ずらして段違いにする
                if self.offset && rid[1].abs() % 2.0 > 0.5 {
                    pos[0] += 0.5 * s;
                }
                let r = mul(ch.dot, [st[0] - pos[0], st[1] - pos[1]]);
                let t = mul_t(ch.screen, pos);
                let cu = [(t[0] + tone[0]) / self.screen, (t[1] + tone[1]) / self.screen];
                let mut rgb = self.tex(cu);
                if !self.additive {
                    rgb = [1.0 - rgb[0], 1.0 - rgb[1], 1.0 - rgb[2]];
                }
                let mut value = rgb[channel.min(2)].clamp(0.0, 1.0);
                if self.additive {
                    value = value.powf(2.2); // 加法混色はリニアにする
                }
                let mut size = value.sqrt() * 0.5 * s;
                let ds = ch.dot_size;
                let scaled = |size: f64, curve: f64, base: f64, max: f64| size * lerp(base, max, value.powf(curve));
                let d = match self.shape {
                    1 => {
                        size = scaled(size, 2.2, 1.14, std::f64::consts::SQRT_2);
                        sd_square(r, [ds * size, ds * size])
                    }
                    2 => {
                        size = scaled(size, 2.2, 1.519539404 * 1.03, 2.309401077);
                        sd_triangle(r, size * ds)
                    }
                    3 => {
                        size = scaled(size, 4.0, 1.049727762 * 1.1, std::f64::consts::SQRT_2);
                        sd_pentagon(r, size * ds)
                    }
                    4 => {
                        size = scaled(size, 4.0, 1.074585693 * 1.1, std::f64::consts::SQRT_2);
                        sd_hexagon(r, size * ds)
                    }
                    5 => {
                        size = scaled(size, 2.2, 1.8, 3.0);
                        sd_pentagram(r, size * ds)
                    }
                    6 => {
                        size = scaled(size, 2.0, 2.3, 2.8);
                        let cs = size * ds;
                        if cs < 1e-6 {
                            1.0
                        } else {
                            sd_heart([r[0] / cs, r[1] / cs + 0.5]) * cs
                        }
                    }
                    7 => {
                        let v2 = value + smoothstep(1.0 - 0.32, 1.0 + 0.32, value);
                        let line = v2.sqrt() * 0.5 * s;
                        sd_square(r, [0.5 * s * ds * (1.0 + (1.0 / self.w * self.screen) * 2.0), line * ds])
                    }
                    _ => {
                        size = scaled(size, 6.0, 1.128379167 * 1.14, std::f64::consts::SQRT_2);
                        sd_circle(r, ds * size)
                    }
                };
                if d < min_d {
                    shape_size = size;
                }
                min_d = smin(min_d, d, self.fusion);
            }
        }
        let aa = self.aa;
        let blend = if self.shape == 7 {
            let t = smoothstep(0.0, 0.5 * s, shape_size * ch.dot_size);
            smoothstep(-aa, lerp(0.0, aa, t), min_d)
        } else {
            let b = smoothstep(-aa, aa, min_d);
            let op = smoothstep(0.0, aa * 1.2, shape_size * ch.dot_size);
            1.0 - (1.0 - b) * op
        };
        [lerp(ch.col[0], self.bg[0], blend), lerp(ch.col[1], self.bg[1], blend), lerp(ch.col[2], self.bg[2], blend), 1.0 - blend]
    }
}

fn rgb(c: [f32; 4]) -> [f64; 3] {
    [c[0] as f64, c[1] as f64, c[2] as f64]
}

fn render(f: &mut Frame) -> FxResult {
    let (w, h) = (f.width, f.height);
    if w == 0 || h == 0 {
        return Ok(());
    }
    let size = f.f(0).max(1.0);
    let dot_size = f.f(1).max(0.0) / 100.0;
    let shift = [f.f(2) * f.scale(), f.f(3) * f.scale()];
    let angle = f.f(4);
    let mut dot_angle = f.f(5);
    let smooth = f.f(6).clamp(0.0, 5.0);
    let fusion = f.f(7).max(0.0) / 100.0;
    let shape = f.i(8).min(7);
    let additive = f.i(9) == 1;
    let offset = f.b(10);
    let bgc = f.color(11);
    let bg_alpha = f.f(12).clamp(0.0, 100.0);
    let keep_alpha = f.b(13);
    dot_angle += match shape {
        1 => 45.0,
        2 => 60.0,
        4 => 30.0,
        5 => 36.0,
        6 => 180.0,
        _ => 0.0,
    };
    let chs: Vec<Ch> = (0..3)
        .map(|k| Ch {
            dot_size,
            shift,
            screen: rot(f.f(17 + k) + angle),
            dot: rot(dot_angle),
            col: rgb(f.color(14 + k)),
            visible: f.b(20 + k),
        })
        .collect();

    let orig = f.input();
    let mut src = orig.clone();
    if smooth > 0.0 {
        src.gaussian_blur((smooth * f.scale()) as f32 * 0.5);
    }
    let screen = (w as f64 * 0.25 * ((100.0 - size) / 100.0)).max(1e-3);
    let bg = [bgc[0] as f64, bgc[1] as f64, bgc[2] as f64, (100.0 - bg_alpha) / 100.0];
    let cx = Ctx { src: &src, w: w as f64, aspect: w as f64 / h as f64, screen, shape, offset, fusion, additive, bg, aa: screen / w as f64 };
    let hidden = [bg[0], bg[1], bg[2], 0.0];
    for y in 0..h {
        for x in 0..w {
            let uv = [(x as f64 + 0.5 - 0.5 * w as f64) / w as f64, (y as f64 + 0.5 - 0.5 * h as f64) / w as f64];
            let p = [uv[0] * screen, uv[1] * screen];
            let c: Vec<[f64; 4]> = chs.iter().enumerate().map(|(k, ch)| if ch.visible { cx.halftone(p, k, ch) } else { hidden }).collect();
            let mut out = [0.0; 3];
            for i in 0..3 {
                out[i] = if additive { c[0][i].max(c[1][i]).max(c[2][i]) } else { c[0][i].min(c[1][i]).min(c[2][i]) };
            }
            let da = c[0][3].max(c[1][3]).max(c[2][3]);
            let mut a = (da + bg[3] * (1.0 - da)).clamp(0.0, 1.0);
            for i in 0..3 {
                out[i] = (out[i] - bg[i] * (1.0 - bg[3]) * (1.0 - da)).clamp(0.0, 1.0);
            }
            if keep_alpha {
                let ta = orig.px(x, y)[3] as f64;
                for v in &mut out {
                    *v *= ta;
                }
                a *= ta;
            }
            f.rgba[(y * w + x) * 4..(y * w + x) * 4 + 4].copy_from_slice(&[out[0] as f32, out[1] as f32, out[2] as f32, a as f32]);
        }
    }
    Ok(())
}

plugin! {
    vendor: "azurite（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "azurite.ColorHalftone", name: "カラーハーフトーン", category: "加工", flags: THREAD_SAFE,
            params: [
                float(c"size", c"サイズ", 50.0, 1.0, 100.0),
                float(c"dot_size", c"ドットサイズ", 100.0, 0.0, 200.0),
                float(c"x", c"X", 0.0, -4000.0, 4000.0),
                float(c"y", c"Y", 0.0, -4000.0, 4000.0),
                float(c"angle", c"回転", 0.0, -360.0, 360.0),
                float(c"dot_angle", c"ドット回転", 0.0, -360.0, 360.0),
                float(c"smoothness", c"滑らかさ", 1.0, 0.0, 5.0),
                float(c"fusion", c"融合度", 0.0, 0.0, 100.0),
                choice(c"shape", c"形状", 0, c"円|四角形|三角形|五角形|六角形|星型|ハート|線"),
                choice(c"mixing", c"混色方法", 0, c"減法混色|加法混色"),
                boolean(c"offset", c"段違い", false),
                color_hex(c"bg_col", c"背景色", 0xffffff),
                float(c"bg_alpha", c"背景色透明度", 0.0, 0.0, 100.0),
                boolean(c"keep_alpha", c"元のアルファを維持", true),
                color_hex(c"col1", c"色1", 0x00ffff),
                color_hex(c"col2", c"色2", 0xff00ff),
                color_hex(c"col3", c"色3", 0xffff00),
                float(c"col1_angle", c"色1スクリーン角度", 75.0, -360.0, 360.0),
                float(c"col2_angle", c"色2スクリーン角度", 15.0, -360.0, 360.0),
                float(c"col3_angle", c"色3スクリーン角度", 60.0, -360.0, 360.0),
                boolean(c"col1_visible", c"色1表示", true),
                boolean(c"col2_visible", c"色2表示", true),
                boolean(c"col3_visible", c"色3表示", true),
            ],
            render: render,
        },
    ]
}

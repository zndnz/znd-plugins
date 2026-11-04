//! 段階グラデーション（てるよし）の移植: 段階数で区切ったグラデーションを、合成モードを選んで重ねる。
//! 「グラデ曲線」はパラメータのキーフレームで描いた曲線（レイヤーの長さ全体）を表として読む（元の時間制御カーブの代わり）。
//! 色の計算は元のシェーダーと同じく、乗算済みの表示値で行う。

use znd_plugin_sdk::*;

const RES: usize = 100;
type C4 = [f64; 4];

fn lerp4(a: C4, b: C4, t: f64) -> C4 {
    [0, 1, 2, 3].map(|i| a[i] + (b[i] - a[i]) * t)
}
fn rgb2hsl(c: C4) -> C4 {
    let (r, g, b) = (c[0], c[1], c[2]);
    let mx = r.max(g).max(b);
    let mn = r.min(g).min(b);
    let l = (mx + mn) / 2.0;
    let (mut h, mut s) = (0.0, 0.0);
    if mx != mn {
        let d = mx - mn;
        s = if l < 0.5 { d / (mx + mn) } else { d / (2.0 - mx - mn) };
        h = if mx == r { (g - b) / d + if g < b { 6.0 } else { 0.0 } } else if mx == g { (b - r) / d + 2.0 } else { (r - g) / d + 4.0 };
        h /= 6.0;
    }
    [h, s, l, c[3]]
}
fn hsl2rgb(c: C4) -> C4 {
    let r = (c[0] * 6.0 - 3.0).abs() - 1.0;
    let g = 2.0 - (c[0] * 6.0 - 2.0).abs();
    let b = 2.0 - (c[0] * 6.0 - 4.0).abs();
    let k = (1.0 - (2.0 * c[2] - 1.0).abs()) * c[1];
    [(r - 0.5) * k + c[2], (g - 0.5) * k + c[2], (b - 0.5) * k + c[2], c[3]]
}
fn wrap(mut h: C4) -> C4 {
    if h[0] > 1.0 {
        h[0] -= 1.0;
    }
    if h[0] < 0.0 {
        h[0] += 1.0;
    }
    h
}
fn step_t(t: f64, step: f64, div_plus_one: bool) -> f64 {
    if step == 0.0 {
        t
    } else {
        (t * step).floor() / if div_plus_one { step + 1.0 } else { step }
    }
}

fn mix(space: usize, c1: C4, c2: C4, t: f64, step: f64) -> C4 {
    if space == 0 {
        return lerp4(c1, c2, step_t(t, step, true));
    }
    let (mut h1, mut h2) = (rgb2hsl(c1), rgb2hsl(c2));
    match space {
        1 | 2 => {
            let far = (h2[0] - h1[0]).abs() > 0.5;
            if far == (space == 1) {
                if h1[0] > h2[0] {
                    h2[0] += 1.0;
                } else {
                    h1[0] += 1.0;
                }
            }
            hsl2rgb(wrap(lerp4(h1, h2, step_t(t, step, true))))
        }
        3 => {
            if h2[0] < h1[0] {
                h2[0] += 1.0;
            }
            hsl2rgb(wrap(lerp4(h1, h2, step_t(t, step, true))))
        }
        // 左回り: 元は 2 色を入れ替えて呼ぶので、ここでも入れ替えたまま計算する
        _ => {
            let (a, b) = (h2, h1);
            let (mut a, b) = (a, b);
            if a[0] > b[0] {
                a[0] -= 1.0;
            }
            hsl2rgb(wrap(lerp4(b, a, step_t(t, step, false))))
        }
    }
}

fn disp(c: [f32; 4]) -> [f64; 3] {
    [0, 1, 2].map(|i| color::linear_to_srgb(c[i]) as f64)
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let strength = (f.f(0) / 100.0) as f32;
    let (w, h) = (f.width as f64, f.height as f64);
    let center = [f.f(1) * sc + w / 2.0, f.f(2) * sc + h / 2.0];
    let angle_deg = f.f(3);
    let width = (f.f(4) * sc).max(1e-6);
    let shape = f.i(5);
    let step = f.f(6).round().max(0.0);
    let curve = if f.b(7) { f.curve(8, RES + 1).into_iter().map(|v| (v / 100.0).clamp(0.0, 1.0)).collect() } else { (0..=RES).map(|i| i as f64 / RES as f64).collect::<Vec<_>>() };
    let (r1, a1) = (disp(f.color(9)), 1.0 - f.f(10) / 100.0);
    let (r2, a2) = (disp(f.color(11)), 1.0 - f.f(12) / 100.0);
    let space = f.i(13);
    let blend = f.i(14);
    let clear = f.b(15);
    let angle = angle_deg.to_radians() + std::f64::consts::FRAC_PI_2;
    let org = f.input();
    let grad = Image::generate(f.width, f.height, |x, y| {
        let oa = org.px(x, y)[3] as f64;
        let c1 = [r1[0] * oa * a1, r1[1] * oa * a1, r1[2] * oa * a1, a1 * oa];
        let c2 = [r2[0] * oa * a2, r2[1] * oa * a2, r2[2] * oa * a2, a2 * oa];
        let p = [x as f64 + 0.5 - center[0], y as f64 + 0.5 - center[1]];
        let d = if shape == 0 {
            p[0] * angle.cos() + p[1] * angle.sin() + width / 2.0 + if step > 0.0 { width / step / 2.0 } else { 0.0 }
        } else {
            (p[0].hypot(p[1]) - angle_deg / 2.0) * 2.0
        };
        let c = if step == 1.0 {
            if d < width { c1 } else { c2 }
        } else if d < 0.0 {
            c1
        } else if d > width {
            c2
        } else {
            let i = ((d / width * RES as f64).floor() as usize).min(RES);
            mix(space, c1, c2, curve[i], step)
        };
        let a = c[3].clamp(0.0, 1.0) as f32;
        if a <= 0.0 {
            return [0.0; 4];
        }
        color::from_display([(c[0] / c[3]) as f32, (c[1] / c[3]) as f32, (c[2] / c[3]) as f32, a].map(|v| v.clamp(0.0, 1.0)))
    });
    let mut base = if clear { Image::new(f.width, f.height) } else { org };
    aul::blend(&mut base, &grad, blend, strength);
    f.set(base);
    Ok(())
}

plugin! {
    vendor: "てるよし（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "teruyoshii.stepGradation", name: "段階グラデーション", category: "色調整", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"str", c"強さ", 100.0, 0.0, 100.0),
                float(c"cx", c"中心X", 0.0, -5000.0, 5000.0),
                float(c"cy", c"中心Y", 0.0, -5000.0, 5000.0),
                float(c"angle", c"角度/内径", 0.0, -3600.0, 3600.0),
                float(c"width", c"幅", 300.0, 0.0, 2000.0),
                choice(c"shape", c"形状", 0, c"線形|円形"),
                float(c"step", c"段階数", 5.0, 0.0, 100.0),
                boolean(c"use_curve", c"グラデ曲線を使う", false),
                float(c"curve", c"グラデ曲線0→100", 0.0, 0.0, 100.0),
                color_hex(c"rgb1", c"開始色", 0xffffff),
                float(c"a1", c"開始色透明度", 0.0, 0.0, 100.0),
                color_hex(c"rgb2", c"終了色", 0x000000),
                float(c"a2", c"終了色透明度", 0.0, 0.0, 100.0),
                choice(c"color_space", c"色空間", 0, c"RGB|HSL短経路|HSL長経路|HSL右回り|HSL左回り"),
                choice(c"blend_mode", c"合成モード", 0, c"通常|加算|減算|乗算|スクリーン|オーバーレイ|比較(明)|比較(暗)|輝度|色差|陰影|明暗|差分"),
                boolean(c"clear", c"元オブジェクトをクリア", false),
            ],
            render: render,
        },
    ]
}

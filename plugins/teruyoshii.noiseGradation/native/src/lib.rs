//! ノイズグラデーション（てるよし）の移植: ノイズやディザの模様で 2 色を塗り分けて、グラデーションに見せる。
//! 「グラデ曲線」はパラメータのキーフレームで描いた曲線（レイヤーの長さ全体）を表として読む（元の時間制御カーブの代わり）。

mod tables;
use std::f64::consts::PI;
use tables::{BLUE, CLUSTER, HALFTONE};
use znd_plugin_sdk::*;

const RES: usize = 255;

fn ibuki(u: [u32; 4]) -> f64 {
    let m = [0xae3cc725u32, 0x9fe72885, 0xae36bfb5, 0x82c1fcad];
    let mut u = [0, 1, 2, 3].map(|i| u[i].wrapping_mul(m[i]));
    let sw = [u[3], u[0], u[1], u[2]];
    u = [0, 1, 2, 3].map(|i| u[i] ^ sw[i] ^ (u[i] >> 13));
    let mut r = (0..4).fold(0u32, |a, i| a.wrapping_add(u[i].wrapping_mul(m[i])));
    r ^= r >> 11;
    r = r.wrapping_mul(r) ^ r;
    r as f64 * 2.328_306_436_538_696_3e-10
}

fn bayer(p: [u32; 2], bits: u32) -> f64 {
    let mut v = 0u32;
    for i in 0..3 {
        let x = (p[0] >> i) & 1;
        let y = (p[1] >> i) & 1;
        v = (v << 2) | ((x ^ y) << 1) | y;
    }
    v >>= 6 - bits * 2;
    (v as f64 + 0.5) / (1u32 << (bits * 2)) as f64
}

fn fract(v: f64) -> f64 {
    v - v.floor()
}

fn pattern(pos: [f64; 2], kind: usize, s: u32, period: f64, angle: f64) -> f64 {
    let p = [pos[0] as u32, pos[1] as u32];
    match kind {
        3..=5 => bayer([p[0].wrapping_add(s % 8), p[1].wrapping_add(s / 8 % 8)], kind as u32 - 2),
        6 => {
            let q = [(p[0] + s % 8) % 8, (p[1] + s / 8 % 8) % 8];
            (CLUSTER[(q[1] * 8 + q[0]) as usize] as f64 + 0.5) / 64.0
        }
        1 => {
            let q = [(p[0] + s % 32) % 32, (p[1] + s / 32 % 32) % 32];
            (BLUE[(q[1] * 32 + q[0]) as usize] as f64 + 0.5) / 1024.0
        }
        2 => {
            let q = [pos[0] + s as f64 * 5.588238, pos[1] + s as f64 * 5.588238];
            fract(52.9829189 * fract(q[0] * 0.06711056 + q[1] * 0.00583715))
        }
        _ => {
            let (sn, cs) = angle.sin_cos();
            let u = [
                (pos[0] * cs + pos[1] * sn) / period + fract(s as f64 * 0.7548777),
                (-pos[0] * sn + pos[1] * cs) / period + fract(s as f64 * 0.5698403),
            ];
            if kind == 7 {
                let fv = (0.5 - ((2.0 * PI * u[0]).cos() + (2.0 * PI * u[1]).cos()) / 4.0).clamp(0.0, 1.0) * 64.0;
                let i = (fv as usize).min(63);
                HALFTONE[i] as f64 + (HALFTONE[i + 1] - HALFTONE[i]) as f64 * (fv - i as f64)
            } else {
                (fract(u[0]) * 2.0 - 1.0).abs()
            }
        }
    }
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let strength = (f.f(0) / 100.0) as f32;
    let (sx, sy) = (f.f(1).max(1.0) * sc, f.f(2).max(1.0) * sc);
    let (cx0, cy0) = (f.f(3) * sc, f.f(4) * sc);
    let angle_deg = f.f(5);
    let mut width = (f.f(6) * sc).max(1e-6);
    let shape = f.i(7);
    let kind = f.i(8);
    let period = f.f(9).max(1e-3);
    let pat_angle = f.f(10).to_radians();
    let curve: Vec<f64> = if f.b(11) { f.curve(12, RES + 1).into_iter().map(|v| v / 100.0).collect() } else { (0..=RES).map(|i| i as f64 / RES as f64).collect() };
    let col = |c: [f32; 4]| [0, 1, 2].map(|i| color::linear_to_srgb(c[i]));
    let (c1, a1) = (col(f.color(13)), (1.0 - f.f(14) / 100.0) as f32);
    let (c2, a2) = (col(f.color(15)), (1.0 - f.f(16) / 100.0) as f32);
    let blend = f.i(17);
    let seed = f.f(18).max(0.0) as u32;
    let clear = f.b(19);
    // 元はドットサイズ分だけ縮めてから模様を作り、拡大し直す。ここでは縮めた座標で計算する
    let (w, h) = (f.width as f64, f.height as f64);
    let (ws, hs) = (w / sx, h / sy);
    let mut angle = angle_deg.to_radians();
    let inner;
    if shape == 0 {
        angle = (sx * angle.cos()).atan2(sy * angle.sin()) + PI / 2.0;
        width *= ((angle.cos() / sx).powi(2) + (angle.sin() / sy).powi(2)).sqrt();
        inner = 0.0;
    } else {
        inner = angle_deg / 2.0;
        width /= 2.0;
    }
    let center = [cx0 / sx + ws / 2.0, cy0 / sy + hs / 2.0];
    let org = f.input();
    let grad = Image::generate(f.width, f.height, |x, y| {
        let pos = [(x as f64 / sx).floor() + 0.5, (y as f64 / sy).floor() + 0.5];
        let r = if kind == 0 { ibuki([pos[0] as u32, pos[1] as u32, 0, 1u32.wrapping_add(seed)]) } else { pattern(pos, kind, seed, period, pat_angle) };
        let d = if shape == 0 {
            (pos[0] - center[0]) * angle.cos() + (pos[1] - center[1]) * angle.sin() + width / 2.0
        } else {
            ((pos[0] - center[0]) * sx).hypot((pos[1] - center[1]) * sy) - inner
        };
        let second = if d < 0.0 {
            false
        } else if d > width {
            true
        } else {
            curve[((d / width * RES as f64).floor() as usize).min(RES)] > r
        };
        let (c, a) = if second { (c2, a2) } else { (c1, a1) };
        let a = a * org.px(x, y)[3];
        color::from_display([c[0], c[1], c[2], a])
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
            id: "teruyoshii.noiseGradation", name: "ノイズグラデーション", category: "色調整", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"str", c"強さ", 100.0, 0.0, 100.0),
                float(c"size_x", c"ドットサイズX", 1.0, 1.0, 100.0),
                float(c"size_y", c"ドットサイズY", 1.0, 1.0, 100.0),
                float(c"cx", c"中心X", 0.0, -5000.0, 5000.0),
                float(c"cy", c"中心Y", 0.0, -5000.0, 5000.0),
                float(c"angle", c"角度/内径", 0.0, -3600.0, 3600.0),
                float(c"width", c"幅", 300.0, 0.0, 2000.0),
                choice(c"shape", c"形状", 0, c"線形|円形"),
                choice(c"noise_type", c"ノイズの種類", 0, c"ホワイトノイズ|ブルーノイズ|インターリーブグラデーションノイズ(IGN)|配列ディザリング(2x2)|配列ディザリング(4x4)|配列ディザリング(8x8)|集中型ディザリング(8x8)|網点|線"),
                float(c"period", c"模様の周期", 8.0, 2.0, 200.0),
                float(c"pattern_angle", c"模様の角度", 45.0, -360.0, 360.0),
                boolean(c"use_curve", c"グラデ曲線を使う", false),
                float(c"curve", c"グラデ曲線0→100", 0.0, 0.0, 100.0),
                color_hex(c"rgb1", c"開始色", 0xff0000),
                float(c"a1", c"開始色透明度", 0.0, 0.0, 100.0),
                color_hex(c"rgb2", c"終了色", 0x0000ff),
                float(c"a2", c"終了色透明度", 0.0, 0.0, 100.0),
                choice(c"blend_mode", c"合成モード", 0, c"通常|加算|減算|乗算|スクリーン|オーバーレイ|比較(明)|比較(暗)|輝度|色差|陰影|明暗|差分"),
                float(c"seed", c"シード", 0.0, 0.0, 10000.0),
                boolean(c"clear", c"元オブジェクトをクリア", false),
            ],
            render: render,
        },
    ]
}

//! extra_blend_mode.anm2（sevenc-nanashi）の移植: 76 種類の合成モードで、このレイヤーを背景に合成する。
//! 各モードの式は元のシェーダー（0〜255 の値）と同じ。

use znd_plugin_sdk::color::{from_display, to_display};
use znd_plugin_sdk::*;

type C = [f32; 3];

fn min_c(c: C) -> f32 {
    c[0].min(c[1]).min(c[2])
}
fn max_c(c: C) -> f32 {
    c[0].max(c[1]).max(c[2])
}
fn luma(c: C) -> f32 {
    c[0] * 0.299 + c[1] * 0.587 + c[2] * 0.114
}
fn hue(c: C) -> f32 {
    let hi = max_c(c);
    let ch = hi - min_c(c);
    if ch == 0.0 {
        return 0.0;
    }
    let h = if hi == c[0] {
        60.0 * (c[1] - c[2]) / ch
    } else if hi == c[1] {
        60.0 * ((c[2] - c[0]) / ch + 2.0)
    } else {
        60.0 * ((c[0] - c[1]) / ch + 4.0)
    };
    if h < 0.0 {
        h + 360.0
    } else {
        h
    }
}
fn hue_range(h: f32, lo: f32, hi: f32) -> C {
    let h = (h / 360.0).rem_euclid(1.0) * 6.0;
    let u = if h < 1.0 {
        [1.0, h, 0.0]
    } else if h < 2.0 {
        [2.0 - h, 1.0, 0.0]
    } else if h < 3.0 {
        [0.0, 1.0, h - 2.0]
    } else if h < 4.0 {
        [0.0, 4.0 - h, 1.0]
    } else if h < 5.0 {
        [h - 4.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 6.0 - h]
    };
    u.map(|v| lo + (hi - lo) * v)
}
fn to_hsv(c: C) -> C {
    let hi = max_c(c);
    let ch = hi - min_c(c);
    [hue(c), if hi == 0.0 { 0.0 } else { ch / hi }, hi]
}
fn from_hsv(h: C) -> C {
    hue_range(h[0], (1.0 - h[1]) * h[2], h[2])
}
fn to_hsl(c: C) -> C {
    let (lo, hi) = (min_c(c) / 255.0, max_c(c) / 255.0);
    let l = (hi + lo) * 0.5;
    if hi == lo {
        return [0.0, 0.0, l];
    }
    [hue(c), (hi - lo) / if l <= 0.5 { hi + lo } else { 2.0 - hi - lo }, l]
}
fn from_hsl(h: C) -> C {
    let hc = (1.0 - (2.0 * h[2] - 1.0).abs()) * h[1] * 0.5;
    hue_range(h[0], (h[2] - hc) * 255.0, (h[2] + hc) * 255.0)
}
fn from_hsvl(h: f32, s: f32, l: f32) -> C {
    let c = hue_range(h, 1.0 - s, 1.0);
    let k = l / luma(c).max(1e-6);
    c.map(|v| (v * k).clamp(0.0, 255.0))
}
fn per(s: C, b: C, f: impl Fn(f32, f32) -> f32) -> C {
    [f(s[0], b[0]), f(s[1], b[1]), f(s[2], b[2])]
}
fn u8c(v: f32) -> u32 {
    v.max(0.0) as u32
}

fn blend(mode: usize, s: C, b: C) -> C {
    match mode {
        0 => per(s, b, |s, b| (s + b).min(255.0)),
        1 => per(s, b, |s, b| (b - s).max(0.0)),
        2 => per(s, b, |s, b| s * b / 255.0),
        3 => per(s, b, |s, b| (b * 256.0 / (s + 1.0)).min(255.0)),
        4 => per(s, b, |s, b| 255.0 - (255.0 - s) * (255.0 - b) / 255.0),
        5 => per(s, b, |s, b| if b < 128.0 { 2.0 * s * b / 255.0 } else { 2.0 * (b + s - b * s / 255.0) - 255.0 }),
        6 => per(s, b, |s, b| b / 255.0 * (b + (255.0 - b) * (2.0 * s / 255.0))),
        7 => per(s, b, f32::max),
        8 => per(s, b, f32::min),
        9 => {
            let d = luma([b[0] - s[0], b[1] - s[1], b[2] - s[2]]);
            s.map(|v| (v + d).clamp(0.0, 255.0))
        }
        10 => per(s, b, |s, b| (s + b - 255.0).max(0.0)),
        11 => per(s, b, |s, b| (b + 2.0 * s - 255.0).clamp(0.0, 255.0)),
        12 => per(s, b, |s, b| (s + 2.0 * b - 255.0).clamp(0.0, 255.0)),
        13 => per(s, b, |s, b| (b - s).abs()),
        14 => per(s, b, |s, b| s + b - 2.0 * s * b / 255.0),
        15 => per(s, b, |s, b| 255.0 - (255.0 - s - b).abs()),
        16 => {
            if luma([s[0] - b[0], s[1] - b[1], s[2] - b[2]]) > 0.0 {
                b.map(|v| 255.0 - v)
            } else {
                b
            }
        }
        17 => if s.iter().sum::<f32>() < b.iter().sum::<f32>() { b } else { s },
        18 => if s.iter().sum::<f32>() < b.iter().sum::<f32>() { s } else { b },
        19 => per(s, b, |s, b| (b * 256.0 / (256.0 - s)).min(255.0)),
        20 => per(s, b, |s, b| (s * 256.0 / (256.0 - b)).min(255.0)),
        21 => per(s, b, |s, b| if s + b < 255.0 { b * 128.0 / (256.0 - s) } else { 255.0 - (256.0 - s) / (2.0 * (b + 1.0)) }),
        22 => per(s, b, |s, b| (255.0 - (255.0 - b) * 256.0 / (s + 1.0)).max(0.0)),
        23 => per(s, b, |s, b| (255.0 - (255.0 - s) * 256.0 / (b + 1.0)).max(0.0)),
        24 => per(s, b, |s, b| if s + b < 256.0 { s * 127.5 / (256.0 - b) } else { 255.0 - (256.0 - b) * 255.0 / (2.0 * (s + 1.0)) }),
        25 => per(s, b, |s, b| if s <= 128.0 { 2.0 * s * b / 256.0 } else { 255.0 - (255.0 - b) * (511.0 - 2.0 * s) / 255.0 }),
        26 => per(s, b, |s, b| {
            let e = if s < 128.0 { (255.0 - s) / 128.0 } else { 128.0 / s };
            if b == 0.0 { 0.0 } else { 255.0 * (b / 255.0).powf(e) }
        }),
        27 => per(s, b, |s, b| ((255.0 - (255.0 - s) * (255.0 - b) / 255.0) * b + s * (255.0 - b)) / 255.0),
        28 => per(s, b, |s, b| {
            if s < 128.0 { ((255.0 - 2.0 * s) * b * b / 255.0 + 2.0 * s * b) / 255.0 } else { (2.0 * s - 255.0) * (b / 255.0).sqrt() + 2.0 * b * (1.0 - s / 255.0) }
        }),
        29 => {
            let v = |s: f32, b: f32, up: bool| if up { b * 255.0 / (511.0 - 2.0 * s) } else { 255.0 - (255.0 - b) * 255.0 / (2.0 * (s + 1.0)) };
            [v(s[0], b[0], s[0] >= 128.0), v(s[1], b[1], s[1] > 128.0), v(s[2], b[2], s[2] > 128.0)].map(|x| x.clamp(0.0, 255.0))
        }
        30 => {
            let v = |s: f32, b: f32, up: bool| {
                if up {
                    255.0 - (255.0 - b) * 255.0 / (2.0 * s - 255.0)
                } else if s == 127.5 {
                    if b == 0.0 { 0.0 } else { 255.0 }
                } else {
                    b * 255.0 / (255.0 - 2.0 * s)
                }
            };
            [v(s[0], b[0], s[0] >= 128.0), v(s[1], b[1], s[1] > 128.0), v(s[2], b[2], s[2] > 128.0)].map(|x| x.clamp(0.0, 255.0))
        }
        31 => per(s, b, |s, b| if s < 128.0 { s.max(b) } else { s.min(b) }),
        32 => per(s, b, |s, b| if s < 128.0 { s.min(b) } else { s.max(b) }),
        33 => per(s, b, |s, b| if s < 128.0 { (2.0 * s).min(b) } else { (2.0 * s - 255.0).max(b) }),
        34 => per(s, b, |s, b| if s + b >= 255.0 { 255.0 } else { 0.0 }),
        35 => { let h = to_hsv(s); if h[1] == 0.0 { b } else { hue_range(h[0], min_c(b), max_c(b)) } }
        36 => { let h = to_hsv(s); if h[1] == 0.0 { b } else { from_hsvl(h[0], to_hsv(b)[1], luma(b)) } }
        37 => {
            let h = to_hsv(s);
            let bh = hue(b);
            let mut d = h[0] - bh;
            if d < 0.0 {
                d += 360.0;
            }
            hue_range(bh - h[2] * h[1] * d / 255.0, min_c(b), max_c(b))
        }
        38 => { let h = to_hsv(s); if h[1] == 0.0 { b } else { from_hsv([h[0], h[1], max_c(b)]) } }
        39 => { let h = to_hsl(s); if h[1] == 0.0 { b } else { from_hsl([h[0], h[1], (min_c(b) + max_c(b)) / 510.0]) } }
        40 => { let h = to_hsv(s); if h[1] == 0.0 { b } else { from_hsvl(h[0], h[1], luma(b)) } }
        41 => from_hsv([hue(b), to_hsv(s)[1], max_c(b)]),
        42 => { let bh = to_hsl(b); from_hsl([bh[0], to_hsl(s)[1], bh[2]]) }
        43 => from_hsvl(hue(b), to_hsv(s)[1], luma(b)),
        44 => { let h = to_hsv(b); from_hsv([h[0], h[1], max_c(s)]) }
        45 => { let h = to_hsl(b); from_hsl([h[0], h[1], (min_c(s) + max_c(s)) / 510.0]) }
        46 => from_hsvl(hue(b), to_hsv(b)[1], luma(s)),
        47 => [255.0 - (255.0 - s[0]) * (255.0 - b[0]) / 255.0, b[1], b[2]],
        48 => [b[0], s[1], b[2]],
        49 => [b[0], b[1], s[2]],
        50 => [s[0], s[1], b[2]],
        51 => [b[0], s[1], s[2]],
        52 => [s[0], b[1], s[2]],
        53 => per(s, b, |s, b| (s + b) * 0.5),
        54 => per(s, b, |s, b| (s * b).sqrt()),
        55 => per(s, b, |s, b| if s + b == 0.0 { 0.0 } else { 2.0 * s * b / (s + b) }),
        56 => per(s, b, |s, b| (s * s / (256.0 - b)).min(255.0)),
        57 => per(s, b, |s, b| (b * b / (256.0 - s)).min(255.0)),
        58 => per(s, b, |s, b| (255.0 - (255.0 - b) * (255.0 - b) / (s + 1.0)).max(0.0)),
        59 => per(s, b, |s, b| (255.0 - (255.0 - s) * (255.0 - s) / (b + 1.0)).max(0.0)),
        60 => per(s, b, |s, b| (b - s + 128.0).clamp(0.0, 255.0)),
        61 => per(s, b, |s, b| (b + s - 128.0).clamp(0.0, 255.0)),
        62 => per(s, b, |s, b| 255.0 - (b - s).abs()),
        63 => per(s, b, |s, b| (u8c(s) & u8c(b)) as f32),
        64 => per(s, b, |s, b| (u8c(s) | u8c(b)) as f32),
        65 => per(s, b, |s, b| (u8c(s) ^ u8c(b)) as f32),
        66 => per(s, b, |s, b| {
            let (sh, v) = (u8c(s) & 7, u8c(b));
            (((v << sh) | (v >> (8 - sh))) & 255) as f32
        }),
        67 => per(s, b, |s, b| (u8c(s + b) & 255) as f32),
        68 => per(s, b, |s, b| (((b - s) as i32) & 255) as f32),
        69 => [(s[0] + b[0]).min(255.0), s[1], s[2]],
        70 => [s[0], (s[1] + b[1]).min(255.0), s[2]],
        71 => [s[0], s[1], (s[2] + b[2]).min(255.0)],
        72 => [(s[0] + b[0]).min(255.0), (s[1] + b[1]).min(255.0), s[2]],
        73 => [s[0], (s[1] + b[1]).min(255.0), (s[2] + b[2]).min(255.0)],
        74 => [(s[0] + b[0]).min(255.0), s[1], (s[2] + b[2]).min(255.0)],
        _ => per(s, b, |s, b| if s >= b { 0.0 } else { 255.0 }),
    }
}

fn render(f: &mut Frame) -> FxResult {
    let mode = f.i(1);
    let Some(bg) = f.layer_image(0) else { return Err("背景のレイヤーを選んでください") };
    let src = f.input();
    let mut out = Image::new(src.w, src.h);
    out.map(|x, y, _| {
        let (sp, bp) = (src.px(x, y), bg.px(x.min(bg.w - 1), y.min(bg.h - 1)));
        if sp[3] <= 0.0 {
            return bp;
        }
        if bp[3] <= 0.0 {
            return sp;
        }
        let (sd, bd) = (to_display(sp), to_display(bp));
        let s: C = [sd[0], sd[1], sd[2]].map(|v| v.clamp(0.0, 1.0));
        let b: C = [bd[0], bd[1], bd[2]].map(|v| v.clamp(0.0, 1.0));
        let m = blend(mode, s.map(|v| v * 255.0), b.map(|v| v * 255.0)).map(|v| (v / 255.0).clamp(0.0, 1.0));
        let (sa, ba) = (sd[3], bd[3]);
        let (ov, so, bo) = (sa * ba, sa * (1.0 - ba), ba * (1.0 - sa));
        let a = sa + bo;
        let rgb: C = std::array::from_fn(|k| (s[k] * so + b[k] * bo + m[k] * ov) / a.max(1e-6));
        from_display([rgb[0], rgb[1], rgb[2], a])
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "sevenc-nanashi（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.extra_blend_mode", name: "拡張合成モード（extra_blend_mode）", category: "合成", flags: THREAD_SAFE,
            params: [
                layer(c"background", c"背景レイヤー"),
                choice(c"mode", c"合成モード", 0, c"加算|減算|乗算|除算|スクリーン|オーバーレイ|オーバーレイ(gimp)|比較(明)|比較(暗)|色差|陰影(焼き込み(リニア))|明暗(リニアライト)|明暗(逆)|差分|除外|ネガ|反転|カラー比較(明)|カラー比較(暗)|覆い焼き(カラー)|覆い焼き(逆カラー)|覆い焼き(ソフト)|焼き込み(カラー)|焼き込み(逆カラー)|焼き込み(ソフト)|ハードライト|ソフトライト|ソフトライト(gimp)|ソフトライト(photoshop)|ビビッドライト|ビビッドライト(photoshop)|ピンライト|ピンライト(逆)|ピンライト(強)|ハードミックス|色相|色相(HSVL)|色相(δ)|カラー(明度)|カラー(輝度)|カラー(HSVL)|彩度(明度)|彩度(輝度)|彩度(HSVL)|明度|輝度|輝度(HSVL)|RED|GREEN|BLUE|YELLOW|AQUA|MAGENTA|相加平均|相乗平均|調和平均|反射|グロー|寒色|暖色|微粒取り出し|微粒結合|フェニックス|AND|OR|XOR|SHIFT|加算(unsigned)|減算(unsigned)|加算(RED)|加算(GREEN)|加算(BLUE)|加算(RED-GREEN)|加算(GREEN-BLUE)|加算(BLUE-RED)|二値化"),
            ],
            render: render,
        },
    ]
}

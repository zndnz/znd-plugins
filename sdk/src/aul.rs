//! AviUtl のスクリプトがよく呼ぶ組み込みフィルタ（`obj.effect("...")`）の代わり。
//! AviUtl の値の感覚（表示値・0〜255・% の倍率）に合わせてある。

use crate::color::{from_display, hsl_to_rgb, luma601, rgb_to_hsl, to_display};
use crate::image::Image;

/// 表示値（ストレートアルファ）で各画素を変換する。`f` は [r, g, b, a] を受け取って返す。
pub fn map_display(img: &mut Image, mut f: impl FnMut([f32; 4]) -> [f32; 4]) {
    for p in img.data.chunks_exact_mut(4) {
        if p[3] <= 0.0 && p[0] == 0.0 && p[1] == 0.0 && p[2] == 0.0 {
            continue;
        }
        let o = from_display(f(to_display([p[0], p[1], p[2], p[3]])));
        p.copy_from_slice(&o);
    }
}

/// 単色化: `color` は表示値の RGB。`strength` は 0〜1。`keep_luma` なら元の輝度を保つ。
pub fn monochrome(img: &mut Image, color: [f32; 3], strength: f32, keep_luma: bool) {
    let cl = luma601(color[0], color[1], color[2]).max(1e-4);
    map_display(img, |c| {
        let target = if keep_luma {
            let l = luma601(c[0], c[1], c[2]);
            // 輝度を保つ: 色の輝度を元の輝度に合わせる（明るい側は白へ寄せる）
            let k = l / cl;
            if k <= 1.0 {
                [color[0] * k, color[1] * k, color[2] * k]
            } else {
                let t = ((l - cl) / (1.0 - cl).max(1e-4)).clamp(0.0, 1.0);
                [color[0] + (1.0 - color[0]) * t, color[1] + (1.0 - color[1]) * t, color[2] + (1.0 - color[2]) * t]
            }
        } else {
            color
        };
        [c[0] + (target[0] - c[0]) * strength, c[1] + (target[1] - c[1]) * strength, c[2] + (target[2] - c[2]) * strength, c[3]]
    });
}

/// 色調補正。brightness は -100〜100（0 で変化なし）、contrast・luminance・saturation は %（100 で変化なし）、
/// hue は度、gamma は -100〜100（0 で変化なし。正で明るく）。
pub fn color_correct(img: &mut Image, brightness: f32, contrast: f32, hue: f32, luminance: f32, saturation: f32, gamma: f32) {
    let (b, c, l, s) = (brightness / 100.0, contrast / 100.0, luminance / 100.0, saturation / 100.0);
    let g = 2f32.powf(gamma / 100.0);
    map_display(img, |p| {
        let mut rgb = [p[0], p[1], p[2]];
        for v in &mut rgb {
            *v = (*v * l + b).max(0.0);
            *v = (*v - 0.5) * c + 0.5;
            *v = v.clamp(0.0, 1.0).powf(1.0 / g);
        }
        if hue != 0.0 || (s - 1.0).abs() > 1e-6 {
            let (h, sat, li) = rgb_to_hsl(rgb[0], rgb[1], rgb[2]);
            let (r, gg, bb) = hsl_to_rgb(h + hue, (sat * s).clamp(0.0, 1.0), li);
            rgb = [r, gg, bb];
        }
        [rgb[0].clamp(0.0, 1.0), rgb[1].clamp(0.0, 1.0), rgb[2].clamp(0.0, 1.0), p[3]]
    });
}

/// ぼかし（範囲は px）。AviUtl の「ぼかし」に見た目を近づけたガウスぼかし。
pub fn blur(img: &mut Image, range: f32) {
    img.gaussian_blur(range / 2.0);
}

/// 左右反転。
pub fn flip_h(img: &mut Image) {
    let w = img.w;
    for row in img.data.chunks_exact_mut(w * 4) {
        for x in 0..w / 2 {
            for k in 0..4 {
                row.swap(x * 4 + k, (w - 1 - x) * 4 + k);
            }
        }
    }
}

/// 上下反転。
pub fn flip_v(img: &mut Image) {
    let (w, h) = (img.w, img.h);
    for y in 0..h / 2 {
        for i in 0..w * 4 {
            img.data.swap(y * w * 4 + i, (h - 1 - y) * w * 4 + i);
        }
    }
}

/// 不透明度（0〜1）を掛ける。
pub fn fade(img: &mut Image, a: f32) {
    img.data.iter_mut().for_each(|v| *v *= a);
}

/// スクリーン合成（下に `dst`、上に `src`。どちらもプリマルチプライド、同じ大きさ）。
pub fn screen(dst: &mut Image, src: &Image) {
    for (d, s) in dst.data.chunks_exact_mut(4).zip(src.data.chunks_exact(4)) {
        for k in 0..3 {
            d[k] = d[k] + s[k] - d[k] * s[k];
        }
        d[3] = d[3] + s[3] - d[3] * s[3];
    }
}

/// AviUtl の合成モードの番号（`obj.setoption("blend", n)`）で、1 色ぶん（表示値）を混ぜる。
/// 0 通常 / 1 加算 / 2 減算 / 3 乗算 / 4 スクリーン / 5 オーバーレイ / 6 比較(明) / 7 比較(暗) /
/// 8 輝度 / 9 色差 / 10 陰影 / 11 明暗 / 12 差分。知らない番号は通常。
pub fn blend_color(mode: usize, b: [f32; 3], s: [f32; 3]) -> [f32; 3] {
    let lum = |c: [f32; 3]| 0.299 * c[0] + 0.587 * c[1] + 0.114 * c[2];
    let set_lum = |c: [f32; 3], l: f32| {
        let d = l - lum(c);
        let c = [c[0] + d, c[1] + d, c[2] + d];
        let (l, n, x) = (lum(c), c[0].min(c[1]).min(c[2]), c[0].max(c[1]).max(c[2]));
        c.map(|v| {
            let mut v = v;
            if n < 0.0 {
                v = l + (v - l) * l / (l - n).max(1e-6);
            }
            if x > 1.0 {
                v = l + (v - l) * (1.0 - l) / (x - l).max(1e-6);
            }
            v
        })
    };
    let per = |f: fn(f32, f32) -> f32| [f(b[0], s[0]), f(b[1], s[1]), f(b[2], s[2])];
    match mode {
        1 => per(|b, s| (b + s).min(1.0)),
        2 => per(|b, s| (b - s).max(0.0)),
        3 => per(|b, s| b * s),
        4 => per(|b, s| b + s - b * s),
        5 => per(|b, s| if b < 0.5 { 2.0 * b * s } else { 1.0 - 2.0 * (1.0 - b) * (1.0 - s) }),
        6 => per(f32::max),
        7 => per(f32::min),
        8 => set_lum(b, lum(s)),
        9 => set_lum(s, lum(b)),
        // 陰影: 焼き込み（カラー）、明暗: ビビッドライト に近い計算
        10 => per(|b, s| if s <= 0.0 { 0.0 } else { (1.0 - (1.0 - b) / s).max(0.0) }),
        11 => per(|b, s| {
            if s < 0.5 {
                if s <= 0.0 { 0.0 } else { (1.0 - (1.0 - b) / (2.0 * s)).max(0.0) }
            } else if s >= 1.0 {
                1.0
            } else {
                (b / (2.0 * (1.0 - s))).min(1.0)
            }
        }),
        12 => per(|b, s| (b - s).abs()),
        _ => s,
    }
}

/// `src` を `dst` の上に合成モード `mode`（[`blend_color`] の番号）・不透明度 `opacity` で重ねる。
/// どちらもプリマルチプライドのリニア、同じ大きさ。色の計算は表示値で行う。
pub fn blend(dst: &mut Image, src: &Image, mode: usize, opacity: f32) {
    for (d, s) in dst.data.chunks_exact_mut(4).zip(src.data.chunks_exact(4)) {
        let sa = s[3] * opacity;
        if sa <= 0.0 {
            continue;
        }
        let cd = to_display([d[0], d[1], d[2], d[3]]);
        let cs = to_display([s[0], s[1], s[2], s[3]]);
        let da = cd[3];
        let bl = blend_color(mode, [cd[0], cd[1], cd[2]], [cs[0], cs[1], cs[2]]);
        let ao = sa + da * (1.0 - sa);
        let mut o = [0.0; 4];
        for k in 0..3 {
            // W3C の合成: 下が透明なところは上の色そのまま
            let mix = cs[k] * sa * (1.0 - da) + bl[k] * sa * da + cd[k] * da * (1.0 - sa);
            o[k] = (mix / ao.max(1e-8)).clamp(0.0, 1.0);
        }
        o[3] = ao;
        d.copy_from_slice(&from_display(o));
    }
}

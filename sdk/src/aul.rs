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

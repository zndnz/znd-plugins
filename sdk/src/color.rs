//! 色の変換。znd の画像はリニアライト・プリマルチプライド。AviUtl のスクリプトの多くは
//! sRGB の表示値（0〜255）で計算するので、移植ではいったん表示値に直して計算し、戻すことが多い。

#[inline]
pub fn srgb_to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

#[inline]
pub fn linear_to_srgb(v: f32) -> f32 {
    if v <= 0.0031308 {
        v * 12.92
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}

/// プリマルチプライドのリニア → ストレートの表示値 (r, g, b, a)。
#[inline]
pub fn to_display(p: [f32; 4]) -> [f32; 4] {
    if p[3] <= 1e-8 {
        return [0.0, 0.0, 0.0, 0.0];
    }
    let a = p[3];
    [linear_to_srgb((p[0] / a).max(0.0)), linear_to_srgb((p[1] / a).max(0.0)), linear_to_srgb((p[2] / a).max(0.0)), a]
}

/// ストレートの表示値 → プリマルチプライドのリニア。
#[inline]
pub fn from_display(c: [f32; 4]) -> [f32; 4] {
    let a = c[3].clamp(0.0, 1.0);
    [srgb_to_linear(c[0]) * a, srgb_to_linear(c[1]) * a, srgb_to_linear(c[2]) * a, a]
}

/// ストレートのリニア色（パラメータの色）とアルファから、プリマルチプライドの画素。
#[inline]
pub fn premul(c: [f32; 4], alpha: f32) -> [f32; 4] {
    let a = c[3] * alpha;
    [c[0] * a, c[1] * a, c[2] * a, a]
}

/// 輝度（Rec.709）。
#[inline]
pub fn luma(r: f32, g: f32, b: f32) -> f32 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// AviUtl の「輝度」に近い Rec.601 の輝度。
#[inline]
pub fn luma601(r: f32, g: f32, b: f32) -> f32 {
    0.299 * r + 0.587 * g + 0.114 * b
}

/// RGB（0〜1）→ HSV（h: 0〜360, s, v: 0〜1）。
pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let h = if d <= 1e-8 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / d).rem_euclid(6.0))
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    let s = if max <= 1e-8 { 0.0 } else { d / max };
    (h, s, max)
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let h = h.rem_euclid(360.0) / 60.0;
    let c = v * s;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let (r, g, b) = match h as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    (r + m, g + m, b + m)
}

/// RGB → HSL（h: 0〜360）。
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let d = max - min;
    if d <= 1e-8 {
        return (0.0, 0.0, l);
    }
    let s = d / (1.0 - (2.0 * l - 1.0).abs()).max(1e-8);
    let (h, _, _) = rgb_to_hsv(r, g, b);
    (h, s.min(1.0), l)
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h.rem_euclid(360.0) / 60.0;
    let x = c * (1.0 - ((hp % 2.0) - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    (r + m, g + m, b + m)
}

/// 0xRRGGBB → 表示値の (r, g, b)。
pub fn hex(c: u32) -> [f32; 3] {
    [((c >> 16) & 255) as f32 / 255.0, ((c >> 8) & 255) as f32 / 255.0, (c & 255) as f32 / 255.0]
}

/// "#rrggbb" / "rrggbb" / "0xrrggbb" を読む。
pub fn parse_hex(s: &str) -> Option<[f32; 3]> {
    let t = s.trim().trim_start_matches('#').trim_start_matches("0x");
    if t.len() != 6 {
        return None;
    }
    u32::from_str_radix(t, 16).ok().map(hex)
}

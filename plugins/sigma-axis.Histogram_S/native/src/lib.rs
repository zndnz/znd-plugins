//! Histogram_S（σ軸）の移植: 画像の色の分布（ヒストグラム）を 12 種類の色空間で表示する編集補助。

use znd_plugin_sdk::color::{from_display, to_display};
use znd_plugin_sdk::*;

fn to_lin(c: f32) -> f32 {
    color::srgb_to_linear(c.abs()) * c.signum()
}

fn mul3(m: [[f32; 3]; 3], c: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| m[i][0] * c[0] + m[i][1] * c[1] + m[i][2] * c[2])
}

/// 色空間の値（0..1 の目盛り）と、色相の重み用の (彩度のチャンネル, 色相のチャンネル)
fn convert(space: usize, rgb: [f32; 3]) -> [f32; 3] {
    match space {
        0 => mul3([[0.4124, 0.3576, 0.1805], [0.2126, 0.7152, 0.0722], [0.0193, 0.1192, 0.9505]], rgb.map(to_lin)),
        1 => rgb,
        2 => rgb.map(to_lin),
        3 | 4 => {
            let lms = mul3([[0.41222146, 0.53633255, 0.051445995], [0.2119035, 0.6806995, 0.10739696], [0.08830246, 0.28171885, 0.6299787]], rgb.map(to_lin)).map(|v| v.signum() * v.abs().cbrt());
            let lab = mul3([[0.21045426, 0.7936178, -0.004072047], [1.9779985, -2.4285922, 0.4505937], [0.025904037, 0.78277177, -0.80867577]], lms);
            if space == 4 {
                let c = lab[1].hypot(lab[2]);
                let h = if c > 1.0 / 4096.0 { 0.5 + (-lab[2]).atan2(-lab[1]) / std::f32::consts::TAU } else { -32768.0 };
                [lab[0], 2.0 * 1.5 * c, h]
            } else {
                [lab[0], lab[1] * 1.5 + 0.5, lab[2] * 1.5 + 0.5]
            }
        }
        5..=7 => {
            let m = match space {
                5 => [[0.299, 0.587, 0.114], [-0.16873589, -0.33126411, 0.5], [0.5, -0.41868759, -0.08131241]],
                6 => [[0.2126, 0.7152, 0.0722], [-0.11457211, -0.38542789, 0.5], [0.5, -0.45415291, -0.04584709]],
                _ => [[0.2627, 0.678, 0.0593], [-0.13963006, -0.36036994, 0.5], [0.5, -0.4597857, -0.0402143]],
            };
            let y = mul3(m, rgb);
            [y[0], y[1] + 0.5, y[2] + 0.5]
        }
        _ => {
            // HSV（円柱・円錐）/ HSL（円柱・双円錐）
            let model = space - 8;
            let mx = rgb[0].max(rgb[1]).max(rgb[2]);
            let mn = rgb[0].min(rgb[1]).min(rgb[2]);
            let d = mx - mn;
            let h = if d > 0.0 { color::rgb_to_hsv(rgb[0], rgb[1], rgb[2]).0 / 360.0 } else { -32768.0 };
            let (mut s, mut v) = (d, mx);
            if model >= 2 {
                v -= s / 2.0;
                if model == 2 {
                    let u = 2.0 * v.min(1.0 - v);
                    s = if u != 0.0 { s / u } else { 0.0 };
                }
            } else if model == 0 {
                s = if v != 0.0 { s / v } else { 0.0 };
            }
            [h, s, v]
        }
    }
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let width = (f.f(0).round().max(256.0) * s).round().max(64.0) as usize;
    let height = (f.f(1).round().max(16.0) * s).round().max(8.0) as usize;
    let space = f.i(3);
    let hue_weighted = f.b(4);
    let with_alpha = f.b(5);
    let amplify = (f.f(6) / 100.0).clamp(0.0, 16.0) as f32;
    let log = f.i(7) == 1;
    let mollify = f.f(8).round().clamp(0.0, 15.0) as usize;
    let draw_c = f.color(9);
    let back_c = f.color(10);
    let back_a = (1.0 - f.f(11) / 100.0).clamp(0.0, 1.0) as f32;
    let colorful = f.b(12);
    let grid = f.b(13);
    let src = if f.has_layer(2) { f.layer_image(2).unwrap_or_else(|| f.input()) } else { f.input() };
    // ビン 0..255 と合計（256）を、チャンネルごとに数える
    let mut hist = vec![[0.0f64; 4]; 257];
    let (wt_src, wt_dst) = match space {
        4 => (Some(1usize), 2usize),
        8..=11 => (Some(1), 0),
        _ => (None, 0),
    };
    for p in src.data.chunks_exact(4) {
        let a = p[3];
        let d = to_display([p[0], p[1], p[2], a]);
        let rgb = if a > 0.0 { [d[0], d[1], d[2]] } else { [0.0; 3] };
        let c = convert(space, rgb);
        let vals = [c[0], c[1], c[2], a];
        let mut w = [a, a, a, 1.0];
        if let (true, Some(ws)) = (hue_weighted, wt_src) {
            w[wt_dst] *= vals[ws];
        }
        for ch in 0..4 {
            let v = vals[ch];
            hist[256][ch] += w[ch] as f64;
            if v >= 1.0 {
                continue;
            }
            let bin = if v < 1.0 / 256.0 { 0 } else { ((v * 256.0) as usize).min(255) };
            hist[bin][ch] += w[ch] as f64;
        }
    }
    let n = (src.w * src.h).max(1) as f64;
    let total: [f64; 4] = std::array::from_fn(|ch| (hist[256][ch] / n * 32768.0).max(4096.0));
    let mut norm: Vec<[f32; 4]> = (0..256).map(|b| std::array::from_fn(|ch| (32768.0 * (hist[b][ch] / n * 32768.0) / total[ch]) as f32)).collect();
    if mollify > 0 {
        let span = mollify + 1;
        let (d0, d1) = ((span >> 1) as isize, (span - (span >> 1)) as isize);
        let orig = norm.clone();
        for x in 0..256isize {
            let mut sum = [0.0f32; 4];
            for dx in -d0..d1 {
                let i = x + dx;
                if (0..256).contains(&i) {
                    for ch in 0..4 {
                        sum[ch] += orig[i as usize][ch];
                    }
                }
            }
            let cnt = (x.min(d0) + (256 - x).min(d1)) as f32;
            norm[x as usize] = sum.map(|v| v / cnt);
        }
    }
    // 色（表示値）
    let dc = to_display(color::premul(draw_c, 1.0));
    let dcol = [dc[0], dc[1], dc[2], 1.0];
    let bd = to_display(color::premul(back_c, 1.0));
    let back = [bd[0] * back_a, bd[1] * back_a, bd[2] * back_a, back_a];
    let ga = if grid { 0.5 } else { 0.0 };
    let gridc = [ga * dcol[0] + (1.0 - ga) * back[0], ga * dcol[1] + (1.0 - ga) * back[1], ga * dcol[2] + (1.0 - ga) * back[2], ga + back_a - ga * back_a];
    let hx = |v: u32| [((v >> 16) & 255) as f32 / 255.0, ((v >> 8) & 255) as f32 / 255.0, (v & 255) as f32 / 255.0, 1.0];
    let mut colors = [dcol; 8];
    let mut hue_ch: i32 = -1;
    if colorful {
        match space {
            0..=2 => {
                for i in 0..2 {
                    colors[i] = [1.0, 0.0, 0.0, 1.0];
                    colors[2 + i] = [0.0, 1.0, 0.0, 1.0];
                    colors[4 + i] = [0.0, 0.0, 1.0, 1.0];
                }
            }
            3 => {
                colors[2] = hx(0x00ffd3);
                colors[3] = hx(0xff0089);
                colors[4] = hx(0x2f00ff);
                colors[5] = hx(0xffcb00);
            }
            4 => {
                for (i, c) in [0xff0089, 0xff7700, 0xd6ff00, 0x00ffd3, 0x00adff, 0x9600ff].iter().enumerate() {
                    colors[i] = hx(*c);
                }
                hue_ch = 2;
            }
            5..=7 => {
                colors[2] = hx(0xd6ff00);
                colors[3] = hx(0x2900ff);
                colors[4] = hx(0x00ffa9);
                colors[5] = hx(0xff0056);
            }
            _ => {
                for (i, c) in [0xff0000, 0xffff00, 0x00ff00, 0x00ffff, 0x0000ff, 0xff00ff].iter().enumerate() {
                    colors[i] = hx(*c);
                }
                hue_ch = 0;
            }
        }
    }
    let channels = if with_alpha { 4 } else { 3 };
    let (iw, ih) = (width + 16, (height + 8) * channels + 8);
    let mut img = Image::new(iw, ih);
    img.map(|x, y, _| {
        let mut px = x;
        let col = if px < 8 || px >= width + 8 {
            back
        } else {
            px -= 8;
            let ch = y / (height + 8);
            let py = y - ch * (height + 8);
            if ch >= channels {
                back
            } else {
                let section = (256 * px / width).min(255);
                let thresh = 1.0 - (py as f32 - 8.0) / height as f32;
                let mut val = norm[section][ch] / 128.0;
                val = if val > 0.0 && log { amplify / 16.0 * val.log2() + 0.75 } else { amplify / 2.0 * val };
                if val <= thresh {
                    if py >= 8 && (px == 0 || px == width - 1 || py == height + 7) {
                        std::array::from_fn(|k| gridc[k] + (1.0 - gridc[3]) * back[k])
                    } else {
                        back
                    }
                } else {
                    let idx = if hue_ch < 0 { 2 * ch + (section >> 7) } else if ch as i32 == hue_ch { (6 * ((section + 21) & 255)) >> 8 } else { 6 };
                    let fade = (py as f32 / 8.0).clamp(0.0, 1.0);
                    let c = colors[idx].map(|v| v * fade);
                    std::array::from_fn(|k| c[k] + (1.0 - c[3]) * back[k])
                }
            }
        };
        // 表示値（プリマルチプライド）→ リニア
        if col[3] <= 0.0 {
            [0.0; 4]
        } else {
            from_display([(col[0] / col[3]).min(1.0), (col[1] / col[3]).min(1.0), (col[2] / col[3]).min(1.0), col[3]])
        }
    });
    let ox = (f.width as i32 - iw as i32) / 2;
    let oy = (f.height as i32 - ih as i32) / 2;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "σ軸（znd 移植: znd-plugins）", version: "1.10", license: "MIT",
    effects: [
        {
            id: "sigma-axis.Histogram_S", name: "ヒストグラム（Histogram_S）", category: "編集補助", flags: THREAD_SAFE,
            params: [
                float(c"width", c"幅", 256.0, 256.0, 1024.0),
                float(c"height", c"高さ", 100.0, 16.0, 400.0),
                layer(c"target", c"対象のレイヤー（なしで自分）"),
                choice(c"space", c"色空間", 1, c"XYZ|RGB|sRGB（リニア）|Oklab|OKLCH|YUV（BT.601）|YUV（BT.709）|YUV（BT.2020）|HSV（円柱）|HSV（円錐）|HSL（円柱）|HSL（双円錐）"),
                boolean(c"hue_weighted", c"色相を彩度で重み付け", false),
                boolean(c"alpha", c"アルファ値", false),
                float(c"amplify", c"拡大率", 100.0, 0.0, 1600.0),
                choice(c"scale", c"スケール", 0, c"線形スケール|対数スケール"),
                float(c"mollify", c"平滑化", 0.0, 0.0, 15.0),
                color_hex(c"col_draw", c"描画色", 0xffffff),
                color_hex(c"col_back", c"背景色", 0x000000),
                float(c"alpha_back", c"背景透明度", 0.0, 0.0, 100.0),
                boolean(c"colorful", c"色を付ける", true),
                boolean(c"grid", c"罫線", true),
            ],
            render: render,
        },
    ]
}

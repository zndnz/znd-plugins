//! 無印ノイズ（猫愛）の移植: AviUtl（無印）のフィルタ「ノイズ」の 6 種類を、同じ表と整数演算で再現する。
//! 表（table.bin）は元のパッケージの aviutl_noise_table.png のアルファから取り出した 16bit 値（257 行 × 256）。

use znd_plugin_sdk::*;

static TABLE: &[u8] = include_bytes!("table.bin");

fn u16_at(x: i32, y: i32) -> i32 {
    let i = ((y as usize) * 256 + (x as usize)) * 2;
    TABLE[i] as i32 | (TABLE[i + 1] as i32) << 8
}
fn table_flat(index: i32) -> i32 {
    let i = index & 0xffff;
    u16_at(i & 255, (i >> 8) & 255) - 2048
}
fn table_xy(x: i32, y: i32) -> i32 {
    u16_at(x & 255, y & 255) - 2048
}
fn fade(f: i32) -> i32 {
    u16_at(f & 255, 256)
}

fn base_noise(x: i32, y: i32, z: i32) -> i32 {
    let (fx, fy, fz) = (x & 255, y & 255, z & 255);
    let (ix, iy, iz) = (x >> 8, y >> 8, z >> 8);
    let s0 = iz.wrapping_mul(79);
    let wx = fade(fx);
    let plane = |s: i32| {
        let (x0, y0) = ((ix + s) & 255, (iy + s) & 255);
        let (x1, y1) = ((x0 + 1) & 255, (y0 + 1) & 255);
        let (a, b, c, d) = (table_xy(x0, y0), table_xy(x1, y0), table_xy(x0, y1), table_xy(x1, y1));
        let top = b + (((a - b) * wx) >> 12);
        let bot = d + (((c - d) * wx) >> 12);
        top + (((bot - top) * fy) >> 8)
    };
    let (n0, n1) = (plane(s0), plane(s0 + 79));
    n1 + (((n0 - n1) * fade(fz)) >> 12)
}

fn octaves(mut x: i32, mut y: i32, mut z: i32, turbulent: bool) -> i32 {
    let mut v = 0;
    for _ in 0..6 {
        let n = base_noise(x, y, z);
        v = (v >> 1) + if turbulent { n.abs() } else { n };
        x = (x >> 1) + 7;
        y = (y >> 1) + 17;
        z = (z >> 1) + 31;
    }
    v
}

fn hash12(s: i32) -> i32 {
    let h73 = s.wrapping_mul(73);
    // 元は符号付きの割り算・剰余（0 に向かって切り捨て）
    let t = (s % 7213).wrapping_add(s).wrapping_add(h73 ^ 0x137bd137);
    let h = t ^ (t.wrapping_shl(7) ^ (t >> 2));
    (h ^ (h / 17)) & 4095
}

fn hash_noise(x: i32, y: i32, z: i32) -> i32 {
    let (fx, fy, fz) = (x & 255, y & 255, z & 255);
    let (ix, iy, iz) = (x >> 8, y >> 8, z >> 8);
    let wx = fade(fx);
    let plane = |shift: i32| {
        let base = ((iy + shift) << 8) + (ix + shift);
        let (a, b, c, d) = (hash12(base), hash12(base + 1), hash12(base + 256), hash12(base + 257));
        let top = b + (((a - b) * wx) >> 12);
        let bot = d + (((c - d) * wx) >> 12);
        top + (((bot - top) * fy) >> 8)
    };
    let s0 = iz.wrapping_mul(79);
    let (n0, n1) = (plane(s0), plane(s0 + 79));
    n1 + (((n0 - n1) * fade(fz)) >> 12)
}

fn noise_value(kind: i32, x: i32, y: i32, z: i32, th: i32, scale: i32) -> i32 {
    let fix = |v: i32| ((v.max(0) as i64 * scale as i64) >> 12).min(4096) as i32;
    match kind {
        2 => fix(4096 - octaves(x, y, z, true) * 2 - th),
        3 => fix(octaves(x, y, z, true) * 2 - th),
        4 => (hash_noise(x, y, z.wrapping_mul(10)) - th).max(0),
        5 => {
            let v = octaves(x, y, z, true);
            let phase = ((2048 - v) as f32 + x as f32 * 0.0625) * 0.0030679615757712823;
            let wave = (phase.sin() * -2048.0) as i32;
            fix(2048 - wave - th)
        }
        6 => {
            let v = octaves(x, y, z, false);
            let s = (v as f32 / 1024.0 * std::f32::consts::TAU).sin();
            let wave = ((s * std::f32::consts::TAU).sin() * -2048.0) as i32;
            fix(2048 - wave - th)
        }
        _ => fix(octaves(x, y, z, false) + 2048 - th),
    }
}

fn trunc(v: f64) -> i32 {
    v.trunc() as i32
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let frame = f.frame() as f64;
    // 周期は 1 画素あたりの量なので、縮小プレビューでは画素あたりの量を増やす
    let px = trunc(f.f(4) / s * 256.0);
    let py = trunc(f.f(5) / s * 256.0);
    let ox = trunc(f.f(1) * frame * 256.0);
    let oy = trunc(f.f(2) * frame * 256.0);
    let oz = trunc(f.f(3) * frame * 25.6);
    let th = trunc(f.f(6) * 4096.0 / 100.0).clamp(0, 4095);
    let scale = 16777216 / (4096 - th);
    let strength = trunc(f.f(0) * 4096.0 / 100.0).clamp(0, 8192);
    let seed = trunc(f.f(7));
    let mode = f.i(8);
    let kind = (f.i(9) as i32 + 1).clamp(1, 6);
    let (w, h) = (f.width as i32, f.height as i32);
    let wi = f.width;
    for (i, p) in f.rgba.chunks_exact_mut(4).enumerate() {
        let (xx, yy) = ((i % wi) as i32, (i / wi) as i32);
        let mut x = xx.wrapping_mul(px) - ((w.wrapping_mul(px)) >> 1) + ox;
        let mut y = yy.wrapping_mul(py) - ((h.wrapping_mul(py)) >> 1) + oy;
        let mut z = oz;
        if seed != 0 {
            x += table_flat(seed) << 8;
            y += table_flat(seed + 1) << 8;
            z += table_flat(seed + 2) << 8;
        }
        let n = noise_value(kind, x, y, z, th, scale);
        let factor = if strength <= 4096 { 4096 + (((n - 4096) * strength) >> 12) } else { (n * (8192 - strength)) >> 12 };
        let k = (factor as f32 / 4096.0).max(0.0);
        if mode == 0 {
            let k = k.min(1.0);
            p.iter_mut().for_each(|v| *v *= k);
        } else {
            // 明るさと乗算: 表示値の RGB に掛ける
            let d = color::to_display([p[0], p[1], p[2], p[3]]);
            let o = color::from_display([(d[0] * k).min(1.0), (d[1] * k).min(1.0), (d[2] * k).min(1.0), d[3]]);
            p.copy_from_slice(&o);
        }
    }
    Ok(())
}

plugin! {
    vendor: "猫愛（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "NeAc.AviUtlNoise", name: "無印ノイズ", category: "スタイライズ", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"strength", c"強さ", 100.0, 0.0, 200.0),
                float(c"vx", c"速度X", 0.0, -800.0, 800.0),
                float(c"vy", c"速度Y", 0.0, -800.0, 800.0),
                float(c"vz", c"変化速度", 0.0, 0.0, 800.0),
                float(c"period_x", c"周期X", 1.0, 0.0, 100.0),
                float(c"period_y", c"周期Y", 1.0, 0.0, 100.0),
                float(c"threshold", c"しきい値", 0.0, 0.0, 100.0),
                float(c"seed", c"シード", 0.0, 0.0, 100.0),
                choice(c"mode", c"合成モード", 0, c"アルファ値と乗算|明るさと乗算"),
                choice(c"noise_type", c"ノイズの種類", 0, c"Type1|Type2|Type3|Type4|Type5|Type6"),
            ],
            render: render,
        },
    ]
}

//! WhiteBalance_S（σ軸）の移植: ホワイトバランス（2 色を指定して一方を他方へ、または成分ごとの倍率）と、色温度（K / Mired）。

use znd_plugin_sdk::color::{from_display, linear_to_srgb, srgb_to_linear, to_display};
use znd_plugin_sdk::*;

const M: [[f64; 3]; 3] = [[0.4124, 0.3576, 0.1805], [0.2126, 0.7152, 0.0722], [0.0193, 0.1192, 0.9505]];
const MI: [[f64; 3]; 3] = [[3.2406254, -1.537208, -0.4986286], [-0.9689307, 1.875756, 0.041517522], [0.055710122, -0.20402105, 1.056996]];

fn mul(m: &[[f64; 3]; 3], c: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| m[i][0] * c[0] + m[i][1] * c[1] + m[i][2] * c[2])
}

/// 倍率を掛ける。xyz なら XYZ 空間（リニア）、そうでなければ表示値の RGB に掛ける
fn apply(f: &mut Frame, rate: [f64; 3], xyz: bool) {
    if rate == [1.0; 3] {
        return;
    }
    for p in f.rgba.chunks_exact_mut(4) {
        let a = p[3];
        if a <= 0.0 {
            continue;
        }
        if xyz {
            let lin = [p[0] as f64 / a as f64, p[1] as f64 / a as f64, p[2] as f64 / a as f64];
            let x = mul(&M, lin);
            let r = mul(&MI, [x[0] * rate[0], x[1] * rate[1], x[2] * rate[2]]);
            for k in 0..3 {
                p[k] = (r[k] * a as f64) as f32;
            }
        } else {
            let d = to_display([p[0], p[1], p[2], a]);
            let o = from_display([(d[0] as f64 * rate[0]) as f32, (d[1] as f64 * rate[1]) as f32, (d[2] as f64 * rate[2]) as f32, a]);
            p.copy_from_slice(&o);
        }
    }
}

fn balance(f: &mut Frame) -> FxResult {
    let rate = (f.f(0) / 100.0).clamp(0.0, 1.0);
    if rate <= 0.0 {
        return Ok(());
    }
    let xyz = f.i(7) == 0;
    let mut comp = [f.f(4) / 100.0, f.f(5) / 100.0, f.f(6) / 100.0].map(|v| v.max(0.0));
    if !f.b(1) {
        // 変換元・変換先の色（パラメータはリニア）
        let (b, d) = (f.color(2), f.color(3));
        let (mut c, mut t) = ([b[0] as f64, b[1] as f64, b[2] as f64], [d[0] as f64, d[1] as f64, d[2] as f64]);
        if xyz {
            c = mul(&M, c);
            t = mul(&M, t);
        } else {
            c = c.map(|v| linear_to_srgb(v as f32) as f64);
            t = t.map(|v| linear_to_srgb(v as f32) as f64);
        }
        if c == t {
            return Ok(());
        }
        comp = std::array::from_fn(|k| t[k] / c[k].max(0.0001));
        if f.b(8) {
            // 灰色の明るさ（Y）が変わらないように正規化する
            let y = if xyz {
                comp[1]
            } else {
                let lin = comp.map(|v| srgb_to_linear(v as f32) as f64);
                linear_to_srgb(mul(&M, lin)[1] as f32) as f64
            };
            if y > 0.0 {
                comp = comp.map(|v| v / y);
            }
        }
    }
    apply(f, comp.map(|v| v.powf(rate)), xyz);
    Ok(())
}

fn calc_xz(t: f64) -> (f64, f64) {
    // https://en.wikipedia.org/wiki/Planckian_locus#Approximation
    let k = 1000.0 / t;
    let x = if t <= 4000.0 { -0.2661239 * k.powi(3) - 0.2343589 * k * k + 0.8776956 * k + 0.179910 } else { -3.0258469 * k.powi(3) + 2.1070379 * k * k + 0.2226347 * k + 0.240390 };
    let y = if t <= 2222.0 {
        -1.1063814 * x.powi(3) - 1.3481102 * x * x + 2.18555832 * x - 0.20219683
    } else if t <= 4000.0 {
        -0.9549476 * x.powi(3) - 1.37418593 * x * x + 2.09137015 * x - 0.16748867
    } else {
        3.081758 * x.powi(3) - 5.8733867 * x * x + 3.75112997 * x - 0.37001483
    };
    (x / y, (1.0 - x) / y - 1.0)
}

fn temperature(f: &mut Frame) -> FxResult {
    let mired = f.i(4) == 1;
    let (mut tb, mut td) = (f.f(0), f.f(1));
    if mired {
        tb = 1e6 / f.f(2).clamp(40.0, 600.0);
        td = 1e6 / f.f(3).clamp(40.0, 600.0);
    }
    let (tb, td) = (tb.clamp(1666.67, 25000.0), td.clamp(1666.67, 25000.0));
    if tb == td {
        return Ok(());
    }
    let xyz = f.i(5) == 0;
    let ((x1, z1), (x2, z2)) = (calc_xz(tb), calc_xz(td));
    let (mut a, mut b) = ([x1, 1.0, z1], [x2, 1.0, z2]);
    if !xyz {
        a = mul(&MI, a);
        b = mul(&MI, b);
    }
    apply(f, std::array::from_fn(|k| b[k] / a[k]), xyz);
    Ok(())
}

plugin! {
    vendor: "σ軸（znd 移植: znd-plugins）", version: "1.10", license: "MIT",
    effects: [
        {
            id: "sigma-axis.WhiteBalance_S", name: "ホワイトバランスσ", category: "カラー補正", flags: THREAD_SAFE,
            params: [
                float(c"rate", c"強さ", 100.0, 0.0, 100.0),
                boolean(c"col_indiv", c"成分を個別に指定", false),
                color_hex(c"col_base", c"変換元", 0xffffff),
                color_hex(c"col_dest", c"変換先", 0xffffff),
                float(c"comp1", c"倍率 X/R", 100.0, 0.0, 200.0),
                float(c"comp2", c"倍率 Y/G", 100.0, 0.0, 200.0),
                float(c"comp3", c"倍率 Z/B", 100.0, 0.0, 200.0),
                choice(c"space", c"色空間", 0, c"XYZ|RGB"),
                boolean(c"normalize", c"正規化", true),
            ],
            render: balance,
        },
        {
            id: "sigma-axis.WhiteBalance_S.temperature", name: "色温度σ", category: "カラー補正", flags: THREAD_SAFE,
            params: [
                float(c"temp_base", c"変換元 K", 6500.0, 1666.67, 25000.0),
                float(c"temp_dest", c"変換先 K", 6500.0, 1666.67, 25000.0),
                float(c"mired_base", c"変換元 M", 154.0, 40.0, 600.0),
                float(c"mired_dest", c"変換先 M", 154.0, 40.0, 600.0),
                choice(c"unit", c"単位", 0, c"K|Mired"),
                choice(c"space", c"色空間", 0, c"XYZ|RGB"),
            ],
            render: temperature,
        },
    ]
}

//! ルーローの多角形（もも猫）の移植: 奇数角のルーロー多角形を描くカスタムオブジェクト。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let size = (f.f(0).max(0.0) * f.scale()).round() as usize;
    let n = (f.f(1).round().max(1.0) * 2.0 + 1.0) as f64;
    let col = f.color(2);
    if size == 0 {
        f.output(Image::new(1, 1), 0, 0);
        return Ok(());
    }
    let r = (PI / (2.0 * n)).cos().powi(2);
    let d = 2.0 * PI / n;
    let inside = |u: f64, v: f64| {
        let p = [v - 0.5, -u + 0.5];
        let dir = (p[1].atan2(p[0]) / d).round() * -d;
        let x = p[0] * dir.sin() + p[1] * dir.cos();
        let y = p[0] * dir.cos() - p[1] * dir.sin();
        x * x + (y + 0.5).powi(2) <= r
    };
    const SS: usize = 4; // 元のぼかし 1px の代わりに 4×4 の超標本化で縁をなめらかにする
    let s = size as f64;
    let img = Image::generate(size, size, |x, y| {
        let mut c = 0;
        for j in 0..SS {
            for i in 0..SS {
                if inside((x as f64 + (i as f64 + 0.5) / SS as f64) / s, (y as f64 + (j as f64 + 0.5) / SS as f64) / s) {
                    c += 1;
                }
            }
        }
        color::premul(col, c as f32 / (SS * SS) as f32)
    });
    let ox = (f.width as i32 - size as i32) / 2;
    let oy = (f.height as i32 - size as i32) / 2;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "もも猫（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "momo_neeeko.ReuleauxPolygon", name: "ルーローの多角形", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"size", c"サイズ", 100.0, 0.0, 4000.0),
                float(c"f", c"頂点数(2n+1)", 1.0, 1.0, 50.0),
                color_hex(c"col", c"色", 0xffffff),
            ],
            render: render,
        },
    ]
}

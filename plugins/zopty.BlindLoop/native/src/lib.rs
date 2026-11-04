//! ブラインドループ（zopty）の移植: 斜めのクリッピングを等間隔に並べたように切り抜く。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let duty = f.f(0) / 100.0;
    let size = (f.f(1) * f.scale()).max(1e-3);
    let rad = (-f.f(2)).to_radians();
    let phase = f.f(3) / 360.0;
    let (w, h) = (f.width, f.height);
    let (s, c) = rad.sin_cos();
    for y in 0..h {
        for x in 0..w {
            let px = x as f64 + 0.5 - w as f64 / 2.0;
            let py = y as f64 + 0.5 - h as f64 / 2.0;
            // HLSL の % と同じく符号を残す余り
            let a = (px * s + py * c) % (size * 2.0) / (size * 2.0);
            let ph = ((a + phase) * 2.0 * PI).cos().asin() / PI;
            if ph + 0.5 <= duty {
                let i = (y * w + x) * 4;
                f.rgba[i..i + 4].fill(0.0);
            }
        }
    }
    Ok(())
}

plugin! {
    vendor: "zopty（znd 移植: znd-plugins）", version: "1.0.0", license: "BSD-3-Clause",
    effects: [
        {
            id: "zopty.BlindLoop", name: "ブラインドループ", category: "クリッピング", flags: THREAD_SAFE,
            params: [
                float(c"duty", c"割合", 50.0, 0.0, 100.0),
                float(c"width", c"幅", 30.0, 1.0, 1000.0),
                float(c"angle", c"角度", 60.0, -1800.0, 1800.0),
                float(c"phase", c"位相", 0.0, -360.0, 360.0),
            ],
            render: render,
        },
    ]
}

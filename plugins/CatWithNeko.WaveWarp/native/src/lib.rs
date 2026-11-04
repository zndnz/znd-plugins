//! Wave Warp（こんにゃくねこ）の移植: 角度と波形を選べるラスター。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

fn saw(t: f64) -> f64 {
    let v = t / (2.0 * PI);
    (v - v.floor()) * 2.0 - 1.0
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let amp = f.f(0) * sc;
    let length = f.f(1).max(1e-3) * sc;
    let phase0 = (f.f(2) + f.f(3) * f.layer_time()).to_radians();
    let rad = f.f(4).to_radians();
    let expand = f.b(5);
    let kind = f.i(6);
    let (dx, dy) = (rad.cos(), rad.sin());
    let (pv, ph) = if expand { ((amp * dx).abs().ceil() as usize, (amp * dy).abs().ceil() as usize) } else { (0, 0) };
    let src = f.input().padded(ph, pv, ph, pv);
    let (w, h) = (src.w as f64, src.h as f64);
    let perp = [-dy, dx];
    let mut out = Image::new(src.w, src.h);
    for y in 0..src.h {
        for x in 0..src.w {
            let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
            let phase = ((px - w * 0.5) * dx + (py - h * 0.5) * dy) * (2.0 * PI / length) + phase0;
            let wave = match kind {
                1 => saw(phase),
                2 => (1.0 - saw(phase).abs()) * 2.0 - 1.0,
                3 => phase.sin().signum() * if phase.sin() == 0.0 { 0.0 } else { 1.0 },
                _ => phase.sin(),
            };
            out.set(x, y, src.sample((px + perp[0] * amp * wave) as f32, (py + perp[1] * amp * wave) as f32));
        }
    }
    f.output(out, -(ph as i32), -(pv as i32));
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.WaveWarp", name: "Wave Warp", category: "変形", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"amp", c"Amp", 100.0, -4000.0, 4000.0),
                float(c"length", c"Length", 100.0, 1.0, 4000.0),
                float(c"phase", c"Phase", 0.0, -40000.0, 40000.0),
                float(c"speed", c"speed", 0.0, -4000.0, 4000.0),
                float(c"angle", c"Angle", 0.0, -3600.0, 3600.0),
                boolean(c"expand", c"Expand", true),
                choice(c"wavetype", c"Type", 0, c"sin|saw|tri|square"),
            ],
            render: render,
        },
    ]
}

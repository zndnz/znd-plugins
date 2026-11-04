//! プレースホルダー（eizo-gak）の移植: 背景・対角線・枠線のある仮置きの四角形。
//! 平面などに掛けると、レイヤーの絵を指定した大きさのプレースホルダーで置き換える（中心はそのまま）。

use znd_plugin_sdk::draw::{fill, fill_union, paint, segment};
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let w = (f.f(0).round().max(1.0) * s).round().max(1.0) as usize;
    let h = (f.f(1).round().max(1.0) * s).round().max(1.0) as usize;
    let mut img = Image::new(w, h);
    let (wf, hf) = (w as f64, h as f64);
    if f.b(2) && f.f(4) < 100.0 {
        let a = (1.0 - f.f(4) / 100.0) as f32;
        let full = vec![vec![V2::new(0.0, 0.0), V2::new(wf, 0.0), V2::new(wf, hf), V2::new(0.0, hf)]];
        paint(&mut img, &fill(w, h, &full), color::premul(f.color(3), a));
    }
    let lw = f.f(7).max(0.1) * s;
    let line = color::premul(f.color(8), (1.0 - f.f(9).clamp(0.0, 100.0) / 100.0) as f32);
    if f.b(5) && f.f(9) < 100.0 {
        let shapes = vec![vec![segment(V2::new(0.0, 0.0), V2::new(wf, hf), lw)], vec![segment(V2::new(wf, 0.0), V2::new(0.0, hf), lw)]];
        paint(&mut img, &fill_union(w, h, &shapes), line);
    }
    if f.b(6) && f.f(9) < 100.0 {
        let bw = lw.min(wf.min(hf) * 0.5);
        let r = |x0: f64, y0: f64, x1: f64, y1: f64| vec![vec![V2::new(x0, y0), V2::new(x1, y0), V2::new(x1, y1), V2::new(x0, y1)]];
        let shapes = vec![r(0.0, 0.0, wf, bw), r(0.0, hf - bw, wf, hf), r(0.0, bw, bw, hf - bw), r(wf - bw, bw, wf, hf - bw)];
        paint(&mut img, &fill_union(w, h, &shapes), line);
    }
    // 元のレイヤーの中心に置く
    let ox = (f.width as i32 - w as i32) / 2;
    let oy = (f.height as i32 - h as i32) / 2;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "eizo-gak（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "eizo-gak.aviutl2-Placeholder", name: "プレースホルダー", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"width", c"幅", 640.0, 1.0, 7680.0),
                float(c"height", c"高さ", 360.0, 1.0, 4320.0),
                boolean(c"show_background", c"背景を表示", true),
                color_hex(c"background_color", c"背景色", 0x808080),
                float(c"background_transparency", c"背景の透明度", 50.0, 0.0, 100.0),
                boolean(c"show_diagonal", c"対角線を表示", true),
                boolean(c"show_border", c"枠線を表示", true),
                float(c"line_width", c"線の太さ", 1.5, 0.1, 100.0),
                color_hex(c"line_color", c"線の色", 0x666666),
                float(c"line_transparency", c"線の透明度", 0.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}

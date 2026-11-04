//! カメラフレーム（Nanashi.）の移植: 四隅に L 字の線を引いた、カメラのファインダーのような枠を描く。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let (cw, ch) = f.comp_size();
    let (w, h) = if f.i(5) == 0 { (cw - f.f(0) * 2.0, ch - f.f(1) * 2.0) } else { (f.f(0), f.f(1)) };
    let (w, h) = ((w * sc).round().max(1.0) as usize, (h * sc).round().max(1.0) as usize);
    let len = f.f(2) * sc;
    let th = f.f(3).max(1.0) * sc;
    let col = color::premul(f.color(4), 1.0);
    let (wf, hf) = (w as f64, h as f64);
    let img = Image::generate(w, h, |x, y| {
        let (x, y) = (x as f64 + 0.5, y as f64 + 0.5);
        let left = x < len && (y < th || y > hf - th) || x < th && (y < len || y > hf - len);
        let right = x > wf - len && (y < th || y > hf - th) || x > wf - th && (y < len || y > hf - len);
        if left || right { col } else { [0.0; 4] }
    });
    let ox = (f.width as i32 - w as i32) / 2;
    let oy = (f.height as i32 - h as i32) / 2;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-scripts.camera_frame-obj2", name: "カメラフレーム", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"width", c"幅", 100.0, 0.0, 2048.0),
                float(c"height", c"高さ", 100.0, 0.0, 2048.0),
                float(c"line_length", c"線の長さ", 150.0, 0.0, 1000.0),
                float(c"line_thickness", c"線の太さ", 5.0, 1.0, 1000.0),
                color_hex(c"line_color", c"線の色", 0xffffff),
                choice(c"size_specifier", c"サイズ指定", 0, c"余白指定|絶対指定"),
            ],
            render: render,
        },
    ]
}

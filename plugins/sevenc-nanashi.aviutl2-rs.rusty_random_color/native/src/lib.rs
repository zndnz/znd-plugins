//! Rusty Random Color Filter（Nanashi.）の移植: ランダムな色の四角形・楕円・三角形を描くカスタムオブジェクト。
//! 元は最初に描いたときに乱数で色を決めて覚える。znd ではレイヤーのシードから色を決める（同じレイヤーなら毎回同じ色）。

use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let w = (f.f(0).max(1.0) * sc).round() as usize;
    let h = (f.f(1).max(1.0) * sc).round() as usize;
    let shape = f.i(2);
    let seed = f.seed() ^ (f.f(3) as u32).wrapping_mul(0x9e37_79b9);
    let c = [0, 1, 2].map(|k| noise::rand_int(0, 255, seed as i64, k) as f32 / 255.0);
    let col = color::from_display([c[0], c[1], c[2], 1.0]);
    let (wf, hf) = (w as f64, h as f64);
    let cov = match shape {
        1 => draw::fill(w, h, &[draw::ellipse(V2::new(wf / 2.0, hf / 2.0), wf / 2.0, hf / 2.0, 0.0)]),
        2 => draw::fill(w, h, &[vec![V2::new(wf / 2.0, 0.0), V2::new(wf, hf), V2::new(0.0, hf)]]),
        _ => vec![1.0; w * h],
    };
    let mut img = Image::new(w, h);
    draw::paint(&mut img, &cov, [col[0], col[1], col[2], 1.0]);
    let ox = (f.width as i32 - w as i32) / 2;
    let oy = (f.height as i32 - h as i32) / 2;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-rs.rusty_random_color", name: "Rusty Random Color", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"width", c"Width", 640.0, 1.0, 4096.0),
                float(c"height", c"Height", 640.0, 1.0, 4096.0),
                choice(c"shape", c"Shape", 0, c"Rectangle|Ellipse|Triangle"),
                float(c"variation", c"色の番号", 0.0, 0.0, 10000.0),
            ],
            render: render,
        },
    ]
}

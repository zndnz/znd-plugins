//! QR コード（sagami121）の移植: 文字列から QR コードを描く（四角・円・角丸のドット、余白、色、背景透明）。
//! 符号化は qrcodegen（MIT, Project Nayuki）で行う（元はスクリプトの中で Reed-Solomon から実装している）。

use qrcodegen::{QrCode, QrCodeEcc};
use znd_plugin_sdk::draw::{circle, fill, regular_polygon, round_corners};
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let cell = (f.f(0).floor().max(1.0) * s).max(1.0);
    let margin = f.f(1).floor().max(0.0) as i32;
    let ecc = [QrCodeEcc::Low, QrCodeEcc::Medium, QrCodeEcc::Quartile, QrCodeEcc::High][f.i(2).min(3)];
    let shape = f.i(3);
    let text = if f.text(4).is_empty() { "https://example.com" } else { f.text(4) };
    let fg = color::premul(f.color(5), 1.0);
    let bg = color::premul(f.color(6), if f.b(7) { 0.0 } else { 1.0 });
    let radius = f.f(8).clamp(0.0, 50.0) / 100.0;
    let qr = QrCode::encode_text(text, ecc).map_err(|_| "文字列が長すぎます")?;
    let n = qr.size() + margin * 2;
    let side = (n as f64 * cell).ceil() as usize;
    let mut img = Image::filled(side, side, bg);
    // 1 マス分の図形（被覆率）を作っておき、黒いマスに置く
    let c = cell.ceil() as usize + 1;
    let shape_cov = match shape {
        1 => fill(c, c, &[circle(V2::new(cell / 2.0, cell / 2.0), cell / 2.0)]),
        2 => {
            let sq = regular_polygon(V2::new(cell / 2.0, cell / 2.0), cell / std::f64::consts::SQRT_2, 4, 45.0);
            fill(c, c, &[round_corners(&sq, cell * radius)])
        }
        _ => fill(c, c, &[vec![V2::new(0.0, 0.0), V2::new(cell, 0.0), V2::new(cell, cell), V2::new(0.0, cell)]]),
    };
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if !qr.get_module(x, y) {
                continue;
            }
            let (ox, oy) = (((x + margin) as f64 * cell).floor() as isize, ((y + margin) as f64 * cell).floor() as isize);
            for dy in 0..c {
                for dx in 0..c {
                    let k = shape_cov[dy * c + dx];
                    if k > 0.0 {
                        img.blend(ox + dx as isize, oy + dy as isize, fg.map(|v| v * k));
                    }
                }
            }
        }
    }
    let ox = (f.width as i32 - side as i32) / 2;
    let oy = (f.height as i32 - side as i32) / 2;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "sagami121（znd 移植: znd-plugins）", version: "1.0.2", license: "MIT",
    effects: [
        {
            id: "sagami121.QRCode", name: "QR コード", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"size", c"サイズ（1 マスの px）", 10.0, 1.0, 100.0),
                float(c"whitespace", c"余白（マス）", 4.0, 0.0, 20.0),
                choice(c"ecc", c"誤り訂正", 0, c"L|M|Q|H"),
                choice(c"shape", c"形状", 0, c"四角|円|角丸"),
                text(c"text", c"テキスト"),
                color_hex(c"fg", c"前景色", 0x000000),
                color_hex(c"bg", c"背景色", 0xffffff),
                boolean(c"bg_trans", c"背景透明", false),
                float(c"corner_r", c"角丸強度", 30.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}

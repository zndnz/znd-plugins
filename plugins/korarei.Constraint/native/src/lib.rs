//! Constraint（Korarei）の移植: 絵（テキストなど）の上下左右に、別のレイヤーの絵をアイコンとして並べる。

use znd_plugin_sdk::draw::textured_quad;
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let position = f.i(1);
    let offset = f.f(2) / 100.0;
    let padding = f.f(3) * f.scale();
    let rz = f.f(4);
    let scale = f.f(5) / 100.0;
    let Some(icon) = f.layer_image(0) else { return Ok(()) };
    let base = f.input();
    let (bw, bh) = (base.w as f64, base.h as f64);
    let t = rz.to_radians();
    let (c, s) = (t.cos().abs(), t.sin().abs());
    let (iw0, ih0) = (icon.w as f64 * scale, icon.h as f64 * scale);
    let (iw, ih) = (iw0 * c + ih0 * s, iw0 * s + ih0 * c);
    if iw * ih < 1e-4 {
        return Ok(());
    }
    let vertical = position % 2 == 0;
    let side = if (position + 1) & 2 != 0 { 1.0 } else { -1.0 };
    let (w, h) = if vertical { (bw.max(iw), bh + ih + padding) } else { (bw + iw + padding, bh.max(ih)) };
    let (ow, oh) = (w.ceil() as usize, h.ceil() as usize);
    let mut out = Image::new(ow, oh);
    // アイコン（中心からのずれ）
    let (dx, dy) = if vertical { ((w - iw) * 0.5 * offset, (h - ih) * 0.5 * side) } else { ((w - iw) * 0.5 * side, (h - ih) * 0.5 * offset) };
    let ctr = V2::new(w / 2.0 + dx, h / 2.0 + dy);
    let corner = |x: f64, y: f64| {
        let p = V2::new(x * scale, y * scale).rot(rz);
        ctr.add(p)
    };
    let (hw, hh) = (icon.w as f64 / 2.0, icon.h as f64 / 2.0);
    textured_quad(
        &mut out,
        &icon,
        [corner(-hw, -hh), corner(hw, -hh), corner(hw, hh), corner(-hw, hh)],
        [V2::new(0.0, 0.0), V2::new(icon.w as f64, 0.0), V2::new(icon.w as f64, icon.h as f64), V2::new(0.0, icon.h as f64)],
        1.0,
    );
    // 元の絵
    let (bx, by) = if vertical { ((bw - w) * 0.5 * offset, (bh - h) * 0.5 * side) } else { ((bw - w) * 0.5 * side, (bh - h) * 0.5 * offset) };
    let px = (w / 2.0 + bx - bw / 2.0).round() as isize;
    let py = (h / 2.0 + by - bh / 2.0).round() as isize;
    out.draw(&base, px, py, 1.0);
    // 元の絵の位置が変わらないように置く
    f.output(out, -px as i32, -py as i32);
    Ok(())
}

plugin! {
    vendor: "Korarei（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "korarei.Constraint", name: "Constraint", category: "配置", flags: THREAD_SAFE,
            params: [
                layer(c"image_layer", c"Image Layer"),
                choice(c"position", c"Position", 3, c"Top|Right|Bottom|Left"),
                float(c"offset", c"Cross Offset", 0.0, -100.0, 100.0),
                float(c"padding", c"Padding", 0.0, 0.0, 4000.0),
                float(c"rz", c"Rotation", 0.0, -720.0, 720.0),
                float(c"scale", c"Scale", 100.0, 0.0, 1000.0),
            ],
            render: render,
        },
    ]
}

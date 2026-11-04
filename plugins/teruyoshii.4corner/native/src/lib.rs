//! 四隅に配置（teruyoshi）の移植: 絵を画面（コンポ）の四隅に 4 つ並べる。余白・大きさ・回転・反転・透明度を隅ごとに決められる。

use znd_plugin_sdk::draw::textured_quad;
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let (cw, ch) = f.comp_size();
    let (hw, hh) = (cw * sc / 2.0, ch * sc / 2.0);
    let (mut mx, mut my) = (f.f(0) * sc, f.f(1) * sc);
    let from_center = f.i(2) == 1;
    if f.b(3) {
        mx = f.f(0) / 100.0 * hw;
        my = f.f(1) / 100.0 * hh;
    }
    let size = f.f(4) / 100.0;
    if size <= 0.0 {
        f.output(Image::new(1, 1), 0, 0);
        return Ok(());
    }
    let sizes = [f.f(5), f.f(6), f.f(7), f.f(8)].map(|v| v / 100.0);
    let rot = f.f(9);
    let rots = [f.f(10), f.f(11), f.f(12), f.f(13)];
    let (flip_x, flip_y) = (f.b(14), f.b(15));
    let alphas = [f.f(16), f.f(17), f.f(18), f.f(19)].map(|v| ((100.0 - v) / 100.0) as f32);
    // 隅の中心の、画面の中心からの位置（px）
    let (tx, ty) = if from_center { (hw - mx, hh - my) } else { (mx, my) };
    let centers = [
        V2::new(-hw + tx, -hh + ty),
        V2::new(hw - tx, -hh + ty),
        V2::new(-hw + tx, hh - ty),
        V2::new(hw - tx, hh - ty),
    ];
    let mirror = [(1.0, 1.0), (if flip_x { -1.0 } else { 1.0 }, 1.0), (1.0, if flip_y { -1.0 } else { 1.0 }), (if flip_x { -1.0 } else { 1.0 }, if flip_y { -1.0 } else { 1.0 })];
    let src = f.input();
    let (w, h) = (src.w as f64, src.h as f64);
    // 出力は画面の大きさ。中心は元の絵の中心に合わせる
    let (ow, oh) = ((hw * 2.0).ceil().max(1.0) as usize, (hh * 2.0).ceil().max(1.0) as usize);
    let mut out = Image::new(ow, oh);
    let o = V2::new(ow as f64 / 2.0, oh as f64 / 2.0);
    for k in 0..4 {
        let s = size * sizes[k];
        if s <= 0.0 || alphas[k] <= 0.0 {
            continue;
        }
        let (mxk, myk) = mirror[k];
        let corner = |x: f64, y: f64| {
            let p = V2::new(x * s, y * s).rot(rot + rots[k]);
            o.add(centers[k]).add(V2::new(p.x * mxk, p.y * myk))
        };
        let (a, b) = (w / 2.0, h / 2.0);
        textured_quad(
            &mut out,
            &src,
            [corner(-a, -b), corner(a, -b), corner(a, b), corner(-a, b)],
            [V2::new(0.0, 0.0), V2::new(w, 0.0), V2::new(w, h), V2::new(0.0, h)],
            alphas[k],
        );
    }
    let ox = (src.w as f64 / 2.0 - o.x).round() as i32;
    let oy = (src.h as f64 / 2.0 - o.y).round() as i32;
    f.output(out, ox, oy);
    Ok(())
}

plugin! {
    vendor: "てるよし（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "teruyoshii.4corner", name: "四隅に配置", category: "配置", flags: THREAD_SAFE,
            params: [
                float(c"mx", c"余白X", 100.0, -2000.0, 2000.0),
                float(c"my", c"余白Y", 100.0, -2000.0, 2000.0),
                choice(c"method", c"配置方法", 0, c"画面端からの余白を指定|中央からの余白を指定"),
                boolean(c"is_percent", c"余白をパーセントで指定する", false),
                float(c"size", c"全体サイズ", 100.0, 0.0, 1000.0),
                float(c"size_ul", c"左上サイズ", 100.0, 0.0, 1000.0),
                float(c"size_ur", c"右上サイズ", 100.0, 0.0, 1000.0),
                float(c"size_dl", c"左下サイズ", 100.0, 0.0, 1000.0),
                float(c"size_dr", c"右下サイズ", 100.0, 0.0, 1000.0),
                float(c"rot", c"全体回転", 0.0, -3600.0, 3600.0),
                float(c"rot_ul", c"左上回転", 0.0, -3600.0, 3600.0),
                float(c"rot_ur", c"右上回転", 0.0, -3600.0, 3600.0),
                float(c"rot_dl", c"左下回転", 0.0, -3600.0, 3600.0),
                float(c"rot_dr", c"右下回転", 0.0, -3600.0, 3600.0),
                boolean(c"flip_x", c"左右反転", true),
                boolean(c"flip_y", c"上下反転", true),
                float(c"alpha_ul", c"左上透明度", 0.0, 0.0, 100.0),
                float(c"alpha_ur", c"右上透明度", 0.0, 0.0, 100.0),
                float(c"alpha_dl", c"左下透明度", 0.0, 0.0, 100.0),
                float(c"alpha_dr", c"右下透明度", 0.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}

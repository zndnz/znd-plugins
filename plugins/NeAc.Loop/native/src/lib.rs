//! Loop（猫愛）の移植: オブジェクトを縦横に並べて複製する（ずらし・中央揃え付き）。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let (mut nx, mut ny) = (f.f(0).round(), f.f(1).round());
    if nx == 0.0 || ny == 0.0 {
        f.output(Image::new(1, 1), 0, 0);
        return Ok(());
    }
    let (rev_x, rev_y) = (nx < 0.0, ny < 0.0);
    if nx < 0.0 && f.b(2) {
        nx -= 2.0;
    }
    if ny < 0.0 && f.b(3) {
        ny -= 2.0;
    }
    let (nx, ny) = (nx.abs() as i64, ny.abs() as i64);
    if nx * ny > 100_000 {
        return Err("個数が多すぎます");
    }
    let src = f.input();
    let (w, h) = (src.w as f64, src.h as f64);
    // 間隔: 既定はオブジェクトの大きさに対する %、「個別基準」で px
    let (gx, gy) = if f.b(9) { (f.f(4) * s, f.f(5) * s) } else { (f.f(4) / 100.0 * w, f.f(5) / 100.0 * h) };
    let (ax, ay) = (f.i(6), f.i(7));
    let (offx, offy) = (f.f(8) / 100.0, f.f(10) / 100.0);
    let pos = |xi: i64, yi: i64| -> (f64, f64) {
        let mut x = if ax >= 1 { gx * (xi as f64 - (nx - 1) as f64 / 2.0) } else { xi as f64 * gx };
        if rev_x {
            x = -x;
        }
        let mut y = if ay >= 1 { gy * (yi as f64 - (ny - 1) as f64 / 2.0) } else { yi as f64 * gy };
        if rev_y {
            y = -y;
        }
        let cx = if ax == 2 { gx * offx * (ny - 1) as f64 / 2.0 } else { 0.0 };
        let cy = if ay == 2 { gy * offy * (nx - 1) as f64 / 2.0 } else { 0.0 };
        (x + gx * offx * yi as f64 - cx, y + gy * offy * xi as f64 - cy)
    };
    let mut pts = Vec::new();
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for yi in 0..ny {
        for xi in 0..nx {
            let (x, y) = pos(xi, yi);
            pts.push((x, y));
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
    }
    let (ox, oy) = (x0.floor(), y0.floor());
    let ow = ((x1 - ox).ceil() + w) as usize;
    let oh = ((y1 - oy).ceil() + h) as usize;
    if ow * oh > 16384 * 16384 {
        return Err("大きすぎます");
    }
    let mut out = Image::new(ow, oh);
    // 先に描いたものが下になる（AviUtl の obj.draw の順と同じ）
    for (x, y) in pts {
        out.draw(&src, (x - ox).round() as isize, (y - oy).round() as isize, 1.0);
    }
    f.output(out, ox as i32, oy as i32);
    Ok(())
}

plugin! {
    vendor: "猫愛（znd 移植: znd-plugins）", version: "r3", license: "MIT",
    effects: [
        {
            id: "NeAc.Loop", name: "Loop（並べて複製）", category: "配置", flags: THREAD_SAFE,
            params: [
                float(c"amount_x", c"X個数", 3.0, -1000.0, 1000.0),
                float(c"amount_y", c"Y個数", 3.0, -1000.0, 1000.0),
                boolean(c"min0minus2_x", c"X個数0以下時-2", false),
                boolean(c"min0minus2_y", c"Y個数0以下時-2", false),
                float(c"space_x", c"X間隔", 100.0, -10000.0, 10000.0),
                float(c"space_y", c"Y間隔", 100.0, -10000.0, 10000.0),
                choice(c"align_x", c"X整列", 2, c"無効|1列目で中央揃え|全体で中央揃え"),
                choice(c"align_y", c"Y整列", 2, c"無効|1列目で中央揃え|全体で中央揃え"),
                float(c"offset_x", c"Xオフセット", 0.0, -10000.0, 10000.0),
                boolean(c"ind_mode", c"個別基準（間隔を px で指定）", false),
                float(c"offset_y", c"Yオフセット", 0.0, -10000.0, 10000.0),
            ],
            render: render,
        },
    ]
}

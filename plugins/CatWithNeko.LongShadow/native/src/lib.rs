//! LongShadow（こんにゃくねこ）の移植: 形を一方向に長く伸ばした影（ロングシャドー・フラットシャドー）。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let l = (f.f(0) * f.scale()).max(0.0);
    let rad = f.f(1).to_radians();
    let alpha = f.f(2) as f32;
    let c1 = f.color(3);
    let c2 = if f.b(4) { f.color(5) } else { c1 };
    let trp = f.b(6);
    let shadow_only = f.b(7);
    let flat = f.b(8);
    let (dx, dy) = (rad.cos(), rad.sin());
    let left = if dx < 0.0 { -dx * l } else { 0.0 }.ceil() as usize;
    let right = if dx > 0.0 { dx * l } else { 0.0 }.ceil() as usize;
    let up = if dy < 0.0 { -dy * l } else { 0.0 }.ceil() as usize;
    let down = if dy > 0.0 { dy * l } else { 0.0 }.ceil() as usize;
    let org = f.input();
    let src = org.padded(left, up, right, down);
    let (w, h) = (src.w, src.h);
    let n = l.floor() as usize;
    let mut out = src.clone();
    let lerp = |t: f32| [0, 1, 2].map(|k| c1[k] + (c2[k] - c1[k]) * t);
    for y in 0..h {
        for x in 0..w {
            for i in 1..=n {
                let sx = (x as f64 + 0.5 - dx * i as f64).floor() as isize;
                let sy = (y as f64 + 0.5 - dy * i as f64).floor() as isize;
                if src.get(sx, sy)[3] > 0.01 {
                    let t = (i as f64 / l) as f32;
                    let a = if trp { (1.0 - t) * alpha } else { alpha };
                    let c = lerp(t);
                    out.set(x, y, [c[0] * a, c[1] * a, c[2] * a, a]);
                    break;
                }
            }
        }
    }
    if flat && l > 0.0 {
        // 影の向きに沿った 1 本のグラデーションで塗り直す（元の「グラデーション」フィルタの代わり）
        let (cx, cy) = (w as f64 / 2.0, h as f64 / 2.0);
        for y in 0..h {
            for x in 0..w {
                let p = out.px(x, y);
                if p[3] <= 0.0 {
                    continue;
                }
                let t = ((((x as f64 + 0.5 - cx) * dx + (y as f64 + 0.5 - cy) * dy) / l) + 0.5).clamp(0.0, 1.0) as f32;
                let c = lerp(t);
                out.set(x, y, [c[0] * p[3], c[1] * p[3], c[2] * p[3], p[3]]);
            }
        }
    }
    if !shadow_only {
        out.draw(&org, left as isize, up as isize, 1.0);
    }
    f.output(out, -(left as i32), -(up as i32));
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.LongShadow", name: "LongShadow", category: "装飾", flags: THREAD_SAFE,
            params: [
                float(c"l", c"Length", 200.0, 0.0, 4000.0),
                float(c"a", c"Angle", 0.0, -1280.0, 1280.0),
                float(c"alpha", c"Alpha", 1.0, 0.0, 1.0),
                color_hex(c"c1", c"Start", 0xffffff),
                boolean(c"grad", c"Grad", true),
                color_hex(c"c2", c"End", 0x000000),
                boolean(c"trp", c"Transparent", false),
                boolean(c"shadow_only", c"ShadowOnly", false),
                boolean(c"flat", c"Flat", false),
            ],
            render: render,
        },
    ]
}

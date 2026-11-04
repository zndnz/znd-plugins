//! MaxMin（こんにゃくねこ）の移植: 回転させた正方形の範囲で、色ごとの最大値・最小値を取る。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let use_max = f.i(0) == 0;
    let scale = (f.f(1) * f.scale()).round().max(0.0) as i64;
    let rad = f.f(2).to_radians();
    let (s, c) = rad.sin_cos();
    let (w, h) = (f.width, f.height);
    let src = f.input();
    let offs: Vec<(isize, isize)> = (-scale..=scale)
        .flat_map(|i| (-scale..=scale).map(move |j| (i as f64, j as f64)))
        .map(|(i, j)| ((i * s - j * c) as isize, (i * c + j * s) as isize))
        .collect();
    for y in 0..h {
        for x in 0..w {
            let col = src.px(x, y);
            let mut m = [col[0], col[1], col[2]];
            for &(dx, dy) in &offs {
                let sx = (x as isize + dx).clamp(0, w as isize - 1) as usize;
                let sy = (y as isize + dy).clamp(0, h as isize - 1) as usize;
                let p = src.px(sx, sy);
                for k in 0..3 {
                    m[k] = if use_max { m[k].max(p[k]) } else { m[k].min(p[k]) };
                }
            }
            let i = (y * w + x) * 4;
            // 元と同じくアルファはそのまま（乗算済みとして色がアルファを超えないようにする）
            let a = col[3];
            f.rgba[i..i + 4].copy_from_slice(&[m[0].min(a), m[1].min(a), m[2].min(a), a]);
        }
    }
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.MaxMin", name: "MaxMin", category: "加工", flags: THREAD_SAFE,
            params: [
                choice(c"maxmin", c"Max/Min", 0, c"Max|Min"),
                float(c"scale", c"Scale", 1.0, 0.0, 100.0),
                float(c"angle", c"Angle", 0.0, -1000.0, 1000.0),
            ],
            render: render,
        },
    ]
}

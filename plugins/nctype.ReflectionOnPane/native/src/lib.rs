//! ReflectionOnPane（nctype）の移植: ガラス面への映り込み。
//! 露光 → 色調補正 → ぼかし → 端のフェード → 強度 → ミラー → 二重像（ずらして半分の濃さで重ねる）。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let mut img = f.input();
    let exposure = (f.f(0).clamp(0.0, 400.0) / 100.0) as f32;
    aul::map_display(&mut img, |c| [(c[0] * exposure).min(1.0), (c[1] * exposure).min(1.0), (c[2] * exposure).min(1.0), c[3]]);
    // 元は「色調補正」(明るさ 0, 輝度 200)。輝度 200 は明るさ 2 倍
    aul::color_correct(&mut img, 0.0, f.f(1) as f32, f.f(4) as f32, 200.0, f.f(3) as f32, f.f(2) as f32);
    let blur = f.f(6) as f32 * f.scale() as f32;
    if blur > 0.0 {
        aul::blur(&mut img, blur);
    }
    let dir = f.i(9);
    let range = (f.f(10).min(100.0) / 100.0).max(0.001) as f32;
    if dir != 0 && f.f(10) > 0.0 {
        let soft = (f.f(11).clamp(0.0, 100.0) / 100.0) as f32;
        let curve = 0.12 + (2.5 - 0.12) * (1.0 - soft);
        let (w, h) = (img.w, img.h);
        img.map(|x, y, p| {
            let (u, v) = ((x as f32 + 0.5) / w as f32, (y as f32 + 0.5) / h as f32);
            let e = match dir {
                1 => u,
                2 => 1.0 - u,
                3 => v,
                _ => 1.0 - v,
            };
            let m = (e / range).clamp(0.0, 1.0).powf(curve);
            [p[0] * m, p[1] * m, p[2] * m, p[3] * m]
        });
    }
    aul::fade(&mut img, (f.f(5).clamp(0.0, 100.0) / 100.0) as f32);
    if f.b(12) {
        aul::flip_h(&mut img);
    }
    let (ox, oy) = (f.f(7) * f.scale(), f.f(8) * f.scale());
    if ox.abs() > 1e-4 || oy.abs() > 1e-4 {
        let mut ghost = img.clone();
        aul::fade(&mut ghost, 0.5);
        let base = img.clone();
        img = base;
        img.draw(&ghost, ox.round() as isize, oy.round() as isize, 1.0);
    }
    f.set(img);
    Ok(())
}

plugin! {
    vendor: "nctype（znd 移植: znd-plugins）", version: "1.0.1", license: "MIT",
    effects: [
        {
            id: "nctype.ReflectionOnPane", name: "ガラスへの映り込み（ReflectionOnPane）", category: "スタイライズ", flags: THREAD_SAFE,
            params: [
                float(c"exposure", c"露光", 100.0, 0.0, 400.0),
                float(c"contrast", c"コントラスト", 150.0, 0.0, 200.0),
                float(c"gamma", c"ガンマ", -100.0, -100.0, 100.0),
                float(c"saturation", c"彩度", 100.0, 0.0, 200.0),
                float(c"hue", c"色相", 0.0, -180.0, 180.0),
                float(c"strength", c"反射強度", 65.0, 0.0, 100.0),
                float(c"blur", c"ぼかし", 3.0, 0.0, 20.0),
                float(c"offset_x", c"Xずれ（重影）", 0.0, -100.0, 100.0),
                float(c"offset_y", c"Yずれ（重影）", 0.0, -100.0, 100.0),
                choice(c"fade_direction", c"エッジ方向", 0, c"なし|左→右|右→左|上→下|下→上"),
                float(c"fade_range", c"エッジ範囲", 0.0, 0.0, 100.0),
                float(c"fade_softness", c"エッジ柔らかさ", 60.0, 0.0, 100.0),
                boolean(c"mirror", c"ミラー", true),
            ],
            render: render,
        },
    ]
}

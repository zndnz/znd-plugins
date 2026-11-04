//! GParticle（sagami121）の移植: 爆発・噴水・コーン・渦巻きのパーティクル。元はコンピュートシェーダー、ここでは同じ計算を CPU で行う。
//! コンポの大きさの平面に掛けると、平面の絵をパーティクルで置き換える（発生源はレイヤーの中心からの位置）。

use std::f32::consts::TAU;
use znd_plugin_sdk::*;

fn pcg(s: u32) -> u32 {
    let state = s.wrapping_mul(747796405).wrapping_add(2891336453);
    let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277803737);
    (word >> 22) ^ word
}
fn rand01(s: u32) -> f32 {
    pcg(s) as f32 / 4294967296.0
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale() as f32;
    let p = |i: usize| f.f(i) as f32;
    let count = p(0).round().clamp(1.0, 5000.0) as u32;
    let life = p(1).max(1e-4);
    let birth_r = p(2) / 100.0;
    let (ex, ey) = (p(3), p(4));
    let (rx, ry) = (p(5), p(6));
    let anim = f.i(7);
    let (vel, vel_var, spread) = (p(8), p(9) / 100.0, p(10).to_radians());
    let (gvy, gvx) = (p(11), p(12));
    let res = p(13) / 10.0;
    let (bs, ds, size_var, scale, intensity) = (p(14), p(15), p(16) / 100.0, p(17), p(18));
    let (c1, c2) = (f.color(19), f.color(20));
    let (a_s, a_e) = (p(21) / 100.0, p(22) / 100.0);
    let time = f.layer_time() as f32;
    let (sw, sh) = (f.width, f.height);
    let mut img = Image::new(sw, sh);
    for i in 0..count {
        let total = time - life * (i as f32 / count as f32);
        if total < 0.0 {
            continue;
        }
        let cycle = (total / life) as u32;
        let t = total % life;
        let seed = i.wrapping_mul(7919).wrapping_add(cycle.wrapping_mul(104729));
        if rand01(seed + 99) >= birth_r {
            continue;
        }
        let ratio = (t / life).clamp(0.0, 1.0);
        let ox = (rand01(seed + 10) * 2.0 - 1.0) * rx;
        let oy = (rand01(seed + 11) * 2.0 - 1.0) * ry;
        let speed = vel * (1.0 - vel_var * rand01(seed + 3));
        let v = match anim {
            0 => {
                let th = (1.0 - 2.0 * rand01(seed + 1)).acos();
                let ph = rand01(seed + 2) * TAU;
                (th.sin() * ph.cos() * speed, th.sin() * ph.sin() * speed)
            }
            2 => {
                let th = rand01(seed + 1) * spread;
                let ph = rand01(seed + 2) * TAU;
                (ph.sin() * th.sin() * speed, th.cos() * speed)
            }
            3 => {
                let a = rand01(seed + 2) * TAU + t * 5.0;
                let d = (1.0 + t * 0.5) * speed;
                (a.cos() * d, -speed * 0.5)
            }
            _ => {
                let th = rand01(seed + 1) * spread;
                let ph = rand01(seed + 2) * TAU;
                (ph.sin() * th.sin() * speed, -th.cos() * speed)
            }
        };
        let drag = (-res.max(0.0) * t).exp();
        let it = if res > 0.001 { (1.0 - drag) / res } else { t };
        let px = ex + ox + v.0 * it + 0.5 * gvx * t * t;
        let py = ey + oy + v.1 * it + 0.5 * gvy * t * t;
        let size = ((bs + (ds - bs) * ratio) * (1.0 - size_var * rand01(seed + 4)) * scale).max(0.1) * sc;
        let col = [c1[0] + (c2[0] - c1[0]) * ratio, c1[1] + (c2[1] - c1[1]) * ratio, c1[2] + (c2[2] - c1[2]) * ratio];
        let alpha = a_s + (a_e - a_s) * ratio;
        // スプライト（中心から外へ 2 乗で薄くなる円）を加算で重ねる
        let (cx, cy) = (px * sc + sw as f32 / 2.0, py * sc + sh as f32 / 2.0);
        let r = size.max(1.0);
        let (x0, x1) = (((cx - r).floor() as isize).max(0), ((cx + r).ceil() as isize).min(sw as isize - 1));
        let (y0, y1) = (((cy - r).floor() as isize).max(0), ((cy + r).ceil() as isize).min(sh as isize - 1));
        for y in y0..=y1 {
            for x in x0..=x1 {
                let d = ((x as f32 + 0.5 - cx).powi(2) + (y as f32 + 0.5 - cy).powi(2)).sqrt();
                if d > size {
                    continue;
                }
                let fall = (1.0 - d / size).clamp(0.0, 1.0).powi(2);
                let a = alpha * fall;
                let i = img.idx(x as usize, y as usize);
                for k in 0..3 {
                    img.data[i + k] = (img.data[i + k] + col[k] * a * intensity).min(1.0);
                }
                img.data[i + 3] = (img.data[i + 3] + a).min(1.0);
            }
        }
    }
    f.set(img);
    Ok(())
}

plugin! {
    vendor: "sagami121（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "sagami121.GParticle", name: "GParticle", category: "シミュレーション", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"count", c"発生量", 300.0, 1.0, 5000.0),
                float(c"life", c"寿命（秒）", 3.0, 0.1, 20.0),
                float(c"birth_r", c"発生割合（%）", 100.0, 1.0, 100.0),
                float(c"emit_x", c"位置X", 0.0, -2000.0, 2000.0),
                float(c"emit_y", c"位置Y", 0.0, -2000.0, 2000.0),
                float(c"rad_x", c"範囲X", 0.0, 0.0, 1000.0),
                float(c"rad_y", c"範囲Y", 0.0, 0.0, 1000.0),
                choice(c"anim_type", c"アニメーション方式", 0, c"全方向爆発|方向指定|コーン状|渦巻き/回転"),
                float(c"vel", c"初速", 150.0, 0.0, 1000.0),
                float(c"vel_var", c"速度分散（%）", 20.0, 0.0, 100.0),
                float(c"spread", c"拡散角度", 60.0, 0.0, 180.0),
                float(c"gvy", c"重力Y", 150.0, -1000.0, 1000.0),
                float(c"gvx", c"重力X", 0.0, -500.0, 500.0),
                float(c"resistance", c"空気抵抗", 0.0, 0.0, 100.0),
                float(c"birth_size", c"生成時サイズ", 20.0, 1.0, 200.0),
                float(c"death_size", c"消滅時サイズ", 2.0, 0.0, 200.0),
                float(c"size_var", c"サイズ分散（%）", 50.0, 0.0, 100.0),
                float(c"scale", c"拡大率", 1.0, 0.1, 10.0),
                float(c"intensity", c"光の強さ", 1.0, 0.0, 5.0),
                color_hex(c"col_birth", c"生成時カラー", 0xffffff),
                color_hex(c"col_death", c"消滅時カラー", 0xff8800),
                float(c"alpha_s", c"生成時不透明度", 100.0, 0.0, 100.0),
                float(c"alpha_e", c"消滅時不透明度", 0.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}

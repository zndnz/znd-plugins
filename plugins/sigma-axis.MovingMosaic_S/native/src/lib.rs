//! MovingMosaic_S（σ軸）の移植: マスの中のランダムな位置から色を拾うモザイク（拾う位置がシードで変わり、動いて見える）。
//! 乱数は元と同じ IbukiHash（Andante, CC0）。

use znd_plugin_sdk::*;

/// IbukiHash by Andante (CC0 1.0)
fn ibuki(v: [u32; 4]) -> f64 {
    const M: [u32; 4] = [0xae3cc725, 0x9fe72885, 0xae36bfb5, 0x82c1fcad];
    let mut u = [v[0].wrapping_mul(M[0]), v[1].wrapping_mul(M[1]), v[2].wrapping_mul(M[2]), v[3].wrapping_mul(M[3])];
    let w = [u[3], u[0], u[1], u[2]];
    for k in 0..4 {
        u[k] ^= w[k] ^ (u[k] >> 13);
    }
    let mut r = u.iter().zip(M).fold(0u32, |a, (x, m)| a.wrapping_add(x.wrapping_mul(m)));
    r ^= r >> 11;
    r = r.wrapping_mul(r) ^ r;
    r as f64 * 2.3283064365386962890625e-10
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let size = f.f(0) * s;
    let aspect = (f.f(3) / 100.0).clamp(-1.0, 1.0);
    let (mut sx, mut sy) = (size, size);
    if aspect > 0.0 {
        sx *= 1.0 - aspect;
    }
    if aspect < 0.0 {
        sy *= 1.0 + aspect;
    }
    let (sx, sy) = (sx.max(1.0), sy.max(1.0));
    if sx <= 1.0 && sy <= 1.0 {
        return Ok(());
    }
    let mut seed = f.f(1).floor() as i64;
    if seed >= 0 {
        // 正のシードはレイヤーごとに違う乱数にする（元はオブジェクト ID などを混ぜる）
        seed += 2525 * (f.seed() as i64 % (1 << 20));
    }
    let seed = seed.rem_euclid(1 << 20) as u32;
    let convex = f.b(2);
    let o = f.point(4);
    let (cx, cy) = (o[0], o[1]);
    let src = f.input();
    let (w, h) = (src.w as f64, src.h as f64);
    let (l, r, t, b) = ((-cx / sx).floor() as i64, ((w - cx) / sx).ceil() as i64, (-cy / sy).floor() as i64, ((h - cy) / sy).ceil() as i64);
    let (tw, th) = ((r - l).max(1) as usize, (b - t).max(1) as usize);
    // マスごとの色
    let mut tiles = Image::new(tw, th);
    for ty in 0..th {
        for tx in 0..tw {
            let (gx, gy) = (tx as i64 + l, ty as i64 + t);
            let (x0, y0) = (gx as f64 * sx + cx, gy as f64 * sy + cy);
            let hx = ibuki([(65536 + gx) as u32, (65536 + gy) as u32, seed, 7]);
            let hy = ibuki([(65536 + gx) as u32, (65536 + gy) as u32, seed, 11]);
            let px = ((x0 + sx * hx).floor() as isize).clamp(0, src.w as isize - 1);
            let py = ((y0 + sy * hy).floor() as isize).clamp(0, src.h as isize - 1);
            tiles.set(tx, ty, src.get(px, py));
        }
    }
    let mut out = Image::new(src.w, src.h);
    out.map(|x, y, _| {
        let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
        let (gx, gy) = (((px - cx) / sx).floor(), ((py - cy) / sy).floor());
        let (tlx, tly) = (gx * sx + cx, gy * sy + cy);
        let (brx, bry) = (tlx + sx, tly + sy);
        if convex {
            // タイル風: マスの上・左の 1px を明るく、下・右の 1px を暗く
            let (ix, iy) = (px.floor(), py.floor());
            let mut rate = 0.0;
            if ix == (tlx - 0.5).ceil() || iy == (tly - 0.5).ceil() {
                rate += 1.0;
            }
            if ix == (brx - 0.5).ceil() - 1.0 || iy == (bry - 0.5).ceil() - 1.0 {
                rate -= 1.0;
            }
            let mut c = tiles.get_clamped(gx as isize - l as isize, gy as isize - t as isize);
            for k in 0..3 {
                c[k] = (c[k] * (1.0 + rate * 0.25)).clamp(0.0, c[3]);
            }
            c
        } else {
            // マスの境目の 1px だけ隣のマスと混ぜる
            let u = gx - l as f64 + 0.5 + (px - tlx).min(0.5) + (px - brx).max(-0.5);
            let v = gy - t as f64 + 0.5 + (py - tly).min(0.5) + (py - bry).max(-0.5);
            tiles.sample_clamped(u as f32, v as f32)
        }
    });
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "σ軸（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "sigma-axis.MovingMosaic_S", name: "動くモザイク（MovingMosaic_S）", category: "スタイライズ", flags: THREAD_SAFE,
            params: [
                float(c"size", c"サイズ", 12.0, 1.0, 2000.0),
                float(c"seed", c"シード", 10000.0, -65536.0, 65535.0),
                boolean(c"convex", c"タイル風", false),
                float(c"aspect", c"縦横比", 0.0, -100.0, 100.0),
                point(c"origin", c"マスの基準位置", 0.5, 0.5),
            ],
            render: render,
        },
    ]
}

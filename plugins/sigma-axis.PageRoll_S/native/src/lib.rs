//! PageRoll_S（σ軸）の移植: 紙を端から丸めて巻き取るように絵を変形する。
//! 巻いた部分の位置・裏地・影の計算は元のシェーダーと同じ。裏地は元の絵か、選んだレイヤーの絵。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

type C = [f64; 4];

fn thicken(c: C, density: f64) -> C {
    let r = if c[3] < 1.0 / 1024.0 { density * (1.0 - c[3]) } else { (1.0 - (1.0 - c[3]).max(0.0).powf(density)) / c[3] };
    c.map(|v| v * r)
}
fn over(base: C, top: C) -> C {
    [0, 1, 2, 3].map(|i| (1.0 - top[3]) * base[i] + top[i])
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let distance = (f.f(0) * sc).max(0.0);
    let angle = f.f(1).to_radians();
    let width = (f.f(2) * sc).max(8.0 * sc);
    let (w, h) = (f.width as f64, f.height as f64);
    let (vx, vy) = (f.f(3) * sc + w / 2.0, f.f(4) * sc + h / 2.0);
    let fov = f.f(5).to_radians().clamp(0.0, 2.0 / 3.0 * PI);
    let shadow = (f.f(6) / 100.0).clamp(0.0, 1.0);
    let unbound = f.b(7);
    let back_orient = f.i(9);
    if distance == 0.0 {
        return Ok(());
    }
    let (c, s) = (angle.cos(), angle.sin());
    let mut px = w / 2.0 - if s > 0.0 { 1.0 } else { -1.0 } * w / 2.0 + s * distance;
    let mut py = h / 2.0 + if c > 0.0 { 1.0 } else { -1.0 } * h / 2.0 - c * distance;
    let d = c * (vx - px) + s * (vy - py);
    px += d * c;
    py += d * s;
    let fov_rate = (fov / 2.0).tan();
    let dd = w.hypot(h);
    let tilt_z = (2.0 / dd * fov_rate).sqrt();
    let rr0 = (width / dd) * fov_rate;
    let rad_rate = 1.0 / (rr0 + (rr0 * rr0 + 1.0).sqrt());
    let radius = rad_rate * width / 2.0;
    let (mut l, mut r, mut t, mut b) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    if unbound {
        let u0 = s * px - c * py;
        let mut ts = Vec::new();
        if u0 > 0.0 { ts.push(0.0); }
        if u0 - s * w > 0.0 { ts.push(c * w); }
        if u0 + c * h > 0.0 { ts.push(s * h); }
        if u0 - s * w + c * h > 0.0 { ts.push(c * w + s * h); }
        if s != 0.0 {
            let x = u0 / s;
            if 0.0 < x && x < w { ts.push(c * x); }
            let x = (c * h + u0) / s;
            if 0.0 < x && x < w { ts.push(c * x + s * h); }
        }
        if c != 0.0 {
            let y = -u0 / c;
            if 0.0 < y && y < h { ts.push(s * y); }
            let y = (s * w - u0) / c;
            if 0.0 < y && y < h { ts.push(c * w + s * y); }
        }
        if !ts.is_empty() {
            let tm = ts.iter().cloned().fold(f64::INFINITY, f64::min);
            let tmx = ts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let t0 = c * px + s * py;
            let k = 1.0 / (1.0 - 2.0 * ((1.0 - (distance / radius).min(PI).cos()) * radius / dd) * fov_rate);
            let (mut a, mut bb, mut cc, mut d2) = (k * c * (tm - t0), k * c * (tmx - t0), k * s * (tm - t0), k * s * (tmx - t0));
            let w2 = (width / 2.0).min(k * (distance / radius).min(PI / 2.0).sin() * radius);
            if a > bb { std::mem::swap(&mut a, &mut bb); }
            if cc > d2 { std::mem::swap(&mut cc, &mut d2); }
            l = (-(a - s.abs() * w2 + px)).ceil().max(0.0);
            r = (bb + s.abs() * w2 + px - w).ceil().max(0.0);
            t = (-(cc - c.abs() * w2 + py)).ceil().max(0.0);
            b = (d2 + c.abs() * w2 + py - h).ceil().max(0.0);
            let lim = 4000.0;
            l = l.min(lim); r = r.min(lim); t = t.min(lim); b = b.min(lim);
        }
    }
    let img = f.input();
    // 裏地: 選んだレイヤーの絵を、元の絵の縦横比に合わせて中央で切り抜く。無ければ元の絵
    let back = f.layer_image(8).map(|bi| {
        let (bw, bh) = (bi.w as f64, bi.h as f64);
        if bw * h < bh * w {
            let cut = ((bh - bw * h / w).round()).min(bh - 1.0) as usize;
            bi.crop(0, (cut - cut / 2) as isize, bi.w, bi.h - cut)
        } else {
            let cut = ((bw - bh * w / h).round()).min(bw - 1.0) as usize;
            bi.crop((cut - cut / 2) as isize, 0, bi.w - cut, bi.h)
        }
    });
    let back_img = back.as_ref().unwrap_or(&img);
    let (bsx, bsy) = (back_img.w as f64 / w, back_img.h as f64 / h);
    let (fx_, fx0) = if back_orient % 2 == 1 { (-1.0, back_img.w as f64) } else { (1.0, 0.0) };
    let (fy_, fy0) = if back_orient >= 2 { (-1.0, back_img.h as f64) } else { (1.0, 0.0) };
    let dir = [s, -c];
    let tz = [tilt_z * c, tilt_z * s];
    let hw = width / 2.0;
    let thr = (width / dd) * fov_rate * rad_rate;
    let (ow, oh) = ((l + w + r) as usize, (t + h + b) as usize);
    let get = |im: &Image, x: f64, y: f64| -> C { let p = im.sample(x as f32, y as f32); [p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64] };
    let roll = [-2.0 * PI * radius * dir[0], -2.0 * PI * radius * dir[1]];
    let inside = |p: [f64; 2]| (0..2).all(|k| if roll[k] < 0.0 { p[k] >= -0.5 } else { p[k] < [w, h][k] + 0.5 });
    let out = Image::generate(ow, oh, |x, y| {
        let p0 = [x as f64 + 0.5 - l, y as f64 + 0.5 - t];
        let ll = dir[0] * (p0[0] - px) + dir[1] * (p0[1] - py);
        let mut col: C = if ll >= 0.0 && p0[0] >= 0.0 && p0[1] >= 0.0 && p0[0] < w && p0[1] < h { let q = img.px(p0[0] as usize, p0[1] as usize); [q[0] as f64, q[1] as f64, q[2] as f64, q[3] as f64] } else { [0.0; 4] };
        if -hw <= ll && ll <= hw {
            let rr = (ll / radius).abs();
            let ss = rr * thr;
            let a00 = ss.atan();
            let a01 = ((rr - ss) / (1.0 + ss * ss).sqrt()).min(1.0).acos();
            let (a1, a2) = if ll < 0.0 { (PI / 2.0 + a00 - a01, PI / 2.0 + a00 + a01) } else { (1.5 * PI - a00 + a01, 1.5 * PI - a00 - a01) };
            let (z1, z2) = (radius * (1.0 - a1.cos()), radius * (1.0 - a2.cos()));
            let hh = (1.0 - ll * ll / (hw * hw)).max(0.0).sqrt();
            let sh = 1.0 - (1.0 - hh) * shadow;
            let density = 1.0 / hh.max(1.0 / 1024.0);
            let pt0 = [p0[0] - ll * dir[0], p0[1] - ll * dir[1]];
            let tdot = tz[0] * (p0[0] - px) + tz[1] * (p0[1] - py);
            let tz0 = [tdot * tz[0], tdot * tz[1]];
            let (mut cb, mut cf) = ([0.0; 4], [0.0; 4]);
            let mut pt = [pt0[0] - a1 * radius * dir[0] - z1 * tz0[0], pt0[1] - a1 * radius * dir[1] - z1 * tz0[1]];
            let mut n = 0;
            while inside(pt) && n < 64 {
                cb = over(cb, thicken(get(&img, pt[0], pt[1]), density));
                pt = [pt[0] + roll[0], pt[1] + roll[1]];
                n += 1;
            }
            let mut pt = [pt0[0] - a2 * radius * dir[0] - z2 * tz0[0], pt0[1] - a2 * radius * dir[1] - z2 * tz0[1]];
            let mut n = 0;
            while inside(pt) && n < 64 {
                let bx = fx_ * pt[0] * bsx + fx0;
                let by = fy_ * pt[1] * bsy + fy0;
                cf = over(thicken(get(back_img, bx, by), density), cf);
                pt = [pt[0] + roll[0], pt[1] + roll[1]];
                n += 1;
            }
            for k in 0..3 { cb[k] *= sh; }
            cb = over(cb, cf);
            for k in 0..3 { cb[k] *= sh; }
            let k = (hw - ll.abs()).clamp(0.0, 1.0);
            cb = cb.map(|v| v * k);
            col = over(col, cb);
        }
        col.map(|v| v as f32)
    });
    f.output(out, -(l as i32), -(t as i32));
    Ok(())
}

plugin! {
    vendor: "σ軸（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sigma-axis.PageRoll_S", name: "PageRoll_S", category: "変形", flags: THREAD_SAFE,
            params: [
                float(c"distance", c"距離", 0.0, 0.0, 4000.0),
                float(c"angle", c"角度", -90.0, -3600.0, 3600.0),
                float(c"width", c"太さ", 80.0, 8.0, 4000.0),
                float(c"x", c"視点X", 0.0, -4000.0, 4000.0),
                float(c"y", c"視点Y", 0.0, -4000.0, 4000.0),
                float(c"fov", c"視野角", 70.0, 0.0, 120.0),
                float(c"shadow", c"陰影", 50.0, 0.0, 100.0),
                boolean(c"unbound", c"領域外も描画", true),
                layer(c"back", c"裏地レイヤー"),
                choice(c"back_orient", c"裏地向き", 0, c"通常|左右反転|上下反転|180°反転"),
            ],
            render: render,
        },
    ]
}

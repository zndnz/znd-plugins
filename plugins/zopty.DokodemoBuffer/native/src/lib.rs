//! どこでもバッファ（zopty）の移植: 他のオブジェクトの絵・自分の過去の絵を取り込んで重ねる。
//! 入出力（他のレイヤーの絵を重ねる）・遅延・残像・ディスプレイスメントの 4 つ。
//! 元は名前付きのバッファに絵を保存して別の場所で取り出すが、znd では LAYER パラメータと過去の時刻の絵の取得で行う。

use znd_plugin_sdk::*;

const BLENDS: &std::ffi::CStr = c"通常|置換|加算|マスク|型抜き";

/// src を dst に重ねる。mode: 0 通常 / 1 置換 / 2 加算 / 3 マスク（アルファを掛ける）/ 4 型抜き（アルファの反転を掛ける）
fn blend(dst: &mut Image, src: &Image, alpha: f32, mode: usize) {
    for (d, s) in dst.data.chunks_exact_mut(4).zip(src.data.chunks_exact(4)) {
        let a = s[3] * alpha;
        match mode {
            0 => {
                for k in 0..4 {
                    d[k] = s[k] * alpha + d[k] * (1.0 - a);
                }
            }
            1 => d.iter_mut().zip(s).for_each(|(d, s)| *d = s * alpha),
            2 => {
                for k in 0..3 {
                    d[k] += s[k] * alpha;
                }
                d[3] = (d[3] + a).min(1.0);
            }
            3 => d.iter_mut().for_each(|v| *v *= a),
            _ => d.iter_mut().for_each(|v| *v *= 1.0 - a),
        }
    }
}

fn io(f: &mut Frame) -> FxResult {
    let Some(src) = f.layer_image(0) else { return Ok(()) };
    let alpha = (1.0 - f.f(1) / 100.0).clamp(0.0, 1.0) as f32;
    let mut img = if f.b(3) { Image::new(f.width, f.height) } else { f.input() };
    blend(&mut img, &src, alpha, f.i(2));
    f.set(img);
    Ok(())
}

fn delayed(f: &Frame, frames: f64) -> Option<Image> {
    let t = f.time() - frames / f.fps().max(1.0);
    if t < f.time() - f.layer_time() - 1e-9 {
        return None;
    }
    f.frame_at(t).filter(|i| i.w == f.width && i.h == f.height)
}

fn delay(f: &mut Frame) -> FxResult {
    let n = f.f(0).round().max(1.0);
    let alpha = (1.0 - f.f(1) / 100.0).clamp(0.0, 1.0) as f32;
    let mode = f.i(2);
    let mut out = Image::new(f.width, f.height);
    if let Some(past) = delayed(f, n) {
        blend(&mut out, &past, alpha, mode);
    }
    let cur = f.input();
    blend(&mut out, &cur, 1.0, 0);
    f.set(out);
    Ok(())
}

fn afterimage(f: &mut Frame) -> FxResult {
    let interval = f.f(0).round().max(1.0);
    let alpha = (1.0 - f.f(1) / 100.0).clamp(0.0, 1.0) as f32;
    let count = f.f(2).round().clamp(1.0, 100.0) as usize;
    let mode = f.i(3);
    let mut out = Image::new(f.width, f.height);
    // 古いものから順に重ねる
    for i in (1..=count).rev() {
        if let Some(past) = delayed(f, interval * i as f64) {
            blend(&mut out, &past, alpha, mode);
        }
    }
    let cur = f.input();
    blend(&mut out, &cur, 1.0, 0);
    f.set(out);
    Ok(())
}

fn displacement(f: &mut Frame) -> FxResult {
    let Some(map) = f.layer_image(0) else { return Ok(()) };
    let s = f.scale();
    let (w, h) = (f.width as f64, f.height as f64);
    let mv = (f.f(1) * s, f.f(2) * s);
    let scale = (-10.0 * f.f(3) / 100.0).exp();
    let rot = (-f.f(4)).to_radians();
    let chab = (-f.f(5) / 1000.0).exp();
    let five = f.i(6) == 1;
    let clear = f.b(7);
    let src = f.input();
    let at = |x: f64, y: f64, d: f64| -> [f32; 4] {
        // 中心からの位置を、値 d に比例して回転・移動・拡大する
        let (u, v) = (x - w / 2.0, y - h / 2.0);
        let (sn, cs) = (rot * d).sin_cos();
        let (ru, rv) = (cs * u - sn * v, sn * u + cs * v);
        let k = (d * scale.ln()).exp();
        let (px, py) = ((ru + mv.0 * d) * k + w / 2.0, (rv + mv.1 * d) * k + h / 2.0);
        src.sample(px as f32, py as f32)
    };
    let mut out = Image::new(src.w, src.h);
    out.map(|x, y, _| {
        let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
        let m = color::to_display(map.px(x.min(map.w - 1), y.min(map.h - 1)));
        let (r, g, b) = ((m[0] * m[3]) as f64, (m[1] * m[3]) as f64, (m[2] * m[3]) as f64);
        let cr = at(px, py, r / (chab * chab));
        let cg = at(px, py, g);
        let cb = at(px, py, b * chab * chab);
        if five {
            let co = at(px, py, 0.5 * (r + g) / chab);
            let cc = at(px, py, 0.5 * (g + b) * chab);
            [(cr[0] + co[0]) / 2.0, (co[1] + cg[1] + cc[1]) / 3.0, (cc[2] + cb[2]) / 2.0, (cr[3] + co[3] + cg[3] + cc[3] + cb[3]) / 5.0]
        } else {
            [cr[0], cg[1], cb[2], (cr[3] + cg[3] + cb[3]) / 3.0]
        }
    });
    if !clear {
        let base = f.input();
        let mut under = base;
        blend(&mut under, &out, 1.0, 0);
        out = under;
    }
    f.set(out);
    Ok(())
}

plugin! {
    vendor: "zopty（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "zopty.DokodemoBuffer.io", name: "どこでもバッファ（入出力）", category: "どこでもバッファ", flags: THREAD_SAFE,
            params: [layer(c"source", c"取り込むレイヤー"), float(c"alpha", c"透明度", 0.0, 0.0, 100.0), choice(c"blend", c"合成モード", 0, BLENDS), boolean(c"clear", c"背景クリア", false)],
            render: io,
        },
        {
            id: "zopty.DokodemoBuffer.delay", name: "どこでもバッファ（遅延）", category: "どこでもバッファ", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [float(c"frames", c"遅延（フレーム）", 1.0, 1.0, 600.0), float(c"alpha", c"透明度", 0.0, 0.0, 100.0), choice(c"blend", c"合成モード", 0, BLENDS)],
            render: delay,
        },
        {
            id: "zopty.DokodemoBuffer.afterimage", name: "どこでもバッファ（残像）", category: "どこでもバッファ", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"interval", c"遅延（フレーム）", 1.0, 1.0, 600.0),
                float(c"alpha", c"透明度", 0.0, 0.0, 100.0),
                float(c"count", c"残像数", 1.0, 1.0, 100.0),
                choice(c"blend", c"合成モード", 0, BLENDS),
            ],
            render: afterimage,
        },
        {
            id: "zopty.DokodemoBuffer.displacement", name: "どこでもバッファ（ディスプレイスメント）", category: "どこでもバッファ", flags: THREAD_SAFE,
            params: [
                layer(c"map", c"マップのレイヤー"),
                float(c"dx", c"移動X", 0.0, -5000.0, 5000.0),
                float(c"dy", c"移動Y", 0.0, -5000.0, 5000.0),
                float(c"ds", c"拡大", 0.0, -100.0, 100.0),
                float(c"rot", c"回転", 0.0, -3600.0, 3600.0),
                float(c"chab", c"色収差の強度", 0.0, -100.0, 100.0),
                choice(c"chab_mode", c"色収差の精度", 0, c"3 段階|5 段階"),
                boolean(c"clear", c"背景クリア", false),
            ],
            render: displacement,
        },
    ]
}

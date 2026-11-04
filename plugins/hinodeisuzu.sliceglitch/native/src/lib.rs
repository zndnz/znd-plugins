//! SliceGlitch（暘 弥涼）の移植: 横の帯をずらすスライス・色ずれ・低音に合わせた振動。

use znd_plugin_sdk::*;

/// 低音の強さ（0～1）。元の getaudio の spectrum 8 帯域の 2～8 番の平均 / 2000 に近い目盛り。
fn bass(f: &Frame) -> f32 {
    let Some(buf) = f.audio(7, 2048) else { return 0.0 };
    let spec = audio::spectrum(&buf);
    let v = audio::band(&spec, 48000.0, 40.0, 400.0) * 32768.0;
    (v / 2000.0).min(1.0)
}

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let slice_amt = (f.f(0) * sc).round() as i64;
    let slice_cnt = f.f(1).round().max(0.0) as i64;
    let prob = f.f(2);
    let chroma = f.f(3) * sc;
    let shake = f.f(4) * sc;
    let use_audio = f.b(5);
    let chroma_bass = f.b(6);
    let fr = f.frame();
    let b = if use_audio { if f.has_layer(7) { bass(f) } else { 0.0 } } else { 1.0 } as f64;
    let src = f.input();
    let (w, h) = (src.w, src.h);
    let mut img = src.clone();
    if slice_cnt > 0 && slice_amt > 0 {
        for i in 1..=slice_cnt {
            if (noise::rand_int(0, 100, i * 7, fr) as f64) < prob {
                let sy = noise::rand_int(0, h as i64 - 1, i * 11, fr);
                let mut sh = noise::rand_int(5, 30, i * 13, fr);
                if sy + sh > h as i64 {
                    sh = h as i64 - sy;
                }
                let ox = noise::rand_int(-slice_amt, slice_amt, i * 17, fr);
                let band = src.crop(0, sy as isize, w, sh.max(0) as usize);
                img.draw(&band, ox as isize, sy as isize, 1.0);
            }
        }
    }
    // 色ずれ: R と B を左右逆にずらす
    let shift = if chroma_bass { chroma * (1.0 + b * 0.5) } else { chroma };
    if shift > 0.0 {
        let s = shift.round() as isize;
        let base = img.clone();
        for y in 0..h {
            for x in 0..w {
                let (xi, yi) = (x as isize, y as isize);
                let r = base.get(xi - s, yi);
                let g = base.get(xi, yi);
                let bl = base.get(xi + s, yi);
                let a = r[3].max(g[3]).max(bl[3]);
                img.set(x, y, [r[0], g[1], bl[2], a]);
            }
        }
    }
    let amp = shake * b;
    let (mut ox, mut oy) = (0, 0);
    if amp > 0.0 {
        ox = (noise::rand_int(-1000, 1000, 101, fr) as f64 / 1000.0 * amp).round() as i32;
        oy = (noise::rand_int(-1000, 1000, 202, fr) as f64 / 1000.0 * amp).round() as i32;
    }
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "暘 弥涼（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "hinodeisuzu.sliceglitch", name: "SliceGlitch", category: "加工", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"slice_amt", c"スライスずれ幅", 25.0, 0.0, 500.0),
                float(c"slice_cnt", c"スライス本数", 6.0, 0.0, 30.0),
                float(c"slice_prob", c"スライス発生率%", 60.0, 0.0, 100.0),
                float(c"chroma", c"色収差", 18.0, 0.0, 100.0),
                float(c"shake", c"振動量", 10.0, 0.0, 200.0),
                boolean(c"audio", c"低音に連動(音声)する", true),
                boolean(c"chroma_bass", c"色収差も低音で増幅", true),
                layer(c"audio_layer", c"音声レイヤー"),
            ],
            render: render,
        },
    ]
}

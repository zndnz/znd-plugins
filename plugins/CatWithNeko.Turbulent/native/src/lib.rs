//! Turbulent（こんにゃくねこ）の移植: ぼかしたノイズの明るさで斜めにずらす簡易タービュレントディスプレイス。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let sc = f.scale();
    let amount = f.f(0) * sc;
    let scale = f.f(1).max(1e-3) * sc;
    let speed = f.f(2);
    let seed = f.f(3) as u32 ^ f.seed();
    let blur = f.f(4) * sc;
    let expand = f.b(5);
    let pad = if expand { amount.ceil().max(0.0) as usize } else { 0 };
    let src = f.input().padded(pad, pad, pad, pad);
    let (w, h) = (src.w, src.h);
    // ノイズの地図（明るさ 0～1）。周期は scale px、時間で speed % ずつ流れる
    let z = (f.time() * speed / 100.0) as f32;
    let mut map = Image::generate(w, h, |x, y| {
        let v = noise::value3(x as f32 / scale as f32, y as f32 / scale as f32, z, seed);
        [v, v, v, 1.0]
    });
    if blur > 0.0 {
        aul::blur(&mut map, blur as f32);
    }
    let mut out = Image::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let lum = 2.0 * map.px(x, y)[0] as f64 - 1.0;
            let d = (lum * amount) as isize;
            out.set(x, y, src.get(x as isize + d, y as isize + d));
        }
    }
    f.output(out, -(pad as i32), -(pad as i32));
    Ok(())
}

plugin! {
    vendor: "こんにゃくねこ（znd 移植: znd-plugins）", version: "1.0.0", license: "CC0-1.0",
    effects: [
        {
            id: "CatWithNeko.Turbulent", name: "Turbulent", category: "変形", flags: THREAD_SAFE | TIME_DEPENDENT,
            params: [
                float(c"amount", c"Amount", 10.0, 0.0, 1000.0),
                float(c"scale", c"Scale", 100.0, 0.0, 4000.0),
                float(c"speed", c"Speed", 100.0, 0.0, 1000.0),
                float(c"seed", c"Seed", 0.0, 0.0, 4000.0),
                float(c"blur", c"Blur", 15.0, 0.0, 1000.0),
                boolean(c"expansion", c"Expansion", true),
            ],
            render: render,
        },
    ]
}

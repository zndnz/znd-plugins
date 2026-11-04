//! 放射クリッピング（supernyan74）の移植: オブジェクトの中心から放射状に、扇形を交互に透明にする。

use znd_plugin_sdk::*;

fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn render(f: &mut Frame) -> FxResult {
    let count = f.f(0).round().clamp(1.0, 100.0);
    let base = f.f(1).rem_euclid(360.0).to_radians();
    let ratio = f.f(2) / 100.0;
    let (cx, cy) = (f.width as f64 / 2.0, f.height as f64 / 2.0);
    let w = f.width;
    let seg = std::f64::consts::TAU / count;
    for (i, p) in f.rgba.chunks_exact_mut(4).enumerate() {
        let (x, y) = ((i % w) as f64 + 0.5 - cx, (i / w) as f64 + 0.5 - cy);
        let r = x.hypot(y);
        let angle = if r > 1e-6 { y.atan2(x) } else { 0.0 };
        let phase = ((angle - base) / seg + 0.5).rem_euclid(1.0);
        let delta = ((phase - 0.5).abs() - ratio / 2.0) * seg;
        // 境界線からの距離（画素）でアンチエイリアスする
        let m = smoothstep(-0.5, 0.5, r * delta.sin()) as f32;
        p.iter_mut().for_each(|v| *v *= m);
    }
    Ok(())
}

plugin! {
    vendor: "supernyan74（znd 移植: znd-plugins）", version: "1.1", license: "MIT",
    effects: [
        {
            id: "supernyan74.RadialClipping", name: "放射クリッピング", category: "クリッピング", flags: THREAD_SAFE,
            params: [
                float(c"count", c"数", 4.0, 1.0, 100.0),
                float(c"base_angle", c"基準角", 0.0, 0.0, 360.0),
                float(c"clip_ratio", c"割合（%）", 50.0, 0.0, 100.0),
            ],
            render: render,
        },
    ]
}

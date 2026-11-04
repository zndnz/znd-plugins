//! RadRotDirBlur_S（σ軸）の移植: 放射ブラー・回転ブラー・方向ブラーを 1 つにした複合ブラー（色収差付き）。
//! 拡大・回転・移動を 0 から目標まで少しずつ変えた絵の平均を取る。元は行列を順に掛けて求めるが、ここでは各段の変換を直接計算する。

use znd_plugin_sdk::*;

#[derive(Clone, Copy)]
struct Path {
    rad: f64,
    rot: f64,
    mx: f64,
    my: f64,
    p1: f64,
    p2: f64,
}

impl Path {
    /// 段 s（p1..p2）で、出力の位置 q（中心からの相対）に写る元の位置（中心からの相対）
    fn inverse(&self, s: f64, q: (f64, f64)) -> (f64, f64) {
        let (x, y) = (q.0 - self.mx * s, q.1 - self.my * s);
        let (sn, cs) = (-self.rot * s).sin_cos();
        let k = self.rad.powf(s);
        ((cs * x - sn * y) / k, (sn * x + cs * y) / k)
    }
    fn forward(&self, s: f64, p: (f64, f64)) -> (f64, f64) {
        let k = self.rad.powf(s);
        let (sn, cs) = (self.rot * s).sin_cos();
        let (x, y) = (p.0 * k, p.1 * k);
        (cs * x - sn * y + self.mx * s, sn * x + cs * y + self.my * s)
    }
    fn at(&self, i: usize, n: usize) -> f64 {
        self.p1 + (self.p2 - self.p1) * i as f64 / (n - 1).max(1) as f64
    }
}

fn render(f: &mut Frame) -> FxResult {
    let s = f.scale();
    let amount = f.f(5) / 100.0;
    let rad = (f.f(2) / 100.0).max(0.01);
    let (dir_x, dir_y, rot0) = (f.f(0) * s, f.f(1) * s, f.f(3).to_radians());
    if amount == 0.0 || (rad == 1.0 && rot0 == 0.0 && dir_x == 0.0 && dir_y == 0.0) {
        return Ok(());
    }
    let rel = (f.f(6) / 100.0).clamp(-1.0, 1.0);
    let chroma = f.i(7);
    let chrm = (f.f(8) / 100.0).clamp(-1.0, 1.0);
    let keep = f.b(9);
    let n = f.f(10).round().clamp(2.0, 4096.0) as usize;
    let c = f.point(4);
    let base = Path { rad: rad.powf(amount), rot: rot0 * amount, mx: dir_x * amount, my: dir_y * amount, p1: (rel - 1.0) / 2.0, p2: (rel + 1.0) / 2.0 };
    let src = f.input();
    let (w, h) = (src.w as f64, src.h as f64);
    // 広げる大きさ: 元の四隅が段ごとにどこへ動くか
    let (mut l, mut t, mut r, mut b) = (0.0f64, 0.0f64, w, h);
    if !keep {
        for i in 0..=64 {
            let sv = base.p1 + (base.p2 - base.p1) * i as f64 / 64.0;
            for (x, y) in [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h), (w / 2.0, 0.0), (w, h / 2.0), (w / 2.0, h), (0.0, h / 2.0)] {
                let (px, py) = base.forward(sv, (x - c[0], y - c[1]));
                l = l.min(px + c[0]);
                t = t.min(py + c[1]);
                r = r.max(px + c[0]);
                b = b.max(py + c[1]);
            }
        }
    }
    let (ox, oy) = (l.floor(), t.floor());
    let (ow, oh) = (((r.ceil() - ox) as usize).min(8192), ((b.ceil() - oy) as usize).min(8192));
    let pick = |x: f64, y: f64| if keep { src.sample_clamped(x as f32, y as f32) } else { src.sample(x as f32, y as f32) };
    // 色ごとの経路（色収差）
    let paths: Option<[Path; 3]> = (chrm != 0.0).then(|| {
        let (t2, t3) = (1.0 - chrm * chrm / 2.0, 1.0 - chrm * chrm);
        let scaled = |k: f64| Path { p1: base.p1 * k, p2: base.p2 * k, ..base };
        let mut ps = [scaled(t3), scaled(t2), base];
        if chrm < 0.0 {
            ps.swap(0, 2);
        }
        match chroma % 3 {
            1 => ps.swap(1, 2),
            2 => ps.swap(0, 1),
            _ => {}
        }
        ps
    });
    let blend_b = chroma >= 3;
    let mut out = Image::new(ow, oh);
    out.map(|x, y, _| {
        let q = (x as f64 + 0.5 + ox - c[0], y as f64 + 0.5 + oy - c[1]);
        match &paths {
            None => {
                let mut acc = [0.0f32; 4];
                for i in 0..n {
                    let p = base.inverse(base.at(i, n), q);
                    let v = pick(p.0 + c[0], p.1 + c[1]);
                    for k in 0..4 {
                        acc[k] += v[k];
                    }
                }
                acc.map(|v| v / n as f32)
            }
            Some(ps) => {
                let (mut col, mut alp) = ([0.0f32; 3], [0.0f32; 3]);
                for (ch, path) in ps.iter().enumerate() {
                    for i in 0..n {
                        let p = path.inverse(path.at(i, n), q);
                        let v = pick(p.0 + c[0], p.1 + c[1]);
                        col[ch] += v[ch];
                        alp[ch] += v[3];
                    }
                }
                let col = col.map(|v| v / n as f32);
                let alp = alp.map(|v| v / n as f32);
                let a = (alp[0] + alp[1] + alp[2]) / 3.0;
                let rgb: [f32; 3] = std::array::from_fn(|k| if blend_b { if alp[k] > 0.0 { col[k] / alp[k] * a } else { 0.0 } } else { col[k].min(a) });
                [rgb[0], rgb[1], rgb[2], a]
            }
        }
    });
    f.output(out, ox as i32, oy as i32);
    Ok(())
}

plugin! {
    vendor: "σ軸（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "sigma-axis.RadRotDirBlur_S", name: "複合ブラー（RadRotDirBlur_S）", category: "ブラー", flags: THREAD_SAFE,
            params: [
                float(c"dir_x", c"移動X", 0.0, -4000.0, 4000.0),
                float(c"dir_y", c"移動Y", 0.0, -4000.0, 4000.0),
                float(c"rad", c"拡大率", 100.0, 1.0, 10000.0),
                float(c"rot", c"回転角", 0.0, -3600.0, 3600.0),
                point(c"center", c"中心", 0.5, 0.5),
                float(c"amount", c"強さ", 100.0, -1000.0, 1000.0),
                float(c"rel_pos", c"相対位置", 0.0, -100.0, 100.0),
                choice(c"chroma", c"色収差", 3, c"赤青A|赤緑A|緑青A|赤青B|赤緑B|緑青B"),
                float(c"chrm_abrr", c"色収差強さ", 0.0, -100.0, 100.0),
                boolean(c"keep_size", c"サイズ固定", false),
                float(c"quality", c"精度", 512.0, 2.0, 4096.0),
            ],
            render: render,
        },
    ]
}

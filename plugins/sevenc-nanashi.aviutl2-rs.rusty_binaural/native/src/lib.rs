//! Rusty Binaural Filter（Nanashi.）の移植: 音をモノラルにして、指定した向きから聞こえるように左右に振り分ける。
//! 元は HRIR（頭部インパルス応答）のデータを畳み込むが、データを同梱できないので、
//! 頭の模型（Woodworth の両耳時間差 + Brown–Duda の頭の影フィルタ）で同じ向きの手がかりを作る。

use std::f64::consts::PI;
use znd_plugin_sdk::*;

const HEAD: f64 = 0.0875; // 頭の半径 (m)
const SOUND: f64 = 343.0; // 音速 (m/s)
const HIST: usize = 256;

/// Brown–Duda の頭の影（1 極 1 零のシェルフ）。θ は耳の向きと音源の角度。
struct Shadow {
    b0: f64,
    b1: f64,
    a1: f64,
    x1: f64,
    y1: f64,
}
impl Shadow {
    fn new() -> Self {
        Shadow { b0: 1.0, b1: 0.0, a1: 0.0, x1: 0.0, y1: 0.0 }
    }
    fn set(&mut self, theta: f64, sr: f64) {
        let w0 = SOUND / HEAD;
        let alpha = 1.05 + 0.95 * (theta / 150f64.to_radians() * PI).cos();
        // 双一次変換: H(s) = (alpha s + 2 w0) / (s + 2 w0)
        let k = 2.0 * sr;
        let (b0, b1) = (alpha * k + 2.0 * w0, -alpha * k + 2.0 * w0);
        let (a0, a1) = (k + 2.0 * w0, -k + 2.0 * w0);
        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.a1 = a1 / a0;
    }
    fn run(&mut self, x: f64) -> f64 {
        let y = self.b0 * x + self.b1 * self.x1 - self.a1 * self.y1;
        self.x1 = x;
        self.y1 = y;
        y
    }
}

struct St {
    hist: Vec<f64>,
    pos: usize,
    sh: [Shadow; 2],
}

static STATE: AudioState<St> = AudioState::new();

fn process(a: &mut Audio) -> FxResult {
    let gain = a.f(0);
    let yaw = a.f(1).to_radians();
    let pitch = a.f(2).to_radians();
    let sr = a.sample_rate();
    // 音源の向き（右が +x、前が +z）。横からの角度で左右の耳への届き方を決める
    let (x, z) = (pitch.cos() * yaw.sin(), pitch.cos() * yaw.cos());
    let lateral = x.clamp(-1.0, 1.0).asin(); // -π/2（左）～ π/2（右）
    let _ = z;
    // 両耳時間差（遠い耳が遅れる）
    let itd = HEAD / SOUND * (lateral.abs() + lateral.abs().sin()) * sr;
    let (dl, dr) = if lateral > 0.0 { (itd, 0.0) } else { (0.0, itd) };
    // 各耳から見た音源の角度（耳は真横 ±90°）
    let th_r = (PI / 2.0 - lateral).abs();
    let th_l = (PI / 2.0 + lateral).abs();
    // 上下: 真上・真下ほど左右差が小さく、少し音量が落ちる
    let elev = 1.0 - 0.15 * pitch.sin().abs();
    let n = a.frames();
    let mut out = vec![0.0f32; n * 2];
    STATE.with(a, || St { hist: vec![0.0; HIST], pos: 0, sh: [Shadow::new(), Shadow::new()] }, |st| {
        st.sh[0].set(th_l, sr);
        st.sh[1].set(th_r, sr);
        for k in 0..n {
            let m = 0.5 * (a.samples[k * 2] + a.samples[k * 2 + 1]) as f64;
            st.hist[st.pos] = m;
            let read = |d: f64| {
                let d = d.clamp(0.0, (HIST - 2) as f64);
                let i = d.floor() as usize;
                let f = d - i as f64;
                let at = |j: usize| st.hist[(st.pos + HIST - j) % HIST];
                at(i) * (1.0 - f) + at(i + 1) * f
            };
            let (l, r) = (read(dl), read(dr));
            st.pos = (st.pos + 1) % HIST;
            out[k * 2] = (st.sh[0].run(l) * gain * elev * 0.5) as f32;
            out[k * 2 + 1] = (st.sh[1].run(r) * gain * elev * 0.5) as f32;
        }
    });
    a.samples.copy_from_slice(&out);
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-rs.rusty_binaural", name: "Rusty Binaural", category: "音声", flags: THREAD_SAFE,
            params: [
                float(c"gain", c"ゲイン", 2.0, 0.0, 8.0),
                float(c"rotate_yaw", c"横回転", 0.0, -180.0, 180.0),
                float(c"rotate_pitch", c"縦回転", 0.0, -90.0, 90.0),
            ],
            audio: process,
        },
    ]
}

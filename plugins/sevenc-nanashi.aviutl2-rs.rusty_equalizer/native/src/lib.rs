//! Rusty Equalizer Filter（Nanashi.）の移植: 低・中・高のピーキング EQ と、ハイパス・ローパス。
//! フィルタは元と同じ双 2 次（Q = 1/√2）。状態はブロックをまたいで続ける。

use std::f64::consts::FRAC_1_SQRT_2 as Q;
use znd_plugin_sdk::audio::{Biquad, BiquadKind};
use znd_plugin_sdk::*;

struct Ch {
    f: [Biquad; 5],
}
struct St {
    ch: [Ch; 2],
}

static STATE: AudioState<St> = AudioState::new();

fn kinds(a: &Audio) -> [(BiquadKind, f64); 5] {
    [
        (BiquadKind::Peak(a.f(3)), a.f(2)),
        (BiquadKind::Peak(a.f(5)), a.f(4)),
        (BiquadKind::Peak(a.f(7)), a.f(6)),
        (BiquadKind::LowPass, a.f(11)),
        (BiquadKind::HighPass, a.f(9)),
    ]
}

fn process(a: &mut Audio) -> FxResult {
    if a.b(0) {
        return Ok(());
    }
    let wet = a.f(1);
    let sr = a.sample_rate();
    let ks = kinds(a);
    let on = [a.f(3) != 0.0, a.f(5) != 0.0, a.f(7) != 0.0, a.b(10), a.b(8)];
    let mk = || {
        let ch = || Ch { f: ks.map(|(k, fr)| Biquad::new(k, sr, fr, Q)) };
        St { ch: [ch(), ch()] }
    };
    let n = a.frames();
    let mut buf = a.samples.to_vec();
    STATE.with(a, mk, |st| {
        for (c, ch) in st.ch.iter_mut().enumerate() {
            for (i, f) in ch.f.iter_mut().enumerate() {
                f.set(ks[i].0, sr, ks[i].1, Q);
            }
            for k in 0..n {
                let orig = buf[k * 2 + c] as f64;
                let mut s = orig;
                for (i, f) in ch.f.iter_mut().enumerate() {
                    if on[i] {
                        s = f.run(s);
                    }
                }
                buf[k * 2 + c] = (s * wet + orig * (1.0 - wet)) as f32;
            }
        }
    });
    a.samples.copy_from_slice(&buf);
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-rs.rusty_equalizer", name: "Rusty Equalizer", category: "音声", flags: THREAD_SAFE,
            params: [
                boolean(c"bypass", c"Bypass", false),
                float(c"wet", c"Wet", 1.0, 0.0, 1.0),
                float(c"bass_freq", c"Bass: Frequency", 100.0, 20.0, 250.0),
                float(c"bass_gain", c"Bass: Gain", 0.0, -15.0, 15.0),
                float(c"mid_freq", c"Mid: Frequency", 1000.0, 250.0, 4000.0),
                float(c"mid_gain", c"Mid: Gain", 0.0, -15.0, 15.0),
                float(c"treble_freq", c"Treble: Frequency", 10000.0, 4000.0, 20000.0),
                float(c"treble_gain", c"Treble: Gain", 0.0, -15.0, 15.0),
                boolean(c"hipass_enable", c"Hi-pass: Enable", false),
                float(c"hipass_freq", c"Hi-pass: Frequency", 20.0, 20.0, 20000.0),
                boolean(c"lopass_enable", c"Lo-pass: Enable", false),
                float(c"lopass_freq", c"Lo-pass: Frequency", 20000.0, 20.0, 20000.0),
            ],
            audio: process,
        },
    ]
}

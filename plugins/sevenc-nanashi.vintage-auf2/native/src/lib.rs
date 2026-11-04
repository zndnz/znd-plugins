//! vintage.auf2（sevenc-nanashi）の移植: 音を 1900 年ごろの録音っぽくする（狭い帯域・揺れ・歪み・ノイズ・粗い量子化）。
//! 計算は元と同じ。フィルタやノイズの状態はブロックをまたいで続ける。

use std::f32::consts::PI;
use znd_plugin_sdk::*;

struct St {
    hp_y: f32,
    lp_y: f32,
    hp_x_prev: f32,
    noise: u32,
    crackle: f32,
}

fn soft_clip(x: f32) -> f32 {
    x / (1.0 + x.abs())
}
fn asym_clip(x: f32) -> f32 {
    if x >= 0.0 { x / (1.0 + x) } else { x / (1.0 + 0.5 * x.abs()) }
}
fn quantize(x: f32, bits: u32) -> f32 {
    let l = (1u32 << bits) as f32;
    (x * l).round() / l
}
fn lcg(s: &mut u32) -> f32 {
    *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
    ((*s >> 9) as f32 / (1u32 << 23) as f32) * 2.0 - 1.0
}

static STATE: AudioState<St> = AudioState::new();

fn process(a: &mut Audio) -> FxResult {
    let wet = (a.f(0) / 100.0) as f32;
    let sr = a.sample_rate() as f32;
    if sr <= 0.0 {
        return Ok(());
    }
    let hp_a = (-2.0 * PI * 420.0 / sr).exp();
    let lp_a = (-2.0 * PI * 2500.0 / sr).exp();
    let start = a.start();
    let n = a.frames();
    let mut out = vec![0.0f32; n];
    STATE.with(a, || St { hp_y: 0.0, lp_y: 0.0, hp_x_prev: 0.0, noise: 0x1234_abcd, crackle: 0.0 }, |st| {
        for (i, o) in out.iter_mut().enumerate() {
            let x = (a.samples[i * 2] + a.samples[i * 2 + 1]) * 0.5;
            let t = (start + i as i64) as f32 / sr;
            let am = 1.0 + (2.0 * PI * 0.35 * t).sin() * 0.05 + (2.0 * PI * 4.5 * t).sin() * 0.015;
            let biased = x * am + 0.02;
            let hp = hp_a * (st.hp_y + biased - st.hp_x_prev);
            st.hp_x_prev = biased;
            st.hp_y = hp;
            let lp = (1.0 - lp_a) * hp + lp_a * st.lp_y;
            st.lp_y = lp;
            let y2 = asym_clip(soft_clip(lp * 2.0) * 1.4);
            let rumble = (2.0 * PI * 28.0 * t).sin() * 0.003 + (2.0 * PI * 55.0 * t).sin() * 0.0015;
            let hiss = lcg(&mut st.noise) * 0.004;
            if lcg(&mut st.noise) > 0.995 {
                st.crackle = 1.0;
            }
            st.crackle *= 0.94;
            let crackle = st.crackle * (lcg(&mut st.noise) * 0.08);
            *o = quantize(y2 + rumble + hiss + crackle, 9).clamp(-1.0, 1.0);
        }
    });
    for (i, y) in out.iter().enumerate() {
        for c in 0..2 {
            let s = &mut a.samples[i * 2 + c];
            *s = *s * (1.0 - wet) + y * wet;
        }
    }
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.vintage-auf2", name: "vintage", category: "音声", flags: THREAD_SAFE,
            params: [
                float(c"wet", c"Wet", 100.0, 0.0, 100.0),
            ],
            audio: process,
        },
    ]
}

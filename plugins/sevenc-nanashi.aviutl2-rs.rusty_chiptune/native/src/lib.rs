//! Rusty Chiptune Filter（Nanashi.）の移植: 矩形波・三角波・のこぎり波・正弦波・ノイズを鳴らす音源。
//! 位相はレイヤーのイン点からのサンプル位置で決める（元と同じ）。

use znd_plugin_sdk::*;

fn process(a: &mut Audio) -> FxResult {
    let vol = a.f(0);
    let wave = a.i(1);
    let freq = if a.i(2) == 0 { 440.0 * 2f64.powf((a.f(3) - 69.0) / 12.0) } else { a.f(4) };
    let spc = a.sample_rate() / freq.max(1e-3);
    let start = a.start();
    let seed = a.seed();
    for k in 0..a.frames() {
        let idx = start + k as i64;
        let phase = (idx as f64 / spc).rem_euclid(1.0);
        let v = match wave {
            1 => if phase < 0.5 { phase * 4.0 - 1.0 } else { (1.0 - phase) * 4.0 - 1.0 },
            2 => phase * 2.0 - 1.0,
            3 => (phase * 2.0 * std::f64::consts::PI).sin(),
            4 => noise::rand01(seed, idx as u32, (idx >> 32) as u32) as f64 * 2.0 - 1.0,
            _ => if phase < 0.5 { 1.0 } else { -1.0 },
        };
        let s = (v * vol) as f32;
        a.samples[k * 2] = s;
        a.samples[k * 2 + 1] = s;
    }
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-rs.rusty_chiptune", name: "Rusty Chiptune", category: "音声", flags: THREAD_SAFE,
            params: [
                float(c"volume", c"音量", 0.5, 0.0, 1.0),
                choice(c"wave_type", c"音源", 0, c"矩形波|三角波|のこぎり波|正弦波|ノイズ"),
                choice(c"freq_mode", c"周波数モード", 0, c"MIDIノート|周波数（Hz）"),
                float(c"midi_note", c"MIDIノート", 60.0, 0.0, 127.0),
                float(c"frequency", c"周波数（Hz）", 440.0, 20.0, 20000.0),
            ],
            audio: process,
        },
    ]
}

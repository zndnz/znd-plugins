//! 音声解析の道具（FFT など）。`Frame::audio` で取った他のレイヤーの音に使う。

/// 実数列の FFT の振幅（長さ n/2）。n は 2 のべき乗に切り詰める。窓はハン窓。
pub fn spectrum(samples: &[f32]) -> Vec<f32> {
    let mut n = 1;
    while n * 2 <= samples.len() {
        n *= 2;
    }
    if n < 2 {
        return Vec::new();
    }
    let mut re: Vec<f32> = (0..n)
        .map(|i| {
            let w = 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / (n - 1) as f32).cos();
            samples[i] * w
        })
        .collect();
    let mut im = vec![0.0f32; n];
    // ビット反転の並べ替え
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let ang = -std::f32::consts::TAU / len as f32;
        for start in (0..n).step_by(len) {
            for k in 0..len / 2 {
                let (s, c) = (ang * k as f32).sin_cos();
                let (a, b) = (start + k, start + k + len / 2);
                let (xr, xi) = (re[b] * c - im[b] * s, re[b] * s + im[b] * c);
                re[b] = re[a] - xr;
                im[b] = im[a] - xi;
                re[a] += xr;
                im[a] += xi;
            }
        }
        len <<= 1;
    }
    (0..n / 2).map(|k| (re[k] * re[k] + im[k] * im[k]).sqrt() * 2.0 / n as f32).collect()
}

/// `spectrum` の結果のうち、lo_hz〜hi_hz の振幅の平均（sample_rate は元の音のサンプリング周波数）。
pub fn band(spec: &[f32], sample_rate: f32, lo_hz: f32, hi_hz: f32) -> f32 {
    if spec.is_empty() {
        return 0.0;
    }
    let bin = sample_rate / (spec.len() * 2) as f32;
    let a = ((lo_hz / bin).floor() as usize).min(spec.len() - 1);
    let b = ((hi_hz / bin).ceil() as usize).clamp(a + 1, spec.len());
    spec[a..b].iter().sum::<f32>() / (b - a) as f32
}

/// 実効値（RMS）。
pub fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    (samples.iter().map(|v| v * v).sum::<f32>() / samples.len() as f32).sqrt()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sine_peak_is_at_its_frequency() {
        let sr = 48000.0;
        let s: Vec<f32> = (0..2048).map(|i| (i as f32 / sr * 1000.0 * std::f32::consts::TAU).sin()).collect();
        let spec = super::spectrum(&s);
        let peak = spec.iter().enumerate().fold((0, 0.0f32), |b, (i, v)| if *v > b.1 { (i, *v) } else { b }).0;
        let hz = peak as f32 * sr / 2048.0;
        assert!((hz - 1000.0).abs() < 30.0, "{hz}");
        assert!(super::band(&spec, sr, 900.0, 1100.0) > super::band(&spec, sr, 5000.0, 6000.0) * 10.0);
    }
}

/// 双 2 次フィルタの種類（RBJ の Audio EQ Cookbook）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BiquadKind {
    LowPass,
    HighPass,
    BandPass,
    Notch,
    /// ピーキング（ゲインは dB）。
    Peak(f64),
    /// ローシェルフ（ゲインは dB）。
    LowShelf(f64),
    /// ハイシェルフ（ゲインは dB）。
    HighShelf(f64),
}

/// 双 2 次フィルタ（直接形 I）。状態を持つので、チャンネルごとに 1 つ使う。
#[derive(Clone, Debug)]
pub struct Biquad {
    b: [f64; 3],
    a: [f64; 2],
    x: [f64; 2],
    y: [f64; 2],
}

impl Biquad {
    /// `freq` は Hz、`q` は Q 値。
    pub fn new(kind: BiquadKind, sample_rate: f64, freq: f64, q: f64) -> Self {
        let mut f = Biquad { b: [1.0, 0.0, 0.0], a: [0.0, 0.0], x: [0.0; 2], y: [0.0; 2] };
        f.set(kind, sample_rate, freq, q);
        f
    }

    /// 係数だけを変える（状態はそのまま）。
    pub fn set(&mut self, kind: BiquadKind, sample_rate: f64, freq: f64, q: f64) {
        let w0 = 2.0 * std::f64::consts::PI * (freq / sample_rate).clamp(1e-6, 0.4999);
        let (s, c) = w0.sin_cos();
        let alpha = s / (2.0 * q.max(1e-6));
        let (b0, b1, b2, a0, a1, a2) = match kind {
            BiquadKind::LowPass => ((1.0 - c) / 2.0, 1.0 - c, (1.0 - c) / 2.0, 1.0 + alpha, -2.0 * c, 1.0 - alpha),
            BiquadKind::HighPass => ((1.0 + c) / 2.0, -(1.0 + c), (1.0 + c) / 2.0, 1.0 + alpha, -2.0 * c, 1.0 - alpha),
            BiquadKind::BandPass => (alpha, 0.0, -alpha, 1.0 + alpha, -2.0 * c, 1.0 - alpha),
            BiquadKind::Notch => (1.0, -2.0 * c, 1.0, 1.0 + alpha, -2.0 * c, 1.0 - alpha),
            BiquadKind::Peak(g) => {
                let a = 10f64.powf(g / 40.0);
                (1.0 + alpha * a, -2.0 * c, 1.0 - alpha * a, 1.0 + alpha / a, -2.0 * c, 1.0 - alpha / a)
            }
            BiquadKind::LowShelf(g) => {
                let a = 10f64.powf(g / 40.0);
                let k = 2.0 * a.sqrt() * alpha;
                (
                    a * ((a + 1.0) - (a - 1.0) * c + k),
                    2.0 * a * ((a - 1.0) - (a + 1.0) * c),
                    a * ((a + 1.0) - (a - 1.0) * c - k),
                    (a + 1.0) + (a - 1.0) * c + k,
                    -2.0 * ((a - 1.0) + (a + 1.0) * c),
                    (a + 1.0) + (a - 1.0) * c - k,
                )
            }
            BiquadKind::HighShelf(g) => {
                let a = 10f64.powf(g / 40.0);
                let k = 2.0 * a.sqrt() * alpha;
                (
                    a * ((a + 1.0) + (a - 1.0) * c + k),
                    -2.0 * a * ((a - 1.0) + (a + 1.0) * c),
                    a * ((a + 1.0) + (a - 1.0) * c - k),
                    (a + 1.0) - (a - 1.0) * c + k,
                    2.0 * ((a - 1.0) - (a + 1.0) * c),
                    (a + 1.0) - (a - 1.0) * c - k,
                )
            }
        };
        self.b = [b0 / a0, b1 / a0, b2 / a0];
        self.a = [a1 / a0, a2 / a0];
    }

    pub fn run(&mut self, x: f64) -> f64 {
        let y = self.b[0] * x + self.b[1] * self.x[0] + self.b[2] * self.x[1] - self.a[0] * self.y[0] - self.a[1] * self.y[1];
        self.x = [x, self.x[0]];
        self.y = [y, self.y[0]];
        y
    }

    pub fn reset(&mut self) {
        self.x = [0.0; 2];
        self.y = [0.0; 2];
    }
}

#[cfg(test)]
mod biquad_tests {
    use super::*;
    #[test]
    fn lowpass_passes_dc_and_cuts_high() {
        let mut f = Biquad::new(BiquadKind::LowPass, 48000.0, 1000.0, std::f64::consts::FRAC_1_SQRT_2);
        let mut y = 0.0;
        for _ in 0..4800 {
            y = f.run(1.0);
        }
        assert!((y - 1.0).abs() < 1e-6);
        let mut f = Biquad::new(BiquadKind::LowPass, 48000.0, 1000.0, std::f64::consts::FRAC_1_SQRT_2);
        let mut peak: f64 = 0.0;
        for i in 0..4800 {
            let v = f.run(if i % 2 == 0 { 1.0 } else { -1.0 });
            if i > 100 {
                peak = peak.max(v.abs());
            }
        }
        assert!(peak < 0.01, "{peak}");
    }
}

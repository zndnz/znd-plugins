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

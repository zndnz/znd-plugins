//! 所有する RGBA 画像（リニアライト・プリマルチプライドの f32）と、よく使う処理。

use crate::ffi;

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub w: usize,
    pub h: usize,
    pub data: Vec<f32>,
}

impl Image {
    /// 透明な画像。
    pub fn new(w: usize, h: usize) -> Self {
        Image { w, h, data: vec![0.0; w * h * 4] }
    }

    /// 1 色で塗った画像（`c` はプリマルチプライド）。
    pub fn filled(w: usize, h: usize, c: [f32; 4]) -> Self {
        Image { w, h, data: c.repeat(w * h) }
    }

    /// # Safety
    /// `img.rgba` は `width*height*4` 要素の有効な領域であること。
    pub unsafe fn from_raw(img: &ffi::Image) -> Self {
        let n = img.width as usize * img.height as usize * 4;
        Image { w: img.width as usize, h: img.height as usize, data: std::slice::from_raw_parts(img.rgba, n).to_vec() }
    }

    pub fn from_slice(w: usize, h: usize, data: &[f32]) -> Self {
        Image { w, h, data: data.to_vec() }
    }

    #[inline]
    pub fn idx(&self, x: usize, y: usize) -> usize {
        (y * self.w + x) * 4
    }

    #[inline]
    pub fn px(&self, x: usize, y: usize) -> [f32; 4] {
        let i = self.idx(x, y);
        [self.data[i], self.data[i + 1], self.data[i + 2], self.data[i + 3]]
    }

    /// 範囲外は透明。
    #[inline]
    pub fn get(&self, x: isize, y: isize) -> [f32; 4] {
        if x < 0 || y < 0 || x >= self.w as isize || y >= self.h as isize {
            [0.0; 4]
        } else {
            self.px(x as usize, y as usize)
        }
    }

    /// 範囲外は端の画素。
    #[inline]
    pub fn get_clamped(&self, x: isize, y: isize) -> [f32; 4] {
        if self.w == 0 || self.h == 0 {
            return [0.0; 4];
        }
        self.px(x.clamp(0, self.w as isize - 1) as usize, y.clamp(0, self.h as isize - 1) as usize)
    }

    /// 範囲外は折り返し（タイル）。
    #[inline]
    pub fn get_wrapped(&self, x: isize, y: isize) -> [f32; 4] {
        if self.w == 0 || self.h == 0 {
            return [0.0; 4];
        }
        self.px(x.rem_euclid(self.w as isize) as usize, y.rem_euclid(self.h as isize) as usize)
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, p: [f32; 4]) {
        let i = self.idx(x, y);
        self.data[i..i + 4].copy_from_slice(&p);
    }

    /// 範囲外は無視して置く。
    #[inline]
    pub fn put(&mut self, x: isize, y: isize, p: [f32; 4]) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            self.set(x as usize, y as usize, p);
        }
    }

    /// プリマルチプライドの「上に重ねる」。
    #[inline]
    pub fn blend(&mut self, x: isize, y: isize, p: [f32; 4]) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            let i = self.idx(x as usize, y as usize);
            let k = 1.0 - p[3];
            for c in 0..4 {
                self.data[i + c] = p[c] + self.data[i + c] * k;
            }
        }
    }

    /// バイリニア補間。画素の中心は +0.5。範囲外は透明。
    pub fn sample(&self, x: f32, y: f32) -> [f32; 4] {
        self.sample_with(x, y, Image::get)
    }

    /// バイリニア補間。範囲外は端の画素。
    pub fn sample_clamped(&self, x: f32, y: f32) -> [f32; 4] {
        self.sample_with(x, y, Image::get_clamped)
    }

    /// バイリニア補間。範囲外は折り返し。
    pub fn sample_wrapped(&self, x: f32, y: f32) -> [f32; 4] {
        self.sample_with(x, y, Image::get_wrapped)
    }

    fn sample_with(&self, x: f32, y: f32, at: fn(&Image, isize, isize) -> [f32; 4]) -> [f32; 4] {
        if !x.is_finite() || !y.is_finite() {
            return [0.0; 4];
        }
        let (u, v) = (x - 0.5, y - 0.5);
        let (x0, y0) = (u.floor(), v.floor());
        let (fx, fy) = (u - x0, v - y0);
        let (x0, y0) = (x0 as isize, y0 as isize);
        let a = at(self, x0, y0);
        let b = at(self, x0 + 1, y0);
        let c = at(self, x0, y0 + 1);
        let d = at(self, x0 + 1, y0 + 1);
        let mut o = [0.0; 4];
        for k in 0..4 {
            let top = a[k] + (b[k] - a[k]) * fx;
            let bot = c[k] + (d[k] - c[k]) * fx;
            o[k] = top + (bot - top) * fy;
        }
        o
    }

    /// 各画素を (x, y, 値) から計算し直す。
    pub fn map(&mut self, mut f: impl FnMut(usize, usize, [f32; 4]) -> [f32; 4]) {
        let w = self.w;
        for (i, p) in self.data.chunks_exact_mut(4).enumerate() {
            let o = f(i % w, i / w, [p[0], p[1], p[2], p[3]]);
            p.copy_from_slice(&o);
        }
    }

    /// (x, y) から新しい画像を作る。
    pub fn generate(w: usize, h: usize, mut f: impl FnMut(usize, usize) -> [f32; 4]) -> Image {
        let mut img = Image::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let p = f(x, y);
                img.set(x, y, p);
            }
        }
        img
    }

    /// 四方に余白を足した画像（左, 上, 右, 下）。
    pub fn padded(&self, l: usize, t: usize, r: usize, b: usize) -> Image {
        let mut out = Image::new(self.w + l + r, self.h + t + b);
        for y in 0..self.h {
            let s = &self.data[y * self.w * 4..(y + 1) * self.w * 4];
            let d = out.idx(l, y + t);
            out.data[d..d + s.len()].copy_from_slice(s);
        }
        out
    }

    /// 矩形を切り出す（範囲外は透明）。
    pub fn crop(&self, x0: isize, y0: isize, w: usize, h: usize) -> Image {
        let mut out = Image::new(w, h);
        for y in 0..h {
            for x in 0..w {
                out.set(x, y, self.get(x0 + x as isize, y0 + y as isize));
            }
        }
        out
    }

    /// `other` を (dx, dy) に重ねる（プリマルチプライドの over）。`opacity` を掛ける。
    pub fn draw(&mut self, other: &Image, dx: isize, dy: isize, opacity: f32) {
        for y in 0..other.h {
            for x in 0..other.w {
                let mut p = other.px(x, y);
                if p[3] <= 0.0 && p[0] == 0.0 && p[1] == 0.0 && p[2] == 0.0 {
                    continue;
                }
                p.iter_mut().for_each(|v| *v *= opacity);
                self.blend(dx + x as isize, dy + y as isize, p);
            }
        }
    }

    /// 不透明な画素を含む最小の矩形（x0, y0, x1, y1。x1, y1 は含まない）。全部透明なら None。
    pub fn opaque_bounds(&self, threshold: f32) -> Option<(usize, usize, usize, usize)> {
        let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
        for y in 0..self.h {
            for x in 0..self.w {
                if self.data[self.idx(x, y) + 3] > threshold {
                    x0 = x0.min(x);
                    y0 = y0.min(y);
                    x1 = x1.max(x + 1);
                    y1 = y1.max(y + 1);
                }
            }
        }
        (x0 != usize::MAX).then_some((x0, y0, x1, y1))
    }

    /// アルファだけの配列。
    pub fn alpha(&self) -> Vec<f32> {
        self.data.chunks_exact(4).map(|p| p[3]).collect()
    }

    /// 縦横の分離可能なガウスぼかし（sigma は画素）。範囲外は透明として扱う。
    pub fn gaussian_blur(&mut self, sigma: f32) {
        if sigma <= 0.05 || self.w == 0 || self.h == 0 {
            return;
        }
        let r = (sigma * 3.0).ceil() as isize;
        let kernel: Vec<f32> = (-r..=r).map(|i| (-(i * i) as f32 / (2.0 * sigma * sigma)).exp()).collect();
        let sum: f32 = kernel.iter().sum();
        let kernel: Vec<f32> = kernel.iter().map(|k| k / sum).collect();
        self.convolve_separable(&kernel, &kernel);
    }

    /// 方向ごとに違う核で畳み込む（範囲外は透明）。
    pub fn convolve_separable(&mut self, kx: &[f32], ky: &[f32]) {
        let (w, h) = (self.w, self.h);
        let rx = (kx.len() / 2) as isize;
        let ry = (ky.len() / 2) as isize;
        let mut tmp = vec![0.0f32; self.data.len()];
        for y in 0..h {
            for x in 0..w {
                let mut acc = [0.0f32; 4];
                for (k, wgt) in kx.iter().enumerate() {
                    let sx = x as isize + k as isize - rx;
                    if sx < 0 || sx >= w as isize {
                        continue;
                    }
                    let i = (y * w + sx as usize) * 4;
                    for c in 0..4 {
                        acc[c] += self.data[i + c] * wgt;
                    }
                }
                tmp[(y * w + x) * 4..(y * w + x) * 4 + 4].copy_from_slice(&acc);
            }
        }
        for y in 0..h {
            for x in 0..w {
                let mut acc = [0.0f32; 4];
                for (k, wgt) in ky.iter().enumerate() {
                    let sy = y as isize + k as isize - ry;
                    if sy < 0 || sy >= h as isize {
                        continue;
                    }
                    let i = (sy as usize * w + x) * 4;
                    for c in 0..4 {
                        acc[c] += tmp[i + c] * wgt;
                    }
                }
                self.data[(y * w + x) * 4..(y * w + x) * 4 + 4].copy_from_slice(&acc);
            }
        }
    }

    /// 箱型ぼかし（半径 r 画素、範囲外は透明）。
    pub fn box_blur(&mut self, r: usize) {
        if r == 0 {
            return;
        }
        let k = vec![1.0 / (2 * r + 1) as f32; 2 * r + 1];
        self.convolve_separable(&k, &k);
    }
}

/// アルファを半径 `r` の円で膨らませる（最大値フィルター）。
pub fn dilate_alpha(alpha: &[f32], w: usize, h: usize, r: f32) -> Vec<f32> {
    morph_alpha(alpha, w, h, r, true)
}

/// アルファを半径 `r` の円で削る（最小値フィルター）。
pub fn erode_alpha(alpha: &[f32], w: usize, h: usize, r: f32) -> Vec<f32> {
    morph_alpha(alpha, w, h, r, false)
}

fn morph_alpha(alpha: &[f32], w: usize, h: usize, r: f32, max: bool) -> Vec<f32> {
    if r <= 0.0 {
        return alpha.to_vec();
    }
    // 距離場を作って閾値で切る方が速いが、ここでは正確さ優先で行ごとの円盤走査（端は線形に滑らかにする）
    let ri = r.ceil() as isize;
    let spans: Vec<(isize, f32)> = (-ri..=ri).map(|dy| (dy, (r * r - (dy * dy) as f32).max(0.0).sqrt())).collect();
    let mut out = vec![if max { 0.0 } else { 1.0 }; w * h];
    // 行ごとに 1 次元の最大値を幅ごとに作るのは複雑なので、素直に回す（半径が大きいときは遅い）
    for y in 0..h as isize {
        for x in 0..w as isize {
            let mut m = if max { 0.0f32 } else { 1.0f32 };
            'outer: for &(dy, half) in &spans {
                let sy = y + dy;
                let hw = half.floor() as isize;
                for dx in -hw..=hw {
                    let sx = x + dx;
                    let v = if sx < 0 || sy < 0 || sx >= w as isize || sy >= h as isize { 0.0 } else { alpha[(sy as usize) * w + sx as usize] };
                    if max {
                        if v > m {
                            m = v;
                            if m >= 1.0 {
                                break 'outer;
                            }
                        }
                    } else if v < m {
                        m = v;
                        if m <= 0.0 {
                            break 'outer;
                        }
                    }
                }
            }
            out[y as usize * w + x as usize] = m;
        }
    }
    out
}

/// 2 値化したアルファからの距離（画素）。内側は 0。`inside` が真の画素を種にする。
pub fn distance_field(inside: &[bool], w: usize, h: usize) -> Vec<f32> {
    // Felzenszwalb の 2 乗距離変換（1 次元を縦横に）
    const INF: f32 = 1e20;
    let mut f: Vec<f32> = inside.iter().map(|&b| if b { 0.0 } else { INF }).collect();
    let n = w.max(h);
    let mut d = vec![0.0f32; n];
    let mut v = vec![0usize; n];
    let mut z = vec![0.0f32; n + 1];
    let mut line = vec![0.0f32; n];
    let mut pass = |f: &mut Vec<f32>, len: usize, stride: usize, count: usize, step: usize| {
        for c in 0..count {
            for i in 0..len {
                line[i] = f[c * step + i * stride];
            }
            dt1(&line[..len], &mut d[..len], &mut v, &mut z);
            for i in 0..len {
                f[c * step + i * stride] = d[i];
            }
        }
    };
    pass(&mut f, h, w, w, 1);
    pass(&mut f, w, 1, h, w);
    f.iter().map(|v| v.sqrt()).collect()
}

fn dt1(f: &[f32], d: &mut [f32], v: &mut [usize], z: &mut [f32]) {
    let n = f.len();
    if n == 0 {
        return;
    }
    let mut k = 0usize;
    v[0] = 0;
    z[0] = f32::NEG_INFINITY;
    z[1] = f32::INFINITY;
    for q in 1..n {
        let mut s;
        loop {
            let p = v[k];
            s = ((f[q] + (q * q) as f32) - (f[p] + (p * p) as f32)) / (2.0 * q as f32 - 2.0 * p as f32);
            // z[0] は -∞ なので k が 0 を下回ることはない
            if s <= z[k] {
                k -= 1;
            } else {
                break;
            }
        }
        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = f32::INFINITY;
    }
    k = 0;
    for (q, out) in d.iter_mut().enumerate().take(n) {
        while z[k + 1] < q as f32 {
            k += 1;
        }
        let p = v[k];
        *out = (q as f32 - p as f32).powi(2) + f[p];
    }
}

//! 乱数とノイズ。AviUtl の `obj.rand(min, max, seed, frame)` に相当する決定的な乱数も用意する。

#[inline]
pub fn hash(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x
}

#[inline]
pub fn hash3(a: u32, b: u32, c: u32) -> u32 {
    hash(a.wrapping_mul(0x9e37_79b1) ^ hash(b.wrapping_mul(0x85eb_ca6b) ^ hash(c)))
}

/// 0..1 の一様乱数。
#[inline]
pub fn rand01(a: u32, b: u32, c: u32) -> f32 {
    (hash3(a, b, c) >> 8) as f32 / (1u32 << 24) as f32
}

/// `obj.rand(min, max, seed, frame)` 相当: min 以上 max 以下の整数。
pub fn rand_int(min: i64, max: i64, seed: i64, frame: i64) -> i64 {
    let (lo, hi) = if min <= max { (min, max) } else { (max, min) };
    let span = (hi - lo + 1) as u64;
    let r = hash3(seed as u32, (seed >> 32) as u32 ^ 0x5bd1_e995, frame as u32) as u64;
    lo + (r % span.max(1)) as i64
}

/// 決定的な乱数列（xorshift64*）。
#[derive(Clone)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    /// 0..1
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.f64()
    }
    /// lo 以上 hi 以下の整数。
    pub fn int(&mut self, lo: i64, hi: i64) -> i64 {
        if hi <= lo {
            return lo;
        }
        lo + (self.next_u64() % (hi - lo + 1) as u64) as i64
    }
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// 3D の値ノイズ（0..1）。
pub fn value3(x: f32, y: f32, z: f32, seed: u32) -> f32 {
    let (xi, yi, zi) = (x.floor(), y.floor(), z.floor());
    let (fx, fy, fz) = (fade(x - xi), fade(y - yi), fade(z - zi));
    let (xi, yi, zi) = (xi as i32, yi as i32, zi as i32);
    let v = |dx: i32, dy: i32, dz: i32| rand01((xi + dx) as u32 ^ seed.wrapping_mul(0x27d4_eb2f), (yi + dy) as u32, (zi + dz) as u32);
    let l = |a: f32, b: f32, t: f32| a + (b - a) * t;
    let x00 = l(v(0, 0, 0), v(1, 0, 0), fx);
    let x10 = l(v(0, 1, 0), v(1, 1, 0), fx);
    let x01 = l(v(0, 0, 1), v(1, 0, 1), fx);
    let x11 = l(v(0, 1, 1), v(1, 1, 1), fx);
    l(l(x00, x10, fy), l(x01, x11, fy), fz)
}

fn grad(h: u32, x: f32, y: f32, z: f32) -> f32 {
    match h & 15 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        3 => -x - y,
        4 => x + z,
        5 => -x + z,
        6 => x - z,
        7 => -x - z,
        8 => y + z,
        9 => -y + z,
        10 => y - z,
        11 => -y - z,
        12 => x + y,
        13 => -x + y,
        14 => -y + z,
        _ => -y - z,
    }
}

/// 3D のパーリンノイズ（おおむね -1..1）。
pub fn perlin3(x: f32, y: f32, z: f32, seed: u32) -> f32 {
    let (xi, yi, zi) = (x.floor(), y.floor(), z.floor());
    let (xf, yf, zf) = (x - xi, y - yi, z - zi);
    let (u, v, w) = (fade(xf), fade(yf), fade(zf));
    let (xi, yi, zi) = (xi as i32, yi as i32, zi as i32);
    let h = |dx: i32, dy: i32, dz: i32| hash3((xi + dx) as u32 ^ seed, (yi + dy) as u32, (zi + dz) as u32);
    let l = |a: f32, b: f32, t: f32| a + (b - a) * t;
    let x1 = l(grad(h(0, 0, 0), xf, yf, zf), grad(h(1, 0, 0), xf - 1.0, yf, zf), u);
    let x2 = l(grad(h(0, 1, 0), xf, yf - 1.0, zf), grad(h(1, 1, 0), xf - 1.0, yf - 1.0, zf), u);
    let y1 = l(x1, x2, v);
    let x1 = l(grad(h(0, 0, 1), xf, yf, zf - 1.0), grad(h(1, 0, 1), xf - 1.0, yf, zf - 1.0), u);
    let x2 = l(grad(h(0, 1, 1), xf, yf - 1.0, zf - 1.0), grad(h(1, 1, 1), xf - 1.0, yf - 1.0, zf - 1.0), u);
    let y2 = l(x1, x2, v);
    l(y1, y2, w)
}

/// フラクタルノイズ（パーリンを重ねる。おおむね -1..1）。
pub fn fbm3(x: f32, y: f32, z: f32, octaves: u32, seed: u32) -> f32 {
    let (mut sum, mut amp, mut norm, mut f) = (0.0, 1.0, 0.0, 1.0);
    for o in 0..octaves.max(1) {
        sum += perlin3(x * f, y * f, z * f + o as f32 * 13.7, seed.wrapping_add(o)) * amp;
        norm += amp;
        amp *= 0.5;
        f *= 2.0;
    }
    sum / norm
}

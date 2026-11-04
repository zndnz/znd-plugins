//! 小さな数学の道具。

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

pub fn lerpf(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    if e1 <= e0 {
        return if x >= e0 { 1.0 } else { 0.0 };
    }
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// 点 (x, y) を原点まわりに `deg` 度回す（画面座標: y 下向きなので時計回り）。
pub fn rotate(x: f64, y: f64, deg: f64) -> (f64, f64) {
    let (s, c) = deg.to_radians().sin_cos();
    (x * c - y * s, x * s + y * c)
}

/// 2D の点。
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    pub const fn new(x: f64, y: f64) -> Self {
        V2 { x, y }
    }
    pub fn len(self) -> f64 {
        self.x.hypot(self.y)
    }
    pub fn add(self, o: V2) -> V2 {
        V2::new(self.x + o.x, self.y + o.y)
    }
    pub fn sub(self, o: V2) -> V2 {
        V2::new(self.x - o.x, self.y - o.y)
    }
    pub fn mul(self, k: f64) -> V2 {
        V2::new(self.x * k, self.y * k)
    }
    pub fn dot(self, o: V2) -> f64 {
        self.x * o.x + self.y * o.y
    }
    pub fn cross(self, o: V2) -> f64 {
        self.x * o.y - self.y * o.x
    }
    pub fn norm(self) -> V2 {
        let l = self.len();
        if l <= 1e-12 {
            V2::new(0.0, 0.0)
        } else {
            self.mul(1.0 / l)
        }
    }
    pub fn rot(self, deg: f64) -> V2 {
        let (x, y) = rotate(self.x, self.y, deg);
        V2::new(x, y)
    }
    pub fn lerp(self, o: V2, t: f64) -> V2 {
        V2::new(lerp(self.x, o.x, t), lerp(self.y, o.y, t))
    }
}

/// 2D アフィン変換 [a c e; b d f]。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine(pub [f64; 6]);

impl Affine {
    pub const IDENTITY: Affine = Affine([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    pub fn translate(x: f64, y: f64) -> Affine {
        Affine([1.0, 0.0, 0.0, 1.0, x, y])
    }
    pub fn scale(sx: f64, sy: f64) -> Affine {
        Affine([sx, 0.0, 0.0, sy, 0.0, 0.0])
    }
    pub fn rotate(deg: f64) -> Affine {
        let (s, c) = deg.to_radians().sin_cos();
        Affine([c, s, -s, c, 0.0, 0.0])
    }
    /// self を適用した後に `o` を適用する変換。
    pub fn then(self, o: Affine) -> Affine {
        let [a, b, c, d, e, f] = self.0;
        let [a2, b2, c2, d2, e2, f2] = o.0;
        Affine([a2 * a + c2 * b, b2 * a + d2 * b, a2 * c + c2 * d, b2 * c + d2 * d, a2 * e + c2 * f + e2, b2 * e + d2 * f + f2])
    }
    pub fn apply(self, p: V2) -> V2 {
        let [a, b, c, d, e, f] = self.0;
        V2::new(a * p.x + c * p.y + e, b * p.x + d * p.y + f)
    }
    pub fn invert(self) -> Option<Affine> {
        let [a, b, c, d, e, f] = self.0;
        let det = a * d - b * c;
        if det.abs() < 1e-12 {
            return None;
        }
        let (ia, ib, ic, id) = (d / det, -b / det, -c / det, a / det);
        Some(Affine([ia, ib, ic, id, -(ia * e + ic * f), -(ib * e + id * f)]))
    }
}

/// イージング（t: 0〜1）。AviUtl の「イージング（通常）」番号に対応するものを並べてある。
pub fn ease(kind: u32, t: f64) -> f64 {
    use std::f64::consts::PI;
    let t = t.clamp(0.0, 1.0);
    let bounce_out = |t: f64| {
        let (n1, d1) = (7.5625, 2.75);
        if t < 1.0 / d1 {
            n1 * t * t
        } else if t < 2.0 / d1 {
            let t = t - 1.5 / d1;
            n1 * t * t + 0.75
        } else if t < 2.5 / d1 {
            let t = t - 2.25 / d1;
            n1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / d1;
            n1 * t * t + 0.984375
        }
    };
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    let c2 = c1 * 1.525;
    match kind {
        0 | 1 => t,
        2 => 1.0 - (t * PI / 2.0).cos(),
        3 => (t * PI / 2.0).sin(),
        4 => -((PI * t).cos() - 1.0) / 2.0,
        5 => t * t,
        6 => 1.0 - (1.0 - t) * (1.0 - t),
        7 => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            }
        }
        8 => t * t * t,
        9 => 1.0 - (1.0 - t).powi(3),
        10 => {
            if t < 0.5 {
                4.0 * t * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
            }
        }
        11 => t.powi(4),
        12 => 1.0 - (1.0 - t).powi(4),
        13 => {
            if t < 0.5 {
                8.0 * t.powi(4)
            } else {
                1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
            }
        }
        14 => t.powi(5),
        15 => 1.0 - (1.0 - t).powi(5),
        16 => {
            if t < 0.5 {
                16.0 * t.powi(5)
            } else {
                1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
            }
        }
        17 => {
            if t == 0.0 {
                0.0
            } else {
                2f64.powf(10.0 * t - 10.0)
            }
        }
        18 => {
            if t == 1.0 {
                1.0
            } else {
                1.0 - 2f64.powf(-10.0 * t)
            }
        }
        19 => {
            if t == 0.0 || t == 1.0 {
                t
            } else if t < 0.5 {
                2f64.powf(20.0 * t - 10.0) / 2.0
            } else {
                (2.0 - 2f64.powf(-20.0 * t + 10.0)) / 2.0
            }
        }
        20 => 1.0 - (1.0 - t * t).max(0.0).sqrt(),
        21 => (1.0 - (t - 1.0).powi(2)).max(0.0).sqrt(),
        22 => {
            if t < 0.5 {
                (1.0 - (1.0 - (2.0 * t).powi(2)).max(0.0).sqrt()) / 2.0
            } else {
                ((1.0 - (-2.0 * t + 2.0).powi(2)).max(0.0).sqrt() + 1.0) / 2.0
            }
        }
        23 => c3 * t * t * t - c1 * t * t,
        24 => 1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2),
        25 => {
            if t < 0.5 {
                ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
            } else {
                ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
            }
        }
        26 => {
            let c4 = 2.0 * PI / 3.0;
            if t == 0.0 || t == 1.0 {
                t
            } else {
                -(2f64.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * c4).sin()
            }
        }
        27 => {
            let c4 = 2.0 * PI / 3.0;
            if t == 0.0 || t == 1.0 {
                t
            } else {
                2f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
            }
        }
        28 => {
            let c5 = 2.0 * PI / 4.5;
            if t == 0.0 || t == 1.0 {
                t
            } else if t < 0.5 {
                -(2f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
            } else {
                (2f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
            }
        }
        29 => 1.0 - bounce_out(1.0 - t),
        30 => bounce_out(t),
        31 => {
            if t < 0.5 {
                (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
            } else {
                (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
            }
        }
        _ => t,
    }
}

/// イージングの名前（`ease` の番号順）。choice パラメータの選択肢に使う。
pub const EASE_NAMES: &str = "直線|直線|Sine In|Sine Out|Sine InOut|Quad In|Quad Out|Quad InOut|Cubic In|Cubic Out|Cubic InOut|Quart In|Quart Out|Quart InOut|Quint In|Quint Out|Quint InOut|Expo In|Expo Out|Expo InOut|Circ In|Circ Out|Circ InOut|Back In|Back Out|Back InOut|Elastic In|Elastic Out|Elastic InOut|Bounce In|Bounce Out|Bounce InOut";

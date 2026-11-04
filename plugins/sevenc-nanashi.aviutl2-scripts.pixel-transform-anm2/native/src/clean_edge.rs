//! cleanEdge（torcado、MIT）の補間。元のスクリプトの HLSL をそのまま写したもの。

pub type C = [f64; 4];
const NONE: C = [-1.0; 4];

pub struct Ce {
    pub highest: [f64; 3],
    pub thr: f64,
    pub line_width: f64,
    pub slope: bool,
    pub cleanup: bool,
}

fn dist(a: C, b: C) -> f64 {
    (0..4).map(|i| (a[i] - b[i]).powi(2)).sum::<f64>().sqrt()
}
fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f64>().sqrt()
}
type V = [f64; 2];
fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1]]
}
fn mulv(a: V, b: V) -> V {
    [a[0] * b[0], a[1] * b[1]]
}

impl Ce {
    fn sim(&self, a: C, b: C) -> bool {
        (a[3] == 0.0 && b[3] == 0.0) || dist(a, b) <= self.thr
    }
    fn sim3(&self, a: C, b: C, c: C) -> bool {
        self.sim(a, b) && self.sim(b, c)
    }
    fn sim4(&self, a: C, b: C, c: C, d: C) -> bool {
        self.sim(a, b) && self.sim(b, c) && self.sim(c, d)
    }
    fn higher(&self, t: C, o: C) -> bool {
        if self.sim(t, o) {
            return false;
        }
        if t[3] == o[3] {
            dist3([t[0], t[1], t[2]], self.highest) < dist3([o[0], o[1], o[2]], self.highest)
        } else {
            t[3] > o[3]
        }
    }
    fn dtl(&self, p: V, p1: V, p2: V, dir: V) -> f64 {
        let ld = [p2[0] - p1[0], p2[1] - p1[1]];
        let perp = [ld[1], -ld[0]];
        let to = [p1[0] - p[0], p1[1] - p[1]];
        let l = perp[0].hypot(perp[1]).max(1e-12);
        let s = if perp[0] * dir[0] + perp[1] * dir[1] > 0.0 { 1.0 } else { -1.0 };
        s * (perp[0] / l * to[0] + perp[1] / l * to[1])
    }

    #[allow(clippy::too_many_arguments)]
    pub fn slice(&self, base: V, main: V, pd: V, ub: C, u: C, uf: C, uff: C, b: C, c: C, f: C, ff: C, db: C, d: C, df: C, dff: C, ddb: C, dd: C, ddf: C) -> C {
        let lw = self.line_width;
        let bp = [main[0] * (base[0] - 0.5) + 0.5, main[1] * (base[1] - 0.5) + 0.5];
        let cd = dist;
        let against = 4.0 * cd(f, d) + cd(uf, c) + cd(c, db) + cd(ff, df) + cd(df, dd);
        let towards = 4.0 * cd(c, df) + cd(u, f) + cd(f, dff) + cd(b, d) + cd(d, ddf);
        let mut slice = against < towards || (against < towards + 0.001 && !self.higher(c, f));
        if self.sim4(f, d, b, u) && self.sim4(uf, df, db, ub) && !self.sim(c, f) {
            slice = false;
        }
        if !slice {
            return NONE;
        }
        let ctr = [0.5, 0.5];
        let pt = |x: f64, y: f64| add(ctr, mulv(pd, [x, y]));
        let neg = [-pd[0], -pd[1]];
        let pick = |dist: f64, a: C, b: C, base: C| if dist - lw / 2.0 <= 0.0 { if cd(base, a) <= cd(base, b) { a } else { b } } else { NONE };
        let mut flip = false;
        if self.slope {
            if self.sim3(f, d, db) && !self.sim3(f, d, b) && !self.sim(uf, db) {
                if !(self.sim(c, df) && self.higher(c, f)) {
                    if self.higher(c, f) {
                        flip = true;
                    }
                    if self.sim(u, f) && !self.sim(c, df) && !self.higher(c, u) {
                        flip = true;
                    }
                }
                let mut dd_ = if flip { lw - self.dtl(bp, pt(1.5, -1.0), pt(-0.5, 0.0), neg) } else { self.dtl(bp, pt(1.5, 0.0), pt(-0.5, 1.0), pd) };
                if self.cleanup && !flip && self.sim(c, uf) && !(self.sim3(c, uf, uff) && !self.sim3(c, uf, ff) && !self.sim(d, uff)) {
                    dd_ = dd_.min(self.dtl(bp, pt(2.0, -1.0), pt(0.0, 1.0), pd));
                }
                return pick(dd_, f, d, c);
            } else if self.sim3(uf, f, d) && !self.sim3(u, f, d) && !self.sim(uf, db) {
                if !(self.sim(c, df) && self.higher(c, d)) {
                    if self.higher(c, d) {
                        flip = true;
                    }
                    if self.sim(b, d) && !self.sim(c, df) && !self.higher(c, d) {
                        flip = true;
                    }
                }
                let mut dd_ = if flip { lw - self.dtl(bp, pt(0.0, -0.5), pt(-1.0, 1.5), neg) } else { self.dtl(bp, pt(1.0, -0.5), pt(0.0, 1.5), pd) };
                if self.cleanup && !flip && self.sim(c, db) && !(self.sim3(c, db, ddb) && !self.sim3(c, db, dd) && !self.sim(f, ddb)) {
                    dd_ = dd_.min(self.dtl(bp, pt(1.0, 0.0), pt(-1.0, 2.0), pd));
                }
                return pick(dd_, f, d, c);
            }
        }
        if self.sim(f, d) {
            if self.sim(c, df) && self.higher(c, f) {
                if !self.sim(c, dd) && !self.sim(c, ff) {
                    flip = true;
                }
            } else {
                if self.higher(c, f) {
                    flip = true;
                }
                if !self.sim(c, b) && self.sim4(b, f, d, u) {
                    flip = true;
                }
            }
            if ((self.sim(f, db) && self.sim3(u, f, df)) || (self.sim(uf, d) && self.sim3(b, d, df))) && !self.sim(c, df) {
                flip = true;
            }
            let mut dd_ = if flip { lw - self.dtl(bp, pt(1.0, -1.0), pt(-1.0, 1.0), neg) } else { self.dtl(bp, pt(1.0, 0.0), pt(0.0, 1.0), pd) };
            if self.slope && self.cleanup {
                if !flip && self.sim3(c, uf, uff) && !self.sim3(c, uf, ff) && !self.sim(d, uff) {
                    dd_ = dd_.max(self.dtl(bp, pt(1.5, 0.0), pt(-0.5, 1.0), pd));
                }
                if !flip && self.sim3(ddb, db, c) && !self.sim3(dd, db, c) && !self.sim(ddb, f) {
                    dd_ = dd_.max(self.dtl(bp, pt(1.0, -0.5), pt(0.0, 1.5), pd));
                }
            }
            return pick(dd_, f, d, c);
        }
        if self.slope {
            if self.sim3(ff, df, d) && !self.sim3(ff, df, c) && !self.sim(uff, d) {
                if !(self.sim(f, dff) && self.higher(f, ff)) {
                    if self.higher(f, ff) {
                        flip = true;
                    }
                    if self.sim(uf, ff) && !self.sim(f, dff) && !self.higher(f, uf) {
                        flip = true;
                    }
                }
                let dd_ = if flip { lw - self.dtl(bp, pt(2.5, -1.0), pt(0.5, 0.0), neg) } else { self.dtl(bp, pt(2.5, 0.0), pt(0.5, 1.0), pd) };
                return pick(dd_, ff, df, f);
            } else if self.sim3(f, df, dd) && !self.sim3(c, df, dd) && !self.sim(f, ddb) {
                if !(self.sim(d, ddf) && self.higher(d, dd)) {
                    if self.higher(d, dd) {
                        flip = true;
                    }
                    if self.sim(db, dd) && !self.sim(d, ddf) && !self.higher(d, dd) {
                        flip = true;
                    }
                }
                let dd_ = if flip { lw - self.dtl(bp, pt(0.0, 0.5), pt(-1.0, 2.5), neg) } else { self.dtl(bp, pt(1.0, 0.5), pt(0.0, 2.5), pd) };
                return pick(dd_, df, dd, d);
            }
        }
        NONE
    }
}

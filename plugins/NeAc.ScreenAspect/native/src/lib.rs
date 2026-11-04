//! 画面アスペクト（猫愛）の移植: 画面の中央に、指定した縦横比の長方形を描く（反転で外側を塗る）。
//! 適応度 100% で画面に内接する大きさ、0% で画面いっぱい。

use znd_plugin_sdk::*;

fn calc(max: f64, min: f64, per: f64) -> f64 {
    min + (max - min) * (1.0 - per)
}

/// (横, 縦) の大きさ（元のスクリプトと同じ計算）。
pub fn rect_size(sw: f64, sh: f64, ax: f64, ay: f64, per: f64) -> (f64, f64) {
    let (mut lr, mut ud);
    if sw <= sh {
        lr = sw;
        ud = calc(sh, lr / ax * ay, per);
        if ax < ay && ud > sh {
            ud = sh;
            lr = calc(sw, ud / ay * ax, per);
        } else if ax == ay {
            ud = lr;
        }
    } else {
        ud = sh;
        lr = calc(sw, ud / ay * ax, per);
        if ax > ay && lr > sw {
            lr = sw;
            ud = calc(sh, lr / ax * ay, per);
        } else if ax == ay {
            lr = ud;
        }
    }
    (lr, ud)
}

fn render(f: &mut Frame) -> FxResult {
    let (w, h) = (f.width as f64, f.height as f64);
    let (lr, ud) = rect_size(w, h, f.f(0).max(1.0), f.f(1).max(1.0), f.f(2) / 100.0);
    let invert = f.b(4);
    let fill = color::premul(f.color(3), 1.0);
    let (x0, x1, y0, y1) = ((w - lr) / 2.0, (w + lr) / 2.0, (h - ud) / 2.0, (h + ud) / 2.0);
    let wi = f.width;
    for (i, p) in f.rgba.chunks_exact_mut(4).enumerate() {
        let (x, y) = ((i % wi) as f64, (i / wi) as f64);
        // 画素と長方形の重なりの面積
        let cx = ((x + 1.0).min(x1) - x.max(x0)).max(0.0);
        let cy = ((y + 1.0).min(y1) - y.max(y0)).max(0.0);
        let mut cov = (cx * cy) as f32;
        if invert {
            cov = 1.0 - cov;
        }
        for k in 0..4 {
            p[k] = fill[k] * cov;
        }
    }
    Ok(())
}

plugin! {
    vendor: "猫愛（znd 移植: znd-plugins）", version: "r3", license: "MIT",
    effects: [
        {
            id: "NeAc.ScreenAspect", name: "画面アスペクト", category: "描画", flags: THREAD_SAFE,
            params: [
                float(c"asp_x", c"X比率", 4.0, 1.0, 100.0),
                float(c"asp_y", c"Y比率", 3.0, 1.0, 100.0),
                float(c"per", c"適応度（%）", 100.0, 0.0, 100.0),
                color(c"color", c"色", [1.0, 1.0, 1.0, 1.0]),
                boolean(c"invert", c"反転", false),
            ],
            render: render,
        },
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fits_4_3_in_16_9() {
        let (w, h) = super::rect_size(1920.0, 1080.0, 4.0, 3.0, 1.0);
        assert_eq!((w, h), (1440.0, 1080.0));
        let (w, h) = super::rect_size(1920.0, 1080.0, 4.0, 3.0, 0.0);
        assert_eq!((w, h), (1920.0, 1080.0));
    }
}

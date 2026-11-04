//! 余白除去 AutoClipping_S（σ軸）の移植。
//!
//! 不透明な画素を含む最小の矩形まで上下左右を切り詰め、そこから指定した余白を足す（負なら削る）。
//! 元のスクリプトはクリッピング・領域拡張のフィルタを繰り返して大きさを変えるが、
//! ここでは ABI 3 の出力の大きさの変更で 1 回で行う。絵の位置はそのまま保たれる。

use znd_plugin_sdk::*;

const THRESH: usize = 0;
const PAD_U: usize = 1;
const PAD_D: usize = 2;
const PAD_L: usize = 3;
const PAD_R: usize = 4;
const EN_U: usize = 5;
const EN_D: usize = 6;
const EN_L: usize = 7;
const EN_R: usize = 8;

fn render(f: &mut Frame) -> FxResult {
    let (w, h) = (f.width as i64, f.height as i64);
    let thresh = (f.f(THRESH) / 100.0).clamp(0.0, 1.0 - 2f64.powi(-24)) as f32;
    let mut pad = [PAD_U, PAD_D, PAD_L, PAD_R].map(|i| f.f(i).round() as i64);
    let enabled = [EN_U, EN_D, EN_L, EN_R].map(|i| f.b(i));
    if enabled.iter().any(|e| *e) {
        let img = f.input();
        match img.opaque_bounds(thresh) {
            Some((x0, y0, x1, y1)) => {
                let cut = [y0 as i64, h - y1 as i64, x0 as i64, w - x1 as i64];
                for k in 0..4 {
                    if enabled[k] {
                        pad[k] -= cut[k];
                    }
                }
            }
            None => {
                // すべて透明: 何も残らない
                f.output(Image::new(1, 1), 0, 0);
                return Ok(());
            }
        }
    }
    let [u, d, l, r] = pad;
    if u == 0 && d == 0 && l == 0 && r == 0 {
        return Ok(());
    }
    let (nw, nh) = (w + l + r, h + u + d);
    if nw <= 0 || nh <= 0 {
        f.output(Image::new(1, 1), 0, 0);
        return Ok(());
    }
    let img = f.input();
    let out = img.crop(-l as isize, -u as isize, nw as usize, nh as usize);
    f.output(out, -l as i32, -u as i32);
    Ok(())
}

plugin! {
    vendor: "σ軸（znd 移植: znd-plugins）", version: "1.20", license: "MIT",
    effects: [
        {
            id: "sigma-axis.AutoClipping_S", name: "余白除去（AutoClipping_S）", category: "クリッピング", flags: THREAD_SAFE,
            params: [
                float(c"thresh", c"αしきい値（%）", 0.0, 0.0, 100.0),
                pixels(c"pad_u", c"上余白", 0.0, -4000.0, 4000.0),
                pixels(c"pad_d", c"下余白", 0.0, -4000.0, 4000.0),
                pixels(c"pad_l", c"左余白", 0.0, -4000.0, 4000.0),
                pixels(c"pad_r", c"右余白", 0.0, -4000.0, 4000.0),
                boolean(c"enable_u", c"上除去", true),
                boolean(c"enable_d", c"下除去", true),
                boolean(c"enable_l", c"左除去", true),
                boolean(c"enable_r", c"右除去", true),
            ],
            render: render,
        },
    ]
}

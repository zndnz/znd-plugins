//! 縦動画ガイド（Nanashi.）の移植: ニコニコショートの画面で UI に隠れる部分を、色の付いた図形で示すガイド。
//! 端末ごとの UI の配置（px）は元のスクリプトの表と同じ。画面の高さ（下の帯を除く）をコンポの高さに合わせる。

use znd_plugin_sdk::draw::{fill, paint, round_corners};
use znd_plugin_sdk::math::V2;
use znd_plugin_sdk::*;

struct L {
    width: f64,
    height: f64,
    footer: f64,
    screen_r: f64,
    comment_py: f64,
    px: f64,
    comment_h: f64,
    tag_py: f64,
    tag_h: f64,
    like_py: f64,
    like: f64,
    author: f64,
    author_px: f64,
    author_py: f64,
    follow_w: f64,
    author_name: f64,
    title_h: f64,
    title_py: f64,
    fwd_w: f64,
    fwd_h: f64,
    fwd_py: f64,
    fwd_r: f64,
    tool: f64,
    tool_gap: f64,
    tool_py: f64,
    notch: Option<[f64; 5]>,
}

const IPHONE: L = L {
    width: 1179.0, height: 2556.0, footer: 273.0, screen_r: 165.0, comment_py: 47.0, px: 47.0, comment_h: 100.0, tag_py: 46.0, tag_h: 40.0,
    like_py: 50.0, like: 165.0, author: 97.0, author_px: 22.0, author_py: 36.0, follow_w: 280.0, author_name: 40.0, title_h: 35.0, title_py: 45.0,
    fwd_w: 581.0, fwd_h: 108.0, fwd_py: 36.0, fwd_r: 24.0, tool: 60.0, tool_gap: 90.0, tool_py: 237.0,
    notch: Some([1179.0 / 2.0 - 376.0 / 2.0, 34.0, 376.0, 110.0, 55.0]),
};
const IPAD: L = L {
    width: 1488.0, height: 2266.0, footer: 170.0, screen_r: 20.0, comment_py: 31.0, px: 31.0, comment_h: 66.0, tag_py: 31.0, tag_h: 24.0,
    like_py: 35.0, like: 110.0, author: 64.0, author_px: 17.0, author_py: 23.0, follow_w: 184.0, author_name: 25.0, title_h: 26.0, title_py: 31.0,
    fwd_w: 381.0, fwd_h: 72.0, fwd_py: 24.0, fwd_r: 18.0, tool: 40.0, tool_gap: 60.0, tool_py: 104.0, notch: None,
};

/// [x, y, 幅, 高さ, 角の半径]（端末の px）。
fn rects(l: &L, show_fwd: bool, show_tags: bool, name_w: f64) -> Vec<[f64; 5]> {
    let (tag_h, tag_py) = if show_tags { (l.tag_h, l.tag_py) } else { (0.0, 0.0) };
    let mut r = Vec::new();
    if let Some(n) = l.notch {
        r.push(n);
    }
    let base = l.height - l.footer;
    r.push([0.0, base, l.width, l.footer, 0.0]);
    let comment_y = base - l.comment_py - l.comment_h;
    r.push([l.px, comment_y, l.width - l.px * 2.0, l.comment_h, l.comment_h / 2.0]);
    let tag_y = comment_y - tag_py - tag_h;
    if show_tags {
        r.push([l.px, tag_y, l.width - l.px * 2.0, tag_h, tag_h / 2.0]);
    }
    r.push([l.width - l.px - l.like, tag_y - l.like_py - l.like, l.like, l.like, l.like / 2.0]);
    let title_y = tag_y - l.title_py - l.title_h;
    r.push([l.px, title_y, l.width - l.px * 2.0 - l.like - l.px, l.title_h, l.title_h / 2.0]);
    let author_y = title_y - l.author_py - l.author;
    r.push([l.px, author_y, l.author, l.author, l.author / 2.0]);
    r.push([l.px + l.author + l.author_px, author_y + l.author / 2.0 - l.author_name / 2.0, name_w, l.author_name, l.author_name / 2.0]);
    r.push([l.px + l.author + l.author_px + name_w + l.author_px, author_y, l.follow_w, l.author, l.author / 2.0]);
    if show_fwd {
        r.push([l.px, author_y - l.fwd_py - l.fwd_h, l.fwd_w, l.fwd_h, l.fwd_r]);
    }
    r.push([l.px, l.tool_py, l.tool, l.tool, l.tool / 2.0]);
    for i in 0..3 {
        r.push([l.width - l.px - l.tool - (l.tool + l.tool_gap) * i as f64, l.tool_py, l.tool, l.tool, l.tool / 2.0]);
    }
    r
}

fn rrect(x: f64, y: f64, w: f64, h: f64, r: f64, k: f64, o: V2) -> Vec<V2> {
    let p = [V2::new(x, y), V2::new(x + w, y), V2::new(x + w, y + h), V2::new(x, y + h)].map(|p| o.add(p.mul(k)));
    round_corners(&p, (r * k).min(w * k / 2.0).min(h * k / 2.0))
}

fn render(f: &mut Frame) -> FxResult {
    let l = if f.i(0) == 1 { &IPAD } else { &IPHONE };
    let col = f.color(1);
    let (show_fwd, show_tags) = (f.b(2), f.b(3));
    let name_w = f.f(4);
    let side = f.f(5);
    let (_, ch) = f.comp_size();
    let k = ch * f.scale() / (l.height - l.footer);
    let (w, h) = (((l.width + side * 2.0) * k).ceil().max(1.0) as usize, (l.height * k).ceil().max(1.0) as usize);
    let o = V2::new(side * k, 0.0);
    // 画面の外（左右の線）を塗り、画面の角丸の内側を抜く
    let screen = fill(w, h, &[rrect(0.0, 0.0, l.width, l.height, l.screen_r, k, o)]);
    let mut img = Image::new(w, h);
    let c = color::premul(col, 1.0);
    for (p, s) in img.data.chunks_exact_mut(4).zip(&screen) {
        let a = 1.0 - s;
        for i in 0..4 {
            p[i] = c[i] * a;
        }
    }
    let shapes: Vec<Vec<V2>> = rects(l, show_fwd, show_tags, name_w).iter().map(|r| rrect(r[0], r[1], r[2], r[3], r[4], k, o)).collect();
    let cov = draw::fill_union(w, h, &shapes.into_iter().map(|s| vec![s]).collect::<Vec<_>>());
    paint(&mut img, &cov, [col[0], col[1], col[2], 1.0]);
    // 下の帯の分だけ下にずらして、画面の部分がコンポの高さに合うように置く
    let ox = (f.width as f64 / 2.0 - w as f64 / 2.0).round() as i32;
    let oy = (f.height as f64 / 2.0 - h as f64 / 2.0 + l.footer * k / 2.0).round() as i32;
    f.output(img, ox, oy);
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-scripts.vertical-video-obj2", name: "縦動画ガイド", category: "描画", flags: THREAD_SAFE,
            params: [
                choice(c"device", c"デバイス", 0, c"iPhone 16（iOS 26）|iPad mini（iPadOS 26）"),
                color_hex(c"color", c"色", 0xff0000),
                boolean(c"show_forward", c"ピン留め動画", true),
                boolean(c"show_tags", c"タグ", true),
                float(c"author_name_width", c"投稿者名の幅", 160.0, 0.0, 1000.0),
                float(c"side_line_width", c"左右線の幅", 64.0, 0.0, 1000.0),
            ],
            render: render,
        },
    ]
}

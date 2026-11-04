//! Rusty Pixel Sort Filter（Nanashi.）の移植: しきい値で区切った区間ごとに、画素を明るさの順に並べ替える。
//! 明るさは元と同じく 8bit の表示値から (77R + 151G + 28B) >> 8 で求める。

use znd_plugin_sdk::*;

fn luma8(p: [f32; 4]) -> u8 {
    let d = color::to_display(p);
    let c = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u16;
    ((c(d[0]) * 77 + c(d[1]) * 151 + c(d[2]) * 28) >> 8) as u8
}

fn sort_line(line: &mut [(u8, [f32; 4])], threshold: u8, above: bool, inverted: bool) {
    let n = line.len();
    let mut start = 0;
    for i in 0..=n {
        let sorting = i < n && if above { line[i].0 >= threshold } else { line[i].0 < threshold };
        if i == n || !sorting {
            let seg = &mut line[start..i];
            seg.sort_by_key(|k| k.0);
            if inverted {
                seg.reverse();
            }
            start = i + 1;
        }
    }
}

fn render(f: &mut Frame) -> FxResult {
    let threshold = (f.f(0) * 255.0) as u8;
    let above = f.i(1) == 0;
    let dir = f.i(2);
    let vertical = dir >= 2;
    let inverted = dir % 2 == 1;
    let (w, h) = (f.width, f.height);
    let px = |rgba: &[f32], i: usize| [rgba[i * 4], rgba[i * 4 + 1], rgba[i * 4 + 2], rgba[i * 4 + 3]];
    let (outer, inner) = if vertical { (w, h) } else { (h, w) };
    let mut line = Vec::with_capacity(inner);
    for o in 0..outer {
        line.clear();
        for k in 0..inner {
            let i = if vertical { k * w + o } else { o * w + k };
            let p = px(f.rgba, i);
            line.push((luma8(p), p));
        }
        sort_line(&mut line, threshold, above, inverted);
        for (k, (_, p)) in line.iter().enumerate() {
            let i = if vertical { k * w + o } else { o * w + k };
            f.rgba[i * 4..i * 4 + 4].copy_from_slice(p);
        }
    }
    Ok(())
}

plugin! {
    vendor: "Nanashi.（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "sevenc-nanashi.aviutl2-rs.rusty_pixelsort", name: "Rusty Pixel Sort", category: "加工", flags: THREAD_SAFE,
            params: [
                float(c"threshold", c"しきい値", 0.5, 0.0, 1.0),
                choice(c"threshold_type", c"ソート対象", 0, c"しきい値以上|しきい値以下"),
                choice(c"direction", c"ソート方向", 0, c"左右|左右（反転）|上下|上下（反転）"),
            ],
            render: render,
        },
    ]
}

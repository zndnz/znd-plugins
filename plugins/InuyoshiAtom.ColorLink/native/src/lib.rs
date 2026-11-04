//! カラーリンク（InuyoshiAtom）の移植。
//! 「カラーマスター」は 3 色を持つだけで何も描かない。「カラーリンク」はレイヤーを単色化する。
//! カラーリンクの色を、上にあるレイヤーのカラーマスターの色へエクスプレッションでつなぐのは、パッケージのコマンドが行う。

use znd_plugin_sdk::color::to_display;
use znd_plugin_sdk::*;

fn master(_: &mut Frame) -> FxResult {
    Ok(())
}

fn link(f: &mut Frame) -> FxResult {
    let c = f.color(0);
    // パラメータの色はリニア。単色化は表示値で行う
    let d = to_display([c[0], c[1], c[2], 1.0]);
    let mut img = f.input();
    aul::monochrome(&mut img, [d[0], d[1], d[2]], (f.f(1) / 100.0).clamp(0.0, 1.0) as f32, f.b(2));
    f.set(img);
    Ok(())
}

plugin! {
    vendor: "InuyoshiAtom（znd 移植: znd-plugins）", version: "1.0.0", license: "MIT",
    effects: [
        {
            id: "InuyoshiAtom.ColorLink.master", name: "カラーマスター", category: "カラーリンク", flags: THREAD_SAFE,
            params: [
                color_hex(c"color_a", c"色A", 0xff0000),
                color_hex(c"color_b", c"色B", 0xffffff),
                color_hex(c"color_c", c"色C", 0x0080ff),
            ],
            render: master,
        },
        {
            id: "InuyoshiAtom.ColorLink.link", name: "カラーリンク", category: "カラーリンク", flags: THREAD_SAFE,
            params: [
                color_hex(c"color", c"色", 0xffffff),
                float(c"strength", c"強さ", 100.0, 0.0, 100.0),
                boolean(c"keep_luma", c"輝度を保持する", false),
            ],
            render: link,
        },
    ]
}

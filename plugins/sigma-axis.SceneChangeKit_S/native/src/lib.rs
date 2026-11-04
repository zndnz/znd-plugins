//! SceneChangeKit_S（σ軸）の移植: シーンチェンジをタイムライン上で自作するための部品。
//!
//! 元のスクリプトは「自作シーンチェンジ」で前後のシーンの絵を保存し、「前シーン」「次シーン」オブジェクトで
//! それを取り出して、その上にフィルタ効果を重ねて演出を作る。znd では、他のレイヤーの絵を取り込むエフェクトで同じことをする。

use znd_plugin_sdk::*;

fn render(f: &mut Frame) -> FxResult {
    let keep_alpha = f.b(1);
    let Some(mut img) = f.layer_image(0) else {
        // レイヤーが選ばれていない: 透明にする（元のスクリプトもキャッシュが無ければ描かない）
        f.rgba.fill(0.0);
        return Ok(());
    };
    if !keep_alpha {
        // 黒の上に置いて不透明にする
        for p in img.data.chunks_exact_mut(4) {
            p[3] = 1.0;
        }
    }
    f.set(img);
    Ok(())
}

plugin! {
    vendor: "σ軸（znd 移植: znd-plugins）", version: "1.00", license: "MIT",
    effects: [
        {
            id: "sigma-axis.SceneChangeKit_S", name: "シーンの取り込み（SceneChangeKit_S）", category: "トランジション", flags: THREAD_SAFE,
            params: [layer(c"scene", c"シーン（レイヤー）"), boolean(c"keep_alpha", c"アルファチャンネルを維持", true)],
            render: render,
        },
    ]
}

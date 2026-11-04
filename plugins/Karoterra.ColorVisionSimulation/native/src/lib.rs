//! 色覚シミュレーション（Karoterra）の移植: P 型・D 型（Brettel 1997）、T 型（Viénot 1999）、全色盲。

use znd_plugin_sdk::*;

type M = [[f32; 3]; 3];

/// HLSL の mul(v, M)（行ベクトル × 行列）。シェーダーへは行ごとに渡しているので、ここも同じ並び。
fn mulv(v: [f32; 3], m: &M) -> [f32; 3] {
    [
        v[0] * m[0][0] + v[1] * m[1][0] + v[2] * m[2][0],
        v[0] * m[0][1] + v[1] * m[1][1] + v[2] * m[2][1],
        v[0] * m[0][2] + v[1] * m[1][2] + v[2] * m[2][2],
    ]
}

const PROTAN: ([f32; 3], M, M) = (
    [0.00048, 0.00393, -0.00441],
    [[0.14980, 1.19548, -0.34528], [0.10764, 0.84864, 0.04372], [0.00384, -0.00540, 1.00156]],
    [[0.14570, 1.16172, -0.30742], [0.10816, 0.85291, 0.03892], [0.00386, -0.00524, 1.00139]],
);
const DEUTAN: ([f32; 3], M, M) = (
    [-0.00281, -0.00611, 0.00892],
    [[0.36477, 0.86381, -0.22858], [0.26294, 0.64245, 0.09462], [-0.02006, 0.02728, 0.99278]],
    [[0.37298, 0.88166, -0.25464], [0.25954, 0.63506, 0.10540], [-0.01980, 0.02784, 0.99196]],
);
const TRITAN: M = [[1.0, 0.14461, -0.14461], [0.0, 0.85924, 0.14076], [0.0, 0.85924, 0.14076]];

fn render(f: &mut Frame) -> FxResult {
    let sev = (f.f(0) / 100.0) as f32;
    let kind = f.i(1);
    if kind == 0 || sev == 0.0 {
        return Ok(());
    }
    for p in f.rgba.chunks_exact_mut(4) {
        let a = p[3];
        if a <= 0.0 {
            continue;
        }
        // znd の画素はもともとリニアなので、そのまま行列を掛ける（元のシェーダーも sRGB → リニアに直してから掛ける）
        let lin = [p[0] / a, p[1] / a, p[2] / a];
        let sim = match kind {
            1 | 2 => {
                let (n, m1, m2) = if kind == 1 { PROTAN } else { DEUTAN };
                let x = lin[0] * n[0] + lin[1] * n[1] + lin[2] * n[2];
                if x >= 0.0 {
                    mulv(lin, &m1)
                } else {
                    mulv(lin, &m2)
                }
            }
            3 => mulv(lin, &TRITAN),
            _ => {
                let l = color::luma(lin[0], lin[1], lin[2]);
                [l, l, l]
            }
        };
        for k in 0..3 {
            p[k] = (lin[k] + (sim[k] - lin[k]) * sev).max(0.0) * a;
        }
    }
    Ok(())
}

plugin! {
    vendor: "Karoterra（znd 移植: znd-plugins）", version: "1.0", license: "MIT",
    effects: [
        {
            id: "Karoterra.ColorVisionSimulation", name: "色覚シミュレーション", category: "カラー補正", flags: THREAD_SAFE,
            params: [float(c"severity", c"強さ（Severity）", 100.0, 0.0, 100.0), choice(c"type", c"種類", 1, c"Normal|Protan（P 型）|Deutan（D 型）|Tritan（T 型）|Achromat（全色盲）")],
            render: render,
        },
    ]
}

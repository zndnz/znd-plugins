# 色覚シミュレーション

色覚多様性（P 型・D 型・T 型・全色盲）の見え方をシミュレーションする。配色の確認用。

- 元のパッケージ: [aviutl2-ColorVisionSimulation](https://github.com/karoterra/aviutl2-ColorVisionSimulation)（作者: Karoterra、ライセンス: MIT）
- カタログ ID: `Karoterra.ColorVisionSimulation`

## znd での使い方

確認したいレイヤー、または調整レイヤーにエフェクト「カラー補正 › 色覚シミュレーション」を掛ける。
書き出しには掛けたくないときは、パッケージの出力フィルターではなく、確認の間だけエフェクトを有効にする。

## 元との違い

行列は元と同じ（P・D 型は Brettel 1997、T 型は Viénot 1999）。元はシェーダーの中で sRGB → リニアに直して計算するが、
znd の画像は最初からリニアライトなので、その変換は要らない。

# Hex-Tiling

画像を、継ぎ目と繰り返しが目立たないよう六角形のタイルで敷き詰める（Mikkelsen 2022 の Hex-Tiling）。地面・壁などのテクスチャを広い範囲に敷くのに使う。

- 元のパッケージ: [AviUtl2-Hex-Tiling](https://github.com/azurite581/AviUtl2-Hex-Tiling)（作者: azurite、ライセンス: MIT。計算は mmikk の hextile-demo（MIT）に基づく）
- カタログ ID: `azurite.Hex-Tiling`

## znd での使い方

エフェクト「スタイライズ › Hex-Tiling」。テクスチャはレイヤー自身の絵、または「テクスチャのレイヤー」で選んだレイヤーの絵。

## 元との違い

- 元の「画像ファイル」の代わりに、テクスチャにするレイヤーを選ぶ（画像を読み込んだレイヤーを非表示にして選ぶ）。
- 計算は元のシェーダーと同じ。`PI` は無い。

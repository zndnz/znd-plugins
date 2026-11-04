# SomethingLikeGlitch

ブロック・横長・ライン状の断片を少しずつずらすティアリング（2 層）、ランダムな色のカラーブロック、色ズレを重ねたグリッチ。
連続・周期・ランダムのタイミング（バースト）と、エッジだけに掛けるモードがある。

- 元のパッケージ: [aviutl2_script_SomethingLikeGlitch](https://github.com/nctype/aviutl2_script_SomethingLikeGlitch)（作者: nctype、ライセンス: MIT）
- カタログ ID: `nctype.SomethingLikeGlitch`

## znd での使い方

エフェクト「スタイライズ › SomethingLikeGlitch」。

## 元との違い

計算は元のシェーダーと同じ（同じハッシュ・同じ断片の形・プリマルチプライドの表示値）。「エッジのみ」の芯の削り方は円形の収縮で近似している（元は正方形の収縮）。

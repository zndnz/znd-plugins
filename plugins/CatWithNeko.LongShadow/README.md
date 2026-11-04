# LongShadow

AviUtl2 ロングシャドー(&フラットシャドー)

- 元のパッケージ: [LongShadow](https://github.com/CatWithNeko/aviutl2-scripts)（作者: こんにゃくねこ、ライセンス: CC0-1.0）
- カタログ ID: `CatWithNeko.LongShadow`

## znd での使い方

エフェクト「LongShadow」（カテゴリ「装飾」）を掛けると、形を Angle の向きに Length px 伸ばした影を後ろに描きます。

- Start・End（Grad を入れたとき）: 影の根元から先への色
- Alpha・Transparent: 影の不透明度と、先へ行くほど透明にするか
- ShadowOnly: 影だけを出す
- Flat: 影全体を向きに沿った 1 本のグラデーションで塗る（フラットシャドー）

## 元との違い

- 元の FixRotate（オブジェクトの回転を打ち消す）はありません。znd ではエフェクトがレイヤーの回転の前に掛かるので、影の向きはレイヤーと一緒に回ります。
- Flat のグラデーションは元の「グラデーション」フィルタの代わりに、影の向きに沿って Length の幅で塗ります。

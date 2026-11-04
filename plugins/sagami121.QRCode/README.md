# QR コード

文字列から QR コードを描く。ドットの形（四角・円・角丸）・余白・誤り訂正レベル・色・背景透明。

- 元のパッケージ: [Aviutl2-QRCode](https://github.com/sagami121/Aviutl2-QRCode)（作者: sagami121、ライセンス: MIT）
- カタログ ID: `sagami121.QRCode`

## znd での使い方

平面レイヤーにエフェクト「描画 › QR コード」を掛け、「テキスト」に URL などを入れる。平面の中心に QR コードが描かれる。

## 元との違い

- 符号化は [qrcodegen](https://github.com/nayuki/QR-Code-generator)（MIT）で行う（元はスクリプトの中で実装している）。版とマスクは自動で選ばれるので、元と同じ模様になるとは限らない（どちらも読み取れる）。

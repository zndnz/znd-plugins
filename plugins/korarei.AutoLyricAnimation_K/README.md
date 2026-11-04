# AutoLyricAnimation_K

歌詞（テキスト）を 1 文字ずつ少しずつずらして、ランダムな方向（X/Y、正負）から入って・出ていくアニメーションを自動で付ける。

- 元のパッケージ: [AviUtl2_AutoLyricAnimation_K_Script](https://github.com/korarei/AviUtl2_AutoLyricAnimation_K_Script)（作者: korarei、ライセンス: MIT）
- カタログ ID: `korarei.AutoLyricAnimation_K`

## znd での使い方

テキストレイヤーを選び、「プラグイン › AutoLyricAnimation_K」パネルで設定して「付ける」。
テキストに「AutoLyricAnimation_K: …」という名前のアニメーター（位置 ±X/±Y・拡大・反転・非表示）が付き、
文字ごとの量はエクスプレッションセレクターの `alaAmount()` が計算する。「外す」で取り除く。

## 元との違い

- 元は「Ref Motion」トラックに打ったキーフレームの動きをなぞる。znd ではパネルの「動き」でイージングを選ぶ。
- 元はアニメーション効果として描画のたびにレイヤーの長さを見る。znd では付けたときのイン点・アウト点を使うので、長さを変えたら付け直す。
- 乱数の列は AviUtl の `obj.rand` と同じではない。

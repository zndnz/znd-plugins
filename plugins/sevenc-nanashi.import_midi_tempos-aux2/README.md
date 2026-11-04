# import_midi_tempos.aux2

MIDIファイルからBPMを取得し、タイムラインに反映するプラグイン

- 元のパッケージ: [import_midi_tempos.aux2](https://github.com/sevenc-nanashi/import_midi_tempos.aux2)（作者: Nanashi.、ライセンス: MIT）
- カタログ ID: `sevenc-nanashi.import_midi_tempos-aux2`

## znd での使い方

コマンド「MIDI ファイルから BPM グリッドとマーカーを設定」（「プラグイン」メニュー）で MIDI ファイルを選ぶと、最初のテンポと拍子をコンポのビートグリッドにし、MIDI のマーカーをコンポのマーカーにします。テンポが途中で変わるところには「BPM 〜」のマーカーを置きます。「BPM グリッドをリセット」で BPM 120・4 拍子に戻します。

## 元との違い

- AviUtl2 の BPM グリッドは区間ごとに BPM を持てますが、znd のビートグリッドは 1 つの BPM なので、最初のテンポを使い、変わるところはマーカーで示します。
- 既存のコンポのマーカーは、元と同じく読み込んだマーカーで置き換えます（読み込むマーカーがないときは残します）。確認の画面はありません。

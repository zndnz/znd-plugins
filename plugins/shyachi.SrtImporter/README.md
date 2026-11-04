# SrtImporter

SRT字幕ファイルをインポートするプラグイン

- 元のパッケージ: [SrtImporter](https://github.com/shyachi/AviUtl2_SrtImporter)（作者: shyachi、ライセンス: MIT）
- カタログ ID: `shyachi.SrtImporter`

## znd での使い方

「プラグイン」メニューから「SRT Importer」パネルを開き、位置（X・Y は画面の中心からのずれ）・サイズ・フォント・文字色・縁取りを決めてから「Import SRT...」を押して SRT ファイルを選ぶと、字幕 1 つにつき 1 つのテキストレイヤーを作ります。字幕は再生ヘッドの位置を 0 秒として並べ、時刻はフレームに切り捨てます（元と同じ）。

## 元との違い

- 元はレイヤー番号を指定しますが、znd では字幕ごとに別のテキストレイヤーを作ります（時刻順に上から並びます）。
- 文字コードは UTF-8 で、改行は CRLF・LF のどちらでも読めます（元は CRLF のみ）。

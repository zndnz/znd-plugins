# AviUtl2 カタログの移植状況

[AviUtl2 Catalog](https://a2cw.sevenc7c.com)（データ: [Neosku/aviutl2-catalog-data](https://github.com/Neosku/aviutl2-catalog-data)）に
載っているすべてのパッケージについて、znd への移植の状況をまとめたもの。`tools/catalog.py` で生成する。

| 状況 | 数 | 意味 |
|---|---|---|
| 移植済み | 1 | `plugins/` に znd のプラグインとして再実装した |
| znd 本体で対応 | 1 | znd 本体に同じ機能があるため、プラグインにしていない |
| 対象外 | 5 | AviUtl2 の画面・ファイル形式そのものを変えるもの（MOD・言語ファイルなど）で、znd には当てはまらない |
| ソース非公開 | 13 | 配布物しかなく、ソースが公開されていない |
| ライセンス不適合 | 92 | ライセンスが組み込みを許していない（独自・不明・GPL など） |
| 未着手 | 335 | まだ手を付けていない |

| パッケージ | 種類 | 作者 | ライセンス | 状況 | 備考 |
|---|---|---|---|---|---|
| [AutoClipping_S](https://github.com/sigma-axis/aviutl2_script_AutoClipping_S)<br>`sigma-axis.AutoClipping_S` | スクリプト | σ軸 | MIT | [移植済み](plugins/sigma-axis.AutoClipping_S) | エフェクト。大きさの変更は ABI 3 の set_output で行う |
| [L-SMASH-Works](https://github.com/Mr-Ojii/L-SMASH-Works-Auto-Builds)<br>`Mr-Ojii.L-SMASH-Works` | 入力プラグイン | Mr-Ojii | MIT | znd 本体で対応 | 動画・音声の読み込み。znd は FFmpeg で同じ形式（MP4/MKV/MOV/WebM など）を読み込む。配布はビルド済みのバイナリのみ |
| [AviUtl2 Community Translation Companion](https://github.com/aviutl2/aviutl2-community-translation)<br>`aviutl2-community.aviutl2_community_translation_companion` | 言語ファイル | Nanashi. | MIT | 対象外 | AviUtl2 の画面の翻訳ファイル。znd の画面には当てはまらない |
| [アクセスキー付き日本語リソース](https://github.com/Garech-mas/jp_akey.aul2)<br>`Garech.jp_akey.aul2` | 言語ファイル | Garech | CC0-1.0 | 対象外 | AviUtl2 の画面の翻訳ファイル。znd の画面には当てはまらない |
| [非公式言語ファイル(Japanese_Mod)](https://github.com/reindex-ot/reindex_AviUtl2_Language)<br>`reindex-ot.reindex_AviUtl2_Language` | 言語ファイル | Re*Index. (ot_inc) | WTFPL license | 対象外 | AviUtl2 の画面の翻訳ファイル。znd の画面には当てはまらない |
| [Language Files for Scripts and Filters by sigma-axis](https://github.com/sigma-axis/aviutl2_translations_sigma-axis)<br>`sigma-axis.translations_sigma-axis` | 言語ファイル | σ軸 | BSD-2-Clause | 対象外 | AviUtl2 の画面の翻訳ファイル。znd の画面には当てはまらない |
| [Korean (한국어)](https://github.com/Victorique-Bernkastel/AviUtl2_Language)<br>`V.Bernkastel.AviUtl2_Language_kr` | 言語ファイル | Victorique Bernkastel | MIT | 対象外 | AviUtl2 の画面の翻訳ファイル。znd の画面には当てはまらない |
| [倍速プレビュー](https://github.com/airiah7/airiah7--aviutl2--widget--speed-preview)<br>`airiah7.widget--speed-preview` | MOD | あいりあなな(airiah7) | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [OA-速度指定移動](https://overpulse.booth.pm/items/8713264)<br>`OverPulse.Custom_Speed_Move` | スクリプト | OverPulse | Unlicense | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [TA-Delayシャッフル](https://overpulse.booth.pm/items/8795713)<br>`OverPulse.Delayed_Shuffle` | スクリプト | OverPulse | Unlicense | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [TA-瞬間カラー変化](https://www.nicovideo.jp/watch/sm45980061)<br>`OverPulse.Flash_Hue` | スクリプト | OverPulse | Unlicense | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [[Beta] MIDI Info Object](https://c30.booth.pm/items/8420669)<br>`c30.midiinfoobject` | フィルタプラグイン | c30 (zel9278 / fuji_midi) | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [スポイトの色を保存](https://github.com/airiah7/airiah7--aviutl2--plugin--picked-color-storage)<br>`airiah7.plugin--picked-color-storage` | 汎用プラグイン | あいりあなな(airiah7) | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [ドラッグ&ドロップでタイムラインに配置](https://github.com/airiah7/airiah7--aviutl2--widget--drag-and-drop-objects)<br>`airiah7.widget--drag-and-drop-objects` | 汎用プラグイン | あいりあなな(airiah7) | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [AviUtl2 Source Monitor](https://www.youtube.com/watch?v=2SjgBuMDvSM)<br>`eizo-gak.aviutl2-SourceMonitor` | 汎用プラグイン | 映像学区 | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [ニコニ・コモンズ作品ID一覧出力プラグイン](https://www.nicovideo.jp/watch/sm46156601)<br>`Fuji.commons_material_extractor` | 汎用プラグイン | Fuji | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [Voice Drop](https://github.com/Fuji07-2525/voice-drop)<br>`Fuji.voice_drop` | 汎用プラグイン | Fuji | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [Voice Valet](https://github.com/Fuji07-2525/voice-valet)<br>`Fuji.voice_valet` | 汎用プラグイン | Fuji | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [出力先トースト通知プラグイン](https://booth.pm/ja/items/7544010)<br>`namarium.toaster` | 汎用プラグイン | namarium | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [AviUtl2-WhisperAutoSub](https://github.com/nkopikaso/AviUtl2-WhisperAutoSub)<br>`nkopikaso.AviUtl2-WhisperAutoSub` | 汎用プラグイン | nkopikaso | MIT | ソース非公開 | ソースが公開されていない（配布物のみ） |
| [3DCG Movie Animation](https://3dma.shidunami.com/)<br>`shidunami.3dcg_movie_animation` | MOD | しづなみ | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [GetColor-V2R](https://www.dropbox.com/scl/fo/x4l4mepoe5w7d9mtt332g/AK3OZhpP3hjggWZPWDe_8dU/GetColor-V2r.zip?dl=0&e=1&file_subpath=%2FGetColor-V2r&rlkey=w17rfsb61pyhdbws5dfo7jqc9)<br>`93.GetColor-V2R` | スクリプト | 93 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [AUL Receptor](https://tourmaline-rock-7c0.notion.site/AUL-Receptor-315bea655f6482cdb2020104ad22b068)<br>`Amai_28ftb.AULReceptor` | スクリプト | 甘井ふたば | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [DTMer Extension Kit](https://tourmaline-rock-7c0.notion.site/DTMerExtensionKit-Wiki-844bea655f648287b0d48185565f8dc8)<br>`Amai_28ftb.DTMerExtensionKit` | スクリプト | 甘井ふたば | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [DTMer Starter Kit](https://tourmaline-rock-7c0.notion.site/DTMerStarterKit-Wiki-AviUtl2-300bea655f64804d975fea11976ad85c)<br>`Amai_28ftb.DTMerStarterKit` | スクリプト | 甘井ふたば | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [手書き風カクカク振動](https://avilityyy.blogspot.com/2026/04/likeHandWriting.html)<br>`Asu.LikeHandWriting` | スクリプト | アス | 独自ライセンス | ライセンス不適合 | ライセンス: 独自ライセンス（組み込めるライセンスではない） |
| [2段階リリックモーション](https://avilityyy.blogspot.com/2025/12/twoStepsLyricmotion.html)<br>`Asu.TwoStepsLyricmotion` | スクリプト | アス | 独自ライセンス | ライセンス不適合 | ライセンス: 独自ライセンス（組み込めるライセンスではない） |
| [自動呼吸アニメーション](https://www.nicovideo.jp/watch/sm37438844)<br>`chihayuki.kokyuu` | スクリプト | ちはユキ | ニコニ・コモンズ | ライセンス不適合 | ライセンス: ニコニ・コモンズ（組み込めるライセンスではない） |
| [補助線](https://www.nicovideo.jp/watch/sm37066079)<br>`ChihaYuki.obj` | スクリプト | ちはユキ | GPLv3 | ライセンス不適合 | ライセンス: GPLv3（組み込めるライセンスではない） |
| [立体化(g)](https://x.com/gometh55/status/1942624351028928756)<br>`gometh.3dconv_g` | スクリプト | gometh | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [装飾パーティクル_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.DecoParticle_H` | スクリプト | HexBrowns | その他 | ライセンス不適合 | ライセンス: その他（組み込めるライセンスではない） |
| [Handle](https://gist.github.com/korarei/7a8aefa23a12ea97d02cc7943e05a4dd)<br>`korarei.Handle` | スクリプト | Korarei | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [ランダム多角形](https://kurolium.booth.pm/items/8883822)<br>`Kurolium.Random_Figure` | スクリプト | Kurolium | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [BPMカクカク振動](https://x.com/momo_neeeko/status/1944343165852426666)<br>`momo_neeeko.bpm_shake` | スクリプト | もも猫 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [ディレイM](https://twitter.com/momo_neeeko/status/2044756345430417897)<br>`momo_neeeko.delay_m` | スクリプト | もも猫 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [折りたたむ](https://x.com/momo_neeeko/status/1946436761141629252)<br>`momo_neeeko.fold` | スクリプト | もも猫 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [平行投影](https://twitter.com/momo_neeeko/status/2071867409636692120)<br>`momo_neeeko.parallel_projection` | スクリプト | もも猫 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [ピクセル化](https://twitter.com/momo_neeeko/status/2044756345430417897)<br>`momo_neeeko.pixelize` | スクリプト | もも猫 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [虹色グラデーション](https://x.com/momo_neeeko/status/1947433839112163352)<br>`momo_neeeko.rainbow_grad` | スクリプト | もも猫 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [角丸長方形配置](https://x.com/momo_neeeko/status/1944343165852426666)<br>`momo_neeeko.rounded_rect_layout` | スクリプト | もも猫 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [潰す](https://x.com/momo_neeeko/status/1944343165852426666)<br>`momo_neeeko.squash_resize` | スクリプト | もも猫 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [CrushPNG](https://scrapbox.io/nanikani-shugo/CrushPNG)<br>`Nagomiku.CrushPNG` | スクリプト | Nagomiku | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [値で図形](https://x.com/nagomiku7359/status/1943304659487396140)<br>`Nagomiku.shape_by_value` | スクリプト | Nagomiku | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [NKN集中線](https://www.nicovideo.jp/watch/sm45167975)<br>`nakano000.nkn_zoom_lines` | スクリプト | nakano000 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [縁取りグラデーション](https://nao-kid.booth.pm/items/7737503)<br>`naltutyo.fuchidorigurade` | スクリプト | Nao | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [二色化](https://booth.pm/ja/items/7234918)<br>`ngw.anm2` | スクリプト | ngw | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [TA-Delay登場](https://overpulse.booth.pm/items/8755928)<br>`OverPulse.Delayed_Entrance` | スクリプト | OverPulse | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [TA-Delay退場](https://overpulse.booth.pm/items/8772365)<br>`OverPulse.Delayed_Exit` | スクリプト | OverPulse | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [Displace(β)](https://x.com/pon_no_main/status/1152576408108982274)<br>`pon.Displace` | スクリプト | pon | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [個別アンカー操作_Rh](https://x.com/Rhapthorne_0/status/1996544042973409715)<br>`Rhapthorne_0.anchorControl_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [基本効果(Delay)](https://x.com/Rhapthorne_0/status/1978378817640018296)<br>`Rhapthorne_0.basicDelay` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [BlockGlitch_Rh](https://x.com/Rhapthorne_0/status/2000355120782164458)<br>`Rhapthorne_0.BlockGlitch_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [BPMエフェクトトグル](https://x.com/Rhapthorne_0/status/1967755313521512906?s=20)<br>`Rhapthorne_0.BPMEffectToggle` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [個別化レイヤー_Rh](https://twitter.com/Rhapthorne_0/status/2063788101093208349)<br>`Rhapthorne_0.CompositionAsMultiObject_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [彩cle_Rh](https://twitter.com/Rhapthorne_0/status/1985646515592061158)<br>`Rhapthorne_0.HueCycle_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [基本効果(個別指定)](https://twitter.com/Rhapthorne_0/status/1990342570116034832)<br>`Rhapthorne_0.IndividualBasicEffects` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [LineGlitch_Rh](https://x.com/Rhapthorne_0/status/1992790280186011754)<br>`Rhapthorne_0.lineGlitch_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [パララックス_Rh](https://x.com/Rhapthorne_0/status/2078646007345136078)<br>`Rhapthorne_0.Parallax_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [放射ラスター_Rh](https://x.com/Rhapthorne_0/status/2009429917847658676)<br>`Rhapthorne_0.RadialRaster_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [ラディアルシフト](https://x.com/Rhapthorne_0/status/1975320165941129277)<br>`Rhapthorne_0.radialShift` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [描画リピーター_Rh](https://twitter.com/Rhapthorne_0/status/2035884231889404163)<br>`Rhapthorne_0.RepeatedDraw_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [回転ラスター_Rh](https://twitter.com/Rhapthorne_0/status/1988879507135094998)<br>`Rhapthorne_0.RotationRaster_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [角丸補正_Rh](https://x.com/Rhapthorne_0/status/1991423812328648848?s=20)<br>`Rhapthorne_0.roundCornerRh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [パラメータ表示_Rh](https://twitter.com/Rhapthorne_0/status/2040957660900544965)<br>`Rhapthorne_0.ShowParameters_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [単色不透明化+モザイク](https://x.com/Rhapthorne_0/status/1971988664227172734)<br>`Rhapthorne_0.SolidColorMosaic` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [区切り毎に個別化_Rh](https://twitter.com/Rhapthorne_0/status/2097230938032025798)<br>`Rhapthorne_0.SplitText_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [ステータスリンク_Rh](https://twitter.com/Rhapthorne_0/status/2003616617566204199)<br>`Rhapthorne_0.StatusLink_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [ストロボ_Rh](https://twitter.com/Rhapthorne_0/status/1985646515592061158)<br>`Rhapthorne_0.Strobe_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [テキスト自動改行](https://x.com/Rhapthorne_0/status/1971818367167483937)<br>`Rhapthorne_0.TextAutoWrap` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [Textスキン](https://x.com/Rhapthorne_0/status/1967755313521512906)<br>`Rhapthorne_0.TextSkin` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [TimeDelay_Rh](https://twitter.com/Rhapthorne_0/status/2056177955601072592)<br>`Rhapthorne_0.TimeDelay_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [座標の回転・拡大_Rh](https://twitter.com/Rhapthorne_0/status/2045698905698914323)<br>`Rhapthorne_0.TransformCoords_Rh` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [テキスト一枚化](https://x.com/Rhapthorne_0/status/1968330680842297713)<br>`Rhapthorne_0.unifyText` | スクリプト | ラプソーン | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [ported_satsuki](https://github.com/sevenc-nanashi/ported_satsuki)<br>`sevenc-nanashi.ported_satsuki` | スクリプト | Nanashi. | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [中心座標調整](https://note.com/sikemoti/n/n4569e629dc35)<br>`sikemoti.AdjustCenterCoordinate` | スクリプト | しけもち | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [グラデーションジェネレーター](https://note.com/sikemoti/n/n23f4da3cd318)<br>`sikemoti.GradientGenerator` | スクリプト | しけもち | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [レベル補正](https://note.com/sikemoti/n/n9b1843bfb32f)<br>`sikemoti.LevelAdjustment` | スクリプト | しけもち | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [リズミカルP](https://www.nicovideo.jp/watch/sm46966158)<br>`sinyaperu.rhythmical` | スクリプト | 深夜ぺる | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [揃え](https://tikubonn.booth.pm/items/8250336)<br>`tikubonn.Alignment` | スクリプト | ちくぼん | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [シネマスコープ](https://tikubonn.booth.pm/items/8270797)<br>`tikubonn.CinemaScope` | スクリプト | ちくぼん | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [アイリスアウト](https://tikubonn.booth.pm/items/8205637)<br>`tikubonn.IrisOut` | スクリプト | ちくぼん | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [ノスタルジーフィルタ](https://tikubonn.booth.pm/items/8901638)<br>`tikubonn.NostalgiaFilter` | スクリプト | ちくぼん | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [テレビ字幕](https://tikubonn.booth.pm/items/8626637)<br>`tikubonn.TVSubtitle` | スクリプト | ちくぼん | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [ジグザグクリッピング](https://tikubonn.booth.pm/items/8775466)<br>`tikubonn.ZigZagClipping` | スクリプト | ちくぼん | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [特定フレームループ](https://www.nicovideo.jp/watch/sm45249838)<br>`tsts4767.LoopPortionOfFrames` | スクリプト | テステス | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [反復移動プラス](https://www.nicovideo.jp/watch/sm36246269)<br>`tsts4767.RepetitiveMovementPlus` | スクリプト | テステス | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [折り畳み](https://x.com/tt_xiaop/status/1942481707682115712)<br>`tt_xiaop.fold` | スクリプト | テツ | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [扇ブラインド](https://x.com/tt_xiaop/status/1943165775189270911)<br>`tt_xiaop.ougi_blind` | スクリプト | テツ | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [トラックバー対応イージングスクリプト 2020版](https://www.nicovideo.jp/watch/sm37721332)<br>`UndoFish.Easing2020` | スクリプト | UndoFish | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [NagoEffect2.mod2](https://scrapbox.io/nanikani-shugo/NagoEffect2.mod2)<br>`Nagomiku.NagoEffect2.mod2` | スクリプトモジュール | Nagomiku | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [Utsbox Audio Effect](https://www.utsbox.com/?p=4050)<br>`Utsbox.AudioEffect` | フィルタプラグイン | うつぼかずら | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [胸揺れ](https://github.com/vramwiz/Shake_PPP)<br>`VRAMWIZ.Shake_PPP` | フィルタプラグイン | VRAMの魔術師 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [グラフ](https://github.com/vramwiz/SYNC_Graph/)<br>`VRAMWIZ.SYNC_Graph` | フィルタプラグイン | VRAMの魔術師 | GPL-3.0 | ライセンス不適合 | ライセンス: GPL-3.0（組み込めるライセンスではない） |
| [歌詞テロップ](https://github.com/vramwiz/SYNC_Lyrics/)<br>`VRAMWIZ.SYNC_Lyrics` | フィルタプラグイン | VRAMの魔術師 | GPL-3.0-only | ライセンス不適合 | ライセンス: GPL-3.0-only（組み込めるライセンスではない） |
| [MVスタジオ](https://github.com/vramwiz/SYNC_MVStudio/)<br>`VRAMWIZ.SYNC_MVStudio` | フィルタプラグイン | VRAMの魔術師 | GPL-3.0 | ライセンス不適合 | ライセンス: GPL-3.0（組み込めるライセンスではない） |
| [キホンイーズ](https://x.com/soten_various/status/1951590669304336443)<br>`SotenTsumugu.BaseEase` | プリセット | 蒼天つむぐ | GPL-3.0 license | ライセンス不適合 | ライセンス: GPL-3.0 license（組み込めるライセンスではない） |
| [OKLCH パレット](https://gist.github.com/zopty/46b16b7d5f90ec772274e67015117a91)<br>`zopty.OKLCHpalette` | プリセット | zopty | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [動画IN](https://github.com/vramwiz/VW_Media_Input/)<br>`VRAMWIZ.VW_Media_Input` | 入力プラグイン | VRAMの魔術師 | GPL-3.0 | ライセンス不適合 | ライセンス: GPL-3.0（組み込めるライセンスではない） |
| [MP4Exporter](https://apps.esugo.net/aviutl2-mp4exporter/)<br>`Esgo.MP4Exporter` | 出力プラグイン | えすご | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [動画OUT](https://github.com/vramwiz/VW_Media_Output/)<br>`VRAMWIZ.VW_Media_Output` | 出力プラグイン | VRAMの魔術師 | GPL-3.0 | ライセンス不適合 | ライセンス: GPL-3.0（組み込めるライセンスではない） |
| [AviUtl ExEdit2](https://spring-fragrance.mints.ne.jp/aviutl/)<br>`Kenkun.AviUtlExEdit2` | 本体 | ＫＥＮくん | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [TextInput](https://github.com/RubymondJ/TextInput-AviUtl2Plugin)<br>`legashiro.TextInput` | 汎用プラグイン | れがしろ | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [Nao式ツールボックス](https://nao-kid.booth.pm/items/7791311)<br>`naltutyo.kakutyo-tsu-ruba-` | 汎用プラグイン | Nao | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [作業ロガーN](https://nao-kid.booth.pm/items/8025601)<br>`naltutyo.sagyouroga-N` | 汎用プラグイン | Nao | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [ScriptBrowserPlugin](https://nao-kid.booth.pm/items/7684889)<br>`naltutyo.ScriptBrowserPlugin` | 汎用プラグイン | Nao | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [時間制御N](https://nao-kid.booth.pm/items/8121576)<br>`naltutyo.zikanseigyoN` | 汎用プラグイン | Nao | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [AI MIRAI](https://github.com/vramwiz/Aul2MIRAI/)<br>`VRAMWIZ.Aul2MIRAI` | 汎用プラグイン | VRAMの魔術師 | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [MMDアニメーションスタジオ](https://github.com/vramwiz/MMDAnimationStudio/)<br>`VRAMWIZ.MMDAnimationStudio` | 汎用プラグイン | VRAMの魔術師 | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [新旧朗2](https://vramwiz.github.io/Syncroh2-site/)<br>`VRAMWIZ.Syncroh2` | 汎用プラグイン | VRAMの魔術師 | カスタムライセンス | ライセンス不適合 | ライセンス: カスタムライセンス（組み込めるライセンスではない） |
| [チャプターマーカー読み込みプラグイン](https://drive.google.com/drive/u/2/folders/1GhO8jXWakX3DZUptUNLVafKBb7Kmnnc0)<br>`Yuzuriha.ChapterMarker` | 汎用プラグイン | ゆずりは | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [ラウドネス視覚分析](https://drive.google.com/drive/folders/1scs7pYc-EUqhfItoDdPXlV1DKFZ9iYQf?usp=sharing)<br>`Yuzuriha.LoudnessVisualExplorer` | 汎用プラグイン | ゆずりは | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [シンプルレコーダー](https://drive.google.com/drive/u/2/folders/1p3wHfas0d7fLNMffXFM2X6bJU9fW1dWt)<br>`Yuzuriha.SimpleRecorder` | 汎用プラグイン | ゆずりは | 不明 | ライセンス不適合 | ライセンス: 不明（組み込めるライセンスではない） |
| [レイヤー固定](https://github.com/ar-ca-na/aviutl2-layer-pin)<br>`ar-ca-na.layer-pin` | MOD | arcana | MIT | 未着手 |  |
| [レイヤー一括切り替え](https://github.com/Garech-mas/AviUtl2_toggle_layers)<br>`Garech.ToggleLayers` | MOD | Garech | MIT | 未着手 |  |
| [ダイアログサイズ変更MOD](https://github.com/hebiiro/al2_dialog_resizer)<br>`hebiiro.al2_dialog_resizer` | MOD | 蛇色 | BSL-1.0 | 未着手 |  |
| [⚠️【非推奨】ダークモード化MOD](https://github.com/hebiiro/al2_jd/)<br>`hebiiro.al2_jd` | MOD | 蛇色 | MIT | 未着手 |  |
| [プリセット追加MOD](https://github.com/hebiiro/al2_preset_adder)<br>`hebiiro.al2_preset_adder` | MOD | 蛇色 | MIT | 未着手 |  |
| [スリム化MOD](https://github.com/hebiiro/al2_slimming)<br>`hebiiro.al2_slimming` | MOD | 蛇色 | MIT | 未着手 |  |
| [UIドレッシングMOD](https://github.com/hebiiro/al2_ui_dressing)<br>`hebiiro.al2_ui_dressing` | MOD | 蛇色 | MIT | 未着手 |  |
| [フォント視覚化MOD](https://github.com/hebiiro/al2_visual_font)<br>`hebiiro.al2_visual_font` | MOD | 蛇色 | MIT | 未着手 |  |
| [GCMZDrops2](https://github.com/oov/aviutl2_gcmzdrops2)<br>`oov.GCMZDrops2` | MOD | おおぶ | MIT | 未着手 |  |
| [DockLayout](https://github.com/routersys/AUE2-DockLayout)<br>`routersys.DockLayout` | MOD | routersys | MIT | 未着手 |  |
| [グランジ背景を生成](https://github.com/ar-ca-na/aviutl2-grunge-bg)<br>`ar-ca-na.grunge-bg` | スクリプト | arcana | MIT | 未着手 |  |
| [波形ライン](https://github.com/ar-ca-na/aviutl2-waveform-line)<br>`ar-ca-na.waveform-line` | スクリプト | arcana | MIT | 未着手 |  |
| [連結ライン](https://github.com/ar-ca-na/connection_line)<br>`arcana.connection_line` | スクリプト | arcana | MIT | 未着手 |  |
| [中心ずらし_A](https://github.com/azurite581/AviUtl2-AdjustPivot_A)<br>`azurite.AdjustPivot_A` | スクリプト | azurite | MIT | 未着手 |  |
| [ColorHalftone](https://github.com/azurite581/AviUtl2-ColorHalftone)<br>`azurite.ColorHalftone` | スクリプト | azurite | MIT | 未着手 |  |
| [グラデーション+](https://github.com/azurite581/AviUtl2-GradientPlus)<br>`azurite.GradientPlus` | スクリプト | azurite | CC0-1.0 | 未着手 |  |
| [Hex-Tiling](https://github.com/azurite581/AviUtl2-Hex-Tiling)<br>`azurite.Hex-Tiling` | スクリプト | azurite | MIT No Attribution | 未着手 |  |
| [レイヤーグリッド表示](https://github.com/azurite581/AviUtl2-LayerGridView)<br>`azurite.LayerGridView` | スクリプト | azurite | MIT | 未着手 |  |
| [ポイントズーム](https://github.com/azurite581/AviUtl2-PointZoom)<br>`azurite.PointZoom` | スクリプト | azurite | MIT | 未着手 |  |
| [scene-lighting](https://github.com/beive60/aviutl2-scene-lighting)<br>`beive60.scene-lighting` | スクリプト | Beive60 | MIT | 未着手 |  |
| [AlphaDither_C](https://github.com/CatWithNeko/aviutl2-scripts#alphadither_c)<br>`CatWithNeko.AlphaDither` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Canny](https://github.com/CatWithNeko/aviutl2-scripts)<br>`CatWithNeko.Canny` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Chromatic Aberration](https://github.com/CatWithNeko/aviutl2-scripts#chromatic-aberration)<br>`CatWithNeko.ChromaticAberration` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Colorama](https://github.com/CatWithNeko/aviutl2-scripts#colorama)<br>`CatWithNeko.Colorama` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Composite](https://github.com/CatWithNeko/aviutl2-scripts)<br>`CatWithNeko.Composite` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Dither](https://github.com/CatWithNeko/aviutl2-scripts#dither)<br>`CatWithNeko.Dither` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Erosion](https://github.com/CatWithNeko/aviutl2-scripts#erosion)<br>`CatWithNeko.Erosion` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Exposure](https://github.com/CatWithNeko/aviutl2-scripts#exposure)<br>`CatWithNeko.Exposure` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Gamma](https://github.com/CatWithNeko/aviutl2-scripts#gamma)<br>`CatWithNeko.Gamma` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Halftone](https://github.com/CatWithNeko/aviutl2-scripts)<br>`CatWithNeko.Halftone` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Leave Color](https://github.com/CatWithNeko/aviutl2-scripts#leave-color)<br>`CatWithNeko.LeaveColor` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Lens](https://github.com/CatWithNeko/aviutl2-scripts)<br>`CatWithNeko.Lens` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Level](https://github.com/CatWithNeko/aviutl2-scripts#level)<br>`CatWithNeko.Level` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [LongShadow](https://github.com/CatWithNeko/aviutl2-scripts)<br>`CatWithNeko.LongShadow` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [MaxMin](https://github.com/CatWithNeko/aviutl2-scripts#maxmin)<br>`CatWithNeko.MaxMin` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Mosaic_C](https://github.com/CatWithNeko/aviutl2-scripts#mosaic_c)<br>`CatWithNeko.Mosaic` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [SelfDisplace](https://github.com/CatWithNeko/aviutl2-scripts#selfdisplace)<br>`CatWithNeko.SelfDisplace` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Tonecurve_C](https://x.com/CatWithNeko/status/1972685066963960255)<br>`CatWithNeko.ToneCurve` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Turbulent](https://github.com/CatWithNeko/aviutl2-scripts#turbulent)<br>`CatWithNeko.Turbulent` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [Wave Warp](https://github.com/CatWithNeko/aviutl2-scripts#wave-warp)<br>`CatWithNeko.WaveWarp` | スクリプト | こんにゃくねこ | CC0-1.0 | 未着手 |  |
| [ペン&矩形ツール](https://github.com/ColorJellyFish/aviutl2_cjf_pentool)<br>`CJF.PenTool` | スクリプト | CJF | MIT | 未着手 |  |
| [PSD連携アニメーション(非公式)](https://github.com/ColorJellyFish/aviutl2_cjf_psdanimation)<br>`CJF.PSD_anime` | スクリプト | CJF | MIT | 未着手 |  |
| [プレースホルダー](https://github.com/eizo-gak/aviutl2-Placeholder)<br>`eizo-gak.aviutl2-Placeholder` | スクリプト | 映像学区 | MIT | 未着手 |  |
| [多角形モーフ_F](https://github.com/fu-majime/AviUtl2_PolygonMorph_F)<br>`Fu-Majime.AviUtl2_PolygonMorph_F` | スクリプト | フマジメ | MIT | 未着手 |  |
| [日付と時刻](https://www.nicovideo.jp/watch/sm36307237)<br>`Garech.DateTime` | スクリプト | Garech | CC0-1.0 | 未着手 |  |
| [リッチな音声波形_Gk](https://github.com/Gohankaiju/aviutl2-rich-audio-spectrum)<br>`Gohankaiju.rich-audio-spectrum` | スクリプト | Gohankaiju | MIT | 未着手 |  |
| [PSDToolkit用自然な口パク](https://github.com/gomi1124/PSDToolKit-Natural-LipSync)<br>`gomi1124.psdtoolkit-aviutl1-lipsync` | スクリプト | ただのごみ | MIT | 未着手 |  |
| [枠内自動フィット_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.AutoFitText_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [ボールアクション_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.BallAction_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [ビーム化_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.Beam_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [泡_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.Bubbles_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [カードダンス_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.CardDance_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [コースティクス_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.Caustics_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [色付き残像(画像履歴)](https://github.com/HexBrowns/ColoredEchoImg)<br>`HexBrowns.ColoredEchoImg` | スクリプト | HexBrowns | MIT | 未着手 |  |
| [色空間合成_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.ColorSpaceBlend_H` | スクリプト | HexBrowns | MIT | 未着手 |  |
| [コンテンツに応じた塗りつぶし_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.ContentAwareFill_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [拡張パーティクル_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.ExtendedParticle_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [押し出し_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.Extrude_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [フィルム調フィルタ集_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.Filters_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [微粒子_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.FineParticles_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [グリッド_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.Grid_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [手持ちカメラ_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.HandheldCamera_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [キーイング_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.Keying_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [2Dライティング_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.Lighting2D_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [光線経路_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.LightPath_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [光束_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.LightShaft_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [ローポリ](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.Lowpoly` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [光学系ボケ](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.OpticalBokeh` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [平面トラッキング_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.PlanarTracking_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [プロシージャル模様_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.ProceduralPattern_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [電波_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.RadioWaves_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [レンジ警告_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.RangeWarning_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [SceneTransitions](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.SceneTransitions` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [ShapeDistort_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.ShapeDistort_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [検証パターン_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.TestPattern_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [タイムワープ_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.TimeWarp_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [ベクトルブラー_H](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.VectorBlur_H` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [テキスト縦書き化](https://github.com/HexBrowns/HexScript)<br>`HexBrowns.VerticalText` | スクリプト | HexBrowns | MIT No Attribution | 未着手 |  |
| [SliceGlitch](https://note.com/isuzu_hinode/n/n3a7d7b761b98)<br>`hinodeisuzu.sliceglitch` | スクリプト | 暘 弥涼 | MIT | 未着手 |  |
| [ColorLink](https://github.com/InuyoshiAtom/aviutl2-colorlink)<br>`InuyoshiAtom.ColorLink` | スクリプト | 犬好アトム | MIT | 未着手 |  |
| [Open Glow](https://github.com/IZUisMe/aviutl2-open-glow)<br>`IZUisMe.OpenGlow` | スクリプト | 絵に描いた虹 | MIT | 未着手 |  |
| [曲げKR](https://github.com/karoterra/aviutl2-Bend)<br>`Karoterra.Bend` | スクリプト | かろてら | MIT | 未着手 |  |
| [色覚シミュレーションKR](https://github.com/karoterra/aviutl2-ColorVisionSimulation)<br>`Karoterra.ColorVisionSimulation` | スクリプト | かろてら | Unlicense | 未着手 |  |
| [油絵KR](https://github.com/karoterra/aviutl2-OilPainting)<br>`Karoterra.OilPainting` | スクリプト | かろてら | MIT | 未着手 |  |
| [KSA](https://github.com/cycloawaodorin/ksa)<br>`KAZOON.KSA` | スクリプト | KAZOON | MIT | 未着手 |  |
| [AutoLyricAnimation_K](https://github.com/korarei/AviUtl2_AutoLyricAnimation_K_Script)<br>`korarei.AutoLyricAnimation_K` | スクリプト | Korarei | MIT | 未着手 |  |
| [CameraTransform_K](https://github.com/korarei/AviUtl2_CameraTransform_K_Script)<br>`korarei.CameraTransform_K` | スクリプト | Korarei | MIT | 未着手 |  |
| [Constraint](https://gist.github.com/korarei/d5b089a69e0c8ba5903d641d677c89e8)<br>`korarei.Constraint` | スクリプト | Korarei | CC0-1.0 | 未着手 |  |
| [Diffusion_K](https://github.com/korarei/AviUtl2_Diffusion_K_Script)<br>`korarei.Diffusion_K` | スクリプト | Korarei | MIT | 未着手 |  |
| [FlowType_K](https://github.com/korarei/AviUtl2_FlowType_K_Script)<br>`korarei.FlowType_K` | スクリプト | Korarei | MIT | 未着手 |  |
| [Kaleidoscope_K](https://github.com/korarei/AviUtl2_Kaleidoscope_K_Script)<br>`korarei.Kaleidoscope_K` | スクリプト | Korarei | MIT | 未着手 |  |
| [PixelSorter_K](https://github.com/korarei/AviUtl2_PixelSorter_K_Script)<br>`korarei.PixelSorter_K` | スクリプト | Korarei | MIT | 未着手 |  |
| [@Stylize_K](https://github.com/korarei/AviUtl2_Stylize_K_Script)<br>`korarei.Stylize_K` | スクリプト | Korarei | MIT | 未着手 |  |
| [audio2obj](https://github.com/rokujyushi/GCMZScripts)<br>`KuroNeko.audio2obj` | スクリプト | 黒猫大福 | MIT | 未着手 |  |
| [midi2EAP2](https://github.com/rokujyushi/GCMZScripts)<br>`KuroNeko.midi2EAP2` | スクリプト | 黒猫大福 | MIT | 未着手 |  |
| [svg2obj](https://github.com/rokujyushi/GCMZScripts)<br>`KuroNeko.svg2obj` | スクリプト | 黒猫大福 | MIT | 未着手 |  |
| [TTSFileRename](https://github.com/rokujyushi/GCMZScripts)<br>`KuroNeko.TTSFileRename` | スクリプト | 黒猫大福 | MIT | 未着手 |  |
| [VariableFont](https://github.com/rokujyushi/Variable-Font-Plug-ins)<br>`KuroNeko.VariableFont` | スクリプト | 黒猫大福 | MIT | 未着手 |  |
| [Unmult](https://github.com/mes51/AVIUtl2_Unmult)<br>`mes51.Unmult` | スクリプト | mes51 | MIT | 未着手 |  |
| [ベクターキャンバス](https://github.com/Mg32/aviutl2-vectorcanvas)<br>`mogesystem.VectorCanvas` | スクリプト | もげ | MIT | 未着手 |  |
| [虹色グラデーションM](https://x.com/momo_neeeko/status/2108488871327764937)<br>`momo_neeeko.RainbowGradationM` | スクリプト | もも猫 | MIT | 未着手 |  |
| [ラスターM](https://x.com/momo_neeeko/status/2041081558011093323)<br>`momo_neeeko.RasterM` | スクリプト | もも猫 | MIT | 未着手 |  |
| [ルーローの多角形](https://drive.google.com/file/d/1X7q80FYOqJINB2CIKaJ5A9nHXAusRX0r/view?usp=drive_link)<br>`momo_neeeko.ReuleauxPolygon` | スクリプト | もも猫 | MIT | 未着手 |  |
| [スカラップ円](https://x.com/momo_neeeko/status/2040680596410941589)<br>`momo_neeeko.ScallopedCircle` | スクリプト | もも猫 | MIT | 未着手 |  |
| [ウェーブクリッピング](https://x.com/momo_neeeko/status/2037491184260784191)<br>`momo_neeeko.WaveClipping` | スクリプト | もも猫 | MIT | 未着手 |  |
| [SoftAnchorBone](https://github.com/Natadecoco2539/AviUtl2-SoftAnchorBone)<br>`Natadecoco.SoftAnchorBone` | スクリプト | なたのこ | MIT | 未着手 |  |
| [TextBackplate](https://github.com/Natadecoco2539/AviUtl2-TextBackplate)<br>`Natadecoco.TextBackplate` | スクリプト | なたのこ | MIT | 未着手 |  |
| [吹き出し背景@テキスト](https://github.com/natuneko722/TextBackgroundBalloonScript)<br>`natuneko722.text-background-balloon-script` | スクリプト | 夏猫 | MIT | 未着手 |  |
| [AutoBlendLight](https://github.com/nctype/aviutl2_script_AutoBlendLight)<br>`nctype.AutoBlendLight` | スクリプト | nctype | MIT | 未着手 |  |
| [FilmPrintLUT](https://github.com/nctype/aviutl2_script_FilmPrintLUT)<br>`nctype.FilmPrintLUT` | スクリプト | nctype | MIT | 未着手 |  |
| [ParametricBG](https://github.com/nctype/aviutl2_script_ParametricBG)<br>`nctype.ParametricBG` | スクリプト | nctype | MIT | 未着手 |  |
| [Pixelizer](https://github.com/nctype/aviutl2_script_Pixelizer)<br>`nctype.Pixelizer` | スクリプト | nctype | MIT | 未着手 |  |
| [PuniCollision](https://github.com/nctype/aviutl2_script_PuniCollision)<br>`nctype.PuniCollision` | スクリプト | nctype | MIT | 未着手 |  |
| [RainOnPane](https://github.com/nctype/aviutl2_script_RainOnPane)<br>`nctype.RainOnPane` | スクリプト | nctype | MIT | 未着手 |  |
| [ReflectionOnPane](https://github.com/nctype/aviutl2_script_ReflectionOnPane)<br>`nctype.ReflectionOnPane` | スクリプト | nctype | MIT | 未着手 |  |
| [SomethingLikeGlitch](https://github.com/nctype/aviutl2_script_SomethingLikeGlitch)<br>`nctype.SomethingLikeGlitch` | スクリプト | nctype | MIT | 未着手 |  |
| [無印ノイズ](https://github.com/nea-c/aviutl2_script_AviUtlNoise)<br>`NeAc.AviUtlNoise` | スクリプト | 猫愛 | MIT | 未着手 |  |
| [BezierPath](https://github.com/nea-c/aviutl2_script_BezierPath)<br>`NeAc.BezierPath` | スクリプト | 猫愛 | MIT | 未着手 |  |
| [シネマスコープ](https://github.com/nea-c/aviutl2_script_CinemaScope)<br>`NeAc.CinemaScope` | スクリプト | 猫愛 | MIT | 未着手 |  |
| [エ霞(えがすみ)](https://github.com/nea-c/aviutl2_script_Egasumi)<br>`NeAc.Egasumi` | スクリプト | 猫愛 | MIT | 未着手 |  |
| [LongShadowN](https://github.com/nea-c/aviutl2_script_LongShadowN)<br>`NeAc.LongShadowN` | スクリプト | 猫愛 | Apache-2.0 | 未着手 |  |
| [Loop](https://github.com/nea-c/aviutl2_script_Loop)<br>`NeAc.Loop` | スクリプト | 猫愛 | MIT | 未着手 |  |
| [LyricMotioN](https://github.com/nea-c/aviutl2_script_LyricMotioN)<br>`NeAc.LyricMotioN` | スクリプト | 猫愛 | MIT | 未着手 |  |
| [Scatter](https://github.com/nea-c/aviutl2_script_Scatter)<br>`NeAc.Scatter` | スクリプト | 猫愛 | MIT | 未着手 |  |
| [画面アスペクト](https://github.com/nea-c/aviutl2_script_ScreenAspect)<br>`NeAc.ScreenAspect` | スクリプト | 猫愛 | MIT | 未着手 |  |
| [スピード線](https://github.com/nea-c/aviutl2_script_SpeedLine)<br>`NeAc.SpeedLine` | スクリプト | 猫愛 | MIT | 未着手 |  |
| [VRM Toolkit for AviUtl2](https://github.com/nove-nove/aviutl2-vrm-toolkit)<br>`novenove.VRMToolkit` | スクリプト | novenove | MIT | 未着手 |  |
| [BlockNoise](https://github.com/sagami121/Aviutl2-BlockNoise)<br>`sagami121.BlockNoise` | スクリプト | 3aGaM1 | MIT | 未着手 |  |
| [Glitch_T](https://github.com/sagami121/Glitch_T)<br>`sagami121.Glitch_T` | スクリプト | 3aGaM1 | MIT | 未着手 |  |
| [GParticle](https://github.com/sagami121/GParticle)<br>`sagami121.GParticle` | スクリプト | 3aGaM1 | MIT | 未着手 |  |
| [QRCode](https://github.com/sagami121/Aviutl2-QRCode)<br>`sagami121.QRCode` | スクリプト | 3aGaM1 | MIT | 未着手 |  |
| [DVD](https://github.com/SagyouDarui900/AviUtl2_Script_SagyouDarui)<br>`Sagyou.DVD` | スクリプト | サ行 | MIT | 未着手 |  |
| [JpegNoise_for_AviUtl2](https://github.com/SagyouDarui900/JPEGNoise_AviUtl2)<br>`Sagyou.JpegNoise` | スクリプト | サ行 | MIT | 未着手 |  |
| [OpticsCompensation_s_for_AviUtl2](https://github.com/SagyouDarui900/OpticsCompensation_s_for_AviUtl2)<br>`Sagyou.OpticsCompensation_s_for_AviUtl2` | スクリプト | サ行 | MIT | 未着手 |  |
| [墨だまり](https://github.com/5qlufz-1536/AviUtl2_SumiDamari)<br>`saluf.SumiDamari` | スクリプト | saluf | MIT | 未着手 |  |
| [角丸正多角形_sl](https://github.com/seilor0/roundedPolygon_sl)<br>`seilor0.roundedPolygonSl` | スクリプト | セイル | MIT | 未着手 |  |
| [twinkle_sl](https://github.com/seilor0/twinkle_sl)<br>`seilor0.twinkle_sl` | スクリプト | セイル | MIT | 未着手 |  |
| [ベベルとエンボス.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/blob/main/scripts/%E3%83%99%E3%83%99%E3%83%AB%E3%81%A8%E3%82%A8%E3%83%B3%E3%83%9C%E3%82%B9/README.md)<br>`sevenc-nanashi.aviutl2-scripts.bevel_and_emboss-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [カメラフレーム.obj2](https://github.com/sevenc-nanashi/aviutl2-scripts/blob/main/scripts/%E3%82%AB%E3%83%A1%E3%83%A9%E3%83%95%E3%83%AC%E3%83%BC%E3%83%A0/README.md)<br>`sevenc-nanashi.aviutl2-scripts.camera_frame-obj2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [CRTディスプレイ風.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/blob/main/scripts/CRT%E3%83%87%E3%82%A3%E3%82%B9%E3%83%97%E3%83%AC%E3%82%A4%E9%A2%A8/README.md)<br>`sevenc-nanashi.aviutl2-scripts.crt-display.anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [ドカベン.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/tree/main/scripts/%E3%83%89%E3%82%AB%E3%83%99%E3%83%B3)<br>`sevenc-nanashi.aviutl2-scripts.dokaben-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [GPUピクセルソート.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/blob/main/scripts/GPU%E3%83%94%E3%82%AF%E3%82%BB%E3%83%AB%E3%82%BD%E3%83%BC%E3%83%88/README.md)<br>`sevenc-nanashi.aviutl2-scripts.gpu_pixelsort-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [出力時に非表示.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/blob/main/scripts/%E5%87%BA%E5%8A%9B%E6%99%82%E3%81%AB%E9%9D%9E%E8%A1%A8%E7%A4%BA/README.md)<br>`sevenc-nanashi.aviutl2-scripts.hide_on_export-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [旧発光.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/tree/main/scripts/%E6%97%A7%E7%99%BA%E5%85%89)<br>`sevenc-nanashi.aviutl2-scripts.legacy_glow-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [ドット絵変形.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/tree/main/scripts/%E3%83%89%E3%83%83%E3%83%88%E7%B5%B5%E5%A4%89%E5%BD%A2#%E3%83%89%E3%83%83%E3%83%88%E7%B5%B5%E5%A4%89%E5%BD%A2anm2)<br>`sevenc-nanashi.aviutl2-scripts.pixel-transform-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [目標指定拡大縮小.anm2](https://github.com/sevenc-nanashi/aviutl2-scripts/blob/main/scripts/%E7%9B%AE%E6%A8%99%E6%8C%87%E5%AE%9A%E6%8B%A1%E5%A4%A7%E7%B8%AE%E5%B0%8F/README.md)<br>`sevenc-nanashi.aviutl2-scripts.targeted-scale-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [縦動画ガイド.obj2](https://github.com/sevenc-nanashi/aviutl2-scripts/blob/main/scripts/%E7%B8%A6%E5%8B%95%E7%94%BB%E3%82%AC%E3%82%A4%E3%83%89/README.md)<br>`sevenc-nanashi.aviutl2-scripts.vertical-video-obj2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [budoux.obj2](https://github.com/sevenc-nanashi/budoux.obj2)<br>`sevenc-nanashi.budoux-obj2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [disassembler.anm2](https://github.com/sevenc-nanashi/disassembler.anm2)<br>`sevenc-nanashi.disassembler-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [extra_blend_mode.anm2](https://github.com/sevenc-nanashi/extra_blend_mode.anm2)<br>`sevenc-nanashi.extra_blend_mode-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [ntsc-rs.anm2](https://github.com/sevenc-nanashi/ntsc-rs.anm2)<br>`sevenc-nanashi.ntsc-rs` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [ported_tim](https://github.com/sevenc-nanashi/ported_tim)<br>`sevenc-nanashi.ported_tim` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [time_control_assistant.tra2](https://github.com/sevenc-nanashi/time_control_assistant.tra2)<br>`sevenc-nanashi.time_control_assistant-tra2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [transform_specific_chars.anm2](https://github.com/sevenc-nanashi/transform_specific_chars.anm2)<br>`sevenc-nanashi.transform_specific_chars-anm2` | スクリプト | Nanashi. | MIT | 未着手 |  |
| [Basic_S](https://github.com/sigma-axis/aviutl2_script_Basic_S)<br>`sigma-axis.Basic_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [ColoredCollar_S](https://github.com/sigma-axis/aviutl2_script_ColoredCollar_S)<br>`sigma-axis.ColoredCollar_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [GroundShadow2_S](https://github.com/sigma-axis/aviutl2_script_GroundShadow2_S)<br>`sigma-axis.GroundShadow2_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [Histogram_S](https://github.com/sigma-axis/aviutl2_script_Histogram_S)<br>`sigma-axis.Histogram_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [最背面に配置](https://github.com/sigma-axis/aviutl2_script_Layback)<br>`sigma-axis.Layback` | スクリプト | σ軸 | Unlicense | 未着手 |  |
| [Lens_S](https://github.com/sigma-axis/aviutl2_script_Lens_S)<br>`sigma-axis.Lens_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [MovingMosaic_S](https://github.com/sigma-axis/aviutl2_script_MovingMosaic_S)<br>`sigma-axis.MovingMosaic_S` | スクリプト | σ軸 | Unlicense | 未着手 |  |
| [PageRoll_S](https://github.com/sigma-axis/aviutl2_script_PageRoll_S)<br>`sigma-axis.PageRoll_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [Path_S](https://github.com/sigma-axis/aviutl2_script_Path_S/)<br>`sigma-axis.Path_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [PixelSnap_S](https://github.com/sigma-axis/aviutl2_script_PixelSnap_S)<br>`sigma-axis.PixelSnap_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [RadRotDirBlur_S](https://github.com/sigma-axis/aviutl2_script_RadRotDirBlur_S)<br>`sigma-axis.RadRotDirBlur_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [Region_S](https://github.com/sigma-axis/aviutl2_script_Region_S)<br>`sigma-axis.Region_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [Resize_S](https://github.com/sigma-axis/aviutl2_script_Resize_S)<br>`sigma-axis.Resize_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [シーンチェンジ自作キット](https://github.com/sigma-axis/aviutl2_script_SceneChangeKit_S)<br>`sigma-axis.SceneChangeKit_S` | スクリプト | σ軸 | Unlicense | 未着手 |  |
| [Tile_S](https://github.com/sigma-axis/aviutl2_script_Tile_S)<br>`sigma-axis.Tile_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [WhiteBalance_S](https://github.com/sigma-axis/aviutl2_script_WhiteBalance_S)<br>`sigma-axis.WhiteBalance_S` | スクリプト | σ軸 | MIT | 未着手 |  |
| [JPEGCrush](https://note.com/sikemoti/n/n7017a0e6eabf)<br>`sikemoti.JPEGCrush` | スクリプト | しけもち | MIT | 未着手 |  |
| [SeiSa](https://github.com/samuelladoco/aviutl2-script-seisa)<br>`SLDC.SeiSa` | スクリプト | SLDC | MIT | 未着手 |  |
| [放射クリッピング](https://github.com/supernyan74/Radial-Clipping)<br>`supernyan74.RadialClipping` | スクリプト | スーパーNyan | MIT | 未着手 |  |
| [IDgen_Su](https://github.com/Suzukeh/IDgen_Su)<br>`Suzuke.IDgen_Su` | スクリプト | Suzuke | MIT | 未着手 |  |
| [四隅に配置](https://github.com/teruyoshii/Teruyoshi_AviUtl2_Scripts/blob/main/%E5%9B%9B%E9%9A%85%E3%81%AB%E9%85%8D%E7%BD%AE.anm2)<br>`teruyoshii.4corner` | スクリプト | てるよし | MIT | 未着手 |  |
| [ノイズグラデーション](https://x.com/teruyoshi_uec/status/1979927691215471002)<br>`teruyoshii.noiseGradation` | スクリプト | てるよし | MIT | 未着手 |  |
| [ノイズ_Te](https://github.com/teruyoshii/AviUtl2_Noise_Te)<br>`teruyoshii.noiseTe` | スクリプト | てるよし | MIT | 未着手 |  |
| [パスブラー](https://github.com/teruyoshii/Teruyoshi_AviUtl2_Scripts/blob/main/%E3%83%91%E3%82%B9%E3%83%96%E3%83%A9%E3%83%BC.anm2)<br>`teruyoshii.pathBlur` | スクリプト | てるよし | MIT | 未着手 |  |
| [パスグラデーション](https://github.com/teruyoshii/Teruyoshi_AviUtl2_Scripts/blob/main/%E3%83%91%E3%82%B9%E3%82%B0%E3%83%A9%E3%83%87%E3%83%BC%E3%82%B7%E3%83%A7%E3%83%B3.anm2)<br>`teruyoshii.pathGradation` | スクリプト | てるよし | MIT | 未着手 |  |
| [段階グラデーション](https://github.com/teruyoshii/Teruyoshi_AviUtl2_Scripts/blob/main/%E6%AE%B5%E9%9A%8E%E3%82%B0%E3%83%A9%E3%83%87%E3%83%BC%E3%82%B7%E3%83%A7%E3%83%B3.anm2)<br>`teruyoshii.stepGradation` | スクリプト | てるよし | MIT | 未着手 |  |
| [トラックバー音声同期](https://x.com/teruyoshi_uec/status/1965389717224276231)<br>`teruyoshii.TrackbarAudioSync` | スクリプト | てるよし | MIT | 未着手 |  |
| [Liquid Glass エフェクト](https://github.com/Tettu0530/Liquid-Glass)<br>`tettu0530.LiquidGlass` | スクリプト | tettu0530 | MIT | 未着手 |  |
| [NCS風音声波形 (NCSAudioVisualizer)](https://github.com/Tettu0530/NCSAudioVisualizer)<br>`tettu0530.NCSVisualizer` | スクリプト | tettu0530 | MIT | 未着手 |  |
| [TypingEffect](https://github.com/vandojapan/TypingEffect.obj2)<br>`vandojapan.typing-effect` | スクリプト | 4vando | MIT | 未着手 |  |
| [ブラインドループ](https://gist.github.com/zopty/ff3439d79fe609c7714d6db4ef1a39a6)<br>`zopty.BlindLoop` | スクリプト | zopty | BSD-3-Clause | 未着手 |  |
| [どこでもバッファ](https://github.com/zopty/DokodemoBuffer)<br>`zopty.DokodemoBuffer` | スクリプト | zopty | BSD-3-Clause | 未着手 |  |
| [光学モーションブラー](https://gist.github.com/zopty/51deda495876451c1dcfb09ceb05345d)<br>`zopty.OpticalFlowMotionBlur` | スクリプト | zopty | BSD-3-Clause | 未着手 |  |
| [OSC4AU2](https://github.com/zopty/osc4au2)<br>`zopty.osc4au2` | スクリプト | zopty | BSD-3-Clause | 未着手 |  |
| [AviUtl2Browser_H](https://github.com/HexBrowns/AviUtl2Browser_H)<br>`HexBrowns.AviUtl2Browser_H` | スクリプトモジュール | HexBrowns | MIT | 未着手 |  |
| [AI S-R ONNX](https://github.com/animeojisan/AI-S-R-ONNX)<br>`animeojisan.AI_S-R_ONNX` | フィルタプラグイン | animeojisan | MIT | 未着手 |  |
| [DepthMapFilter](https://github.com/azurite581/AviUtl2-DepthMapFilter)<br>`azurite.DepthMapFilter` | フィルタプラグイン | azurite | MIT | 未着手 |  |
| [TrackingStabilizer for AviUtl ExEdit2](https://github.com/Bialis-dev/TrackingStabilizer-for-AviUtl-ExEdit2)<br>`Bialis.TrackingStabilizer-for-AviUtl-ExEdit2` | フィルタプラグイン | Bialis | MIT | 未着手 |  |
| [External Audio Processing 2 (EAP2)](https://github.com/Book-0225/aviutl2_External_Audio_Processing)<br>`Book-0225.EAP2` | フィルタプラグイン | Book0225 | MIT | 未着手 |  |
| [選択ツール式マスク&クリッピング](https://github.com/ColorJellyFish/aviutl2_cjf_maskselector)<br>`CJF.MaskSelector` | フィルタプラグイン | CJF | MIT | 未着手 |  |
| [1クリック物体切り抜き（静止画）](https://github.com/clean262/sam_frame_export_filter)<br>`cleaning.sam_frame_export_filter` | フィルタプラグイン | クリーニング | MIT | 未着手 |  |
| [AviUtl2-LaTeX](https://github.com/Estellellet/AviUtl2-LaTeX)<br>`Estelle.AviUtl2-LaTeX` | フィルタプラグイン | Estelle | MIT | 未着手 |  |
| [HL_Blobox for AviUtl2](https://github.com/Hallkun19/HL_Blobox-for-aul2)<br>`halkun19.HL_Blobox` | フィルタプラグイン | はるくん / harukun19 | MIT | 未着手 |  |
| [ラウドネスノーマライズ_H](https://github.com/HexBrowns/LoudnessNormalize_H)<br>`HexBrowns.LoudnessNormalize_H` | フィルタプラグイン | HexBrowns | MIT | 未着手 |  |
| [風揺れT(DLL)](https://github.com/kanade-ak/kazeyureT)<br>`kanade-ak.kazeyureT` | フィルタプラグイン | kanade-ak | MIT | 未着手 |  |
| [Typst for AviUtl2](https://github.com/karoterra/aviutl2-typst)<br>`Karoterra.Typst` | フィルタプラグイン | かろてら | Apache-2.0 / MIT | 未着手 |  |
| [Lanczos3 / 画素平均法リサイズ](https://github.com/cycloawaodorin/resize_ks)<br>`KAZOON.resize_ks` | フィルタプラグイン | KAZOON | MIT | 未着手 |  |
| [ColorLUT_K](https://github.com/korarei/AviUtl2_ColorLUT_K_Plugin)<br>`korarei.ColorLUT_K` | フィルタプラグイン | Korarei | MIT | 未着手 |  |
| [MotionBlur_K](https://github.com/korarei/AviUtl2_ObjectMotionBlur_LK_Script)<br>`korarei.ObjectMotionBlur_LK` | フィルタプラグイン | Korarei | MIT | 未着手 |  |
| [InterpolationObject](https://github.com/KYU49/InterpolationObject)<br>`KYU.InterpolationObject` | フィルタプラグイン | KYU | Apache-2.0 | 未着手 |  |
| [MorphBridge](https://github.com/nea-c/aviutl2_plugin_MorphBridge)<br>`NeAc.MorphBridge` | フィルタプラグイン | 猫愛 | Apache-2.0 | 未着手 |  |
| [Karaoke Lyric Renderer](https://github.com/nove-nove/aviutl2-karaoke-lyric-renderer)<br>`novenove.KaraokeLyricRenderer` | フィルタプラグイン | novenove | MIT | 未着手 |  |
| [アナログ風](https://github.com/Pachira762/AnalogFuu)<br>`Pachira762.AnalogFuu` | フィルタプラグイン | Pachira762 | MIT | 未着手 |  |
| [vsr4aviutl](https://github.com/Pachira762/vsr4aviutl)<br>`Pachira762.vsr4aviutl` | フィルタプラグイン | Pachira762 | MIT | 未着手 |  |
| [mojie](https://github.com/ronin-gw/aviutl2_mojie)<br>`ronin_gw.mojie` | フィルタプラグイン | Andy.F | MIT | 未着手 |  |
| [Rusty Binaural Filter](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_binaural` | フィルタプラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Chiptune Filter](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_chiptune` | フィルタプラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Equalizer Filter](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_equalizer` | フィルタプラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Pixel Sort Filter](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_pixelsort` | フィルタプラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Random Color Filter](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_random_color` | フィルタプラグイン | Nanashi. | MIT | 未着手 |  |
| [svg.aux2](https://github.com/sevenc-nanashi/svg.aux2)<br>`sevenc-nanashi.svg-aux2` | フィルタプラグイン | Nanashi. | MIT | 未着手 |  |
| [tex.auf2](https://github.com/sevenc-nanashi/tex.auf2)<br>`sevenc-nanashi.tex-auf2` | フィルタプラグイン | Nanashi. | MIT | 未着手 |  |
| [vintage.auf2](https://github.com/sevenc-nanashi/vintage.auf2)<br>`sevenc-nanashi.vintage-auf2` | フィルタプラグイン | Nanashi. | MIT | 未着手 |  |
| [void.auf2](https://github.com/sevenc-nanashi/void.auf2)<br>`sevenc-nanashi.void-auf2` | フィルタプラグイン | Nanashi. | MIT | 未着手 |  |
| [Border_S](https://github.com/sigma-axis/aviutl2_Border_S)<br>`sigma-axis.Border_S` | フィルタプラグイン | σ軸 | MIT | 未着手 |  |
| [AudioFilter](https://github.com/vramwiz/Aul2AudioFilter)<br>`VRAMWIZ.AudioFilter` | フィルタプラグイン | VRAMの魔術師 | MIT | 未着手 |  |
| [呼吸](https://github.com/vramwiz/SYNC_Breath/)<br>`VRAMWIZ.SYNC_Breath` | フィルタプラグイン | VRAMの魔術師 | MIT | 未着手 |  |
| [音楽同期アニメーション](https://github.com/vramwiz/SYNC_Motion/)<br>`VRAMWIZ.SYNC_Motion` | フィルタプラグイン | VRAMの魔術師 | MIT | 未着手 |  |
| [ピアノロール](https://github.com/vramwiz/SYNC_PianoRoll/)<br>`VRAMWIZ.SYNC_PianoRoll` | フィルタプラグイン | VRAMの魔術師 | MIT | 未着手 |  |
| [画面レイアウト](https://github.com/vramwiz/SYNC_ScreenLayout/)<br>`VRAMWIZ.SYNC_ScreenLayout` | フィルタプラグイン | VRAMの魔術師 | MIT | 未着手 |  |
| [パンチラ](https://github.com/vramwiz/TurnOver_PPP)<br>`VRAMWIZ.TurnOver_PPP` | フィルタプラグイン | VRAMの魔術師 | MIT | 未着手 |  |
| [手ブレ補正プラグイン](https://github.com/yu7400ki/aviutl2-stabilizer-plugin)<br>`yu7400ki.stabilizer` | フィルタプラグイン | yu7400ki | MIT | 未着手 |  |
| [HariboteReader](https://github.com/rokujyushi/Haribote)<br>`KuroNeko.Haribote` | 入力プラグイン | 黒猫大福 | MIT | 未着手 |  |
| [WebPFileReader-Plugin](https://github.com/Mr-Ojii/AviUtl2-WebPFileReader-Plugin)<br>`Mr-Ojii.WebPFileReader-Plugin` | 入力プラグイン | Mr-Ojii | MIT | 未着手 |  |
| [HEIF_HEIC Input Plugin](https://github.com/sagami121/HEIF-HEIC-Input)<br>`sagami121.HEIF-HEIC-Input` | 入力プラグイン | 3aGaM1 | MIT | 未着手 |  |
| [NEF Input Plugin](https://github.com/sagami121/NEF-Input)<br>`sagami121.NEF-Input` | 入力プラグイン | 3aGaM1 | MIT | 未着手 |  |
| [Rusty Image Input](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_image_rs_input` | 入力プラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Piano Midi Player Input](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_midi_player` | 入力プラグイン | Nanashi. | MIT | 未着手 |  |
| [ImageCropExport](https://github.com/azurite581/AviUtl2-ImageCropExport)<br>`azurite.ImageCropExport` | 出力プラグイン | azurite | MIT | 未着手 |  |
| [WAVファイル出力](https://github.com/hebiiro/al2_wav_saver/)<br>`hebiiro.al2_wav_saver` | 出力プラグイン | 蛇色 | MIT | 未着手 |  |
| [SMR for F-ZERO GX](https://github.com/cycloawaodorin/fzgx_smr_ks)<br>`KAZOON.fzgx_smr_ks` | 出力プラグイン | KAZOON | MIT | 未着手 |  |
| [WAVファイル出力](https://github.com/cycloawaodorin/wav_output_ks)<br>`KAZOON.wav_output_ks` | 出力プラグイン | KAZOON | MIT | 未着手 |  |
| [MFOutput](https://github.com/MonogoiNoobs/aviutl2-media-foundation-output-plugin)<br>`MonogoiNoobs.MFOutput` | 出力プラグイン | 依神紫苑 | MIT | 未着手 |  |
| [セリフ準備から.srtファイル生成プラグイン](https://github.com/mugi995/Serifu-JunbiToSRT-for-AviUtl2)<br>`Mugi995.Serifu-JunbiToSRT` | 出力プラグイン | Mugi995 | MIT | 未着手 |  |
| [GIFベイヤーディザ出力](https://github.com/osakana4242/aviutl2-gif-bayer-dither-output)<br>`osakana4242.GifBayerDitherOutput` | 出力プラグイン | 三川おさかな | 0BSD | 未着手 |  |
| [ffmpegOut](https://github.com/rigaya/ffmpegOut)<br>`rigaya.ffmpegOut` | 出力プラグイン | rigaya | MIT | 未着手 |  |
| [NVEnc](https://github.com/rigaya/NVEnc)<br>`rigaya.NVEnc` | 出力プラグイン | rigaya | MIT | 未着手 |  |
| [QSVEnc](https://github.com/rigaya/QSVEnc)<br>`rigaya.QSVEnc` | 出力プラグイン | rigaya | MIT | 未着手 |  |
| [svtAV1guiEx](https://github.com/rigaya/svtAV1guiEx)<br>`rigaya.svtAV1guiEx` | 出力プラグイン | rigaya | MIT | 未着手 |  |
| [VCEEnc](https://github.com/rigaya/VCEEnc)<br>`rigaya.VCEEnc` | 出力プラグイン | rigaya | MIT | 未着手 |  |
| [VVenCguiEx](https://github.com/rigaya/VVenCguiEx)<br>`rigaya.VVenCguiEx` | 出力プラグイン | rigaya | MIT | 未着手 |  |
| [x264guiEx](https://github.com/rigaya/x264guiEx)<br>`rigaya.x264guiEx` | 出力プラグイン | rigaya | MIT | 未着手 |  |
| [x265guiEx](https://github.com/rigaya/x265guiEx)<br>`rigaya.x265guiEx` | 出力プラグイン | rigaya | MIT | 未着手 |  |
| [AudioEnc](https://github.com/sagami121/Aviutl2-AudioEnc)<br>`sagami121.AudioEnc` | 出力プラグイン | 3aGaM1 | MIT | 未着手 |  |
| [Rusty FFmpeg Output](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_ffmpeg` | 出力プラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Image Output](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_image_rs_output` | 出力プラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Single Image Output](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_single_image_output` | 出力プラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Statistics Output](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_statistics` | 出力プラグイン | Nanashi. | MIT | 未着手 |  |
| [APNG出力プラグイン](https://yu7400ki.me/aviutl2-animated-image-output/)<br>`yu7400ki.anmImOutput-apng` | 出力プラグイン | yu7400ki | MIT | 未着手 |  |
| [AVIF出力プラグイン](https://yu7400ki.me/aviutl2-animated-image-output/)<br>`yu7400ki.anmImOutput-avif` | 出力プラグイン | yu7400ki | MIT | 未着手 |  |
| [GIF出力プラグイン](https://yu7400ki.me/aviutl2-animated-image-output/)<br>`yu7400ki.anmImOutput-gif` | 出力プラグイン | yu7400ki | MIT | 未着手 |  |
| [JXL出力プラグイン](https://yu7400ki.me/aviutl2-animated-image-output/)<br>`yu7400ki.anmImOutput-jxl` | 出力プラグイン | yu7400ki | MIT | 未着手 |  |
| [WebP出力プラグイン](https://yu7400ki.me/aviutl2-animated-image-output/)<br>`yu7400ki.anmImOutput-webp` | 出力プラグイン | yu7400ki | MIT | 未着手 |  |
| [aup2 リンク修復](https://github.com/ar-ca-na/aviutl2-aup2-relink)<br>`ar-ca-na.aup2-relink` | 外部プログラム | arcana | MIT | 未着手 |  |
| [前回のプロジェクトを開く](https://github.com/ar-ca-na/aviutl2-open-last-project)<br>`ar-ca-na.open-last-project` | 外部プログラム | arcana | MIT | 未着手 |  |
| [AviUtl2_Discord_RPC](https://github.com/amaii4/AviUtl2_Discord_RPC)<br>`amaii4.AviUtl2_Discord_RPC` | 汎用プラグイン | 甘味 | MIT | 未着手 |  |
| [Gradient Editor](https://github.com/azurite581/AviUtl2-GradientEditor)<br>`azurite.GradientEditor` | 汎用プラグイン | azurite | MIT | 未着手 |  |
| [aviutl2-alias-inserter](https://github.com/beive60/aviutl2-alias-inserter)<br>`beive60.aviutl2-alias-inserter` | 汎用プラグイン | Beive60 | MIT | 未着手 |  |
| [aviutl2-audio-rec](https://github.com/beive60/aviutl2-audio-rec)<br>`beive60.aviutl2-audio-rec` | 汎用プラグイン | Beive60 | MIT | 未着手 |  |
| [AviUtl2 relink](https://github.com/Book-0225/aviutl2_relink)<br>`Book-0225.relink` | 汎用プラグイン | Book0225 | MIT | 未着手 |  |
| [1クリック動画切り抜き](https://github.com/clean262/sam3_bb_gb_generator)<br>`cleaning.sam3_bb_gb_generator` | 汎用プラグイン | クリーニング | MIT | 未着手 |  |
| [BPMグリッド倍化](https://github.com/Garech-mas/DoubleBPMGrid)<br>`Garech.DoubleBPMGrid` | 汎用プラグイン | Garech | MIT | 未着手 |  |
| [フィルタ分離](https://github.com/Garech-mas/AviUtl2_SplitFilters)<br>`Garech.SplitFilters` | 汎用プラグイン | Garech | MIT | 未着手 |  |
| [HL_Ezing for AviUtl2](https://github.com/Hallkun19/HL_Ezing-for-aul2)<br>`halkun19.HL_Ezing` | 汎用プラグイン | はるくん / harukun19 | MIT | 未着手 |  |
| [HL_FxWheel for AviUtl2](https://github.com/Hallkun19/HL_FxWheel-for-aul2)<br>`halkun19.HL_FxWheel` | 汎用プラグイン | はるくん / harukun19 | MIT | 未着手 |  |
| [メモ帳プラグイン](https://github.com/hebiiro/al2_memo)<br>`hebiiro.al2_memo` | 汎用プラグイン | 蛇色 | MIT | 未着手 |  |
| [タイムラインオブジェクト整列プラグイン](https://github.com/hebiiro/al2_timeline_object_arranger)<br>`hebiiro.al2_timeline_object_arranger` | 汎用プラグイン | 蛇色 | BSL-1.0 | 未着手 |  |
| [BackupDiff_H](https://github.com/HexBrowns/BackupDiff_H)<br>`HexBrowns.BackupDiff_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [ColorHistory_H](https://github.com/HexBrowns/ColorHistory_H)<br>`HexBrowns.ColorHistory_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [CurveEditor_H](https://github.com/HexBrowns/CurveEditor_H)<br>`HexBrowns.CurveEditor_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [EditScript_H](https://github.com/HexBrowns/EditScript_H)<br>`HexBrowns.EditScript_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [FileFinder_H](https://github.com/HexBrowns/FileFinder_H)<br>`HexBrowns.FileFinder_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [FullSizePreview_H](https://github.com/HexBrowns/FullSizePreview_H)<br>`HexBrowns.FullSizePreview_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [レイヤー指定無音カット_H](https://github.com/HexBrowns/LayerSilenceCut_H)<br>`HexBrowns.LayerSilenceCut_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [ManuscriptPanel_H](https://github.com/HexBrowns/ManuscriptPanel_H)<br>`HexBrowns.ManuscriptPanel_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [MediaCollect_H](https://github.com/HexBrowns/MediaCollect_H)<br>`HexBrowns.MediaCollect_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [MediaProbe_H](https://github.com/HexBrowns/MediaProbe_H)<br>`HexBrowns.MediaProbe_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [MidpointTable_H](https://github.com/HexBrowns/MidpointTable_H)<br>`HexBrowns.MidpointTable_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [PdfPage_H](https://github.com/HexBrowns/PdfPage_H)<br>`HexBrowns.PdfPage_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [SplitLayout_H](https://github.com/HexBrowns/SplitLayout_H)<br>`HexBrowns.SplitLayout_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [SVG_TRACE](https://github.com/HexBrowns/svg_trace)<br>`HexBrowns.svg_trace` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [TextReplace_H](https://github.com/HexBrowns/TextReplace_H)<br>`HexBrowns.TextReplace_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [VideoScope_H](https://github.com/HexBrowns/VideoScope_H)<br>`HexBrowns.VideoScope_H` | 汎用プラグイン | HexBrowns | MIT | 未着手 |  |
| [リップル削除](https://github.com/Joster777/Aviutl2_Plugin_RippleDelete)<br>`JobsJoestar.RippleDelete` | 汎用プラグイン | JobsJoestar | MIT | 未着手 |  |
| [文字位置調整](https://github.com/kanade-ak/TextCharEditor)<br>`kanade-ak.TextCharEditor` | 汎用プラグイン | kanade-ak | MIT | 未着手 |  |
| [CustomToolbox](https://github.com/Kanata844/CustomToolbox)<br>`Kanata844.CustomToolbox` | 汎用プラグイン | Kanata844 | MIT | 未着手 |  |
| [MediaReplacer](https://github.com/Kanata844/MediaReplacer)<br>`Kanata844.MediaReplacer` | 汎用プラグイン | Kanata844 | MIT | 未着手 |  |
| [Waveform Preview](https://github.com/karoterra/aviutl2-waveform-preview)<br>`Karoterra.WaveformPreview` | 汎用プラグイン | かろてら | MIT | 未着手 |  |
| [オブジェクト行複製](https://github.com/koisi0319/Aviutl2-Duplicate-objects-at-equal-intervals)<br>`koisi0319.Duplicate_objects` | 汎用プラグイン | koisi0319 | MIT | 未着手 |  |
| [CopyAlias](https://github.com/rokujyushi/Copy_alias)<br>`KuroNeko.CopyAlias` | 汎用プラグイン | 黒猫大福 | MIT | 未着手 |  |
| [FontPreview](https://github.com/rokujyushi/FontPreview)<br>`KuroNeko.FontPreview` | 汎用プラグイン | 黒猫大福 | MIT | 未着手 |  |
| [Aup2GapFiller](https://github.com/miya999949/Aup2GapFiller)<br>`MIYA.Aup2GapFiller` | 汎用プラグイン | みや | MIT | 未着手 |  |
| [LocusMotion](https://github.com/momoneeeko/LocusMotion.aux2)<br>`momo_neeeko.LocusMotion` | 汎用プラグイン | もも猫 | MIT | 未着手 |  |
| [Any Shortcut for AviUtl2](https://github.com/mugi995/Any_Shortcut_for_AviUtl2)<br>`Mugi995.Any_Shortcut_for_AviUtl2` | 汎用プラグイン | Mugi995 | MIT | 未着手 |  |
| [MyAssetManager](https://github.com/Natadecoco2539/AviUtl2-MyAssetManager)<br>`Natadecoco.MyAssetManager` | 汎用プラグイン | なたのこ | MIT | 未着手 |  |
| [TextEditor](https://github.com/Natadecoco2539/AviUtl2-TextEditor)<br>`Natadecoco.TextEditor` | 汎用プラグイン | なたのこ | MIT | 未着手 |  |
| [TextSplitter](https://github.com/Natadecoco2539/AviUtl2-TextSplitter)<br>`Natadecoco.TextSplitter` | 汎用プラグイン | なたのこ | MIT | 未着手 |  |
| [フレームキャプチャー](https://github.com/natuneko722/FrameCapturePlugin)<br>`natuneko722.FrameCapturePlugin` | 汎用プラグイン | 夏猫 | MIT | 未着手 |  |
| [OCR文字抽出プラグイン](https://github.com/natuneko722/OCRTextExtractorPlugin)<br>`natuneko722.OCRTextExtractorPlugin` | 汎用プラグイン | 夏猫 | MIT | 未着手 |  |
| [MotionTracking MK-II Plus for AviUtl2](https://github.com/nullruptr/MotionTracking_MK-II_Plus_for_AviUtl2)<br>`nullru.MotionTracking_MK-II_Plus_for_AviUtl2` | 汎用プラグイン | nullru | MIT | 未着手 |  |
| [Easing Quick Setup](https://github.com/NumberZero-0621/AviUtl2-easing-quick-setup)<br>`Number-0621.easing-quick-setup` | 汎用プラグイン | No.0 | MIT-0 License | 未着手 |  |
| [SelectEasing](https://github.com/NumberZero-0621/AviUtl2-Plugin-SelectEasing)<br>`Number-0621.SelectEasing` | 汎用プラグイン | No.0 | MIT-0  license | 未着手 |  |
| [PSDToolKit2](https://github.com/oov/aviutl2_psdtoolkit2)<br>`oov.PSDToolKit2` | 汎用プラグイン | おおぶ | MIT | 未着手 |  |
| [Aviutl2-Calculator](https://github.com/sagami121/Aviutl2-Calculator)<br>`sagami121.Calculator` | 汎用プラグイン | 3aGaM1 | MIT | 未着手 |  |
| [Rusty Local Alias Plugin](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_local_alias_plugin` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Metronome Plugin](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_metronome_plugin` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Restart Shortcut Plugin](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_restart_shortcut` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty Scripts Search Plugin](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_scripts_search` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [Rusty SRT File Plugin](https://github.com/sevenc-nanashi/aviutl2-rs)<br>`sevenc-nanashi.aviutl2-rs.rusty_srt_file` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [clipboard.aux2](https://github.com/sevenc-nanashi/clipboard.aux2)<br>`sevenc-nanashi.clipboard-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [create_control_object.aux2](https://github.com/sevenc-nanashi/create_control_object.aux2)<br>`sevenc-nanashi.create_control_object-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [discord_activity.aux2](https://github.com/sevenc-nanashi/discord_activity.aux2)<br>`sevenc-nanashi.discord_activity-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [enhanced_tracks.aux2](https://github.com/sevenc-nanashi/enhanced_tracks.aux2)<br>`sevenc-nanashi.enhanced_tracks-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [filter_effect_toggle.aux2](https://github.com/sevenc-nanashi/filter_effect_toggle.aux2)<br>`sevenc-nanashi.filter_effect_toggle-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [import_midi_tempos.aux2](https://github.com/sevenc-nanashi/import_midi_tempos.aux2)<br>`sevenc-nanashi.import_midi_tempos-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [insert.aux2](https://github.com/sevenc-nanashi/insert.aux2)<br>`sevenc-nanashi.insert-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [modify_group_num_layers.aux2](https://github.com/sevenc-nanashi/modify_group_num_layers.aux2)<br>`sevenc-nanashi.modify_group_num_layers-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [object_as_frame_selection.aux2](https://github.com/sevenc-nanashi/object_as_frame_selection.aux2)<br>`sevenc-nanashi.object_as_frame_selection-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [open_project_directory.aux2](https://github.com/sevenc-nanashi/open_project_directory.aux2)<br>`sevenc-nanashi.open_project_directory-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [quantizer.aux2](https://github.com/sevenc-nanashi/quantizer.aux2)<br>`sevenc-nanashi.quantizer-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [reset_frame_selection_on_focus.aux2](https://github.com/sevenc-nanashi/reset_frame_selection_on_focus.aux2)<br>`sevenc-nanashi.reset_frame_selection_on_focus-aux2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [rikky_modoki.aux2](https://github.com/sevenc-nanashi/rikky_modoki.aux2)<br>`sevenc-nanashi.rikky_modoki-anm2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [Tinted AviUtl2](https://github.com/sevenc-nanashi/tinted-aviutl2)<br>`sevenc-nanashi.tinted-aviutl2` | 汎用プラグイン | Nanashi. | MIT | 未着手 |  |
| [SrtImporter](https://github.com/shyachi/AviUtl2_SrtImporter)<br>`shyachi.SrtImporter` | 汎用プラグイン | shyachi | MIT | 未着手 |  |
| [TLショトカ移動2](https://github.com/sigma-axis/aviutl2_tl_walkaround2)<br>`sigma-axis.tl_walkaround2` | 汎用プラグイン | σ軸 | MIT | 未着手 |  |
| [Palette History](https://github.com/teruyoshii/AviUtl2_Palette_History)<br>`teruyoshii.Palette_History` | 汎用プラグイン | てるよし | MIT | 未着手 |  |
| [AutoZWJ](https://github.com/TNOTawa/AutoZWJ)<br>`tnotawa.autozwj` | 汎用プラグイン | TNOTawa | MIT | 未着手 |  |
| [aviutl2_mcp_bridge](https://github.com/TranczMeq/aviutl2-live-mcp)<br>`tranczmeq.aviutl2-mcp-bridge` | 汎用プラグイン | TranczMeq | MIT | 未着手 |  |
| [comfont.aux2](https://github.com/vandojapan/comfont)<br>`vandojapan.comfont` | 汎用プラグイン | 4vando | MIT | 未着手 |  |
| [地図](https://github.com/vramwiz/MapRaku/)<br>`VRAMWIZ.SYNC_MapRaku` | 汎用プラグイン | VRAMの魔術師 | MIT | 未着手 |  |
| [AviUtl2 MCP](https://github.com/yh2237/AviUtl2-MCP)<br>`yh2237.AviUtl2-MCP` | 汎用プラグイン | yh | MIT | 未着手 |  |

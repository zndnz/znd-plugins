# AGENTS.md

- 1 パッケージ（カタログの 1 項目）を移植するごとに 1 コミットする。`main` に直接コミットする。
- 手順: `tools/scaffold.py <ID> [--native] [--license-from 元のLICENSE]` → 実装 → `tools/check.sh` → `tools/done.py <ID> ported` → コミット。
- 移植しないものも `tools/done.py <ID> builtin|na|closed|license "<理由>"` で理由を残す。
- コミットメッセージは日本語。1 行目は `<カタログID>: 変更の要約`、空行のあとに箇条書き。
- `git push` は頼まれたときだけ。

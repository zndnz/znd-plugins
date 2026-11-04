#!/bin/sh
# 1 パッケージ分をコミットする。
#   tools/commit.sh <ID> <状況> <備考> <1 行目の要約> [箇条書き...]
set -e
cd "$(dirname "$0")/.."
id=$1; st=$2; note=$3; title=$4; shift 4
python3 tools/done.py "$id" "$st" "$note" >/dev/null
msg="$id: $title
"
for b in "$@"; do msg="$msg
- $b"; done
msg="$msg

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"
git add -A catalog CATALOG.md Cargo.lock; [ -d "plugins/$id" ] && git add -A "plugins/$id"; git add -A harness
git commit -q -m "$msg"
git log --oneline -1

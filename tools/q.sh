#!/bin/sh
# tools/check.sh を静かに実行し、失敗したときだけエラーを出して 1 を返す。
cd "$(dirname "$0")/.."
log=$(mktemp)
if tools/check.sh >"$log" 2>&1 && ! grep -q "^warning: unused\|^warning: .*never used\|FAILED" "$log"; then rm -f "$log"; echo "check: ok"; exit 0; fi
grep -E "^error|^warning|FAILED|panicked" -A9 "$log" | head -60
rm -f "$log"
exit 1

#!/bin/sh
# すべてのプラグインをビルドし、ネイティブのライブラリを各パッケージの lib/ に置く。
#   tools/build.sh            リリースビルド
#   tools/build.sh debug      デバッグビルド（テスト用）
# できた plugins/<ID>/ フォルダを znd のプラグインフォルダ
# （macOS: ~/Library/Application Support/znd/plugins）にコピーするか、ZND_PLUGIN_PATH に plugins/ を指定すると使える。
set -e
cd "$(dirname "$0")/.."
profile=${1:-release}
if [ "$profile" = release ]; then cargo build --workspace --release --exclude harness; else cargo build --workspace --exclude harness; fi
case "$(uname -s)" in
  Darwin) ext=dylib ;;
  MINGW*|MSYS*|CYGWIN*) ext=dll ;;
  *) ext=so ;;
esac
for toml in plugins/*/native/Cargo.toml; do
  [ -f "$toml" ] || continue
  dir=$(dirname "$(dirname "$toml")")
  name=$(sed -n 's/^name *= *"\(.*\)"/\1/p' "$toml" | head -1 | tr - _)
  src="target/$profile/lib$name.$ext"
  [ -f "$src" ] || src="target/$profile/$name.$ext"
  mkdir -p "$dir/lib"
  cp "$src" "$dir/lib/"
done
echo "ビルドしました（$profile）"

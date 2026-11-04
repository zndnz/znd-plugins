#!/usr/bin/env python3
"""移植するパッケージの雛形を作る。

  tools/scaffold.py <カタログID> [--native] [--license-from <元のLICENSEファイル>]

plugins/<ID>/ に plugin.json・LICENSE・README.md（native なら native/Cargo.toml と src/lib.rs）を作る。
既にあるファイルは上書きしない。
"""
import json, os, re, sys
root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
args = sys.argv[1:]
cid = args[0]
native = '--native' in args
lic_from = args[args.index('--license-from') + 1] if '--license-from' in args else None
cat = {p['id']: p for p in json.load(open(os.path.join(root, 'catalog/catalog.json')))}
p = cat[cid]
d = os.path.join(root, 'plugins', cid)
os.makedirs(d, exist_ok=True)

def write(rel, text):
    path = os.path.join(d, rel)
    if os.path.exists(path):
        return
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, 'w').write(text)

SPDX = {'MIT No Attribution': 'MIT-0', 'MIT-0 License': 'MIT-0', 'MIT-0  license': 'MIT-0', 'WTFPL license': 'WTFPL'}
lics = [SPDX.get(l['type'], l['type']) for l in p['licenses']]
spdx = ' AND '.join(lics)
if lic_from:
    write('LICENSE', open(lic_from, encoding='utf-8', errors='replace').read())
else:
    parts = []
    for l in p['licenses']:
        t = SPDX.get(l['type'], l['type'])
        text = open(os.path.join(root, 'tools/licenses', t + '.txt')).read()
        holders = ', '.join(f"{c.get('years', '')} {c.get('holder', '')}".strip() for c in l['copyrights']) or p['author']
        text = text.replace('<year> <copyright holders>', holders).replace('<YEAR> <COPYRIGHT HOLDER>', holders)
        text = text.replace('<year> <owner>', holders).replace('[yyyy] [name of copyright owner]', holders)
        if t in ('CC0-1.0', 'Unlicense', 'WTFPL', 'Apache-2.0', 'MIT-0', 'BSL-1.0', '0BSD') and holders not in text:
            text = f"Copyright (c) {holders}\n\n" + text
        parts.append(text)
    write('LICENSE', '\n\n'.join(parts))

crate = 'zp-' + re.sub(r'[^a-z0-9]+', '-', cid.lower()).strip('-')
lib = crate.replace('-', '_')
manifest = {
    'id': cid,
    'name': p['name'],
    'version': '1.0.0',
    'vendor': f"{p['author']}（znd 移植: znd-plugins）",
    'license': spdx,
    'description': p['summary'],
    'permissions': [],
}
if native:
    manifest['effects'] = ['lib/' + lib]
write('plugin.json', json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
write('README.md', f"""# {p['name']}

{p['summary']}

- 元のパッケージ: [{p['name']}]({p['repo']})（作者: {p['author']}、ライセンス: {spdx}）
- カタログ ID: `{cid}`

## znd での使い方

（移植した内容を書く）

## 元との違い

（AviUtl2 との違いを書く）
""")
if native:
    write('native/Cargo.toml', f"""[package]
name = "{crate}"
version = "1.0.0"
edition = "2021"
license = "{spdx}"
publish = false

[lib]
crate-type = ["cdylib"]
path = "src/lib.rs"

[dependencies]
znd-plugin-sdk.workspace = true
""")
    write('native/src/lib.rs', "use znd_plugin_sdk::*;\n")
print(d)

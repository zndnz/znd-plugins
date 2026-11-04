#!/usr/bin/env python3
"""catalog/catalog.json と catalog/status.json から CATALOG.md を作る。"""
import json, os, collections
root = os.path.join(os.path.dirname(__file__), '..')
cat = json.load(open(os.path.join(root, 'catalog/catalog.json')))
status = json.load(open(os.path.join(root, 'catalog/status.json')))
LABEL = {
    'ported': '移植済み',
    'builtin': 'znd 本体で対応',
    'na': '対象外',
    'closed': 'ソース非公開',
    'license': 'ライセンス不適合',
    'pending': '未着手',
}
ORDER = ['ported', 'builtin', 'na', 'closed', 'license', 'pending']
count = collections.Counter(status[p['id']]['status'] for p in cat)
lines = [
    '# AviUtl2 カタログの移植状況',
    '',
    '[AviUtl2 Catalog](https://a2cw.sevenc7c.com)（データ: [Neosku/aviutl2-catalog-data](https://github.com/Neosku/aviutl2-catalog-data)）に',
    '載っているすべてのパッケージについて、znd への移植の状況をまとめたもの。`tools/catalog.py` で生成する。',
    '',
    '| 状況 | 数 | 意味 |',
    '|---|---|---|',
    f"| {LABEL['ported']} | {count['ported']} | `plugins/` に znd のプラグインとして再実装した |",
    f"| {LABEL['builtin']} | {count['builtin']} | znd 本体に同じ機能があるため、プラグインにしていない |",
    f"| {LABEL['na']} | {count['na']} | AviUtl2 の画面・ファイル形式そのものを変えるもの（MOD・言語ファイルなど）で、znd には当てはまらない |",
    f"| {LABEL['closed']} | {count['closed']} | 配布物しかなく、ソースが公開されていない |",
    f"| {LABEL['license']} | {count['license']} | ライセンスが組み込みを許していない（独自・不明・GPL など） |",
]
if count['pending']:
    lines.append(f"| {LABEL['pending']} | {count['pending']} | まだ手を付けていない |")
lines += ['', '| パッケージ | 種類 | 作者 | ライセンス | 状況 | 備考 |', '|---|---|---|---|---|---|']
def key(p):
    return (ORDER.index(status[p['id']]['status']), p['type'], p['id'].lower())
for p in sorted(cat, key=key):
    s = status[p['id']]
    lic = ' / '.join(l['type'] for l in p['licenses']) or '—'
    name = p['name'].replace('|', '\\|')
    link = f"[{name}]({p['repo']})" if p['repo'] else name
    st = LABEL[s['status']]
    if s['status'] == 'ported' and s.get('path'):
        st = f"[{st}]({s['path']})"
    note = (s.get('note') or '').replace('|', '\\|').replace('\n', ' ')
    lines.append(f"| {link}<br>`{p['id']}` | {p['type']} | {p['author'].replace('|', '/')} | {lic} | {st} | {note} |")
open(os.path.join(root, 'CATALOG.md'), 'w').write('\n'.join(lines) + '\n')
print(dict(count))

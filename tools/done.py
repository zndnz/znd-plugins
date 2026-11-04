#!/usr/bin/env python3
"""catalog/status.json の状況を書き換えて CATALOG.md を作り直す。

  tools/done.py <カタログID> ported [備考]
  tools/done.py <カタログID> builtin|na|closed|license <備考>
"""
import json, os, subprocess, sys
root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
cid, st = sys.argv[1], sys.argv[2]
note = sys.argv[3] if len(sys.argv) > 3 else ''
path = os.path.join(root, 'catalog/status.json')
s = json.load(open(path))
assert cid in s, cid
s[cid] = {'status': st, 'note': note}
if st == 'ported':
    s[cid]['path'] = f'plugins/{cid}'
json.dump(s, open(path, 'w'), ensure_ascii=False, indent=1, sort_keys=True)
subprocess.run([sys.executable, os.path.join(root, 'tools/catalog.py')], check=True)

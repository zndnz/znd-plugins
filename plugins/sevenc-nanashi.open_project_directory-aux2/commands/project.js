// open_project_directory.aux2（Nanashi.）の移植: プロジェクトファイルのフォルダを開く・パスをコピーする。

function __saved() {
  if (!ctx.projectPath) { host.alert('プロジェクトがまだ保存されていません。'); return null; }
  return String(ctx.projectPath);
}
function __dir(p) { var i = Math.max(p.lastIndexOf('/'), p.lastIndexOf('\\')); return i >= 0 ? p.substring(0, i) : p; }

function openFolder() { if (__saved()) host.openProjectFolder(); }
function copyFolderPath() { var p = __saved(); if (p) { host.copyText(__dir(p)); host.status('フォルダのパスをコピーしました'); } }
function copyFilePath() { var p = __saved(); if (p) { host.copyText(p); host.status('プロジェクトファイルのパスをコピーしました'); } }

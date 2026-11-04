mod common;
use common::*;

#[test]
fn copies_paths_and_opens_folder() {
    let p = solid_project(10, 10, 100, 100, vec![]);
    let mut ctx = context("main", &[]);
    ctx["projectPath"] = serde_json::json!("/tmp/work/a.znd");
    let key = |k: &str| format!("sevenc-nanashi.open_project_directory-aux2/{k}");
    let (_, a) = run_command(&p, &key("copy_dir"), ctx.clone());
    assert!(a.iter().any(|x| x["type"] == "copy_text" && x["text"] == "/tmp/work"));
    let (_, a) = run_command(&p, &key("copy_file"), ctx.clone());
    assert!(a.iter().any(|x| x["type"] == "copy_text" && x["text"] == "/tmp/work/a.znd"));
    let (_, a) = run_command(&p, &key("open"), ctx);
    assert!(a.iter().any(|x| x["type"] == "open_project_folder"));
    let (_, a) = run_command(&p, &key("open"), context("main", &[]));
    assert!(a.iter().any(|x| x["type"] == "alert"), "未保存なら知らせる");
}

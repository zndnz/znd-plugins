mod common;
use common::*;

#[test]
fn calculates() {
    load();
    let p = solid_project(10, 10, 100, 100, vec![]);
    let pan = znd_render::plugins::panels().into_iter().find(|x| x.key == "sagami121.Calculator/calc").unwrap();
    let src = format!("{}\n;function __go() {{ var s = {{}}; ['k1','k2','k+','k3','k×','k2','k='].forEach(function (k) {{ onEvent(k, null, s); }}); return s.display + '|' + s.equation; }}", pan.source);
    let run = znd_script::run_plugin(&p, &src, Some(("__go", &[])), &context("main", &[])).unwrap();
    assert_eq!(run.ret, serde_json::json!("30|15 × 2 ="), "(12 + 3) × 2");
}

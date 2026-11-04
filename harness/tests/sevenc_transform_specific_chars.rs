mod common;
use common::*;
use znd_core::*;

#[test]
fn only_target_characters_are_moved() {
    let p = text_project("aXbX", 40.0, 300, 200);
    let mut state = serde_json::json!({});
    let _ = &mut state;
    // パネルの状態を持たせるため、onEvent を 2 回呼ぶ代わりに、対象文字を入れたラッパーで呼ぶ
    load();
    let pan = znd_render::plugins::panels().into_iter().find(|x| x.key == "sevenc-nanashi.transform_specific_chars-anm2/tsc").unwrap();
    let wrapper = format!("{}\n;function __go() {{ var s = {{ chars: 'X', dy: 50 }}; onEvent('apply', null, s); }}", pan.source);
    let run = znd_script::run_plugin(&p, &wrapper, Some(("__go", &[])), &context("main", &["t"])).unwrap();
    let p = run.project.unwrap();
    let LayerKind::Text { animators, .. } = &p.compositions[0].layers[0].kind else { unreachable!() };
    let TextSelector::Expression { source, .. } = &animators[0].selectors[0] else { unreachable!() };
    assert!(source.starts_with("[1,3]"), "{source}");
}

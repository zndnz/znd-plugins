mod common;
use common::*;
use znd_core::*;

fn eval_text(src: &str, t: f64) -> String {
    load();
    let mut p = text_project("x", 20.0, 300, 100);
    if let LayerKind::Text { text_expression, .. } = &mut p.compositions[0].layers[0].kind {
        *text_expression = Some(Expression { source: src.into(), enabled: true });
    }
    let engine = znd_script::ExprEngine::new();
    let c = p.comp(&CompId::new("main")).unwrap();
    let r = znd_render::Resolver::resolve(&engine, &p, c, Time::from_seconds(t)).unwrap().unwrap();
    match &r.layers[0].kind {
        LayerKind::Text { document, .. } => document.text.clone(),
        _ => unreachable!(),
    }
}

#[test]
fn formats_dates() {
    let s = eval_text("dateTimeText({ start: [2020, 6, 21, 15, 2, 46], form: 1 })", 0.0);
    assert_eq!(s, "2020年06月21日(日) 15時02分46秒");
    let s = eval_text("dateTimeText({ start: [2020, 6, 21, 15, 2, 46], form: 2 })", 0.0);
    assert_eq!(s, "令和02年06月21日(日) 15時02分46秒");
    let s = eval_text("dateTimeText({ start: [2020, 6, 21, 15, 2, 46], form: 6 })", 0.0);
    assert_eq!(s, "03:02:46 p.m.");
    let s = eval_text("dateTimeText({ start: [2020, 6, 21, 0, 0, 0], form: 'hh:nn:ss', speed: 1, speedUnit: 2 })", 2.0);
    assert_eq!(s, "00:02:00", "1 再生秒で 1 分進む");
    let s = eval_text("dateTimeText({ start: [2020, 1, 1, 0, 0, 0], form: 'h:nn:ss', speed: 1, elapsed: true })", 3.0);
    assert_eq!(s, "00:00:03", "経過時間モード（最上位は時間、幅 2）");
}

mod common;
use common::*;
use znd_core::*;

#[test]
fn text_becomes_a_uuid() {
    let p = text_project("x", 20.0, 400, 60);
    let p = panel_event(&p, "Suzuke.IDgen_Su", "idgen", "apply", serde_json::Value::Null, context("main", &["t"]));
    let engine = znd_script::ExprEngine::new();
    let c = p.comp(&CompId::new("main")).unwrap();
    let r = znd_render::Resolver::resolve(&engine, &p, c, Time::ZERO).unwrap().unwrap();
    let LayerKind::Text { document, .. } = &r.layers[0].kind else { unreachable!() };
    let t = &document.text;
    assert_eq!(t.len(), 36, "{t}");
    assert_eq!(t.as_bytes()[14], b'4', "UUIDv4 の版: {t}");
    assert_eq!(t.matches('-').count(), 4);
}

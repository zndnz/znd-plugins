mod common;
use common::*;
use znd_core::*;

#[test]
fn moves_effects_to_adjustment_layer_and_back() {
    load();
    let p = solid_project(10, 10, 100, 100, vec![fx("plugin:CatWithNeko.Gamma", 10, 10)]);
    let key = "sevenc-nanashi.filter_effect_toggle-aux2/toggle";
    let (p, _) = run_command(&p, key, context("main", &["a"]));
    let ls = &p.compositions[0].layers;
    assert_eq!(ls.len(), 2);
    assert!(matches!(ls[0].kind, LayerKind::Adjustment) && ls[0].effects.len() == 1 && ls[1].effects.is_empty());
    assert!(p.validate().is_empty());
    let adj = ls[0].id.0.clone();
    let (p, _) = run_command(&p, key, context("main", &[&adj]));
    let ls = &p.compositions[0].layers;
    assert_eq!(ls.len(), 1);
    assert_eq!(ls[0].effects.len(), 1);
}

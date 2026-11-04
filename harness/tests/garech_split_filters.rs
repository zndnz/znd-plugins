mod common;
use common::*;
use znd_core::*;

#[test]
fn splits_effects_into_adjustment_layers_and_joins_up() {
    load();
    let p = solid_project(10, 10, 100, 100, vec![fx("plugin:CatWithNeko.Gamma", 10, 10), fx("plugin:CatWithNeko.Level", 10, 10)]);
    let (p2, _) = run_command(&p, "Garech.SplitFilters/split", context("main", &["a"]));
    let ls = &p2.compositions[0].layers;
    assert_eq!(ls.len(), 3);
    assert!(ls[..2].iter().all(|l| matches!(l.kind, LayerKind::Adjustment) && l.effects.len() == 1));
    assert_eq!(ls[1].effects[0].kind, "plugin:CatWithNeko.Gamma", "最初のエフェクトが元のレイヤーのすぐ上");
    assert!(p2.validate().is_empty());
    // 上へ結合: 平面を 2 つにして下のエフェクトを上へ
    let mut p3 = p.clone();
    let mut top = p3.compositions[0].layers[0].clone();
    top.id = LayerId::new("top");
    top.name = "top".into();
    top.effects.clear();
    p3.compositions[0].layers.insert(0, top);
    let (p3, _) = run_command(&p3, "Garech.SplitFilters/join_first", context("main", &["a"]));
    assert_eq!((p3.compositions[0].layers[0].effects.len(), p3.compositions[0].layers[1].effects.len()), (1, 1));
}

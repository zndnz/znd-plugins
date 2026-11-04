mod common;
use common::*;
use znd_core::*;

#[test]
fn link_follows_master_color() {
    load();
    let mut p = solid_project(20, 20, 40, 40, vec![]);
    // マスター用のレイヤーを上に足す
    let mut m = p.compositions[0].layers[0].clone();
    m.id = LayerId::new("m");
    m.name = "マスター".into();
    m.effects = vec![fx("plugin:InuyoshiAtom.ColorLink.master", 20, 20).set_c("color_b", [0.0, 1.0, 0.0, 1.0])];
    m.transform.opacity = znd_core::property::Property::constant(0.0);
    p.compositions[0].layers.insert(0, m);
    let (p, _) = run_command(&p, "InuyoshiAtom.ColorLink/link_b", context("main", &["a"]));
    let buf = render_expr(&p, 0.0);
    let c = buf.pixel(20, 20);
    close(c[0], 0.0, 1e-3, "赤");
    close(c[1], 1.0, 1e-3, "緑");
}

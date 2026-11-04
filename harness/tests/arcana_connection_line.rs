mod common;
use common::*;
use znd_core::*;

#[test]
fn command_connects_two_layers() {
    let mut p = solid_project(10, 10, 200, 100, vec![]);
    let mut b = p.compositions[0].layers[0].clone();
    b.id = LayerId::new("b");
    b.name = "b".into();
    b.transform.position = znd_core::property::Property::spatial([170.0, 50.0]);
    p.compositions[0].layers[0].transform.position = znd_core::property::Property::spatial([30.0, 50.0]);
    p.compositions[0].layers.push(b);
    let (p, _) = run_command(&p, "arcana.connection_line/connect", context("main", &["a", "b"]));
    assert_eq!(p.compositions[0].layers.len(), 3);
    let buf = render_expr(&p, 0.0);
    // 2 つの間の真ん中に線がある
    assert!(buf.pixel(100, 50)[3] > 0.9);
    assert!(buf.pixel(100, 20)[3] < 0.1);
}

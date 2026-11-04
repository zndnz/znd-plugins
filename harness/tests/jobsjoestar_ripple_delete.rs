mod common;
use common::*;
use znd_core::*;

#[test]
fn trims_head_and_ripples_following_layers() {
    let mut p = solid_project(10, 10, 100, 100, vec![]);
    let mut b = p.compositions[0].layers[0].clone();
    b.id = LayerId::new("b");
    b.in_point = Time::from_seconds(4.0);
    b.out_point = Time::from_seconds(6.0);
    let a = &mut p.compositions[0].layers[0];
    a.out_point = Time::from_seconds(4.0);
    p.compositions[0].layers.push(b);
    let mut ctx = context("main", &["a"]);
    ctx["time"] = serde_json::json!(1.0);
    let (p, actions) = run_command(&p, "JobsJoestar.RippleDelete/ripple_delete", ctx);
    let ls = &p.compositions[0].layers;
    let s = |t: Time| t.seconds();
    assert_eq!((s(ls[0].in_point), s(ls[0].out_point), s(ls[0].start_time)), (0.0, 3.0, -1.0));
    assert_eq!((s(ls[1].in_point), s(ls[1].out_point)), (3.0, 5.0), "後ろのレイヤーも 1 秒詰める");
    assert!(!actions.is_empty());
}

mod common;
use common::*;
use znd_core::*;

#[test]
fn multiplies_divides_and_shifts_grid() {
    load();
    let mut p = solid_project(10, 10, 100, 100, vec![]);
    p.compositions[0].beat_grid = Some(BeatGrid::new(120.0));
    let pan = znd_render::plugins::panels().into_iter().find(|x| x.key == "Garech.DoubleBPMGrid/bpm").unwrap();
    let go = |p: &Project, body: &str| {
        let src = format!("{}\n;function __go() {{ var s = {{}}; {body} }}", pan.source);
        znd_script::run_plugin(p, &src, Some(("__go", &[])), &context("main", &[])).unwrap().project.unwrap()
    };
    let p = go(&p, "onEvent('mul', null, s);");
    assert_eq!(p.compositions[0].beat_grid.as_ref().unwrap().bpm, 240.0);
    let p = go(&p, "onEvent('ratio', 4, s); onEvent('div', null, s); onEvent('fwd', null, s);");
    let g = p.compositions[0].beat_grid.as_ref().unwrap();
    assert_eq!(g.bpm, 60.0);
    assert!((g.offset.seconds() - 1.0 / 30.0).abs() < 1e-6);
}

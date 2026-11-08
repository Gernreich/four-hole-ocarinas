//! A build as a SolidCraft command script: the same parts the cut files and
//! the 3D view are drawn from, each plate sketched in its own frame and
//! extruded `ply` inward, so `solidcraft-cli run SCRIPT --save X.sc` makes a
//! design that opens in SolidCraft with its parameters live. Prototype on the
//! cube, 2026-11-08.
//!
//! The parameters: `ply`, the sheet's thickness (set it to a measured sheet to
//! see where plates would collide: the joints are drawn for 3 mm, so the
//! fingers do not follow it), and `d_R2`, `d_R1`, `d_L1`, `d_L2`, the finger
//! holes, which do follow. Every point is fixed, so each sketch solves at 0
//! degrees of freedom.
//!
//! The cube is written by parametric_cube() instead, its size live too (see
//! there); script() writes any other build as its fixed parts.
//!
//! cavity_script() is a check run on top of the cube's design, not part of
//! it: the outside as a box, every face plate cut from it, what is left
//! measured.

use crate::ocarina_model as m;
use crate::{Info, OCut, OPart, P2};

fn n(v: f64) -> String {
    let v = if v.abs() < 5e-13 { 0.0 } else { v };
    format!("{v}")
}

fn v3(a: [f64; 3]) -> String {
    format!("[{},{},{}]", n(a[0]), n(a[1]), n(a[2]))
}

fn loop_pts(poly: &[P2]) -> String {
    let mut q: Vec<P2> = poly.to_vec();
    if q.len() > 1 && q[0] == q[q.len() - 1] {
        q.pop();
    }
    q.dedup();
    q.iter().map(|&(x, y)| format!("[{},{}]", n(x), n(y))).collect::<Vec<_>>().join(",")
}

/// The body name SolidCraft gets for a part: its name, made unique by number.
pub fn body_name(p: &OPart) -> String {
    format!("{} #{}", p.name, p.num)
}

pub fn script(kind: &str, parts: &[OPart], info: &Info, diams: &[f64]) -> String {
    let mut c: Vec<String> = vec![];
    let cmd = |name: &str, params: String| format!("{{\"command\":\"{name}\",\"params\":{params}}}");
    c.push(cmd("parameters.add", format!("{{\"name\":\"ply\",\"expression\":\"{} mm\",\"comment\":\"sheet thickness; the joints are drawn for 3 mm\"}}", n(crate::T))));
    for (h, d) in m::HOLES.iter().zip(diams) {
        c.push(cmd("parameters.add", format!("{{\"name\":\"d_{h}\",\"expression\":\"{} mm\",\"comment\":\"finger hole {h}\"}}", n((d * 1e4).round() / 1e4))));
    }
    for p in parts {
        let mm = p.m.expect("every part is placed");
        let col = |j: usize| [mm[0][j], mm[1][j], mm[2][j]];
        let bn = body_name(p);
        c.push(cmd(
            "sketch.create",
            format!("{{\"plane\":{{\"origin\":{},\"x_dir\":{},\"y_dir\":{}}},\"name\":\"{bn} sketch\"}}", v3(col(3)), v3(col(0)), v3(col(1))),
        ));
        let mut lines = 0usize;
        let mut fix: Vec<String> = vec![];
        let mut add_loop = |c: &mut Vec<String>, poly: &[P2]| {
            let pts = loop_pts(poly);
            let k = pts.matches('[').count();
            c.push(cmd("sketch.line", format!("{{\"points\":[{pts}],\"closed\":true}}")));
            for i in 0..k {
                fix.push(format!("\"l{}\"", lines + i + 1));
            }
            lines += k;
        };
        add_loop(&mut c, &p.outline);
        let mut circles = 0usize;
        let mut centres: Vec<String> = vec![];
        for cut in &p.cuts {
            match cut {
                OCut::Rect(q) => add_loop(&mut c, q),
                OCut::Circle(cx, cy, r) => {
                    // a finger hole is the one info.places puts here
                    let hole = info.places.iter().position(|&(_, fi, xy)| Some(fi) == p.face && (xy.0 - cx).abs() < 1e-9 && (xy.1 - cy).abs() < 1e-9);
                    let dia = match hole {
                        Some(k) => format!("\"d_{}\"", m::HOLES[k]),
                        None => format!("\"{} mm\"", n(2.0 * r)),
                    };
                    c.push(cmd("sketch.circle.center", format!("{{\"center\":[{},{}],\"diameter\":{dia}}}", n(*cx), n(*cy))));
                    circles += 1;
                    centres.push(format!("\"c{circles}.center\""));
                }
            }
        }
        fix.extend(centres);
        c.push(cmd("sketch.constraint.fix", format!("{{\"entities\":[{}]}}", fix.join(","))));
        c.push(cmd("sketch.finish", "{}".into()));
        c.push(cmd("solid.extrude", format!("{{\"distance\":\"ply\",\"operation\":\"new\",\"name\":\"{bn}\",\"body_name\":\"{bn}\"}}")));
    }
    let _ = kind;
    format!("{{\"commands\":[\n{}\n]}}\n", c.join(",\n"))
}

/// The cube's air: its outside box, L x W x H as the design's parameters
/// have it, less every face plate, measured. The bird and the chin layer lie
/// outside the box and take no part. Run on top of the parametric design.
pub fn cavity_script(parts: &[OPart]) -> String {
    let tools: Vec<String> = parts.iter().filter(|p| p.face.is_some()).map(|p| format!("\"{}\"", body_name(p))).collect();
    format!(
        "{{\"commands\":[\n{{\"command\":\"solid.box\",\"params\":{{\"corner\":[\"-L/2\",\"-W/2\",\"-H/2\"],\"length\":\"L\",\"width\":\"W\",\"height\":\"H\",\"operation\":\"new\",\"name\":\"Air\",\"body_name\":\"Air\"}}}},\n{{\"command\":\"solid.combine\",\"params\":{{\"target\":\"Air\",\"tools\":[{}],\"operation\":\"cut\",\"keep_tools\":true}}}},\n{{\"command\":\"inspect.measure\",\"params\":{{}}}}\n]}}\n",
        tools.join(",")
    )
}

// --- the cube, parametric ------------------------------------------------------
//
// The same cube with its size live: `L` (mouth to back, along x), `W` (across,
// y), `H` (up, z), `ply`, and the finger holes. The generator's joint rules
// are SolidCraft expressions here, so a bigger wall gets more fingers:
//
// - an edge is cut into as many cells as fit with none under `finger`, odd
//   where the top or the bottom owns both ends (every edge they share), even on
//   the four upright edges, whose ends belong to neither of their plates
//   (polyhedron.rs cells());
// - the top and the bottom own the cells at the ends of their edges, and on an
//   upright edge the plate that owns the bottom cell is the generator's;
// - under the beak the top keeps every cell of its mouth edge (force() in
//   build_faces): the mouth side keeps only the odd cells clear of the beak.
//
// Every plate starts as a full slab and loses the cells its neighbour owns, cut
// as patterned boxes whose count is an expression. The holes, the window, the
// beak, the bird and the chin layer are placed from the mouth edge, as the
// generator places them. What does not follow: the holes' sizes, which the
// acoustic model sets from the cavity (rerun the generator for a new size).

use crate::{BEAK_W, COLUMNS, FINGER, ROW_1, ROW_2};

/// The six faces of an axis-aligned box, by outward normal, with the corner of
/// each edge prism they share. Ends of the edge prism along `axis`.
struct Plate {
    name: String,
    n: [i32; 3],
}

pub fn parametric_cube(parts: &[OPart], info: &Info, diams: &[f64]) -> String {
    let mut c: Vec<String> = vec![];
    let cmd = |name: &str, params: String| format!("{{\"command\":\"{name}\",\"params\":{params}}}");
    let e = crate::cube_edge(crate::CUBE_VOLUME);
    let par = |c: &mut Vec<String>, name: &str, expr: String, unit: Option<&str>, comment: &str| {
        let u = unit.map_or(String::new(), |u| format!(",\"unit\":\"{u}\""));
        c.push(cmd("parameters.add", format!("{{\"name\":\"{name}\",\"expression\":\"{expr}\"{u},\"comment\":\"{comment}\"}}")));
    };
    par(&mut c, "L", format!("{} mm", n(e)), None, "outside, mouth face to back (x)");
    par(&mut c, "W", "L".into(), None, "outside, across (y)");
    par(&mut c, "H", "L".into(), None, "outside, bottom to top (z)");
    par(&mut c, "ply", format!("{} mm", n(crate::T)), None, "sheet thickness");
    par(&mut c, "finger", format!("{} mm", n(FINGER)), None, "the shortest cell along an edge");
    for (h, d) in m::HOLES.iter().zip(diams) {
        par(&mut c, &format!("d_{h}"), format!("{} mm", n((d * 1e4).round() / 1e4)), None, &format!("finger hole {h}; set by the acoustic model for the default size"));
    }
    par(&mut c, "window_w", format!("{} mm", n(m::WINDOW_W)), None, "the window across, the duct's width");
    par(&mut c, "window_l", format!("{} mm", n(m::WINDOW_L)), None, "the window along the jet");
    par(&mut c, "row_1", format!("{} mm", n(ROW_1)), None, "the near holes, in from the mouth wall");
    par(&mut c, "row_2", format!("{} mm", n(ROW_2)), None, "the far holes, in from the mouth wall");
    par(&mut c, "columns", format!("{} mm", n(COLUMNS)), None, "between the two columns of holes");
    for (a, len) in [("X", "L"), ("Y", "W")] {
        let fl = format!("floor({len}/finger + 1e-9)");
        par(&mut c, &format!("cells{a}"), format!("{fl} - 1 + mod({fl}, 2)"), Some(""), &format!("cells on an edge along {a}: odd, the top or the bottom owns both ends"));
    }
    let fl = "floor(H/finger + 1e-9)";
    par(&mut c, "cellsZ", format!("{fl} - mod({fl}, 2)"), Some(""), "cells on an upright edge: even");
    par(&mut c, "keep", format!("floor(floor((W/2 - {} mm)/(W/cellsY) + 1e-9)/2)", n(BEAK_W / 2.0)), Some(""), "the mouth side's cells on each side of the beak");
    par(&mut c, "cavity_cm3", "(L - 2*ply)*(W - 2*ply)*(H - 2*ply)/(1 cm)^3".into(), Some(""), "the air inside in cm3, for reading");

    // the plates, named as the generator names them, found by their normals
    let mut plates: Vec<Plate> = vec![];
    for p in parts.iter().filter(|p| p.face.is_some()) {
        let nv = info.faces[p.face.unwrap()].n;
        plates.push(Plate { name: body_name(p), n: [nv[0].round() as i32, nv[1].round() as i32, nv[2].round() as i32] });
    }
    let by = |nn: [i32; 3]| plates.iter().find(|p| p.n == nn).map(|p| p.name.clone()).expect("a cube face");
    let (xp, xm, yp, ym, zp, zm) = (by([1, 0, 0]), by([-1, 0, 0]), by([0, 1, 0]), by([0, -1, 0]), by([0, 0, 1]), by([0, 0, -1]));
    let half = ["L/2", "W/2", "H/2"];
    let full = ["L", "W", "H"];
    // a slab: full along two axes, ply thick on the third, on the + or - side
    let slab = |c: &mut Vec<String>, name: &str, axis: usize, plus: bool| {
        let mut corner: Vec<String> = (0..3).map(|i| format!("-{}", half[i])).collect();
        let mut size: Vec<String> = (0..3).map(|i| full[i].to_string()).collect();
        if plus {
            corner[axis] = format!("{} - ply", half[axis]);
        }
        size[axis] = "ply".into();
        c.push(cmd(
            "solid.box",
            format!(
                "{{\"corner\":[\"{}\",\"{}\",\"{}\"],\"length\":\"{}\",\"width\":\"{}\",\"height\":\"{}\",\"operation\":\"new\",\"name\":\"{name}\",\"body_name\":\"{name}\"}}",
                corner[0], corner[1], corner[2], size[0], size[1], size[2]
            ),
        ));
    };
    slab(&mut c, &xp, 0, true);
    slab(&mut c, &yp, 1, true);
    slab(&mut c, &zp, 2, true);
    slab(&mut c, &zm, 2, false);
    slab(&mut c, &ym, 1, false);
    slab(&mut c, &xm, 0, false);

    // cells: along `axis`, at the edge prism whose other two corners are
    // `at` (expressions for the low corner on those axes), starting at cell k0
    // and every other cell from there, `count` of them, toward + or -
    let cells = ["cellsX", "cellsY", "cellsZ"];
    let mut nfeat = 0;
    let mut cut = |c: &mut Vec<String>, op: &str, target: &str, axis: usize, at: [&str; 2], k0: &str, count: &str, up: bool| {
        nfeat += 1;
        let w = format!("{}/{}", full[axis], cells[axis]);
        let others: Vec<usize> = (0..3).filter(|&i| i != axis).collect();
        let mut corner = vec![String::new(); 3];
        corner[axis] = format!("-{} + ({k0})*({w})", half[axis]);
        corner[others[0]] = at[0].to_string();
        corner[others[1]] = at[1].to_string();
        let mut size = vec!["ply".to_string(); 3];
        size[axis] = w.clone();
        let name = format!("cells {nfeat}");
        c.push(cmd(
            "solid.box",
            format!(
                "{{\"corner\":[\"{}\",\"{}\",\"{}\"],\"length\":\"{}\",\"width\":\"{}\",\"height\":\"{}\",\"operation\":\"{op}\",\"targets\":[\"{target}\"],\"name\":\"{name}\"}}",
                corner[0], corner[1], corner[2], size[0], size[1], size[2]
            ),
        ));
        let mut dir = [0, 0, 0];
        dir[axis] = if up { 1 } else { -1 };
        c.push(cmd(
            "solid.pattern.rectangular",
            format!("{{\"features\":[\"{name}\"],\"dir1\":[{},{},{}],\"count1\":\"{count}\",\"spacing1\":\"2*({w})\",\"name\":\"{name} pattern\"}}", dir[0], dir[1], dir[2]),
        ));
    };
    // the top's and the bottom's edges, but the top's mouth edge: they own the
    // even cells, ends included
    let lo = |ax: usize, plus: bool| if plus { format!("{} - ply", half[ax]) } else { format!("-{}", half[ax]) };
    for (tb, zplus) in [(&zp, true), (&zm, false)] {
        let z = lo(2, zplus);
        for (side, yplus) in [(&yp, true), (&ym, false)] {
            let y = lo(1, yplus);
            cut(&mut c, "cut", side, 0, [&y, &z], "0", "(cellsX + 1)/2", true);
            cut(&mut c, "cut", tb, 0, [&y, &z], "1", "(cellsX - 1)/2", true);
        }
        for (side, xplus) in [(&xp, true), (&xm, false)] {
            let x = lo(0, xplus);
            if zplus && !xplus {
                continue; // the mouth edge, below
            }
            cut(&mut c, "cut", side, 1, [&x, &z], "0", "(cellsY + 1)/2", true);
            cut(&mut c, "cut", tb, 1, [&x, &z], "1", "(cellsY - 1)/2", true);
        }
    }
    // the mouth edge: the mouth side keeps `keep` odd cells at each end, the
    // top everything else
    {
        let (x, z) = (lo(0, false), lo(2, true));
        c.push(cmd(
            "solid.box",
            format!(
                "{{\"corner\":[\"{x}\",\"-W/2\",\"{z}\"],\"length\":\"ply\",\"width\":\"W\",\"height\":\"ply\",\"operation\":\"cut\",\"targets\":[\"{xm}\"],\"name\":\"mouth edge cleared\"}}"
            ),
        ));
        cut(&mut c, "join", &xm, 1, [&x, &z], "1", "keep", true);
        cut(&mut c, "join", &xm, 1, [&x, &z], "cellsY - 2", "keep", false);
        cut(&mut c, "cut", &zp, 1, [&x, &z], "1", "keep", true);
        cut(&mut c, "cut", &zp, 1, [&x, &z], "cellsY - 2", "keep", false);
    }
    // the upright edges: the plate that owns the bottom cell, as the generator has it
    let upright = [((&xp, true), (&yp, true)), ((&yp, true), (&xm, false)), ((&xm, false), (&ym, false)), ((&ym, false), (&xp, true))];
    for ((own, _), (other, _)) in upright.iter() {
        let pick = |p: &str| -> (usize, bool) {
            if p == xp.as_str() { (0, true) } else if p == xm.as_str() { (0, false) } else if p == yp.as_str() { (1, true) } else { (1, false) }
        };
        let (a, b) = (pick(own), pick(other));
        let (xx, yy) = if a.0 == 0 { (lo(0, a.1), lo(1, b.1)) } else { (lo(0, b.1), lo(1, a.1)) };
        cut(&mut c, "cut", other, 2, [&xx, &yy], "0", "cellsZ/2", true);
        cut(&mut c, "cut", own, 2, [&xx, &yy], "1", "cellsZ/2", true);
    }
    // the beak, the bird and the chin layer: the generator's own outline,
    // mouthpiece(), its tip a fixed shape and its back end on the mouth face
    // (or, for the bird, back to the window's near edge at ply), every point
    // tied to the mouth face so it moves with L
    let piece = |c: &mut Vec<String>, label: &str, z0: &str, x1: &str, op: &str, target: &str| {
        let pts: Vec<String> = crate::mouthpiece(0.0, 0.0)
            .iter()
            .map(|&(x, y)| {
                let xs = if x == 0.0 { format!("-L/2 + {x1}") } else { format!("-L/2 - {}", n(-x)) };
                format!("[\"{xs}\",\"{} mm\"]", n(y))
            })
            .collect();
        c.push(cmd("sketch.create", format!("{{\"plane\":\"XY\",\"offset\":\"{z0}\",\"name\":\"{label} sketch\"}}")));
        c.push(cmd("sketch.line", format!("{{\"points\":[{}],\"closed\":true}}", pts.join(","))));
        c.push(cmd("sketch.finish", "{}".into()));
        let tb = if op == "new" { format!("\"body_name\":\"{label}\"") } else { format!("\"targets\":[\"{target}\"]") };
        c.push(cmd("solid.extrude", format!("{{\"distance\":\"ply\",\"operation\":\"{op}\",{tb},\"name\":\"{label}\"}}")));
    };
    piece(&mut c, "beak", "H/2 - ply", "0 mm", "join", &zp);
    // the window, through the top just inside the mouth wall
    c.push(cmd(
        "solid.box",
        format!("{{\"corner\":[\"-L/2 + ply\",\"-window_w/2\",\"H/2 - ply\"],\"length\":\"window_l\",\"width\":\"window_w\",\"height\":\"ply\",\"operation\":\"cut\",\"targets\":[\"{zp}\"],\"name\":\"window\"}}"),
    ));
    // the finger holes, from the mouth wall's inside face; R on -y (the player
    // sees the other face), as info.places has them
    for (k, h) in m::HOLES.iter().enumerate() {
        let (_, _, xy) = info.places[k];
        let row = if (xy.0 - crate::T - ROW_1).abs() < 1e-9 { "row_1" } else { "row_2" };
        let y = if xy.1 > 0.0 { "-columns/2" } else { "columns/2" }; // the top's frame has v = -y
        c.push(cmd(
            "solid.hole",
            format!("{{\"position\":[\"-L/2 + ply + {row}\",\"{y}\",\"H/2\"],\"diameter\":\"d_{h}\",\"depth\":\"ply\",\"name\":\"hole {h}\"}}"),
        ));
    }
    // the bird on the top, back to the window; the chin layer under the beak
    let named = |p: &str| parts.iter().find(|q| q.name == p).map(body_name).unwrap();
    piece(&mut c, &named("bird"), "H/2", "ply", "new", "");
    piece(&mut c, &named("chin layer"), "H/2 - 2*ply", "0 mm", "new", "");
    format!("{{\"commands\":[\n{}\n]}}\n", c.join(",\n"))
}

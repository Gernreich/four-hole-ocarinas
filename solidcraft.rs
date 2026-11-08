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
//! cavity_script() is a check run on top of the design, not part of it: the
//! solid's outside as a box, every face plate cut from it, what is left
//! measured. Only for a solid whose outside is a box (the cube).

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

/// The cube's air: its outside box less every face plate, measured. The
/// bird and the chin layer lie outside the box and take no part.
pub fn cavity_script(parts: &[OPart], info: &Info) -> String {
    let all: Vec<[f64; 3]> = info.faces.iter().flat_map(|f| f.verts.iter().copied()).collect();
    let lo = |i: usize| all.iter().map(|v| v[i]).fold(f64::INFINITY, f64::min);
    let hi = |i: usize| all.iter().map(|v| v[i]).fold(f64::NEG_INFINITY, f64::max);
    let tools: Vec<String> = parts.iter().filter(|p| p.face.is_some()).map(|p| format!("\"{}\"", body_name(p))).collect();
    format!(
        "{{\"commands\":[\n{{\"command\":\"solid.box\",\"params\":{{\"corner\":[{},{},{}],\"length\":\"{}\",\"width\":\"{}\",\"height\":\"{}\",\"operation\":\"new\",\"name\":\"Air\",\"body_name\":\"Air\"}}}},\n{{\"command\":\"solid.combine\",\"params\":{{\"target\":\"Air\",\"tools\":[{}],\"operation\":\"cut\",\"keep_tools\":true}}}},\n{{\"command\":\"inspect.measure\",\"params\":{{}}}}\n]}}\n",
        n(lo(0)), n(lo(1)), n(lo(2)), n(hi(0) - lo(0)), n(hi(1) - lo(1)), n(hi(2) - lo(2)), tools.join(",")
    )
}

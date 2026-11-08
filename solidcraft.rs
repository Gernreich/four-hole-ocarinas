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
    // the top first: cut after the walls, its duct and bevel leave
    // SolidCraft's boolean with no result
    let mut faced: Vec<&OPart> = parts.iter().filter(|p| p.face.is_some()).collect();
    faced.sort_by_key(|p| p.name != "top plate");
    let tools: Vec<String> = faced.iter().map(|p| format!("\"{}\"", body_name(p))).collect();
    format!(
        "{{\"commands\":[\n{{\"command\":\"solid.box\",\"params\":{{\"corner\":[\"-L/2\",\"-W/2\",\"-H/2\"],\"length\":\"L\",\"width\":\"W\",\"height\":\"H\",\"operation\":\"new\",\"name\":\"Air\",\"body_name\":\"Air\"}}}},\n{{\"command\":\"solid.combine\",\"params\":{{\"target\":\"Air\",\"tools\":[{}],\"operation\":\"cut\",\"keep_tools\":true}}}},\n{{\"command\":\"inspect.measure\",\"params\":{{}}}}\n]}}\n",
        tools.join(",")
    )
}

// --- the cube, parametric ------------------------------------------------------
//
// The cube with everything that follows from a rule written as an expression,
// so changing one number rebuilds the rest:
//
// THE TUNE. ocarina_model.rs's Helmholtz model, in closed form. The window's
// conductance kw = A / (ply + 2 r rayleigh mouth); each hole's conductance is
// its ratio (the fingering search's result, a constant) times kw, and its
// diameter solves pi d^2/4 = k (ply + rayleigh d). The closed note is
// c/2pi sqrt(kw/V). So `key_hz` sets the cavity, and the cavity the size: by
// default the note a 280 cm3 cube sounds, the generator's cube. Type a
// frequency into `key_hz` and the cube resizes to sound it; the holes follow
// the window and ply, not the size, since the scale is ratios of kw.
//
// THE SIZE. `L` (mouth to back, along x), `W` (across, y), `H` (up, z), each
// the cube's edge unless given its own value; `pitch_hz` reads what the box
// actually sounds.
//
// THE JOINTS, the generator's rules:
// - an edge is cut into as many cells as fit with none under `finger`, odd
//   where the top or the bottom owns both ends (every edge they share), even on
//   the four upright edges, whose ends belong to neither of their plates
//   (polyhedron.rs cells());
// - the top and the bottom own the cells at the ends of their edges, and on an
//   upright edge the plate that owns the bottom cell is the generator's;
// - under the beak the top keeps every cell of its mouth edge (force() in
//   build_faces): the mouth side keeps only the odd cells clear of the beak.
// Every plate starts as a full slab and loses the cells its neighbour owns, cut
// as patterned boxes whose count is an expression.
//
// THE MOUTHPIECE. The beak, the bird and the chin layer are the generator's
// mouthpiece() outline, its tip an elliptical quarter each side, with `beak`,
// `beak_w` and `tip` live; the duct is engraved `duct_depth` into the top's
// outside face and the bird's underside, and the window's far edge is filed to
// `bevel`. The corner numbers, single engraved lines, are left out.
//
// THE LIMITS, read-only: `room_far` (past the far holes to the back wall's
// inside face), `room_side` (beside the holes to a side wall's) and `keep` (the
// mouth side's cells each side of the beak) can fall to 0, where the top takes
// the whole mouth edge.

use crate::{BEAK, BEAK_W, BEVEL, COLUMNS, FINGER, ROW_1, ROW_2, TIP_ROUND, TIP_STEPS};

/// The six faces of an axis-aligned box, by outward normal, with the corner of
/// each edge prism they share. Ends of the edge prism along `axis`.
struct Plate {
    name: String,
    n: [i32; 3],
}

pub fn parametric_cube(parts: &[OPart], info: &Info, diams: &[f64]) -> String {
    let mut c: Vec<String> = vec![];
    let cmd = |name: &str, params: String| format!("{{\"command\":\"{name}\",\"params\":{params}}}");
    let par = |c: &mut Vec<String>, name: &str, expr: String, unit: Option<&str>, comment: &str| {
        let u = unit.map_or(String::new(), |u| format!(",\"unit\":\"{u}\""));
        c.push(cmd("parameters.add", format!("{{\"name\":\"{name}\",\"expression\":\"{expr}\"{u},\"comment\":\"{comment}\"}}")));
    };
    let (_, _, ratios, _) = m::design();
    let _ = diams;
    let nu = Some("");
    // the sheet and the air
    par(&mut c, "ply", format!("{} mm", n(crate::T)), None, "sheet thickness; every opening is through it");
    par(&mut c, "finger", format!("{} mm", n(FINGER)), None, "the shortest cell along an edge");
    par(&mut c, "temp_c", n(m::T_C), nu, "the air's temperature, warm from the breath, degrees C");
    par(&mut c, "c_mm_s", "331300*sqrt(1 + temp_c/273.15)".into(), nu, "speed of sound, mm/s");
    par(&mut c, "a4", n(m::A4), nu, "concert pitch, Hz");
    // the window and the holes
    par(&mut c, "window_w", format!("{} mm", n(m::WINDOW_W)), None, "the window across, the duct's width");
    par(&mut c, "window_l", format!("{} mm", n(m::WINDOW_L)), None, "the window along the jet");
    par(&mut c, "mouth", n(m::mouth()), nu, "scales the window's end correction (the generator's --mouth)");
    par(&mut c, "rayleigh", "8/(3*pi)".into(), nu, "end correction per radius, a flanged end");
    par(&mut c, "kw", "window_w*window_l/(ply + 2*rayleigh*sqrt(window_w*window_l/pi)*mouth)".into(), None, "the window's conductance");
    for (h, r) in m::HOLES.iter().zip(&ratios) {
        par(&mut c, &format!("ratio_{h}"), n(*r), nu, &format!("hole {h}'s conductance over the window's: the fingering search's result"));
    }
    for h in m::HOLES {
        let k = format!("(ratio_{h}*kw)");
        par(&mut c, &format!("d_{h}"), format!("({k}*rayleigh + sqrt(({k}*rayleigh)^2 + pi*{k}*ply))/(pi/2)"), None, &format!("finger hole {h}: pi d^2/4 = k (ply + rayleigh d)"));
    }
    // the note, the cavity, the size
    par(&mut c, "key_hz", format!("c_mm_s/(2*pi)*sqrt((kw/(1 mm))/{})", n(crate::CUBE_VOLUME)), nu, "the closed note; by default what a 280 cm3 cavity sounds. Type a frequency to resize");
    par(&mut c, "cavity_mm3", "(kw/(1 mm))*(c_mm_s/(2*pi*key_hz))^2".into(), nu, "the cavity key_hz needs, mm3");
    par(&mut c, "L", "pow(cavity_mm3, 1/3)*(1 mm) + 2*ply".into(), None, "outside, mouth face to back (x): the cube that holds cavity_mm3");
    par(&mut c, "W", "L".into(), None, "outside, across (y)");
    par(&mut c, "H", "L".into(), None, "outside, bottom to top (z)");
    par(&mut c, "pitch_hz", "c_mm_s/(2*pi)*sqrt((kw/(1 mm))/((L - 2*ply)*(W - 2*ply)*(H - 2*ply)/(1 mm)^3))".into(), nu, "read only: what this box sounds closed");
    par(&mut c, "semitones_a4", "12*ln(pitch_hz/a4)/ln(2)".into(), nu, "read only: pitch_hz in semitones from A4 (-7 = D4)");
    // the mouthpiece and the hole layout
    par(&mut c, "beak", format!("{} mm", n(BEAK)), None, "the beak, out from the mouth face");
    par(&mut c, "beak_w", format!("{} mm", n(BEAK_W)), None, "the beak across");
    par(&mut c, "tip", format!("{} mm", n(TIP_ROUND)), None, "the beak tip's rounding, along the beak");
    par(&mut c, "duct_depth", "0.5 mm".into(), None, "each half of the duct, engraved in the top and in the bird");
    par(&mut c, "bevel", format!("{} deg", n(BEVEL)), None, "the window's far edge, filed on the outside face");
    par(&mut c, "row_1", format!("{} mm", n(ROW_1)), None, "the near holes, in from the mouth wall");
    par(&mut c, "row_2", format!("{} mm", n(ROW_2)), None, "the far holes, in from the mouth wall");
    par(&mut c, "columns", format!("{} mm", n(COLUMNS)), None, "between the two columns of holes");
    // the joints
    for (a, len) in [("X", "L"), ("Y", "W")] {
        let fl = format!("floor({len}/finger + 1e-9)");
        par(&mut c, &format!("cells{a}"), format!("{fl} - 1 + mod({fl}, 2)"), nu, &format!("read only: cells on an edge along {a}, odd: the top or the bottom owns both ends"));
    }
    let fl = "floor(H/finger + 1e-9)";
    par(&mut c, "cellsZ", format!("{fl} - mod({fl}, 2)"), nu, "read only: cells on an upright edge, even");
    par(&mut c, "keep", "floor(floor((W/2 - beak_w/2)/(W/cellsY) + 1e-9)/2)".into(), nu, "read only: the mouth side's cells each side of the beak; at 0 the top takes the whole mouth edge");
    // the limits
    par(&mut c, "room_far", "L - 2*ply - row_2 - max(d_R2, d_L2)/2".into(), None, "read only: past the far holes to the back wall; below 0 they run into it");
    par(&mut c, "room_side", "W/2 - ply - columns/2 - max(d_R2, d_R1, d_L1, d_L2)/2".into(), None, "read only: beside the holes to a side wall; below 0 they run into it");

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
    // top everything else. Only cuts, so that keep = 0 still builds: the
    // mouth side loses the even cells at each end (keep + 1 of them, the
    // last already inside the middle) and the whole middle run; the top loses
    // the mouth side's cells, its cutter lifted clear above the top when
    // keep = 0, where a cut that misses is no cut
    {
        let (x, z) = (lo(0, false), lo(2, true));
        cut(&mut c, "cut", &xm, 1, [&x, &z], "0", "keep + 1", true);
        cut(&mut c, "cut", &xm, 1, [&x, &z], "cellsY - 1", "keep + 1", false);
        c.push(cmd(
            "solid.box",
            format!(
                "{{\"corner\":[\"{x}\",\"-W/2 + 2*keep*(W/cellsY)\",\"{z}\"],\"length\":\"ply\",\"width\":\"(cellsY - 4*keep)*(W/cellsY)\",\"height\":\"ply\",\"operation\":\"cut\",\"targets\":[\"{xm}\"],\"name\":\"mouth edge, under the beak\"}}"
            ),
        ));
        let zl = format!("{z} + (1 - min(keep, 1))*4*ply");
        cut(&mut c, "cut", &zp, 1, [&x, &zl], "1", "max(keep, 1)", true);
        cut(&mut c, "cut", &zp, 1, [&x, &zl], "cellsY - 2", "max(keep, 1)", false);
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
    // the beak, the bird and the chin layer: the generator's mouthpiece()
    // outline, its tip TIP_STEPS + 1 points on an elliptical arc `tip` along
    // and `beak_w`/2 across, its back end on the mouth face (or, for the bird,
    // at the window's near edge, ply in), every point an expression
    let tip_pts = |half: &str, back: &str| -> Vec<String> {
        // half: the arc's half-angle; the arc runs from -half to +half
        let mut v = vec![format!("[\"-L/2 + {back}\",\"-beak_w/2*sin({half})\"]")];
        for k in 0..=TIP_STEPS {
            let s = n(2.0 * k as f64 / TIP_STEPS as f64 - 1.0);
            v.push(format!("[\"-L/2 - beak + tip*(1 - cos({half}*{s}))\",\"beak_w/2*sin({half}*{s})\"]"));
        }
        v.push(format!("[\"-L/2 + {back}\",\"beak_w/2*sin({half})\"]"));
        v
    };
    let piece = |c: &mut Vec<String>, label: &str, z0: &str, x1: &str, op: &str, target: &str| {
        let pts = tip_pts("90 deg", x1);
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
    // the duct: the same rounded tip, window_w wide, from the tip back to the
    // window's near edge; half its depth in the top's outside face, half in
    // the bird's underside, both from the plane they meet on
    let bird = named("bird");
    let dpts = tip_pts("asin(window_w/beak_w)", "ply");
    c.push(cmd("sketch.create", "{\"plane\":\"XY\",\"offset\":\"H/2\",\"name\":\"duct sketch\"}".into()));
    c.push(cmd("sketch.line", format!("{{\"points\":[{}],\"closed\":true}}", dpts.join(","))));
    c.push(cmd("sketch.finish", "{}".into()));
    c.push(cmd("solid.extrude", format!("{{\"sketch\":\"duct sketch\",\"distance\":\"duct_depth\",\"direction\":\"negative\",\"operation\":\"cut\",\"targets\":[\"{zp}\"],\"name\":\"duct, top half\"}}")));
    c.push(cmd("solid.extrude", format!("{{\"sketch\":\"duct sketch\",\"distance\":\"duct_depth\",\"operation\":\"cut\",\"targets\":[\"{bird}\"],\"name\":\"duct, bird half\"}}")));
    // the bevel: the window's far edge, sharp at the inside face, filed back
    // ply / tan(bevel) on the outside face; drawn across the window on XZ
    // (sketch y = -z, normal +y) and cut through window_w
    c.push(cmd("sketch.create", "{\"plane\":\"XZ\",\"offset\":\"-window_w/2\",\"name\":\"bevel sketch\"}".into()));
    c.push(cmd(
        "sketch.line",
        "{\"points\":[[\"-L/2 + ply + window_l\",\"-(H/2 - ply)\"],[\"-L/2 + ply + window_l + ply/tan(bevel)\",\"-H/2\"],[\"-L/2 + ply + window_l\",\"-H/2\"]],\"closed\":true}".into(),
    ));
    c.push(cmd("sketch.finish", "{}".into()));
    c.push(cmd("solid.extrude", format!("{{\"distance\":\"window_w\",\"operation\":\"cut\",\"targets\":[\"{zp}\"],\"name\":\"bevel\"}}")));
    format!("{{\"commands\":[\n{}\n]}}\n", c.join(",\n"))
}

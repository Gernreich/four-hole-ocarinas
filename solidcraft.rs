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
    par(&mut c, "cavity_mm3", "(kw/(1 mm))*(c_mm_s/(2*pi*key_hz))^2".into(), nu, "the cavity key_hz needs, mm3 (the generator's --cavity-mm3)");
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

// --- any solid, parametric -------------------------------------------------------
//
// Every other build, scaled: `size` is the build's own measure (an edge, or
// the octagon's flats) and every point of the solid is `s` = size / its
// default times where the generator puts it. What follows the size is the
// generator's rule, as for the cube:
//
// THE PLATES are the face polygons, extruded `ply` inward. Along each edge
// the cells (polyhedron.rs cells()) alternate between its two plates, as
// many as fit with none under `finger`, odd or even as the corners' owners
// ask; each plate loses its neighbour's cells as a strip ply * sin(theta)
// deep, patterned along the edge with an expression count. Where an end cell
// is the neighbour's, the strip runs on past the corner, so that two set-back
// lines meet as outline_full() has them; an end whose owner turns on the
// count's parity is cut or moved clear by an expression. Where more than three
// plates meet, `nip` cuts each corner off square to its bisector.
//
// THE MOUTH. Under the beak the top keeps every cell of its mouth edge (the
// generator's force()); a sloping mouth face stops ply / sin(slope) short of
// the beak there. The beak, the bird, the chin layer, the window, the bevel,
// the duct and the top's holes are the cube's, placed from the top's mouth
// edge.
//
// THE TUNE. The cavity is a cubic in s and ply, fitted exactly to
// polyhedron.rs inner() (the inside moved ply in keeps its shape class, so
// the volume is a polynomial); `key_hz` sets s by four Newton steps, and
// `pitch_hz` reads it back.

use crate::polyhedron::{self as p3, Face, Frame, V3};
use lasermade_tools::py;

fn v2(fr: &Frame, p: V3) -> (f64, f64) {
    (p3::dot(fr.u, p), p3::dot(fr.v, p))
}

fn scaled(faces: &[Face], s: f64) -> Vec<Face> {
    faces.iter().map(|f| Face::new(&f.name, f.n, f.off * s, f.verts.iter().map(|&v| p3::mul(v, s)).collect())).collect()
}

/// The cavity's volume as a*s^3 + b*s^2*ply + c*s*ply^2 + d*ply^3, from
/// inner() at ply = T and four sizes; checked at a fifth.
fn volume_cubic(faces: &[Face]) -> [f64; 4] {
    let t = crate::T;
    let ss = [0.85, 0.95, 1.05, 1.15];
    let v: Vec<f64> = ss.iter().map(|&s| p3::inner(&scaled(faces, s)).0).collect();
    // V(s) = A s^3 + B s^2 + C s + D, solved by Gaussian elimination
    let mut m: Vec<[f64; 5]> = ss.iter().zip(&v).map(|(&s, &v)| [s * s * s, s * s, s, 1.0, v]).collect();
    for i in 0..4 {
        let p = (i..4).max_by(|&a, &b| m[a][i].abs().partial_cmp(&m[b][i].abs()).unwrap()).unwrap();
        m.swap(i, p);
        for j in 0..4 {
            if j != i {
                let f = m[j][i] / m[i][i];
                for k in 0..5 {
                    m[j][k] -= f * m[i][k];
                }
            }
        }
    }
    let x: Vec<f64> = (0..4).map(|i| m[i][4] / m[i][i]).collect();
    let check = p3::inner(&scaled(faces, 1.0)).0;
    let fit = x[0] + x[1] + x[2] + x[3];
    assert!((fit - check).abs() < 1e-6 * check, "the cavity is not a cubic in the size: {fit} against {check}");
    [x[0], x[1] / t, x[2] / (t * t), x[3] / (t * t * t)]
}

pub fn parametric_solid(kind: &str, size_name: &str, size0: f64, parts: &[OPart], info: &Info, diams: &[f64]) -> String {
    let _ = diams;
    let mut c: Vec<String> = vec![];
    let cmd = |name: &str, params: String| format!("{{\"command\":\"{name}\",\"params\":{params}}}");
    let par = |c: &mut Vec<String>, name: &str, expr: String, unit: Option<&str>, comment: &str| {
        let u = unit.map_or(String::new(), |u| format!(",\"unit\":\"{u}\""));
        c.push(cmd("parameters.add", format!("{{\"name\":\"{name}\",\"expression\":\"{expr}\"{u},\"comment\":\"{comment}\"}}")));
    };
    let nu = Some("");
    let faces = &info.faces;
    let (_, _, ratios, _) = m::design();
    let vc = volume_cubic(faces);
    let vexpr = |s: &str| format!("({}*{s}^3 + {}*{s}^2*(ply/(1 mm)) + {}*{s}*(ply/(1 mm))^2 + {}*(ply/(1 mm))^3)", n(vc[0]), n(vc[1]), n(vc[2]), n(vc[3]));
    let dvexpr = |s: &str| format!("(3*{}*{s}^2 + 2*{}*{s}*(ply/(1 mm)) + {}*(ply/(1 mm))^2)", n(vc[0]), n(vc[1]), n(vc[2]));
    // the sheet, the air, the window and the holes: the cube's
    par(&mut c, "ply", format!("{} mm", n(crate::T)), None, "sheet thickness; every opening is through it");
    par(&mut c, "finger", format!("{} mm", n(FINGER)), None, "the shortest cell along an edge");
    par(&mut c, "temp_c", n(m::T_C), nu, "the air's temperature, warm from the breath, degrees C");
    par(&mut c, "c_mm_s", "331300*sqrt(1 + temp_c/273.15)".into(), nu, "speed of sound, mm/s");
    par(&mut c, "a4", n(m::A4), nu, "concert pitch, Hz");
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
    // the note sets the size: Newton on the cavity's cubic
    par(&mut c, "key_hz", format!("c_mm_s/(2*pi)*sqrt((kw/(1 mm))/{})", vexpr("1")), nu, "the closed note; by default what the generator's build sounds. Type a frequency to resize");
    par(&mut c, "cavity_mm3", "(kw/(1 mm))*(c_mm_s/(2*pi*key_hz))^2".into(), nu, "the cavity key_hz needs, mm3");
    par(&mut c, "s0", format!("pow(cavity_mm3/{}, 1/3)", vexpr("1")), nu, "Newton's start");
    for i in 1..=4 {
        let p = format!("s{}", i - 1);
        par(&mut c, &format!("s{i}"), format!("{p} - ({} - cavity_mm3)/{}", vexpr(&p), dvexpr(&p)), nu, "Newton step");
    }
    par(&mut c, size_name, format!("s4*{} mm", n(size0)), None, &format!("the build's {size_name}: the size that holds cavity_mm3; type a length to set it outright"));
    par(&mut c, "s", format!("{size_name}/({} mm)", n(size0)), nu, "the scale, against the generator's build (the generator's --scale)");
    par(&mut c, "pitch_hz", format!("c_mm_s/(2*pi)*sqrt((kw/(1 mm))/{})", vexpr("s")), nu, "read only: what this build sounds closed");
    par(&mut c, "semitones_a4", "12*ln(pitch_hz/a4)/ln(2)".into(), nu, "read only: pitch_hz in semitones from A4");
    par(&mut c, "beak", format!("{} mm", n(BEAK)), None, "the beak, out from the mouth face");
    par(&mut c, "beak_w", format!("{} mm", n(BEAK_W)), None, "the beak across");
    par(&mut c, "tip", format!("{} mm", n(TIP_ROUND)), None, "the beak tip's rounding, along the beak");
    par(&mut c, "duct_depth", "0.5 mm".into(), None, "each half of the duct, engraved in the top and in the bird");
    par(&mut c, "bevel", format!("{} deg", n(BEVEL)), None, "the window's far edge, filed on the outside face");
    par(&mut c, "row_1", format!("{} mm", n(ROW_1)), None, "the near holes, in from the mouth wall");
    par(&mut c, "row_2", format!("{} mm", n(ROW_2)), None, "the far holes, in from the mouth wall");
    par(&mut c, "columns", format!("{} mm", n(COLUMNS)), None, "between the two columns of holes");
    par(&mut c, "nip", format!("{} mm", n(info.nip)), None, "each corner cut back this far, square to its bisector, where more than three plates meet");

    // the room the top's holes leave to the walls: each hole's edge against
    // every side of the top's inside face, that side the neighbour's plane
    // moved ply in, cut by the plane ply under the top
    if !crate::FACE_HOLES.contains(&kind) {
        let top = &faces[info.ft];
        let zt0 = info.fr_top.o[2];
        let xe0 = info.fr_top.o[0];
        let mut terms = vec![];
        for (j, f) in faces.iter().enumerate() {
            if j == info.ft || f.n[2] > 1.0 - 1e-9 {
                continue;
            }
            let touches = f.verts.iter().filter(|v| (v[2] - zt0).abs() < 1e-6).count() >= 2;
            if !touches {
                continue;
            }
            let h = py::hypot(f.n[0], f.n[1]);
            for (k, hn) in m::HOLES.iter().enumerate() {
                let (_, _, xy) = info.places[k];
                let row = if (xy.0 - crate::T - ROW_1).abs() < 1e-9 { "row_1" } else { "row_2" };
                let y = if xy.1 > 0.0 { "(-columns/2)" } else { "(columns/2)" };
                terms.push(format!(
                    "(s*{} mm - ply*{} - ({}*(s*{} mm + ply + {row}) + {}*{y}))/{} - d_{hn}/2",
                    n(f.off - f.n[2] * zt0), n(1.0 - f.n[2]), n(f.n[0]), n(xe0), n(f.n[1]), n(h)
                ));
            }
        }
        let _ = top;
        par(&mut c, "room", format!("min({})", terms.join(", ")), None, "read only: the least web from a hole to a wall's inside face; below 0 the build is too small for the hand");
    }
    // the plates
    let fr_of = |fi: usize| if fi == info.ft { info.fr_top } else { info.frames[fi] };
    let plate_of: Vec<String> = (0..faces.len()).map(|fi| body_name(parts.iter().find(|p| p.face == Some(fi)).unwrap())).collect();
    let plane = |fi: usize| {
        let fr = fr_of(fi);
        format!("{{\"origin\":[0,0,0],\"x_dir\":{},\"y_dir\":{}}},\"offset\":\"-s*{} mm\"", v3(fr.u), v3(fr.v), n(faces[fi].off))
    };
    let pt = |x: String, y: String| format!("[\"{x}\",\"{y}\"]");
    let mut nsk = 0;
    let mut sketch = |c: &mut Vec<String>, fi: usize, pts: &[String]| -> String {
        nsk += 1;
        let name = format!("sk{nsk}");
        c.push(cmd("sketch.create", format!("{{\"plane\":{},\"name\":\"{name}\"}}", plane(fi))));
        c.push(cmd("sketch.line", format!("{{\"points\":[{}],\"closed\":true}}", pts.join(","))));
        c.push(cmd("sketch.finish", "{}".into()));
        name
    };
    for (fi, f) in faces.iter().enumerate() {
        let fr = fr_of(fi);
        let pts: Vec<String> = f.verts.iter().map(|&v| { let (x, y) = v2(&fr, v); pt(format!("s*{} mm", n(x)), format!("s*{} mm", n(y))) }).collect();
        let sk = sketch(&mut c, fi, &pts);
        c.push(cmd("solid.extrude", format!("{{\"sketch\":\"{sk}\",\"distance\":\"ply\",\"operation\":\"new\",\"name\":\"{0}\",\"body_name\":\"{0}\"}}", plate_of[fi])));
    }

    // the cells: each edge walked from the end whose owner is fixed
    let owner_at = |v: V3| -> usize {
        let on: Vec<usize> = (0..faces.len()).filter(|&i| (p3::dot(v, faces[i].n) - faces[i].off).abs() < 1e-6).collect();
        let pick: Vec<usize> = on.iter().copied().filter(|&i| faces[i].name == "top" || faces[i].name == "bottom").collect();
        if pick.is_empty() { on[0] } else { pick[0] }
    };
    let (ft, fm) = (info.ft, info.fm);
    let mut nfeat = 0;
    for e in info.edges.iter() {
        let (a, b) = (e.a, e.b);
        let o = |v: V3| { let x = owner_at(v); if x == a || x == b { Some(x) } else { None } };
        let (ep, eq) = (o(e.p), o(e.q));
        let want = match (ep, eq) { (Some(x), Some(y)) => Some(if x == y { 1 } else { 0 }), (None, None) => Some(0), _ => None };
        // walk from p unless only q's owner is fixed
        let (start, dirv, first) = match (ep, eq) {
            (None, Some(y)) => (e.q, p3::unit(p3::sub(e.p, e.q)), y),
            (Some(x), _) => (e.p, p3::unit(p3::sub(e.q, e.p)), x),
            (None, None) => (e.p, p3::unit(p3::sub(e.q, e.p)), a),
        };
        // with (None, Some) walked from q, index 0 is q's cell: cells() gives
        // it to y when (n - 1 - i) is even, i = n - 1, so y owns the even ones
        // counted from q, as here
        let fl = format!("floor({} mm*s/finger + 1e-9)", n(e.l));
        let ncell = match want {
            Some(1) => format!("({fl} - 1 + mod({fl}, 2))"),
            Some(_) => format!("({fl} - mod({fl}, 2))"),
            None => format!("({fl})"),
        };
        let wexpr = format!("({} mm*s/{ncell})", n(e.l));
        let lexpr = format!("({} mm*s)", n(e.l));
        let mouth_edge = (a == ft && b == fm) || (a == fm && b == ft);
        // the beak's span along the walk, as t0 < t1, and the cells it forces
        let (bt0, bt1) = if mouth_edge {
            let (sy, dy) = (start[1], dirv[1]);
            let u0 = format!("((-beak_w/2 - s*{} mm)/({}))", n(sy), n(dy));
            let u1 = format!("((beak_w/2 - s*{} mm)/({}))", n(sy), n(dy));
            if dy > 0.0 { (u0, u1) } else { (u1, u0) }
        } else {
            (String::new(), String::new())
        };
        let kmin = format!("floor({bt0}/{wexpr})");
        let kmax = format!("(ceil({bt1}/{wexpr}) - 1)");
        for pl in [a, b] {
            let fr = fr_of(pl);
            let s2 = v2(&fr, start);
            let d2 = (p3::dot(fr.u, dirv), p3::dot(fr.v, dirv));
            let cen = v2(&fr, faces[pl].centre);
            let mut nin = (-d2.1, d2.0);
            if (cen.0 - s2.0) * nin.0 + (cen.1 - s2.1) * nin.1 < 0.0 {
                nin = (-nin.0, -nin.1);
            }
            let sin_t = py::msin(py::radians(e.theta));
            let dc = format!("(ply*{})", n(sin_t));
            // a strip from t0 to t1 along the walk, from -ply outside the edge
            // line to depth in, shifted out by `away` (0 to cut, else clear)
            let strip = |t0: &str, t1: &str, depth: &str, away: &str| -> Vec<String> {
                let at = |t: &str, dd: &str| {
                    pt(
                        format!("s*{} mm + ({t})*{} + ({dd} - ({away}))*{}", n(s2.0), n(d2.0), n(nin.0)),
                        format!("s*{} mm + ({t})*{} + ({dd} - ({away}))*{}", n(s2.1), n(d2.1), n(nin.1)),
                    )
                };
                vec![at(t0, "-ply"), at(t1, "-ply"), at(t1, depth), at(t0, depth)]
            };
            let target = plate_of[pl].clone();
            let mut cutter = |c: &mut Vec<String>, pts: Vec<String>, count: Option<(&str, &str)>| {
                let sk = sketch(c, pl, &pts);
                nfeat += 1;
                let name = format!("cells {nfeat}");
                c.push(cmd("solid.extrude", format!("{{\"sketch\":\"{sk}\",\"distance\":\"ply\",\"operation\":\"cut\",\"targets\":[\"{target}\"],\"name\":\"{name}\"}}")));
                if let Some((cnt, spacing)) = count {
                    c.push(cmd(
                        "solid.pattern.rectangular",
                        format!("{{\"features\":[\"{name}\"],\"dir1\":{},\"count1\":\"{cnt}\",\"spacing1\":\"{spacing}\",\"name\":\"{name} pattern\"}}", v3(dirv)),
                    ));
                }
            };
            // the plate's neighbour's cells: odd ones if it owns the even
            let k0 = if pl == first { 1 } else { 0 };
            let base_far = format!("(1 - mod({ncell} - 1 - {k0}, 2))"); // 1 when the last cell is the neighbour's
            let base_start = if k0 == 0 { "1".to_string() } else { "0".to_string() };
            let prog = |c: &mut Vec<String>, cutter: &mut dyn FnMut(&mut Vec<String>, Vec<String>, Option<(&str, &str)>), from: &str, cnt: &str| {
                let away = format!("(1 - min({cnt}, 1))*1000 mm");
                let t0 = format!("({from})*{wexpr}");
                let t1 = format!("({from} + 1)*{wexpr}");
                cutter(c, strip(&t0, &t1, &dc, &away), Some((&format!("max({cnt}, 1)"), &format!("2*{wexpr}"))));
            };
            let (start_cond, far_cond);
            if !mouth_edge {
                prog(&mut c, &mut cutter, &k0.to_string(), &format!("floor(({ncell} - {k0} + 1)/2)"));
                start_cond = base_start.clone();
                far_cond = base_far.clone();
            } else if pl == ft {
                // the mouth face's cells, but none the beak forces to the top
                prog(&mut c, &mut cutter, &k0.to_string(), &format!("max(0, floor(({kmin} - {k0} + 1)/2))"));
                let ka = format!("({kmax} + 1 + mod({kmax} + 1 - {k0}, 2))");
                prog(&mut c, &mut cutter, &ka, &format!("max(0, floor(({ncell} - {ka} + 1)/2))"));
                start_cond = format!("({base_start})*min({kmin}, 1)");
                far_cond = format!("({base_far})*min(max({ncell} - 1 - {kmax}, 0), 1)");
            } else {
                // the top's cells, and every cell the beak forces to it
                prog(&mut c, &mut cutter, &k0.to_string(), &format!("floor(({ncell} - {k0} + 1)/2)"));
                let t0 = format!("max({kmin}, 0)*{wexpr}");
                let t1 = format!("(min({kmax}, {ncell} - 1) + 1)*{wexpr}");
                cutter(&mut c, strip(&t0, &t1, &dc, "0 mm"), None);
                if faces[fm].n[2] > 1e-9 {
                    // a mouth face sloping away under the beak stops short of it
                    let db = format!("(ply/{})", n((1.0 - faces[fm].n[2] * faces[fm].n[2]).sqrt()));
                    cutter(&mut c, strip(&bt0, &bt1, &db, "0 mm"), None);
                }
                start_cond = format!("max({base_start}, 1 - min(max({kmin}, 0), 1))");
                far_cond = format!("max({base_far}, min(max({kmax} - {ncell} + 2, 0), 1))");
            }
            // past each end: where the end cell is the neighbour's, the strip
            // runs on beyond the corner
            let ext = "4*ply";
            if start_cond != "0" {
                let away = format!("(1 - ({start_cond}))*1000 mm");
                cutter(&mut c, strip(&format!("-{ext}"), "0 mm", &dc, &away), None);
            }
            let away = format!("(1 - ({far_cond}))*1000 mm");
            cutter(&mut c, strip(&lexpr, &format!("{lexpr} + {ext}"), &dc, &away), None);
        }
    }
    // the nip: every corner of every plate, where more than three plates meet
    if info.nip != 0.0 {
        for (fi, f) in faces.iter().enumerate() {
            let fr = fr_of(fi);
            let k = f.verts.len();
            let cen = v2(&fr, f.centre);
            for j in 0..k {
                let vv = v2(&fr, f.verts[j]);
                let (pv, nv) = (v2(&fr, f.verts[(j + k - 1) % k]), v2(&fr, f.verts[(j + 1) % k]));
                let un = |a: (f64, f64)| { let l = py::hypot(a.0, a.1); (a.0 / l, a.1 / l) };
                let uc = un((cen.0 - vv.0, cen.1 - vv.1));
                let (e1, e2) = (un((pv.0 - vv.0, pv.1 - vv.1)), un((nv.0 - vv.0, nv.1 - vv.1)));
                let (ka, kb) = (1.0 / (e1.0 * uc.0 + e1.1 * uc.1), 1.0 / (e2.0 * uc.0 + e2.1 * uc.1));
                let p_at = |e: (f64, f64), kk: f64| pt(format!("s*{} mm + nip*{}", n(vv.0), n(kk * e.0)), format!("s*{} mm + nip*{}", n(vv.1), n(kk * e.1)));
                // the cut is the wedge inside the corner's two edges, nearer
                // the corner than nip along its bisector
                let tri = vec![pt(format!("s*{} mm", n(vv.0)), format!("s*{} mm", n(vv.1))), p_at(e1, ka), p_at(e2, kb)];
                let sk = sketch(&mut c, fi, &tri);
                nfeat += 1;
                c.push(cmd("solid.extrude", format!("{{\"sketch\":\"{sk}\",\"distance\":\"ply\",\"operation\":\"cut\",\"targets\":[\"{}\"],\"name\":\"nip {nfeat}\"}}", plate_of[fi])));
            }
        }
    }
    // the mouthpiece, the window, the bevel, the duct and the top's holes,
    // from the top's mouth edge at x = xe, its outside face at z = zt
    let xe = format!("(s*{} mm)", n(info.fr_top.o[0]));
    let zt = format!("(s*{} mm)", n(info.fr_top.o[2]));
    let tip_pts = |half: &str, back: &str| -> Vec<String> {
        let mut v = vec![format!("[\"{xe} + {back}\",\"-beak_w/2*sin({half})\"]")];
        for k in 0..=TIP_STEPS {
            let sx = n(2.0 * k as f64 / TIP_STEPS as f64 - 1.0);
            v.push(format!("[\"{xe} - beak + tip*(1 - cos({half}*{sx}))\",\"beak_w/2*sin({half}*{sx})\"]"));
        }
        v.push(format!("[\"{xe} + {back}\",\"beak_w/2*sin({half})\"]"));
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
    let top = plate_of[ft].clone();
    piece(&mut c, "beak", &format!("{zt} - ply"), "0 mm", "join", &top);
    c.push(cmd(
        "solid.box",
        format!("{{\"corner\":[\"{xe} + ply\",\"-window_w/2\",\"{zt} - ply\"],\"length\":\"window_l\",\"width\":\"window_w\",\"height\":\"ply\",\"operation\":\"cut\",\"targets\":[\"{top}\"],\"name\":\"window\"}}"),
    ));
    if !crate::FACE_HOLES.contains(&kind) {
        for (k, h) in m::HOLES.iter().enumerate() {
            let (_, _, xy) = info.places[k];
            let row = if (xy.0 - crate::T - ROW_1).abs() < 1e-9 { "row_1" } else { "row_2" };
            let y = if xy.1 > 0.0 { "-columns/2" } else { "columns/2" };
            // a cylinder cut, not a hole: one that runs off the top at a small
            // size must not stop the rebuild, and `room` reports it
            c.push(cmd("solid.cylinder", format!("{{\"base\":[\"{xe} + ply + {row}\",\"{y}\",\"{zt} - ply\"],\"diameter\":\"d_{h}\",\"height\":\"ply\",\"operation\":\"cut\",\"targets\":[\"{top}\"],\"name\":\"hole {h}\"}}")));
        }
    }
    if crate::FACE_HOLES.contains(&kind) {
        let got = face_hole_places(info, diams);
        let mut webs = vec![];
        for (k, (fi, x, y, web)) in got.iter().enumerate() {
            let h = m::HOLES[k];
            let mates = got.iter().filter(|g| g.0 == *fi).count();
            // D1, D2: this hole alone, or the pair's near and far holes
            let (d1, d2) = if mates == 1 { (format!("d_{h}"), format!("d_{h}")) } else { (format!("d_{}1", &h[..1]), format!("d_{}2", &h[..1])) };
            // the plate's plane, its circle in place of a polygon
            let sk = sketch(&mut c, *fi, &[]);
            c.pop();
            c.pop();
            c.push(cmd("sketch.circle.center", format!("{{\"center\":[\"{}\",\"{}\"],\"diameter\":\"d_{h}\"}}", lin_expr(x, &d1, &d2), lin_expr(y, &d1, &d2))));
            c.push(cmd("sketch.finish", "{}".into()));
            c.push(cmd("solid.extrude", format!("{{\"sketch\":\"{sk}\",\"distance\":\"ply\",\"operation\":\"cut\",\"targets\":[\"{}\"],\"name\":\"hole {h}\"}}", plate_of[*fi])));
            webs.push(lin_expr(web, &d1, &d2));
        }
        par(&mut c, "room", format!("min({})", webs.join(", ")), None, "read only: the least web from a hole to a wall's inside face, or half the web between two; below 0 the build is too small for the hand");
    }
    let named = |p: &str| parts.iter().find(|q| q.name == p).map(body_name).unwrap();
    let mf = &faces[fm];
    let back = format!(
        "max(0 mm, (s*{} mm - ({}*{xe} + {}*({zt} - 2*ply)))/({}))",
        n(mf.off), n(mf.n[0]), n(mf.n[2]), n(-mf.n[0])
    );
    piece(&mut c, &named("bird"), &zt, "ply", "new", "");
    piece(&mut c, &named("chin layer"), &format!("{zt} - 2*ply"), &format!("-({back})"), "new", "");
    let bird = named("bird");
    let dpts = tip_pts("asin(window_w/beak_w)", "ply");
    c.push(cmd("sketch.create", format!("{{\"plane\":\"XY\",\"offset\":\"{zt}\",\"name\":\"duct sketch\"}}")));
    c.push(cmd("sketch.line", format!("{{\"points\":[{}],\"closed\":true}}", dpts.join(","))));
    c.push(cmd("sketch.finish", "{}".into()));
    c.push(cmd("solid.extrude", format!("{{\"sketch\":\"duct sketch\",\"distance\":\"duct_depth\",\"direction\":\"negative\",\"operation\":\"cut\",\"targets\":[\"{top}\"],\"name\":\"duct, top half\"}}")));
    c.push(cmd("solid.extrude", format!("{{\"sketch\":\"duct sketch\",\"distance\":\"duct_depth\",\"operation\":\"cut\",\"targets\":[\"{bird}\"],\"name\":\"duct, bird half\"}}")));
    c.push(cmd("sketch.create", "{\"plane\":\"XZ\",\"offset\":\"-window_w/2\",\"name\":\"bevel sketch\"}".into()));
    c.push(cmd(
        "sketch.line",
        format!("{{\"points\":[[\"{xe} + ply + window_l\",\"-({zt} - ply)\"],[\"{xe} + ply + window_l + ply/tan(bevel)\",\"-{zt}\"],[\"{xe} + ply + window_l\",\"-{zt}\"]],\"closed\":true}}"),
    ));
    c.push(cmd("sketch.finish", "{}".into()));
    c.push(cmd("solid.extrude", format!("{{\"distance\":\"window_w\",\"operation\":\"cut\",\"targets\":[\"{top}\"],\"name\":\"bevel\"}}")));
    format!("{{\"commands\":[\n{}\n]}}\n", c.join(",\n"))
}

// --- holes on the side faces, as expressions ------------------------------------
//
// deepest() and pair() find their points by a narrowing grid search. Each is a
// linear programme over the face's inside polygon, whose sides are the
// neighbours' planes moved ply in, so each side's offset is linear in s and
// ply: deepest() maximises r with every side at least r away; pair() maximises
// z with each hole's web to every side, and half the web between them, at
// least z. At the optimum some of the constraints hold with equality, and
// solving those gives the point as a linear function of the parameters. The
// active set is found at the generator's size and holds round it.

/// A linear form over the parameters [s, ply, d1, d2, 1].
type Lin = [f64; 5];
const LIN_NAMES: [&str; 5] = ["s", "(ply/(1 mm))", "(D1/(1 mm))", "(D2/(1 mm))", "1"];

fn lin_val(l: &Lin, at: &Lin) -> f64 {
    (0..5).map(|i| l[i] * at[i]).sum()
}

/// Solve the square system rows * x = rhs (rhs linear forms) by Gaussian
/// elimination; None when singular.
fn solve_lin(rows: &[Vec<f64>], rhs: &[Lin]) -> Option<Vec<Lin>> {
    let k = rows.len();
    let mut a: Vec<Vec<f64>> = rows.to_vec();
    let mut b: Vec<Lin> = rhs.to_vec();
    for i in 0..k {
        let p = (i..k).max_by(|&x, &y| a[x][i].abs().partial_cmp(&a[y][i].abs()).unwrap())?;
        if a[p][i].abs() < 1e-9 {
            return None;
        }
        a.swap(i, p);
        b.swap(i, p);
        for j in 0..k {
            if j != i {
                let f = a[j][i] / a[i][i];
                for c in 0..k {
                    a[j][c] -= f * a[i][c];
                }
                for c in 0..5 {
                    b[j][c] -= f * b[i][c];
                }
            }
        }
    }
    Some((0..k).map(|i| { let mut l = b[i]; for c in 0..5 { l[c] /= a[i][i]; } l }).collect())
}

/// Maximise the last variable subject to rows * x <= rhs, by every square
/// active set; the solution as linear forms, at `at`.
fn lp_max(rows: &[Vec<f64>], rhs: &[Lin], at: &Lin) -> Vec<Lin> {
    let mut obj = vec![0.0; rows[0].len()];
    *obj.last_mut().unwrap() = 1.0;
    lp_opt(rows, rhs, at, &obj)
}

/// Maximise obj . x subject to rows * x <= rhs, by every square active set.
fn lp_opt(rows: &[Vec<f64>], rhs: &[Lin], at: &Lin, obj: &[f64]) -> Vec<Lin> {
    let nv = rows[0].len();
    let m = rows.len();
    let mut best: Option<(f64, Vec<Lin>)> = None;
    let mut idx: Vec<usize> = (0..nv).collect();
    loop {
        let r: Vec<Vec<f64>> = idx.iter().map(|&i| rows[i].clone()).collect();
        let h: Vec<Lin> = idx.iter().map(|&i| rhs[i]).collect();
        if let Some(x) = solve_lin(&r, &h) {
            let xv: Vec<f64> = x.iter().map(|l| lin_val(l, at)).collect();
            let ok = (0..m).all(|j| (0..nv).map(|c| rows[j][c] * xv[c]).sum::<f64>() <= lin_val(&rhs[j], at) + 1e-7);
            let val: f64 = (0..nv).map(|c| obj[c] * xv[c]).sum();
            if ok && best.as_ref().map_or(true, |b| val > b.0 + 1e-9) {
                best = Some((val, x));
            }
        }
        // next combination
        let mut i = nv;
        loop {
            if i == 0 {
                return best.expect("a feasible active set").1;
            }
            i -= 1;
            if idx[i] < m - nv + i {
                idx[i] += 1;
                for j in i + 1..nv {
                    idx[j] = idx[j - 1] + 1;
                }
                break;
            }
        }
    }
}

/// The inside polygon of face fi as half-planes n . (X, Y) <= c(s, ply), in
/// the face frame's world-origin coordinates (u . p, v . p).
fn inside_sides(faces: &[Face], fi: usize, fr: &Frame) -> Vec<((f64, f64), Lin)> {
    let f = &faces[fi];
    let w = fr.w;
    let mut out = vec![];
    for (j, g) in faces.iter().enumerate() {
        if j == fi {
            continue;
        }
        let (a, b) = (p3::dot(g.n, fr.u), p3::dot(g.n, fr.v));
        let h = py::hypot(a, b);
        if h < 1e-9 {
            continue;
        }
        let nw = p3::dot(g.n, w);
        // n_j . x <= s off_j - ply, with x = X u + Y v - (s off_i - ply) w
        let c: Lin = [(g.off + f.off * nw) / h, -(1.0 + nw) / h, 0.0, 0.0, 0.0];
        out.push(((a / h, b / h), c));
    }
    out
}

/// Each hole's place on its face, as linear forms (X, Y) in the face frame:
/// deepest() or, for two on one face, pair(), solved exactly.
pub fn face_hole_places(info: &Info, diams: &[f64]) -> Vec<(usize, Lin, Lin, Lin)> {
    let faces = &info.faces;
    let at: Lin = [1.0, crate::T, 0.0, 0.0, 1.0];
    let mut out = vec![];
    for (k, &(h, fi, _)) in info.places.iter().enumerate() {
        let fr = info.frames[fi];
        let sides = inside_sides(faces, fi, &fr);
        let mates: Vec<usize> = info.places.iter().enumerate().filter(|(_, p)| p.1 == fi).map(|(i, _)| i).collect();
        if mates.len() == 1 {
            let rows: Vec<Vec<f64>> = sides.iter().map(|((a, b), _)| vec![*a, *b, 1.0]).collect();
            let rhs: Vec<Lin> = sides.iter().map(|(_, c)| *c).collect();
            let x = lp_max(&rows, &rhs, &at);
            // the web: r less the hole's radius, as D1 is this hole's diameter
            let mut web = x[2];
            web[2] -= 0.5;
            out.push((fi, x[0], x[1], web));
        } else {
            // the first of the pair (h1) is the one nearer the beak: further
            // along -ahead, ahead being world +x in the face frame
            let (h1, h2) = (m::HOLES.iter().position(|x| x.ends_with('1') && x.starts_with(&h[..1])).unwrap(), m::HOLES.iter().position(|x| x.ends_with('2') && x.starts_with(&h[..1])).unwrap());
            let (ax, ay) = (fr.u[0], fr.v[0]);
            let l = py::hypot(ax, ay);
            let (ax, ay) = (ax / l, ay / l);
            let mut at2 = at;
            at2[2] = diams[h1];
            at2[3] = diams[h2];
            let mut rows: Vec<Vec<f64>> = vec![];
            let mut rhs: Vec<Lin> = vec![];
            for ((a, b), c) in &sides {
                let dot = a * ax + b * ay;
                rows.push(vec![*a, *b, -dot, 1.0]);
                let mut r = *c;
                r[2] -= 0.5;
                rhs.push(r);
                rows.push(vec![*a, *b, dot, 1.0]);
                let mut r = *c;
                r[3] -= 0.5;
                rhs.push(r);
            }
            rows.push(vec![0.0, 0.0, -1.0, 1.0]);
            rhs.push([0.0, 0.0, -0.25, -0.25, 0.0]);
            let mut x = lp_max(&rows, &rhs, &at2);
            // where the best is not one point but a run of them (the
            // rhombus): hold z at its best and push the pair each way along
            // every side's direction to find the run's ends
            rows.push(vec![0.0, 0.0, 0.0, -1.0]);
            let mut zr = x[3];
            for c in 0..5 {
                zr[c] = -zr[c];
            }
            rhs.push(zr);
            for ((a, b), _) in &sides {
                let dir = [-*b, *a, 0.0, 0.0];
                let hi = lp_opt(&rows, &rhs, &at2, &dir);
                let lo = lp_opt(&rows, &rhs, &at2, &dir.map(|v| -v));
                let span = (lin_val(&hi[0], &at2) - lin_val(&lo[0], &at2)).abs() + (lin_val(&hi[1], &at2) - lin_val(&lo[1], &at2)).abs();
                if span > 1e-6 {
                    // as far along the run as the generator's search stopped,
                    // so the default build is the cut file's; the fraction
                    // holds as the size changes, and every point of the run
                    // is as good
                    let (ox, oy) = (p3::dot(fr.u, fr.o), p3::dot(fr.v, fr.o));
                    let g = info.places[h1].2;
                    let (gx, gy) = (g.0 + ox, g.1 + oy);
                    let at_c = |l: &Lin| lin_val(l, &at2);
                    let p_of = |x: &[Lin], sg: f64| (at_c(&x[0]) + sg * ax * at_c(&x[2]), at_c(&x[1]) + sg * ay * at_c(&x[2]));
                    let (ph, pl) = (p_of(&hi, -1.0), p_of(&lo, -1.0));
                    let f = if (ph.0 - pl.0).abs() > (ph.1 - pl.1).abs() { (gx - pl.0) / (ph.0 - pl.0) } else { (gy - pl.1) / (ph.1 - pl.1) };
                    let f = f.clamp(0.0, 1.0);
                    x = (0..4).map(|i| { let mut l = hi[i]; for c in 0..5 { l[c] = lo[i][c] + f * (hi[i][c] - lo[i][c]); } l }).collect();
                    break;
                }
            }
            let sgn = if k == h1 { -1.0 } else { 1.0 };
            let mut px = x[0];
            let mut py_ = x[1];
            for c in 0..5 {
                px[c] += sgn * ax * x[2][c];
                py_[c] += sgn * ay * x[2][c];
            }
            out.push((fi, px, py_, x[3]));
        }
    }
    out
}

/// A linear form as an expression, D1 and D2 the two holes' diameters.
fn lin_expr(l: &Lin, d1: &str, d2: &str) -> String {
    let mut t = vec![];
    for i in 0..5 {
        if l[i].abs() > 1e-12 {
            t.push(format!("{}*{}", n(l[i]), LIN_NAMES[i].replace("D1", d1).replace("D2", d2)));
        }
    }
    if t.is_empty() { "0 mm".into() } else { format!("({})*(1 mm)", t.join(" + ")) }
}

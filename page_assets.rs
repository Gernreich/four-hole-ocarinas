//! The published page's moving parts, built from the generator's own
//! build_solid() so the page cannot disagree with the cut files. The writeup
//! is README.md, built into index.html by md2html like every page here; this
//! adds what markdown cannot carry.
//!
//!     target/release/page_assets                      # viewer.js, turn.html and plans/*.svg
//!     target/release/page_assets --artifact OUT.html  # the whole page, one file
//!
//!   viewer.js    the 3D view of every build, loaded on the page through a
//!                page-only line in README.md
//!   turn.html    viewer.js on a page of its own, for the hub to frame
//!   plans/       the top plate drawn to scale, and the fingering chart, as
//!                plain SVG, so they read on GitHub as well as on the page
//!
//! The viewer's styling, its page wrapper and turn.html's are the square
//! flute's, from its page_assets.rs; the script that places the parts is this
//! instrument's own, since the parts are. Python (page_assets.py) until
//! 2026-11-07.
use crate::ocarina_model as m;
use crate::*;
use lasermade_tools::turn3d::{dump_str, Num, J};
use square_flute::page_assets::text as sfp;

const FONT: &str = "font-family=\"Helvetica, Arial, sans-serif\"";
const F: fn(f64) -> Num = Num::F;

fn here() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

fn f3(x: f64) -> J {
    J::N(F(py::round_dp(x, 3)))
}
fn pts3(o: &[P2]) -> J {
    J::A(o.iter().map(|q| J::A(vec![f3(q.0), f3(q.1)])).collect())
}

/// A build's parts for the view: each places itself by p.m, moved so the
/// solid's mouth face sits at x = 0, and comes apart along its face's
/// outward normal; the bird turns over, as the square flute's does.
pub fn solid_solids(parts: &[OPart], info: &Info) -> J {
    let faces = &info.faces;
    let all: Vec<V3> = faces.iter().flat_map(|f| f.verts.iter().copied()).collect();
    let shift = -all.iter().map(|v| v[0]).fold(f64::INFINITY, py::min);
    let zmax = all.iter().map(|v| v[2]).fold(f64::NEG_INFINITY, py::max);
    let zmin = all.iter().map(|v| v[2]).fold(f64::INFINITY, py::min);
    let mut out = vec![];
    for p in parts {
        let mut mm = p.m.unwrap();
        mm[0][3] += shift;
        mm[2][3] -= zmin;
        let mut flip = false;
        let (e, g): (J, &str) = if p.name == "bird" {
            flip = true;
            (J::A(vec![J::N(Num::I(0)), J::N(F(-1.6)), J::N(F(1.6))]), "bird")
        } else if p.name == "chin layer" {
            (J::A(vec![J::N(F(-1.6)), J::N(Num::I(0)), J::N(F(-0.6))]), "bird")
        } else {
            let n = faces[p.face.unwrap()].n;
            (
                J::A(n.iter().map(|x| J::N(F(py::round_dp(1.3 * x, 4)))).collect()),
                if p.name == "top plate" || p.name == "bottom" { "plate" } else { "wall" },
            )
        };
        let holes: Vec<J> = p
            .cuts
            .iter()
            .map(|c| match c {
                OCut::Rect(q) => J::obj(vec![("p", pts3(q))]),
                OCut::Circle(cx, cy, r) => J::obj(vec![("c", J::A(vec![f3(*cx), f3(*cy), f3(*r)]))]),
            })
            .collect();
        let zc = mm[2][3] + mm[2][2] * T / 2.0; // mid-thickness, for the flip
        let mut o = vec![
            ("o", pts3(&p.outline)),
            ("h", J::A(holes)),
            ("m", J::A(mm.iter().map(|r| J::A(r.iter().map(|v| J::N(F(py::round_dp(*v, 4)))).collect())).collect())),
            ("e", e),
            ("g", J::S(g.into())),
            ("name", J::S(p.name.clone())),
            ("f", J::A(p.fills.iter().map(|f| pts3(f)).collect())),
            ("fc", p.ink.map_or(J::Null, |s| J::S(s.into()))),
            ("flip", J::B(flip)),
            ("zc", f3(zc)),
        ];
        if let Some(nums) = &p.numbers {
            // drawn inside face up for the laser; back into the plate's frame
            let st: Vec<J> = inside_up(p)
                .strokes
                .iter()
                .map(|s| J::A(s.iter().map(|q| J::A(vec![f3(q.0), f3(-q.1)])).collect()))
                .collect();
            o.push(("n", J::A(st)));
            o.push(("nl", J::A(nums.iter().map(|(t, c, _)| J::A(vec![J::S(t.clone()), J::N(F(c.0)), J::N(F(c.1))])).collect())));
        }
        out.push(J::obj(o));
    }
    let span = all.iter().map(|v| v[0]).fold(f64::NEG_INFINITY, py::max) + shift;
    J::obj(vec![
        ("parts", J::A(out)),
        ("len", J::N(F(py::round_dp(span, 1)))),
        ("top", J::N(F(py::round_dp(zmax - zmin, 2)))),
        ("cav", J::S(format!("{:.0}cm³", info.v / 1000.0))),
    ])
}

/// The cube's top plate from outside, to scale, beak on the left, as it lies
/// on the sheet: the bird's footprint dashed, the duct in magenta, the
/// window, the four holes with their names and sizes, and the cavity's walls
/// dotted under the plate. Fixed colours on a white ground, like the
/// previews.
pub fn plan_svg(parts: &[OPart], info: &Info, diams: &[f64], title: &str) -> String {
    let tp = &parts[info.ft];
    let us: Vec<f64> = tp.outline.iter().map(|q| q.0).collect();
    let vs: Vec<f64> = tp.outline.iter().map(|q| q.1).collect();
    let (umin, vmin) = (square_flute::fmin(&us), square_flute::fmin(&vs));
    let (x0, y0) = (6.0 - umin, 6.0 - vmin);
    let (w, h) = (square_flute::fmax(&us) - umin + 12.0, square_flute::fmax(&vs) - vmin + 12.0);
    let d = |pts: &[P2]| format!("M {} Z", pts.iter().map(|&(u, v)| format!("{:.2},{:.2}", u + x0, v + y0)).collect::<Vec<_>>().join(" L "));
    let mut s = vec![
        format!("<rect x=\"0\" y=\"0\" width=\"{w:.1}\" height=\"{h:.1}\" fill=\"#ffffff\"/>"),
        format!("<path d=\"{}\" fill=\"#e7dcc4\" stroke=\"#1d232b\" stroke-width=\".4\"/>", d(&tp.outline)),
        format!("<path d=\"{}\" fill=\"none\" stroke=\"#8a7350\" stroke-width=\".5\" stroke-dasharray=\"1 1\"/>", d(&info.inner[info.ft])),
        format!("<path d=\"{}\" fill=\"none\" stroke=\"#1d232b\" stroke-width=\".5\" stroke-dasharray=\"2 1.5\"/>", d(&mouthpiece(T, 0.0))),
        format!("<path d=\"{}\" fill=\"{NEST_INK}\" fill-opacity=\".55\"/>", d(&tp.fills[0])),
    ];
    for c in &tp.cuts {
        if let OCut::Rect(q) = c {
            s.push(format!("<path d=\"{}\" fill=\"#ffffff\" stroke=\"#a85800\" stroke-width=\".6\"/>", d(q)));
        }
    }
    for st in &tp.strokes {
        let p = d(st);
        s.push(format!("<path d=\"{}\" fill=\"none\" stroke=\"{BLUE}\" stroke-width=\".5\"/>", &p[..p.len() - 2]));
    }
    let t = format!("{FONT} text-anchor=\"middle\"");
    for ((hn, _, (u, v)), dia) in info.places.iter().zip(diams) {
        s.push(format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"#ffffff\" stroke=\"#a85800\" stroke-width=\".6\"/>",
            u + x0, v + y0, dia / 2.0
        ));
        let dy = if *v < 0.0 { -dia / 2.0 - 2.0 } else { dia / 2.0 + 5.0 };
        s.push(format!("<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"4\" fill=\"#1d232b\" {t}>{hn} ø{dia:.1}</text>", u + x0, v + y0 + dy));
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w:.0} {h:.0}\">\n<title>{title}</title>\n<desc>Display drawing, not a cut file. The cube's top plate from outside, to scale, beak on the left, its edges cut into fingers: the bird's footprint dashed, the duct under it in magenta, the nest's half engraved in this face and the bird's matching it above, the window, the four finger holes named for their fingers with their diameters, the blue line past the window that marks where the splitting edge's bevel should reach, and the cavity's walls dotted beneath. The right hand's holes are at the bottom of the drawing: the player sees the other face.</desc>\n{}\n</svg>\n",
        s.join("\n")
    )
}

/// The fingering chart: one cell per note of the scale, each the four holes
/// as the player's hands meet them -- left hand on the left, index fingers'
/// row on top, beak up -- drawn in proportion, filled where a finger covers
/// the hole.
pub fn chart_svg(diams: &[f64], title: &str) -> String {
    let scale = 0.55;
    let (cw, ch, top) = (34.0, 46.0, 4.0);
    let pos = |h: &str| match h {
        "L1" => (-8.0, 10.0),
        "L2" => (-8.0, 22.0),
        "R1" => (8.0, 10.0),
        _ => (8.0, 22.0),
    };
    let n = m::NOTES.len() as f64;
    let mut s = vec![format!("<rect x=\"0\" y=\"0\" width=\"{:.0}\" height=\"{ch:.0}\" fill=\"#ffffff\"/>", cw * n)];
    let t = format!("{FONT} text-anchor=\"middle\"");
    for (i, (note, pat)) in m::NOTES.iter().zip(m::PATTERN).enumerate() {
        let cx = cw * i as f64 + cw / 2.0;
        s.push(format!(
            "<rect x=\"{:.1}\" y=\"{top:.1}\" width=\"26\" height=\"28\" rx=\"3\" fill=\"#e7dcc4\" stroke=\"#1d232b\" stroke-width=\".4\"/>",
            cx - 13.0
        ));
        s.push(format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"6\" height=\"3\" fill=\"#b8925c\" stroke=\"#1d232b\" stroke-width=\".3\"/>",
            cx - 3.0,
            top - 3.0
        ));
        for ((h, d), o) in m::HOLES.iter().zip(diams).zip(pat.chars()) {
            let (px, py_) = pos(h);
            let fill = if o == '1' { "#ffffff" } else { "#1d232b" };
            s.push(format!(
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.2}\" fill=\"{fill}\" stroke=\"#1d232b\" stroke-width=\".4\"/>",
                cx + px,
                top + py_,
                py::max(d * scale / 2.0, 1.0)
            ));
        }
        s.push(format!("<text x=\"{cx:.1}\" y=\"{:.1}\" font-size=\"6.5\" fill=\"#1d232b\" {t}>{}</text>", top + 39.0, note.replace('#', "♯")));
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {:.0} {ch:.0}\">\n<title>{title}</title>\n<desc>Display drawing. The G major scale from G4 to G5, one cell per note: the four holes as the player's hands meet them, beak up, left hand on the left, index fingers' holes on the upper row, each drawn in proportion to its size. Black is covered, white is open.</desc>\n{}\n</svg>\n",
        cw * n,
        s.join("\n")
    )
}

pub const MARKUP: &str = "<section class=\"viewer\" id=\"turn\" aria-label=\"Turn it in 3D\">\n  <div class=\"vbar\" role=\"group\" aria-label=\"Build\">__BUTTONS__</div>\n  <div class=\"stage\" id=\"stage\"><canvas id=\"cv\" aria-label=\"The selected build in 3D. Drag to turn; right-drag, shift-drag or two fingers to pan; scroll or pinch to zoom; double-click a part to turn round it.\"></canvas>\n    <p class=\"vinfo\" id=\"vinfo\"></p></div>\n  <div class=\"vctl\">\n    <label for=\"explode\">Explode</label><input id=\"explode\" type=\"range\" min=\"0\" max=\"1\" step=\"0.01\" value=\"0\">\n    <button type=\"button\" id=\"vreset\">Reset view</button>\n    <label class=\"chk\"><input type=\"checkbox\" id=\"xray\"> See through the walls</label>\n    <label class=\"chk\"><input type=\"checkbox\" id=\"nums\" checked> Corner numbers</label>\n  </div>\n  <details class=\"note vhow\"><summary>How to use this view</summary><p>Drag to turn; right-drag, shift-drag or move two fingers to slide the view; scroll or pinch to zoom in on whatever is under the pointer; double-click a part to turn round it, and Reset view to go back. The parts are the generator's own outlines before the kerf, so this is the ocarina as built: five solids, each finger-jointed on every edge at its own angle; the window in the top, the four holes in the top or on faces of their own; and the beak, the bird glued over it and the chin layer under it. The duct is drawn in its two engrave colours, red on the bird and magenta on the top, and exploded, the bird comes out toward you and turns over so its half faces up.</p></details>\n</section>";

const NUM_JS: &str = r#"  let labels = [];
  const numTex = new Map();
  function numLabel(t) {   // a corner number: white on red, facing the camera, a fixed size on screen
    if (!numTex.has(t)) {
      const c = document.createElement('canvas'); c.width = 96; c.height = 64;
      const g = c.getContext('2d');
      g.fillStyle = '#c62020'; g.beginPath(); g.roundRect(4, 4, 88, 56, 14); g.fill();
      g.fillStyle = '#ffffff'; g.font = 'bold 40px system-ui, sans-serif';
      g.textAlign = 'center'; g.textBaseline = 'middle'; g.fillText(t, 48, 34);
      const tex = new THREE.CanvasTexture(c); tex.colorSpace = THREE.SRGBColorSpace; numTex.set(t, tex);
    }
    const sp = new THREE.Sprite(new THREE.SpriteMaterial({ map: numTex.get(t), depthTest: false, sizeAttenuation: false }));
    sp.scale.set(0.054, 0.036, 1); sp.renderOrder = 10;
    return sp;
  }
  function showNums() {    // on the plates whose outside faces the camera; all of them through the walls
    const on = document.getElementById('nums').checked, w = new THREE.Vector3(), n = new THREE.Vector3();
    if (labels.length) scene.updateMatrixWorld(true);
    labels.forEach(sp => {
      if (!on) { sp.visible = false; return; }
      sp.getWorldPosition(w);
      n.setFromMatrixColumn(sp.parent.matrixWorld, 2).negate();
      sp.visible = xr.checked || n.dot(camera.position.clone().sub(w)) > 0;
    });
  }
"#;

fn jstr(s: &str) -> String {
    let mut o = String::new();
    dump_str(s, &mut o);
    o
}

fn changed() -> ! {
    lasermade_tools::die("AssertionError: the square flute's viewer script changed")
}

pub fn viewer_js(three: &[(String, J)]) -> String {
    // the square flute's script, less its three toggles: here a button per
    // build is the only choice. Cut at the toggles' comment, which starts the
    // part this view does not have, and closed with the plain picker below
    let mut js = sfp::JS.split("  // one button per construction").next().unwrap().to_string();
    js = js.replace("'mm · bore ' + d.W.toFixed(2) + ' × ' + d.D.toFixed(2) + 'mm'", "'mm body · cavity ' + d.cav");
    js = js.replace(
        "let yaw = -0.65, pitch = 0.42, dist = 600;\n  const home = { yaw: -0.65, pitch: 0.42, dist: 600 };",
        "let yaw = -0.65, pitch = 0.62, dist = 380;\n  const home = { yaw: -0.65, pitch: 0.62, dist: 380 };",
    );
    // a solid's corner numbers, red lines just inside each plate's inner face
    let a = "      const m = p.m, M = new THREE.Matrix4();";
    if !js.contains(a) {
        changed();
    }
    js = js.replace(
        a,
        &format!("      (p.n || []).forEach(st => holder.add(new THREE.Line(new THREE.BufferGeometry().setFromPoints(st.map(q => new THREE.Vector3(q[0], q[1], 3.05))), numMat)));\n{a}"),
    );
    let b = "  function build(name) {";
    if !js.contains(b) {
        changed();
    }
    js = js.replace(b, &format!("  const numMat = new THREE.LineBasicMaterial({{ color: 0xd02020 }});\n{NUM_JS}{b}"));
    // each number also as a label facing the camera, on the plates turned
    // toward it (all of them, seeing through the walls), off by its checkbox
    let c = "    const d = DATA[name];";
    if !js.contains(c) {
        changed();
    }
    js = js.replace(c, &format!("{c}\n    labels = [];"));
    js = js.replace(
        a,
        &format!("      (p.nl || []).forEach(l => {{ const sp = numLabel(l[0]); sp.position.set(l[1], l[2], -0.6); holder.add(sp); labels.push(sp); }});\n{a}"),
    );
    let r = "    renderer.render(scene, camera);\n  }";
    if js.matches(r).count() != 1 {
        changed();
    }
    js = js.replace(r, &format!("    showNums();\n{r}"));
    if !(js.contains("d.cav") && js.contains("dist = 380")) {
        changed();
    }
    js += "  const bb = [...document.querySelectorAll('#turn .vbar button[data-v]')];\n  bb.forEach(b => b.addEventListener('click', () => {\n    bb.forEach(x => x.setAttribute('aria-pressed', x === b)); build(b.dataset.v);\n  }));\n  document.getElementById('nums').addEventListener('change', render);\n  window.addEventListener('resize', size);\n  build(bb[0].dataset.v); size();\n})();\n";
    let mut data = String::new();
    J::O(three.to_vec()).dump(&mut data);
    js = js.replace("__DATA__", &data);
    let buttons: String = three
        .iter()
        .enumerate()
        .map(|(i, (n, _))| format!("<button type=\"button\" data-v=\"{n}\" aria-pressed=\"{}\"><code>{n}</code></button>", i == 0))
        .collect();
    sfp::VIEWER_JS
        .replace("__CSS__", &jstr(sfp::CSS))
        .replace("__MARKUP__", &jstr(&MARKUP.replace("__BUTTONS__", &buttons)))
        .replace("__JS__", &js)
}

/// index.html with viewer.js inlined and every local image a data URI, one
/// file, without the document skeleton the artifact host supplies; `h` is
/// the repository whose page it is (the octagonal flute's borrows this).
pub fn artifact(h: &str, out: &str) {
    let read = |p: &str| std::fs::read_to_string(format!("{h}/{p}")).unwrap_or_else(|e| lasermade_tools::die(format!("{p}: {e}")));
    let mut page = read("index.html").replace("<script src=\"viewer.js\"></script>", &format!("<script>{}</script>", read("viewer.js")));
    let src = lasermade_tools::re::rx(r#"src="([^"]+\.(?:png|svg))""#);
    page = lasermade_tools::re::sub(&src, &page, |c| {
        let s = &c[1];
        let p = format!("{h}/{s}");
        if s.starts_with("http") || s.starts_with("data:") || !std::path::Path::new(&p).exists() {
            return c[0].to_string();
        }
        let kind = if s.ends_with(".svg") { "image/svg+xml" } else { "image/png" };
        format!("src=\"data:{kind};base64,{}\"", py::b64(&std::fs::read(&p).unwrap()))
    });
    let skel = lasermade_tools::re::rx(r"(?is)<!doctype[^>]*>|</?html[^>]*>|</?body[^>]*>|</?head>");
    page = lasermade_tools::re::sub_str(&skel, &page, "");
    std::fs::write(out, &page).unwrap_or_else(|e| lasermade_tools::die(format!("{out}: {e}")));
    println!("  wrote {out}: {}KB", page.chars().count() / 1024);
}

pub const PLANS: [&str; 2] = ["four-hole-ocarina-top-plate.svg", "four-hole-ocarina-fingering.svg"];

pub fn main() -> i32 {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() == 2 && a[0] == "--artifact" {
        artifact(here(), &a[1]);
        return 0;
    }
    // --viewer-to=FILE, --plans-to=DIR and --turn-to=FILE write one asset
    // where .repro asks, so the reproduction gate compares them like any
    // other file
    let pick = |k: &str| -> Vec<String> { a.iter().filter_map(|x| x.strip_prefix(k).map(String::from)).collect() };
    let (vt, pt, tt) = (pick("--viewer-to="), pick("--plans-to="), pick("--turn-to="));
    if vt.len() + pt.len() + tt.len() != a.len() {
        lasermade_tools::die("usage: page_assets [--viewer-to=FILE] [--plans-to=DIR] [--turn-to=FILE] | --artifact OUT.html");
    }
    let h = here();
    let write = |f: &str, s: &str| std::fs::write(f, s).unwrap_or_else(|e| lasermade_tools::die(format!("{f}: {e}")));
    if !tt.is_empty() || a.is_empty() {
        let page = sfp::TURN_PAGE.replace("<title>Square-bore flute in 3D</title>", "<title>Four-hole ocarinas in 3D</title>");
        if !page.contains("Four-hole ocarinas") {
            lasermade_tools::die("AssertionError: the square flute's turn page changed");
        }
        write(&tt.first().cloned().unwrap_or(format!("{h}/turn.html")), &page);
    }
    let (_, _, _, diams) = m::design();
    let mut three = vec![];
    let mut cube = None;
    for (name, _) in BUILDS {
        let (parts, info) = build_solid(name, &diams).unwrap_or_else(|e| lasermade_tools::die(e));
        three.push((name.to_string(), solid_solids(&parts, &info)));
        if name == "cube" {
            cube = Some((parts, info));
        }
    }
    if !vt.is_empty() || a.is_empty() {
        write(&vt.first().cloned().unwrap_or(format!("{h}/viewer.js")), &viewer_js(&three));
    }
    if !pt.is_empty() || a.is_empty() {
        let d = pt.first().cloned().unwrap_or(format!("{h}/plans"));
        std::fs::create_dir_all(&d).unwrap_or_else(|e| lasermade_tools::die(format!("{d}: {e}")));
        let (parts, info) = cube.unwrap();
        let d = d.trim_end_matches('/');
        write(&format!("{d}/{}", PLANS[0]), &plan_svg(&parts, &info, &diams, "The cube's top plate from outside, to scale"));
        write(&format!("{d}/{}", PLANS[1]), &chart_svg(&diams, "Fingering chart, G4 to G5"));
    }
    println!("  wrote viewer.js: {} builds; plans: {}", three.len(), PLANS.join(", "));
    0
}

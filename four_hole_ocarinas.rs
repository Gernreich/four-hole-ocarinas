//! Cut files for four-hole ocarinas, English fingering, in 3mm ply.
//!
//!     target/release/four_hole_ocarinas --no-write          # the checks, write nothing
//!     target/release/four_hole_ocarinas --out=DIR           # every build into DIR/<build>/
//!     target/release/four_hole_ocarinas --only=cube --out=DIR
//!     target/release/four_hole_ocarinas --only=cube --flat --out=DIR   # no subfolder
//!     target/release/four_hole_ocarinas --mouth=1.2 ...     # the window correction, as the model
//!     target/release/four_hole_ocarinas --kerf=0.18 ...     # for a laser with that kerf
//!
//! Rust since 2026-11-07 (four_hole_ocarinas.py until then; the move changed
//! no byte of any cut file or page). The panpipes and the chanters borrow
//! its mitred offset, and the tube panpipes polyhedron.rs's solid.
//!
//! The acoustics are ocarina_model.rs's: the cavity's volume and the four
//! holes' sizes. This only builds what that asks for. An ocarina's holes go
//! wherever the fingers fall, since a Helmholtz resonator hears their size
//! and not their place, so the layout here is the hands', not the model's.
//!
//! EVERY BUILD IS A SOLID, built by polyhedron.rs and finger-jointed on every
//! edge at whatever angle its faces meet (see BUILDS). Each holds what its
//! shape holds and plays in the key that gives; the model's G4 is only the
//! frame the scale is worked out in. Until 2026-10-18 there was a G4 build, a
//! box 60 x 18mm inside and as long as the volume needed, finger-jointed or
//! laminated as a stack; at the 14 x 7mm window it ran 144mm, near enough a
//! quarter wave to act partly as a tube, and the author made it a cube sized
//! to G4, then the same day a cube of about 280cm^3, as the octahedron.
//!
//! All are tuned at the window and the holes.
//!
//! FROM THE MOUTH END, the same in every build:
//!
//!   beak     the top plate run on past the end, BEAK long and BEAK_W wide:
//!            what the lips close on
//!   bird     one layer glued over the beak and on to the window's near
//!            edge. The duct, the window's width, runs from the beak's tip to
//!            the window as a fill engrave in two inks: BIRD_INK in the bird's
//!            underside, NEST_INK in the top plate's outside face under it, so
//!            either or both can be run (see BIRD_INK)
//!   chin     one plain layer glued under the beak, so the mouthpiece is
//!            three plies thick, 9mm, with the duct near its middle. Until
//!            2026-10-17 the bird was three layers stacked on top, copied from
//!            the square flute, and the beak had nothing under it
//!   window   the true sound hole, WINDOW_W x WINDOW_L, against the cavity's
//!            mouth-end wall, so the jet crosses it over the cavity
//!   holes    on the cube, the two prisms, the antiprism, the cuboctahedron
//!            and the elongated square pyramid, two rows across the top: the
//!            index fingers' row ROW_1 past the window, the middle fingers'
//!            ROW_2 past that, each hand's pair in a column COLUMNS apart;
//!            the solids in FACE_HOLES give them faces of their own (see
//!            build_solid)
//!
//! THE SPLITTING EDGE is the window's far edge, and wants the bevel a laser
//! cannot cut, as on the square flute: file it, about 30 degrees, on the face
//! the bird sits on, before the bird goes on. That face is the outside of the
//! top, and fixes which hand is which: looking at it with the beak up, the
//! right hand's holes are on the LEFT, because the player sees the other face.
//!
//! The finger's size, the kerf and the sheet packing are the square flute's,
//! borrowed from square_flute.rs beside this repository, and the path writer
//! is swept-curve's ribbon_bore.rs. The joint is polyhedron.rs's, cut for each
//! edge's own angle.
pub mod ocarina_model;
pub mod page_assets;
pub mod polyhedron;
pub mod solid_survey;

use lasermade_tools::py;

/// Any simple polygon grown by d on every side (shrunk if d < 0): each
/// vertex moves along its two edges' outward normals, mitred, so both edges
/// move d.
pub fn offset(poly: &[(f64, f64)], d: f64) -> Vec<(f64, f64)> {
    let n = poly.len();
    let mut area = 0.0;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        area += a.0 * b.1 - b.0 * a.1;
    }
    let s = if area > 0.0 { 1.0 } else { -1.0 };
    let normal = |a: (f64, f64), b: (f64, f64)| {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l = py::hypot(dx, dy);
        (s * dy / l, -s * dx / l)
    };
    (0..n)
        .map(|i| {
            let (p0, p1, p2) = (poly[(i + n - 1) % n], poly[i], poly[(i + 1) % n]);
            let (na, nb) = (normal(p0, p1), normal(p1, p2));
            let k = d / (1.0 + na.0 * nb.0 + na.1 * nb.1);
            (p1.0 + k * (na.0 + nb.0), p1.1 + k * (na.1 + nb.1))
        })
        .collect()
}

use ocarina_model as m;
use polyhedron as p3;
use polyhedron::{Edge, Face, Frame, V3};
use ribbon_bore::path;
use square_flute::{kerf, pack, Sheet, BED, BLACK, BLUE, MIN_WEB, ORANGE, T};
use std::collections::BTreeMap;
use std::f64::consts::PI;

pub const FINGER: f64 = square_flute::FINGER;
// The duct is a savanna flue, cut both ways at once: the bird's half in its
// underside and the nest's half in the top plate's outside face, over the
// beak, each in an ink of its own. Run one for a woodland (bird) flue or a
// plains (nest) one, about 1mm deep, or both at about 0.5mm each for a
// savanna flue. The house order has one engrave ink, blue, which the corner
// numbers keep, and green and cyan are cut stages, so these two are new.
pub const BIRD_INK: &str = "#ff0000";
pub const NEST_INK: &str = "#ff00ff";

pub const BEAK: f64 = 18.0;
pub const BEAK_W: f64 = 24.0;
// the beak's front, where the lips go: a half-ellipse across its whole width
// and TIP_ROUND deep, on the top plate, the bird and the chin layer alike, so
// the three plies round off as one (the author, 2026-10-17)
pub const TIP_ROUND: f64 = 8.0;
pub const TIP_STEPS: usize = 32;
// the splitting edge's bevel, filed by hand on the top's outside face; a blue
// line there marks where the bevel meets the face (the author, 2026-10-18)
pub const BEVEL: f64 = 30.0;
// the hole rows, from the cavity's mouth-end wall, and the two hands'
// columns, centre to centre: set to fit the 54mm inside of the G4 cube that
// was a build for a day. The box's were 30, 52 and 36mm
pub const ROW_1: f64 = 20.0;
pub const ROW_2: f64 = 42.0;
pub const COLUMNS: f64 = 34.0;
pub const D_MIN: f64 = 2.5; // a hole that still speaks
pub const D_MAX: f64 = 13.0; // one a finger covers
pub const HOLE_GAP: f64 = 3.0; // ply between neighbouring holes

pub const BUILDS: [(&str, &str); 12] = [
    ("cube", "cube, finger-jointed"),
    ("octagon", "uniform octagonal prism, finger-jointed"),
    ("square-antiprism", "square antiprism, finger-jointed"),
    ("octahedron", "regular octahedron, finger-jointed"),
    ("dodecahedron", "regular dodecahedron, finger-jointed"),
    ("truncated-octahedron", "truncated octahedron, finger-jointed"),
    ("hexagonal-prism", "uniform hexagonal prism, finger-jointed"),
    ("icosahedron", "regular icosahedron, finger-jointed"),
    ("cuboctahedron", "cuboctahedron, finger-jointed"),
    ("rhombic-dodecahedron", "rhombic dodecahedron, finger-jointed"),
    ("elongated-square-pyramid", "elongated square pyramid (Johnson J8), finger-jointed"),
    ("elongated-square-bipyramid", "elongated square bipyramid (Johnson J15), finger-jointed"),
];
// Solids, built by polyhedron.rs, since 2026-10-17. Each cavity is what its
// shape holds, and its key follows from it. The holes are sized against the
// window, not the cavity, so every fingering and every interval is the same
// in all of them; only the key moves.
//   cube            its edge whatever holds CUBE_VOLUME (cube_edge), about
//                   the octahedron's, at the author's asking (2026-10-18)
pub const CUBE_VOLUME: f64 = 280e3;
//   octagon         OCT_FLATS across the flats, outside, and as tall as a
//                   wall is wide, so every wall is square and the prism is
//                   uniform, as the cube is; it was 36mm tall until 2026-10-18
pub const OCT_FLATS: f64 = 110.0;
pub fn oct_height() -> f64 {
    OCT_FLATS * py::mtan(PI / 8.0)
}
// The other solids, the antiprism's and the dodecahedron's edges a whole
// number of 6mm fingers:
//   square-antiprism  72mm, so the top square takes the window and the four
//                   holes on the cube's grid, ROW_1 and the rest; its square
//                   is a quarter turn from the one below it, with eight
//                   triangles between
//   octahedron      standing on a triangle, every edge OCTA_EDGE, sized for a
//                   cavity of about 280cm^3 at the author's asking
//                   (2026-10-18; 72mm and 127cm^3 until then); the top is too
//                   small for four holes, so each hand has a triangle of its
//                   own, the index finger's hole nearer the beak
//   dodecahedron    standing on a pentagon, every edge DODECA_EDGE; each hole
//                   has a pentagon of its own, the index fingers' the two
//                   either side of the mouth, the middle fingers' the next
// The four the author asked for next (2026-10-18), each at about 280cm^3 or
// as much more as its holes need:
//   truncated-octahedron  standing on a square, which takes only the window;
//                   each hand's two holes share the hexagon on its side
//   hexagonal-prism uniform, as the octagon is: square walls, the holes on
//                   the top's grid
//   icosahedron     standing on a triangle, each hole on a triangle of its own
//   cuboctahedron   standing on a square, the holes on its grid; 54.5mm, not
//                   the 53.2mm of 280cm^3, so the biggest hole keeps 2mm of ply
//                   to a wall after the millimetre it may be filed up
// Three from a survey of zonohedra and Johnson solids (2026-11-04, the
// author's pick of the ones that passed every check here at 280cm^3):
//   rhombic-dodecahedron  a zonohedron standing on a rhombus, every joint 120
//                   degrees; the top takes the window, and each hand's two
//                   holes share the rhombus on its side, 22.7mm apart, the
//                   nearest of any build with holes off the top to the grid's
//                   22mm
//   elongated-square-pyramid  a cube with a square pyramid under it, hung
//                   point down from its square top, which takes the holes on
//                   the cube's grid
//   elongated-square-bipyramid  a cube with a pyramid on each end, standing
//                   on a triangle; each hand's two holes share the triangle on
//                   its side, 20mm apart, and the window is 38mm from the
//                   furthest hole, the closest of all
// Four plates meet at the antiprism's, the octahedron's and the
// cuboctahedron's corners, at some of the rhombic dodecahedron's and the two
// Johnson solids', and five at the icosahedron's, and NIP cuts each corner
// back that far, square to its bisector, so that the plates sharing only the
// corner stay apart: half a millimetre over the least that clears.
pub const SAP_EDGE: f64 = 72.0;
pub const OCTA_EDGE: f64 = 91.5;
pub const DODECA_EDGE: f64 = 36.0;
pub const HEX_EDGE: f64 = 52.0;
pub const ICOSA_EDGE: f64 = 54.5;
pub const TRUNC_OCTA_EDGE: f64 = 31.6;
pub const CUBOCTA_EDGE: f64 = 54.5;
pub const RHOMBIC_EDGE: f64 = 48.6;
pub const ESP_EDGE: f64 = 66.4;
pub const ESB_EDGE: f64 = 62.6;
pub fn nip_of(kind: &str) -> f64 {
    match kind {
        "square-antiprism" => 4.25,
        "octahedron" => 4.75,
        "icosahedron" => 3.5,
        "cuboctahedron" => 3.5,
        "rhombic-dodecahedron" => 3.5,
        "elongated-square-pyramid" => 4.25,
        "elongated-square-bipyramid" => 4.5,
        _ => 0.0,
    }
}
pub const FACE_HOLES: [&str; 6] =
    ["octahedron", "dodecahedron", "icosahedron", "truncated-octahedron", "rhombic-dodecahedron", "elongated-square-bipyramid"];

/// The cube's outside edge whose cavity holds V: two plates more than the
/// inside's cube root.
pub fn cube_edge(v: f64) -> f64 {
    py::powf(v, 1.0 / 3.0) + 2.0 * T
}

pub type P2 = (f64, f64);

pub fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<P2> {
    vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
}

/// A polygon with its two corners at the beak's tip, x = -BEAK, swapped for
/// the half-ellipse there: the tip's TIP_ROUND deep and BEAK_W/2 across from
/// the centre line, sampled TIP_STEPS times over the half. A corner inside
/// the beak's width -- the duct's -- lands on the curve where its own y meets
/// it, so the duct's mouth follows the tip's.
pub fn rounded(poly: &[P2], yc: f64) -> Vec<P2> {
    let poly: Vec<P2> = poly.iter().map(|&(x, y)| if (x + BEAK).abs() < 1e-6 { (-BEAK, y) } else { (x, y) }).collect();
    let mut ix: Vec<usize> = (0..poly.len()).filter(|&i| poly[i].0 == -BEAK).collect();
    let n = poly.len() as i64;
    assert!(
        ix.len() == 2 && [1, n - 1].contains(&(ix[1] as i64 - ix[0] as i64).rem_euclid(n)),
        "the beak's tip must be one edge of the polygon"
    );
    if ix[1] - ix[0] != 1 {
        // the tip edge wraps round the end
        ix.reverse();
    }
    let (i, j) = (ix[0], ix[1]);
    let b = BEAK_W / 2.0;
    let (t0, t1) = (((poly[i].1 - yc) / b).asin(), ((poly[j].1 - yc) / b).asin());
    let arc: Vec<P2> = (0..=TIP_STEPS)
        .map(|k| {
            let t = t0 + (t1 - t0) * k as f64 / TIP_STEPS as f64;
            (-BEAK + TIP_ROUND * (1.0 - py::mcos(t)), yc + b * py::msin(t))
        })
        .collect();
    if j == 0 {
        // the curve closes the polygon
        return [&poly[1..i], &arc[..]].concat();
    }
    [&poly[..i], &arc[..], &poly[j + 1..]].concat()
}

/// The bird's or the chin layer's outline: the beak's width, from its
/// rounded tip back to x1.
pub fn mouthpiece(x1: f64, yc: f64) -> Vec<P2> {
    rounded(&rect(-BEAK, yc - BEAK_W / 2.0, x1, yc + BEAK_W / 2.0), yc)
}

/// The duct, from the rounded tip to the window's near edge at c0.
pub fn duct(c0: f64, yc: f64) -> Vec<P2> {
    rounded(&rect(-BEAK, yc - m::WINDOW_W / 2.0, c0, yc + m::WINDOW_W / 2.0), yc)
}

// --- parts ---------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum OCut {
    Rect(Vec<P2>),
    Circle(f64, f64, f64),
}

/// A corner number: its text, centre and up, in the plate's frame.
pub type Number = (String, P2, P2);

/// A part: the square flute's, with what an ocarina plate carries besides.
#[derive(Clone, Debug)]
pub struct OPart {
    pub name: String,
    pub num: usize,
    pub note: String,
    pub outline: Vec<P2>,
    pub cuts: Vec<OCut>,
    pub fills: Vec<Vec<P2>>,
    pub ink: Option<&'static str>,
    pub strokes: Vec<Vec<P2>>,
    pub m: Option<[[f64; 4]; 3]>,
    pub face: Option<usize>,
    pub numbers: Option<Vec<Number>>,
    pub x0: f64,
    pub y0: f64,
    pub w: f64,
    pub h: f64,
}

impl OPart {
    pub fn new(name: &str, num: usize, outline: Vec<P2>, note: &str) -> OPart {
        let xs: Vec<f64> = outline.iter().map(|q| q.0).collect();
        let ys: Vec<f64> = outline.iter().map(|q| q.1).collect();
        let (x0, y0) = (square_flute::fmin(&xs), square_flute::fmin(&ys));
        let (w, h) = (square_flute::fmax(&xs) - x0, square_flute::fmax(&ys) - y0);
        OPart {
            name: name.into(), num, note: note.into(), outline, cuts: vec![], fills: vec![], ink: None, strokes: vec![],
            m: None, face: None, numbers: None, x0, y0, w, h,
        }
    }
    pub fn hole(&mut self, cx: f64, cy: f64, d: f64) {
        self.cuts.push(OCut::Circle(cx, cy, d / 2.0));
    }
    pub fn slot(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        self.cuts.push(OCut::Rect(rect(x0, y0, x1, y1)));
    }
}

// --- checks -----------------------------------------------------------------

/// Even-odd point in polygon.
pub fn inside(pt: P2, poly: &[P2]) -> bool {
    let (x, y) = pt;
    let mut c = false;
    let n = poly.len();
    for i in 0..n {
        let ((x0, y0), (x1, y1)) = (poly[i], poly[(i + 1) % n]);
        if (y0 > y) != (y1 > y) && x < x0 + (y - y0) * (x1 - x0) / (y1 - y0) {
            c = !c;
        }
    }
    c
}

/// The plates whose solid holds the world point q: q taken back into each
/// plate's own frame through its matrix, whose axes are orthonormal, so its
/// transpose undoes it.
pub fn owners(parts: &[OPart], q: V3) -> usize {
    let mut hit = 0;
    for p in parts {
        let Some(m) = p.m else { continue };
        let d = [q[0] - m[0][3], q[1] - m[1][3], q[2] - m[2][3]];
        let uvw = |j: usize| m[0][j] * d[0] + m[1][j] * d[1] + m[2][j] * d[2];
        let (u, v, w) = (uvw(0), uvw(1), uvw(2));
        if 0.0 < w && w < T && inside((u, v), &p.outline) {
            hit += 1;
        }
    }
    hit
}

// --- the solids ----------------------------------------------------------------

pub fn faces_of(kind: &str) -> Vec<Face> {
    match kind {
        "octagon" => p3::octagonal_prism(OCT_FLATS, oct_height()),
        "cube" => p3::cube(cube_edge(CUBE_VOLUME)),
        "square-antiprism" => p3::square_antiprism(SAP_EDGE),
        "octahedron" => p3::octahedron(OCTA_EDGE),
        "dodecahedron" => p3::dodecahedron(DODECA_EDGE),
        "truncated-octahedron" => p3::truncated_octahedron(TRUNC_OCTA_EDGE),
        "hexagonal-prism" => p3::hexagonal_prism(HEX_EDGE),
        "icosahedron" => p3::icosahedron(ICOSA_EDGE),
        "cuboctahedron" => p3::cuboctahedron(CUBOCTA_EDGE),
        "rhombic-dodecahedron" => p3::rhombic_dodecahedron(RHOMBIC_EDGE),
        "elongated-square-pyramid" => p3::elongated_square_pyramid(ESP_EDGE),
        "elongated-square-bipyramid" => p3::elongated_square_bipyramid(ESB_EDGE),
        _ => lasermade_tools::die(format!("KeyError: '{kind}'")),
    }
}

/// What build_solid() says about a build beside its parts.
#[derive(Clone, Debug)]
pub struct Info {
    pub faces: Vec<Face>,
    pub edges: Vec<Edge>,
    pub cellmap: Vec<Vec<(f64, f64, usize)>>,
    pub ft: usize,
    pub fm: usize,
    pub v: f64,
    pub inner: Vec<Vec<P2>>,
    /// each hole, in HOLES order: the face it is on and where
    pub places: Vec<(&'static str, usize, P2)>,
    pub fr_top: Frame,
    pub frames: Vec<Frame>,
    pub back: f64,
    pub nip: f64,
    pub inner_pts: Vec<V3>,
}

/// Every part of a solid build, from its faces (faces_of, unless the survey
/// poses a candidate as one). The world is the solid's own: z up, the hole
/// face on top, the mouth face looking down -x, so the beak runs out along
/// -x from the middle of the top's mouth edge. Each part carries its frame as
/// p.m, the matrix from its own (u, v, inward) to the world.
pub fn build_solid(kind: &str, diams: &[f64]) -> Result<(Vec<OPart>, Info), String> {
    build_faces(kind, faces_of(kind), diams, FACE_HOLES.contains(&kind), nip_of(kind))
}

pub fn build_faces(kind: &str, faces: Vec<Face>, diams: &[f64], face_holes: bool, nip: f64) -> Result<(Vec<OPart>, Info), String> {
    let _ = kind;
    let ft = faces.iter().position(|f| f.name == "top").ok_or("StopIteration")?;
    let fm = faces.iter().position(|f| f.name.starts_with("mouth")).ok_or("StopIteration")?;
    let (top, mouth) = (faces[ft].clone(), faces[fm].clone());
    let x_edge = top.verts.iter().map(|v| v[0]).fold(f64::INFINITY, py::min); // the top's edge with the mouth face
    let z_top = top.off;

    // The face that owns a corner: the top or the bottom where the corner is
    // on one, as every corner is but the dodecahedron's middle ten, and there
    // the first face found.
    let owner_at = |v: V3| -> usize {
        let on: Vec<usize> = (0..faces.len()).filter(|&i| (p3::dot(v, faces[i].n) - faces[i].off).abs() < 1e-6).collect();
        let pick: Vec<usize> = on.iter().copied().filter(|&i| faces[i].name == "top" || faces[i].name == "bottom").collect();
        assert!(pick.len() <= 1, "corner {v:?} has {} owners", pick.len());
        if pick.is_empty() { on[0] } else { pick[0] }
    };
    // Under the beak the top keeps every cell, so the beak runs on from solid
    // ply and the duct crosses no seam.
    let force = |e: &Edge, s0: f64, s1: f64, o: usize| -> usize {
        if !((e.a == ft && e.b == fm) || (e.a == fm && e.b == ft)) {
            return o;
        }
        let d = p3::unit(p3::sub(e.q, e.p));
        let (a, b) = (e.p[1] + s0 * d[1], e.p[1] + s1 * d[1]);
        let (y0, y1) = if a <= b { (a, b) } else { (b, a) };
        if y1 > -BEAK_W / 2.0 && y0 < BEAK_W / 2.0 { ft } else { o }
    };
    let es = p3::edges(&faces);
    let cellmap: Vec<Vec<(f64, f64, usize)>> = es.iter().map(|e| p3::cells(e, owner_at, Some(&force))).collect();
    let mut parts: Vec<OPart> = vec![];
    let mut num = 0;
    let fr_top = p3::frame(&top, Some([1.0, 0.0, 0.0]), Some([x_edge, 0.0, z_top]));
    let mut frames = vec![];
    // the mouth edge of a face: the edge along x = x_edge (and on the top's
    // plane, for the mouth face), and the beak's span along it
    let beak_span = |f: &Face, on_top: bool| -> (f64, f64, usize) {
        let k = f.verts.len();
        let j = (0..k)
            .find(|&j| {
                let (a, b) = (f.verts[j], f.verts[(j + 1) % k]);
                (a[0] - x_edge).abs() < 1e-6 && (b[0] - x_edge).abs() < 1e-6 && (!on_top || ((a[2] - z_top).abs() < 1e-6 && (b[2] - z_top).abs() < 1e-6))
            })
            .unwrap();
        let (a, b) = (f.verts[j], f.verts[(j + 1) % k]);
        let sy = (b[1] - a[1]) / p3::dist(a, b);
        let (u, v) = ((-BEAK_W / 2.0 - a[1]) / sy, (BEAK_W / 2.0 - a[1]) / sy);
        let (b0, b1) = if u <= v { (u, v) } else { (v, u) };
        (b0, b1, j)
    };
    for (i, f) in faces.iter().enumerate() {
        let fr = if i == ft { fr_top } else { p3::frame(f, None, None) };
        frames.push(fr);
        let mut beak = None;
        if i == ft {
            // the top's mouth edge, and the beak's span along it
            let (b0, b1, j) = beak_span(f, false);
            beak = Some((b0, b1, BEAK, j));
        }
        if i == fm && mouth.n[2] > 1e-9 {
            // a mouth face that slopes away under the beak stops short of it
            // by T / sin of the slope, so its edge clears the beak's
            // underside, where an upright one stops T short like any other
            let (b0, b1, j) = beak_span(f, true);
            beak = Some((b0, b1, -T / (1.0 - py::powf(mouth.n[2], 2.0)).sqrt(), j));
        }
        let mut o = p3::outline_full(i, f, &fr, &es, &cellmap, beak, nip);
        if i == ft {
            o = rounded(&o, 0.0);
        }
        let mut note = if f.name == "top" {
            format!(
                "the window{}; file the window's far edge to a 30 degree bevel on the outside face, back to the blue line, before the bird goes on",
                if face_holes { "" } else { " and the four holes" }
            )
        } else {
            "fingers on every edge".to_string()
        };
        if nip != 0.0 {
            note += ", the corners nipped";
        }
        num += 1;
        let mut p = OPart::new(if f.name != "top" { &f.name } else { "top plate" }, num, o, &note);
        p.m = Some(p3::matrix(&fr));
        p.face = Some(i);
        parts.push(p);
    }
    {
        let tp = &mut parts[ft];
        // the window against the mouth face's inner side, the holes from
        // there, the right hand on the player's right: holes face forward
        // (+z), the beak is up (-x), so right = forward x up = -y. On the
        // sheet, drawn from outside with v = -y, the right hand's holes are
        // at v > 0
        tp.slot(T, -m::WINDOW_W / 2.0, T + m::WINDOW_L, m::WINDOW_W / 2.0);
        // a BEVEL through the plate, sharp at the inside face, reaches the
        // outside face T / tan BEVEL past the window's far edge: file to there
        let gx = T + m::WINDOW_L + T / py::mtan(py::radians(BEVEL));
        tp.strokes = vec![vec![(gx, -m::WINDOW_W / 2.0), (gx, m::WINDOW_W / 2.0)]];
        tp.note += ", the nest's half of the duct fill-engraved in its outside face";
        tp.fills.push(duct(T, 0.0));
        tp.ink = Some(NEST_INK);
    }
    let (vol, polys) = p3::inner(&faces);
    let mut places: Vec<(&'static str, usize, P2)> = vec![];
    if face_holes {
        // the faces round the top on either side of it, left hand +y, right
        // -y, nearest the beak first: the index finger's hole on the first
        // and the middle finger's on the next, each where it is furthest from
        // every wall of its face's inside, or both on one face where a hand
        // has only one
        let up: Vec<usize> = (0..faces.len())
            .filter(|&i| faces[i].n[2] > 1e-9 && faces[i].name != "top" && !faces[i].name.starts_with("mouth"))
            .collect();
        let dia = |h: &str| diams[m::HOLES.iter().position(|x| *x == h).unwrap()];
        let mut got: Vec<(&'static str, usize, P2)> = vec![];
        for (hand, sgn) in [("L", 1.0), ("R", -1.0)] {
            let mut mine: Vec<usize> = up.iter().copied().filter(|&i| faces[i].centre[1] * sgn > 1.0).collect();
            mine.sort_by(|&a, &b| faces[a].centre[0].partial_cmp(&faces[b].centre[0]).unwrap());
            let (h1, h2) = if hand == "L" { ("L1", "L2") } else { ("R1", "R2") };
            if mine.len() >= 2 {
                for (h, i) in [(h1, mine[0]), (h2, mine[1])] {
                    let poly: Vec<P2> = polys[i].iter().map(|&c| p3::to2d(&frames[i], c)).collect();
                    got.push((h, i, deepest(&poly)));
                }
            } else {
                let i = *mine.first().ok_or("IndexError: list index out of range")?;
                let c = p3::to2d(&frames[i], faces[i].centre);
                let ahead = p3::to2d(&frames[i], p3::add(faces[i].centre, [1.0, 0.0, 0.0]));
                let poly: Vec<P2> = polys[i].iter().map(|&q| p3::to2d(&frames[i], q)).collect();
                let (xy1, xy2) = pair(&poly, (ahead.0 - c.0, ahead.1 - c.1), dia(h1), dia(h2));
                got.push((h1, i, xy1));
                got.push((h2, i, xy2));
            }
        }
        for h in m::HOLES {
            let g = got.iter().find(|g| g.0 == h).unwrap();
            places.push(*g);
        }
    } else {
        places = vec![
            ("R2", ft, (T + ROW_2, COLUMNS / 2.0)),
            ("R1", ft, (T + ROW_1, COLUMNS / 2.0)),
            ("L1", ft, (T + ROW_1, -COLUMNS / 2.0)),
            ("L2", ft, (T + ROW_2, -COLUMNS / 2.0)),
        ];
    }
    for (k, d) in diams.iter().enumerate() {
        let (_, i, xy) = places[k];
        parts[i].hole(xy.0, xy.1, *d);
    }
    let Frame { o, u, v, w } = fr_top;
    // the chin layer stops where it would run into a sloping mouth face: its
    // lower inside corner just touching the mouth face's outside
    let n = mouth.n;
    let back = py::max(0.0, (mouth.off - p3::dot(n, [x_edge, 0.0, z_top - 2.0 * T])) / -n[0]);
    num += 1;
    let mut bird = OPart::new("bird", num, mouthpiece(T, 0.0), "the bird's half of the duct fill-engraved in its underside");
    bird.fills.push(duct(T, 0.0));
    bird.ink = Some(BIRD_INK);
    // the bird faces down: its own w = 0, the face the duct is engraved in,
    // lies on the top plate, so it is turned over about U, which its outline
    // and duct, both symmetric across, allow
    bird.m = Some(p3::matrix(&Frame { o, u, v: p3::mul(v, -1.0), w: p3::mul(w, -1.0) }));
    num += 1;
    let mut chin = OPart::new("chin layer", num, mouthpiece(-back, 0.0), "glued under the beak, its end against the mouth face");
    chin.m = Some(p3::matrix(&Frame { o: p3::sub(o, [0.0, 0.0, T]), u, v, w }));
    parts.push(bird);
    parts.push(chin);
    let inner2d: Vec<Vec<P2>> = frames.iter().zip(&polys).map(|(fr, pl)| pl.iter().map(|&q| p3::to2d(fr, q)).collect()).collect();
    let vnum = corner_numbers(&faces);
    // the top carries the nest's duct on its outside face and is drawn
    // outside face up, so it takes no numbers: the beak, the window and the
    // holes place it, and every corner it touches is numbered on the others
    for k in 0..faces.len() {
        let fi = parts[k].face.unwrap();
        if fi == ft {
            continue;
        }
        let nums = number_corners(&faces[fi], &frames[fi], &inner2d[fi], &parts[k].cuts, &vnum)?;
        parts[k].numbers = Some(nums);
    }
    let inner_pts = polys.iter().flatten().copied().collect();
    Ok((parts, Info { faces, edges: es, cellmap, ft, fm, v: vol, inner: inner2d, places, fr_top, frames, back, nip, inner_pts }))
}

// --- corner numbers ----------------------------------------------------------
// Every corner of a solid has a number, and every plate engraves it near each
// of its own corners, on its inside face: the plates that meet at a corner
// all carry its number, so the solid goes together by matching them. The
// digits are the trumpet tooling's single-stroke ones (its tools/glyphs.rs), x
// across and y up, in a cell 0.62 of the height wide.
pub fn digit(ch: char) -> &'static [&'static [(f64, f64)]] {
    match ch {
        '0' => &[&[(0.5, 1.0), (0.85, 0.8), (0.85, 0.2), (0.5, 0.0), (0.15, 0.2), (0.15, 0.8), (0.5, 1.0)]],
        '1' => &[&[(0.30, 0.78), (0.52, 1.0), (0.52, 0.0)], &[(0.28, 0.0), (0.78, 0.0)]],
        '2' => &[&[(0.10, 0.78), (0.30, 1.0), (0.70, 1.0), (0.90, 0.78), (0.90, 0.60), (0.10, 0.0), (0.90, 0.0)]],
        '3' => &[&[(0.10, 1.0), (0.90, 1.0), (0.45, 0.55)], &[(0.45, 0.55), (0.90, 0.55), (0.90, 0.16), (0.72, 0.0), (0.28, 0.0), (0.10, 0.16)]],
        '4' => &[&[(0.70, 0.0), (0.70, 1.0), (0.12, 0.32), (0.92, 0.32)]],
        '5' => &[&[(0.85, 1.0), (0.20, 1.0), (0.15, 0.55), (0.50, 0.62), (0.80, 0.50), (0.88, 0.28), (0.75, 0.06), (0.40, 0.0), (0.15, 0.12)]],
        '6' => &[&[
            (0.82, 0.92), (0.55, 1.0), (0.25, 0.85), (0.15, 0.45), (0.15, 0.18), (0.35, 0.0), (0.62, 0.0), (0.85, 0.18), (0.85, 0.38), (0.62, 0.55),
            (0.30, 0.55), (0.15, 0.45),
        ]],
        '7' => &[&[(0.12, 1.0), (0.90, 1.0), (0.42, 0.0)]],
        '8' => &[&[
            (0.5, 0.55), (0.22, 0.68), (0.22, 0.87), (0.5, 1.0), (0.78, 0.87), (0.78, 0.68), (0.5, 0.55), (0.18, 0.40), (0.18, 0.14), (0.5, 0.0),
            (0.82, 0.14), (0.82, 0.40), (0.5, 0.55),
        ]],
        _ => &[&[
            (0.18, 0.08), (0.45, 0.0), (0.75, 0.15), (0.85, 0.55), (0.85, 0.82), (0.65, 1.0), (0.38, 1.0), (0.15, 0.82), (0.15, 0.62), (0.38, 0.45),
            (0.70, 0.45), (0.85, 0.55),
        ]],
    }
}
pub const NUM_H: f64 = 4.0; // digit height
pub const NUM_CLEAR: f64 = 1.0; // from the joints, the holes and the window

/// A corner's key, round(x, 6) in each axis, as Python's dict compares it
/// (0.0 and -0.0 the same).
pub fn r6(v: V3) -> [u64; 3] {
    let r = |x: f64| {
        let y = py::round_dp(x, 6);
        (if y == 0.0 { 0.0 } else { y }).to_bits()
    };
    [r(v[0]), r(v[1]), r(v[2])]
}

/// The corners, round(x, 6), each once, keeping the first seen.
pub fn corners(faces: &[Face]) -> Vec<V3> {
    let mut seen: Vec<[u64; 3]> = vec![];
    let mut out = vec![];
    for f in faces {
        for &v in &f.verts {
            let k = r6(v);
            if !seen.contains(&k) {
                seen.push(k);
                out.push([py::round_dp(v[0], 6), py::round_dp(v[1], 6), py::round_dp(v[2], 6)]);
            }
        }
    }
    out
}

/// Python's x % y for floats: the sign of y.
fn pymod(a: f64, b: f64) -> f64 {
    let m = a % b;
    if m != 0.0 {
        if (b < 0.0) != (m < 0.0) {
            m + b
        } else {
            m
        }
    } else {
        0f64.copysign(b)
    }
}

/// Every corner of the solid numbered from 1: top down, and round each level
/// anticlockwise from -x, the beak's side.
pub fn corner_numbers(faces: &[Face]) -> Vec<([u64; 3], usize)> {
    let mut vs = corners(faces);
    let key = |v: &V3| (-py::round_dp(v[2], 3), pymod(py::matan2(v[1], v[0]) - PI, 2.0 * PI));
    vs.sort_by(|a, b| key(a).partial_cmp(&key(b)).unwrap());
    vs.iter().enumerate().map(|(i, v)| (r6(*v), i + 1)).collect()
}

/// The four corners of a number's box centred on c, its top toward up, in
/// the plate's frame as seen from outside.
pub fn number_box(text: &str, c: P2, up: P2) -> Vec<P2> {
    let n = text.len() as f64;
    let w = n * 0.62 * NUM_H + (n - 1.0) * 0.2 * NUM_H;
    let r = (up.1, -up.0);
    [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
        .iter()
        .map(|&(i, j): &(f64, f64)| (c.0 + i * r.0 * w / 2.0 + j * up.0 * NUM_H / 2.0, c.1 + i * r.1 * w / 2.0 + j * up.1 * NUM_H / 2.0))
        .collect()
}

fn minmax(v: &[f64]) -> (f64, f64) {
    (square_flute::fmin(v), square_flute::fmax(v))
}

/// A plate's corner numbers: (text, centre, up) in its frame as seen from
/// outside, each moved in from its corner along the line to the face's
/// centre until its box clears the plate's joints -- it lies inside the
/// face's inside, the part of it the cavity sees -- and its holes and window
/// by NUM_CLEAR. Up points at the corner.
pub fn number_corners(face: &Face, fr: &Frame, inner: &[P2], cuts: &[OCut], vnum: &[([u64; 3], usize)]) -> Result<Vec<Number>, String> {
    let g = p3::to2d(fr, face.centre);
    let keep: Vec<(P2, f64)> = cuts
        .iter()
        .map(|c| match c {
            OCut::Circle(cx, cy, r) => ((*cx, *cy), *r),
            OCut::Rect(q) => {
                let (x0, x1) = minmax(&q.iter().map(|p| p.0).collect::<Vec<_>>());
                let (y0, y1) = minmax(&q.iter().map(|p| p.1).collect::<Vec<_>>());
                (((x0 + x1) / 2.0, (y0 + y1) / 2.0), py::hypot(x1 - x0, y1 - y0) / 2.0)
            }
        })
        .collect();
    let mut out = vec![];
    for &v in &face.verts {
        let k = vnum.iter().find(|e| e.0 == r6(v)).unwrap().1;
        let a = p3::to2d(fr, v);
        let l = py::hypot(g.0 - a.0, g.1 - a.1);
        let d = ((g.0 - a.0) / l, (g.1 - a.1) / l);
        let up = (-d.0, -d.1);
        let mut s = 0.0;
        let text = k.to_string();
        let mut c;
        loop {
            if s >= l {
                return Err(format!("ValueError: no room for corner {k} on a {}", face.name));
            }
            c = (a.0 + s * d.0, a.1 + s * d.1);
            let bx = number_box(&text, c, up);
            let half = py::hypot(bx[0].0 - c.0, bx[0].1 - c.1);
            if bx.iter().all(|&q| in_poly_margin(q, NUM_CLEAR, inner) >= 0.0)
                && keep.iter().all(|&(h, r)| py::hypot(c.0 - h.0, c.1 - h.1) >= r + NUM_CLEAR + half)
            {
                break;
            }
            s += 0.25;
        }
        out.push((text, (py::round_dp(c.0, 3), py::round_dp(c.1, 3)), up));
    }
    Ok(out)
}

/// A solid's plate as the laser sees it, inside face up, so the corner
/// numbers engrave on the inside: mirrored across u, the numbers drawn as
/// strokes reading the right way round.
pub fn inside_up(p: &OPart) -> OPart {
    let mut q = OPart::new(&p.name, p.num, p.outline.iter().map(|&(x, y)| (x, -y)).collect(), &p.note);
    q.cuts = p
        .cuts
        .iter()
        .map(|c| match c {
            OCut::Rect(r) => OCut::Rect(r.iter().map(|&(x, y)| (x, -y)).collect()),
            OCut::Circle(cx, cy, r) => OCut::Circle(*cx, -cy, *r),
        })
        .collect();
    q.fills = p.fills.iter().map(|f| f.iter().map(|&(x, y)| (x, -y)).collect()).collect();
    q.ink = Some(p.ink.unwrap_or(BLUE));
    q.m = p.m;
    q.face = p.face;
    for (text, (cx, cy), (ux, uy)) in p.numbers.as_ref().unwrap() {
        let (c, up) = ((*cx, -cy), (*ux, -uy));
        let r = (-up.1, up.0); // reading direction, y down
        let n = text.len() as f64;
        let w = n * 0.62 * NUM_H + (n - 1.0) * 0.2 * NUM_H;
        let o = (c.0 - r.0 * w / 2.0 - up.0 * NUM_H / 2.0, c.1 - r.1 * w / 2.0 - up.1 * NUM_H / 2.0);
        let mut x0 = 0.0;
        for ch in text.chars() {
            for st in digit(ch) {
                q.strokes.push(
                    st.iter()
                        .map(|&(px, py_)| {
                            (
                                o.0 + r.0 * (x0 + px * 0.62 * NUM_H) + up.0 * py_ * NUM_H,
                                o.1 + r.1 * (x0 + px * 0.62 * NUM_H) + up.1 * py_ * NUM_H,
                            )
                        })
                        .collect(),
                );
            }
            x0 += 0.82 * NUM_H;
        }
    }
    q
}

fn bounds(poly: &[P2]) -> (f64, f64, f64, f64) {
    let (x0, x1) = minmax(&poly.iter().map(|p| p.0).collect::<Vec<_>>());
    let (y0, y1) = minmax(&poly.iter().map(|p| p.1).collect::<Vec<_>>());
    (x0, x1, y0, y1)
}

/// The point inside a convex polygon furthest from all its sides, found by a
/// grid search that narrows round the best point so far.
pub fn deepest(poly: &[P2]) -> P2 {
    let (x0, x1, y0, y1) = bounds(poly);
    let mut best = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    let mut h = py::max(x1 - x0, y1 - y0) / 2.0;
    while h > 1e-4 {
        let mut top: Option<(f64, P2)> = None;
        for i in -4..=4 {
            for j in -4..=4 {
                let c = (best.0 + i as f64 * h / 4.0, best.1 + j as f64 * h / 4.0);
                let s = in_poly_margin(c, 0.0, poly);
                if top.map_or(true, |t| s > t.0) {
                    top = Some((s, c));
                }
            }
        }
        best = top.unwrap().1;
        h /= 2.0;
    }
    (py::round_dp(best.0, 3), py::round_dp(best.1, 3))
}

/// Two holes, d1 and d2 across, in one convex polygon: the first further
/// along `ahead` the other way -- nearer the beak -- than the second, placed
/// so that the narrowest of their webs to the walls and half the web between
/// them is as wide as it can be, by a grid search that narrows round the
/// best so far.
pub fn pair(poly: &[P2], ahead: P2, d1: f64, d2: f64) -> (P2, P2) {
    let a = py::hypot(ahead.0, ahead.1);
    let (ax, ay) = (ahead.0 / a, ahead.1 / a);
    let (x0, x1, y0, y1) = bounds(poly);
    let at = |b: (f64, f64, f64)| {
        let (cx, cy, t) = b;
        ((cx - t * ax, cy - t * ay), (cx + t * ax, cy + t * ay))
    };
    let score = |b: (f64, f64, f64)| {
        let (p, q) = at(b);
        py::min(py::min(in_poly_margin(p, d1 / 2.0, poly), in_poly_margin(q, d2 / 2.0, poly)), (2.0 * b.2 - (d1 + d2) / 2.0) / 2.0)
    };
    let mut best = ((x0 + x1) / 2.0, (y0 + y1) / 2.0, py::max(x1 - x0, y1 - y0) / 4.0);
    let mut h = py::max(x1 - x0, y1 - y0) / 2.0;
    while h > 1e-3 {
        let mut top: Option<(f64, (f64, f64, f64))> = None;
        for i in -3..=3 {
            for j in -3..=3 {
                for k in -3..=3 {
                    let b = (best.0 + i as f64 * h / 3.0, best.1 + j as f64 * h / 3.0, best.2 + k as f64 * h / 3.0);
                    let s = score(b);
                    if top.map_or(true, |t| s > t.0) {
                        top = Some((s, b));
                    }
                }
            }
        }
        best = top.unwrap().1;
        h /= 2.0;
    }
    let (p, q) = at(best);
    let r = |p: P2| (py::round_dp(p.0, 3), py::round_dp(p.1, 3));
    (r(p), r(q))
}

/// Is the circle about c, radius r, inside the convex polygon poly? The
/// least margin to any side; negative when it is not.
pub fn in_poly_margin(c: P2, r: f64, poly: &[P2]) -> f64 {
    let k = poly.len();
    let (mut sx, mut sy) = (0.0, 0.0);
    for p in poly {
        sx += p.0;
    }
    for p in poly {
        sy += p.1;
    }
    let (cx, cy) = (sx / k as f64, sy / k as f64);
    let mut worst = 1e9;
    for i in 0..k {
        let (a, b) = (poly[i], poly[(i + 1) % k]);
        let l = py::hypot(b.0 - a.0, b.1 - a.1);
        let mut n = ((b.1 - a.1) / l, -(b.0 - a.0) / l);
        if (cx - a.0) * n.0 + (cy - a.1) * n.1 < 0.0 {
            n = (-n.0, -n.1);
        }
        worst = py::min(worst, (c.0 - a.0) * n.0 + (c.1 - a.1) * n.1 - r);
    }
    worst
}

/// A plate's world point from its frame's (u, v).
fn world(fr: &Frame, xy: P2) -> V3 {
    p3::add(fr.o, p3::add(p3::mul(fr.u, xy.0), p3::mul(fr.v, xy.1)))
}

pub type Check = (bool, String, String);

pub fn solid_checks(kind: &str, parts: &[OPart], info: &Info, kw: f64, diams: &[f64], worst: f64) -> (Vec<Check>, Vec<Sheet>, Vec<OPart>, f64) {
    let mut out: Vec<Check> = vec![];
    let mut note = |ok: bool, what: &str, detail: String| out.push((ok, what.to_string(), detail));
    note(worst < 20.0, "the model holds the scale", format!("worst {worst:.1} cents of eight notes"));
    let v = info.v;
    if kind == "cube" {
        let e = cube_edge(CUBE_VOLUME);
        note(
            (v / CUBE_VOLUME - 1.0).abs() < 1e-6,
            "the cavity is the volume asked for",
            format!("{e:.2}mm edge, {:.2}mm inside, {:.2}cm^3", e - 2.0 * T, v / 1000.0),
        );
    }
    let f0 = m::freq(0.0, kw, v);
    let mut span = f64::NEG_INFINITY;
    for &a in &info.inner_pts {
        for &b in &info.inner_pts {
            span = py::max(span, p3::dist(a, b));
        }
    }
    let lam = m::c() / f0;
    note(
        span < lam / 4.0,
        "the cavity is small against the wavelength",
        format!("{:.1}cm^3, sounding {f0:.1}Hz; {span:.0}mm across, a quarter wave {:.0}mm", v / 1000.0, lam / 4.0),
    );
    let poly = &info.inner[info.ft];
    let web = info.places.iter().zip(diams).map(|((_, i, xy), d)| in_poly_margin(*xy, d / 2.0, &info.inner[*i])).fold(f64::INFINITY, py::min);
    let win = [(T, -m::WINDOW_W / 2.0), (T + m::WINDOW_L, -m::WINDOW_W / 2.0), (T + m::WINDOW_L, m::WINDOW_W / 2.0), (T, m::WINDOW_W / 2.0)];
    let wweb = py::min(in_poly_margin(win[1], 0.0, poly), in_poly_margin(win[2], 0.0, poly));
    note(web >= MIN_WEB && wweb >= 0.0, "every hole and the window open into the cavity", format!("nearest hole {web:.1}mm from a wall"));
    let pl: Vec<(V3, f64)> = info.places.iter().zip(diams).map(|((_, i, xy), d)| (world(&info.frames[*i], *xy), *d)).collect();
    let mut gap = f64::INFINITY;
    for i in 0..pl.len() {
        for j in i + 1..pl.len() {
            gap = py::min(gap, p3::dist(pl[i].0, pl[j].0) - (pl[i].1 + pl[j].1) / 2.0);
        }
    }
    // and between the window and any hole on the top with it
    let mut wgap = f64::INFINITY;
    let mut any = false;
    for ((_, i, (x, y)), d) in info.places.iter().zip(diams) {
        if *i == info.ft {
            let a = py::max(py::max(T - x, 0.0), x - T - m::WINDOW_L);
            let v = py::hypot(a, py::max(y.abs() - m::WINDOW_W / 2.0, 0.0)) - d / 2.0;
            wgap = if any { py::min(wgap, v) } else { v };
            any = true;
        }
    }
    note(
        py::min(gap, wgap) >= HOLE_GAP,
        "ply left between neighbouring holes and the window",
        format!("narrowest {gap:.1}mm{}", if wgap < f64::INFINITY { format!(", {wgap:.1}mm from the window") } else { String::new() }),
    );
    // the hands, from the geometry rather than the names: right = forward x up
    let fwd = info.faces[info.ft].n;
    let right = p3::cross(fwd, [-1.0, 0.0, 0.0]);
    let ok = info.places.iter().all(|(h, i, xy)| (p3::dot(world(&info.frames[*i], *xy), right) > 0.0) == h.starts_with('R'));
    note(ok, "each hand's holes lie under that hand", "held holes forward, beak up".into());
    let bird = parts.iter().find(|p| p.name == "bird").unwrap();
    let tp = &parts[info.ft];
    let (fx0, fx1) = minmax(&bird.fills[0].iter().map(|q| q.0).collect::<Vec<_>>());
    note(
        (fx0 + BEAK).abs() < 1e-9 && (fx1 - T).abs() < 1e-9 && tp.fills == bird.fills && tp.numbers.is_none(),
        "the duct runs from the beak's tip to the window, in the bird and the top alike",
        format!("{}mm long, {}mm wide", py::g(T + BEAK), py::g(m::WINDOW_W)),
    );
    let (g0, g1) = (tp.strokes[0][0], tp.strokes[0][1]);
    let gx = g0.0;
    let mut near = f64::INFINITY;
    let mut any = false;
    for ((_, i, (x, y)), d) in info.places.iter().zip(diams) {
        if *i == info.ft {
            let v = py::hypot(py::max((x - gx).abs(), 0.0), py::max(y.abs() - m::WINDOW_W / 2.0, 0.0)) - d / 2.0;
            near = if any { py::min(near, v) } else { v };
            any = true;
        }
    }
    note(
        (gx - T - m::WINDOW_L - T / py::mtan(py::radians(BEVEL))).abs() < 1e-9
            && (g1.1 - g0.1 - m::WINDOW_W).abs() < 1e-9
            && near >= NUM_CLEAR
            && in_poly_margin((gx, 0.0), m::WINDOW_W / 2.0, &info.inner[info.ft]) >= 0.0,
        "the bevel guide marks where a 30 degree bevel meets the outside",
        format!(
            "{:.2}mm past the window, {}mm long, over the cavity{}",
            gx - T - m::WINDOW_L,
            py::g(m::WINDOW_W),
            if near < f64::INFINITY { format!(", {near:.1}mm from the nearest hole") } else { String::new() }
        ),
    );
    let (es, cm) = (&info.edges, &info.cellmap);
    // (round(theta, 2), round(L, 2)) -> the cell counts, sorted as Python's tuples
    let mut kinds: BTreeMap<(u64, u64), (f64, f64, Vec<usize>)> = BTreeMap::new();
    let ord = |x: f64| if x >= 0.0 { x.to_bits() ^ (1 << 63) } else { !x.to_bits() };
    for (k, e) in es.iter().enumerate() {
        let (th, l) = (py::round_dp(e.theta, 2), py::round_dp(e.l, 2));
        let ent = kinds.entry((ord(th), ord(l))).or_insert((th, l, vec![]));
        if !ent.2.contains(&cm[k].len()) {
            ent.2.push(cm[k].len());
        }
    }
    let widths: Vec<f64> = es.iter().enumerate().map(|(k, e)| e.l / cm[k].len() as f64).collect();
    let wmin = widths.iter().copied().fold(f64::INFINITY, py::min);
    let kinds_s: Vec<String> = kinds
        .values()
        .map(|(th, l, n)| {
            let mut n = n.clone();
            n.sort();
            format!("{}mm at {} deg, {} cells", py::g(*l), py::g(*th), n.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("/"))
        })
        .collect();
    note(
        wmin >= FINGER - 1e-9,
        "every edge is finger-jointed, as many as fit",
        format!("{} edges: {}; narrowest {wmin:.2}mm", es.len(), kinds_s.join("; ")),
    );
    let (mut coll, mut void, mut n) = (0usize, vec![], 0usize);
    let verts = corners(&info.faces);
    for (k, e) in es.iter().enumerate() {
        for (_, q) in p3::joint_points(e, &info.faces, &cm[k]) {
            n += 1;
            let o = owners(parts, q);
            if o > 1 {
                coll += 1;
            } else if o == 0 {
                void.push(verts.iter().map(|&v| p3::dist(q, v)).fold(f64::INFINITY, py::min));
            }
        }
    }
    note(
        coll == 0,
        "no two plates in the same place, along every joint",
        format!("{n} points sampled{}", if coll > 0 { format!(", {coll} in two plates") } else { ", each in one plate or none".into() }),
    );
    let far = void.iter().copied().fold(0.0, |a: f64, b| if void.is_empty() { a } else { py::max(a, b) });
    let far = if void.is_empty() { 0.0 } else { void.iter().copied().fold(f64::NEG_INFINITY, py::max).max(far.min(far)) };
    note(
        far <= 2.0 * T,
        "every joint closes, but within a finger of a corner",
        format!("{} of {n} points in a plate; the rest at corners, at most {far:.1}mm from one: shellac fills them", n - void.len()),
    );
    let (cn, two, _) = corner_scan(parts, info);
    note(
        two == 0,
        "no two plates in the same place, round every corner",
        format!(
            "{cn} points within {}mm of the {} corners, in the walls{}",
            py::g(CORNER_R),
            verts.len(),
            if two > 0 {
                format!(", {two} in two plates")
            } else {
                format!(", each in one plate or none{}", if info.nip != 0.0 { format!("; corners nipped {}mm", py::g(info.nip)) } else { String::new() })
            }
        ),
    );
    let (mn, mtwo) = mouthpiece_scan(parts, 0.5);
    note(
        mtwo == 0,
        "the beak, the bird and the chin layer clear the body",
        if mtwo == 0 { format!("{mn} points through them, none in a second part") } else { format!("{mtwo} of {mn} points in a second part") },
    );
    let nums: Vec<&Vec<Number>> = parts.iter().filter_map(|p| p.numbers.as_ref()).collect();
    let mut each: Vec<(String, usize)> = vec![];
    for p in parts {
        for (text, _, _) in p.numbers.iter().flatten() {
            match each.iter_mut().find(|e| &e.0 == text) {
                Some(e) => e.1 += 1,
                None => each.push((text.clone(), 1)),
            }
        }
    }
    let tv = info.faces[info.ft].verts.len();
    let emin = each.iter().map(|e| e.1).min().unwrap_or(0);
    let emax = each.iter().map(|e| e.1).max().unwrap_or(0);
    let total: usize = each.iter().map(|e| e.1).sum();
    note(
        nums.len() == info.faces.len() - 1 && nums.iter().map(|n| n.len()).sum::<usize>() == info.faces.iter().map(|f| f.verts.len()).sum::<usize>() - tv && emin >= 2,
        "every plate but the top numbers its corners, inside, clear of joints and holes",
        format!("{} corners, {total} numbers, {emin} to {emax} plates to a corner", each.len()),
    );
    let drawn: Vec<OPart> = parts.iter().map(|p| if p.numbers.is_some() { inside_up(p) } else { p.clone() }).collect();
    let sheets = pack(&drawn.iter().map(|p| (p.w, p.h)).collect::<Vec<_>>());
    let wmax = sheets.iter().map(|s| s.w).fold(f64::NEG_INFINITY, py::max);
    let hmax = sheets.iter().map(|s| s.h).fold(f64::NEG_INFINITY, py::max);
    note(
        sheets.iter().all(|s| s.w <= BED.0 && s.h <= BED.1),
        "every sheet fits the bed",
        format!("{} sheet(s), largest {wmax:.0} x {hmax:.0}mm", sheets.len()),
    );
    let placed: usize = sheets.iter().map(|s| s.parts.len()).sum();
    note(placed == parts.len(), "every part is on a sheet", format!("{placed} of {}", parts.len()));
    (out, sheets, drawn, f0)
}

pub const CORNER_R: f64 = 5.0;
pub const CORNER_STEP: f64 = 0.4;

/// The equal-tempered note nearest f, as A4 = 440Hz names it.
pub fn note_name(f: f64) -> String {
    let n = py::round(12.0 * py::log2(f / 440.0) + 69.0);
    format!("{}{}", ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"][n.rem_euclid(12) as usize], n.div_euclid(12) - 1)
}

/// Every point of a grid round each corner that lies in the walls -- inside
/// the solid, within T of its outside -- and how many of them lie in two
/// plates or in none. The joint check samples along the edges; this one
/// finds plates that share only a corner, which meet nowhere else. The grid
/// is set off a little so no point lands on a cell's boundary.
pub fn corner_scan(parts: &[OPart], info: &Info) -> (usize, usize, usize) {
    let faces = &info.faces;
    let k = (CORNER_R / CORNER_STEP) as i64;
    let (mut n, mut two, mut void) = (0, 0, 0);
    for v in corners(faces) {
        for i in -k..=k {
            for j in -k..=k {
                for mm in -k..=k {
                    let q = [
                        v[0] + i as f64 * CORNER_STEP + 0.0137,
                        v[1] + j as f64 * CORNER_STEP + 0.0291,
                        v[2] + mm as f64 * CORNER_STEP + 0.0173,
                    ];
                    let depth = faces.iter().map(|f| f.off - p3::dot(f.n, q)).fold(f64::INFINITY, py::min);
                    if depth < 0.0 || depth > T {
                        continue;
                    }
                    n += 1;
                    let o = owners(parts, q);
                    two += (o > 1) as usize;
                    void += (o == 0) as usize;
                }
            }
        }
    }
    (n, two, void)
}

/// A grid through the mouthpiece -- the bird, the chin layer and the top
/// plate's beak, out past the body -- at the middle of their thickness and a
/// little inside each face: how many points are also in another part.
pub fn mouthpiece_scan(parts: &[OPart], step: f64) -> (usize, usize) {
    let (mut n, mut two) = (0, 0);
    for p in parts {
        if !["bird", "chin layer", "top plate"].contains(&p.name.as_str()) {
            continue;
        }
        let (x0, x1, y0, y1) = bounds(&p.outline);
        let m = p.m.unwrap();
        for w in [0.1, T / 2.0, T - 0.1] {
            let mut u = x0 + 0.013;
            while u < x1 {
                let mut v = y0 + 0.029;
                while v < y1 {
                    if inside((u, v), &p.outline) && (u < 0.0 || p.name != "top plate") {
                        let q = [0, 1, 2].map(|i| m[i][0] * u + m[i][1] * v + m[i][2] * w + m[i][3]);
                        n += 1;
                        two += (owners(parts, q) > 1) as usize;
                    }
                    v += step;
                }
                u += step;
            }
        }
    }
    (n, two)
}

// --- sheets ------------------------------------------------------------------

fn group(col: &str, items: &[String]) -> String {
    square_flute::group(col, items)
}

pub fn svg_sheet(name: &str, k: usize, n: usize, s: &Sheet, drawn: &[OPart], what: &str, key: &str) -> String {
    let kf = kerf();
    let (mut orange, mut black, mut nums) = (vec![], vec![], vec![]);
    let mut fills: Vec<(&str, Vec<String>)> = vec![(BIRD_INK, vec![]), (NEST_INK, vec![])];
    for &(i, sx, sy) in &s.parts {
        let p = &drawn[i];
        let (dx, dy) = (sx - p.x0 + kf / 2.0, sy - p.y0 + kf / 2.0);
        let mv = |pts: &[P2]| -> Vec<P2> { pts.iter().map(|&(x, y)| (x + dx, y + dy)).collect() };
        black.push(path(&mv(&offset(&p.outline, kf / 2.0)), true));
        for c in &p.cuts {
            match c {
                OCut::Rect(r) => orange.push(path(&mv(&offset(r, -kf / 2.0)), true)),
                OCut::Circle(cx, cy, r) => orange.push(format!("<circle cx=\"{:.3}\" cy=\"{:.3}\" r=\"{:.3}\"/>", cx + dx, cy + dy, r - kf / 2.0)),
            }
        }
        for f in &p.fills {
            let ink = p.ink.unwrap_or(BLUE);
            match fills.iter_mut().find(|e| e.0 == ink) {
                Some(e) => e.1.push(path(&mv(f), true)),
                None => fills.push((ink, vec![path(&mv(f), true)])),
            }
        }
        for st in &p.strokes {
            nums.push(path(&mv(st), false));
        }
    }
    let desc = format!(
        "1 user unit = 1mm. Four-hole ocarina in {key}, English fingering, {name}, sheet {k} of {n}: {what}. 3mm ply, drawn for a {}mm kerf. The duct is a fill engrave in two inks, run either or both: red {BIRD_INK} in the bird's underside, magenta {NEST_INK} in the top's outside face. One alone about 1mm deep, both about 0.5mm each -- set the depth on scrap first. Blue {BLUE} engraves single lines: on the top, which lies outside face up, a line across past the window where the {} degree bevel filed on its far edge should reach; on every other plate, inside face up, the corner numbers, each carried by the plates that meet at that corner. Then the cuts: orange {ORANGE} cuts the window and the holes, and black {BLACK} frees the parts.",
        py::g(kf), py::g(BEVEL)
    );
    let mut out = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w:.2}mm\" height=\"{h:.2}mm\" viewBox=\"0 0 {w:.2} {h:.2}\">\n<title>Four-hole ocarina in {key}, {name}, sheet {k} of {n}</title>\n<desc>{desc}</desc>\n",
        w = s.w, h = s.h
    );
    for (ink, ds) in &fills {
        out += &format!("  <g fill=\"{ink}\" stroke=\"none\">\n");
        for d in ds {
            out += &format!("    <path d=\"{d}\"/>\n");
        }
        out += "  </g>\n";
    }
    if !nums.is_empty() {
        out += &group(BLUE, &nums);
    }
    out + &group(ORANGE, &orange) + &group(BLACK, &black) + "</svg>\n"
}

pub fn main() -> i32 {
    let (mut out, mut only, mut write, mut flat): (Option<String>, Option<String>, bool, bool) = (None, None, true, false);
    let names: Vec<&str> = BUILDS.iter().map(|b| b.0).collect();
    for x in std::env::args().skip(1) {
        if x == "--no-write" {
            write = false;
        } else if x == "--flat" {
            flat = true;
        } else if let Some(v) = x.strip_prefix("--out=") {
            out = Some(v.to_string());
        } else if let Some(v) = x.strip_prefix("--mouth=") {
            m::set_mouth(py::float(v).unwrap_or_else(|| lasermade_tools::die(format!("ValueError: could not convert string to float: {}", py::str_repr(v)))));
        } else if x.starts_with("--kerf=") {
            square_flute::set_kerf(&x);
        } else if let Some(v) = x.strip_prefix("--only=") {
            if !names.contains(&v) {
                lasermade_tools::die(format!("error: {v} is not a build. They are {}.", names.join(", ")));
            }
            only = Some(v.to_string());
        } else {
            lasermade_tools::die(format!("error: {x} is not a flag this generator reads. It takes --out=, --only=, --flat, --mouth=, --kerf= and --no-write."));
        }
    }
    if write && out.is_none() {
        lasermade_tools::die("error: --out=DIR, or --no-write to run the checks only.");
    }
    let (_, kw, _, diams) = m::design();
    let v0 = m::design().0;
    let rows = m::fingering_table(v0, kw, &diams);
    let worst = rows.iter().filter(|r| r.4).map(|r| r.3.abs()).fold(f64::NEG_INFINITY, py::max);
    let mut bad = 0;
    for (name, how) in BUILDS {
        if only.as_deref().is_some_and(|o| o != name) {
            continue;
        }
        let (parts, info) = build_solid(name, &diams).unwrap_or_else(|e| lasermade_tools::die(e));
        let (res, sheets, drawn, f0) = solid_checks(name, &parts, &info, kw, &diams, worst);
        let key = note_name(f0);
        println!(
            "\n{name}: {how}, cavity {:.1}cm^3, G4's fingering a {:+} cents from G4: {f0:.1}Hz with every hole closed, {key}",
            info.v / 1000.0,
            py::round(1200.0 * py::log2(f0 / m::f0()))
        );
        println!("  holes: {}", m::HOLES.iter().zip(&diams).map(|(h, d)| format!("{h} ø{d:.2}")).collect::<Vec<_>>().join(", "));
        for (ok, what, detail) in &res {
            println!("  {}  {what:52} {detail}", if *ok { "pass" } else { "FAIL" });
            bad += !ok as usize;
        }
        if write && bad == 0 {
            let o = out.as_deref().unwrap();
            let d = if flat { o.to_string() } else { format!("{}/{name}", o.trim_end_matches('/')) };
            std::fs::create_dir_all(&d).unwrap_or_else(|e| lasermade_tools::die(format!("{d}: {e}")));
            for (k, s) in sheets.iter().enumerate() {
                let mut ns: Vec<&str> = s.parts.iter().map(|&(i, _, _)| drawn[i].name.as_str()).collect();
                ns.sort();
                ns.dedup();
                let fname = format!("four-hole-ocarina-{name}-sheet{}-cut-files.svg", k + 1);
                let f = format!("{}/{fname}", d.trim_end_matches('/'));
                std::fs::write(&f, svg_sheet(name, k + 1, sheets.len(), s, &drawn, &ns.join(", "), &key))
                    .unwrap_or_else(|e| lasermade_tools::die(format!("{f}: {e}")));
                println!("  wrote {fname}  {:.0} x {:.0}mm, {} parts", s.w, s.h, s.parts.len());
            }
        }
    }
    if bad > 0 {
        println!("\n  {bad} check(s) failed. Nothing written.");
        return 1;
    }
    0
}

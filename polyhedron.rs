//! Convex solids finger-jointed on every edge, at any angle the faces meet:
//! the cube, the octagonal and hexagonal prisms, the square antiprism, the
//! octahedron, the dodecahedron, the icosahedron, the truncated octahedron,
//! the cuboctahedron, the rhombic dodecahedron and two Johnson solids, the
//! elongated square pyramid and bipyramid, that four_hole_ocarinas.rs builds,
//! and the tube panpipes' tubes. Rust since 2026-11-07 (polyhedron.py until
//! then; the move changed no byte of any cut file).
//!
//! A solid is any convex polyhedron, given as its faces' planes, and every
//! plate is that face's polygon, 3mm thick, inward from the outside surface.
//!
//! THE JOINT, in the plane across an edge. Two plates meet with their
//! outside faces on the edge line, at the dihedral angle theta, measured
//! inside the solid. Along the edge the run is cut into cells that alternate
//! between the two plates. In a plate's own cell it reaches the edge line;
//! in the other's, it stops short by
//!
//!     clear(theta) = T sin(theta)       (theta from 90 to 180 degrees)
//!
//! the least that keeps its 3mm slab out of the neighbour's. At 90 degrees
//! that is T, the ordinary finger joint. At 135, the octagon's walls, it is
//! 2.12mm: the end of the short plate then touches the inner corner of the
//! long one, so the joint closes on a line, and a V-shaped notch shows
//! outside, which the shellac fills. Every angle here is 90 or more, where no
//! finger pokes out of the outside surface. Boxes' Platonic generator, by
//! contrast, cuts every finger a full T deep, as at 90 degrees, whatever the
//! angle.
//!
//! THE CORNERS. One face owns each vertex -- the top or the bottom where the
//! vertex is on one, and elsewhere the first face found -- and the cells at
//! that end of its two edges are its own. The other edges there take no part
//! in that choice. Where three faces meet, every pair shares an edge, and
//! that is all. Where four meet, as on the square antiprism and the
//! octahedron, the two plates that share only the corner run into each other
//! just inside it, so outline() can nip every corner off, square to its
//! bisector.
//!
//! HOW MANY CELLS. As many as fit with none under FINGER, the square flute's
//! 6mm, and of the parity the corners need: odd when both ends belong to the
//! same plate, even when they belong to different ones. An edge whose ends no
//! plate owns takes an even count, starting with whichever plate walks it
//! from that end, so every wall of the prism is the same part.
use lasermade_tools::py;
use std::f64::consts::PI;

pub const T: f64 = 3.0;
pub const FINGER: f64 = 6.0;

pub type V3 = [f64; 3];

pub fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub fn mul(a: V3, k: f64) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}
/// Python's sum() starts from 0, so an all-zero dot is +0.0, never -0.0:
/// the sign atan2 reads on a corner straight across a face.
pub fn dot(a: V3, b: V3) -> f64 {
    0.0 + a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
pub fn unit(a: V3) -> V3 {
    mul(a, 1.0 / dot(a, a).sqrt())
}
pub fn dist(p: V3, q: V3) -> f64 {
    dot(sub(p, q), sub(p, q)).sqrt()
}

/// One face: its outward normal n, its plane n.p = off, and its corners in
/// order, anticlockwise seen from outside.
#[derive(Clone, Debug)]
pub struct Face {
    pub name: String,
    pub n: V3,
    pub off: f64,
    pub verts: Vec<V3>,
    pub centre: V3,
}

impl Face {
    pub fn new(name: &str, n: V3, off: f64, verts: Vec<V3>) -> Face {
        let n = unit(n);
        let mut s = [0.0; 3];
        for v in &verts {
            for k in 0..3 {
                s[k] += v[k];
            }
        }
        let c = mul(s, 1.0 / verts.len() as f64);
        let u = unit(sub(verts[0], c));
        let w = cross(n, u);
        let mut keyed: Vec<(f64, V3)> =
            verts.iter().map(|&p| (py::matan2(dot(sub(p, c), w), dot(sub(p, c), u)), p)).collect();
        keyed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Face { name: name.into(), n, off, verts: keyed.into_iter().map(|x| x.1).collect(), centre: c }
    }
}

/// Faces from planes and vertices: each plane's face is the vertices on it.
pub fn solid(planes: &[(String, V3, f64)], verts: &[V3]) -> Vec<Face> {
    planes
        .iter()
        .map(|(name, n, off)| {
            let n = unit(*n);
            let on: Vec<V3> = verts.iter().copied().filter(|&v| (dot(v, n) - off).abs() < 1e-6).collect();
            Face::new(name, n, *off, on)
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub a: usize,
    pub b: usize,
    pub p: V3,
    pub q: V3,
    pub l: f64,
    pub theta: f64,
}

/// Every edge: the two faces, its ends P and Q in the walking order of the
/// first face, and the dihedral angle inside the solid.
pub fn edges(faces: &[Face]) -> Vec<Edge> {
    let mut out = vec![];
    for (i, a) in faces.iter().enumerate() {
        let k = a.verts.len();
        for j in 0..k {
            let (p, q) = (a.verts[j], a.verts[(j + 1) % k]);
            for (b, bf) in faces.iter().enumerate() {
                if b <= i {
                    continue;
                }
                if bf.verts.iter().any(|&v| dist(p, v) < 1e-6) && bf.verts.iter().any(|&v| dist(q, v) < 1e-6) {
                    let c = py::max(-1.0, py::min(1.0, dot(a.n, bf.n)));
                    let theta = 180.0 - py::degrees(py::macos(c));
                    out.push(Edge { a: i, b, p, q, l: dist(p, q), theta });
                }
            }
        }
    }
    out
}

/// How far a plate stops short of the edge line in its neighbour's cell.
pub fn clear(theta: f64) -> f64 {
    assert!(90.0 - 1e-9 <= theta && theta <= 180.0, "{theta:.1} degrees: fingers would poke out");
    T * py::msin(py::radians(theta))
}

/// Cut edge E into cells and give each an owner, a face index. owner_at maps
/// a vertex to the face that owns it; force(E, s0, s1, owner) may name an
/// owner for a cell outright.
pub fn cells(e: &Edge, owner_at: impl Fn(V3) -> usize, force: Option<&dyn Fn(&Edge, f64, f64, usize) -> usize>) -> Vec<(f64, f64, usize)> {
    let (a, b, l) = (e.a, e.b, e.l);
    let ends: Vec<Option<usize>> =
        [owner_at(e.p), owner_at(e.q)].iter().map(|&o| if o == a || o == b { Some(o) } else { None }).collect();
    let mut n = (l / FINGER + 1e-9) as i64;
    let want = match (ends[0], ends[1]) {
        (Some(x), Some(y)) => Some(if x == y { 1 } else { 0 }),
        (None, None) => Some(0),
        _ => None,
    };
    if let Some(w) = want {
        if n.rem_euclid(2) != w {
            n -= 1;
        }
    }
    let other = |f: usize| if f == a { b } else { a };
    let own = |i: i64| -> usize {
        if let Some(e0) = ends[0] {
            if i % 2 == 0 {
                e0
            } else {
                other(e0)
            }
        } else if let Some(e1) = ends[1] {
            if (n - 1 - i).rem_euclid(2) == 0 {
                e1
            } else {
                other(e1)
            }
        } else if i % 2 == 0 {
            a
        } else {
            b
        }
    };
    let w = l / n as f64;
    (0..n)
        .map(|i| {
            let (s0, s1) = (i as f64 * w, (i + 1) as f64 * w);
            let mut o = own(i);
            if let Some(f) = force {
                o = f(e, s0, s1, o);
            }
            (s0, s1, o)
        })
        .collect()
}

/// A face's drawing frame: origin O and axes U, V in its plane, W inward.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub o: V3,
    pub u: V3,
    pub v: V3,
    pub w: V3,
}

pub fn frame(face: &Face, u: Option<V3>, o: Option<V3>) -> Frame {
    let w = mul(face.n, -1.0);
    let u = u.unwrap_or_else(|| unit(sub(face.verts[1], face.verts[0])));
    let v = cross(w, u);
    Frame { o: o.unwrap_or(face.centre), u, v, w }
}

pub fn to2d(fr: &Frame, p: V3) -> (f64, f64) {
    let d = sub(p, fr.o);
    (dot(d, fr.u), dot(d, fr.v))
}

pub fn matrix(fr: &Frame) -> [[f64; 4]; 3] {
    let mut m = [[0.0; 4]; 3];
    for i in 0..3 {
        m[i] = [fr.u[i], fr.v[i], fr.w[i], fr.o[i]];
    }
    m
}

fn r6(p: V3) -> [u64; 3] {
    // round(x, 6), compared as Python compares the tuples (0.0 == -0.0)
    let r = |x: f64| {
        let y = py::round_dp(x, 6);
        (if y == 0.0 { 0.0 } else { y }).to_bits()
    };
    [r(p[0]), r(p[1]), r(p[2])]
}

/// outline() with neither a beak nor a nip, as the tubes are.
pub fn outline(fi: usize, face: &Face, fr: &Frame, all: &[Edge], cellmap: &[Vec<(f64, f64, usize)>]) -> Vec<(f64, f64)> {
    outline_full(fi, face, fr, all, cellmap, None, 0.0)
}

/// The plate's outline in its frame: each edge walked from corner to
/// corner, at depth 0 in its own cells and clear(theta) inside them in its
/// neighbour's; corners where two edges meet set back to where the two set
/// lines cross. `beak`, (s0, s1, depth, edge index), moves a span of one
/// edge to -depth: out past it for the top's beak, in from it for the face
/// under the beak. `nip` cuts every corner off square to its bisector, that
/// far in from the solid's corner: where more than three plates meet, the
/// ones that share only the corner would otherwise run into each other just
/// inside it.
#[allow(clippy::too_many_arguments)]
pub fn outline_full(
    fi: usize, face: &Face, fr: &Frame, all: &[Edge], cellmap: &[Vec<(f64, f64, usize)>],
    beak: Option<(f64, f64, f64, usize)>, nip: f64,
) -> Vec<(f64, f64)> {
    let k = face.verts.len();
    let p2: Vec<(f64, f64)> = face.verts.iter().map(|&v| to2d(fr, v)).collect();
    let (mut cx, mut cy) = (0.0, 0.0);
    for p in &p2 {
        cx += p.0;
    }
    for p in &p2 {
        cy += p.1;
    }
    let c2 = (cx / k as f64, cy / k as f64);
    type Run = ((f64, f64), (f64, f64), (f64, f64), Vec<(f64, f64, f64)>);
    let mut runs: Vec<Run> = vec![];
    for j in 0..k {
        let (a, b) = (face.verts[j], face.verts[(j + 1) % k]);
        let want = {
            let mut s = [r6(a), r6(b)];
            s.sort();
            s
        };
        let ei = all
            .iter()
            .position(|e| {
                (fi == e.a || fi == e.b) && {
                    let mut s = [r6(e.p), r6(e.q)];
                    s.sort();
                    s == want
                }
            })
            .unwrap();
        let e = &all[ei];
        let (l, d) = (e.l, clear(e.theta));
        let same = dist(e.p, a) < 1e-6;
        let mut segs: Vec<(f64, f64, f64)> = cellmap[ei]
            .iter()
            .map(|&(s0, s1, o)| {
                let (s0, s1) = if same { (s0, s1) } else { (l - s1, l - s0) };
                (s0, s1, if o == fi { 0.0 } else { d })
            })
            .collect();
        segs.sort_by(|x, y| x.partial_cmp(y).unwrap());
        if let Some((b0, b1, depth, bj)) = beak {
            if bj == j {
                let mut cut = vec![];
                for &(s0, s1, dp) in &segs {
                    if s1 <= b0 || s0 >= b1 {
                        cut.push((s0, s1, dp));
                        continue;
                    }
                    if s0 < b0 {
                        cut.push((s0, b0, dp));
                    }
                    cut.push((py::max(s0, b0), py::min(s1, b1), -depth));
                    if s1 > b1 {
                        cut.push((b1, s1, dp));
                    }
                }
                segs = cut;
            }
        }
        let (p, q) = (p2[j], p2[(j + 1) % k]);
        let l2 = py::hypot(q.0 - p.0, q.1 - p.1);
        let (dx, dy) = ((q.0 - p.0) / l2, (q.1 - p.1) / l2);
        let (mut nx, mut ny) = (-dy, dx);
        if (c2.0 - p.0) * nx + (c2.1 - p.1) * ny < 0.0 {
            nx = -nx;
            ny = -ny;
        }
        runs.push((p, (dx, dy), (nx, ny), segs));
    }
    let mut pts = vec![];
    for j in 0..k {
        let (p, dv, nv, segs) = &runs[j];
        let at = |s: f64, dp: f64| (p.0 + s * dv.0 + dp * nv.0, p.1 + s * dv.1 + dp * nv.1);
        let line: Vec<(f64, f64)> = segs.iter().flat_map(|&(s0, s1, dp)| [at(s0, dp), at(s1, dp)]).collect();
        let (pp, pd, pn, psegs) = &runs[(j + k - 1) % k];
        let x = cross2(*pp, *pd, *pn, psegs.last().unwrap().2, *p, *dv, *nv, segs[0].2);
        pts.push(x);
        pts.extend_from_slice(&line[1..line.len() - 1]);
    }
    if nip != 0.0 {
        for j in 0..k {
            pts = clip(&pts, p2[j], p2[(j + k - 1) % k], p2[(j + 1) % k], c2, nip);
        }
    }
    simplify(&pts)
}

/// The polygon pts with its corner cut off: everything nearer `corner` than
/// d, measured along the line to the centre, and inside the corner's own two
/// edges -- so a beak running out past an edge is left alone.
/// Sutherland-Hodgman against the cutting line.
pub fn clip(pts: &[(f64, f64)], corner: (f64, f64), prev: (f64, f64), nxt: (f64, f64), centre: (f64, f64), d: f64) -> Vec<(f64, f64)> {
    let l = py::hypot(centre.0 - corner.0, centre.1 - corner.1);
    let n = ((centre.0 - corner.0) / l, (centre.1 - corner.1) / l);
    // is p on the centre's side of the line from corner to q?
    let within = |p: (f64, f64), q: (f64, f64)| {
        let side = |r: (f64, f64)| (q.0 - corner.0) * (r.1 - corner.1) - (q.1 - corner.1) * (r.0 - corner.0);
        side(p) * side(centre) >= -1e-6 * l * l
    };
    let keep = |p: (f64, f64)| (p.0 - corner.0) * n.0 + (p.1 - corner.1) * n.1 - d;
    let cut = |p: (f64, f64)| keep(p) < 0.0 && within(p, prev) && within(p, nxt);
    let m = pts.len();
    let mut out = vec![];
    for i in 0..m {
        let (p, q) = (pts[(i + m - 1) % m], pts[i]);
        if cut(p) != cut(q) {
            let (kp, kq) = (keep(p), keep(q));
            let t = kp / (kp - kq);
            out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
        }
        if !cut(q) {
            out.push(q);
        }
    }
    out
}

/// Where line 1 (through p1 + dp1 n1, along d1) meets line 2.
#[allow(clippy::too_many_arguments)]
pub fn cross2(p1: (f64, f64), d1: (f64, f64), n1: (f64, f64), dp1: f64, p2: (f64, f64), d2: (f64, f64), n2: (f64, f64), dp2: f64) -> (f64, f64) {
    let a = (p1.0 + dp1 * n1.0, p1.1 + dp1 * n1.1);
    let b = (p2.0 + dp2 * n2.0, p2.1 + dp2 * n2.1);
    let det = d1.0 * (-d2.1) - d1.1 * (-d2.0);
    let r = ((b.0 - a.0) * (-d2.1) - (b.1 - a.1) * (-d2.0)) / det;
    (a.0 + r * d1.0, a.1 + r * d1.1)
}

pub fn simplify(pts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = vec![];
    for p in pts {
        let p = (py::round_dp(p.0, 6), py::round_dp(p.1, 6));
        if out.last() != Some(&p) {
            out.push(p);
        }
    }
    if out[0] == *out.last().unwrap() {
        out.pop();
    }
    let mut changed = true;
    while changed {
        changed = false;
        let n = out.len();
        for i in 0..n {
            let (a, b, c) = (out[(i + n - 1) % n], out[i], out[(i + 1) % n]);
            if ((b.0 - a.0) * (c.1 - b.1) - (b.1 - a.1) * (c.0 - b.0)).abs() < 1e-9 {
                out.remove(i);
                changed = true;
                break;
            }
        }
    }
    out
}

/// The cavity: every face's plane moved T inward. Returns the volume and each
/// face's inner polygon, as 3D points.
pub fn inner(faces: &[Face]) -> (f64, Vec<Vec<V3>>) {
    let planes: Vec<(V3, f64)> = faces.iter().map(|f| (f.n, f.off - T)).collect();
    let mut pts: Vec<V3> = vec![];
    let m = planes.len();
    for i in 0..m {
        for j in i + 1..m {
            for k in j + 1..m {
                let ((n1, o1), (n2, o2), (n3, o3)) = (planes[i], planes[j], planes[k]);
                let det = dot(n1, cross(n2, n3));
                if det.abs() < 1e-9 {
                    continue;
                }
                let p = mul(add(add(mul(cross(n2, n3), o1), mul(cross(n3, n1), o2)), mul(cross(n1, n2), o3)), 1.0 / det);
                if planes.iter().all(|&(n, o)| dot(n, p) <= o + 1e-6) && !pts.iter().any(|&q| dist(p, q) < 1e-6) {
                    pts.push(p);
                }
            }
        }
    }
    let (mut vol, mut polys) = (0.0, vec![]);
    for &(n, o) in &planes {
        let on: Vec<V3> = pts.iter().copied().filter(|&p| (dot(n, p) - o).abs() < 1e-6).collect();
        if on.len() < 3 {
            polys.push(vec![]);
            continue;
        }
        let g = Face::new("", n, o, on);
        let mut area = 0.0;
        let k = g.verts.len();
        for i in 0..k {
            area += dot(cross(g.verts[i], g.verts[(i + 1) % k]), n) / 2.0;
        }
        vol += o * area / 3.0;
        polys.push(g.verts);
    }
    (vol, polys)
}

/// Points to test the joint of edge E with: in every cell, near its ends and
/// at its middle, along the line from the outside edge to the inner corner
/// where both plates' inner faces meet -- the line any leak through that
/// joint would follow.
pub fn joint_points(e: &Edge, faces: &[Face], cells: &[(f64, f64, usize)]) -> Vec<(f64, V3)> {
    let (a, b) = (&faces[e.a], &faces[e.b]);
    let (wa, wb) = (mul(a.n, -1.0), mul(b.n, -1.0));
    let c = dot(wa, wb);
    let i = mul(add(wa, wb), T / (1.0 + c));
    let d = unit(sub(e.q, e.p));
    let mut out = vec![];
    for &(s0, s1, _) in cells {
        for f in [0.08, 0.5, 0.92] {
            let s = s0 + f * (s1 - s0);
            for lam in [0.12, 0.5, 0.88] {
                out.push((s, add(add(e.p, mul(d, s)), mul(i, lam))));
            }
        }
    }
    out
}

// --- the solids ------------------------------------------------------------------

fn plane(name: &str, n: V3, off: f64) -> (String, V3, f64) {
    (name.to_string(), n, off)
}

/// A regular octagonal prism, flats across and height tall, outside: its
/// axis z, z from 0 to height, a flat facing -x for the mouth end.
pub fn octagonal_prism(flats: f64, height: f64) -> Vec<Face> {
    let r = flats / 2.0 / py::mcos(PI / 8.0);
    let mut verts = vec![];
    for k in 0..8 {
        let t = PI / 8.0 + k as f64 * PI / 4.0;
        for z in [0.0, height] {
            verts.push([r * py::mcos(t), r * py::msin(t), z]);
        }
    }
    let mut planes = vec![plane("top", [0.0, 0.0, 1.0], height), plane("bottom", [0.0, 0.0, -1.0], 0.0)];
    for k in 0..8 {
        let a = k as f64 * PI / 4.0;
        planes.push(plane(if k == 4 { "mouth wall" } else { "wall" }, [py::mcos(a), py::msin(a), 0.0], flats / 2.0));
    }
    solid(&planes, &verts)
}

/// The uniform hexagonal prism, every edge `edge` long: two regular hexagons
/// and six square walls, its axis z, z from 0 to edge, a flat facing -x for
/// the mouth end.
pub fn hexagonal_prism(edge: f64) -> Vec<Face> {
    let mut verts = vec![];
    for k in 0..6 {
        let t = PI / 6.0 + k as f64 * PI / 3.0;
        for z in [0.0, edge] {
            verts.push([edge * py::mcos(t), edge * py::msin(t), z]);
        }
    }
    let mut planes = vec![plane("top", [0.0, 0.0, 1.0], edge), plane("bottom", [0.0, 0.0, -1.0], 0.0)];
    for k in 0..6 {
        let a = k as f64 * PI / 3.0;
        planes.push(plane(if k == 3 { "mouth wall" } else { "wall" }, [py::mcos(a), py::msin(a), 0.0], edge * 3f64.sqrt() / 2.0));
    }
    solid(&planes, &verts)
}

/// Every pair of verts, in itertools.combinations' order.
fn pairs(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..n).flat_map(move |i| (i + 1..n).map(move |j| (i, j)))
}

/// A solid from its corners, scaled so every edge is `edge` long, then
/// turned so that the face nearest +z lies flat on top and its edge nearest
/// -x faces -x squarely, for the beak. Faces are named top, bottom if there
/// is one, `name` for the rest, and 'mouth ' + name for the one under the
/// beak. rank=1 takes the top edge whose normal is next nearest -x instead:
/// on a rhombus, the other way round, its edges in the other direction
/// across the beak.
pub fn stand(verts: &[V3], edge: f64, name: &str, rank: usize) -> Vec<Face> {
    let e0 = pairs(verts.len()).map(|(i, j)| dist(verts[i], verts[j])).fold(f64::INFINITY, py::min);
    let verts: Vec<V3> = verts.iter().map(|&v| mul(v, edge / e0)).collect();
    // the plane keeping the first of equals, as max() does
    let key = |n: V3| (py::round_dp(n[2], 9), -n[0], -n[1]);
    let mut best: Option<(V3, (f64, f64, f64))> = None;
    for (n, _) in hull(&verts) {
        let k = key(n);
        if best.map_or(true, |b| k > b.1) {
            best = Some((n, k));
        }
    }
    let n = best.unwrap().0;
    // the top face's normal onto +z, then the edge midpoint nearest -x onto -x
    let ex = if n[1].abs() < 0.9 { unit(cross([0.0, 1.0, 0.0], n)) } else { unit(cross(n, [1.0, 0.0, 0.0])) };
    let ey = cross(n, ex);
    let verts: Vec<V3> = verts.iter().map(|&v| [dot(v, ex), dot(v, ey), dot(v, n)]).collect();
    let zt = verts.iter().map(|v| v[2]).fold(f64::NEG_INFINITY, py::max);
    let mut top: Vec<V3> = verts.iter().copied().filter(|v| (v[2] - zt).abs() < 1e-6).collect();
    let mut s = [0.0; 3];
    for v in &top {
        for k in 0..3 {
            s[k] += v[k];
        }
    }
    let c = mul(s, 1.0 / top.len() as f64);
    let mut keyed: Vec<(f64, V3)> = top.iter().map(|&p| (py::matan2(p[1] - c[1], p[0] - c[0]), p)).collect();
    keyed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    top = keyed.into_iter().map(|x| x.1).collect();
    // each top edge's outward normal in the plane; the one facing most nearly
    // -x is turned onto -x, so that edge runs square across the beak. On a
    // regular face the normal points at the edge's midpoint, but on a rhombus
    // it does not, and only the normal makes the edge square
    let mut norms = vec![];
    for i in 0..top.len() {
        let (p, q) = (top[i], top[(i + 1) % top.len()]);
        let l = py::hypot(q[0] - p[0], q[1] - p[1]);
        let (mut nx, mut ny) = ((q[1] - p[1]) / l, -(q[0] - p[0]) / l);
        if nx * ((p[0] + q[0]) / 2.0 - c[0]) + ny * ((p[1] + q[1]) / 2.0 - c[1]) < 0.0 {
            nx = -nx;
            ny = -ny;
        }
        norms.push((nx, ny));
    }
    let mut keyed: Vec<(f64, (f64, f64))> = norms.iter().map(|&d| (py::powf(py::matan2(d.1, -d.0), 2.0), d)).collect();
    keyed.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let m = keyed[rank].1;
    let a = PI - py::matan2(m.1, m.0);
    let (ca, sa) = (py::mcos(a), py::msin(a));
    let r12 = |x: f64| py::round_dp(x, 12);
    let verts: Vec<V3> = verts.iter().map(|&[x, y, z]| [r12(x * ca - y * sa), r12(x * sa + y * ca), r12(z)]).collect();
    let planes: Vec<(String, V3, f64)> = hull(&verts)
        .into_iter()
        .map(|(n, off)| {
            let nm = if n[2] > 1.0 - 1e-9 { "top" } else if n[2] < -1.0 + 1e-9 { "bottom" } else { name };
            (nm.to_string(), n, off)
        })
        .collect();
    let mut faces = solid(&planes, &verts);
    let top = faces.iter().find(|f| f.name == "top").unwrap().clone();
    let xe = top.verts.iter().map(|v| v[0]).fold(f64::INFINITY, py::min);
    for f in faces.iter_mut() {
        let on = f.verts.iter().filter(|v| (v[0] - xe).abs() < 1e-6 && (v[2] - top.off).abs() < 1e-6).count();
        if f.name == name && on == 2 {
            f.name = format!("mouth {name}");
        }
    }
    faces
}

/// itertools.product((1, -1), repeat=3), as floats
fn cube_corners() -> Vec<V3> {
    let mut v = vec![];
    for a in [1.0, -1.0] {
        for b in [1.0, -1.0] {
            for c in [1.0, -1.0] {
                v.push([a, b, c]);
            }
        }
    }
    v
}

/// The cube, every edge `edge` long, centred on the origin.
pub fn cube(edge: f64) -> Vec<Face> {
    stand(&cube_corners(), edge, "side", 0)
}

/// The square antiprism, every edge `edge` long: two squares, top and
/// bottom, a quarter turn apart, and eight equilateral triangles between
/// them. Its height is edge / 2**(1/4), which makes the slanting edges as
/// long as the squares'.
pub fn square_antiprism(edge: f64) -> Vec<Face> {
    let (r, h) = (1.0 / 2f64.sqrt(), py::powf(2.0, -0.25));
    let mut verts = vec![];
    for k in 0..4 {
        for s in 0..2 {
            let t = k as f64 * PI / 2.0 + s as f64 * PI / 4.0;
            verts.push([r * py::mcos(t), r * py::msin(t), (if s == 1 { h } else { -h }) / 2.0]);
        }
    }
    stand(&verts, edge, "triangle", 0)
}

/// The regular octahedron, every edge `edge` long, standing on a face: eight
/// equilateral triangles, four to a corner.
pub fn octahedron(edge: f64) -> Vec<Face> {
    let mut verts = vec![];
    for ax in 0..3 {
        for s in [1.0, -1.0] {
            let mut v = [0.0; 3];
            v[ax] = s;
            verts.push(v);
        }
    }
    stand(&verts, edge, "triangle", 0)
}

fn phi() -> f64 {
    (1.0 + 5f64.sqrt()) / 2.0
}

/// The regular dodecahedron, every edge `edge` long, standing on a face:
/// twelve regular pentagons, three to a corner. Its corners are the cube's
/// (+-1, +-1, +-1) and the cyclic permutations of (0, +-1/phi, +-phi).
pub fn dodecahedron(edge: f64) -> Vec<Face> {
    let p = phi();
    let mut verts = cube_corners();
    for a in [1.0 / p, -1.0 / p] {
        for b in [p, -p] {
            verts.extend([[0.0, a, b], [a, b, 0.0], [b, 0.0, a]]);
        }
    }
    stand(&verts, edge, "pentagon", 0)
}

/// The regular icosahedron, every edge `edge` long, standing on a face:
/// twenty equilateral triangles, five to a corner. Its corners are the
/// cyclic permutations of (0, +-1, +-phi).
pub fn icosahedron(edge: f64) -> Vec<Face> {
    let p = phi();
    let mut verts = vec![];
    for a in [1.0, -1.0] {
        for b in [p, -p] {
            verts.extend([[0.0, a, b], [a, b, 0.0], [b, 0.0, a]]);
        }
    }
    stand(&verts, edge, "triangle", 0)
}

/// sorted(set(permutations of each of these)), as Python sorts int tuples
fn perms_sorted(seeds: &[[i64; 3]]) -> Vec<V3> {
    let mut all: Vec<[i64; 3]> = vec![];
    for v in seeds {
        for p in [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]] {
            let q = [v[p[0]], v[p[1]], v[p[2]]];
            if !all.contains(&q) {
                all.push(q);
            }
        }
    }
    all.sort();
    all.into_iter().map(|q| [q[0] as f64, q[1] as f64, q[2] as f64]).collect()
}

/// The truncated octahedron, every edge `edge` long, standing on a square:
/// six squares and eight regular hexagons, three to a corner. Its corners
/// are the permutations of (0, +-1, +-2).
pub fn truncated_octahedron(edge: f64) -> Vec<Face> {
    let mut seeds = vec![];
    for a in [1, -1] {
        for b in [2, -2] {
            seeds.push([0, a, b]);
        }
    }
    stand(&perms_sorted(&seeds), edge, "face", 0)
}

/// The cuboctahedron, every edge `edge` long, standing on a square: six
/// squares and eight equilateral triangles, four to a corner. Its corners
/// are the permutations of (+-1, +-1, 0).
pub fn cuboctahedron(edge: f64) -> Vec<Face> {
    let mut seeds = vec![];
    for a in [1, -1] {
        for b in [1, -1] {
            seeds.push([a, b, 0]);
        }
    }
    stand(&perms_sorted(&seeds), edge, "face", 0)
}

/// The rhombic dodecahedron's corners: the cube's (+-1, +-1, +-1) and
/// (+-2, 0, 0) in every axis.
pub fn rhombic_corners() -> Vec<V3> {
    let mut verts = cube_corners();
    for a in 0..3 {
        for s in [1.0, -1.0] {
            let mut v = [0.0; 3];
            v[a] = 2.0 * s;
            verts.push(v);
        }
    }
    verts
}

/// The rhombic dodecahedron, every edge `edge` long, standing on a rhombus:
/// twelve rhombi, every joint at 120 degrees, three or four to a corner. A
/// zonohedron, the solid swept by four vectors, the cube's body diagonals.
pub fn rhombic_dodecahedron(edge: f64) -> Vec<Face> {
    // the rhombus's other edges across the beak: index to middle finger the
    // same, but the hands 67mm apart instead of 77, and the window 47mm from
    // the furthest hole instead of 62 (solid_survey)
    stand(&rhombic_corners(), edge, "rhombus", 1)
}

/// A unit cube's corners as a square prism, z from 0 to 1, and the height of
/// a square pyramid with unit edges.
fn square_prism() -> (Vec<V3>, f64) {
    let r = 1.0 / 2f64.sqrt();
    let mut v = vec![];
    for k in 0..4 {
        let t = k as f64 * PI / 2.0;
        for z in [0.0, 1.0] {
            v.push([r * py::mcos(t), r * py::msin(t), z]);
        }
    }
    (v, (1.0 - r * r).sqrt())
}

/// Johnson solid J8, every edge `edge` long: a cube with a square pyramid of
/// four equilateral triangles on its bottom, so it hangs point down from its
/// square top.
pub fn elongated_square_pyramid(edge: f64) -> Vec<Face> {
    let (mut v, h) = square_prism();
    v.push([0.0, 0.0, -h]);
    stand(&v, edge, "face", 0)
}

/// Johnson solid J15, every edge `edge` long: a cube with a square pyramid
/// on each end, four squares and eight equilateral triangles, standing on a
/// triangle.
pub fn elongated_square_bipyramid(edge: f64) -> Vec<Face> {
    let (mut v, h) = square_prism();
    v.push([0.0, 0.0, -h]);
    v.push([0.0, 0.0, 1.0 + h]);
    stand(&v, edge, "face", 0)
}

/// The planes of the convex hull of verts, as (outward unit normal, offset):
/// every plane through three corners with all the others on its inner side,
/// each once.
pub fn hull(verts: &[V3]) -> Vec<(V3, f64)> {
    let mut out: Vec<(V3, f64)> = vec![];
    let n = verts.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (p, q, r) = (verts[i], verts[j], verts[k]);
                let mut nn = cross(sub(q, p), sub(r, p));
                if dot(nn, nn) < 1e-12 {
                    continue;
                }
                nn = unit(nn);
                let mut off = dot(nn, p);
                let side: Vec<f64> = verts.iter().map(|&v| dot(nn, v) - off).collect();
                if side.iter().copied().fold(f64::NEG_INFINITY, py::max) > 1e-6 {
                    if side.iter().copied().fold(f64::INFINITY, py::min) < -1e-6 {
                        continue;
                    }
                    nn = mul(nn, -1.0);
                    off = -off;
                }
                if !out.iter().any(|(m, _)| dot(nn, *m) > 1.0 - 1e-9) {
                    out.push((nn, off));
                }
            }
        }
    }
    out
}

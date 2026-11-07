//! polyhedron.py's solid, in the parts tube_panpipes uses: faces from
//! planes, every edge, its cells, a plate's outline in its frame, and the
//! cavity. The joint is the same: a plate reaches the edge line in its own
//! cells and stops clear(theta) short in its neighbour's.
use lasermade_tools::py;

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
pub fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
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

/// The plate's outline in its frame: each edge walked from corner to corner,
/// at depth 0 in its own cells and clear(theta) in from it in its
/// neighbour's; corners set back to where the two set lines cross. (No beak,
/// no nip: the tubes have neither.)
pub fn outline(fi: usize, face: &Face, fr: &Frame, all: &[Edge], cellmap: &[Vec<(f64, f64, usize)>]) -> Vec<(f64, f64)> {
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
    simplify(&pts)
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

//! Which other solids would make a four-hole ocarina, and would any reach
//! the hands better? Zonohedra and Johnson solids, against the builds.
//!
//!     target/release/solid_survey                  # every build and every candidate
//!     target/release/solid_survey rhombic-dodecahedron cube   # just these
//!
//! Every candidate has equal edges and is sized to the cube's 280cm^3
//! inside, then run through four_hole_ocarinas.rs's own build_faces() and
//! solid_checks(), twice: with the holes on the top's grid, and with each
//! hand's holes on the faces at its side. A candidate with a joint under 90
//! degrees is skipped, since polyhedron.rs's fingers would poke out of it.
//! Corner nips are tried from none to 5.5mm and the first that passes is
//! shown.
//!
//! For each that passes, and for each build, it prints the hands'
//! distances, hole centre to hole centre: index to middle finger (the larger
//! of the two hands), left index to right index, the most any hole's face
//! tilts from the top, and the window's centre to the furthest hole. The
//! grid's are 22, 34, 0 and 42.1mm.
//!
//! It poses each candidate as the cube, so the generator's checks run
//! unchanged. Rust since 2026-11-07 (solid_survey.py until then, which took
//! over ten minutes; its output is unchanged). Three candidates are built
//! from Python sets, whose order picks which face is found first, so PySet
//! below keeps CPython's order.
use crate::ocarina_model as m;
use crate::polyhedron as p3;
use crate::*;

fn phi() -> f64 {
    (1.0 + 5f64.sqrt()) / 2.0
}

fn ring(n: usize, r: f64, z: f64, a0: f64) -> Vec<V3> {
    (0..n)
        .map(|k| {
            let a = a0 + 2.0 * PI * k as f64 / n as f64;
            [r * py::mcos(a), r * py::msin(a), z]
        })
        .collect()
}

/// circumradius of a regular n-gon of unit edge
fn rr(n: usize) -> f64 {
    1.0 / (2.0 * py::msin(PI / n as f64))
}

/// apex height over a regular n-gon, unit edges
fn pyramid_h(n: usize) -> f64 {
    (1.0 - py::powf(rr(n), 2.0)).sqrt()
}

/// A unit-edge n-cupola on a 2n-gon at z0: (bottom ring, top ring).
fn cupola(n: usize, z0: f64, up: f64) -> (Vec<V3>, Vec<V3>) {
    let (r2, r1) = (rr(2 * n), rr(n));
    let bot = ring(2 * n, r2, z0, PI / (2 * n) as f64);
    // top edge k parallel to bottom edge 2k: its vertices straddle that
    // edge's mid-direction by pi/n
    let top = ring(n, r1, 0.0, 0.0);
    // height from one top vertex to its nearest bottom vertex being 1
    let t = top[0];
    let d = bot.iter().map(|b| py::hypot(t[0] - b[0], t[1] - b[1])).fold(f64::INFINITY, py::min);
    let h = (1.0 - d * d).sqrt();
    let top = top.iter().map(|&[x, y, _]| [x, y, z0 + up * h]).collect();
    (bot, top)
}

// --- CPython 3.9's set, for its iteration order ------------------------------

const MODULUS: u64 = (1 << 61) - 1;

fn frexp(v: f64) -> (f64, i32) {
    let bits = v.to_bits();
    let exp = ((bits >> 52) & 0x7ff) as i32;
    if exp == 0 {
        let (m, e) = frexp(v * 18446744073709551616.0);
        return (m, e - 64);
    }
    (f64::from_bits((bits & !(0x7ffu64 << 52)) | (1022u64 << 52)), exp - 1022)
}

/// hash(x) for a finite float, as _Py_HashDouble; an integral one hashes as
/// the int, so ints in these tuples need nothing else.
fn hash_float(v: f64) -> u64 {
    if v == 0.0 {
        return 0;
    }
    let (mut m, mut e) = frexp(v);
    let neg = m < 0.0;
    if neg {
        m = -m;
    }
    let mut x: u64 = 0;
    while m != 0.0 {
        x = ((x << 28) & MODULUS) | x >> (61 - 28);
        m *= 268435456.0;
        e -= 28;
        let y = m as u64;
        m -= y as f64;
        x += y;
        if x >= MODULUS {
            x -= MODULUS;
        }
    }
    let e = if e >= 0 { e % 61 } else { 61 - 1 - ((-1 - e) % 61) } as u32;
    x = ((x << e) & MODULUS) | x >> (61 - e);
    if neg {
        x = x.wrapping_neg();
    }
    if x == u64::MAX {
        x = u64::MAX - 1;
    }
    x
}

/// hash() of a 3-tuple of numbers: xxHash's lanes, as tuplehash.
fn hash_tuple(t: &V3) -> u64 {
    const P1: u64 = 11400714785074694791;
    const P2: u64 = 14029467366897019727;
    const P5: u64 = 2870177450012600261;
    let mut acc = P5;
    for &x in t {
        acc = acc.wrapping_add(hash_float(x).wrapping_mul(P2));
        acc = acc.rotate_left(31);
        acc = acc.wrapping_mul(P1);
    }
    acc = acc.wrapping_add(3 ^ (P5 ^ 3527539));
    if acc == u64::MAX {
        return 1546275796;
    }
    acc
}

/// A set of 3-tuples, inserted and iterated as CPython 3.9 does (nothing is
/// ever removed, so there are no dummies).
struct PySet {
    table: Vec<Option<(u64, V3)>>,
    fill: usize,
}

impl PySet {
    fn new() -> PySet {
        PySet { table: vec![None; 8], fill: 0 }
    }
    fn mask(&self) -> usize {
        self.table.len() - 1
    }
    fn same(e: &Option<(u64, V3)>, hash: u64, key: &V3) -> bool {
        matches!(e, Some((h, k)) if *h == hash && k == key)
    }
    fn add(&mut self, key: V3) {
        let h = hash_tuple(&key);
        self.add_entry(key, h);
    }
    fn add_entry(&mut self, key: V3, hash: u64) {
        let mask = self.mask();
        let mut i = hash as usize & mask;
        let mut slot = i;
        if self.table[i].is_some() {
            let mut perturb = hash;
            'probe: loop {
                if Self::same(&self.table[i], hash, &key) {
                    return;
                }
                if i + 9 <= mask {
                    for j in 1..=9 {
                        match &self.table[i + j] {
                            None => {
                                slot = i + j;
                                break 'probe;
                            }
                            e if Self::same(e, hash, &key) => return,
                            _ => {}
                        }
                    }
                }
                perturb >>= 5;
                i = (i.wrapping_mul(5).wrapping_add(1).wrapping_add(perturb as usize)) & mask;
                if self.table[i].is_none() {
                    slot = i;
                    break;
                }
            }
        }
        self.table[slot] = Some((hash, key));
        self.fill += 1;
        if self.fill * 5 >= mask * 3 {
            self.resize(if self.fill > 50000 { self.fill * 2 } else { self.fill * 4 });
        }
    }
    fn insert_clean(table: &mut [Option<(u64, V3)>], key: V3, hash: u64) {
        let mask = table.len() - 1;
        let mut perturb = hash;
        let mut i = hash as usize & mask;
        loop {
            if table[i].is_none() {
                table[i] = Some((hash, key));
                return;
            }
            if i + 9 <= mask {
                for j in 1..=9 {
                    if table[i + j].is_none() {
                        table[i + j] = Some((hash, key));
                        return;
                    }
                }
            }
            perturb >>= 5;
            i = (i.wrapping_mul(5).wrapping_add(1).wrapping_add(perturb as usize)) & mask;
        }
    }
    fn resize(&mut self, minused: usize) {
        let mut size = 8;
        while size <= minused {
            size <<= 1;
        }
        let mut table = vec![None; size];
        for (h, k) in self.table.iter().flatten() {
            Self::insert_clean(&mut table, *k, *h);
        }
        self.table = table;
    }
    /// set |= other
    fn merge(&mut self, other: &PySet) {
        if other.fill == 0 {
            return;
        }
        if (self.fill + other.fill) * 5 >= self.mask() * 3 {
            self.resize((self.fill + other.fill) * 2);
        }
        for (h, k) in other.table.iter().flatten() {
            self.add_entry(*k, *h);
        }
    }
    fn list(&self) -> Vec<V3> {
        self.table.iter().flatten().map(|e| e.1).collect()
    }
}

/// round(x, 9), with -0.0 kept as Python keeps it
fn r9(x: f64) -> f64 {
    py::round_dp(x, 9)
}

fn zonohedron(gens: &[V3]) -> Vec<V3> {
    let mut pts = PySet::new();
    pts.add([0.0, 0.0, 0.0]);
    for g in gens {
        let mut more = PySet::new();
        for p in pts.list() {
            more.add([r9(p[0] + g[0]), r9(p[1] + g[1]), r9(p[2] + g[2])]);
        }
        pts.merge(&more);
    }
    // the hull's corners only
    let pts = pts.list();
    let planes = p3::hull(&pts);
    let corners: Vec<V3> = pts
        .iter()
        .copied()
        .filter(|&p| planes.iter().filter(|(n, o)| (p3::dot(*n, p) - o).abs() < 1e-6).count() >= 3)
        .collect();
    let mut c = [0.0; 3];
    for k in 0..3 {
        let mut s = 0.0;
        for p in &corners {
            s += p[k];
        }
        c[k] = s / corners.len() as f64;
    }
    corners.iter().map(|p| [p[0] - c[0], p[1] - c[1], p[2] - c[2]]).collect()
}

fn flip(v: &[V3]) -> Vec<V3> {
    v.iter().map(|&[x, y, z]| [x, -y, -z]).collect()
}

/// itertools.permutations of three, in its order
fn perms(t: [f64; 3]) -> Vec<[f64; 3]> {
    [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]].iter().map(|p| [t[p[0]], t[p[1]], t[p[2]]]).collect()
}

fn signs() -> Vec<[f64; 3]> {
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

pub fn candidates() -> Vec<(String, Vec<V3>)> {
    let p = phi();
    let mut c: Vec<(String, Vec<V3>)> = vec![];
    // --- zonohedra ---------------------------------------------------------
    c.push(("rhombic-dodecahedron".into(), p3::rhombic_corners()));
    let six = [[0.0, 1.0, p], [0.0, -1.0, p], [1.0, p, 0.0], [-1.0, p, 0.0], [p, 0.0, 1.0], [p, 0.0, -1.0]];
    c.push(("rhombic-triacontahedron".into(), zonohedron(&six)));
    c.push(("rhombic-icosahedron".into(), zonohedron(&six[..5])));
    let s2 = 2f64.sqrt();
    let mut tc = PySet::new();
    for pm in perms([1.0, 1.0 + s2, 1.0 + 2.0 * s2]) {
        for sg in signs() {
            tc.add([sg[0] * pm[0], sg[1] * pm[1], sg[2] * pm[2]]);
        }
    }
    c.push(("truncated-cuboctahedron".into(), tc.list()));
    // --- Johnson solids, unit edges, built so a big face is on top ---------
    for (n, nm) in [(4, "square"), (5, "pentagonal"), (3, "triangular")] {
        // elongated pyramid, point down: prism z 0..1, apex below
        let v = [ring(n, rr(n), 0.0, 0.0), ring(n, rr(n), 1.0, 0.0)].concat();
        c.push((format!("elongated-{nm}-pyramid"), [v.clone(), vec![[0.0, 0.0, -pyramid_h(n)]]].concat()));
        c.push((format!("elongated-{nm}-bipyramid"), [v, vec![[0.0, 0.0, -pyramid_h(n)], [0.0, 0.0, 1.0 + pyramid_h(n)]]].concat()));
    }
    for (n, nm) in [(4, "square"), (5, "pentagonal")] {
        // gyroelongated pyramid, point down: antiprism, apex below
        let h = (1.0 - py::powf(2.0 * rr(n) * py::msin(PI / (2 * n) as f64), 2.0)).sqrt();
        let v = [ring(n, rr(n), 0.0, 0.0), ring(n, rr(n), h, PI / n as f64)].concat();
        c.push((format!("gyroelongated-{nm}-pyramid"), [v, vec![[0.0, 0.0, -pyramid_h(n)]]].concat()));
    }
    for (n, nm) in [(3, "triangular"), (4, "square"), (5, "pentagonal")] {
        let (bot, top) = cupola(n, 1.0, 1.0);
        let prism: Vec<V3> = bot.iter().map(|&[x, y, _]| [x, y, 0.0]).collect();
        c.push((format!("elongated-{nm}-cupola"), [prism.clone(), bot.clone(), top.clone()].concat())); // small n-gon on top
        c.push((format!("{nm}-cupola-upside-down"), flip(&[bot.clone(), top.clone()].concat()))); // 2n-gon on top
        let (_, t2) = cupola(n, 0.0, -1.0);
        // (x, y, -h) for (x, y, h) in (p0, p1, -(p2 - 0)): the same point
        let t2: Vec<V3> = t2.iter().map(|p| [p[0], p[1], -(-(p[2] - 0.0))]).collect();
        c.push((format!("elongated-{nm}-orthobicupola"), [bot, top, t2, prism].concat()));
    }
    for (n, nm) in [(3, "triangular"), (4, "square")] {
        let (bot, top) = cupola(n, 0.0, 1.0);
        let (_, t2) = cupola(n, 0.0, -1.0);
        c.push((format!("{nm}-orthobicupola"), [bot, top, t2].concat()));
    }
    c
}

fn dihedrals(verts: &[V3]) -> (f64, Vec<Face>, Vec<Edge>) {
    let planes: Vec<(String, V3, f64)> = p3::hull(verts).into_iter().map(|(n, o)| ("f".to_string(), n, o)).collect();
    let faces = p3::solid(&planes, verts);
    let es = p3::edges(&faces);
    let th = es.iter().map(|e| e.theta).fold(f64::INFINITY, py::min);
    (th, faces, es)
}

fn degree(faces: &[Face]) -> usize {
    let mut cnt: Vec<([u64; 3], usize)> = vec![];
    for f in faces {
        for &v in &f.verts {
            let k = r6(v);
            match cnt.iter_mut().find(|e| e.0 == k) {
                Some(e) => e.1 += 1,
                None => cnt.push((k, 1)),
            }
        }
    }
    cnt.iter().map(|e| e.1).max().unwrap()
}

fn faces_of_v(verts: &[V3], edge: f64, rank: usize) -> Vec<Face> {
    p3::stand(verts, edge, "face", rank)
}

/// Does every top edge's normal point at its midpoint, as on a regular face?
/// If not (a rhombus), the beak can take either pair of edges.
fn regular_top(verts: &[V3]) -> bool {
    let fs = faces_of_v(verts, 1.0, 0);
    let top = fs.iter().find(|f| f.name == "top").unwrap();
    let (c, k) = (top.centre, top.verts.len());
    for i in 0..k {
        let (a, b) = (top.verts[i], top.verts[(i + 1) % k]);
        let mm = p3::sub(p3::mul(p3::add(a, b), 0.5), c);
        if p3::dot(p3::unit(mm), p3::unit(p3::sub(b, a))).abs() > 1e-6 {
            return false;
        }
    }
    true
}

fn size(verts: &[V3]) -> f64 {
    let v = 280e3;
    let (mut lo, mut hi) = (5.0, 300.0);
    for _ in 0..60 {
        let e = (lo + hi) / 2.0;
        let (vol, _) = p3::inner(&faces_of_v(verts, e, 0));
        if vol < v {
            lo = e;
        } else {
            hi = e;
        }
    }
    (lo + hi) / 2.0
}

struct Ran {
    parts: Vec<OPart>,
    info: Info,
    res: Vec<Check>,
    sheets: usize,
    f0: f64,
    diams: Vec<f64>,
}

/// What went wrong, as Python's `type: message`, from a panic's payload.
fn panic_text(e: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = e.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = e.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "panic".into()
    }
}

/// The generator's build and checks on these faces, posing as the cube.
fn run(faces: &[Face], mode: &str, nip: f64) -> Result<Ran, String> {
    let faces = faces.to_vec();
    let mode = mode.to_string();
    std::panic::catch_unwind(move || {
        let (v, kw, _, diams) = m::design();
        let rows = m::fingering_table(v, kw, &diams);
        let worst = rows.iter().filter(|r| r.4).map(|r| r.3.abs()).fold(f64::NEG_INFINITY, py::max);
        let (parts, info) = build_faces("cube", faces, &diams, mode == "faces", nip)?;
        let (res, sheets, _, f0) = solid_checks("cube", &parts, &info, kw, &diams, worst);
        let res = res.into_iter().filter(|r| r.1 != "the cavity is the volume asked for").collect();
        Ok(Ran { parts, info, res, sheets: sheets.len(), f0, diams })
    })
    .unwrap_or_else(|e| Err(panic_text(e)))
}

fn hands(info: &Info, _diams: &[f64]) -> (f64, f64, f64, f64) {
    let world = |i: usize, xy: P2| {
        let fr = &info.frames[i];
        p3::add(fr.o, p3::add(p3::mul(fr.u, xy.0), p3::mul(fr.v, xy.1)))
    };
    let at = |h: &str| info.places.iter().find(|p| p.0 == h).unwrap();
    let w = |h: &str| {
        let (_, i, xy) = *at(h);
        world(i, xy)
    };
    let top_n = info.faces[info.ft].n;
    let same = py::max(p3::dist(w("L1"), w("L2")), p3::dist(w("R1"), w("R2")));
    let across = p3::dist(w("L1"), w("R1"));
    let tilt = m::HOLES
        .iter()
        .map(|h| py::degrees(py::macos(py::max(-1.0, py::min(1.0, p3::dot(info.faces[at(h).1].n, top_n))))))
        .fold(f64::NEG_INFINITY, py::max);
    let wpt = world(info.ft, (T + m::WINDOW_L / 2.0, 0.0));
    let reach = m::HOLES.iter().map(|h| p3::dist(wpt, w(h))).fold(f64::NEG_INFINITY, py::max);
    (same, across, tilt, reach)
}

enum Row {
    Pass { mode: &'static str, nip: f64, v: f64, f0: f64, np: usize, h: (f64, f64, f64, f64), ns: usize, back: f64 },
    Fail { mode: &'static str, last: String },
}

fn report(faces: &[Face], nip_guess: f64) -> Vec<Row> {
    let mut out = vec![];
    let mut nips = vec![0.0, nip_guess, 3.5, 4.25, 4.75, 5.5];
    nips.sort_by(|a, b| a.partial_cmp(b).unwrap());
    nips.dedup();
    for mode in ["grid", "faces"] {
        let mut last = String::new();
        let mut passed = false;
        for &nip in &nips {
            match run(faces, mode, nip) {
                Err(e) => last = e,
                Ok(r) => {
                    let fails: Vec<&String> = r.res.iter().filter(|x| !x.0).map(|x| &x.1).collect();
                    last = format!("[{}]", fails.iter().map(|s| py::str_repr(s)).collect::<Vec<_>>().join(", "));
                    if fails.is_empty() {
                        let h = hands(&r.info, &r.diams);
                        out.push(Row::Pass { mode, nip, v: r.info.v, f0: r.f0, np: r.parts.len(), h, ns: r.sheets, back: r.info.back });
                        passed = true;
                        break;
                    }
                }
            }
        }
        if !passed {
            out.push(Row::Fail { mode, last });
        }
    }
    out
}

pub fn main() -> i32 {
    // a panic inside run() is a failed candidate, reported as Python reported
    // an exception: not printed as it happens
    std::panic::set_hook(Box::new(|_| {}));
    let which: Vec<String> = std::env::args().skip(1).collect();
    let wanted = |k: &str| which.is_empty() || which.iter().any(|w| w == k);
    println!("THE BUILDS");
    let (v, kw, _, diams) = m::design();
    let _ = m::fingering_table(v, kw, &diams);
    for (kind, _) in BUILDS {
        if !wanted(kind) {
            continue;
        }
        let (parts, info) = build_solid(kind, &diams).unwrap_or_else(|e| lasermade_tools::die(e));
        let (same, across, tilt, reach) = hands(&info, &diams);
        let f0 = m::freq(0.0, kw, info.v);
        println!(
            "  {kind:28} {:6.1}cm3 {:4} parts {:3}  idx-mid {same:5.1}  L-R {across:5.1}  tilt {tilt:5.1}  win-hole {reach:5.1}  chin-back {:.1}",
            info.v / 1e3,
            note_name(f0),
            parts.len(),
            info.back
        );
    }
    println!("\nCANDIDATES");
    for (name, v) in candidates() {
        if !wanted(&name) {
            continue;
        }
        let (th, faces, es) = dihedrals(&v);
        let ls: Vec<f64> = es.iter().map(|e| e.l).collect();
        let (lmin, lmax) = (ls.iter().copied().fold(f64::INFINITY, py::min), ls.iter().copied().fold(f64::NEG_INFINITY, py::max));
        if lmax / lmin > 1.001 {
            println!("  {name:32} BAD CONSTRUCTION: edges {lmin:.3} to {lmax:.3}");
            continue;
        }
        let deg = degree(&faces);
        let e = es.len();
        if th < 90.0 - 1e-6 {
            println!("  {name:32} skip: a {th:.1} degree joint (needs 90 or more)");
            continue;
        }
        let vv = v.clone();
        let sized = std::panic::catch_unwind(move || {
            let edge = size(&vv);
            (edge, faces_of_v(&vv, edge, 0))
        });
        let (edge, fs) = match sized {
            Ok(x) => x,
            Err(ex) => {
                println!("  {name:32} skip: {}", panic_text(ex));
                continue;
            }
        };
        let tops: Vec<&Face> = fs.iter().filter(|f| f.name == "top").collect();
        let ranks: Vec<usize> = if regular_top(&v) { vec![0] } else { vec![0, 1] };
        println!(
            "  {name:32} {} faces, {e} edges, min joint {th:.1} deg, up to {deg} at a corner, edge {edge:.1}mm, top a {}-gon",
            fs.len(),
            tops[0].verts.len()
        );
        for &rank in &ranks {
            if ranks.len() > 1 {
                println!("    the beak on the top's edges of rank {rank}");
            }
            for row in report(&faces_of_v(&v, edge, rank), if deg == 3 { 0.0 } else { 4.25 }) {
                match row {
                    Row::Fail { mode, last } => println!("      {mode:5}: fails {last}"),
                    Row::Pass { mode, nip, v, f0, np, h: (same, across, tilt, reach), ns, back } => println!(
                        "      {mode:5}: PASS nip {}  {:.1}cm3 {} {f0:.1}Hz  parts {np}, sheets {ns}  idx-mid {same:.1}  L-R {across:.1}  tilt {tilt:.1}  win-hole {reach:.1}  chin-back {back:.1}",
                        py::g(nip),
                        v / 1e3,
                        note_name(f0)
                    ),
                }
            }
        }
    }
    0
}

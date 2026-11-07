//! What the panpipes and the chanters borrow from four_hole_ocarinas.py and
//! polyhedron.py, in Rust: the mitred offset, and the finger-jointed solid
//! (faces from planes, edges, cells, a plate's outline, the inner cavity)
//! that tube_panpipes builds its tubes with. The Python stays for the
//! ocarinas; the two give the same numbers, which the panpipes' .repro gate
//! proves.
pub mod polyhedron;

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

#!/usr/bin/env python3
"""Convex solids finger-jointed on every edge, at any angle the faces meet:
the cube, the octagonal prism, the square antiprism, the octahedron and the
dodecahedron that four_hole_ocarinas.py builds.

A solid is any convex polyhedron, given as its faces' planes, and every
plate is that face's polygon, 3mm thick, inward from the outside surface.

THE JOINT, in the plane across an edge. Two plates meet with their outside
faces on the edge line, at the dihedral angle theta, measured inside the
solid. Along the edge the run is cut into cells that alternate between the
two plates. In a plate's own cell it reaches the edge line; in the other's,
it stops short by

    clear(theta) = T sin(theta)       (theta from 90 to 180 degrees)

the least that keeps its 3mm slab out of the neighbour's. At 90 degrees that
is T, the ordinary finger joint. At 135, the octagon's walls, it is 2.12mm:
the end of the short plate then touches the inner corner of the long one,
so the joint closes on a line, and a V-shaped notch shows outside, which the
shellac fills. Every angle here is 90 or more, where no finger pokes out of
the outside surface. Boxes' Platonic generator, by contrast, cuts every
finger a full T deep, as at 90 degrees, whatever the angle.

THE CORNERS. One face owns each vertex -- the top or the bottom where the
vertex is on one, and elsewhere the first face found -- and the cells at
that end of its two edges are its own. The other edges there take no part
in that choice. Where three faces meet, every pair shares an edge, and that
is all. Where four meet, as on the square antiprism and the octahedron,
the two plates that share only the corner run into each other just inside
it, so outline() can nip every corner off, square to its bisector.

HOW MANY CELLS. As many as fit with none under FINGER, the square flute's
6mm, and of the parity the corners need: odd when both ends belong to the
same plate, even when they belong to different ones. An edge whose ends no
plate owns takes an even count, starting with whichever plate walks it from
that end, so every wall of the prism is the same part.
"""
import itertools
import math

T = 3.0
FINGER = 6.0


# --- vectors -------------------------------------------------------------------

def add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def sub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def mul(a, k):
    return tuple(x * k for x in a)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0])


def unit(a):
    return mul(a, 1 / math.sqrt(dot(a, a)))


# --- solids --------------------------------------------------------------------

class Face:
    """One face: its outward normal n, its plane n.p = off, and its corners
    in order, anticlockwise seen from outside."""

    def __init__(self, name, n, off, verts):
        self.name, self.n, self.off = name, unit(n), off
        c = mul(tuple(map(sum, zip(*verts))), 1 / len(verts))
        u = unit(sub(verts[0], c))
        w = cross(self.n, u)
        self.verts = sorted(verts, key=lambda p: math.atan2(dot(sub(p, c), w),
                                                            dot(sub(p, c), u)))
        self.centre = c


def solid(planes, verts):
    """Faces from planes and vertices: each plane's face is the vertices that
    lie on it."""
    faces = []
    for name, n, off in planes:
        n = unit(n)
        on = [v for v in verts if abs(dot(v, n) - off) < 1e-6]
        faces.append(Face(name, n, off, on))
    return faces


def octagonal_prism(flats, height):
    """A regular octagonal prism, flats across and height tall, outside: its
    axis z, z from 0 to height, a flat facing -x for the mouth end."""
    R = flats / 2 / math.cos(math.pi / 8)
    ring = [(R * math.cos(math.pi / 8 + k * math.pi / 4),
             R * math.sin(math.pi / 8 + k * math.pi / 4)) for k in range(8)]
    verts = [(x, y, z) for x, y in ring for z in (0.0, height)]
    planes = [('top', (0, 0, 1), height), ('bottom', (0, 0, -1), 0.0)]
    for k in range(8):
        a = k * math.pi / 4
        planes.append(('mouth wall' if k == 4 else 'wall',
                       (math.cos(a), math.sin(a), 0), flats / 2))
    return solid(planes, verts)


def stand(verts, edge, name):
    """A solid from its corners, scaled so every edge is `edge` long, then
    turned so that the face nearest +z lies flat on top and its edge
    nearest -x faces -x squarely, for the beak. Faces are named top, bottom
    if there is one, `name` for the rest, and 'mouth ' + name for the one
    under the beak."""
    e0 = min(dist(a, b) for a, b in itertools.combinations(verts, 2))
    verts = [mul(v, edge / e0) for v in verts]
    n, _ = max(hull(verts), key=lambda p: (round(p[0][2], 9), -p[0][0], -p[0][1]))
    # the top face's normal onto +z, then the edge midpoint nearest -x onto -x
    ex = unit(cross((0.0, 1.0, 0.0), n)) if abs(n[1]) < 0.9 else unit(cross(n, (1.0, 0.0, 0.0)))
    ey = cross(n, ex)
    verts = [(dot(v, ex), dot(v, ey), dot(v, n)) for v in verts]
    zt = max(v[2] for v in verts)
    top = [v for v in verts if abs(v[2] - zt) < 1e-6]
    c = mul(tuple(map(sum, zip(*top))), 1 / len(top))
    top.sort(key=lambda p: math.atan2(p[1] - c[1], p[0] - c[0]))
    mids = [mul(add(top[i], top[(i + 1) % len(top)]), 0.5) for i in range(len(top))]
    m = min(mids, key=lambda p: math.atan2(p[1] - c[1], -(p[0] - c[0])) ** 2)
    a = math.pi - math.atan2(m[1] - c[1], m[0] - c[0])
    ca, sa = math.cos(a), math.sin(a)
    verts = [(round(x * ca - y * sa, 12), round(x * sa + y * ca, 12), round(z, 12))
             for x, y, z in verts]
    planes = []
    for n, off in hull(verts):
        planes.append(('top' if n[2] > 1 - 1e-9 else 'bottom' if n[2] < -1 + 1e-9
                       else name, n, off))
    faces = solid(planes, verts)
    top = next(f for f in faces if f.name == 'top')
    xe = min(v[0] for v in top.verts)
    for f in faces:
        on = [v for v in f.verts if abs(v[0] - xe) < 1e-6 and abs(v[2] - top.off) < 1e-6]
        if f.name == name and len(on) == 2:
            f.name = 'mouth ' + name
    return faces


def cube(edge):
    """The cube, every edge `edge` long, centred on the origin."""
    return stand(list(itertools.product((1, -1), repeat=3)), edge, 'side')


def square_antiprism(edge):
    """The square antiprism, every edge `edge` long: two squares, top and
    bottom, a quarter turn apart, and eight equilateral triangles between
    them. Its height is edge / 2**(1/4), which makes the slanting edges
    as long as the squares'."""
    r, h = 1 / math.sqrt(2), 2 ** -0.25
    verts = [(r * math.cos(k * math.pi / 2 + s * math.pi / 4),
              r * math.sin(k * math.pi / 2 + s * math.pi / 4), (h if s else -h) / 2)
             for k in range(4) for s in (0, 1)]
    return stand(verts, edge, 'triangle')


def octahedron(edge):
    """The regular octahedron, every edge `edge` long, standing on a face:
    eight equilateral triangles, four to a corner."""
    verts = [tuple(s if i == ax else 0 for i in range(3))
             for ax in range(3) for s in (1, -1)]
    return stand(verts, edge, 'triangle')


def dodecahedron(edge):
    """The regular dodecahedron, every edge `edge` long, standing on a face:
    twelve regular pentagons, three to a corner. Its corners are the cube's
    (+-1, +-1, +-1) and the cyclic permutations of (0, +-1/phi, +-phi)."""
    p = (1 + math.sqrt(5)) / 2
    verts = list(itertools.product((1, -1), repeat=3))
    for a, b in itertools.product((1 / p, -1 / p), (p, -p)):
        verts += [(0, a, b), (a, b, 0), (b, 0, a)]
    return stand(verts, edge, 'pentagon')


def hull(verts):
    """The planes of the convex hull of verts, as (outward unit normal,
    offset): every plane through three corners with all the others on its
    inner side, each once."""
    out = []
    for p, q, r in itertools.combinations(verts, 3):
        n = cross(sub(q, p), sub(r, p))
        if dot(n, n) < 1e-12:
            continue
        n = unit(n)
        off = dot(n, p)
        side = [dot(n, v) - off for v in verts]
        if max(side) > 1e-6:
            if min(side) < -1e-6:
                continue
            n, off = mul(n, -1), -off
        if not any(dot(n, m) > 1 - 1e-9 for m, _ in out):
            out.append((n, off))
    return out


def edges(faces):
    """Every edge: the two faces, its ends P and Q in the walking order of
    the first face, and the dihedral angle inside the solid."""
    out = []
    for i, A in enumerate(faces):
        k = len(A.verts)
        for j in range(k):
            P, Q = A.verts[j], A.verts[(j + 1) % k]
            for b, B in enumerate(faces):
                if b <= i:
                    continue
                if any(dist(P, v) < 1e-6 for v in B.verts) and \
                        any(dist(Q, v) < 1e-6 for v in B.verts):
                    theta = 180 - math.degrees(math.acos(max(-1, min(1, dot(A.n, B.n)))))
                    out.append(dict(a=i, b=b, P=P, Q=Q, L=dist(P, Q), theta=theta))
    return out


def dist(p, q):
    return math.sqrt(dot(sub(p, q), sub(p, q)))


def clear(theta):
    """How far a plate stops short of the edge line in its neighbour's cell."""
    assert 90 - 1e-9 <= theta <= 180, f'{theta:.1f} degrees: fingers would poke out'
    return T * math.sin(math.radians(theta))


def cells(E, owner_at, force=None):
    """Cut edge E into cells and give each an owner, a face index. owner_at
    maps a vertex to the face that owns it. force(E, s0, s1) may name an
    owner for a cell outright: the top plate under the beak."""
    a, b, L = E['a'], E['b'], E['L']
    ends = [owner_at(E['P']), owner_at(E['Q'])]
    ends = [o if o in (a, b) else None for o in ends]
    n = int(L / FINGER + 1e-9)          # 42 / 6 is 6.999... from the corners
    if ends[0] is not None and ends[1] is not None:
        want = 1 if ends[0] == ends[1] else 0          # odd, or even
    elif ends[0] is None and ends[1] is None:
        want = 0
    else:
        want = None
    if want is not None and n % 2 != want:
        n -= 1
    other = lambda f: b if f == a else a
    if ends[0] is not None:
        own = lambda i: ends[0] if i % 2 == 0 else other(ends[0])
    elif ends[1] is not None:
        own = lambda i: ends[1] if (n - 1 - i) % 2 == 0 else other(ends[1])
    else:
        own = lambda i: a if i % 2 == 0 else b
    w = L / n
    out = []
    for i in range(n):
        s0, s1 = i * w, (i + 1) * w
        o = own(i)
        if force:
            o = force(E, s0, s1, o)
        out.append((s0, s1, o))
    return out


# --- plates --------------------------------------------------------------------

def frame(face, U=None, O=None):
    """A face's drawing frame: origin O and axes U, V in its plane, W inward,
    with U x V = W, so a part drawn with u across and v down the sheet is the
    face seen from outside."""
    W = mul(face.n, -1)
    if U is None:
        U = unit(sub(face.verts[1], face.verts[0]))
    V = cross(W, U)
    return (O or face.centre), U, V, W


def to2d(fr, p):
    O, U, V, W = fr
    d = sub(p, O)
    return (dot(d, U), dot(d, V))


def matrix(fr):
    O, U, V, W = fr
    return [[U[i], V[i], W[i], O[i]] for i in range(3)]


def outline(fi, face, fr, all_edges, cellmap, beak=None, nip=0.0):
    """The plate's outline in its frame: each edge walked from corner to
    corner, at depth 0 in its own cells and clear(theta) inside them in its
    neighbour's; corners where two edges meet set back to where the two set
    lines cross. `beak`, (s0, s1, depth, edge index), moves a span of one
    edge to -depth: out past it for the top's beak, in from it for the face
    under the beak. `nip` cuts every corner off square to its bisector,
    that far in from the solid's corner: where more than three plates meet,
    the ones that share only the corner would otherwise run into each other
    just inside it."""
    k = len(face.verts)
    P2 = [to2d(fr, v) for v in face.verts]
    c2 = (sum(p[0] for p in P2) / k, sum(p[1] for p in P2) / k)
    runs = []
    for j in range(k):
        A, B = face.verts[j], face.verts[(j + 1) % k]
        E = next(e for e in all_edges if fi in (e['a'], e['b']) and
                 ({tuple(map(lambda x: round(x, 6), e['P'])), tuple(map(lambda x: round(x, 6), e['Q']))}
                  == {tuple(map(lambda x: round(x, 6), A)), tuple(map(lambda x: round(x, 6), B))}))
        L, d = E['L'], clear(E['theta'])
        same = dist(E['P'], A) < 1e-6
        segs = []
        for s0, s1, o in cellmap[id(E)]:
            if not same:
                s0, s1 = L - s1, L - s0
            segs.append((s0, s1, 0.0 if o == fi else d))
        segs.sort()
        if beak and beak[3] == j:
            b0, b1, depth = beak[0], beak[1], beak[2]
            cut = []
            for s0, s1, dp in segs:
                if s1 <= b0 or s0 >= b1:
                    cut.append((s0, s1, dp))
                    continue
                if s0 < b0:
                    cut.append((s0, b0, dp))
                cut.append((max(s0, b0), min(s1, b1), -depth))
                if s1 > b1:
                    cut.append((b1, s1, dp))
            segs = cut
        p, q = P2[j], P2[(j + 1) % k]
        L2 = math.hypot(q[0] - p[0], q[1] - p[1])
        dx, dy = (q[0] - p[0]) / L2, (q[1] - p[1]) / L2
        nx, ny = -dy, dx
        if (c2[0] - p[0]) * nx + (c2[1] - p[1]) * ny < 0:
            nx, ny = -nx, -ny
        runs.append((p, (dx, dy), (nx, ny), segs))
    pts = []
    for j, (p, dv, nv, segs) in enumerate(runs):
        at = lambda s, dp: (p[0] + s * dv[0] + dp * nv[0], p[1] + s * dv[1] + dp * nv[1])
        line = [at(s, dp) for s0, s1, dp in segs for s in (s0, s1)]
        # the corner at this run's start: where this run's first set line
        # crosses the last set line of the run before
        pp, pd, pn, psegs = runs[j - 1]
        X = cross2(pp, pd, pn, psegs[-1][2], p, dv, nv, segs[0][2])
        pts += [X] + line[1:-1]
    if nip:
        for j in range(k):
            pts = clip(pts, P2[j], P2[j - 1], P2[(j + 1) % k], c2, nip)
    return simplify(pts)


def clip(pts, corner, prev, nxt, centre, d):
    """The polygon pts with its corner cut off: everything nearer `corner`
    than d, measured along the line to the centre, and inside the corner's
    own two edges -- so a beak running out past an edge is left alone.
    Sutherland-Hodgman against the cutting line."""
    L = math.hypot(centre[0] - corner[0], centre[1] - corner[1])
    n = ((centre[0] - corner[0]) / L, (centre[1] - corner[1]) / L)

    def within(p, q):
        """Is p on the centre's side of the line from corner to q?"""
        side = lambda r: ((q[0] - corner[0]) * (r[1] - corner[1])
                          - (q[1] - corner[1]) * (r[0] - corner[0]))
        return side(p) * side(centre) >= -1e-6 * L * L
    keep = lambda p: (p[0] - corner[0]) * n[0] + (p[1] - corner[1]) * n[1] - d
    cut = lambda p: keep(p) < 0 and within(p, prev) and within(p, nxt)
    out = []
    for i in range(len(pts)):
        p, q = pts[i - 1], pts[i]
        if cut(p) != cut(q):
            kp, kq = keep(p), keep(q)
            t = kp / (kp - kq)
            out.append((p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1])))
        if not cut(q):
            out.append(q)
    return out


def cross2(p1, d1, n1, dp1, p2, d2, n2, dp2):
    """Where line 1 (through p1 + dp1 n1, along d1) meets line 2."""
    a = (p1[0] + dp1 * n1[0], p1[1] + dp1 * n1[1])
    b = (p2[0] + dp2 * n2[0], p2[1] + dp2 * n2[1])
    det = d1[0] * (-d2[1]) - d1[1] * (-d2[0])
    r = ((b[0] - a[0]) * (-d2[1]) - (b[1] - a[1]) * (-d2[0])) / det
    return (a[0] + r * d1[0], a[1] + r * d1[1])


def simplify(pts):
    out = []
    for p in pts:
        p = (round(p[0], 6), round(p[1], 6))
        if not out or p != out[-1]:
            out.append(p)
    if out[0] == out[-1]:
        out.pop()
    changed = True
    while changed:
        changed = False
        for i in range(len(out)):
            a, b, c = out[i - 1], out[i], out[(i + 1) % len(out)]
            if abs((b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0])) < 1e-9:
                out.pop(i)
                changed = True
                break
    return out


# --- the inside ----------------------------------------------------------------

def inner(faces):
    """The cavity: every face's plane moved T inward. Its corners are where
    three of those planes meet inside all the rest; its volume, the sum over
    its faces of a third of the plane's distance from the origin times the
    face's area (the origin is inside every solid here). Returns the volume
    and each face's inner polygon, as 3D points."""
    planes = [(f.n, f.off - T) for f in faces]
    pts = []
    for (n1, o1), (n2, o2), (n3, o3) in itertools.combinations(planes, 3):
        det = dot(n1, cross(n2, n3))
        if abs(det) < 1e-9:
            continue
        p = mul(add(add(mul(cross(n2, n3), o1), mul(cross(n3, n1), o2)),
                    mul(cross(n1, n2), o3)), 1 / det)
        if all(dot(n, p) <= o + 1e-6 for n, o in planes):
            if not any(dist(p, q) < 1e-6 for q in pts):
                pts.append(p)
    vol, polys = 0.0, []
    for (n, o), f in zip(planes, faces):
        on = [p for p in pts if abs(dot(n, p) - o) < 1e-6]
        if len(on) < 3:
            polys.append([])
            continue
        g = Face('', n, o, on)
        polys.append(g.verts)
        area = 0.0
        for i in range(len(g.verts)):
            area += dot(cross(g.verts[i], g.verts[(i + 1) % len(g.verts)]), n) / 2
        vol += o * area / 3
    return vol, polys


def joint_points(E, faces, cellmap):
    """Points to test the joint of edge E with: in every cell, near its ends
    and at its middle, along the line from the outside edge to the inner
    corner where both plates' inner faces meet -- the line any leak through
    that joint would follow."""
    A, B = faces[E['a']], faces[E['b']]
    wa, wb = mul(A.n, -1), mul(B.n, -1)
    c = dot(wa, wb)
    I = mul(add(wa, wb), T / (1 + c))
    d = unit(sub(E['Q'], E['P']))
    out = []
    for s0, s1, o in cellmap[id(E)]:
        for f in (0.08, 0.5, 0.92):
            s = s0 + f * (s1 - s0)
            for lam in (0.12, 0.5, 0.88):
                out.append((s, add(add(E['P'], mul(d, s)), mul(I, lam))))
    return out

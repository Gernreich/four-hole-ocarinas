#!/usr/bin/env python3
"""Cut files for a four-hole ocarina in G4, English fingering, in 3mm ply.

    python3 four_hole_ocarinas.py --no-write          # the checks, write nothing
    python3 four_hole_ocarinas.py --out=DIR           # every build into DIR/<build>/
    python3 four_hole_ocarinas.py --only=box --out=DIR
    python3 four_hole_ocarinas.py --only=box --flat --out=DIR   # no subfolder
    python3 four_hole_ocarinas.py --mouth=1.2 ...     # the window correction, as the model

The acoustics are ocarina_model.py's: the cavity's volume and the four
holes' sizes. This file only builds what that asks for. An ocarina's holes
go wherever the fingers fall, since a Helmholtz resonator hears their size
and not their place, so the layout here is the hands', not the model's.

THE TWO BUILDS share the cavity, W_IN x D_IN across and deep and as long as
the volume needs, and every opening:

  box     a box finger-jointed on all twelve edges: top, bottom, two side
          walls and two end walls, every edge carrying as many fingers as
          fit at the square flute's 6mm (see joint_cells)
  stack   a laminated stack: a bottom layer, six frame layers with the
          cavity cut out of them, and the top, glued

Both are tuned at the window and the holes. The box had a sliding far plug
to tune with until 2026-10-17, when the author asked for as many finger
joints as possible: the ends became jointed walls, and the plug went.

FROM THE MOUTH END, the same in both:

  beak     the top plate run on past the end, BEAK long and BEAK_W wide:
           what the lips close on
  bird     one layer glued over the beak and on to the window's near
           edge, carrying the duct on its underside as a fill engrave, the
           window's width and about 1mm deep, from the beak's tip to the
           window
  chin     one plain layer glued under the beak, so the mouthpiece is
           three plies thick, 9mm, with the duct near its middle. Until
           2026-10-17 the bird was three layers stacked on top, copied from
           the square flute, and the beak had nothing under it
  window   the true sound hole, WINDOW_W x WINDOW_L, against the cavity's
           mouth-end wall, so the jet crosses it over the cavity
  holes    two rows across the plate: the index fingers' row ROW_1 past the
           window, the middle fingers' ROW_2 past that, each hand's pair in
           a column COLUMNS apart

THE SPLITTING EDGE is the window's far edge, and wants the bevel a laser
cannot cut, as on the square flute: file it, about 30 degrees, on the face
the bird sits on, before the bird goes on. That face is the outside of the
top, and fixes which hand is which: looking at it with the beak up, the
right hand's holes are on the LEFT, because the player sees the other face.

The finger's size, the kerf and the sheet packing are the square flute's,
imported from square_flute.py beside this repository, which imports the path
writer from swept-curve's ribbon_bore.py in turn. The joint itself is drawn
here, since the flute's runs along two edges and this one closes a box.
"""
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, '..', 'square-flute'))
sys.path.insert(0, os.path.join(HERE, '..', 'swept-curve'))
import ocarina_model as M                                      # noqa: E402
# the square flute's box: its finger joint, plates, discs and packer, imported
# so the two instruments are cut with one joint
import square_flute as SF                                      # noqa: E402
import polyhedron as P                                         # noqa: E402
from square_flute import Part, rect, pack                     # noqa: E402
from ribbon_bore import path                                   # noqa: E402

T, KERF, FINGER = SF.T, SF.KERF, SF.FINGER
MIN_WEB = SF.MIN_WEB
BED, MARGIN = SF.BED, SF.MARGIN
BLUE, ORANGE, BLACK = SF.BLUE, SF.ORANGE, SF.BLACK

W_IN = 60.0          # the cavity across: room for two columns of holes
D_IN = 18.0          # and deep: six 3mm frame layers in the stack
SIDE = 6.0           # the stack's frame, all round
BEAK, BEAK_W = 18.0, 24.0
# the beak's front, where the lips go: a half-ellipse across its whole width
# and TIP_ROUND deep, on the top plate, the bird and the chin layer alike,
# so the three plies round off as one (the author, 2026-10-17)
TIP_ROUND, TIP_STEPS = 8.0, 32
ROW_1, ROW_2 = 30.0, 52.0    # hole rows, from the cavity's mouth-end wall
COLUMNS = 36.0               # the two hands' columns, centre to centre
D_MIN, D_MAX = 2.5, 13.0     # a hole that still speaks; one a finger covers
HOLE_GAP = 3.0               # ply between neighbouring holes

BUILDS = {
    'box': 'finger-jointed box',
    'stack': 'laminated stack',
    'octagon': 'regular octagonal prism, finger-jointed',
    'cube': 'cube, finger-jointed',
    'square-antiprism': 'square antiprism, finger-jointed',
    'octahedron': 'regular octahedron, finger-jointed',
    'dodecahedron': 'regular dodecahedron, finger-jointed',
}
# The second set, since 2026-10-17, and those after it: solids, built by polyhedron.py,
# whose cavity is what their shape holds and whose key follows from it. The
# holes are sized against the window, not the cavity, so every fingering
# and every interval is the first set's; only the key moves.
#   octagon         flats across and height, outside: bigger than the box on
#                   all three axes, at the author's asking
OCT_FLATS, OCT_HEIGHT = 110.0, 36.0
# The other solids, the same day, every edge a whole number of 6mm fingers:
#   cube, square-antiprism  72mm, so the top square takes the window and
#                   the four holes on the octagon's grid; the antiprism's
#                   square is a quarter turn from the one below it, with
#                   eight triangles between
#   octahedron      standing on a triangle, every edge OCTA_EDGE; the top is
#                   too small for four holes, so each hand has a triangle of
#                   its own, the index finger's hole nearer the beak
#   dodecahedron    standing on a pentagon, every edge DODECA_EDGE; each hole
#                   has a pentagon of its own, the index fingers' the two
#                   either side of the mouth, the middle fingers' the next
# Four plates meet at the antiprism's and the octahedron's corners, and
# NIP cuts each corner back that far, square to its bisector, so that the
# two sharing only the corner stay apart.
CUBE_EDGE = 72.0
SAP_EDGE = 72.0
OCTA_EDGE = 72.0
DODECA_EDGE = 36.0
NIP = {'square-antiprism': 4.25, 'octahedron': 4.75}
SOLIDS = ('octagon', 'cube', 'square-antiprism', 'octahedron', 'dodecahedron')
FACE_HOLES = ('octahedron', 'dodecahedron')


def cavity_length(V):
    return V / (W_IN * D_IN)


def hole_places(PW):
    """(x, y) of each hole in HOLES order, x from the cavity's mouth-end
    wall. The right hand at y > PW/2: on the sheet, and on the outside face
    with the beak up, that is the viewer's left and the player's right."""
    yc = PW / 2
    return {'R2': (ROW_2, yc + COLUMNS / 2), 'R1': (ROW_1, yc + COLUMNS / 2),
            'L1': (ROW_1, yc - COLUMNS / 2), 'L2': (ROW_2, yc - COLUMNS / 2)}


def rounded(poly, yc):
    """A polygon with its two corners at the beak's tip, x = -BEAK, swapped
    for the half-ellipse there: the tip's TIP_ROUND deep and BEAK_W/2 across
    from the centre line, sampled TIP_STEPS times over the half. A corner
    inside the beak's width -- the duct's -- lands on the curve where its
    own y meets it, so the duct's mouth follows the tip's."""
    poly = [(-BEAK, y) if abs(x + BEAK) < 1e-6 else (x, y) for x, y in poly]
    ix = [i for i, (x, y) in enumerate(poly) if x == -BEAK]
    assert len(ix) == 2 and (ix[1] - ix[0]) % len(poly) in (1, len(poly) - 1), \
        'the beak\'s tip must be one edge of the polygon'
    if ix[1] - ix[0] != 1:              # the tip edge wraps round the end
        ix = ix[::-1]
    i, j = ix
    b = BEAK_W / 2
    t0, t1 = (math.asin((poly[k][1] - yc) / b) for k in (i, j))
    arc = [(-BEAK + TIP_ROUND * (1 - math.cos(t)), yc + b * math.sin(t))
           for t in [t0 + (t1 - t0) * k / TIP_STEPS for k in range(TIP_STEPS + 1)]]
    if j == 0:                          # the curve closes the polygon
        return poly[1:i] + arc
    return poly[:i] + arc + poly[j + 1:]


def offset(poly, d):
    """Any simple polygon grown by d on every side (shrunk if d < 0): each
    vertex moves along its two edges' outward normals, mitred, so both edges
    move d. The square flute's offset() adds the normals, which is the same
    at a right angle and wrong at any other, and the beak's tip has no right
    angles."""
    s = 1 if sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1)
                 in zip(poly, poly[1:] + poly[:1])) > 0 else -1
    n = len(poly)
    out = []
    for i in range(n):
        p0, p1, p2 = poly[i - 1], poly[i], poly[(i + 1) % n]

        def normal(a, b):
            dx, dy = b[0] - a[0], b[1] - a[1]
            L = math.hypot(dx, dy)
            return (s * dy / L, -s * dx / L)
        na, nb = normal(p0, p1), normal(p1, p2)
        k = d / (1 + na[0] * nb[0] + na[1] * nb[1])
        out.append((p1[0] + k * (na[0] + nb[0]), p1[1] + k * (na[1] + nb[1])))
    return out


def mouthpiece(x1, yc):
    """The bird's or the chin layer's outline: the beak's width, from its
    rounded tip back to x1."""
    return rounded(rect(-BEAK, yc - BEAK_W / 2, x1, yc + BEAK_W / 2), yc)


def duct(c0, yc):
    """The duct, from the rounded tip to the window's near edge at c0."""
    return rounded(rect(-BEAK, yc - M.WINDOW_W / 2, c0, yc + M.WINDOW_W / 2), yc)


def beaked(outline, x0, PW):
    """A plate outline closing along its mouth end, x = x0, with the beak
    let into that end: BEAK_W wide, centred, BEAK long."""
    assert outline[0] == (x0, 0) and outline[-1] == (x0, PW)
    yc = PW / 2
    return outline + [(x0, yc + BEAK_W / 2), (x0 - BEAK, yc + BEAK_W / 2),
                      (x0 - BEAK, yc - BEAK_W / 2), (x0, yc - BEAK_W / 2)]


# --- the box's joint -----------------------------------------------------------

def joint_cells(L):
    """How many fingers an edge with L of inner length takes: the most an odd
    count allows with none under FINGER, the square flute's 6mm. Odd, so both
    ends of the run belong to the same plate and the pattern is the same read
    from either end; at 6mm, because that is the joint whose fit is known."""
    n = int(L // FINGER)
    return n if n % 2 else n - 1


def run(L, owned, out=()):
    """One edge's cells, as (s0, s1, depth) from the edge's start: the run of
    L between the two corner cubes, cut into joint_cells(L), at depth 0 where
    this plate owns the cell and T where its neighbour's finger comes in.
    owned(i) says which; a cell overlapping any (a, b) span in `out` is
    owned whatever owned() says, and that span is pushed out past the edge by
    its third member: the top plate's beak."""
    n = joint_cells(L)
    w = L / n
    segs = []
    for i in range(n):
        a, b = T + i * w, T + (i + 1) * w
        mine = owned(i) or any(a < s1 and b > s0 for s0, s1, _ in out)
        segs.append((a, b, 0.0 if mine else T))
    for s0, s1, d in out:
        cut = []
        for a, b, dep in segs:
            if b <= s0 or a >= s1:
                cut.append((a, b, dep))
                continue
            if a < s0:
                cut.append((a, s0, dep))
            cut.append((max(a, s0), min(b, s1), -d))
            if b > s1:
                cut.append((s1, b, dep))
        segs = cut
    return segs


def jointed(A, B, edges, corners):
    """A plate A x B whose four edges are finger runs: edges[k] is run()'s
    list for edge k, walked anticlockwise on the sheet from (0, 0) -- v = 0,
    u = A, v = B, u = 0 -- and corners[k] says whether this plate owns the
    corner cube at the start of edge k. An owned corner is the plate's own
    square corner; one it does not own is a T x T notch."""
    starts = [(0, 0), (A, 0), (A, B), (0, B)]
    dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    norms = [(0, 1), (-1, 0), (0, -1), (1, 0)]
    lens = [A, B, A, B]
    pts = []
    for k in range(4):
        (px, py), (dx, dy), (nx, ny), L = starts[k], dirs[k], norms[k], lens[k]
        at = lambda s, d: (px + s * dx + d * nx, py + s * dy + d * ny)
        pts += [at(0, 0), at(T, 0)] if corners[k] else [at(T, T)]
        for a, b, d in edges[k]:
            pts += [at(a, d), at(b, d)]
        pts += [at(L - T, 0), at(L, 0)] if corners[(k + 1) % 4] else [at(L - T, T)]
    return simplify(pts)


def simplify(pts):
    """Drop repeated points and points on a straight run: offset() moves a
    vertex along the sum of its edges' normals, which doubles the step at a
    point between two collinear edges."""
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
            if (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0]) == 0:
                out.pop(i)
                changed = True
                break
    return out


def box_parts(Lc, PW, Hz, num):
    """The six plates of a box Lc x W_IN x D_IN inside, jointed on every edge.
    Each edge's cells alternate between its two plates, and the corner cubes
    go to the side walls, so every bit of every edge belongs to exactly one
    plate:

      top, bottom   the even cells of all four edges (each run starts and
                    ends with one); the top also keeps the cells under the
                    beak at the mouth end, so the beak runs on from solid ply
                    and the duct crosses no seam
      side walls    the odd cells along the top and bottom, the even cells
                    up the ends, and all four corners
      end walls     the odd cells all round, less those under the beak

    Each plate is placed by p.m, a 3x4 matrix from its own (u, v, out of the
    sheet) to the box's (x along from the mouth end, y across, z up), for
    the 3D view and the check that the plates tile the box."""
    Lx = Lc + 2 * T
    even, odd = (lambda i: i % 2 == 0), (lambda i: i % 2 == 1)
    beak = ((PW - BEAK_W) / 2, (PW + BEAK_W) / 2, BEAK)
    tb = lambda b: [run(Lc, even), run(W_IN, even), run(Lc, even),
                    run(W_IN, even, (beak,) if b else ())]
    side = [run(Lc, odd), run(D_IN, even), run(Lc, odd), run(D_IN, even)]
    end = lambda b: [run(W_IN, lambda i: odd(i) and not (
        T + i * W_IN / joint_cells(W_IN) < beak[1] and
        T + (i + 1) * W_IN / joint_cells(W_IN) > beak[0]) if b else odd(i)),
        run(D_IN, odd), run(W_IN, odd), run(D_IN, odd)]
    none, every = [False] * 4, [True] * 4
    spec = [
        ('top plate', jointed(Lx, PW, tb(True), none),
         [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, Hz - T]]),
        ('bottom plate', jointed(Lx, PW, tb(False), none),
         [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0]]),
        ('side wall', jointed(Lx, Hz, side, every),
         [[1, 0, 0, 0], [0, 0, 1, 0], [0, -1, 0, Hz]]),
        ('side wall', jointed(Lx, Hz, side, every),
         [[1, 0, 0, 0], [0, 0, 1, PW - T], [0, -1, 0, Hz]]),
        ('end wall', jointed(PW, Hz, end(True), none),
         [[0, 0, 1, 0], [-1, 0, 0, PW], [0, -1, 0, Hz]]),
        ('end wall', jointed(PW, Hz, end(False), none),
         [[0, 0, 1, Lx - T], [-1, 0, 0, PW], [0, -1, 0, Hz]]),
    ]
    notes = {'top plate': 'the window and the four holes; file the window\'s far edge '
                          'to a 30 degree bevel on the outside face before the bird goes on',
             'bottom plate': 'plain', 'side wall': 'fingers on all four edges',
             'end wall': 'fingers on all four edges'}
    parts = []
    for name, outline, m in spec:
        p = Part(name, next(num), outline, notes[name])
        p.m = m
        parts.append(p)
    parts[4].note = 'the mouth end: no fingers into the top under the beak'
    return parts


def build(kind, V, diams):
    """Every part of one build, in its own flat coordinates: x along the
    ocarina from the mouth end of the body (the beak runs to -BEAK), y
    across it."""
    Lc = cavity_length(V)
    if kind == 'box':
        PW = W_IN + 2 * T
        c0 = T                                  # the cavity's mouth-end wall
        total = c0 + Lc + T
    else:
        PW = W_IN + 2 * SIDE
        c0 = SIDE
        total = c0 + Lc + SIDE
    yc = PW / 2
    places = {h: (c0 + x, y) for h, (x, y) in hole_places(PW).items()}
    parts = []
    num = iter(range(1, 100))

    def top_plate(outline):
        p = Part('top plate', next(num), rounded(outline, yc),
                 'the window and the four holes; file the window\'s far edge to a '
                 '30 degree bevel on the outside face before the bird goes on')
        p.slot(c0, yc - M.WINDOW_W / 2, c0 + M.WINDOW_L, yc + M.WINDOW_W / 2)
        for h, d in zip(M.HOLES, diams):
            p.hole(*places[h], d)
        return p

    if kind == 'box':
        box = box_parts(Lc, PW, D_IN + 2 * T, num)
        top = top_plate(box[0].outline)
        top.m = box[0].m
        parts += [top] + box[1:]
    else:
        parts.append(top_plate(beaked(rect(0, 0, total, PW), 0, PW)))
        parts.append(Part('bottom layer', next(num), rect(0, 0, total, PW), 'plain'))
        for k in range(int(round(D_IN / T))):
            p = Part('frame layer', next(num), rect(0, 0, total, PW),
                     'the cavity cut out')
            p.slot(c0, SIDE, c0 + Lc, SIDE + W_IN)
            parts.append(p)
    p = Part('bird', next(num), mouthpiece(c0, yc),
             'the duct fill-engraved in its underside, about 1mm deep')
    p.fills.append(duct(c0, yc))
    parts.append(p)
    # under the beak only: the body starts at x = 0
    parts.append(Part('chin layer', next(num), mouthpiece(0, yc),
                      'glued under the beak, its end against the body'))
    return parts, dict(total=total, PW=PW, c0=c0, Lc=Lc, places=places)


# --- checks -----------------------------------------------------------------

def checks(kind, parts, info, V, diams, worst):
    out = []

    def note(ok, what, detail):
        out.append((ok, what, detail))
    PW, c0, Lc = info['PW'], info['c0'], info['Lc']
    yc = PW / 2
    note(worst < 12.5, 'the model holds the scale',
         f'worst {worst:.1f} cents of eight notes')
    built = W_IN * D_IN * Lc
    note(abs(built / V - 1) < 1e-6, 'the cavity is the volume the model asks for',
         f'{W_IN:g} x {D_IN:g} x {Lc:.2f}mm = {built / 1000:.2f}cm^3')
    note(min(diams) >= D_MIN and max(diams) <= D_MAX,
         'every hole speaks and a finger covers it',
         f'ø{min(diams):.2f} to ø{max(diams):.2f}mm, against {D_MIN:g} and {D_MAX:g}')
    web = min(min(x - d / 2 - (c0 + M.WINDOW_L), c0 + Lc - (x + d / 2),
                  y - d / 2 - (yc - W_IN / 2), yc + W_IN / 2 - (y + d / 2))
              for (x, y), d in zip(info['places'].values(), diams))
    note(web >= MIN_WEB, 'every hole opens into the cavity, clear of its walls',
         f'nearest wall or window {web:.1f}mm off')
    pl = list(zip(info['places'].values(), diams))
    gap = min(math.hypot(a[0] - b[0], a[1] - b[1]) - (da + db) / 2
              for i, (a, da) in enumerate(pl) for b, db in pl[i + 1:])
    note(gap >= HOLE_GAP, 'ply left between neighbouring holes', f'narrowest {gap:.1f}mm')
    note(c0 + M.WINDOW_L < c0 + Lc and M.WINDOW_W < W_IN,
         'the window opens into the cavity at its mouth-end wall',
         f'{M.WINDOW_W:g} x {M.WINDOW_L:g}mm from x = {c0:g}')
    bird = [p for p in parts if p.fills][0]
    fx = [q[0] for q in bird.fills[0]]
    note(min(fx) == -BEAK and max(fx) == c0 == bird.x0 + bird.w,
         'the duct runs from the beak\'s tip to the window',
         f'{c0 + BEAK:g}mm long, {M.WINDOW_W:g}mm wide')
    chin = [p for p in parts if p.name == 'chin layer']
    note(len(chin) == 1 and chin[0].x0 == -BEAK and chin[0].x0 + chin[0].w == 0
         and chin[0].h == BEAK_W,
         'the chin layer fills the beak and stops at the body',
         f'{BEAK:g} x {BEAK_W:g}mm; the mouthpiece {3 * T:g}mm thick')
    if kind == 'box':
        n = [joint_cells(L) for L in (Lc, W_IN, D_IN)]
        w = min(L / k for L, k in zip((Lc, W_IN, D_IN), n))
        note(all(k >= 3 for k in n) and w >= FINGER,
             'every edge is finger-jointed, as many as fit',
             f'{n[0]} cells along, {n[1]} across, {n[2]} up; narrowest {w:.2f}mm')
        bad, seen = tiling(parts, info)
        note(not bad, 'the six plates fill every edge and corner once',
             f'{seen} cells and corners, ' + (f'{len(bad)} wrong: {bad[0]}' if bad else 'each in one plate'))
    sheets = pack(parts)
    note(all(s['w'] <= BED[0] and s['h'] <= BED[1] for s in sheets),
         'every sheet fits the bed',
         f'{len(sheets)} sheet(s), largest '
         f'{max(s["w"] for s in sheets):.0f} x {max(s["h"] for s in sheets):.0f}mm')
    placed = sum(len(s['parts']) for s in sheets)
    note(placed == len(parts), 'every part is on a sheet', f'{placed} of {len(parts)}')
    return out, sheets


def inside(pt, poly):
    """Even-odd point in polygon."""
    x, y = pt
    c = False
    for (x0, y0), (x1, y1) in zip(poly, poly[1:] + poly[:1]):
        if (y0 > y) != (y1 > y) and x < x0 + (y - y0) * (x1 - x0) / (y1 - y0):
            c = not c
    return c


def owners(parts, q):
    """The plates whose solid holds the box-coordinate point q: q taken back
    into each plate's own frame through its matrix, which is a signed
    permutation, so its transpose undoes it."""
    hit = []
    for p in parts:
        m = getattr(p, 'm', None)
        if not m:
            continue
        d = [q[i] - m[i][3] for i in range(3)]
        u, v, w = (sum(m[i][j] * d[i] for i in range(3)) for j in range(3))
        if 0 < w < T and inside((u, v), p.outline):
            hit.append(p.name)
    return hit


def tiling(parts, info):
    """Every finger cell of all twelve edges, and all eight corner cubes,
    sampled at its centre and at points near its ends: each must be in
    exactly one plate. A cell in two is a collision on assembly; in none, a
    gap to seal. Also one point in each face's middle, which must be in that
    face's plate alone."""
    Lx, PW, Hz = info['total'], info['PW'], D_IN + 2 * T
    h = T / 2
    pts = []
    axes = [(0, Lx), (1, PW), (2, Hz)]
    for ax, L in axes:
        others = [a for a, _ in axes if a != ax]
        for c0 in (h, (Lx, PW, Hz)[others[0]] - h):
            for c1 in (h, (Lx, PW, Hz)[others[1]] - h):
                n = joint_cells(L - 2 * T)
                w = (L - 2 * T) / n
                for i in range(n):
                    for f in (0.1, 0.5, 0.9):
                        q = [0, 0, 0]
                        q[ax], q[others[0]], q[others[1]] = T + (i + f) * w, c0, c1
                        pts.append((f'{"xyz"[ax]}-edge cell {i}', q))
    for x in (h, Lx - h):
        for y in (h, PW - h):
            for z in (h, Hz - h):
                pts.append(('corner', [x, y, z]))
    for q in ([Lx / 2, PW / 2, h], [Lx / 2, PW / 2, Hz - h], [Lx / 2, h, Hz / 2],
              [h, PW / 2 + BEAK_W, Hz / 2], [Lx - h, PW / 2, Hz / 2]):
        pts.append(('face', q))
    bad = [f'{what} at ({q[0]:.1f}, {q[1]:.1f}, {q[2]:.1f}) in {owners(parts, q) or "none"}'
           for what, q in pts if len(owners(parts, q)) != 1]
    return bad, len(pts)


# --- the solids ----------------------------------------------------------------

def build_solid(kind, diams):
    """Every part of a solid build. The world is the solid's own: z up, the
    hole face on top, the mouth face looking down -x, so the beak runs out
    along -x from the middle of the top's mouth edge. Each part carries its
    frame as p.m, the matrix from its own (u, v, inward) to the world."""
    faces = {'octagon': lambda: P.octagonal_prism(OCT_FLATS, OCT_HEIGHT),
             'cube': lambda: P.cube(CUBE_EDGE),
             'square-antiprism': lambda: P.square_antiprism(SAP_EDGE),
             'octahedron': lambda: P.octahedron(OCTA_EDGE),
             'dodecahedron': lambda: P.dodecahedron(DODECA_EDGE)}[kind]()
    ft = next(i for i, f in enumerate(faces) if f.name == 'top')
    fm = next(i for i, f in enumerate(faces) if f.name.startswith('mouth'))
    top, mouth = faces[ft], faces[fm]
    x_edge = min(v[0] for v in top.verts)      # the top's edge with the mouth face
    z_top = top.off
    nip = NIP.get(kind, 0.0)

    def owner_at(v):
        """The face that owns a corner: the top or the bottom where the corner
        is on one, as every corner is but the dodecahedron's middle ten, and
        there the first face found."""
        on = [i for i, f in enumerate(faces) if abs(P.dot(v, f.n) - f.off) < 1e-6]
        pick = [i for i in on if faces[i].name in ('top', 'bottom')]
        assert len(pick) <= 1, f'corner {v} has {len(pick)} owners'
        return (pick or on)[0]

    def force(E, s0, s1, o):
        """Under the beak the top keeps every cell, so the beak runs on from
        solid ply and the duct crosses no seam."""
        if {E['a'], E['b']} != {ft, fm}:
            return o
        d = P.unit(P.sub(E['Q'], E['P']))
        y0, y1 = sorted((E['P'][1] + s0 * d[1], E['P'][1] + s1 * d[1]))
        return ft if y1 > -BEAK_W / 2 and y0 < BEAK_W / 2 else o
    es = P.edges(faces)
    cellmap = {id(E): P.cells(E, owner_at, force) for E in es}
    parts = []
    num = iter(range(1, 100))
    fr_top = P.frame(top, U=(1.0, 0.0, 0.0), O=(x_edge, 0.0, z_top))
    frames = []
    for i, f in enumerate(faces):
        fr = fr_top if i == ft else P.frame(f)
        frames.append(fr)
        beak = None
        if i == ft:
            # the top's mouth edge, and the beak's span along it
            k = len(f.verts)
            j = next(j for j in range(k) if abs(f.verts[j][0] - x_edge) < 1e-6
                     and abs(f.verts[(j + 1) % k][0] - x_edge) < 1e-6)
            A, B = f.verts[j], f.verts[(j + 1) % k]
            sy = (B[1] - A[1]) / P.dist(A, B)
            b0, b1 = sorted(((-BEAK_W / 2 - A[1]) / sy, (BEAK_W / 2 - A[1]) / sy))
            beak = (b0, b1, BEAK, j)
        if i == fm and mouth.n[2] > 1e-9:
            # a mouth face that slopes away under the beak stops short of it
            # by T / sin of the slope, so its edge clears the beak's
            # underside, where an upright one stops T short like any other
            k = len(f.verts)
            j = next(j for j in range(k) if abs(f.verts[j][0] - x_edge) < 1e-6
                     and abs(f.verts[(j + 1) % k][0] - x_edge) < 1e-6
                     and abs(f.verts[j][2] - z_top) < 1e-6
                     and abs(f.verts[(j + 1) % k][2] - z_top) < 1e-6)
            A, B = f.verts[j], f.verts[(j + 1) % k]
            sy = (B[1] - A[1]) / P.dist(A, B)
            b0, b1 = sorted(((-BEAK_W / 2 - A[1]) / sy, (BEAK_W / 2 - A[1]) / sy))
            beak = (b0, b1, -T / math.sqrt(1 - mouth.n[2] ** 2), j)
        o = P.outline(i, f, fr, es, cellmap, beak, nip)
        if i == ft:
            o = rounded(o, 0.0)
        note = {'top': 'the window' + ('' if kind in FACE_HOLES else ' and the four holes')
                       + '; file the window\'s far edge to a 30 degree bevel on the '
                       'outside face before the bird goes on'}.get(
                    f.name, 'fingers on every edge') + (', the corners nipped' if nip else '')
        p = Part(f.name if f.name != 'top' else 'top plate', next(num), o, note)
        p.m = P.matrix(fr)
        p.face = i
        parts.append(p)
    tp = parts[ft]
    # the window against the mouth face's inner side, the holes from there,
    # the right hand on the player's right: holes face forward (+z), the beak
    # is up (-x), so right = forward x up = -y. On the sheet, drawn from
    # outside with v = -y, the right hand's holes are at v > 0
    tp.slot(T, -M.WINDOW_W / 2, T + M.WINDOW_L, M.WINDOW_W / 2)
    vol, polys = P.inner(faces)
    if kind in FACE_HOLES:
        # the faces round the top on either side of it, left hand +y, right
        # -y, nearest the beak first: the index finger's hole on the first
        # and the middle finger's on the next, each where it is furthest from
        # every wall of its face's inside, or both on one face where a hand
        # has only one
        up = [i for i, f in enumerate(faces) if f.n[2] > 1e-9 and f.name != 'top'
              and not f.name.startswith('mouth')]
        dia = dict(zip(M.HOLES, diams))
        places = {}
        for hand, sgn in (('L', 1), ('R', -1)):
            mine = sorted((i for i in up if faces[i].centre[1] * sgn > 1),
                          key=lambda i: faces[i].centre[0])
            if len(mine) >= 2:
                for h, i in ((hand + '1', mine[0]), (hand + '2', mine[1])):
                    places[h] = (i, deepest([P.to2d(frames[i], c) for c in polys[i]]))
            else:
                i = mine[0]
                c = P.to2d(frames[i], faces[i].centre)
                ahead = P.to2d(frames[i], P.add(faces[i].centre, (1.0, 0.0, 0.0)))
                xy1, xy2 = pair([P.to2d(frames[i], q) for q in polys[i]],
                                P.sub(ahead, c) + (0.0,), dia[hand + '1'], dia[hand + '2'])
                places[hand + '1'], places[hand + '2'] = (i, xy1), (i, xy2)
        places = {h: places[h] for h in M.HOLES}
    else:
        places = {h: (ft, xy) for h, xy in
                  {'R2': (T + ROW_2, COLUMNS / 2), 'R1': (T + ROW_1, COLUMNS / 2),
                   'L1': (T + ROW_1, -COLUMNS / 2), 'L2': (T + ROW_2, -COLUMNS / 2)}.items()}
    for h, d in zip(M.HOLES, diams):
        i, xy = places[h]
        parts[i].hole(*xy, d)
    O, U, V, W = fr_top
    # the chin layer stops where it would run into a sloping mouth face: its
    # lower inside corner just touching the mouth face's outside
    n = mouth.n
    back = max(0.0, (mouth.off - P.dot(n, (x_edge, 0.0, z_top - 2 * T))) / -n[0])
    bird = Part('bird', next(num), mouthpiece(T, 0.0),
                'the duct fill-engraved in its underside, about 1mm deep')
    bird.fills.append(duct(T, 0.0))
    bird.m = P.matrix((P.add(O, (0, 0, T)), U, V, W))
    chin = Part('chin layer', next(num), mouthpiece(-back, 0.0),
                'glued under the beak, its end against the mouth face')
    chin.m = P.matrix((P.sub(O, (0, 0, T)), U, V, W))
    parts += [bird, chin]
    return parts, dict(kind=kind, faces=faces, edges=es, cellmap=cellmap, ft=ft, fm=fm,
                       V=vol, inner=[[P.to2d(fr, q) for q in pl] for fr, pl in zip(frames, polys)],
                       places=places, fr_top=fr_top, frames=frames, back=back, nip=nip,
                       inner_pts=[q for pl in polys for q in pl])


def deepest(poly):
    """The point inside a convex polygon furthest from all its sides, found
    by a grid search that narrows round the best point so far."""
    x0, x1 = min(x for x, _ in poly), max(x for x, _ in poly)
    y0, y1 = min(y for _, y in poly), max(y for _, y in poly)
    best = ((x0 + x1) / 2, (y0 + y1) / 2)
    h = max(x1 - x0, y1 - y0) / 2
    while h > 1e-4:
        best = max(((best[0] + i * h / 4, best[1] + j * h / 4)
                    for i in range(-4, 5) for j in range(-4, 5)),
                   key=lambda c: in_poly_margin(c, 0, poly))
        h /= 2
    return (round(best[0], 3), round(best[1], 3))


def pair(poly, ahead, d1, d2):
    """Two holes, d1 and d2 across, in one convex polygon: the first
    further along `ahead` the other way -- nearer the beak -- than the
    second, placed so that the narrowest of their webs to the walls and
    half the web between them is as wide as it can be, by a grid search
    that narrows round the best so far."""
    a = math.hypot(ahead[0], ahead[1])
    ax, ay = ahead[0] / a, ahead[1] / a
    x0, x1 = min(x for x, _ in poly), max(x for x, _ in poly)
    y0, y1 = min(y for _, y in poly), max(y for _, y in poly)

    def at(b):
        cx, cy, t = b
        return (cx - t * ax, cy - t * ay), (cx + t * ax, cy + t * ay)

    def score(b):
        p, q = at(b)
        return min(in_poly_margin(p, d1 / 2, poly), in_poly_margin(q, d2 / 2, poly),
                   (2 * b[2] - (d1 + d2) / 2) / 2)
    best = ((x0 + x1) / 2, (y0 + y1) / 2, max(x1 - x0, y1 - y0) / 4)
    h = max(x1 - x0, y1 - y0) / 2
    while h > 1e-3:
        best = max(((best[0] + i * h / 3, best[1] + j * h / 3, best[2] + k * h / 3)
                    for i in range(-3, 4) for j in range(-3, 4) for k in range(-3, 4)),
                   key=score)
        h /= 2
    return tuple(tuple(round(x, 3) for x in p) for p in at(best))


def in_poly_margin(c, r, poly):
    """Is the circle about c, radius r, inside the convex polygon poly?"""
    k = len(poly)
    cx = sum(p[0] for p in poly) / k
    cy = sum(p[1] for p in poly) / k
    worst = 1e9
    for i in range(k):
        a, b = poly[i], poly[(i + 1) % k]
        L = math.hypot(b[0] - a[0], b[1] - a[1])
        n = ((b[1] - a[1]) / L, -(b[0] - a[0]) / L)
        if (cx - a[0]) * n[0] + (cy - a[1]) * n[1] < 0:
            n = (-n[0], -n[1])
        worst = min(worst, (c[0] - a[0]) * n[0] + (c[1] - a[1]) * n[1] - r)
    return worst


def solid_checks(kind, parts, info, V0, kw, diams, worst):
    out = []

    def note(ok, what, detail):
        out.append((ok, what, detail))
    note(worst < 12.5, 'the model holds the scale', f'worst {worst:.1f} cents of eight notes')
    V = info['V']
    f0 = M.freq(0, kw, V)
    span = max(P.dist(a, b) for a in info['inner_pts'] for b in info['inner_pts'])
    lam = M.C / f0
    note(span < lam / 4, 'the cavity is small against the wavelength',
         f'{V / 1000:.1f}cm^3, sounding {f0:.1f}Hz; {span:.0f}mm across, '
         f'a quarter wave {lam / 4:.0f}mm')
    poly = info['inner'][info['ft']]
    web = min(in_poly_margin(xy, d / 2, info['inner'][i])
              for (i, xy), d in zip(info['places'].values(), diams))
    win = [(T, -M.WINDOW_W / 2), (T + M.WINDOW_L, -M.WINDOW_W / 2),
           (T + M.WINDOW_L, M.WINDOW_W / 2), (T, M.WINDOW_W / 2)]
    wweb = min(in_poly_margin(c, 0, poly) for c in win[1:3])
    note(web >= MIN_WEB and wweb >= 0, 'every hole and the window open into the cavity',
         f'nearest hole {web:.1f}mm from a wall')
    def world(i, xy):
        O, U, V_, W = info['frames'][i]
        return P.add(O, P.add(P.mul(U, xy[0]), P.mul(V_, xy[1])))
    pl = [(world(*ixy), d) for ixy, d in zip(info['places'].values(), diams)]
    gap = min(P.dist(a, b) - (da + db) / 2
              for i, (a, da) in enumerate(pl) for b, db in pl[i + 1:])
    note(gap >= HOLE_GAP, 'ply left between neighbouring holes', f'narrowest {gap:.1f}mm')
    # the hands, from the geometry rather than the names: right = forward x up
    fwd = info['faces'][info['ft']].n
    up = (-1.0, 0.0, 0.0)
    right = P.cross(fwd, up)
    ok = all((P.dot(world(*ixy), right) > 0) == h.startswith('R')
             for h, ixy in info['places'].items())
    note(ok, 'each hand\'s holes lie under that hand', 'held holes forward, beak up')
    bird = [p for p in parts if p.fills][0]
    fx = [q[0] for q in bird.fills[0]]
    note(abs(min(fx) + BEAK) < 1e-9 and abs(max(fx) - T) < 1e-9,
         'the duct runs from the beak\'s tip to the window',
         f'{T + BEAK:g}mm long, {M.WINDOW_W:g}mm wide')
    es, cm = info['edges'], info['cellmap']
    kinds = {}
    for E in es:
        key = (round(E['theta'], 2), round(E['L'], 2))
        kinds.setdefault(key, set()).add(len(cm[id(E)]))
    widths = [E['L'] / len(cm[id(E)]) for E in es]
    note(min(widths) >= FINGER - 1e-9, 'every edge is finger-jointed, as many as fit',
         f'{len(es)} edges: ' + '; '.join(f'{L:g}mm at {th:g} deg, {"/".join(map(str, sorted(n)))} cells'
                                         for (th, L), n in sorted(kinds.items()))
         + f'; narrowest {min(widths):.2f}mm')
    coll, void, n = [], [], 0
    verts = {tuple(round(x, 6) for x in v) for f in info['faces'] for v in f.verts}
    for E in es:
        for s_, q in P.joint_points(E, info['faces'], cm):
            n += 1
            o = owners(parts, q)
            if len(o) > 1:
                coll.append((E, q, o))
            elif not o:
                void.append(min(P.dist(q, v) for v in verts))
    note(not coll, 'no two plates in the same place, along every joint',
         f'{n} points sampled' + (f', {len(coll)} in two plates' if coll else ', each in one plate or none'))
    far = max(void) if void else 0
    note(far <= 2 * T, 'every joint closes, but within a finger of a corner',
         f'{n - len(void)} of {n} points in a plate; the rest at corners, '
         f'at most {far:.1f}mm from one: shellac fills them')
    n, two, void = corner_scan(parts, info)
    note(not two, 'no two plates in the same place, round every corner',
         f'{n} points within {CORNER_R:g}mm of the {len(verts)} corners, in the walls'
         + (f', {two} in two plates' if two else f', each in one plate or none'
            + (f'; corners nipped {info["nip"]:g}mm' if info['nip'] else '')))
    n, two = mouthpiece_scan(parts)
    note(not two, 'the beak, the bird and the chin layer clear the body',
         f'{n} points through them, none in a second part' if not two
         else f'{two} of {n} points in a second part')
    sheets = pack(parts)
    note(all(s['w'] <= BED[0] and s['h'] <= BED[1] for s in sheets), 'every sheet fits the bed',
         f'{len(sheets)} sheet(s), largest {max(s["w"] for s in sheets):.0f} x '
         f'{max(s["h"] for s in sheets):.0f}mm')
    placed = sum(len(s['parts']) for s in sheets)
    note(placed == len(parts), 'every part is on a sheet', f'{placed} of {len(parts)}')
    return out, sheets, f0


CORNER_R, CORNER_STEP = 5.0, 0.4


def corner_scan(parts, info):
    """Every point of a grid round each corner that lies in the walls --
    inside the solid, within T of its outside -- and how many of them lie in
    two plates or in none. The joint check samples along the edges; this one
    finds plates that share only a corner, which meet nowhere else. The grid
    is set off a little so no point lands on a cell's boundary."""
    faces = info['faces']
    verts = {tuple(round(x, 6) for x in v) for f in faces for v in f.verts}
    k = int(CORNER_R / CORNER_STEP)
    n = two = void = 0
    for v in sorted(verts):
        for i in range(-k, k + 1):
            for j in range(-k, k + 1):
                for m in range(-k, k + 1):
                    q = (v[0] + i * CORNER_STEP + 0.0137, v[1] + j * CORNER_STEP + 0.0291,
                         v[2] + m * CORNER_STEP + 0.0173)
                    depth = [f.off - P.dot(f.n, q) for f in faces]
                    if min(depth) < 0 or min(depth) > T:
                        continue
                    n += 1
                    o = len(owners(parts, q))
                    two += o > 1
                    void += o == 0
    return n, two, void


def mouthpiece_scan(parts, step=0.5):
    """A grid through the mouthpiece -- the bird, the chin layer and the top
    plate's beak, out past the body -- at the middle of their thickness and a
    little inside each face: how many points are also in another part."""
    n = two = 0
    for p in parts:
        if p.name not in ('bird', 'chin layer', 'top plate'):
            continue
        xs = [x for x, _ in p.outline]
        ys = [y for _, y in p.outline]
        m = p.m
        for w in (0.1, T / 2, T - 0.1):
            u = min(xs) + 0.013
            while u < max(xs):
                v = min(ys) + 0.029
                while v < max(ys):
                    if inside((u, v), p.outline) and (u < 0 or p.name != 'top plate'):
                        q = [m[i][0] * u + m[i][1] * v + m[i][2] * w + m[i][3] for i in range(3)]
                        n += 1
                        two += len(owners(parts, q)) > 1
                    v += step
                u += step
    return n, two



# --- sheets ------------------------------------------------------------------

def svg_sheet(name, k, n, s, what):
    orange, black, fills = [], [], []
    for p, sx, sy in s['parts']:
        dx, dy = sx - p.x0 + KERF / 2, sy - p.y0 + KERF / 2
        mv = lambda pts: [(x + dx, y + dy) for x, y in pts]
        black.append(path(mv(offset(p.outline, KERF / 2))))
        for c in p.cuts:
            if c[0] == 'rect':
                orange.append(path(mv(offset(c[1], -KERF / 2))))
            else:
                _, cx, cy, r = c
                orange.append(f'<circle cx="{cx + dx:.3f}" cy="{cy + dy:.3f}" '
                              f'r="{r - KERF / 2:.3f}"/>')
        for f in p.fills:
            fills.append(path(mv(f)))
    desc = (f'1 user unit = 1mm. Four-hole ocarina in G4, English fingering, {name}, '
            f'sheet {k} of {n}: {what}. 3mm ply, drawn for a {KERF:g}mm kerf. '
            f'Blue {BLUE} engraves the duct, a fill engrave about 1mm deep -- set the '
            f'depth on scrap first; orange {ORANGE} cuts the window, the holes'
            + (' and the cavity' if name == 'stack' else '')
            + f'; black {BLACK} frees the parts.')
    g = lambda col, items: (
        f'  <g fill="none" stroke="{col}" stroke-width="0.2">\n'
        + ''.join(f'    {i}\n' if i.startswith('<') else f'    <path d="{i}"/>\n'
                  for i in items) + '  </g>\n')
    return (f'<?xml version="1.0" encoding="utf-8"?>\n'
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{s["w"]:.2f}mm" '
            f'height="{s["h"]:.2f}mm" viewBox="0 0 {s["w"]:.2f} {s["h"]:.2f}">\n'
            f'<title>Four-hole ocarina in G4, {name}, sheet {k} of {n}</title>\n'
            f'<desc>{desc}</desc>\n'
            f'  <g fill="{BLUE}" stroke="none">\n'
            + ''.join(f'    <path d="{d}"/>\n' for d in fills) + '  </g>\n'
            + g(ORANGE, orange) + g(BLACK, black) + '</svg>\n')


def main():
    out, only, write, flat = None, None, True, False
    for x in sys.argv[1:]:
        if x == '--no-write':
            write = False
        elif x == '--flat':
            flat = True
        elif x.startswith('--out='):
            out = x.split('=', 1)[1]
        elif x.startswith('--mouth='):
            M.MOUTH = float(x.split('=', 1)[1])
        elif x.startswith('--only='):
            only = x.split('=', 1)[1]
            if only not in BUILDS:
                raise SystemExit(f'error: {only} is not a build. They are '
                                 + ', '.join(BUILDS) + '.')
        else:
            raise SystemExit(f'error: {x} is not a flag this generator reads. '
                             f'It takes --out=, --only=, --flat, --mouth= and --no-write.')
    if write and not out:
        raise SystemExit('error: --out=DIR, or --no-write to run the checks only.')
    V, kw, ratios, diams = M.design()
    rows = M.fingering_table(V, kw, diams)
    worst = max(abs(c) for f, p, nm, c, used in rows if used)
    bad = 0
    for name, how in BUILDS.items():
        if only and name != only:
            continue
        if name in SOLIDS:
            parts, info = build_solid(name, diams)
            res, sheets, f0 = solid_checks(name, parts, info, V, kw, diams, worst)
            print(f'\n{name}: {how}, cavity {info["V"] / 1000:.1f}cm^3, '
                  f'G4\'s fingering a {1200 * math.log2(f0 / M.F0):+.0f} cents from G4: '
                  f'{f0:.1f}Hz with every hole closed')
            print('  holes: ' + ', '.join(f'{h} ø{d:.2f}' for h, d in zip(M.HOLES, diams)))
            for ok, what, detail in res:
                print(f'  {"pass" if ok else "FAIL"}  {what:52} {detail}')
                bad += not ok
            if write and not bad:
                d = out if flat else os.path.join(out, name)
                os.makedirs(d, exist_ok=True)
                for k, s in enumerate(sheets, 1):
                    what = ', '.join(sorted({p.name for p, _, _ in s['parts']}))
                    f = os.path.join(d, f'four-hole-ocarina-{name}-sheet{k}-cut-files.svg')
                    open(f, 'w').write(svg_sheet(name, k, len(sheets), s, what))
                    print(f'  wrote {os.path.basename(f)}  {s["w"]:.0f} x {s["h"]:.0f}mm, '
                          f'{len(s["parts"])} parts')
            continue
        parts, info = build(name, V, diams)
        print(f'\n{name}: {how}, cavity {W_IN:g} x {D_IN:g} x {info["Lc"]:.1f}mm, '
              f'{V / 1000:.2f}cm^3; body {info["total"]:.1f} x {info["PW"]:.1f}mm, '
              f'beak {BEAK:g}mm more')
        print('  holes: ' + ', '.join(f'{h} ø{d:.2f}' for h, d in zip(M.HOLES, diams)))
        res, sheets = checks(name, parts, info, V, diams, worst)
        for ok, what, detail in res:
            print(f'  {"pass" if ok else "FAIL"}  {what:52} {detail}')
            bad += not ok
        if write and not bad:
            d = out if flat else os.path.join(out, name)
            os.makedirs(d, exist_ok=True)
            for k, s in enumerate(sheets, 1):
                what = ', '.join(sorted({p.name for p, _, _ in s['parts']}))
                f = os.path.join(d, f'four-hole-ocarina-G4-{name}-sheet{k}-cut-files.svg')
                open(f, 'w').write(svg_sheet(name, k, len(sheets), s, what))
                print(f'  wrote {os.path.basename(f)}  {s["w"]:.0f} x {s["h"]:.0f}mm, '
                      f'{len(s["parts"])} parts')
    if bad:
        print(f'\n  {bad} check(s) failed. Nothing written.')
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())

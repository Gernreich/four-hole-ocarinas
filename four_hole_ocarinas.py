#!/usr/bin/env python3
"""Cut files for four-hole ocarinas, English fingering, in 3mm ply.

    python3 four_hole_ocarinas.py --no-write          # the checks, write nothing
    python3 four_hole_ocarinas.py --out=DIR           # every build into DIR/<build>/
    python3 four_hole_ocarinas.py --only=cube --out=DIR
    python3 four_hole_ocarinas.py --only=cube --flat --out=DIR   # no subfolder
    python3 four_hole_ocarinas.py --mouth=1.2 ...     # the window correction, as the model
    python3 four_hole_ocarinas.py --kerf=0.18 ...     # for a laser with that kerf

The acoustics are ocarina_model.py's: the cavity's volume and the four
holes' sizes. This file only builds what that asks for. An ocarina's holes
go wherever the fingers fall, since a Helmholtz resonator hears their size
and not their place, so the layout here is the hands', not the model's.

EVERY BUILD IS A SOLID, built by polyhedron.py and finger-jointed on every
edge at whatever angle its faces meet (see BUILDS). Each holds what its
shape holds and plays in the key that gives; the model's G4 is only the
frame the scale is worked out in. Until 2026-10-18 there was a G4 build, a
box 60 x 18mm inside and as long as the volume needed, finger-jointed or
laminated as a stack; at the 14 x 7mm window it ran 144mm, near enough a
quarter wave to act partly as a tube, and the author made it a cube sized
to G4, then the same day a cube of about 280cm^3, as the octahedron.

All are tuned at the window and the holes.

FROM THE MOUTH END, the same in every build:

  beak     the top plate run on past the end, BEAK long and BEAK_W wide:
           what the lips close on
  bird     one layer glued over the beak and on to the window's near
           edge. The duct, the window's width, runs from the beak's tip to
           the window as a fill engrave in two inks: BIRD_INK in the bird's
           underside, NEST_INK in the top plate's outside face under it, so
           either or both can be run (see BIRD_INK)
  chin     one plain layer glued under the beak, so the mouthpiece is
           three plies thick, 9mm, with the duct near its middle. Until
           2026-10-17 the bird was three layers stacked on top, copied from
           the square flute, and the beak had nothing under it
  window   the true sound hole, WINDOW_W x WINDOW_L, against the cavity's
           mouth-end wall, so the jet crosses it over the cavity
  holes    on the cube, the two prisms, the antiprism and the
           cuboctahedron, two rows across the top: the index fingers' row
           ROW_1 past the window, the middle fingers' ROW_2 past that, each
           hand's pair in a column COLUMNS apart; the solids in FACE_HOLES
           give them faces of their own (see build_solid)

THE SPLITTING EDGE is the window's far edge, and wants the bevel a laser
cannot cut, as on the square flute: file it, about 30 degrees, on the face
the bird sits on, before the bird goes on. That face is the outside of the
top, and fixes which hand is which: looking at it with the beak up, the
right hand's holes are on the LEFT, because the player sees the other face.

The finger's size, the kerf and the sheet packing are the square flute's,
imported from square_flute.py beside this repository, which imports the path
writer from swept-curve's ribbon_bore.py in turn. The joint is
polyhedron.py's, cut for each edge's own angle.
"""
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, '..', 'square-flute'))
sys.path.insert(0, os.path.join(HERE, '..', 'swept-curve'))
import ocarina_model as M                                      # noqa: E402
# the square flute's plates, finger size, kerf and packer, imported so the
# two instruments are cut alike
import square_flute as SF                                      # noqa: E402
import polyhedron as P                                         # noqa: E402
from square_flute import Part, rect, pack                     # noqa: E402
from ribbon_bore import path                                   # noqa: E402

T, KERF, FINGER = SF.T, SF.KERF, SF.FINGER
MIN_WEB = SF.MIN_WEB
BED, MARGIN = SF.BED, SF.MARGIN
BLUE, ORANGE, BLACK = SF.BLUE, SF.ORANGE, SF.BLACK
# The duct is a savanna flue, cut both ways at once: the bird's half in its
# underside and the nest's half in the top plate's outside face, over the
# beak, each in an ink of its own. Run one for a woodland (bird) flue or a
# plains (nest) one, about 1mm deep, or both at about 0.5mm each for a
# savanna flue. The house order has one engrave ink, blue, which the corner
# numbers keep, and green and cyan are cut stages, so these two are new.
BIRD_INK, NEST_INK = '#ff0000', '#ff00ff'

BEAK, BEAK_W = 18.0, 24.0
# the beak's front, where the lips go: a half-ellipse across its whole width
# and TIP_ROUND deep, on the top plate, the bird and the chin layer alike,
# so the three plies round off as one (the author, 2026-10-17)
TIP_ROUND, TIP_STEPS = 8.0, 32
# the splitting edge's bevel, filed by hand on the top's outside face; a
# blue line there marks where the bevel meets the face (the author,
# 2026-10-18)
BEVEL = 30.0
# the hole rows, from the cavity's mouth-end wall, and the two hands'
# columns, centre to centre: set to fit the 54mm inside of the G4 cube that
# was a build for a day. The box's were 30, 52 and 36mm
ROW_1, ROW_2 = 20.0, 42.0
COLUMNS = 34.0
D_MIN, D_MAX = 2.5, 13.0     # a hole that still speaks; one a finger covers
HOLE_GAP = 3.0               # ply between neighbouring holes

BUILDS = {
    'cube': 'cube, finger-jointed',
    'octagon': 'uniform octagonal prism, finger-jointed',
    'square-antiprism': 'square antiprism, finger-jointed',
    'octahedron': 'regular octahedron, finger-jointed',
    'dodecahedron': 'regular dodecahedron, finger-jointed',
    'truncated-octahedron': 'truncated octahedron, finger-jointed',
    'hexagonal-prism': 'uniform hexagonal prism, finger-jointed',
    'icosahedron': 'regular icosahedron, finger-jointed',
    'cuboctahedron': 'cuboctahedron, finger-jointed',
}
# Solids, built by polyhedron.py, since 2026-10-17. Each cavity is what its
# shape holds, and its key follows from it. The holes are sized against the
# window, not the cavity, so every fingering and every interval is the same
# in all of them; only the key moves.
#   cube            its edge whatever holds CUBE_VOLUME (cube_edge), about
#                   the octahedron's, at the author's asking (2026-10-18)
CUBE_VOLUME = 280e3
#   octagon         OCT_FLATS across the flats, outside, and as tall as a
#                   wall is wide, so every wall is square and the prism is
#                   uniform, as the cube is; it was 36mm tall until
#                   2026-10-18
OCT_FLATS = 110.0
OCT_HEIGHT = OCT_FLATS * math.tan(math.pi / 8)
# The other solids, the antiprism's and the dodecahedron's edges a whole
# number of 6mm fingers:
#   square-antiprism  72mm, so the top square takes the window and the four
#                   holes on the cube's grid, ROW_1 and the rest; its square is a quarter turn
#                   from the one below it, with eight triangles between
#   octahedron      standing on a triangle, every edge OCTA_EDGE, sized for
#                   a cavity of about 280cm^3 at the author's asking
#                   (2026-10-18; 72mm and 127cm^3 until then); the top is
#                   too small for four holes, so each hand has a triangle of
#                   its own, the index finger's hole nearer the beak
#   dodecahedron    standing on a pentagon, every edge DODECA_EDGE; each hole
#                   has a pentagon of its own, the index fingers' the two
#                   either side of the mouth, the middle fingers' the next
# The four the author asked for next (2026-10-18), each at about 280cm^3
# or as much more as its holes need:
#   truncated-octahedron  standing on a square, which takes only the
#                   window; each hand's two holes share the hexagon on its side
#   hexagonal-prism uniform, as the octagon is: square walls, the holes on
#                   the top's grid
#   icosahedron     standing on a triangle, each hole on a triangle of its own
#   cuboctahedron   standing on a square, the holes on its grid; 54.5mm, not
#                   the 53.2mm of 280cm^3, so the biggest hole keeps 2mm of ply
#                   to a wall after the millimetre it may be filed up
# Four plates meet at the antiprism's, the octahedron's and the
# cuboctahedron's corners and five at the icosahedron's, and NIP cuts each
# corner back that far, square to its bisector, so that the plates sharing
# only the corner stay apart: half a millimetre over the least that clears.
SAP_EDGE = 72.0
OCTA_EDGE = 91.5
DODECA_EDGE = 36.0
HEX_EDGE = 52.0
ICOSA_EDGE = 54.5
TRUNC_OCTA_EDGE = 31.6
CUBOCTA_EDGE = 54.5
NIP = {'square-antiprism': 4.25, 'octahedron': 4.75, 'icosahedron': 3.5, 'cuboctahedron': 3.5}
FACE_HOLES = ('octahedron', 'dodecahedron', 'icosahedron', 'truncated-octahedron')


def cube_edge(V):
    """The cube's outside edge whose cavity holds V: two plates more than the
    inside's cube root."""
    return V ** (1 / 3) + 2 * T


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


# --- checks -----------------------------------------------------------------

def inside(pt, poly):
    """Even-odd point in polygon."""
    x, y = pt
    c = False
    for (x0, y0), (x1, y1) in zip(poly, poly[1:] + poly[:1]):
        if (y0 > y) != (y1 > y) and x < x0 + (y - y0) * (x1 - x0) / (y1 - y0):
            c = not c
    return c


def owners(parts, q):
    """The plates whose solid holds the world point q: q taken back into
    each plate's own frame through its matrix, whose axes are orthonormal,
    so its transpose undoes it."""
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


# --- the solids ----------------------------------------------------------------

def build_solid(kind, diams):
    """Every part of a solid build. The world is the solid's own: z up, the
    hole face on top, the mouth face looking down -x, so the beak runs out
    along -x from the middle of the top's mouth edge. Each part carries its
    frame as p.m, the matrix from its own (u, v, inward) to the world."""
    faces = {'octagon': lambda: P.octagonal_prism(OCT_FLATS, OCT_HEIGHT),
             'cube': lambda: P.cube(cube_edge(CUBE_VOLUME)),
             'square-antiprism': lambda: P.square_antiprism(SAP_EDGE),
             'octahedron': lambda: P.octahedron(OCTA_EDGE),
             'dodecahedron': lambda: P.dodecahedron(DODECA_EDGE),
             'truncated-octahedron': lambda: P.truncated_octahedron(TRUNC_OCTA_EDGE),
             'hexagonal-prism': lambda: P.hexagonal_prism(HEX_EDGE),
             'icosahedron': lambda: P.icosahedron(ICOSA_EDGE),
             'cuboctahedron': lambda: P.cuboctahedron(CUBOCTA_EDGE)}[kind]()
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
                       'outside face, back to the blue line, before the bird goes on'}.get(
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
    # a BEVEL through the plate, sharp at the inside face, reaches the
    # outside face T / tan BEVEL past the window's far edge: file to there
    gx = T + M.WINDOW_L + T / math.tan(math.radians(BEVEL))
    tp.strokes = [[(gx, -M.WINDOW_W / 2), (gx, M.WINDOW_W / 2)]]
    tp.note += ', the nest\'s half of the duct fill-engraved in its outside face'
    tp.fills.append(duct(T, 0.0))
    tp.ink = NEST_INK
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
                'the bird\'s half of the duct fill-engraved in its underside')
    bird.fills.append(duct(T, 0.0))
    bird.ink = BIRD_INK
    # the bird faces down: its own w = 0, the face the duct is engraved
    # in, lies on the top plate, so it is turned over
    # about U, which its outline and duct, both symmetric across, allow
    bird.m = P.matrix((O, U, P.mul(V, -1), P.mul(W, -1)))
    chin = Part('chin layer', next(num), mouthpiece(-back, 0.0),
                'glued under the beak, its end against the mouth face')
    chin.m = P.matrix((P.sub(O, (0, 0, T)), U, V, W))
    parts += [bird, chin]
    inner2d = [[P.to2d(fr, q) for q in pl] for fr, pl in zip(frames, polys)]
    vnum = corner_numbers(faces)
    # the top carries the nest's duct on its outside face and is drawn
    # outside face up, so it takes no numbers: the beak, the window and the
    # holes place it, and every corner it touches is numbered on the others
    for p in parts[:len(faces)]:
        if p.face == ft:
            continue
        p.numbers = number_corners(faces[p.face], frames[p.face], inner2d[p.face], p.cuts, vnum)
    return parts, dict(kind=kind, faces=faces, edges=es, cellmap=cellmap, ft=ft, fm=fm,
                       V=vol, inner=[[P.to2d(fr, q) for q in pl] for fr, pl in zip(frames, polys)],
                       places=places, fr_top=fr_top, frames=frames, back=back, nip=nip,
                       inner_pts=[q for pl in polys for q in pl])


# --- corner numbers ----------------------------------------------------------
# Every corner of a solid has a number, and every plate engraves it near each
# of its own corners, on its inside face: the plates that meet at a corner
# all carry its number, so the solid goes together by matching them. The
# digits are the trumpet tooling's single-stroke ones (bore_split.py's G),
# x across and y up, in a cell 0.62 of the height wide.
DIGITS = {
    '0': [[(.5, 1), (.85, .8), (.85, .2), (.5, 0), (.15, .2), (.15, .8), (.5, 1)]],
    '1': [[(.30, .78), (.52, 1), (.52, 0)], [(.28, 0), (.78, 0)]],
    '2': [[(.10, .78), (.30, 1), (.70, 1), (.90, .78), (.90, .60), (.10, 0), (.90, 0)]],
    '3': [[(.10, 1), (.90, 1), (.45, .55)],
          [(.45, .55), (.90, .55), (.90, .16), (.72, 0), (.28, 0), (.10, .16)]],
    '4': [[(.70, 0), (.70, 1), (.12, .32), (.92, .32)]],
    '5': [[(.85, 1), (.20, 1), (.15, .55), (.50, .62), (.80, .50), (.88, .28), (.75, .06),
           (.40, 0), (.15, .12)]],
    '6': [[(.82, .92), (.55, 1), (.25, .85), (.15, .45), (.15, .18), (.35, 0), (.62, 0),
           (.85, .18), (.85, .38), (.62, .55), (.30, .55), (.15, .45)]],
    '7': [[(.12, 1), (.90, 1), (.42, 0)]],
    '8': [[(.5, .55), (.22, .68), (.22, .87), (.5, 1), (.78, .87), (.78, .68), (.5, .55),
           (.18, .40), (.18, .14), (.5, 0), (.82, .14), (.82, .40), (.5, .55)]],
    '9': [[(.18, .08), (.45, 0), (.75, .15), (.85, .55), (.85, .82), (.65, 1), (.38, 1),
           (.15, .82), (.15, .62), (.38, .45), (.70, .45), (.85, .55)]],
}
NUM_H = 4.0          # digit height
NUM_CLEAR = 1.0      # from the joints, the holes and the window


def corner_numbers(faces):
    """Every corner of the solid numbered from 1: top down, and round
    each level anticlockwise from -x, the beak's side."""
    vs = {tuple(round(x, 6) for x in v) for f in faces for v in f.verts}
    key = lambda v: (-round(v[2], 3), (math.atan2(v[1], v[0]) - math.pi) % (2 * math.pi))
    return {v: i + 1 for i, v in enumerate(sorted(vs, key=key))}


def number_box(text, c, up):
    """The four corners of a number's box centred on c, its top toward up,
    in the plate's frame as seen from outside."""
    w = len(text) * 0.62 * NUM_H + (len(text) - 1) * 0.2 * NUM_H
    r = (up[1], -up[0])
    return [(c[0] + i * r[0] * w / 2 + j * up[0] * NUM_H / 2,
             c[1] + i * r[1] * w / 2 + j * up[1] * NUM_H / 2)
            for i, j in ((-1, -1), (1, -1), (1, 1), (-1, 1))]


def number_corners(face, fr, inner, cuts, vnum):
    """A plate's corner numbers: (text, centre, up) in its frame as seen
    from outside, each moved in from its corner along the line to the face's
    centre until its box clears the plate's joints -- it lies inside the
    face's inside, the part of it the cavity sees -- and its holes and window
    by NUM_CLEAR. Up points at the corner."""
    g = P.to2d(fr, face.centre)
    keep = []
    for c in cuts:
        if c[0] == 'circle':
            keep.append(((c[1], c[2]), c[3]))
        else:
            xs, ys = [q[0] for q in c[1]], [q[1] for q in c[1]]
            keep.append((((min(xs) + max(xs)) / 2, (min(ys) + max(ys)) / 2),
                         math.hypot(max(xs) - min(xs), max(ys) - min(ys)) / 2))
    out = []
    for v in face.verts:
        k = vnum[tuple(round(x, 6) for x in v)]
        a = P.to2d(fr, v)
        L = math.hypot(g[0] - a[0], g[1] - a[1])
        d = ((g[0] - a[0]) / L, (g[1] - a[1]) / L)
        up = (-d[0], -d[1])
        s = 0.0
        while s < L:
            c = (a[0] + s * d[0], a[1] + s * d[1])
            box = number_box(str(k), c, up)
            half = math.hypot(*(box[0][i] - c[i] for i in range(2)))
            if (all(in_poly_margin(q, NUM_CLEAR, inner) >= 0 for q in box)
                    and all(math.hypot(c[0] - h[0], c[1] - h[1]) >= r + NUM_CLEAR + half
                            for h, r in keep)):
                break
            s += 0.25
        else:
            raise ValueError(f'no room for corner {k} on a {face.name}')
        out.append((str(k), (round(c[0], 3), round(c[1], 3)), up))
    return out


def inside_up(p):
    """A solid's plate as the laser sees it, inside face up, so the corner
    numbers engrave on the inside: mirrored across u, the numbers drawn as
    strokes reading the right way round."""
    q = Part(p.name, p.num, [(x, -y) for x, y in p.outline], p.note)
    q.cuts = [('rect', [(x, -y) for x, y in c[1]]) if c[0] == 'rect'
              else ('circle', c[1], -c[2], c[3]) for c in p.cuts]
    q.fills = [[(x, -y) for x, y in f] for f in p.fills]
    q.ink = getattr(p, 'ink', BLUE)
    q.strokes = []
    for text, (cx, cy), (ux, uy) in p.numbers:
        c, up = (cx, -cy), (ux, -uy)
        r = (-up[1], up[0])                       # reading direction, y down
        w = len(text) * 0.62 * NUM_H + (len(text) - 1) * 0.2 * NUM_H
        o = (c[0] - r[0] * w / 2 - up[0] * NUM_H / 2, c[1] - r[1] * w / 2 - up[1] * NUM_H / 2)
        x0 = 0.0
        for ch in text:
            for st in DIGITS[ch]:
                q.strokes.append([(o[0] + r[0] * (x0 + px * 0.62 * NUM_H) + up[0] * py * NUM_H,
                                   o[1] + r[1] * (x0 + px * 0.62 * NUM_H) + up[1] * py * NUM_H)
                                  for px, py in st])
            x0 += 0.82 * NUM_H
    return q


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
    note(worst < 20.0, 'the model holds the scale', f'worst {worst:.1f} cents of eight notes')
    V = info['V']
    if kind == 'cube':
        e = cube_edge(CUBE_VOLUME)
        note(abs(V / CUBE_VOLUME - 1) < 1e-6, 'the cavity is the volume asked for',
             f'{e:.2f}mm edge, {e - 2 * T:.2f}mm inside, {V / 1000:.2f}cm^3')
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
    # and between the window and any hole on the top with it
    wgap = min((math.hypot(max(T - x, 0, x - T - M.WINDOW_L),
                           max(abs(y) - M.WINDOW_W / 2, 0)) - d / 2
                for (i, (x, y)), d in zip(info['places'].values(), diams)
                if i == info['ft']), default=math.inf)
    note(min(gap, wgap) >= HOLE_GAP, 'ply left between neighbouring holes and the window',
         f'narrowest {gap:.1f}mm'
         + (f', {wgap:.1f}mm from the window' if wgap < math.inf else ''))
    # the hands, from the geometry rather than the names: right = forward x up
    fwd = info['faces'][info['ft']].n
    up = (-1.0, 0.0, 0.0)
    right = P.cross(fwd, up)
    ok = all((P.dot(world(*ixy), right) > 0) == h.startswith('R')
             for h, ixy in info['places'].items())
    note(ok, 'each hand\'s holes lie under that hand', 'held holes forward, beak up')
    bird = next(p for p in parts if p.name == 'bird')
    tp = parts[info['ft']]
    fx = [q[0] for q in bird.fills[0]]
    note(abs(min(fx) + BEAK) < 1e-9 and abs(max(fx) - T) < 1e-9 and tp.fills == bird.fills
         and not hasattr(tp, 'numbers'),
         'the duct runs from the beak\'s tip to the window, in the bird and the top alike',
         f'{T + BEAK:g}mm long, {M.WINDOW_W:g}mm wide')
    (g0, g1), = tp.strokes
    gx = g0[0]
    near = min((math.hypot(max(abs(x - gx), 0), max(abs(y) - M.WINDOW_W / 2, 0)) - d / 2
                for (i, (x, y)), d in zip(info['places'].values(), diams)
                if i == info['ft']), default=math.inf)
    note(abs(gx - T - M.WINDOW_L - T / math.tan(math.radians(BEVEL))) < 1e-9
         and abs(g1[1] - g0[1] - M.WINDOW_W) < 1e-9 and near >= NUM_CLEAR
         and in_poly_margin((gx, 0.0), M.WINDOW_W / 2, info['inner'][info['ft']]) >= 0,
         'the bevel guide marks where a 30 degree bevel meets the outside',
         f'{gx - T - M.WINDOW_L:.2f}mm past the window, {M.WINDOW_W:g}mm long, over the cavity'
         + (f', {near:.1f}mm from the nearest hole' if near < math.inf else ''))
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
    nums = [p.numbers for p in parts if hasattr(p, 'numbers')]
    each = {}
    for p in parts:
        for text, _, _ in getattr(p, 'numbers', []):
            each[text] = each.get(text, 0) + 1
    tv = len(info['faces'][info['ft']].verts)
    note(len(nums) == len(info['faces']) - 1 and
         sum(map(len, nums)) == sum(len(f.verts) for f in info['faces']) - tv
         and min(each.values()) >= 2,
         'every plate but the top numbers its corners, inside, clear of joints and holes',
         f'{len(each)} corners, {sum(each.values())} numbers, '
         f'{min(each.values())} to {max(each.values())} plates to a corner')
    sheets = pack([inside_up(p) if hasattr(p, 'numbers') else p for p in parts])
    note(all(s['w'] <= BED[0] and s['h'] <= BED[1] for s in sheets), 'every sheet fits the bed',
         f'{len(sheets)} sheet(s), largest {max(s["w"] for s in sheets):.0f} x '
         f'{max(s["h"] for s in sheets):.0f}mm')
    placed = sum(len(s['parts']) for s in sheets)
    note(placed == len(parts), 'every part is on a sheet', f'{placed} of {len(parts)}')
    return out, sheets, f0


CORNER_R, CORNER_STEP = 5.0, 0.4


def note_name(f):
    """The equal-tempered note nearest f, as A4 = 440Hz names it."""
    n = round(12 * math.log2(f / 440) + 69)
    return ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'][n % 12] + str(n // 12 - 1)


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

def svg_sheet(name, k, n, s, what, key):
    orange, black, nums = [], [], []
    fills = {BIRD_INK: [], NEST_INK: []}
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
            fills[p.ink].append(path(mv(f)))
        for st in getattr(p, 'strokes', []):
            nums.append(path(mv(st), close=False))
    desc = (f'1 user unit = 1mm. Four-hole ocarina in {key}, English fingering, {name}, '
            f'sheet {k} of {n}: {what}. 3mm ply, drawn for a {KERF:g}mm kerf. '
            f'The duct is a fill engrave in two inks, run either or both: red {BIRD_INK} '
            f'in the bird\'s underside, magenta {NEST_INK} in the top\'s outside face. One '
            f'alone about 1mm deep, both about 0.5mm each -- set the depth on scrap first. '
            + f'Blue {BLUE} engraves single lines: on the top, which lies outside face up, '
            f'a line across past the window where the {BEVEL:g} degree bevel filed on its '
            f'far edge should reach; on every other plate, inside face up, the corner '
            f'numbers, each carried by the plates that meet at that corner. '
            + f'Then the cuts: orange {ORANGE} cuts the window and the holes, '
            f'and black {BLACK} frees the parts.')
    g = lambda col, items: (
        f'  <g fill="none" stroke="{col}" stroke-width="0.2">\n'
        + ''.join(f'    {i}\n' if i.startswith('<') else f'    <path d="{i}"/>\n'
                  for i in items) + '  </g>\n')
    return (f'<?xml version="1.0" encoding="utf-8"?>\n'
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{s["w"]:.2f}mm" '
            f'height="{s["h"]:.2f}mm" viewBox="0 0 {s["w"]:.2f} {s["h"]:.2f}">\n'
            f'<title>Four-hole ocarina in {key}, {name}, sheet {k} of {n}</title>\n'
            f'<desc>{desc}</desc>\n'
            + ''.join(f'  <g fill="{ink}" stroke="none">\n'
                      + ''.join(f'    <path d="{d}"/>\n' for d in ds) + '  </g>\n'
                      for ink, ds in fills.items())
            + (g(BLUE, nums) if nums else '') + g(ORANGE, orange) + g(BLACK, black) + '</svg>\n')


def main():
    global KERF
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
        elif x.startswith('--kerf='):
            # the square flute's packer reads its own KERF, so both are set
            KERF = SF.set_kerf(x)
        elif x.startswith('--only='):
            only = x.split('=', 1)[1]
            if only not in BUILDS:
                raise SystemExit(f'error: {only} is not a build. They are '
                                 + ', '.join(BUILDS) + '.')
        else:
            raise SystemExit(f'error: {x} is not a flag this generator reads. '
                             f'It takes --out=, --only=, --flat, --mouth=, --kerf= and --no-write.')
    if write and not out:
        raise SystemExit('error: --out=DIR, or --no-write to run the checks only.')
    V, kw, ratios, diams = M.design()
    rows = M.fingering_table(V, kw, diams)
    worst = max(abs(c) for f, p, nm, c, used in rows if used)
    bad = 0
    for name, how in BUILDS.items():
        if only and name != only:
            continue
        parts, info = build_solid(name, diams)
        res, sheets, f0 = solid_checks(name, parts, info, V, kw, diams, worst)
        key = note_name(f0)
        print(f'\n{name}: {how}, cavity {info["V"] / 1000:.1f}cm^3, '
              f'G4\'s fingering a {round(1200 * math.log2(f0 / M.F0)) + 0:+d} cents from G4: '
              f'{f0:.1f}Hz with every hole closed, {key}')
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
                open(f, 'w').write(svg_sheet(name, k, len(sheets), s, what, key))
                print(f'  wrote {os.path.basename(f)}  {s["w"]:.0f} x {s["h"]:.0f}mm, '
                      f'{len(s["parts"])} parts')
    if bad:
        print(f'\n  {bad} check(s) failed. Nothing written.')
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())

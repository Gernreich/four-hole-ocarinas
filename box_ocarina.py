#!/usr/bin/env python3
"""Cut files for a four-hole ocarina in G4, English fingering, in 3mm ply.

    python3 box_ocarina.py --no-write          # the checks, write nothing
    python3 box_ocarina.py --out=DIR           # every build into DIR/<build>/
    python3 box_ocarina.py --only=box --out=DIR
    python3 box_ocarina.py --only=box --flat --out=DIR   # no subfolder
    python3 box_ocarina.py --mouth=1.2 ...     # the window correction, as the model

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
from square_flute import Part, rect, offset, pack             # noqa: E402
from ribbon_bore import path                                   # noqa: E402

T, KERF, FINGER = SF.T, SF.KERF, SF.FINGER
MIN_WEB = SF.MIN_WEB
BED, MARGIN = SF.BED, SF.MARGIN
BLUE, ORANGE, BLACK = SF.BLUE, SF.ORANGE, SF.BLACK

W_IN = 60.0          # the cavity across: room for two columns of holes
D_IN = 18.0          # and deep: six 3mm frame layers in the stack
SIDE = 6.0           # the stack's frame, all round
BEAK, BEAK_W = 18.0, 24.0
ROW_1, ROW_2 = 30.0, 52.0    # hole rows, from the cavity's mouth-end wall
COLUMNS = 36.0               # the two hands' columns, centre to centre
D_MIN, D_MAX = 2.5, 13.0     # a hole that still speaks; one a finger covers
HOLE_GAP = 3.0               # ply between neighbouring holes

BUILDS = {
    'box': 'finger-jointed box',
    'stack': 'laminated stack',
}


def cavity_length(V):
    return V / (W_IN * D_IN)


def hole_places(PW):
    """(x, y) of each hole in HOLES order, x from the cavity's mouth-end
    wall. The right hand at y > PW/2: on the sheet, and on the outside face
    with the beak up, that is the viewer's left and the player's right."""
    yc = PW / 2
    return {'R2': (ROW_2, yc + COLUMNS / 2), 'R1': (ROW_1, yc + COLUMNS / 2),
            'L1': (ROW_1, yc - COLUMNS / 2), 'L2': (ROW_2, yc - COLUMNS / 2)}


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
        p = Part('top plate', next(num), outline,
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
    p = Part('bird', next(num), rect(-BEAK, yc - BEAK_W / 2, c0, yc + BEAK_W / 2),
             'the duct fill-engraved in its underside, about 1mm deep')
    p.fills.append(rect(-BEAK, yc - M.WINDOW_W / 2, c0, yc + M.WINDOW_W / 2))
    parts.append(p)
    # under the beak only: the body starts at x = 0
    parts.append(Part('chin layer', next(num), rect(-BEAK, yc - BEAK_W / 2, 0, yc + BEAK_W / 2),
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
                f = os.path.join(d, f'box-ocarina-G4-{name}-sheet{k}-cut-files.svg')
                open(f, 'w').write(svg_sheet(name, k, len(sheets), s, what))
                print(f'  wrote {os.path.basename(f)}  {s["w"]:.0f} x {s["h"]:.0f}mm, '
                      f'{len(s["parts"])} parts')
    if bad:
        print(f'\n  {bad} check(s) failed. Nothing written.')
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())

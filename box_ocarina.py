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

  box     a finger-jointed box: top, bottom and two side walls with the
          square flute's joint, and each end closed by a plug of two discs
          pressed into the tube. The far plug sits TUNE in from the end, so
          it can be pushed in to sharpen the ocarina or drawn out to flatten
          it, before the shellac fixes it
  stack   a laminated stack: a bottom layer, six frame layers with the
          cavity cut out of them, and the top. Glued, so it is tuned by the
          window alone

FROM THE MOUTH END, the same in both:

  beak     the top plate run on past the end, BEAK long and BEAK_W wide:
           what the lips close on
  bird     three layers glued over the beak and on to the window's near
           edge; the bottom one carries the duct as a fill engrave, the
           window's width and about 1mm deep, from the beak's tip to the
           window
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

The finger joint, the plug discs' press-fit and the sheet packing are the
square flute's, imported from square_flute.py beside this repository, which
imports the path writer from swept-curve's ribbon_bore.py in turn.
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
from square_flute import (Part, rect, offset, fingers_x,       # noqa: E402
                          finger_side, notched, pack)
from ribbon_bore import path                                   # noqa: E402

T, KERF = SF.T, SF.KERF
INTERF_ACROSS, MIN_WEB = SF.INTERF_ACROSS, SF.MIN_WEB
BED, MARGIN = SF.BED, SF.MARGIN
BLUE, ORANGE, BLACK = SF.BLUE, SF.ORANGE, SF.BLACK

W_IN = 60.0          # the cavity across: room for two columns of holes
D_IN = 18.0          # and deep: six 3mm frame layers in the stack
SIDE = 6.0           # the stack's frame, all round
PLUG_DISCS = 2       # each end plug in the box, 6mm of ply
TUNE = 3.0           # the box's far plug sits this far in from the tube's end
BEAK, BEAK_W = 18.0, 24.0
BIRD_LAYERS = 3
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


def build(kind, V, diams):
    """Every part of one build, in its own flat coordinates: x along the
    ocarina from the mouth end of the body (the beak runs to -BEAK), y
    across it."""
    Lc = cavity_length(V)
    if kind == 'box':
        PW = W_IN + 2 * T
        c0 = PLUG_DISCS * T                     # the cavity's mouth-end wall
        total = c0 + Lc + PLUG_DISCS * T + TUNE
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
        xs = fingers_x(total)
        parts.append(top_plate(beaked(notched(0, total, PW, xs), 0, PW)))
        parts.append(Part('bottom plate', next(num), notched(0, total, PW, xs),
                          'plain'))
        for k in range(2):
            parts.append(finger_side(total, D_IN, xs, next(num)))
        for k in range(2 * PLUG_DISCS):
            parts.append(Part('plug disc', next(num),
                              rect(0, 0, W_IN + INTERF_ACROSS, D_IN + INTERF_ACROSS),
                              f'{PLUG_DISCS} press-fit into each end; the far pair '
                              f'{TUNE:g}mm in, to tune'))
    else:
        parts.append(top_plate(beaked(rect(0, 0, total, PW), 0, PW)))
        parts.append(Part('bottom layer', next(num), rect(0, 0, total, PW), 'plain'))
        for k in range(int(round(D_IN / T))):
            p = Part('frame layer', next(num), rect(0, 0, total, PW),
                     'the cavity cut out')
            p.slot(c0, SIDE, c0 + Lc, SIDE + W_IN)
            parts.append(p)
    for k in range(BIRD_LAYERS):
        p = Part('bird layer', next(num),
                 rect(-BEAK, yc - BEAK_W / 2, c0, yc + BEAK_W / 2),
                 'the duct fill-engraved in the bottom one, about 1mm deep'
                 if k == 0 else 'glued on the one below')
        if k == 0:
            p.fills.append(rect(-BEAK, yc - M.WINDOW_W / 2, c0, yc + M.WINDOW_W / 2))
        parts.append(p)
    return parts, dict(total=total, PW=PW, c0=c0, Lc=Lc, places=places,
                       xs=xs if kind == 'box' else None)


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
    if kind == 'box':
        sides = [p for p in parts if p.name == 'side wall']
        note(min(p.y0 for p in sides) == -T and max(p.y0 + p.h for p in sides) == D_IN + T,
             'the fingers run to the outside faces',
             f'{T:g}mm proud each way, nothing outboard of them')
        note(len(info['xs']) >= 2, 'the side walls have fingers',
             f'{len(info["xs"])} each edge')
    sheets = pack(parts)
    note(all(s['w'] <= BED[0] and s['h'] <= BED[1] for s in sheets),
         'every sheet fits the bed',
         f'{len(sheets)} sheet(s), largest '
         f'{max(s["w"] for s in sheets):.0f} x {max(s["h"] for s in sheets):.0f}mm')
    placed = sum(len(s['parts']) for s in sheets)
    note(placed == len(parts), 'every part is on a sheet', f'{placed} of {len(parts)}')
    return out, sheets


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
        if name == 'box':
            per_mm = 1200 * math.log2(math.sqrt(info['Lc'] / (info['Lc'] - 1)))
            print(f'  the far plug: {per_mm:.1f} cents sharper for each mm it goes in')
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

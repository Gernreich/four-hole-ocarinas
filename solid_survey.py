#!/usr/bin/env python3
"""Which other solids would make a four-hole ocarina, and would any reach
the hands better? Zonohedra and Johnson solids, against the builds.

    python3 solid_survey.py                  # every build and every candidate
    python3 solid_survey.py rhombic-dodecahedron cube   # just these

Every candidate has equal edges and is sized to the cube's 280cm^3 inside,
then run through four_hole_ocarinas.py's own build_solid() and
solid_checks(), twice: with the holes on the top's grid, and with each
hand's holes on the faces at its side. A candidate with a joint under 90
degrees is skipped, since polyhedron.py's fingers would poke out of it.
Corner nips are tried from none to 5.5mm and the first that passes is shown.

For each that passes, and for each build, it prints the hands' distances,
hole centre to hole centre: index to middle finger (the larger of the two
hands), left index to right index, the most any hole's face tilts from the
top, and the window's centre to the furthest hole. The grid's are 22, 34,
0 and 42.1mm.

It poses each candidate as the cube by swapping polyhedron.cube for the
length of one build, so the generator's checks run unchanged.
"""
import itertools
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import four_hole_ocarinas as G   # noqa: E402
P, M = G.P, G.M

phi = (1 + 5 ** 0.5) / 2


def ring(n, r, z, a0=0.0):
    return [(r * math.cos(a0 + 2 * math.pi * k / n), r * math.sin(a0 + 2 * math.pi * k / n), z)
            for k in range(n)]


def R(n):            # circumradius of a regular n-gon of unit edge
    return 1 / (2 * math.sin(math.pi / n))


def pyramid_h(n):    # apex height over a regular n-gon, unit edges
    return math.sqrt(1 - R(n) ** 2)


def cupola(n, z0, up=1):
    """A unit-edge n-cupola on a 2n-gon at z0: (bottom ring, top ring)."""
    r2, r1 = R(2 * n), R(n)
    bot = ring(2 * n, r2, z0, math.pi / (2 * n))
    # top edge k parallel to bottom edge 2k: its vertices straddle that
    # edge's mid-direction by pi/n
    top = ring(n, r1, 0, 0.0)
    # height from one top vertex to its nearest bottom vertex being 1
    t = top[0]
    d = min(math.hypot(t[0] - b[0], t[1] - b[1]) for b in bot)
    h = math.sqrt(1 - d * d)
    top = [(x, y, z0 + up * h) for x, y, _ in top]
    return bot, top


def zonohedron(gens):
    pts = {(0.0, 0.0, 0.0)}
    for g in gens:
        pts |= {tuple(round(p[i] + g[i], 9) for i in range(3)) for p in pts}
    # the hull's corners only
    pts = list(pts)
    planes = P.hull(pts)
    corners = [p for p in pts if sum(abs(P.dot(n, p) - o) < 1e-6 for n, o in planes) >= 3]
    c = [sum(x) / len(corners) for x in zip(*corners)]
    return [tuple(p[i] - c[i] for i in range(3)) for p in corners]


def flip(v):
    return [(x, -y, -z) for x, y, z in v]


C = {}
# --- zonohedra -------------------------------------------------------------
C['rhombic-dodecahedron'] = list(itertools.product((1, -1), repeat=3)) + \
    [tuple(2 * s if i == a else 0 for i in range(3)) for a in range(3) for s in (1, -1)]
C['rhombic-triacontahedron'] = zonohedron([(0, 1, phi), (0, -1, phi), (1, phi, 0), (-1, phi, 0),
                                           (phi, 0, 1), (phi, 0, -1)])
C['rhombic-icosahedron'] = zonohedron([(0, 1, phi), (0, -1, phi), (1, phi, 0), (-1, phi, 0),
                                       (phi, 0, 1)])
C['truncated-cuboctahedron'] = list({tuple(s * v for s, v in zip(sg, pm))
                                     for pm in itertools.permutations((1, 1 + 2 ** .5, 1 + 2 * 2 ** .5))
                                     for sg in itertools.product((1, -1), repeat=3)})
# --- Johnson solids, unit edges, built so a big face is on top ---------------
for n, nm in ((4, 'square'), (5, 'pentagonal'), (3, 'triangular')):
    # elongated pyramid, point down: prism z 0..1, apex below
    v = ring(n, R(n), 0) + ring(n, R(n), 1)
    if n in (3, 4, 5):
        C[f'elongated-{nm}-pyramid'] = v + [(0, 0, -pyramid_h(n))]
        C[f'elongated-{nm}-bipyramid'] = v + [(0, 0, -pyramid_h(n)), (0, 0, 1 + pyramid_h(n))]
for n, nm in ((4, 'square'), (5, 'pentagonal')):
    # gyroelongated pyramid, point down: antiprism, apex below
    h = math.sqrt(1 - (2 * R(n) * math.sin(math.pi / (2 * n))) ** 2)
    v = ring(n, R(n), 0) + ring(n, R(n), h, math.pi / n)
    C[f'gyroelongated-{nm}-pyramid'] = v + [(0, 0, -pyramid_h(n))]
for n, nm in ((3, 'triangular'), (4, 'square'), (5, 'pentagonal')):
    bot, top = cupola(n, 1.0)
    prism = [(x, y, 0.0) for x, y, _ in bot]
    C[f'elongated-{nm}-cupola'] = prism + bot + top          # small n-gon on top
    C[f'{nm}-cupola-upside-down'] = flip(bot + top)           # 2n-gon on top
    b2, t2 = cupola(n, 0.0, up=-1)
    C[f'elongated-{nm}-orthobicupola'] = bot + top + [(x, y, -h) for x, y, h in
                                                      [(p[0], p[1], -(p[2] - 0)) for p in t2]] + prism
for n, nm in ((3, 'triangular'), (4, 'square')):
    bot, top = cupola(n, 0.0)
    b2, t2 = cupola(n, 0.0, up=-1)
    C[f'{nm}-orthobicupola'] = bot + top + t2


def dihedrals(verts):
    planes = P.hull(verts)
    faces = P.solid([('f', n, o) for n, o in planes], verts)
    es = P.edges(faces)
    return min(E['theta'] for E in es), faces, es


def degree(faces):
    cnt = {}
    for f in faces:
        for v in f.verts:
            k = tuple(round(x, 6) for x in v)
            cnt[k] = cnt.get(k, 0) + 1
    return max(cnt.values())


def faces_of(verts, edge, rank=0):
    return P.stand(verts, edge, 'face', rank)


def regular_top(verts):
    """Does every top edge's normal point at its midpoint, as on a regular
    face? If not (a rhombus), the beak can take either pair of edges."""
    top = next(f for f in faces_of(verts, 1.0) if f.name == 'top')
    c, k = top.centre, len(top.verts)
    for i in range(k):
        a, b = top.verts[i], top.verts[(i + 1) % k]
        m = P.sub(P.mul(P.add(a, b), 0.5), c)
        if abs(P.dot(P.unit(m), P.unit(P.sub(b, a)))) > 1e-6:
            return False
    return True


def size(verts, V=280e3):
    lo, hi = 5.0, 300.0
    for _ in range(60):
        e = (lo + hi) / 2
        vol, _ = P.inner(faces_of(verts, e))
        lo, hi = (e, hi) if vol < V else (lo, e)
    return (lo + hi) / 2


def run(kind, faces, mode, nip):
    """The generator's build and checks on these faces, posing as the cube."""
    saved = (P.cube, G.FACE_HOLES, dict(G.NIP))
    P.cube = lambda e: faces
    G.FACE_HOLES = ('cube',) if mode == 'faces' else ()
    G.NIP['cube'] = nip
    try:
        V, kw, ratios, diams = M.design()
        rows = M.fingering_table(V, kw, diams)
        worst = max(abs(c) for f, p, nm, c, used in rows if used)
        parts, info = G.build_solid('cube', diams)
        res, sheets, f0 = G.solid_checks('cube', parts, info, V, kw, diams, worst)
    except Exception as ex:   # noqa: BLE001
        return None, f'{type(ex).__name__}: {ex}'
    finally:
        P.cube, G.FACE_HOLES = saved[0], saved[1]
        G.NIP.clear(); G.NIP.update(saved[2])
    res = [r for r in res if r[1] != 'the cavity is the volume asked for']
    return (parts, info, res, sheets, f0, diams), None


def hands(info, diams):
    def world(i, xy):
        O, U, V_, W = info['frames'][i]
        return P.add(O, P.add(P.mul(U, xy[0]), P.mul(V_, xy[1])))
    w = {h: world(*info['places'][h]) for h in M.HOLES}
    n = {h: info['faces'][info['places'][h][0]].n for h in M.HOLES}
    top_n = info['faces'][info['ft']].n
    same = [P.dist(w['L1'], w['L2']), P.dist(w['R1'], w['R2'])]
    across = P.dist(w['L1'], w['R1'])
    tilt = max(math.degrees(math.acos(max(-1, min(1, P.dot(n[h], top_n))))) for h in M.HOLES)
    win = (G.T + M.WINDOW_L / 2, 0.0)
    wpt = world(info['ft'], win)
    reach = max(P.dist(wpt, w[h]) for h in M.HOLES)
    return max(same), across, tilt, reach


def report(name, faces, nip_guess):
    out = []
    for mode in ('grid', 'faces'):
        for nip in sorted({0.0, nip_guess, 3.5, 4.25, 4.75, 5.5}):
            r, err = run(name, faces, mode, nip)
            if r is None:
                last = err
                continue
            parts, info, res, sheets, f0, diams = r
            fails = [w for ok, w, d in res if not ok]
            last = fails
            if not fails:
                same, across, tilt, reach = hands(info, diams)
                out.append((mode, nip, info['V'], f0, len(parts), same, across, tilt, reach,
                            len(sheets), info['back']))
                break
        else:
            out.append((mode, 'FAIL', last))
    return out


if __name__ == '__main__':
    which = sys.argv[1:] or None
    print('THE BUILDS')
    for kind in G.BUILDS:
        if which and kind not in which:
            continue
        V, kw, ratios, diams = M.design()
        rows = M.fingering_table(V, kw, diams)
        worst = max(abs(c) for f, p, nm, c, used in rows if used)
        parts, info = G.build_solid(kind, diams)
        same, across, tilt, reach = hands(info, diams)
        f0 = M.freq(0, kw, info['V'])
        print(f'  {kind:28} {info["V"]/1e3:6.1f}cm3 {G.note_name(f0):4} parts {len(parts):3}  '
              f'idx-mid {same:5.1f}  L-R {across:5.1f}  tilt {tilt:5.1f}  win-hole {reach:5.1f}  chin-back {info["back"]:.1f}')
    print('\nCANDIDATES')
    for name, v in C.items():
        if which and name not in which:
            continue
        th, faces, es = dihedrals(v)
        Ls = [E['L'] for E in es]
        if max(Ls) / min(Ls) > 1.001:
            print(f'  {name:32} BAD CONSTRUCTION: edges {min(Ls):.3f} to {max(Ls):.3f}')
            continue
        deg = degree(faces)
        e = len(es)
        if th < 90 - 1e-6:
            print(f'  {name:32} skip: a {th:.1f} degree joint (needs 90 or more)')
            continue
        try:
            edge = size(v)
            fs = faces_of(v, edge)
        except Exception as ex:  # noqa: BLE001
            print(f'  {name:32} skip: {ex}')
            continue
        tops = [f for f in fs if f.name == 'top']
        ranks = (0,) if regular_top(v) else (0, 1)
        print(f'  {name:32} {len(fs)} faces, {e} edges, min joint {th:.1f} deg, up to {deg} at a corner, '
              f'edge {edge:.1f}mm, top a {len(tops[0].verts)}-gon')
        for rank in ranks:
            if len(ranks) > 1:
                print(f'    the beak on the top\'s edges of rank {rank}')
            for row in report(name, faces_of(v, edge, rank), 0.0 if deg == 3 else 4.25):
                if row[1] == 'FAIL':
                    print(f'      {row[0]:5}: fails {row[2]}')
                    continue
                mode, nip, V, f0, np_, same, across, tilt, reach, ns, back = row
                print(f'      {mode:5}: PASS nip {nip:g}  {V/1e3:.1f}cm3 {G.note_name(f0)} {f0:.1f}Hz  '
                      f'parts {np_}, sheets {ns}  idx-mid {same:.1f}  L-R {across:.1f}  '
                      f'tilt {tilt:.1f}  win-hole {reach:.1f}  chin-back {back:.1f}')

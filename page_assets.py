#!/usr/bin/env python3
"""The published page's moving parts, built from the generator's own build()
so the page cannot disagree with the cut files. The writeup is README.md,
built into index.html by md2html.py like every page here; this adds what
markdown cannot carry.

    python3 page_assets.py                     # viewer.js, turn.html and plans/*.svg
    python3 page_assets.py --artifact OUT.html # the whole page, one file

  viewer.js    the 3D view of both builds, loaded on the page through a
               page-only line in README.md
  turn.html    viewer.js on a page of its own, for the hub to frame
  plans/       the top plate drawn to scale, and the fingering chart, as
               plain SVG, so they read on GitHub as well as on the page

The viewer's styling, its page wrapper and turn.html's are the square
flute's, imported from its page_assets.py; the script that places the parts
is this instrument's own, since the parts are.
"""
import base64
import importlib.util
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import four_hole_ocarinas as B                                        # noqa: E402
M = B.M
# loaded by path: by name, `import page_assets` finds this file
_spec = importlib.util.spec_from_file_location(
    'square_flute_page_assets', os.path.join(HERE, '..', 'square-flute', 'page_assets.py'))
SFP = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(SFP)

T = B.T
FONT = 'font-family="Helvetica, Arial, sans-serif"'


def solids(kind, parts, info):
    """Every part as a flat outline, its holes, and where it stands: a 3x4
    matrix taking the part's own (x, y, extrusion) to the ocarina's (x along,
    y across, z up), and the way it moves when the view is exploded.
    Physical outlines, before the kerf: this is the ocarina, not the sheet."""
    PW, D, W = info['PW'], B.D_IN, B.W_IN
    plate = lambda z0: [[1, 0, 0, 0], [0, 1, 0, -PW / 2], [0, 0, 1, z0]]
    out = []
    k_side = k_end = k_frame = 0
    for p in parts:
        holes = []
        for c in p.cuts:
            if c[0] == 'rect':
                holes.append({'p': [[round(x, 3), round(y, 3)] for x, y in c[1]]})
            else:
                holes.append({'c': [round(c[1], 3), round(c[2], 3), round(c[3], 3)]})
        n = p.name
        flip = False
        if getattr(p, 'm', None):
            # a box plate places itself; the view centres y on the box
            m = [list(r) for r in p.m]
            m[1][3] -= PW / 2
            out_dir = {'top plate': [0, 0, 1.2], 'bottom plate': [0, 0, -1],
                       'side wall': [0, 1 if k_side else -1, 0],
                       'end wall': [-1.4 if k_end == 0 else 1.4, 0, 0]}[n]
            e = out_dir
            k_side += n == 'side wall'
            k_end += n == 'end wall'
            g = 'plate' if n.endswith('plate') else 'wall'
        elif n in ('bottom plate', 'bottom layer'):
            m, e, g = plate(0), [0, 0, -1], 'plate'
        elif n == 'frame layer':
            m = plate(T + k_frame * T)
            e = [0, 0, -0.75 + 0.25 * k_frame]
            k_frame += 1
            g = 'wall'
        elif n == 'bird':
            # the duct is on its underside, hidden from above: exploded, it
            # comes out toward the viewer and turns over, as the square
            # flute's bird does
            m, e, g, flip = plate(2 * T + D), [0, -1.6, 1.6], 'bird', True
        elif n == 'chin layer':                    # under the beak
            m, e, g = plate(D), [-0.6, 0, -1.6], 'bird'
        else:
            raise ValueError(f'no place for a {n}')
        out.append({'o': [[round(x, 3), round(y, 3)] for x, y in p.outline],
                    'h': holes, 'm': [[round(v, 4) for v in r] for r in m],
                    'e': e, 'g': g, 'name': n,
                    'f': [[[round(x, 3), round(y, 3)] for x, y in f] for f in p.fills],
                    'fc': getattr(p, 'ink', None),
                    'flip': flip, 'zc': round(m[2][3] + m[2][2] * T / 2, 3)})
    return {'parts': out, 'len': round(info['total'], 1), 'top': round(2 * T + D, 2),
            'cav': f'{W:g} × {D:g} × {info["Lc"]:.1f}mm'}


def solid_solids(parts, info):
    """A solid build's parts for the view: each places itself by p.m, moved
    so the solid's mouth face sits at x = 0 like the box's, and comes apart
    along its face's outward normal; the bird turns over as the box's does."""
    faces = info['faces']
    shift = -min(v[0] for f in faces for v in f.verts)
    zmax = max(v[2] for f in faces for v in f.verts)
    zmin = min(v[2] for f in faces for v in f.verts)
    out = []
    for p in parts:
        m = [list(r) for r in p.m]
        m[0][3] += shift
        m[2][3] -= zmin
        flip = False
        if p.name == 'bird':
            e, g, flip = [0, -1.6, 1.6], 'bird', True
        elif p.name == 'chin layer':
            e, g = [-1.6, 0, -0.6], 'bird'
        else:
            n = faces[p.face].n
            e = [round(1.3 * x, 4) for x in n]
            g = 'plate' if p.name in ('top plate', 'bottom') else 'wall'
        holes = [{'p': [[round(x, 3), round(y, 3)] for x, y in c[1]]} if c[0] == 'rect'
                 else {'c': [round(c[1], 3), round(c[2], 3), round(c[3], 3)]} for c in p.cuts]
        zc = m[2][3] + m[2][2] * T / 2            # mid-thickness, for the flip
        out.append({'o': [[round(x, 3), round(y, 3)] for x, y in p.outline],
                    'h': holes, 'm': [[round(v, 4) for v in r] for r in m],
                    'e': e, 'g': g, 'name': p.name,
                    'f': [[[round(x, 3), round(y, 3)] for x, y in f] for f in p.fills],
                    'fc': getattr(p, 'ink', None),
                    'flip': flip, 'zc': round(zc, 3)})
        if hasattr(p, 'numbers'):
            # drawn inside face up for the laser; back into the plate's frame
            out[-1]['n'] = [[[round(x, 3), round(-y, 3)] for x, y in st]
                            for st in B.inside_up(p).strokes]
            out[-1]['nl'] = [[t, c[0], c[1]] for t, c, _ in p.numbers]
    span = max(v[0] for f in faces for v in f.verts) + shift
    return {'parts': out, 'len': round(span, 1), 'top': round(zmax - zmin, 2),
            'cav': f'{info["V"] / 1000:.0f}cm³'}


def plan_svg(info, diams, title):
    """The top plate from above, to scale, beak on the left: the bird's
    footprint dashed, the duct in magenta, the window, the four holes with
    their names and sizes, and the cavity's walls dotted under the plate.
    Fixed colours on a white ground, like the previews."""
    PW, c0, Lc, total = info['PW'], info['c0'], info['Lc'], info['total']
    yc, y0, x0 = PW / 2, 6.0, B.BEAK + 4
    X = lambda x: x + x0
    Y = lambda y: y + y0
    o = [(X(x), Y(y)) for x, y in
         B.rounded(B.beaked([(0, 0), (total, 0), (total, PW), (0, PW)], 0, PW), yc)]
    d = lambda pts: 'M ' + ' L '.join(f'{X(x):.2f},{Y(y):.2f}' for x, y in pts) + ' Z'
    s = [f'<rect x="0" y="0" width="{X(total) + 6:.1f}" height="{Y(PW) + 6:.1f}" fill="#ffffff"/>',
         '<path d="M ' + ' L '.join(f'{x:.2f},{y:.2f}' for x, y in o)
         + ' Z" fill="#e7dcc4" stroke="#1d232b" stroke-width=".4"/>',
         f'<rect x="{X(c0):.2f}" y="{Y(yc - B.W_IN / 2):.2f}" width="{Lc:.2f}" height="{B.W_IN:g}" '
         f'fill="none" stroke="#8a7350" stroke-width=".5" stroke-dasharray="1 1"/>',
         f'<path d="{d(B.mouthpiece(c0, yc))}" fill="none" stroke="#1d232b" stroke-width=".5" '
         f'stroke-dasharray="2 1.5"/>',
         f'<path d="{d(B.duct(c0, yc))}" fill="{B.NEST_INK}" fill-opacity=".55"/>',
         f'<rect x="{X(c0):.2f}" y="{Y(yc - M.WINDOW_W / 2):.2f}" width="{M.WINDOW_L:g}" '
         f'height="{M.WINDOW_W:g}" fill="#ffffff" stroke="#a85800" stroke-width=".6"/>']
    t = f'{FONT} text-anchor="middle"'
    for (h, (x, y)), d in zip(info['places'].items(), diams):
        s.append(f'<circle cx="{X(x):.2f}" cy="{Y(y):.2f}" r="{d / 2:.2f}" fill="#ffffff" '
                 f'stroke="#a85800" stroke-width=".6"/>')
        dy = -d / 2 - 2.5 if y < yc else d / 2 + 6
        s.append(f'<text x="{X(x):.2f}" y="{Y(y) + dy:.2f}" font-size="4.5" fill="#1d232b" {t}>'
                 f'{h} ø{d:.1f}</text>')
    return (f'<?xml version="1.0" encoding="utf-8"?>\n'
            f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {X(total) + 6:.0f} {Y(PW) + 6:.0f}">\n'
            f'<title>{title}</title>\n'
            f'<desc>Display drawing, not a cut file. The top plate from outside, to scale, beak '
            f'on the left: the bird\'s footprint dashed, the duct under it in magenta, the nest\'s '
            f'half engraved in this face and the bird\'s matching it above, the window, '
            f'the four finger holes named for their fingers with their diameters, and the '
            f'cavity\'s walls dotted beneath. The right hand\'s holes are at the bottom of the '
            f'drawing: the player sees the other face.</desc>\n' + '\n'.join(s) + '\n</svg>\n')


def chart_svg(diams, title):
    """The fingering chart: one cell per note of the scale, each the four
    holes as the player's hands meet them -- left hand on the left, index
    fingers' row on top, beak up -- drawn in proportion, filled where a
    finger covers the hole."""
    scale = 0.55
    cw, ch, top = 34.0, 46.0, 4.0
    pos = {'L1': (-8, 10), 'L2': (-8, 22), 'R1': (8, 10), 'R2': (8, 22)}
    s = [f'<rect x="0" y="0" width="{cw * len(M.NOTES):.0f}" height="{ch:.0f}" fill="#ffffff"/>']
    t = f'{FONT} text-anchor="middle"'
    for i, (note, pat) in enumerate(zip(M.NOTES, M.PATTERN)):
        cx = cw * i + cw / 2
        s.append(f'<rect x="{cx - 13:.1f}" y="{top:.1f}" width="26" height="28" rx="3" '
                 f'fill="#e7dcc4" stroke="#1d232b" stroke-width=".4"/>')
        s.append(f'<rect x="{cx - 3:.1f}" y="{top - 3:.1f}" width="6" height="3" '
                 f'fill="#b8925c" stroke="#1d232b" stroke-width=".3"/>')
        for h, d, o in zip(M.HOLES, diams, pat):
            px, py = pos[h]
            fill = '#ffffff' if o == '1' else '#1d232b'
            s.append(f'<circle cx="{cx + px:.1f}" cy="{top + py:.1f}" r="{max(d * scale / 2, 1.0):.2f}" '
                     f'fill="{fill}" stroke="#1d232b" stroke-width=".4"/>')
        s.append(f'<text x="{cx:.1f}" y="{top + 39:.1f}" font-size="6.5" fill="#1d232b" {t}>'
                 f'{note.replace("#", "♯")}</text>')
    return (f'<?xml version="1.0" encoding="utf-8"?>\n'
            f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {cw * len(M.NOTES):.0f} {ch:.0f}">\n'
            f'<title>{title}</title>\n'
            f'<desc>Display drawing. The G major scale from G4 to G5, one cell per note: the '
            f'four holes as the player\'s hands meet them, beak up, left hand on the left, '
            f'index fingers\' holes on the upper row, each drawn in proportion to its size. '
            f'Black is covered, white is open.</desc>\n' + '\n'.join(s) + '\n</svg>\n')


MARKUP = '<section class="viewer" id="turn" aria-label="Turn it in 3D">\n  <div class="vbar" role="group" aria-label="Build">__BUTTONS__</div>\n  <div class="stage" id="stage"><canvas id="cv" aria-label="The selected build in 3D. Drag to turn, scroll or pinch to zoom."></canvas>\n    <p class="vinfo" id="vinfo"></p></div>\n  <div class="vctl">\n    <label for="explode">Explode</label><input id="explode" type="range" min="0" max="1" step="0.01" value="0">\n    <button type="button" id="vreset">Reset view</button>\n    <label class="chk"><input type="checkbox" id="xray"> See through the walls</label>\n    <label class="chk"><input type="checkbox" id="nums" checked> Corner numbers</label>\n  </div>\n  <details class="note vhow"><summary>How to use this view</summary><p>Drag to turn, scroll or pinch to zoom. The parts are the generator\'s own outlines before the kerf, so this is the ocarina as built: the box finger-jointed on all twelve edges, every finger flush to the outside; the five solids jointed on every edge at their own angles; the stack\'s six frame layers round the cavity; the window and the four holes in the top; and the beak, the bird glued over it and the chin layer under it. The duct is drawn blue, and exploded, the bird comes out toward you and turns over so the duct faces up.</p></details>\n</section>'


NUM_JS = '''  let labels = [];
  const numTex = new Map();
  function numLabel(t) {   // a corner number: white on red, facing the camera, a fixed size on screen
    if (!numTex.has(t)) {
      const c = document.createElement('canvas'); c.width = 96; c.height = 64;
      const g = c.getContext('2d');
      g.fillStyle = '#c62020'; g.beginPath(); g.roundRect(4, 4, 88, 56, 14); g.fill();
      g.fillStyle = '#ffffff'; g.font = 'bold 40px system-ui, sans-serif';
      g.textAlign = 'center'; g.textBaseline = 'middle'; g.fillText(t, 48, 34);
      const tex = new THREE.CanvasTexture(c); tex.colorSpace = THREE.SRGBColorSpace; numTex.set(t, tex);
    }
    const sp = new THREE.Sprite(new THREE.SpriteMaterial({ map: numTex.get(t), depthTest: false, sizeAttenuation: false }));
    sp.scale.set(0.054, 0.036, 1); sp.renderOrder = 10;
    return sp;
  }
  function showNums() {    // on the plates whose outside faces the camera; all of them through the walls
    const on = document.getElementById('nums').checked, w = new THREE.Vector3(), n = new THREE.Vector3();
    if (labels.length) scene.updateMatrixWorld(true);
    labels.forEach(sp => {
      if (!on) { sp.visible = false; return; }
      sp.getWorldPosition(w);
      n.setFromMatrixColumn(sp.parent.matrixWorld, 2).negate();
      sp.visible = xr.checked || n.dot(camera.position.clone().sub(w)) > 0;
    });
  }
'''


def viewer_js(three):
    # the square flute's script, less its three toggles: here a button per
    # build is the only choice. Cut at the toggles' comment, which starts the
    # part this view does not have, and closed with the plain picker below
    js = SFP.JS.split('  // one button per construction')[0]
    js = js.replace("'mm · bore ' + d.W.toFixed(2) + ' × ' + d.D.toFixed(2) + 'mm'",
                    "'mm body · cavity ' + d.cav")
    js = js.replace('let yaw = -0.65, pitch = 0.42, dist = 600;\n  const home = { yaw: -0.65, pitch: 0.42, dist: 600 };',
                    'let yaw = -0.65, pitch = 0.62, dist = 380;\n  const home = { yaw: -0.65, pitch: 0.62, dist: 380 };')
    # a solid's corner numbers, red lines just inside each plate's inner face
    a = '      const m = p.m, M = new THREE.Matrix4();'
    assert a in js, 'the square flute\'s viewer script changed'
    js = js.replace(a, "      (p.n || []).forEach(st => holder.add(new THREE.Line(new THREE.BufferGeometry()"
                       ".setFromPoints(st.map(q => new THREE.Vector3(q[0], q[1], 3.05))), numMat)));\n" + a)
    a = '  function build(name) {'
    assert a in js, 'the square flute\'s viewer script changed'
    js = js.replace(a, "  const numMat = new THREE.LineBasicMaterial({ color: 0xd02020 });\n" + NUM_JS + a)
    # each number also as a label facing the camera, on the plates turned
    # toward it (all of them, seeing through the walls), off by its checkbox
    a = '    const d = DATA[name];'
    assert a in js, 'the square flute\'s viewer script changed'
    js = js.replace(a, a + '\n    labels = [];')
    a = "      const m = p.m, M = new THREE.Matrix4();"
    js = js.replace(a, "      (p.nl || []).forEach(l => { const sp = numLabel(l[0]); sp.position.set(l[1], l[2], -0.6); "
                       "holder.add(sp); labels.push(sp); });\n" + a)
    a = '    renderer.render(scene, camera);\n  }'
    assert js.count(a) == 1, 'the square flute\'s viewer script changed'
    js = js.replace(a, "    showNums();\n" + a)
    assert "d.cav" in js and 'dist = 380' in js, 'the square flute\'s viewer script changed'
    js += ("  const bb = [...document.querySelectorAll('#turn .vbar button[data-v]')];\n"
           "  bb.forEach(b => b.addEventListener('click', () => {\n"
           "    bb.forEach(x => x.setAttribute('aria-pressed', x === b)); build(b.dataset.v);\n"
           "  }));\n"
           "  document.getElementById('nums').addEventListener('change', render);\n"
           "  window.addEventListener('resize', size);\n"
           "  build(bb[0].dataset.v); size();\n"
           "})();\n")
    js = js.replace('__DATA__', json.dumps(three, separators=(',', ':')))
    buttons = ''.join(f'<button type="button" data-v="{n}" aria-pressed="{str(i == 0).lower()}">'
                      f'<code>{n}</code></button>' for i, n in enumerate(three))
    return (SFP.VIEWER_JS.replace('__CSS__', json.dumps(SFP.CSS))
            .replace('__MARKUP__', json.dumps(MARKUP.replace('__BUTTONS__', buttons)))
            .replace('__JS__', js))


def artifact(out):
    """index.html with viewer.js inlined and every local image a data URI,
    one file, without the document skeleton the artifact host supplies."""
    page = open(os.path.join(HERE, 'index.html')).read()
    page = page.replace('<script src="viewer.js"></script>',
                        '<script>' + open(os.path.join(HERE, 'viewer.js')).read() + '</script>')

    def data(m):
        src = m.group(1)
        p = os.path.join(HERE, src)
        if src.startswith(('http', 'data:')) or not os.path.exists(p):
            return m.group(0)
        kind = 'image/svg+xml' if src.endswith('.svg') else 'image/png'
        return f'src="data:{kind};base64,' + base64.b64encode(open(p, 'rb').read()).decode() + '"'
    page = re.sub(r'src="([^"]+\.(?:png|svg))"', data, page)
    page = re.sub(r'(?is)<!doctype[^>]*>|</?html[^>]*>|</?body[^>]*>|</?head>', '', page)
    open(out, 'w').write(page)
    print(f'  wrote {out}: {len(page) // 1024}KB')


PLANS = ('four-hole-ocarina-G4-top-plate.svg', 'four-hole-ocarina-G4-fingering.svg')


def main():
    a = sys.argv[1:]
    if a[:1] == ['--artifact'] and len(a) == 2:
        return artifact(a[1])
    # --viewer-to=FILE, --plans-to=DIR and --turn-to=FILE write one asset where
    # .repro asks, so the reproduction gate compares them like any other file
    vt = [x.split('=', 1)[1] for x in a if x.startswith('--viewer-to=')]
    pt = [x.split('=', 1)[1] for x in a if x.startswith('--plans-to=')]
    tt = [x.split('=', 1)[1] for x in a if x.startswith('--turn-to=')]
    if len(vt) + len(pt) + len(tt) != len(a):
        raise SystemExit('usage: page_assets.py [--viewer-to=FILE] [--plans-to=DIR] '
                         '[--turn-to=FILE] | --artifact OUT.html')
    if tt or not a:
        page = SFP.TURN_PAGE.replace('<title>Square-bore flute in 3D</title>',
                                     '<title>Four-hole ocarinas in 3D</title>')
        assert 'Four-hole ocarinas' in page, 'the square flute\'s turn page changed'
        open(tt[0] if tt else os.path.join(HERE, 'turn.html'), 'w').write(page)
    V, kw, ratios, diams = M.design()
    three, infos = {}, {}
    for name in B.BUILDS:
        if name in B.SOLIDS:
            parts, info = B.build_solid(name, diams)
            three[name] = solid_solids(parts, info)
            continue
        parts, info = B.build(name, V, diams)
        three[name], infos[name] = solids(name, parts, info), info
    if vt or not a:
        open(vt[0] if vt else os.path.join(HERE, 'viewer.js'), 'w').write(viewer_js(three))
    if pt or not a:
        d = pt[0] if pt else os.path.join(HERE, 'plans')
        os.makedirs(d, exist_ok=True)
        open(os.path.join(d, PLANS[0]), 'w').write(
            plan_svg(infos['box'], diams, 'The box\'s top plate from outside, to scale'))
        open(os.path.join(d, PLANS[1]), 'w').write(
            chart_svg(diams, 'Fingering chart, G4 to G5'))
    print(f'  wrote viewer.js: {len(three)} builds; plans: {", ".join(PLANS)}')
    return 0


if __name__ == '__main__':
    sys.exit(main())

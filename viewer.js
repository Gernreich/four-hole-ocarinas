// The 3D view of every build, for index.html. Built by page_assets.py from
// the generators' own parts: do not edit by hand.
(function () {
  var mount = document.getElementById('turn3d');
  if (!mount) return;
  var style = document.createElement('style');
  style.textContent = "#turn{display:grid;gap:12px;margin:8px 0 24px}\n#turn .vbar{display:flex;flex-wrap:wrap;align-items:center;gap:8px}\n#turn .vbar button:disabled{opacity:.45;cursor:default}\n#turn button{font:500 13px ui-monospace,Menlo,monospace;color:var(--fg);background:var(--card);border:1px solid var(--line);border-radius:4px;padding:6px 10px;cursor:pointer}\n#turn .vbar button[aria-pressed=\"true\"]{border-color:var(--accent);box-shadow:inset 0 -3px 0 var(--accent)}\n#turn button:focus-visible,#turn input:focus-visible{outline:2px solid var(--accent);outline-offset:2px}\n#turn .stage{position:relative;background:var(--card);border:1px solid var(--line);border-radius:6px;overflow:hidden;touch-action:none}\n#turn canvas{display:block;width:100%;cursor:grab}\n#turn .vinfo{position:absolute;left:12px;bottom:10px;margin:0;font:12px ui-monospace,Menlo,monospace;color:var(--mut);pointer-events:none}\n#turn .vctl{display:flex;flex-wrap:wrap;align-items:center;gap:10px 16px;font-size:14px}\n#turn input[type=range]{width:min(260px,60vw);accent-color:var(--accent)}\n#turn .chk{display:inline-flex;gap:6px;align-items:center}\n#turn .note{color:var(--mut);font-size:14px;margin:0}\n#turn .vhow summary{cursor:pointer;width:max-content;max-width:100%}\n#turn .vhow p{margin:6px 0 0}";
  document.head.appendChild(style);
  mount.innerHTML = "<section class=\"viewer\" id=\"turn\" aria-label=\"Turn it in 3D\">\n  <div class=\"vbar\" role=\"group\" aria-label=\"Build\"><button type=\"button\" data-v=\"box\" aria-pressed=\"true\"><code>box</code></button><button type=\"button\" data-v=\"stack\" aria-pressed=\"false\"><code>stack</code></button></div>\n  <div class=\"stage\" id=\"stage\"><canvas id=\"cv\" aria-label=\"The selected build in 3D. Drag to turn, scroll or pinch to zoom.\"></canvas>\n    <p class=\"vinfo\" id=\"vinfo\"></p></div>\n  <div class=\"vctl\">\n    <label for=\"explode\">Explode</label><input id=\"explode\" type=\"range\" min=\"0\" max=\"1\" step=\"0.01\" value=\"0\">\n    <button type=\"button\" id=\"vreset\">Reset view</button>\n    <label class=\"chk\"><input type=\"checkbox\" id=\"xray\"> See through the walls</label>\n  </div>\n  <details class=\"note vhow\"><summary>How to use this view</summary><p>Drag to turn, scroll or pinch to zoom. The parts are the generator's own outlines before the kerf, so this is the ocarina as built: the box's fingers in their notches, flush to the outside, and its two end plugs, the far one set in from the end to tune; the stack's six frame layers round the cavity; the window and the four holes in the top; and the bird over the beak. The duct is drawn blue, and exploded, the bird's bottom layer comes out toward you and turns over so the duct faces up.</p></details>\n</section>";
  var s = document.createElement('script');
  s.src = 'https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js';
  s.onload = run;
  s.onerror = function () { document.getElementById('vinfo').textContent = 'The 3D view needs three.js, which did not load.'; };
  document.head.appendChild(s);
  function run() {
(function () {
  const DATA = {"box":{"parts":[{"o":[[0,0],[15.28,0],[15.28,3.0],[21.28,3.0],[21.28,0],[27.28,0],[27.28,3.0],[33.28,3.0],[33.28,0],[39.28,0],[39.28,3.0],[45.28,3.0],[45.28,0],[51.28,0],[51.28,3.0],[57.28,3.0],[57.28,0],[63.28,0],[63.28,3.0],[69.28,3.0],[69.28,0],[75.28,0],[75.28,3.0],[81.28,3.0],[81.28,0],[87.28,0],[87.28,3.0],[93.28,3.0],[93.28,0],[108.56,0],[108.56,66.0],[93.28,66.0],[93.28,63.0],[87.28,63.0],[87.28,66.0],[81.28,66.0],[81.28,63.0],[75.28,63.0],[75.28,66.0],[69.28,66.0],[69.28,63.0],[63.28,63.0],[63.28,66.0],[57.28,66.0],[57.28,63.0],[51.28,63.0],[51.28,66.0],[45.28,66.0],[45.28,63.0],[39.28,63.0],[39.28,66.0],[33.28,66.0],[33.28,63.0],[27.28,63.0],[27.28,66.0],[21.28,66.0],[21.28,63.0],[15.28,63.0],[15.28,66.0],[0,66.0],[0,45.0],[-18.0,45.0],[-18.0,21.0],[0,21.0]],"h":[{"p":[[6.0,28.0],[11.0,28.0],[11.0,38.0],[6.0,38.0]]},{"c":[58.0,51.0,1.473]},{"c":[36.0,51.0,2.617]},{"c":[36.0,15.0,4.02]},{"c":[58.0,15.0,6.116]}],"m":[[1,0,0,0],[0,1,0,-33.0],[0,0,1,21.0]],"e":[0,0,1.2],"g":"plate","name":"top plate","f":[],"flip":false,"zc":22.5},{"o":[[0,0],[15.28,0],[15.28,3.0],[21.28,3.0],[21.28,0],[27.28,0],[27.28,3.0],[33.28,3.0],[33.28,0],[39.28,0],[39.28,3.0],[45.28,3.0],[45.28,0],[51.28,0],[51.28,3.0],[57.28,3.0],[57.28,0],[63.28,0],[63.28,3.0],[69.28,3.0],[69.28,0],[75.28,0],[75.28,3.0],[81.28,3.0],[81.28,0],[87.28,0],[87.28,3.0],[93.28,3.0],[93.28,0],[108.56,0],[108.56,66.0],[93.28,66.0],[93.28,63.0],[87.28,63.0],[87.28,66.0],[81.28,66.0],[81.28,63.0],[75.28,63.0],[75.28,66.0],[69.28,66.0],[69.28,63.0],[63.28,63.0],[63.28,66.0],[57.28,66.0],[57.28,63.0],[51.28,63.0],[51.28,66.0],[45.28,66.0],[45.28,63.0],[39.28,63.0],[39.28,66.0],[33.28,66.0],[33.28,63.0],[27.28,63.0],[27.28,66.0],[21.28,66.0],[21.28,63.0],[15.28,63.0],[15.28,66.0],[0,66.0]],"h":[],"m":[[1,0,0,0],[0,1,0,-33.0],[0,0,1,0]],"e":[0,0,-1],"g":"plate","name":"bottom plate","f":[],"flip":false,"zc":1.5},{"o":[[0,0],[15.28,0],[15.28,-3.0],[21.28,-3.0],[21.28,0],[27.28,0],[27.28,-3.0],[33.28,-3.0],[33.28,0],[39.28,0],[39.28,-3.0],[45.28,-3.0],[45.28,0],[51.28,0],[51.28,-3.0],[57.28,-3.0],[57.28,0],[63.28,0],[63.28,-3.0],[69.28,-3.0],[69.28,0],[75.28,0],[75.28,-3.0],[81.28,-3.0],[81.28,0],[87.28,0],[87.28,-3.0],[93.28,-3.0],[93.28,0],[108.56,0],[108.56,18.0],[93.28,18.0],[93.28,21.0],[87.28,21.0],[87.28,18.0],[81.28,18.0],[81.28,21.0],[75.28,21.0],[75.28,18.0],[69.28,18.0],[69.28,21.0],[63.28,21.0],[63.28,18.0],[57.28,18.0],[57.28,21.0],[51.28,21.0],[51.28,18.0],[45.28,18.0],[45.28,21.0],[39.28,21.0],[39.28,18.0],[33.28,18.0],[33.28,21.0],[27.28,21.0],[27.28,18.0],[21.28,18.0],[21.28,21.0],[15.28,21.0],[15.28,18.0],[0,18.0]],"h":[],"m":[[1,0,0,0],[0,0,1,30.0],[0,-1,0,21.0]],"e":[0,1,0],"g":"wall","name":"side wall","f":[],"flip":false,"zc":22.5},{"o":[[0,0],[15.28,0],[15.28,-3.0],[21.28,-3.0],[21.28,0],[27.28,0],[27.28,-3.0],[33.28,-3.0],[33.28,0],[39.28,0],[39.28,-3.0],[45.28,-3.0],[45.28,0],[51.28,0],[51.28,-3.0],[57.28,-3.0],[57.28,0],[63.28,0],[63.28,-3.0],[69.28,-3.0],[69.28,0],[75.28,0],[75.28,-3.0],[81.28,-3.0],[81.28,0],[87.28,0],[87.28,-3.0],[93.28,-3.0],[93.28,0],[108.56,0],[108.56,18.0],[93.28,18.0],[93.28,21.0],[87.28,21.0],[87.28,18.0],[81.28,18.0],[81.28,21.0],[75.28,21.0],[75.28,18.0],[69.28,18.0],[69.28,21.0],[63.28,21.0],[63.28,18.0],[57.28,18.0],[57.28,21.0],[51.28,21.0],[51.28,18.0],[45.28,18.0],[45.28,21.0],[39.28,21.0],[39.28,18.0],[33.28,18.0],[33.28,21.0],[27.28,21.0],[27.28,18.0],[21.28,18.0],[21.28,21.0],[15.28,21.0],[15.28,18.0],[0,18.0]],"h":[],"m":[[1,0,0,0],[0,0,1,-33.0],[0,-1,0,21.0]],"e":[0,-1,0],"g":"wall","name":"side wall","f":[],"flip":false,"zc":22.5},{"o":[[0,0],[60.12,0],[60.12,18.12],[0,18.12]],"h":[],"m":[[0,0,1,0.0],[1,0,0,-30.06],[0,1,0,3.0]],"e":[-1.45,0,0],"g":"plug","name":"plug disc","f":[],"flip":false,"zc":4.5},{"o":[[0,0],[60.12,0],[60.12,18.12],[0,18.12]],"h":[],"m":[[0,0,1,3.0],[1,0,0,-30.06],[0,1,0,3.0]],"e":[-1.2,0,0],"g":"plug","name":"plug disc","f":[],"flip":false,"zc":4.5},{"o":[[0,0],[60.12,0],[60.12,18.12],[0,18.12]],"h":[],"m":[[0,0,1,99.5596],[1,0,0,-30.06],[0,1,0,3.0]],"e":[1.2,0,0],"g":"plug","name":"plug disc","f":[],"flip":false,"zc":4.5},{"o":[[0,0],[60.12,0],[60.12,18.12],[0,18.12]],"h":[],"m":[[0,0,1,102.5596],[1,0,0,-30.06],[0,1,0,3.0]],"e":[1.45,0,0],"g":"plug","name":"plug disc","f":[],"flip":false,"zc":4.5},{"o":[[-18.0,21.0],[6.0,21.0],[6.0,45.0],[-18.0,45.0]],"h":[],"m":[[1,0,0,0],[0,1,0,-33.0],[0,0,1,24.0]],"e":[0,-1.6,1.6],"g":"bird","name":"bird layer","f":[[[-18.0,28.0],[6.0,28.0],[6.0,38.0],[-18.0,38.0]]],"flip":true,"zc":25.5},{"o":[[-18.0,21.0],[6.0,21.0],[6.0,45.0],[-18.0,45.0]],"h":[],"m":[[1,0,0,0],[0,1,0,-33.0],[0,0,1,27.0]],"e":[0,0,3.0],"g":"bird","name":"bird layer","f":[],"flip":false,"zc":28.5},{"o":[[-18.0,21.0],[6.0,21.0],[6.0,45.0],[-18.0,45.0]],"h":[],"m":[[1,0,0,0],[0,1,0,-33.0],[0,0,1,30.0]],"e":[0,0,3.4000000000000004],"g":"bird","name":"bird layer","f":[],"flip":false,"zc":31.5}],"len":108.6,"top":24.0,"cav":"60 \u00d7 18 \u00d7 93.6mm"},"stack":{"parts":[{"o":[[0,0],[105.56,0],[105.56,72.0],[0,72.0],[0,48.0],[-18.0,48.0],[-18.0,24.0],[0,24.0]],"h":[{"p":[[6.0,31.0],[11.0,31.0],[11.0,41.0],[6.0,41.0]]},{"c":[58.0,54.0,1.473]},{"c":[36.0,54.0,2.617]},{"c":[36.0,18.0,4.02]},{"c":[58.0,18.0,6.116]}],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,21.0]],"e":[0,0,1.2],"g":"plate","name":"top plate","f":[],"flip":false,"zc":22.5},{"o":[[0,0],[105.56,0],[105.56,72.0],[0,72.0]],"h":[],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,0]],"e":[0,0,-1],"g":"plate","name":"bottom layer","f":[],"flip":false,"zc":1.5},{"o":[[0,0],[105.56,0],[105.56,72.0],[0,72.0]],"h":[{"p":[[6.0,6.0],[99.56,6.0],[99.56,66.0],[6.0,66.0]]}],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,3.0]],"e":[0,0,-0.75],"g":"wall","name":"frame layer","f":[],"flip":false,"zc":4.5},{"o":[[0,0],[105.56,0],[105.56,72.0],[0,72.0]],"h":[{"p":[[6.0,6.0],[99.56,6.0],[99.56,66.0],[6.0,66.0]]}],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,6.0]],"e":[0,0,-0.5],"g":"wall","name":"frame layer","f":[],"flip":false,"zc":7.5},{"o":[[0,0],[105.56,0],[105.56,72.0],[0,72.0]],"h":[{"p":[[6.0,6.0],[99.56,6.0],[99.56,66.0],[6.0,66.0]]}],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,9.0]],"e":[0,0,-0.25],"g":"wall","name":"frame layer","f":[],"flip":false,"zc":10.5},{"o":[[0,0],[105.56,0],[105.56,72.0],[0,72.0]],"h":[{"p":[[6.0,6.0],[99.56,6.0],[99.56,66.0],[6.0,66.0]]}],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,12.0]],"e":[0,0,0.0],"g":"wall","name":"frame layer","f":[],"flip":false,"zc":13.5},{"o":[[0,0],[105.56,0],[105.56,72.0],[0,72.0]],"h":[{"p":[[6.0,6.0],[99.56,6.0],[99.56,66.0],[6.0,66.0]]}],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,15.0]],"e":[0,0,0.25],"g":"wall","name":"frame layer","f":[],"flip":false,"zc":16.5},{"o":[[0,0],[105.56,0],[105.56,72.0],[0,72.0]],"h":[{"p":[[6.0,6.0],[99.56,6.0],[99.56,66.0],[6.0,66.0]]}],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,18.0]],"e":[0,0,0.5],"g":"wall","name":"frame layer","f":[],"flip":false,"zc":19.5},{"o":[[-18.0,24.0],[6.0,24.0],[6.0,48.0],[-18.0,48.0]],"h":[],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,24.0]],"e":[0,-1.6,1.6],"g":"bird","name":"bird layer","f":[[[-18.0,31.0],[6.0,31.0],[6.0,41.0],[-18.0,41.0]]],"flip":true,"zc":25.5},{"o":[[-18.0,24.0],[6.0,24.0],[6.0,48.0],[-18.0,48.0]],"h":[],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,27.0]],"e":[0,0,3.0],"g":"bird","name":"bird layer","f":[],"flip":false,"zc":28.5},{"o":[[-18.0,24.0],[6.0,24.0],[6.0,48.0],[-18.0,48.0]],"h":[],"m":[[1,0,0,0],[0,1,0,-36.0],[0,0,1,30.0]],"e":[0,0,3.4000000000000004],"g":"bird","name":"bird layer","f":[],"flip":false,"zc":31.5}],"len":105.6,"top":24.0,"cav":"60 \u00d7 18 \u00d7 93.6mm"}};
  if (!window.THREE) { document.getElementById('vinfo').textContent = 'The 3D view needs three.js, which did not load.'; return; }
  const css = getComputedStyle(document.documentElement);
  const tok = n => new THREE.Color(css.getPropertyValue(n).trim() || '#888');
  const cv = document.getElementById('cv'), stage = document.getElementById('stage');
  const renderer = new THREE.WebGLRenderer({ canvas: cv, antialias: true, alpha: true });
  renderer.setPixelRatio(Math.min(2, window.devicePixelRatio || 1));
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(32, 2, 1, 5000);
  camera.up.set(0, 0, 1);
  scene.add(new THREE.HemisphereLight(0xffffff, 0x8a8070, 0.85));
  const sun = new THREE.DirectionalLight(0xffffff, 0.75); sun.position.set(-300, -500, 700); scene.add(sun);
  const mats = {};
  function material(g) {
    const base = { plate: '#d9c6a0', wall: '#cdb68a', plug: '#a98d5f', dbl: '#e2d2b0', bird: '#b8925c' }[g];
    return new THREE.MeshStandardMaterial({ color: base, roughness: 0.85, metalness: 0, transparent: true, opacity: 1 });
  }
  const flueMat = new THREE.MeshBasicMaterial({ color: 0x2f5bff, side: THREE.DoubleSide });   // two-sided: the bird flue faces into the wood, and one-sided it was culled from every side (the author, 2026-10-13)
  const edgeMat = new THREE.LineBasicMaterial({ color: 0x3b2f20, transparent: true, opacity: 0.55 });
  let group = null, meshes = [], cur = null;
  function shapeOf(o) { const s = new THREE.Shape(); o.forEach((p, i) => i ? s.lineTo(p[0], p[1]) : s.moveTo(p[0], p[1])); s.closePath(); return s; }
  function build(name) {
    if (group) { scene.remove(group); group.traverse(o => { if (o.geometry) o.geometry.dispose(); }); }
    group = new THREE.Group(); meshes = [];
    const d = DATA[name];
    d.parts.forEach(p => {
      const s = shapeOf(p.o);
      p.h.forEach(h => {
        const path = new THREE.Path();
        if (h.c) path.absarc(h.c[0], h.c[1], h.c[2], 0, Math.PI * 2, true);
        else h.p.forEach((q, i) => i ? path.lineTo(q[0], q[1]) : path.moveTo(q[0], q[1]));
        s.holes.push(path);
      });
      const geo = new THREE.ExtrudeGeometry(s, { depth: 3, bevelEnabled: false, curveSegments: 24 });
      const mat = mats[p.g] || (mats[p.g] = material(p.g));
      const mesh = new THREE.Mesh(geo, mat);
      const holder = new THREE.Group();
      holder.add(mesh);
      holder.add(new THREE.LineSegments(new THREE.EdgesGeometry(geo, 30), edgeMat));
      p.f.forEach(f => {   // the flue, on the face it is engraved into
        const fg = new THREE.ShapeGeometry(shapeOf(f));
        const fm = new THREE.Mesh(fg, flueMat); fm.position.z = p.ft ? 3.05 : -0.05; holder.add(fm);
      });
      const m = p.m, M = new THREE.Matrix4();
      M.set(m[0][0], m[0][1], m[0][2], m[0][3], m[1][0], m[1][1], m[1][2], m[1][3], m[2][0], m[2][1], m[2][2], m[2][3], 0, 0, 0, 1);
      holder.matrixAutoUpdate = false; holder.userData = { M, e: p.e, flip: p.flip, zc: p.zc };
      group.add(holder); meshes.push(holder);
    });
    scene.add(group);
    cur = d;
    target.set(d.len / 2, 0, d.top / 2);
    document.getElementById('vinfo').textContent = name + ' · ' + d.parts.length + ' parts · ' + d.len.toFixed(0) + 'mm body · cavity ' + d.cav;
    place(); render();
  }
  const target = new THREE.Vector3();
  let yaw = -0.65, pitch = 0.62, dist = 380;
  const home = { yaw: -0.65, pitch: 0.62, dist: 380 };
  const ex = document.getElementById('explode'), xr = document.getElementById('xray');
  function place() {
    const k = parseFloat(ex.value) * 28;
    meshes.forEach(h => {
      const u = h.userData, T = new THREE.Matrix4().makeTranslation(u.e[0] * k, u.e[1] * k, u.e[2] * k);
      if (u.flip) {   // turned over about its own long axis as it comes out
        const c = new THREE.Matrix4().makeTranslation(0, 0, u.zc), ci = new THREE.Matrix4().makeTranslation(0, 0, -u.zc);
        T.multiply(c).multiply(new THREE.Matrix4().makeRotationX(Math.PI * parseFloat(ex.value))).multiply(ci);
      }
      h.matrix.copy(T.multiply(u.M));
    });
    Object.entries(mats).forEach(([g, m]) => { m.opacity = xr.checked && (g === 'wall' || g === 'plate' || g === 'dbl') ? 0.28 : 1; m.depthWrite = !(xr.checked && g !== 'plug' && g !== 'bird'); });
  }
  function render() {
    camera.position.set(target.x + dist * Math.cos(pitch) * Math.sin(yaw), target.y - dist * Math.cos(pitch) * Math.cos(yaw), target.z + dist * Math.sin(pitch));
    camera.lookAt(target);
    renderer.render(scene, camera);
  }
  function size() {
    const w = stage.clientWidth, h = Math.max(260, Math.min(520, w * 0.55));
    renderer.setSize(w, h, false); cv.style.height = h + 'px'; camera.aspect = w / h; camera.updateProjectionMatrix(); render();
  }
  const pts = new Map(); let pinch0 = 0, dist0 = dist;
  cv.addEventListener('pointerdown', e => { cv.setPointerCapture(e.pointerId); pts.set(e.pointerId, { x: e.clientX, y: e.clientY }); if (pts.size === 2) { const [a, b] = [...pts.values()]; pinch0 = Math.hypot(a.x - b.x, a.y - b.y); dist0 = dist; } });
  cv.addEventListener('pointermove', e => {
    if (!pts.has(e.pointerId)) return;
    const prev = pts.get(e.pointerId); pts.set(e.pointerId, { x: e.clientX, y: e.clientY });
    if (pts.size === 1) { yaw += (e.clientX - prev.x) * 0.008; pitch = Math.max(-1.45, Math.min(1.45, pitch + (e.clientY - prev.y) * 0.008)); }
    else if (pts.size === 2 && pinch0) { const [a, b] = [...pts.values()]; dist = Math.max(120, Math.min(2000, dist0 * pinch0 / Math.hypot(a.x - b.x, a.y - b.y))); }
    render();
  });
  const up = e => { pts.delete(e.pointerId); if (pts.size < 2) pinch0 = 0; };
  cv.addEventListener('pointerup', up); cv.addEventListener('pointercancel', up);
  cv.addEventListener('wheel', e => { e.preventDefault(); dist = Math.max(120, Math.min(2000, dist * (1 + Math.sign(e.deltaY) * 0.08))); render(); }, { passive: false });
  ex.addEventListener('input', () => { place(); render(); });
  xr.addEventListener('change', () => { place(); render(); });
  document.getElementById('vreset').addEventListener('click', () => { ({ yaw, pitch, dist } = home); ex.value = 0; place(); render(); });
  const bb = [...document.querySelectorAll('#turn .vbar button[data-v]')];
  bb.forEach(b => b.addEventListener('click', () => {
    bb.forEach(x => x.setAttribute('aria-pressed', x === b)); build(b.dataset.v);
  }));
  window.addEventListener('resize', size);
  build(bb[0].dataset.v); size();
})();

  }
})();

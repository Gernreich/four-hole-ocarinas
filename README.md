# Box ocarina

<!-- readme-only -->
**[Read the writeup](https://gernreich.github.io/box-ocarina/)** ·
**[Turn all four builds in 3D](https://gernreich.github.io/box-ocarina/#turn-it-in-3d)**

<!-- page-only **[Read the README](https://github.com/Gernreich/box-ocarina)** -->

A four-hole ocarina in G4, in 3mm plywood: a rectangular box with a beak to blow
into, four finger holes in its top, and the English fingering, which gets a whole
octave out of four holes. All closed it sounds G4; the fingerings below give the
G major scale up to G5, and five more notes besides.

Two more are solids, finger-jointed on every edge at whatever angle their faces
meet: a regular octagonal prism and a regular truncated cube. They take the same
window, holes and fingering, so every interval is the same; only their cavities
differ, and with them the key.

Output is millimetre-true — `1 user unit = 1mm` with a physical `width`/`height` — so
it prints and cuts at real size.

**No build has been cut or played yet.** The cavity and the holes come from an
acoustic model whose window term is not measured, and the first one cut will set it.
[Before you cut](#before-you-cut) says what that means for the first build.

<table>
<tr>
<td align="center"><a href="cut-files/box/box-ocarina-G4-box-sheet1-cut-files.svg"><img src="previews/box/box-ocarina-G4-box-sheet1-cut-files.svg" alt="One sheet of parts: a top plate with a short beak at one end, its tip rounded, a small rectangular window beside the beak and four round holes of four sizes, every edge notched for finger joints; a matching bottom plate without the beak; two side walls and two end walls with fingers on all four edges, one end wall plain along part of its top edge where the beak goes; and two small pieces for the mouthpiece, rounded at one end like the beak, one with a blue band across it for the duct" width="100%"></a></td>
</tr>
<tr>
<td align="center"><sub>box · the whole ocarina on one 579 × 104mm sheet</sub></td>
</tr>
</table>

Built for **[LaserMadeMusic](https://www.youtube.com/@LaserMadeMusic)**.

**[The rest of the build files](https://gernreich.github.io/)** — every instrument,
generator and tool, indexed.

**[Download everything as a ZIP](https://github.com/Gernreich/box-ocarina/archive/refs/heads/main.zip)**

## The builds

Four builds in three sets. The first set is two ways to build the same G4 ocarina:
its cavity, window and four holes are identical, and only the walls round the
cavity differ. The second and third sets are the solids, which play in whatever key
their shape holds.

| Build | Construction | Cavity | Key, all closed | Body | Parts | Sheets |
|---|---|---|---|---|---|---|
| `box` | finger-jointed on all twelve edges | 60 × 18 × 93.6mm, 101cm³ | G4, 392.0Hz | 99.6 × 66.0 × 24mm | 8 | 1 |
| `stack` | laminated stack | 60 × 18 × 93.6mm, 101cm³ | G4, 392.0Hz | 105.6 × 72.0 × 24mm | 10 | 1 |
| `octagon` | regular octagonal prism, finger-jointed on all 24 edges | 269cm³ | B3 −47 cents, 240.3Hz | 110 × 110 × 36mm | 12 | 1 |
| `truncated-cube` | regular truncated cube, finger-jointed on all 36 edges | 845cm³ | C#3 −38 cents, 135.6Hz | 101.4mm every way | 16 | 1 |

In the box and the solids the bird and the chin layer are glued and the rest is
press-fit; the stack is glued throughout.

The body is length × width × depth; the beak runs 18mm past it at the mouth end, the
bird stands 3mm on the top over it, and the chin layer hangs 3mm under it.

- **The box** is six plates finger-jointed on all twelve edges, every finger flush to
  the outside faces. Each edge carries as many fingers as fit at the square flute's
  6mm: 15 cells along the box, 9 across it and 3 up each corner, alternating between
  the two plates that meet there. The corner cubes belong to the side walls. The one
  gap in the pattern is under the beak: the mouth-end wall sends no fingers up into
  the top there, so the beak runs on from solid ply and the duct crosses no seam.
- **The stack** is a bottom layer, six frame layers with the cavity cut out of them,
  and the top, all glued.
- **The octagon** is a regular octagonal prism, 110mm across the flats and 36mm
  tall: two regular octagons, top and bottom, and eight walls 45.6mm wide. It is
  bigger than the box on all three axes. The walls meet each other at 135 degrees
  with six fingers to an edge, and the top and bottom at 90 with seven; the top and
  bottom own the corners. The holes and the window are in the top, the beak runs out
  from the middle of one wall's top edge, and that wall sends no fingers up into the
  top, as the box's end wall does not.
- **The truncated cube** is the regular one: six regular octagons and eight
  equilateral triangles, every edge 42mm, so it is 101.4mm every way. It is not one
  of the five Platonic solids but Archimedean, so
  [Boxes](https://github.com/florianfesti/boxes)' Platonic generator does not make
  it. Octagons meet each other at 90 degrees with six fingers to an edge, and
  triangles at 125.3 with seven, which at 42mm makes every finger exactly 6mm. The
  triangles, the truncated corners, own the corners: each triangle's three edges all
  start and end in its own fingers. One octagon is the top, with the holes and the
  window, and the beak runs out from its edge with the octagon below it.

**How the solids' fingers go together.** Where two plates meet at an angle wider
than 90 degrees, a plate's finger reaches the outside edge, and in its neighbour's
finger it stops short by 3mm × sin of the angle: 2.12mm at 135 degrees, 2.45mm at
125.3. That is the least that keeps the two plates out of each other, and it leaves
the joint closed along a line, with a shallow V-shaped notch outside each finger and
a small void at each corner. The shellac fills both.

Neither has anything to slide, so both are tuned at the window and the holes. The box
had a press-fit end plug to tune with until it was jointed on every edge.

All four carry the same mouthpiece: the top plate runs on past the body as a beak 24mm
wide. One layer, the bird, is glued over it and one, the chin layer, under it, so the
mouthpiece is three plies, 9mm thick, with the duct near its middle. The duct is a
fill engrave in the bird's underside, 10mm wide and about 1mm deep, from the beak's
tip to the window. Only the bird is needed for the sound; the chin layer gives the
lips a thicker mouthpiece and stiffens the beak. The tip, where the lips close, is a
half-ellipse across the beak's full 24mm and 8mm deep, cut the same on all three plies
so they round off as one, and the duct's mouth follows the curve.

## Turn it in 3D

<!-- readme-only -->
All four builds turn in 3D on the [published page](https://gernreich.github.io/box-ocarina/#turn-it-in-3d),
which GitHub does not run: drag to turn it, pull it apart into its parts, or see
through the walls into the cavity.

<!-- page-only <div id="turn3d"></div><script src="viewer.js"></script> -->

## Fingering

Black is covered, white is open. The chart shows the holes as your hands meet them:
beak up, left hand on the left, index fingers on the row nearer the beak.

![Fingering chart for the G major scale from G4 to G5: eight small drawings of the ocarina's hole face, beak up, each showing which of the four holes are covered in black and which are open in white](plans/box-ocarina-G4-fingering.svg)

Hold it with the holes facing away from you and your lips on the beak, thumbs on the
bottom. Each hole is named for the finger that covers it: `L1` is the left index,
`L2` the left middle, `R1` the right index, `R2` the right middle.

| Note | Open | Model | From equal temperament |
|---|---|---|---|
| G4 | none | 392.0Hz | 0 |
| A4 | R2 | 436.9Hz | −12.3c |
| B4 | R1 | 490.4Hz | −12.3c |
| C5 | R2 R1 | 527.0Hz | +12.3c |
| D5 | R2 L1 | 588.3Hz | +2.8c |
| E5 | R2 R1 L1 | 658.0Hz | −3.4c |
| F♯5 | R2 R1 L2 | 735.8Hz | −9.8c |
| G5 | R2 L1 L2 | 780.9Hz | −6.9c |

The other eight fingerings are not in the scale, and three of them are close enough to
use. With every hole open it sounds G♯5, a semitone over the octave.

| Open | Model | Nearest | |
|---|---|---|---|
| L1 | 555.8Hz | C♯5 | +4.3c |
| R1 L1 | 629.0Hz | D♯5 | +18.8c |
| L2 | 646.1Hz | E5 | −35.0c |
| R2 L2 | 674.2Hz | E5 | +38.9c |
| R1 L2 | 710.1Hz | F5 | +28.6c |
| L1 L2 | 756.7Hz | F♯5 | +38.6c |
| R1 L1 L2 | 812.0Hz | G♯5 | −39.1c |
| all four | 834.6Hz | G♯5 | +8.4c |

The octagon and the truncated cube take these fingerings unchanged. A hole's pitch
rises by the same ratio whatever cavity it opens, so each solid plays this major
scale and these cents from its own closed note: B3 −47 cents on the octagon, C♯3
−38 on the truncated cube.

## What the model says

**An ocarina is a vessel, not a tube.** Its cavity is about 100mm across and the
wavelength at G4 is 880mm, so the air inside acts as one spring and the air in each
opening as a mass on it: a Helmholtz resonator. The pitch depends on the cavity's
volume and on how much air each opening lets through, its conductance, and **not at
all on where a hole is**. That is why the holes sit wherever the fingers fall.

Each opening's conductance is its area over its effective length: the 3mm plate it
goes through, plus Rayleigh's end correction for a flanged end, once inside and once
out. Open holes add to the window's in parallel, so every note asks for a sum:

| Hole | Finger | Conductance | Diameter |
|---|---|---|---|
| `R2` | right middle | 0.242 × the window's | 2.95mm |
| `R1` | right index | 0.565 × | 5.23mm |
| `L1` | left index | 1.010 × | 8.04mm |
| `L2` | left middle | 1.716 × | 12.23mm |

The window is 10 × 5mm, with a conductance of 5.12mm. G4 from that window asks for a
cavity of 101.0cm³, which is the 60 × 18 × 93.6mm the box and the stack have.

**No four holes play a perfect scale.** Two holes opened together always add their
conductances, and the scale's steps do not add that way. Here C5 opens the A4 hole and
the B4 hole together, and the two give more than C5 wants. So the model shares the
error out: A4 and B4 sit 12.3 cents flat and C5 12.3 sharp, and nothing else is out by
more than 10. Of the 53 fingering patterns `ocarina_model.py --search` finds, none
does better than 12.3 cents. Breath bends an ocarina by more than that: blow a little
harder for A4 and B4, a little softer for C5.

**The fingering is found, not copied.** John Taylor's English system of 1964 is the
standard four-hole fingering, and no published source found for this gives its hole
sizes. So the pattern here is the tightest one a search over all four conductances
found, and of those, the one with the fewest finger changes up the scale. It may not
match a commercial English pendant's chart note for note.

## Before you cut

**The window's end correction is not measured.** The jet blowing across the window
lengthens it, by an amount found only in simulation so far: a 2009 simulation of an
ocarina ([Kobayashi and others](https://arxiv.org/abs/0911.3567)) put it well above
the textbook figure. It is the one term every other number rests on: the cavity is
sized to the window, and every hole is sized against it. With every hole closed,
blow the cut ocarina and read its pitch. Then

```sh
python3 ocarina_model.py --heard=380   # the pitch you heard, in Hz
```

prints the `--mouth` that explains it, and the command to redraw both builds with
it. A first build that sounds 50 cents flat is not a failure; it is the measurement.

**The solids are a step further from anything measured.** Their window is the G4
alto's, 10 × 5mm, on cavities 2.7 and 8.4 times as big. Big ocarinas are usually
given bigger windows, and a small one on a big cavity may speak weakly or breathily,
the truncated cube most of all. Their pitch does not matter here, but whether they
sound does: cut the octagon before the truncated cube.

**The holes can only go up.** A hole that plays flat can be opened with a file. A hole
that plays sharp cannot be closed up, so file in small steps.

**File the splitting edge by hand.** The window's far edge, the one away from the
beak, is where the air splits, and it needs a clean bevel of about 30° that a laser
cannot cut. File it on the outside face, the face the bird sits on, while the top is
still a flat plate on the bench.

**The duct is a fill engrave**, about 1mm deep and the window's 10mm width. The depth
is a power setting, so find it on scrap first.

**The smallest hole is 2.95mm.** It is right for the note, but at the kerf it is the
one most likely to come out ragged or choked with char. Clear it with a 3mm drill
held in the fingers.

**Cut it in 3mm Baltic birch plywood**, at a 0.13mm kerf. The fingers are the square
flute's size or a little over: each edge is split into equal cells, 6.24mm along the
box, 6.67mm across it and 6.00mm up the corners, with no play, so the fit comes from
the kerf compensation alone. Glue the bird on the top, the chin layer under the
beak, and every layer of the stack. Press-fit
the box, then seal every seam with shellac, inside where you can reach and outside
everywhere: a Helmholtz resonator leaks its note out of any gap.

Run the colours **blue, orange, black**: blue engraves the duct, orange cuts the
window and the holes (and the stack's cavity), and black frees the parts. The parts
are not numbered: the 3D view shows where each one goes.

## Tuning

1. File the splitting edge, then glue the bird on, its end flush with the window's
   near edge so the duct opens straight on to it, and the chin layer under the beak,
   its end against the body.
2. Press the box together, or glue up the stack.
3. With every hole closed, play G4. If it is out by more than breath will bend, run
   `--heard` and note the `--mouth` for the next cut.
4. Open each hole alone, smallest first, and file it up to the model's pitch for
   it: `R2` alone 436.9Hz, `R1` 490.4Hz, `L1` 555.8Hz and `L2` 646.1Hz. The
   scale's other notes are sums of these four and follow from them. The solids
   play lower but by the same steps: on the octagon `R2` alone is 267.9Hz, `R1`
   300.7Hz, `L1` 340.7Hz and `L2` 396.1Hz; on the truncated cube 151.1, 169.6,
   192.2 and 223.4Hz. Tune the solids from whatever the closed note turns out to be:
   each hole alone sounds 1.115, 1.251, 1.418 and 1.648 times it.
5. Seal with shellac once it plays in tune.

## Regenerating it

```sh
python3 ocarina_model.py                       # the design and all sixteen fingerings
python3 ocarina_model.py --mouth=1.2           # with a different window correction
python3 ocarina_model.py --heard=380           # the correction a cut ocarina implies
python3 ocarina_model.py --search              # the fingering patterns, ranked
python3 box_ocarina.py --no-write              # the generator's checks, writing nothing
python3 box_ocarina.py --out=cut-files         # all four builds
python3 page_assets.py                         # viewer.js, turn.html and the two plans
```

`ocarina_model.py` is the Helmholtz model above. For a fingering pattern it finds the
four conductances by a pattern search that minimises the worst note's error in cents.
`--search` asks which pattern, from 400 random starts: it found 53 that give eight
different fingerings, and the one used here heads the list, at 12.3 cents and 11
finger moves up the scale.

`box_ocarina.py` builds all four from what the model asks for and checks them
before writing anything. For the box and the stack:

- the model holds the scale within 12.5 cents
- the cavity is the volume the model asks for
- every hole speaks and a finger covers it: from 2.5 to 13mm across
- every hole opens into the cavity, clear of its walls and the window
- ply left between neighbouring holes
- the window opens into the cavity at its mouth-end wall
- the duct runs from the beak's tip to the window
- the chin layer fills the beak's underside and stops at the body
- in the box, every edge is finger-jointed with as many 6mm cells as fit, and the
  six plates fill every cell of all twelve edges and all eight corners exactly once:
  337 points sampled, each found in one plate
- every sheet fits the bed, and every part is on a sheet

The solids are built by `polyhedron.py`, which takes any convex solid as its faces'
planes and joints every edge at its own angle. For them it checks:

- the model holds the scale, the same as the first set's
- the cavity is small against a quarter wavelength, so it rings as one chamber
- every hole and the window open into the cavity, and ply is left between holes
- each hand's holes lie under that hand, worked out from the geometry
- the duct runs from the beak's tip to the window
- every edge is finger-jointed with as many 6mm cells as fit
- along every joint, no point lies in two plates: 1440 points on the octagon and
  2160 on the truncated cube, through each finger from the outside edge to the
  inner corner
- every joint closes except within a finger of a corner, where the shellac goes
- every sheet fits the bed, and every part is on a sheet

The last two were proved by breaking them: with no step back every sampled point
lies in two plates, and with three times the step the joints open.

It borrows the finger size, the plate class and the sheet packer from
[`square_flute.py`](https://github.com/Gernreich/square-flute/blob/main/square_flute.py),
and the SVG path writer from
[`ribbon_bore.py`](https://github.com/Gernreich/swept-curve/blob/main/ribbon_bore.py)
in swept-curve, so it expects both repositories cloned beside this one.

`.repro` holds the exact command for every generated file here.

The top plate, from outside and to scale, beak on the left:

![The box's top plate from outside, to scale: the beak on the left with the bird's footprint dashed over it and the duct in blue, the small rectangular window at the start of the cavity, whose walls are dotted beneath the plate, and the four round holes, L1 of 8.0mm and L2 of 12.2mm on the upper side, R1 of 5.2mm and R2 of 2.9mm on the lower](plans/box-ocarina-G4-top-plate.svg)

## Every cut sheet

Each picture is a display rendering; click it for the cut file.

**`box`** · finger-jointed on all twelve edges, cavity 60 × 18 × 93.6mm · 8 parts on 1 sheet

[![box, sheet 1 of 1: the parts laid out for the laser, the window and holes in orange, outlines in black, the duct in blue](previews/box/box-ocarina-G4-box-sheet1-cut-files.svg)](cut-files/box/box-ocarina-G4-box-sheet1-cut-files.svg)

**`stack`** · laminated stack, cavity 60 × 18 × 93.6mm · 10 parts on 1 sheet

[![stack, sheet 1 of 1: the parts laid out for the laser, the window, holes and six cavity cut-outs in orange, outlines in black, the duct in blue](previews/stack/box-ocarina-G4-stack-sheet1-cut-files.svg)](cut-files/stack/box-ocarina-G4-stack-sheet1-cut-files.svg)

**`octagon`** · regular octagonal prism, finger-jointed on all 24 edges, cavity 269cm³ · 12 parts on 1 sheet

[![octagon, sheet 1 of 1: two regular octagons with fingers on every edge, one with the beak, the window and four holes in orange; eight rectangular walls with fingers on all four edges, one plain along part of an edge where the beak goes; the bird with the duct in blue and the chin layer](previews/octagon/box-ocarina-octagon-sheet1-cut-files.svg)](cut-files/octagon/box-ocarina-octagon-sheet1-cut-files.svg)

**`truncated-cube`** · regular truncated cube, finger-jointed on all 36 edges, cavity 845cm³ · 16 parts on 1 sheet

[![truncated-cube, sheet 1 of 1: six regular octagons with fingers on every edge, one with the beak, the window and four holes in orange; eight equilateral triangles with fingers on all three edges; the bird with the duct in blue and the chin layer](previews/truncated-cube/box-ocarina-truncated-cube-sheet1-cut-files.svg)](cut-files/truncated-cube/box-ocarina-truncated-cube-sheet1-cut-files.svg)

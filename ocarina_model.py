#!/usr/bin/env python3
"""A four-hole ocarina in G4, English fingering, as a Helmholtz resonator:
how big the cavity is, and how big each finger hole, for a G major scale
from G4 to G5.

    python3 ocarina_model.py                 # the design and all 16 fingerings
    python3 ocarina_model.py --mouth=1.3     # scale the window's end correction
    python3 ocarina_model.py --heard=380     # the --mouth a cut ocarina implies
    python3 ocarina_model.py --search        # the fingering patterns, ranked

AN OCARINA IS A VESSEL, NOT A TUBE. Its cavity is small against the
wavelength -- about 100mm across at G4, whose wavelength is 880mm -- so the
air inside is a spring and the air in each opening is a mass on it. Every
opening is a neck of conductance

    K = A / L',   L' = t + 2 * (8 / 3pi) * r

A its area, t the plate it goes through, r the radius of a circle of that
area, and 8/3pi * r Rayleigh's end correction for a flanged end, once
inside and once out. Open necks add in parallel, so

    f = c / 2pi * sqrt(K_total / V)

and only the SIZE of a hole matters, never where it is. That is why the
holes can go wherever the fingers fall.

The window is always open, so its conductance K_w sets the note with every
hole closed. Opening holes adds to it, and each note needs

    sum of the open holes' K  =  K_w * ((f / f0)^2 - 1)

THE FINGERING. Four holes give sixteen fingerings, and eight of them have to
be a scale. John Taylor's English system (1964) does it; no published source
gives its hole sizes, so the eight here are found by search: the pattern
below, with the four conductances the minimax that pattern allows;
--search reruns the search. The tightest pattern found gets every note within 12.3 cents of equal
temperament, and no pattern the search found does better. What binds is
the bottom of the scale: C5 opens the A4 hole and the B4 hole together, and
the two add up to more conductance than C5 wants, so A4 and B4 sit 12.3
cents flat and C5 12.3 sharp. Breath bends an ocarina by
more than that.

THE WINDOW IS THE UNCALIBRATED PART, as in the square flute. Its end
correction is Rayleigh's, but the jet across it lengthens it, by an amount
measured only in simulation (arXiv:0911.3567 found it larger than the
textbook value). Every hole is sized against K_w, so --mouth scales the
window's correction, and --heard= reads it back off a cut ocarina.
"""
import math
import sys

# --- air, warm from the breath: the square flute's figures -------------------
T_C = 25.0
C = 331.3 * math.sqrt(1 + T_C / 273.15) * 1000   # 346.1 m/s, in mm/s

# --- the tune ----------------------------------------------------------------
A4 = 440.0
F0 = A4 * 2 ** (-2 / 12)                          # G4, 392.0Hz
NOTES = ['G4', 'A4', 'B4', 'C5', 'D5', 'E5', 'F#5', 'G5']
SEMIS = [0, 2, 4, 5, 7, 9, 11, 12]
TARGETS = [F0 * 2 ** (s / 12) for s in SEMIS]
CHROMATIC = ['G', 'G#', 'A', 'A#', 'B', 'C', 'C#', 'D', 'D#', 'E', 'F', 'F#']

# --- the openings (mm) -------------------------------------------------------
WALL = 3.0                         # every opening is in the 3mm top plate
WINDOW_W, WINDOW_L = 10.0, 5.0     # across x along the jet; the duct's width
MOUTH = 1.0                        # scales the window's end correction
RAYLEIGH = 8 / (3 * math.pi)       # 0.849 r, a flanged end

# THE HOLES, smallest first, each named for the finger that covers it. The
# right hand takes the two small holes and plays the bottom of the scale; the
# left the two big ones and the top. Index fingers on the row nearer the
# mouthpiece, middle fingers on the row past it.
HOLES = ['R2', 'R1', 'L1', 'L2']
FINGER = {'R2': 'right middle', 'R1': 'right index',
          'L1': 'left index', 'L2': 'left middle'}
# 1 = open, in the order of HOLES. The tightest of the patterns the search
# found, and of those, the one with fewest finger changes up the scale: 11.
PATTERN = ['0000', '1000', '0100', '1100', '1010', '1110', '1101', '1011']


def conductance(area, t, ec=1.0):
    r = math.sqrt(area / math.pi)
    return area / (t + 2 * RAYLEIGH * r * ec)


def window_k():
    return conductance(WINDOW_W * WINDOW_L, WALL, MOUTH)


def hole_d(k, t=WALL):
    """The diameter whose conductance is k: pi d^2/4 = k (t + RAYLEIGH d),
    a quadratic in d."""
    a, b, c = math.pi / 4, -k * RAYLEIGH, -k * t
    return (-b + math.sqrt(b * b - 4 * a * c)) / (2 * a)


def hole_k(d, t=WALL):
    return conductance(math.pi * d * d / 4, t)


def cents(f, target):
    return 1200 * math.log2(f / target)


def freq(k_open, kw, V):
    return C / (2 * math.pi) * math.sqrt((kw + k_open) / V)


def solve_ratios(pattern=None):
    """Each hole's conductance as a multiple of the window's, the minimax of
    the cents error over the scale for `pattern`, PATTERN by default, from
    the naive start: one hole per step for the first notes."""
    pattern = pattern or PATTERN
    x, e = descend(lambda x: max(abs(600 * math.log2((1 + sum(
        xi for xi, o in zip(x, p) if o == '1')) / (1 + w)))
        for p, w in zip(pattern[1:], WANT[1:])),
        [WANT[1], WANT[2], WANT[4] - WANT[1], WANT[6] - WANT[1] - WANT[2]])
    return x


# each note's open conductance as a multiple of the window's
WANT = [(f / F0) ** 2 - 1 for f in TARGETS]


def descend(worst, x):
    """Minimise worst(x) over four positive conductances: a pattern search
    whose step is halved whenever no move helps. The moves are each
    conductance alone and every pair together: along the axes alone it
    stalls at 13.5 cents, on a ridge where two notes tie and only a diagonal
    lowers both."""
    moves = [{i: 1} for i in range(4)] + [
        {i: 1, j: s} for i in range(4) for j in range(i + 1, 4) for s in (1, -1)]
    e, step = worst(x), 0.05
    while step > 1e-7:
        moved = False
        for mv in moves:
            for s in (step, -step):
                y = [xi + s * mv.get(i, 0) for i, xi in enumerate(x)]
                if min(y) > 0 and worst(y) < e - 1e-12:
                    x, e, moved = y, worst(y), True
        if not moved:
            step /= 2
    return x, e


def changes(pattern):
    """Fingers that move, summed over the steps up the scale."""
    return sum(a != b for p, q in zip(pattern, pattern[1:]) for a, b in zip(p, q))


def search(starts=400, seed=1):
    """Which fingering? Not assumed: from random conductances, descend on the
    scale's worst note when each note may take whichever of the fifteen
    fingerings comes nearest, and keep the pattern each start lands on. Each
    pattern that gives eight different fingerings is then solved exactly by
    solve_ratios. Seeded, so the same run prints the same list."""
    import random
    rnd = random.Random(seed)
    subsets = [format(m, '04b')[::-1] for m in range(1, 16)]

    def opened(x, p):
        return sum(xi for xi, o in zip(x, p) if o == '1')

    def nearest(x):
        return ['0000'] + [min(subsets, key=lambda p: abs(math.log((1 + opened(x, p)) / (1 + w))))
                           for w in WANT[1:]]

    def worst(x):
        return max(abs(600 * math.log2((1 + opened(x, p)) / (1 + w)))
                   for p, w in zip(nearest(x)[1:], WANT[1:]))
    found = set()
    for _ in range(starts):
        x, _ = descend(worst, sorted(rnd.uniform(0.05, 2.0) for _ in range(4)))
        pat = tuple(nearest(x))
        if len(set(pat)) == 8:
            found.add(pat)
    rows = []
    for pat in found:
        x = solve_ratios(list(pat))
        e = max(abs(600 * math.log2((1 + opened(x, p)) / (1 + w)))
                for p, w in zip(pat[1:], WANT[1:]))
        rows.append((round(e, 2), changes(pat), pat, x))
    return sorted(rows)


def design():
    """The cavity volume and the four holes for the current MOUTH: volume
    from the window and G4, holes from the window and the ratios."""
    kw = window_k()
    V = kw * (C / (2 * math.pi * F0)) ** 2
    ratios = solve_ratios()
    diams = [hole_d(r * kw) for r in ratios]
    return V, kw, ratios, diams


def fingering_table(V, kw, diams):
    """All sixteen fingerings, lowest first: the open holes, the note, and
    how far it sits from the nearest equal-tempered note."""
    rows = []
    for m in range(16):
        p = format(m, '04b')[::-1]
        k = sum(hole_k(d) for d, o in zip(diams, p) if o == '1')
        f = freq(k, kw, V)
        s = 12 * math.log2(f / F0)
        n = round(s)
        rows.append((f, p, CHROMATIC[n % 12] + str(4 + (n + 7) // 12),
                     100 * (s - n), p in PATTERN))
    return sorted(rows)


def mouth_for(heard, V):
    """The --mouth that gives an ocarina of volume V a window conductance
    that sounds `heard` with every hole closed: bisection, K_w falls as the
    correction grows."""
    want = V * (2 * math.pi * heard / C) ** 2
    lo, hi = 0.05, 20.0
    for _ in range(80):
        mid = (lo + hi) / 2
        if conductance(WINDOW_W * WINDOW_L, WALL, mid) > want:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def main():
    global MOUTH
    heard = None
    for x in sys.argv[1:]:
        if x.startswith('--mouth='):
            MOUTH = float(x.split('=', 1)[1])
        elif x.startswith('--heard='):
            heard = float(x.split('=', 1)[1])
        elif x == '--search':
            rows = search()
            print(f'{len(rows)} fingering patterns found; the best, worst note first, '
                  f'then fingers moved up the scale:')
            for e, ch, pat, x in rows[:10]:
                print(f'  {e:5.1f}c  {ch:2d} moves  {" ".join(pat)}  '
                      + ' '.join(f'{v:.3f}' for v in x)
                      + ('   <- PATTERN' if list(pat) == PATTERN else ''))
            return 0
        else:
            raise SystemExit(f'error: {x} is not a flag this model reads. '
                             f'It takes --mouth=, --heard= and --search.')
    V, kw, ratios, diams = design()
    print(f'G4 = {F0:.2f}Hz, air at {T_C:g}C ({C / 1000:.1f} m/s), '
          f'window {WINDOW_W:g} x {WINDOW_L:g}mm in {WALL:g}mm, mouth x{MOUTH:g}')
    print(f'window conductance {kw:.3f}mm, cavity {V / 1000:.2f}cm^3')
    print('\nholes, smallest first:')
    for h, r, d in zip(HOLES, ratios, diams):
        print(f'  {h}  {FINGER[h]:13} {r:6.4f} K_w   ø{d:5.2f}mm')
    print('\nthe scale:')
    worst = 0
    for name, target, p in zip(NOTES, TARGETS, PATTERN):
        k = sum(hole_k(d) for d, o in zip(diams, p) if o == '1')
        f = freq(k, kw, V)
        c = cents(f, target)
        worst = max(worst, abs(c))
        opened = ' '.join(h for h, o in zip(HOLES, p) if o == '1') or 'none'
        print(f'  {name:4} {p}  {f:7.2f}Hz  {c:+6.1f}c   open: {opened}')
    print(f'  worst {worst:.1f} cents')
    print('\nall sixteen fingerings:')
    for f, p, note, c, used in fingering_table(V, kw, diams):
        print(f'  {p}  {f:7.2f}Hz  {note:4} {c:+6.1f}c{"   scale" if used else ""}')
    if heard:
        m = mouth_for(heard, V)
        print(f'\nheard {heard:g}Hz with every hole closed: the window acts as if '
              f'--mouth={m:.3f}')
        print(f'  python3 four_hole_ocarinas.py --mouth={m:.3f} --out=cut-files   # recut to that')
    return 0


if __name__ == '__main__':
    sys.exit(main())

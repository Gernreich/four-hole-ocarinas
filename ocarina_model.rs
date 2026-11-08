//! A four-hole ocarina in G4, English fingering, as a Helmholtz resonator:
//! how big the cavity is, and how big each finger hole, for a G major scale
//! from G4 to G5. Rust since 2026-11-07 (ocarina_model.py until then; the
//! move changed no byte of any file).
//!
//!     target/release/ocarina_model                 # the design and all 16 fingerings
//!     target/release/ocarina_model --mouth=1.3     # scale the window's end correction
//!     target/release/ocarina_model --heard=290 --cavity=280   # the --mouth a cut ocarina implies
//!     target/release/ocarina_model --search        # the fingering patterns, ranked
//!
//! AN OCARINA IS A VESSEL, NOT A TUBE. Its cavity is small against the
//! wavelength -- about 140mm across at G4, whose wavelength is 880mm -- so the
//! air inside is a spring and the air in each opening is a mass on it. Every
//! opening is a neck of conductance
//!
//!     K = A / L',   L' = t + 2 * (8 / 3pi) * r
//!
//! A its area, t the plate it goes through, r the radius of a circle of that
//! area, and 8/3pi * r Rayleigh's end correction for a flanged end, once
//! inside and once out. Open necks add in parallel, so
//!
//!     f = c / 2pi * sqrt(K_total / V)
//!
//! and only the SIZE of a hole matters, never where it is. That is why the
//! holes can go wherever the fingers fall.
//!
//! The window is always open, so its conductance K_w sets the note with
//! every hole closed. Opening holes adds to it, and each note needs
//!
//!     sum of the open holes' K  =  K_w * ((f / f0)^2 - 1)
//!
//! THE FINGERING. Four holes give sixteen fingerings, and eight of them have
//! to be a scale. John Taylor's English system (1964) does it; no published
//! source gives its hole sizes, so the eight here are found by search: the
//! pattern below, with the four conductances the minimax that pattern
//! allows; --search reruns the search and lists 53. The pattern is not the
//! tightest. That one, every note within 12.3 cents, needs one hole far
//! bigger than the others, 1.72 K_w, and so caps the window: at a window big
//! enough to suit the larger solids its biggest hole would be 17mm, past a
//! fingertip. The pattern here opens all four holes at the top, which shares
//! the conductance out, so its biggest hole is 1.07 K_w. It gives the window
//! room, starts the scale as the tightest one does, ends with every hole
//! open, and takes ten finger moves up the scale. It costs tuning: every note
//! is within 19.2 cents of equal temperament, B4, C5, F#5 and G5 the
//! furthest. Breath bends an ocarina by that much (the author chose it,
//! 2026-10-18).
//!
//! THE WINDOW is as big as the holes allow: WINDOW_W x WINDOW_L, kept at the
//! 2:1 of the first design, makes the biggest hole 11.8mm, a millimetre under
//! the 12mm that leaves filing room below the 13mm a finger covers. Every
//! build shares it. Big ocarinas are given big windows, since a low note
//! through a small window asks for a slow, weak jet; the window was 10 x 5mm
//! until 2026-10-18.
//!
//! THE WINDOW IS THE UNCALIBRATED PART, as in the square flute. Its end
//! correction is Rayleigh's, but the jet across it lengthens it, by an amount
//! measured only in simulation (arXiv:0911.3567 found it larger than the
//! textbook value). Every hole is sized against K_w, so --mouth scales the
//! window's correction, and --heard= reads it back off a cut ocarina: the
//! pitch it sounds with every hole closed, and --cavity= the volume in cm^3
//! the generator printed for that build. Without --cavity it takes the G4
//! volume design() sizes, which no build has had since 2026-10-18.
use lasermade_tools::py;
use std::f64::consts::PI;
use std::sync::atomic::{AtomicU64, Ordering};

// --- air, warm from the breath: the square flute's figures -------------------
pub const T_C: f64 = 25.0;
/// 346.1 m/s, in mm/s
pub fn c() -> f64 {
    331.3 * (1.0 + T_C / 273.15).sqrt() * 1000.0
}

// --- the tune ----------------------------------------------------------------
pub const A4: f64 = 440.0;
/// G4, 392.0Hz
pub fn f0() -> f64 {
    A4 * py::powf(2.0, -2.0 / 12.0)
}
pub const NOTES: [&str; 8] = ["G4", "A4", "B4", "C5", "D5", "E5", "F#5", "G5"];
pub const SEMIS: [i32; 8] = [0, 2, 4, 5, 7, 9, 11, 12];
pub fn targets() -> Vec<f64> {
    SEMIS.iter().map(|&s| f0() * py::powf(2.0, s as f64 / 12.0)).collect()
}
pub const CHROMATIC: [&str; 12] = ["G", "G#", "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#"];

// --- the openings (mm) -------------------------------------------------------
pub const WALL: f64 = 3.0; // every opening is in the 3mm top plate
pub const WINDOW_W: f64 = 14.0; // across, the duct's width
pub const WINDOW_L: f64 = 7.0; // along the jet
pub fn rayleigh() -> f64 {
    8.0 / (3.0 * PI) // 0.849 r, a flanged end
}

static MOUTH: AtomicU64 = AtomicU64::new(0x3ff0000000000000); // 1.0
/// Scales the window's end correction; --mouth= sets it.
pub fn mouth() -> f64 {
    f64::from_bits(MOUTH.load(Ordering::Relaxed))
}
pub fn set_mouth(m: f64) {
    MOUTH.store(m.to_bits(), Ordering::Relaxed);
}

// THE HOLES, each named for the finger that covers it. The right hand takes
// the two small holes and plays the bottom of the scale; the left the two big
// ones and the top. Index fingers on the row nearer the mouthpiece, middle
// fingers on the row past it.
pub const HOLES: [&str; 4] = ["R2", "R1", "L1", "L2"];
pub fn finger(h: &str) -> &'static str {
    match h {
        "R2" => "right middle",
        "R1" => "right index",
        "L1" => "left index",
        _ => "left middle",
    }
}
// 1 = open, in the order of HOLES. Not the tightest the search finds (that is
// 0000 1000 0100 1100 1010 1110 1101 1011, 12.3 cents, the pattern until
// 2026-10-18) but the one whose biggest hole lets the window grow most: see
// THE FINGERING above.
pub const PATTERN: [&str; 8] = ["0000", "1000", "0100", "1100", "1001", "1101", "0111", "1111"];

pub fn conductance(area: f64, t: f64, ec: f64) -> f64 {
    let r = (area / PI).sqrt();
    area / (t + 2.0 * rayleigh() * r * ec)
}

pub fn window_k() -> f64 {
    conductance(WINDOW_W * WINDOW_L, WALL, mouth())
}

/// The diameter whose conductance is k: pi d^2/4 = k (t + RAYLEIGH d), a
/// quadratic in d.
pub fn hole_d(k: f64, t: f64) -> f64 {
    let (a, b, c) = (PI / 4.0, -k * rayleigh(), -k * t);
    (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)
}

pub fn hole_k(d: f64, t: f64) -> f64 {
    conductance(PI * d * d / 4.0, t, 1.0)
}

pub fn cents(f: f64, target: f64) -> f64 {
    1200.0 * py::log2(f / target)
}

pub fn freq(k_open: f64, kw: f64, v: f64) -> f64 {
    c() / (2.0 * PI) * ((kw + k_open) / v).sqrt()
}

/// each note's open conductance as a multiple of the window's
pub fn want() -> Vec<f64> {
    targets().iter().map(|f| py::powf(f / f0(), 2.0) - 1.0).collect()
}

/// sum(x for each hole open in p), from 0 as Python's sum() starts
fn opened(x: &[f64], p: &str) -> f64 {
    let mut s = 0.0;
    for (xi, o) in x.iter().zip(p.chars()) {
        if o == '1' {
            s += xi;
        }
    }
    s
}

/// The worst note, in cents of the half-interval measure the search uses,
/// for conductances x and a pattern.
fn worst_of(x: &[f64], pattern: &[String], w: &[f64]) -> f64 {
    let mut m: Option<f64> = None;
    for (p, wi) in pattern[1..].iter().zip(&w[1..]) {
        let v = (600.0 * py::log2((1.0 + opened(x, p)) / (1.0 + wi))).abs();
        m = Some(m.map_or(v, |a| py::max(a, v)));
    }
    m.unwrap()
}

/// Each hole's conductance as a multiple of the window's, the minimax of the
/// cents error over the scale for `pattern`, PATTERN by default, from the
/// naive start: one hole per step for the first notes.
pub fn solve_ratios(pattern: Option<&[String]>) -> Vec<f64> {
    let def: Vec<String> = PATTERN.iter().map(|s| s.to_string()).collect();
    let pattern = pattern.unwrap_or(&def);
    let w = want();
    let (x, _) = descend(|x| worst_of(x, pattern, &w), vec![w[1], w[2], w[4] - w[1], w[6] - w[1] - w[2]]);
    x
}

/// Minimise worst(x) over four positive conductances: a pattern search whose
/// step is halved whenever no move helps. The moves are each conductance
/// alone and every pair together: along the axes alone it stalls at 13.5
/// cents, on a ridge where two notes tie and only a diagonal lowers both.
pub fn descend(worst: impl Fn(&[f64]) -> f64, mut x: Vec<f64>) -> (Vec<f64>, f64) {
    let mut moves: Vec<[f64; 4]> = (0..4)
        .map(|i| {
            let mut m = [0.0; 4];
            m[i] = 1.0;
            m
        })
        .collect();
    for i in 0..4 {
        for j in i + 1..4 {
            for s in [1.0, -1.0] {
                let mut m = [0.0; 4];
                m[i] = 1.0;
                m[j] = s;
                moves.push(m);
            }
        }
    }
    // which entries a move names: s * mv.get(i, 0), where 0 is an int and
    // s * 0 is 0.0 or -0.0, added to x the same either way
    let (mut e, mut step) = (worst(&x), 0.05);
    while step > 1e-7 {
        let mut moved = false;
        for mv in &moves {
            for s in [step, -step] {
                let y: Vec<f64> = x.iter().enumerate().map(|(i, xi)| xi + s * mv[i]).collect();
                if y.iter().copied().fold(f64::INFINITY, py::min) > 0.0 && worst(&y) < e - 1e-12 {
                    e = worst(&y);
                    x = y;
                    moved = true;
                }
            }
        }
        if !moved {
            step /= 2.0;
        }
    }
    (x, e)
}

/// Fingers that move, summed over the steps up the scale.
pub fn changes(pattern: &[String]) -> usize {
    pattern.windows(2).map(|w| w[0].chars().zip(w[1].chars()).filter(|(a, b)| a != b).count()).sum()
}

/// format(m, '04b')[::-1]: hole i open when bit i of m is set
fn bits(m: usize) -> String {
    format!("{m:04b}").chars().rev().collect()
}

/// CPython's random.Random: MT19937, seeded and drawn as Python does, so
/// --search lists what the Python listed.
struct Mt {
    s: [u32; 624],
    i: usize,
}

impl Mt {
    fn new(seed: u32) -> Mt {
        let mut m = Mt { s: [0; 624], i: 624 };
        m.s[0] = 19650218;
        for i in 1..624 {
            m.s[i] = 1812433253u32.wrapping_mul(m.s[i - 1] ^ (m.s[i - 1] >> 30)).wrapping_add(i as u32);
        }
        // init_by_array with the key [seed]
        let key = [seed];
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..624 {
            m.s[i] = (m.s[i] ^ (m.s[i - 1] ^ (m.s[i - 1] >> 30)).wrapping_mul(1664525)).wrapping_add(key[j]).wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= 624 {
                m.s[0] = m.s[623];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..623 {
            m.s[i] = (m.s[i] ^ (m.s[i - 1] ^ (m.s[i - 1] >> 30)).wrapping_mul(1566083941)).wrapping_sub(i as u32);
            i += 1;
            if i >= 624 {
                m.s[0] = m.s[623];
                i = 1;
            }
        }
        m.s[0] = 0x80000000;
        m
    }
    fn next(&mut self) -> u32 {
        if self.i >= 624 {
            for k in 0..624 {
                let y = (self.s[k] & 0x80000000) | (self.s[(k + 1) % 624] & 0x7fffffff);
                self.s[k] = self.s[(k + 397) % 624] ^ (y >> 1) ^ if y & 1 == 1 { 0x9908b0df } else { 0 };
            }
            self.i = 0;
        }
        let mut y = self.s[self.i];
        self.i += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^ (y >> 18)
    }
    /// random(): 53 bits from two draws
    fn random(&mut self) -> f64 {
        let (a, b) = ((self.next() >> 5) as f64, (self.next() >> 6) as f64);
        (a * 67108864.0 + b) * (1.0 / 9007199254740992.0)
    }
    fn uniform(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.random()
    }
}

/// Which fingering? Not assumed: from random conductances, descend on the
/// scale's worst note when each note may take whichever of the fifteen
/// fingerings comes nearest, and keep the pattern each start lands on. Each
/// pattern that gives eight different fingerings is then solved exactly by
/// solve_ratios. Seeded, so the same run prints the same list.
pub fn search(starts: usize, seed: u32) -> Vec<(f64, usize, Vec<String>, Vec<f64>)> {
    let mut rnd = Mt::new(seed);
    let subsets: Vec<String> = (1..16).map(bits).collect();
    let w = want();
    let nearest = |x: &[f64]| -> Vec<String> {
        let mut out = vec!["0000".to_string()];
        for wi in &w[1..] {
            let mut best: Option<(f64, &String)> = None;
            for p in &subsets {
                let v = ((1.0 + opened(x, p)) / (1.0 + wi)).ln().abs();
                if best.map_or(true, |b| v < b.0) {
                    best = Some((v, p));
                }
            }
            out.push(best.unwrap().1.clone());
        }
        out
    };
    let worst = |x: &[f64]| worst_of(x, &nearest(x), &w);
    let mut found: Vec<Vec<String>> = vec![];
    for _ in 0..starts {
        let mut x0: Vec<f64> = (0..4).map(|_| rnd.uniform(0.05, 2.0)).collect();
        x0.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let (x, _) = descend(worst, x0);
        let pat = nearest(&x);
        let mut u = pat.clone();
        u.sort();
        u.dedup();
        if u.len() == 8 && !found.contains(&pat) {
            found.push(pat);
        }
    }
    let mut rows = vec![];
    for pat in found {
        let x = solve_ratios(Some(&pat));
        let e = worst_of(&x, &pat, &w);
        rows.push((py::round_dp(e, 2), changes(&pat), pat, x));
    }
    rows.sort_by(|a, b| a.partial_cmp(b).unwrap());
    rows
}

/// The cavity volume and the four holes for the current mouth: volume from
/// the window and G4, holes from the window and the ratios.
pub fn design() -> (f64, f64, Vec<f64>, Vec<f64>) {
    let kw = window_k();
    let v = kw * py::powf(c() / (2.0 * PI * f0()), 2.0);
    let ratios = solve_ratios(None);
    let diams = ratios.iter().map(|r| hole_d(r * kw, WALL)).collect();
    (v, kw, ratios, diams)
}

/// The open holes' conductance for a pattern, summed from 0 as Python does.
pub fn open_k(diams: &[f64], p: &str) -> f64 {
    let mut k = 0.0;
    for (d, o) in diams.iter().zip(p.chars()) {
        if o == '1' {
            k += hole_k(*d, WALL);
        }
    }
    k
}

/// All sixteen fingerings, lowest first: the frequency, the open holes, the
/// note, how far it sits from the nearest equal-tempered note, and whether
/// the scale uses it.
pub fn fingering_table(v: f64, kw: f64, diams: &[f64]) -> Vec<(f64, String, String, f64, bool)> {
    let mut rows = vec![];
    for m in 0..16 {
        let p = bits(m);
        let f = freq(open_k(diams, &p), kw, v);
        let s = 12.0 * py::log2(f / f0());
        let n = py::round(s);
        let name = format!("{}{}", CHROMATIC[n.rem_euclid(12) as usize], 4 + (n + 7).div_euclid(12));
        let used = PATTERN.contains(&p.as_str());
        rows.push((f, p, name, 100.0 * (s - n as f64), used));
    }
    rows.sort_by(|a, b| a.partial_cmp(b).unwrap());
    rows
}

/// The --mouth that gives an ocarina of volume V a window conductance that
/// sounds `heard` with every hole closed: bisection, K_w falls as the
/// correction grows.
pub fn mouth_for(heard: f64, v: f64) -> f64 {
    let want = v * py::powf(2.0 * PI * heard / c(), 2.0);
    let (mut lo, mut hi) = (0.05, 20.0);
    for _ in 0..80 {
        let mid = (lo + hi) / 2.0;
        if conductance(WINDOW_W * WINDOW_L, WALL, mid) > want {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) / 2.0
}

pub fn main() -> i32 {
    let (mut heard, mut cavity): (Option<f64>, Option<f64>) = (None, None);
    let num = |x: &str| {
        let t = x.split_once('=').map(|p| p.1).unwrap_or("");
        py::float(t).unwrap_or_else(|| lasermade_tools::die(format!("ValueError: could not convert string to float: {}", py::str_repr(t))))
    };
    for x in std::env::args().skip(1) {
        if x.starts_with("--mouth=") {
            set_mouth(num(&x));
        } else if x.starts_with("--heard=") {
            heard = Some(num(&x));
        } else if x.starts_with("--cavity=") {
            cavity = Some(num(&x) * 1000.0);
        } else if x == "--search" {
            let rows = search(400, 1);
            println!("{} fingering patterns found; the best, worst note first, then fingers moved up the scale:", rows.len());
            for (e, ch, pat, x) in rows.iter().take(10) {
                let is = pat.iter().zip(PATTERN).all(|(a, b)| a == b);
                println!(
                    "  {e:5.1}c  {ch:2} moves  {}  {}{}",
                    pat.join(" "),
                    x.iter().map(|v| format!("{v:.3}")).collect::<Vec<_>>().join(" "),
                    if is { "   <- PATTERN" } else { "" }
                );
            }
            return 0;
        } else {
            lasermade_tools::die(format!("error: {x} is not a flag this model reads. It takes --mouth=, --heard=, --cavity= and --search."));
        }
    }
    let (v, kw, ratios, diams) = design();
    println!(
        "G4 = {:.2}Hz, air at {}C ({:.1} m/s), window {} x {}mm in {}mm, mouth x{}",
        f0(), py::g(T_C), c() / 1000.0, py::g(WINDOW_W), py::g(WINDOW_L), py::g(WALL), py::g(mouth())
    );
    println!("window conductance {kw:.3}mm, cavity {:.2}cm^3", v / 1000.0);
    println!("\nholes:");
    for ((h, r), d) in HOLES.iter().zip(&ratios).zip(&diams) {
        println!("  {h}  {:13} {r:6.4} K_w   ø{d:5.2}mm", finger(h));
    }
    println!("\nthe scale:");
    let mut worst = 0.0f64;
    for ((name, target), p) in NOTES.iter().zip(targets()).zip(PATTERN) {
        let f = freq(open_k(&diams, p), kw, v);
        let c = cents(f, target);
        worst = py::max(worst, c.abs());
        let open: Vec<&str> = HOLES.iter().zip(p.chars()).filter(|(_, o)| *o == '1').map(|(h, _)| *h).collect();
        let opened = if open.is_empty() { "none".to_string() } else { open.join(" ") };
        println!("  {name:4} {p}  {f:7.2}Hz  {c:+6.1}c   open: {opened}");
    }
    println!("  worst {worst:.1} cents");
    println!("\nall sixteen fingerings:");
    for (f, p, note, c, used) in fingering_table(v, kw, &diams) {
        println!("  {p}  {f:7.2}Hz  {note:4} {c:+6.1}c{}", if used { "   scale" } else { "" });
    }
    if let Some(h) = heard.filter(|&h| h != 0.0) {
        let vol = cavity.filter(|&c| c != 0.0).unwrap_or(v);
        let m = mouth_for(h, vol);
        println!(
            "\nheard {}Hz with every hole closed, on a cavity of {:.1}cm^3: the window acts as if --mouth={m:.3}",
            py::g(h), vol / 1000.0
        );
        println!("  target/release/four_hole_ocarinas --mouth={m:.3} --out=cut-files   # recut to that");
    }
    0
}

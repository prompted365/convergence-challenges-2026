//! Torque Normalization Layer — upgraded per Assessment Recommendations #3–#10.
//!
//! Wraps every forensic I/O operation in an ambient-pattern envelope so that
//! attacker-controlled process monitors cannot distinguish our reads from
//! normal device housekeeping.
//!
//! Changes from `0/torque.rs`:
//!   • Rec #3  — 25-probe equilibrium with trimmed mean (drop top/bottom 10%)
//!   • Rec #4  — EMA re-sampling every 50 ops (α = 0.3)
//!   • Rec #5  — /dev/urandom entropy XOR'd into jitter PRNG seed
//!   • Rec #6  — Right-skewed exponential jitter instead of uniform
//!   • Rec #7  — Shuffled ambient-read rotation with 10 % skip probability
//!   • Rec #8  — Variable pre/post ambient padding (1-3 pre, 0-2 post)
//!   • Rec #9  — Per-path z-score drift thresholds (ManifestEntry.sigma_bytes)
//!   • Rec #10 — All filesystem access through `&dyn Fs` (io.rs trait)

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::io::Fs;

// ─── Constants ──────────────────────────────────────────────────────────────

/// Paths sampled during equilibrium measurement and ambient padding.
const PROBE_PATHS: &[&str] = &[
    "/proc/stat",
    "/proc/meminfo",
    "/proc/loadavg",
    "/proc/uptime",
    "/proc/vmstat",
    "/proc/diskstats",
    "/proc/net/dev",
    "/proc/net/tcp",
    "/proc/net/udp",
    "/proc/interrupts",
    "/proc/softirqs",
    "/proc/schedstat",
    "/proc/self/status",
    "/proc/self/stat",
    "/proc/self/io",
    "/proc/self/maps",
    "/proc/self/fd",
    "/proc/zoneinfo",
    "/proc/pagetypeinfo",
    "/proc/buddyinfo",
    "/proc/slabinfo",
    "/proc/filesystems",
    "/proc/mounts",
    "/proc/swaps",
    "/proc/version",
];

const EQUILIBRIUM_PROBES: usize = 25;
const TRIM_FRACTION: f64 = 0.10; // drop top/bottom 10 %
const EMA_ALPHA: f64 = 0.3;
const RESAMPLE_INTERVAL: u64 = 50;
const AMBIENT_READ_BYTES: usize = 512;
const SKIP_PROBABILITY_PCT: u64 = 10; // 10 % chance to skip an ambient read
#[cfg(test)]
const DEFAULT_DRIFT_THRESHOLD_PCT: f64 = 2.0;
const Z_SCORE_THRESHOLD: f64 = 2.0;

// ─── PRNG (xorshift64*) ────────────────────────────────────────────────────

/// Lightweight non-cryptographic PRNG.  Seeded with /dev/urandom entropy
/// (Rec #5) XOR'd with a monotonic timestamp fallback.
pub(crate) struct Rng {
    state: u64,
}

impl Rng {
    fn new(fs: &dyn Fs) -> Self {
        let ts = Instant::now().elapsed().as_nanos() as u64 | 1;
        let entropy = fs
            .read_bytes(Path::new("/dev/urandom"), 8)
            .ok()
            .and_then(|b| {
                if b.len() >= 8 {
                    Some(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
                } else {
                    None
                }
            })
            .unwrap_or(0xCAFE_BABE_DEAD_BEEFu64);
        Self {
            state: (ts ^ entropy) | 1,
        }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform u64 in `[0, bound)`.
    fn next_bound(&mut self, bound: u64) -> u64 {
        if bound == 0 {
            return 0;
        }
        self.next() % bound
    }

    /// Uniform f64 in `[0.0, 1.0)`.
    fn next_f64(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

// ─── Equilibrium ────────────────────────────────────────────────────────────

/// Ambient timing noise-floor measured from `/proc/*` reads.
///
/// Rec #3 — 25 probes, trimmed mean (drop top/bottom 10 %).
/// Rec #4 — EMA re-sampling every `RESAMPLE_INTERVAL` ops.
#[derive(Debug, Clone)]
pub struct Equilibrium {
    pub mean_interval_ms: u64,
    pub jitter_ms: u64,
    pub ambient_read_size: usize,
    pub sampled_at: Instant,
}

impl Equilibrium {
    /// Sample the ambient noise floor.
    pub fn sample(fs: &dyn Fs) -> Self {
        let mut intervals: Vec<u64> = Vec::with_capacity(EQUILIBRIUM_PROBES);
        for i in 0..EQUILIBRIUM_PROBES {
            let path = Path::new(PROBE_PATHS[i % PROBE_PATHS.len()]);
            let t0 = Instant::now();
            let _ = fs.read_bytes(path, AMBIENT_READ_BYTES);
            let elapsed = t0.elapsed().as_millis() as u64;
            intervals.push(elapsed.max(1));
        }
        intervals.sort_unstable();
        let trim_count = ((intervals.len() as f64) * TRIM_FRACTION).round() as usize;
        let trimmed = &intervals[trim_count..intervals.len().saturating_sub(trim_count)];
        let mean = if trimmed.is_empty() {
            5
        } else {
            trimmed.iter().sum::<u64>() / trimmed.len() as u64
        };
        let jitter = trimmed.last().copied().unwrap_or(2).saturating_sub(mean).max(1);
        Self {
            mean_interval_ms: mean.max(1),
            jitter_ms: jitter.max(1),
            ambient_read_size: AMBIENT_READ_BYTES,
            sampled_at: Instant::now(),
        }
    }

    /// Apply EMA update from a fresh sample (Rec #4).
    pub fn ema_update(&mut self, fresh: &Equilibrium) {
        self.mean_interval_ms = ((EMA_ALPHA * fresh.mean_interval_ms as f64)
            + ((1.0 - EMA_ALPHA) * self.mean_interval_ms as f64))
            as u64;
        self.jitter_ms = ((EMA_ALPHA * fresh.jitter_ms as f64)
            + ((1.0 - EMA_ALPHA) * self.jitter_ms as f64))
            as u64;
        self.sampled_at = Instant::now();
    }

    /// Compute a right-skewed jitter delay (Rec #6).
    /// Uses exponential distribution: `base + (-ln(U) * jitter)`, capped at
    /// `mean + 3 * jitter`.
    pub(crate) fn next_delay(&self, rng: &mut Rng) -> Duration {
        let u = rng.next_f64().max(1e-10); // avoid ln(0)
        let exp_sample = -u.ln() * self.jitter_ms as f64;
        let delay_ms = (self.mean_interval_ms as f64 + exp_sample)
            .min((self.mean_interval_ms + 3 * self.jitter_ms) as f64);
        Duration::from_millis(delay_ms.max(0.0) as u64)
    }
}

// ─── Drift envelope ─────────────────────────────────────────────────────────

/// Monotonically increasing timing shift so no two operations have identical
/// inter-arrival profiles.
#[derive(Debug, Clone)]
pub struct DriftEnvelope {
    pub cumulative_drift_ms: u64,
    pub max_drift_per_op_ms: u64,
    pub op_count: u64,
}

impl DriftEnvelope {
    pub fn new(max_per_op: u64) -> Self {
        Self {
            cumulative_drift_ms: 0,
            max_drift_per_op_ms: max_per_op,
            op_count: 0,
        }
    }

    pub fn advance(&mut self) {
        self.op_count += 1;
        let inc = ((self.op_count as f64).ln() * 1.0).ceil() as u64;
        let capped = inc.min(self.max_drift_per_op_ms);
        self.cumulative_drift_ms = self.cumulative_drift_ms.saturating_add(capped);
    }

    pub fn current(&self) -> Duration {
        Duration::from_millis(self.cumulative_drift_ms)
    }
}

// ─── Ambient padding helpers ────────────────────────────────────────────────

/// Perform one ambient read from a random probe path (Rec #7).
/// Returns `true` if the read was performed, `false` if skipped.
fn pad_ambient(fs: &dyn Fs, rng: &mut Rng, eq: &Equilibrium) -> bool {
    // 10 % probability to skip entirely (Rec #7)
    if rng.next_bound(100) < SKIP_PROBABILITY_PCT {
        return false;
    }
    let idx = rng.next_bound(PROBE_PATHS.len() as u64) as usize;
    let path = Path::new(PROBE_PATHS[idx]);
    let _ = fs.read_bytes(path, eq.ambient_read_size);
    true
}

// ─── TorqueExecutor ─────────────────────────────────────────────────────────

/// Orchestrates every forensic I/O call inside an ambient-pattern envelope.
///
/// Rec #10 — all filesystem access via `&dyn Fs`.
pub struct TorqueExecutor<'a> {
    fs: &'a dyn Fs,
    eq: Equilibrium,
    drift: DriftEnvelope,
    rng: Rng,
    op_counter: u64,
}

impl<'a> TorqueExecutor<'a> {
    pub fn new(fs: &'a dyn Fs) -> Self {
        let eq = Equilibrium::sample(fs);
        let rng = Rng::new(fs);
        Self {
            fs,
            eq,
            drift: DriftEnvelope::new(15),
            rng,
            op_counter: 0,
        }
    }

    /// Execute a forensic operation inside the ambient envelope.
    ///
    /// Rec #8 — variable pre/post padding: 1-3 pre-ambient reads, 0-2 post.
    pub fn exec<F, T>(&mut self, op: F) -> T
    where
        F: FnOnce() -> T,
    {
        // ── Periodic EMA re-sample (Rec #4) ──
        self.op_counter += 1;
        if self.op_counter % RESAMPLE_INTERVAL == 0 {
            let fresh = Equilibrium::sample(self.fs);
            self.eq.ema_update(&fresh);
        }

        // ── Pre-jitter ──
        let delay = self.eq.next_delay(&mut self.rng);
        std::thread::sleep(delay);

        // ── Pre-ambient padding (1-3 reads, Rec #8) ──
        let pre_count = 1 + self.rng.next_bound(3) as usize; // 1..=3
        // Use field-split borrows: fs and eq are disjoint from rng
        let fs = self.fs;
        let eq = &self.eq;
        for _ in 0..pre_count {
            pad_ambient(fs, &mut self.rng, eq);
        }

        // ── The real operation ──
        let result = op();

        // ── Drift advance ──
        self.drift.advance();
        std::thread::sleep(self.drift.current());

        // ── Post-ambient padding (0-2 reads, Rec #8) ──
        let post_count = self.rng.next_bound(3) as usize; // 0..=2
        let fs = self.fs;
        let eq = &self.eq;
        for _ in 0..post_count {
            pad_ambient(fs, &mut self.rng, eq);
        }

        // ── Post-jitter ──
        let delay = self.eq.next_delay(&mut self.rng);
        std::thread::sleep(delay);

        result
    }

    // ── Torqued primitives ──────────────────────────────────────────────────

    /// Read file to string inside envelope (Rec #10 — via Fs trait).
    pub fn torqued_read(&mut self, path: &Path) -> Result<String> {
        let fs = self.fs;
        let p = path.to_path_buf();
        self.exec(move || fs.read_to_string(&p))
    }

    /// Read raw bytes inside envelope.
    pub fn torqued_read_bytes(&mut self, path: &Path) -> Result<Vec<u8>> {
        let fs = self.fs;
        let p = path.to_path_buf();
        self.exec(move || fs.read(&p))
    }

    /// Stat a file inside envelope.
    pub fn torqued_stat(&mut self, path: &Path) -> Result<TorquedStat> {
        let fs = self.fs;
        let p = path.to_path_buf();
        let meta = self.exec(move || fs.metadata(&p))?;
        Ok(TorquedStat {
            path: path.to_path_buf(),
            size_bytes: meta.size_bytes,
            modified: meta.modified,
            is_file: meta.is_file,
            is_dir: meta.is_dir,
        })
    }

    /// SHA-256 hash via Fs trait (Rec #10).
    pub fn torqued_sha256(&mut self, path: &Path) -> Result<String> {
        let fs = self.fs;
        let p = path.to_path_buf();
        let data = self.exec(move || fs.read(&p))?;
        Ok(sha256_hex(&data))
    }

    /// Walk a directory tree through the Fs trait (Rec #10), collecting all
    /// file paths recursively.
    pub fn walk_collect(&self, root: &Path) -> Result<Vec<PathBuf>> {
        let mut out = Vec::new();
        self.walk_inner(root, &mut out)?;
        Ok(out)
    }

    fn walk_inner(&self, dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
        let entries = self.fs.read_dir(dir)?;
        for e in entries {
            if e.meta.is_dir {
                self.walk_inner(&e.path, out)?;
            } else if e.meta.is_file {
                out.push(e.path);
            }
        }
        Ok(())
    }

    /// Walk + per-file callback inside the torque envelope.
    pub fn torqued_find<F>(&mut self, root: &Path, mut per_file: F) -> Result<()>
    where
        F: FnMut(&Path, &[u8]),
    {
        let paths = self.walk_collect(root)?;
        for path in paths {
            let fs = self.fs;
            let p = path.clone();
            let data = self.exec(move || fs.read(&p));
            if let Ok(bytes) = data {
                per_file(&path, &bytes);
            }
        }
        Ok(())
    }

    /// Compute recursive directory size through Fs trait (Rec #10).
    pub fn cumulative_size(&self, root: &Path) -> u64 {
        cumulative_size_fs(self.fs, root)
    }

    /// Human-readable summary of the torque envelope state for audit.
    pub fn envelope_summary(&self) -> String {
        format!(
            "[torque] ops={} drift_ms={} eq_mean_ms={} eq_jitter_ms={}",
            self.drift.op_count,
            self.drift.cumulative_drift_ms,
            self.eq.mean_interval_ms,
            self.eq.jitter_ms,
        )
    }
}

/// Stat wrapper returned by `torqued_stat`.
#[derive(Debug, Clone)]
pub struct TorquedStat {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub modified: Option<u64>,
    pub is_file: bool,
    pub is_dir: bool,
}

// ─── Recursive size via Fs trait (Rec #10) ──────────────────────────────────

pub fn cumulative_size_fs(fs: &dyn Fs, root: &Path) -> u64 {
    let entries = match fs.read_dir(root) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    let mut total: u64 = 0;
    for e in entries {
        if e.meta.is_file {
            total += e.meta.size_bytes;
        } else if e.meta.is_dir {
            total += cumulative_size_fs(fs, &e.path);
        }
    }
    total
}

// ─── SHA-256 (pure Rust, no openssl dep) ────────────────────────────────────

fn sha256_hex(data: &[u8]) -> String {
    // Minimal SHA-256 — K constants + compression.
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    // Padding
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    // Process 512-bit blocks
    for block in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[4 * i],
                block[4 * i + 1],
                block[4 * i + 2],
                block[4 * i + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    h.iter().map(|v| format!("{:08x}", v)).collect()
}

// ─── Manifest baseline (Rec #9 — per-path z-score) ─────────────────────────

/// Single manifest entry with optional statistical sigma (Rec #9).
#[derive(Debug, Clone)]
pub struct ManifestEntry {
    pub path: PathBuf,
    pub expected_size: u64,
    /// Optional historical standard deviation in bytes.
    /// When present, drift is evaluated as a z-score; when absent, falls back
    /// to the uniform percentage threshold.
    pub sigma_bytes: Option<f64>,
}

/// Per-path drift result.
#[derive(Debug, Clone)]
pub struct DriftFinding {
    pub path: PathBuf,
    pub expected: u64,
    pub actual: u64,
    pub delta_pct: f64,
    /// z-score if sigma was available for this entry.
    pub z_score: Option<f64>,
    pub flagged: bool,
}

/// Aggregate manifest from a known-good baseline.
#[derive(Debug, Clone)]
pub struct ManifestBaseline {
    pub version: String,
    pub entries: Vec<ManifestEntry>,
}

impl ManifestBaseline {
    /// Evaluate drift against a live filesystem (Rec #9 + #10).
    ///
    /// For entries with `sigma_bytes`, flags when `|delta| / sigma > Z_SCORE_THRESHOLD`.
    /// Otherwise falls back to `threshold_pct` uniform comparison.
    pub fn check_drift(
        &self,
        fs: &dyn Fs,
        threshold_pct: f64,
    ) -> Vec<DriftFinding> {
        self.entries
            .iter()
            .map(|entry| {
                let actual = cumulative_size_fs(fs, &entry.path);
                let delta = actual as f64 - entry.expected_size as f64;
                let delta_pct = if entry.expected_size > 0 {
                    (delta.abs() / entry.expected_size as f64) * 100.0
                } else if actual > 0 {
                    100.0
                } else {
                    0.0
                };

                let (z_score, flagged) = match entry.sigma_bytes {
                    Some(sigma) if sigma > 0.0 => {
                        let z = delta.abs() / sigma;
                        (Some(z), z > Z_SCORE_THRESHOLD)
                    }
                    _ => (None, delta_pct > threshold_pct),
                };

                DriftFinding {
                    path: entry.path.clone(),
                    expected: entry.expected_size,
                    actual,
                    delta_pct,
                    z_score,
                    flagged,
                }
            })
            .collect()
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::MockFs;

    #[test]
    fn sha256_empty() {
        let hash = sha256_hex(b"");
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn sha256_hello() {
        let hash = sha256_hex(b"hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn rng_produces_varied_output() {
        let fs = MockFs::new();
        let mut rng = Rng::new(&fs);
        let a = rng.next();
        let b = rng.next();
        let c = rng.next();
        assert!(a != b || b != c, "PRNG should produce varied output");
    }

    #[test]
    fn rng_next_bound() {
        let fs = MockFs::new();
        let mut rng = Rng::new(&fs);
        for _ in 0..100 {
            let v = rng.next_bound(10);
            assert!(v < 10);
        }
    }

    #[test]
    fn equilibrium_sample_mock() {
        let mut fs = MockFs::new();
        // Insert a few probe paths so sample doesn't error out entirely
        fs.insert_file("/proc/stat", b"cpu  1234 5678".to_vec());
        fs.insert_file("/proc/meminfo", b"MemTotal: 8192 kB".to_vec());
        let eq = Equilibrium::sample(&fs);
        assert!(eq.mean_interval_ms >= 1);
        assert!(eq.jitter_ms >= 1);
    }

    #[test]
    fn drift_monotonic() {
        let mut d = DriftEnvelope::new(10);
        let mut prev = 0u64;
        for _ in 0..20 {
            d.advance();
            assert!(d.cumulative_drift_ms >= prev);
            prev = d.cumulative_drift_ms;
        }
    }

    #[test]
    fn drift_capped_per_op() {
        let mut d = DriftEnvelope::new(5);
        let before = d.cumulative_drift_ms;
        d.advance();
        let inc = d.cumulative_drift_ms - before;
        assert!(inc <= 5);
    }

    #[test]
    fn ema_update_moves_toward_fresh() {
        let mut old = Equilibrium {
            mean_interval_ms: 100,
            jitter_ms: 10,
            ambient_read_size: 512,
            sampled_at: Instant::now(),
        };
        let fresh = Equilibrium {
            mean_interval_ms: 200,
            jitter_ms: 20,
            ambient_read_size: 512,
            sampled_at: Instant::now(),
        };
        old.ema_update(&fresh);
        // After EMA with α=0.3: 0.3*200 + 0.7*100 = 130
        assert_eq!(old.mean_interval_ms, 130);
        assert_eq!(old.jitter_ms, 13); // 0.3*20 + 0.7*10 = 13
    }

    #[test]
    fn cumulative_size_mock() {
        let mut fs = MockFs::new();
        fs.insert_file("/data/a.txt", vec![0u8; 100]);
        fs.insert_file("/data/b.txt", vec![0u8; 200]);
        fs.insert_file("/data/sub/c.txt", vec![0u8; 50]);
        fs.insert_dir("/data/sub");
        let total = cumulative_size_fs(&fs, Path::new("/data"));
        assert_eq!(total, 350);
    }

    #[test]
    fn cumulative_size_missing() {
        let fs = MockFs::new();
        assert_eq!(cumulative_size_fs(&fs, Path::new("/nonexistent")), 0);
    }

    #[test]
    fn manifest_drift_uniform_threshold() {
        let mut fs = MockFs::new();
        fs.insert_file("/app/bin/a", vec![0u8; 1000]);
        fs.insert_dir("/app/bin");

        let baseline = ManifestBaseline {
            version: "1.0".into(),
            entries: vec![ManifestEntry {
                path: PathBuf::from("/app/bin"),
                expected_size: 1000,
                sigma_bytes: None,
            }],
        };
        let results = baseline.check_drift(&fs, DEFAULT_DRIFT_THRESHOLD_PCT);
        assert_eq!(results.len(), 1);
        assert!(!results[0].flagged); // exactly matches → no drift
        assert!(results[0].z_score.is_none());
    }

    #[test]
    fn manifest_drift_z_score_flag() {
        let mut fs = MockFs::new();
        // Actual = 1500, expected = 1000, sigma = 100 → z = 5.0 → flagged
        fs.insert_file("/app/lib/x", vec![0u8; 1500]);
        fs.insert_dir("/app/lib");

        let baseline = ManifestBaseline {
            version: "1.0".into(),
            entries: vec![ManifestEntry {
                path: PathBuf::from("/app/lib"),
                expected_size: 1000,
                sigma_bytes: Some(100.0),
            }],
        };
        let results = baseline.check_drift(&fs, DEFAULT_DRIFT_THRESHOLD_PCT);
        assert_eq!(results.len(), 1);
        assert!(results[0].flagged);
        let z = results[0].z_score.unwrap();
        assert!((z - 5.0).abs() < 0.01);
    }

    #[test]
    fn manifest_drift_z_score_within() {
        let mut fs = MockFs::new();
        // Actual = 1050, expected = 1000, sigma = 100 → z = 0.5 → NOT flagged
        fs.insert_file("/app/cfg/y", vec![0u8; 1050]);
        fs.insert_dir("/app/cfg");

        let baseline = ManifestBaseline {
            version: "1.0".into(),
            entries: vec![ManifestEntry {
                path: PathBuf::from("/app/cfg"),
                expected_size: 1000,
                sigma_bytes: Some(100.0),
            }],
        };
        let results = baseline.check_drift(&fs, DEFAULT_DRIFT_THRESHOLD_PCT);
        assert!(!results[0].flagged);
    }

    #[test]
    fn manifest_empty() {
        let fs = MockFs::new();
        let baseline = ManifestBaseline {
            version: "test".into(),
            entries: vec![],
        };
        assert!(baseline.check_drift(&fs, 2.0).is_empty());
    }

    #[test]
    fn walk_collect_mock() {
        let mut fs = MockFs::new();
        fs.insert_file("/root/a.txt", b"aaa".to_vec());
        fs.insert_file("/root/sub/b.txt", b"bbb".to_vec());
        fs.insert_dir("/root/sub");

        let te = TorqueExecutor::new(&fs);
        let paths = te.walk_collect(Path::new("/root")).unwrap();
        assert!(paths.len() >= 2);
    }
}

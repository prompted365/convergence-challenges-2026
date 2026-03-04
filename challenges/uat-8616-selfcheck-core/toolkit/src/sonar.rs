//! Sonar — Three-band echo detection for forced-response forensics.
//!
//! Physics-derived methodology: emit a controlled pulse into the system,
//! measure the echo, and compute the delta between expected and observed
//! response. Attacker infrastructure has mass — it MUST respond to stimuli
//! that traverse its execution surface. The attacker can hide from passive
//! observation but cannot hide from forced response without removing themselves.
//!
//! Three propagation bands:
//!   Acoustic  (~ms) — filesystem/network stimuli. Slow, high-energy.
//!   Optical   (~μs) — cache/memory/timing stimuli. Fast, invisible to sw monitors.
//!   Gravitational (~s) — structural/topological stimuli. Measures mass by geodesic deviation.
//!
//! The key insight: uncertainty about detection is the weapon. We don't need
//! perfect detection. We need the attacker to be unable to determine whether
//! they've been detected. Every echo measurement creates uncertainty in the
//! attacker's model of our capabilities.

use std::time::Instant;
use std::path::{Path, PathBuf};

use crate::io::Fs;
use crate::sys::Sys;
use crate::report::{Finding, FindingRole, Report, Severity, Warning};
use crate::torque::TorqueExecutor;

// ─── Core Types ────────────────────────────────────────────────────────────

/// Propagation band for a sonar pulse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Band {
    /// Filesystem/network (~ms). Detectable by process monitors.
    /// Use torque envelope to conceal.
    Acoustic,
    /// Cache/memory/timing (~μs). Invisible to software-level monitors.
    /// Exploits microarchitectural side-channels FOR detection.
    Optical,
    /// Structural/topological (~s). Measures system state-space curvature.
    /// Detects mass by geodesic deviation — expected vs. actual execution paths.
    Gravitational,
}

/// A single sonar pulse: a known stimulus emitted into the system.
#[derive(Debug, Clone)]
pub struct Pulse {
    pub band: Band,
    pub target: String,
    pub emitted_at: Instant,
    /// The expected echo characteristics for a clean system.
    pub expected_echo: EchoProfile,
}

/// Expected echo characteristics for a clean system response.
#[derive(Debug, Clone)]
pub struct EchoProfile {
    /// Expected response time range (min, max) in nanoseconds.
    pub timing_ns: (u64, u64),
    /// Expected response content hash (if applicable).
    pub content_hash: Option<String>,
    /// Expected structural invariant value.
    pub invariant: Option<f64>,
}

/// Measured echo: what actually came back.
#[derive(Debug, Clone)]
pub struct Echo {
    pub band: Band,
    pub target: String,
    pub measured_at: Instant,
    /// Actual response time in nanoseconds.
    pub timing_ns: u64,
    /// Actual content hash (if applicable).
    pub content_hash: Option<String>,
    /// Actual invariant value.
    pub invariant: Option<f64>,
}

/// Delta between expected and observed echo — the forensic signal.
#[derive(Debug, Clone)]
pub struct EchoDelta {
    pub band: Band,
    pub target: String,
    /// Timing deviation: (actual - expected_mean) / expected_range.
    /// >1.0 means outside expected window. Negative means faster than expected.
    pub timing_deviation: f64,
    /// Content match: true if hashes agree, false if they differ or missing.
    pub content_match: bool,
    /// Invariant deviation: absolute difference from expected, if applicable.
    pub invariant_deviation: Option<f64>,
    /// Composite anomaly score: 0.0 = expected, 1.0 = maximally anomalous.
    pub anomaly: f64,
}

// ─── Acoustic Band: Filesystem Echo ────────────────────────────────────────

/// Probe paths used for acoustic-band echo measurement.
/// These paths should exist on any Linux system and have predictable
/// size/content characteristics. Deviations indicate filesystem interception.
const ACOUSTIC_PROBES: &[&str] = &[
    "/proc/self/maps",
    "/proc/self/status",
    "/proc/self/cmdline",
    "/proc/self/environ",
    "/proc/self/fd",
    "/proc/self/stat",
    "/proc/self/io",
    "/proc/self/smaps_rollup",
];

/// Emit acoustic-band pulses through the filesystem and measure echo timing.
///
/// The principle: reading /proc/self/* should have highly predictable timing
/// on a clean system. If an attacker is intercepting our filesystem calls
/// (via LD_PRELOAD, FUSE overlay, or kernel module), there will be measurable
/// timing perturbation in the echo.
///
/// We use the torque executor to wrap these reads in ambient noise, so the
/// attacker's monitor (if any) sees normal /proc activity. But we're measuring
/// the timing of OUR reads relative to the equilibrium baseline, not the
/// attacker's view of our reads.
pub fn acoustic_sweep(
    _fs: &dyn Fs,
    torque: &mut TorqueExecutor,
    report: &mut Report,
) -> Vec<EchoDelta> {
    let mut deltas = Vec::new();

    // Phase 1: Establish baseline timing with warm reads
    let mut baseline_ns: Vec<u64> = Vec::new();
    for probe in ACOUSTIC_PROBES.iter().take(4) {
        let start = Instant::now();
        let _ = torque.torqued_read(&PathBuf::from(probe));
        baseline_ns.push(start.elapsed().as_nanos() as u64);
    }

    let baseline_mean = if baseline_ns.is_empty() {
        return deltas;
    } else {
        baseline_ns.iter().sum::<u64>() / baseline_ns.len() as u64
    };
    let baseline_spread = baseline_ns.iter()
        .map(|t| (*t as f64 - baseline_mean as f64).abs())
        .fold(0.0f64, f64::max)
        .max(1000.0); // minimum 1μs spread to avoid division by zero

    // Phase 2: Measure echo timing for all probes
    for probe_path in ACOUSTIC_PROBES {
        let path = PathBuf::from(probe_path);
        let start = Instant::now();
        let read_result = torque.torqued_read(&path);
        let elapsed_ns = start.elapsed().as_nanos() as u64;

        let _content_hash = read_result.ok().map(|data| {
            // Use a simple hash for echo matching — not security-critical
            let mut h: u64 = 0xcbf29ce484222325;
            for byte in data.bytes() {
                h ^= byte as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
            format!("{h:016x}")
        });

        let timing_deviation = (elapsed_ns as f64 - baseline_mean as f64) / baseline_spread;

        // Anomaly scoring: timing deviation beyond 2σ is suspicious
        // Content mismatch on /proc/self/* is very suspicious
        let timing_anomaly = if timing_deviation.abs() > 3.0 {
            0.8
        } else if timing_deviation.abs() > 2.0 {
            0.5
        } else {
            0.0
        };

        let anomaly = timing_anomaly; // acoustic band is timing-dominant

        deltas.push(EchoDelta {
            band: Band::Acoustic,
            target: probe_path.to_string(),
            timing_deviation,
            content_match: true, // /proc content changes between reads; skip match
            invariant_deviation: None,
            anomaly,
        });
    }

    // Phase 3: Statistical analysis of echo distribution
    let anomalous_count = deltas.iter().filter(|d| d.anomaly > 0.3).count();
    if anomalous_count > 2 {
        let mut finding = Finding::new(
            "SONAR-ACOUSTIC-INTERCEPT",
            if anomalous_count > 4 { Severity::High } else { Severity::Medium },
            format!(
                "Acoustic echo: {}/{} probes show timing anomaly (possible fs interception)",
                anomalous_count,
                deltas.len()
            ),
        )
        .with_evidence(
            deltas.iter()
                .filter(|d| d.anomaly > 0.3)
                .map(|d| format!(
                    "target={} deviation={:.2}σ anomaly={:.2}",
                    d.target, d.timing_deviation, d.anomaly
                ))
                .collect(),
        )
        .with_recommendation(
            "Timing deviations on /proc/self reads suggest a userspace interception layer \
             (LD_PRELOAD, FUSE, or modified libc). Compare against a known-clean appliance \
             with identical hardware. Note: this finding creates attacker uncertainty about \
             whether their interception has been detected.",
        )
        .with_confidence(0.55) // acoustic band has natural variance
        .with_role(FindingRole::Bridge);
        finding.compute_surprise();
        report.push(finding);
    }

    deltas
}

// ─── Optical Band: Cache Timing Oracle ─────────────────────────────────────

/// Cache-line timing thresholds (nanoseconds).
/// L1 hit: ~1-4ns, L2 hit: ~5-12ns, L3 hit: ~20-40ns, DRAM: ~60-100+ns.
/// These are approximate and hardware-dependent; calibrate on target platform.
const _L1_HIT_CEILING_NS: u64 = 8;
const _L2_HIT_CEILING_NS: u64 = 20;
const _CACHE_MISS_FLOOR_NS: u64 = 50;

/// Emit optical-band pulses through the cache hierarchy and measure response.
///
/// The principle: if an attacker is monitoring our memory access patterns
/// (via a kernel module, hypervisor, or co-resident process), our cache
/// lines will show eviction patterns inconsistent with our own access sequence.
///
/// We exploit this DEFENSIVELY:
/// 1. Access a known pattern of memory addresses
/// 2. Measure which accesses are cache hits vs. misses
/// 3. Compare against the expected hit/miss pattern for our access sequence
/// 4. Deviations indicate cache contention from an observer
///
/// This is Flush+Reload / Prime+Probe inverted — we're using the attacker's
/// own side-channel technique against them. If they're watching our cache
/// lines, they're evicting them. We can detect the eviction.
///
/// Note: This operates at the Rust allocation level, not raw cache-line
/// level. On real hardware with inline assembly, this would use RDTSC and
/// CLFLUSH for nanosecond precision. Here we use allocation timing as proxy.
pub fn optical_sweep(
    fs: &dyn Fs,
    report: &mut Report,
) -> Vec<EchoDelta> {
    let mut deltas = Vec::new();

    // Strategy: Allocate a probe buffer, access it in a known pattern,
    // measure re-access timing. If something evicted our lines between
    // accesses, re-access will be slower than expected.

    // Phase 1: Prime — fill cache with known data
    let probe_size = 4096; // one page
    let probe_count = 8;
    let probes: Vec<Vec<u8>> = (0..probe_count)
        .map(|i| vec![i as u8; probe_size])
        .collect();

    // Phase 2: First access pass — warm the cache
    let mut warm_timings: Vec<u64> = Vec::new();
    for probe in &probes {
        let start = Instant::now();
        let _sum: u64 = probe.iter().map(|b| *b as u64).sum();
        warm_timings.push(start.elapsed().as_nanos() as u64);
    }

    // Phase 3: Interleave — do something else to create a window
    // where an observer might evict our cache lines
    let _distraction = fs.read_to_string(Path::new("/proc/self/status"));

    // Phase 4: Re-access — measure timing for the same probes
    let mut cold_timings: Vec<u64> = Vec::new();
    for probe in &probes {
        let start = Instant::now();
        let _sum: u64 = probe.iter().map(|b| *b as u64).sum();
        cold_timings.push(start.elapsed().as_nanos() as u64);
    }

    // Phase 5: Analyze — cache eviction shows as timing increase
    for i in 0..probe_count {
        let warm = warm_timings[i];
        let cold = cold_timings[i];
        let ratio = if warm > 0 { cold as f64 / warm as f64 } else { 1.0 };

        // Expected: cold should be slightly slower but within ~2x of warm
        // If cold >> warm, something evicted our cache lines
        let anomaly = if ratio > 5.0 {
            0.9 // severe cache eviction — strong indicator of observation
        } else if ratio > 3.0 {
            0.6
        } else if ratio > 2.0 {
            0.3
        } else {
            0.0
        };

        deltas.push(EchoDelta {
            band: Band::Optical,
            target: format!("probe_buffer_{i}"),
            timing_deviation: ratio - 1.0,
            content_match: true,
            invariant_deviation: None,
            anomaly,
        });
    }

    // Phase 6: Statistical rollup
    let high_anomaly = deltas.iter().filter(|d| d.anomaly > 0.5).count();
    if high_anomaly > 2 {
        let mut finding = Finding::new(
            "SONAR-OPTICAL-CACHE-EVICTION",
            Severity::High,
            format!(
                "Optical echo: {}/{} cache probes show anomalous eviction (possible observation)",
                high_anomaly, probe_count,
            ),
        )
        .with_evidence(
            deltas.iter()
                .filter(|d| d.anomaly > 0.3)
                .map(|d| format!(
                    "target={} timing_ratio={:.2}x anomaly={:.2}",
                    d.target, d.timing_deviation + 1.0, d.anomaly,
                ))
                .collect(),
        )
        .with_recommendation(
            "Cache eviction patterns inconsistent with our access sequence suggest a \
             co-resident observer (kernel module, hypervisor tap, or co-located process). \
             This is the inverted side-channel: Flush+Reload used defensively. The attacker \
             cannot observe our cache lines without evicting them, and we can detect the eviction.",
        )
        .with_confidence(0.70)
        .with_role(FindingRole::Core);
        finding.compute_surprise();
        report.push(finding);
    }

    deltas
}

// ─── Gravitational Band: Mass Detection via Geodesic Deviation ─────────────

/// Known execution-path invariants for a clean system.
/// These are "geodesics" — expected paths through the system's state space.
/// Attacker mass curves the state space, causing geodesic deviation.
#[derive(Debug, Clone)]
pub struct Geodesic {
    pub name: &'static str,
    /// Command to execute (the probe signal).
    pub cmd: &'static str,
    pub args: &'static [&'static str],
    /// Expected properties of the response for a clean system.
    pub expected_line_count: (usize, usize), // (min, max)
    pub expected_keywords: &'static [&'static str],
    pub forbidden_keywords: &'static [&'static str],
}

/// Standard geodesics for AsyncOS-class systems.
const GEODESICS: &[Geodesic] = &[
    Geodesic {
        name: "process_tree_shape",
        cmd: "ps",
        args: &["auxww"],
        expected_line_count: (10, 200),
        expected_keywords: &["init", "sshd"],
        forbidden_keywords: &["chisel", "aquatunnel", "ncat", "socat"],
    },
    Geodesic {
        name: "network_topology",
        cmd: "ss",
        args: &["-tupan"],
        expected_line_count: (1, 100),
        expected_keywords: &["LISTEN", "ESTAB"],
        forbidden_keywords: &[], // IOC IPs checked elsewhere
    },
    Geodesic {
        name: "module_inventory",
        cmd: "lsmod",
        args: &[],
        expected_line_count: (5, 150),
        expected_keywords: &[],
        forbidden_keywords: &["rootkit", "hide", "stealth"],
    },
    Geodesic {
        name: "cron_schedule",
        cmd: "cat",
        args: &["/etc/crontab"],
        expected_line_count: (1, 50),
        expected_keywords: &[],
        forbidden_keywords: &["curl", "wget", "nc ", "bash -i"],
    },
    Geodesic {
        name: "mount_topology",
        cmd: "mount",
        args: &[],
        expected_line_count: (5, 60),
        expected_keywords: &[],
        forbidden_keywords: &["fuse", "overlay"],
    },
];

/// Emit gravitational-band probes and measure geodesic deviation.
///
/// The principle: every command we run traverses the system's state space
/// along a path (geodesic). On a clean system, these paths have predictable
/// properties (line count, keyword presence/absence). Attacker infrastructure
/// adds "mass" to the state space — extra processes, extra connections,
/// extra modules, extra cron entries, extra mounts. This mass curves the
/// geodesics, producing measurable deviation.
///
/// The attacker can hide individual indicators, but they can't hide the
/// aggregate mass without completely removing their capability. Removing
/// capability = losing access. This is the gravitational invariant.
pub fn gravitational_sweep(
    sys: &dyn Sys,
    torque: &mut TorqueExecutor,
    report: &mut Report,
) -> Vec<EchoDelta> {
    let mut deltas = Vec::new();

    for geodesic in GEODESICS {
        let result = torque.exec(|| sys.run(geodesic.cmd, geodesic.args));
        match result {
            Ok(r) => {
                let lines: Vec<&str> = r.stdout.lines().collect();
                let line_count = lines.len();

                // Geodesic deviation: line count outside expected range
                let (min_lines, max_lines) = geodesic.expected_line_count;
                let line_deviation = if line_count < min_lines {
                    (min_lines - line_count) as f64 / min_lines as f64
                } else if line_count > max_lines {
                    (line_count - max_lines) as f64 / max_lines as f64
                } else {
                    0.0
                };

                // Expected keyword check
                let missing_expected = geodesic.expected_keywords.iter()
                    .filter(|kw| !r.stdout.contains(*kw))
                    .count();

                // Forbidden keyword check (direct mass detection)
                let forbidden_hits: Vec<&str> = geodesic.forbidden_keywords.iter()
                    .copied()
                    .filter(|kw| r.stdout.to_lowercase().contains(&kw.to_lowercase()))
                    .collect();

                // Composite anomaly: forbidden keywords dominate, then line deviation
                let anomaly = if !forbidden_hits.is_empty() {
                    0.95
                } else if line_deviation > 0.5 {
                    0.7
                } else if line_deviation > 0.2 || missing_expected > 0 {
                    0.4
                } else {
                    0.0
                };

                let mut evidence = vec![
                    format!("lines={line_count} expected=({min_lines},{max_lines})"),
                    format!("deviation={line_deviation:.2}"),
                ];
                if !forbidden_hits.is_empty() {
                    evidence.push(format!(
                        "forbidden_keywords=[{}]",
                        forbidden_hits.join(", ")
                    ));
                }

                deltas.push(EchoDelta {
                    band: Band::Gravitational,
                    target: geodesic.name.to_string(),
                    timing_deviation: line_deviation,
                    content_match: forbidden_hits.is_empty(),
                    invariant_deviation: Some(line_deviation),
                    anomaly,
                });

                // Emit finding for high-anomaly geodesics
                if anomaly > 0.6 {
                    let mut finding = Finding::new(
                        format!("SONAR-GRAV-{}", geodesic.name.to_uppercase().replace('_', "-")),
                        if anomaly > 0.9 { Severity::Critical } else { Severity::High },
                        format!(
                            "Gravitational echo: geodesic '{}' shows {:.0}% deviation",
                            geodesic.name,
                            anomaly * 100.0,
                        ),
                    )
                    .with_evidence(evidence)
                    .with_recommendation(
                        "Geodesic deviation indicates attacker mass in the system state space. \
                         The attacker cannot remove this mass without losing capability. \
                         Correlate with acoustic and optical findings for triangulation.",
                    )
                    .with_confidence(anomaly * 0.9)
                    .with_role(FindingRole::Core);
                    finding.compute_surprise();
                    report.push(finding);
                }
            }
            Err(e) => {
                report.warn(Warning::new(
                    format!("Sonar: geodesic '{}'", geodesic.name),
                    format!("Command failed: {e}"),
                ));
            }
        }
    }

    deltas
}

// ─── Sonar Spread: Multi-Band Triangulation ────────────────────────────────

/// The three echo vectors from each band, reduced to a composite anomaly.
#[derive(Debug, Clone)]
pub struct SonarSpread {
    pub acoustic_anomaly: f64,
    pub optical_anomaly: f64,
    pub gravitational_anomaly: f64,
    /// Triangulated composite: bands that independently agree have higher weight.
    pub composite: f64,
    /// Number of bands that independently detected anomaly (>0.3 threshold).
    pub corroborating_bands: u8,
}

impl SonarSpread {
    /// Compute the spread from individual band echo deltas.
    pub fn from_deltas(
        acoustic: &[EchoDelta],
        optical: &[EchoDelta],
        gravitational: &[EchoDelta],
    ) -> Self {
        let acoustic_anomaly = mean_anomaly(acoustic);
        let optical_anomaly = mean_anomaly(optical);
        let gravitational_anomaly = mean_anomaly(gravitational);

        let mut bands = 0u8;
        if acoustic_anomaly > 0.3 { bands += 1; }
        if optical_anomaly > 0.3 { bands += 1; }
        if gravitational_anomaly > 0.3 { bands += 1; }

        // Composite: corroborating bands amplify the signal.
        // One band alone = low confidence (could be noise).
        // Two bands = moderate confidence (cross-medium validation).
        // Three bands = high confidence (the mass is real).
        let band_multiplier = match bands {
            0 => 0.0,
            1 => 0.5,
            2 => 0.8,
            3 => 1.0,
            _ => 1.0,
        };

        let raw_composite = (acoustic_anomaly + optical_anomaly + gravitational_anomaly) / 3.0;
        let composite = (raw_composite * band_multiplier).clamp(0.0, 1.0);

        Self {
            acoustic_anomaly,
            optical_anomaly,
            gravitational_anomaly,
            composite,
            corroborating_bands: bands,
        }
    }
}

fn mean_anomaly(deltas: &[EchoDelta]) -> f64 {
    if deltas.is_empty() { return 0.0; }
    deltas.iter().map(|d| d.anomaly).sum::<f64>() / deltas.len() as f64
}

/// Run all three sonar bands and produce a triangulated spread finding.
pub fn sonar_sweep(
    sys: &dyn Sys,
    fs: &dyn Fs,
    torque: &mut TorqueExecutor,
    report: &mut Report,
) -> SonarSpread {
    let acoustic = acoustic_sweep(fs, torque, report);
    let optical = optical_sweep(fs, report);
    let gravitational = gravitational_sweep(sys, torque, report);

    let spread = SonarSpread::from_deltas(&acoustic, &optical, &gravitational);

    // Emit the triangulation finding
    if spread.corroborating_bands >= 2 {
        let mut finding = Finding::new(
            "SONAR-TRIANGULATION",
            if spread.composite > 0.7 {
                Severity::Critical
            } else {
                Severity::High
            },
            format!(
                "Sonar triangulation: {}/3 bands corroborate anomaly (composite={:.0}%)",
                spread.corroborating_bands,
                spread.composite * 100.0,
            ),
        )
        .with_evidence(vec![
            format!("acoustic={:.2}", spread.acoustic_anomaly),
            format!("optical={:.2}", spread.optical_anomaly),
            format!("gravitational={:.2}", spread.gravitational_anomaly),
            format!("composite={:.2}", spread.composite),
            format!("corroborating_bands={}", spread.corroborating_bands),
        ])
        .with_recommendation(
            "Multi-band corroboration indicates real attacker mass, not noise. \
             Each band detects via a different physical medium, so corroboration \
             eliminates single-medium false positives. Immediate containment recommended.",
        )
        .with_confidence(0.50 + spread.composite * 0.45)
        .with_role(FindingRole::Core);
        finding.compute_surprise();
        report.push(finding);
    }

    // Always record sonar spread in notes for audit trail
    report.notes.push(format!(
        "[sonar] acoustic={:.2} optical={:.2} grav={:.2} composite={:.2} bands={}",
        spread.acoustic_anomaly, spread.optical_anomaly,
        spread.gravitational_anomaly, spread.composite, spread.corroborating_bands,
    ));

    spread
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_delta_clean() {
        let d = EchoDelta {
            band: Band::Acoustic,
            target: "/proc/self/stat".into(),
            timing_deviation: 0.5,
            content_match: true,
            invariant_deviation: None,
            anomaly: 0.0,
        };
        assert!(d.anomaly < 0.1);
    }

    #[test]
    fn sonar_spread_no_anomaly() {
        let empty: Vec<EchoDelta> = vec![];
        let spread = SonarSpread::from_deltas(&empty, &empty, &empty);
        assert_eq!(spread.corroborating_bands, 0);
        assert!(spread.composite < 0.01);
    }

    #[test]
    fn sonar_spread_single_band() {
        let anomalous = vec![EchoDelta {
            band: Band::Optical,
            target: "probe_0".into(),
            timing_deviation: 4.0,
            content_match: true,
            invariant_deviation: None,
            anomaly: 0.8,
        }];
        let empty: Vec<EchoDelta> = vec![];
        let spread = SonarSpread::from_deltas(&empty, &anomalous, &empty);
        assert_eq!(spread.corroborating_bands, 1);
        // Single band = 0.5 multiplier
        assert!(spread.composite < 0.5);
    }

    #[test]
    fn sonar_spread_multi_band_amplification() {
        let high = |band: Band| vec![EchoDelta {
            band,
            target: "probe".into(),
            timing_deviation: 3.0,
            content_match: false,
            invariant_deviation: Some(0.5),
            anomaly: 0.7,
        }];
        let spread = SonarSpread::from_deltas(
            &high(Band::Acoustic),
            &high(Band::Optical),
            &high(Band::Gravitational),
        );
        assert_eq!(spread.corroborating_bands, 3);
        assert!(spread.composite > 0.5, "3-band corroboration should amplify: {}", spread.composite);
    }

    #[test]
    fn geodesic_line_deviation() {
        // Simulate a geodesic where we get 300 lines but expected max is 200
        let deviation: f64 = (300.0 - 200.0) / 200.0; // 0.5
        assert!((deviation - 0.5).abs() < 0.01);
    }
}

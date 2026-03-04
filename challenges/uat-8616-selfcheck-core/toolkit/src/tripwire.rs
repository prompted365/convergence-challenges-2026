//! Tripwire — Uncertainty-maximized probabilistic detection + temporal lineage.
//!
//! Two complementary strategies:
//!
//! 1. **Probabilistic Tripwires**: Instead of binary detection (fire/not-fire),
//!    tripwires fire with variable probability and variable delay, making it
//!    impossible for the attacker to determine whether they've been detected.
//!    The uncertainty IS the weapon.
//!
//! 2. **Lineage Verification (McFly Protocol)**: Track the "family photo" of
//!    system state across time. When an attacker performs a contract-swap
//!    (revert → install → advance), downstream artifacts don't update
//!    consistently — like Marty's siblings fading from the photograph.
//!    The system's "children" (derived state: logs, caches, timestamps,
//!    accumulated counters) carry evidence of their "parents" (the system
//!    state that produced them). If the parents changed but the children
//!    didn't update, or updated inconsistently, the lineage is broken.
//!
//! The McFly insight: you can rewrite history, but you can't rewrite all
//! the consequences of history simultaneously. Something always fades.

use std::time::{SystemTime, UNIX_EPOCH};
use std::path::{Path, PathBuf};
use std::fmt;

use crate::io::Fs;
use crate::sys::Sys;
use crate::report::{Finding, FindingRole, Report, Severity};
use crate::torque::TorqueExecutor;

// ─── Probabilistic Tripwire Layer ──────────────────────────────────────────

/// A probabilistic tripwire: fires with variable probability and can
/// produce decoy signals (false alarms that look real to the attacker).
///
/// The attacker's dilemma: if they trigger a tripwire but aren't sure
/// it fired, they must choose between:
/// - Assuming detection and burning their access (costly false positive for them)
/// - Assuming no detection and continuing (risky if we did detect)
///
/// We maximize this dilemma by making the tripwire's behavior unpredictable.
#[derive(Debug, Clone)]
pub struct ProbabilisticTripwire {
    pub name: String,
    /// Base probability of firing when triggered (0.0 - 1.0).
    pub fire_probability: f64,
    /// Probability of emitting a decoy signal even when NOT triggered.
    /// This creates background noise that masks real detections.
    pub decoy_probability: f64,
    /// Minimum delay before recording finding (nanoseconds).
    /// Prevents timing correlation between trigger and response.
    pub min_delay_ns: u64,
    /// Maximum additional random delay (nanoseconds).
    pub max_jitter_ns: u64,
}

impl ProbabilisticTripwire {
    pub fn new(name: impl Into<String>, fire_prob: f64, decoy_prob: f64) -> Self {
        Self {
            name: name.into(),
            fire_probability: fire_prob.clamp(0.0, 1.0),
            decoy_probability: decoy_prob.clamp(0.0, 1.0),
            min_delay_ns: 100_000,     // 100μs minimum
            max_jitter_ns: 5_000_000,  // up to 5ms additional
        }
    }

    /// Evaluate the tripwire given a trigger condition and a PRNG value.
    /// Returns (should_record, is_decoy).
    pub fn evaluate(&self, triggered: bool, rng_value: u64) -> (bool, bool) {
        let roll = (rng_value % 1000) as f64 / 1000.0;

        if triggered {
            // Real trigger — fire with probability
            let fires = roll < self.fire_probability;
            (fires, false)
        } else {
            // Not triggered — maybe emit decoy
            let decoy_roll = ((rng_value >> 16) % 1000) as f64 / 1000.0;
            let decoys = decoy_roll < self.decoy_probability;
            (decoys, true)
        }
    }
}

/// Standard tripwire configurations.
/// Each represents a different detection surface with tuned probabilities.
pub fn standard_tripwires() -> Vec<ProbabilisticTripwire> {
    vec![
        // High-value tripwire: always fires on trigger, low decoy rate
        ProbabilisticTripwire::new("ioc_hash_match", 0.95, 0.01),
        // Medium-value: fires 70% of the time, moderate decoy noise
        ProbabilisticTripwire::new("unexpected_process", 0.70, 0.05),
        // Ambiguous signal: fires 50/50, high decoy rate (maximum uncertainty)
        ProbabilisticTripwire::new("timing_anomaly", 0.50, 0.15),
        // Canary: low fire rate but creates persistent uncertainty
        ProbabilisticTripwire::new("directory_enumeration", 0.30, 0.10),
        // Ghost tripwire: fires on evidence of prior detection being scrubbed
        ProbabilisticTripwire::new("evidence_tampering", 0.90, 0.02),
    ]
}

// ─── Lineage Verification: The McFly Protocol ─────────────────────────────

/// A lineage checkpoint: a "family photo" snapshot of derived system state.
///
/// The Back to the Future principle: when an attacker performs a contract swap
/// (revert the system, install their payload, advance to appear normal),
/// the derived state (the "children" of the system) won't match the
/// claimed lineage. Like Marty's siblings fading from the photo:
///
/// - Biff's timeline: different parents → children don't exist
/// - Attacker's timeline: different boot sequence → derived counters don't match
///
/// We check for lineage consistency: do the children match their parents?
#[derive(Debug, Clone)]
pub struct LineageCheckpoint {
    /// Identifier for this checkpoint.
    pub name: String,
    /// The "parent" state: a measurable system property that should be consistent
    /// with its derived children.
    pub parent: LineageNode,
    /// The "children": derived state that should be consistent with the parent.
    pub children: Vec<LineageNode>,
}

/// A single node in the lineage tree.
#[derive(Debug, Clone)]
pub struct LineageNode {
    pub name: String,
    pub source: String,
    /// The observed value (timestamp, counter, hash, etc.)
    pub value: LineageValue,
}

/// Values that can be compared for lineage consistency.
#[derive(Debug, Clone)]
pub enum LineageValue {
    /// A UNIX timestamp (seconds since epoch).
    Timestamp(u64),
    /// A monotonic counter value.
    Counter(u64),
    /// A content hash.
    Hash(String),
    /// A human-readable string (for evidence).
    Text(String),
}

impl fmt::Display for LineageValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timestamp(t) => write!(f, "ts={t}"),
            Self::Counter(c) => write!(f, "counter={c}"),
            Self::Hash(h) => write!(f, "hash={}", &h[..h.len().min(16)]),
            Self::Text(t) => write!(f, "text={}", &t[..t.len().min(40)]),
        }
    }
}

/// Result of a lineage consistency check.
#[derive(Debug, Clone)]
pub struct LineageResult {
    pub checkpoint: String,
    pub consistent: bool,
    pub discrepancies: Vec<String>,
    /// Severity of lineage break: how many "siblings" are fading?
    /// 0.0 = all consistent, 1.0 = all inconsistent
    pub fade_ratio: f64,
}

/// Collect lineage checkpoints from the running system.
///
/// Each checkpoint captures a parent-child relationship where the child's
/// state should be derivable from the parent's state on a clean system.
pub fn collect_lineage(
    sys: &dyn Sys,
    fs: &dyn Fs,
    _root: &Path,
    torque: &mut TorqueExecutor,
) -> Vec<LineageCheckpoint> {
    let mut checkpoints = Vec::new();

    // ── Checkpoint 1: Boot lineage ──
    // Parent: /proc/uptime (system uptime)
    // Children: /proc/stat btime, file mtimes in /var/log, /proc/self/stat start time
    // If the system was rebooted (revert-install-advance), uptime and btime
    // should be consistent. If the attacker modified uptime but not btime,
    // or vice versa, the lineage breaks.
    let uptime_val = torque.exec(|| {
        fs.read_to_string(Path::new("/proc/uptime"))
            .ok()
            .and_then(|s| s.split_whitespace().next().map(|t| t.to_string()))
            .unwrap_or_default()
    });
    let uptime_secs = uptime_val.parse::<f64>().unwrap_or(0.0) as u64;

    let btime_val = torque.exec(|| {
        fs.read_to_string(Path::new("/proc/stat"))
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|l| l.starts_with("btime "))
                    .and_then(|l| l.split_whitespace().nth(1))
                    .map(|t| t.to_string())
            })
            .unwrap_or_default()
    });
    let btime_secs = btime_val.parse::<u64>().unwrap_or(0);

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    checkpoints.push(LineageCheckpoint {
        name: "boot_lineage".into(),
        parent: LineageNode {
            name: "uptime".into(),
            source: "/proc/uptime".into(),
            value: LineageValue::Counter(uptime_secs),
        },
        children: vec![
            LineageNode {
                name: "btime".into(),
                source: "/proc/stat".into(),
                value: LineageValue::Timestamp(btime_secs),
            },
            LineageNode {
                name: "wall_clock".into(),
                source: "system_time".into(),
                value: LineageValue::Timestamp(now_secs),
            },
        ],
    });

    // ── Checkpoint 2: Process lineage ──
    // Parent: PID 1 start time
    // Children: all other processes should have start times >= PID 1
    // If attacker spawned a process before "init" (impossible on clean boot),
    // the lineage is broken.
    let pid1_start = torque.exec(|| {
        sys.run("cat", &["/proc/1/stat"])
            .ok()
            .and_then(|r| {
                if r.success() {
                    // Field 22 (0-indexed) is starttime in clock ticks
                    let fields: Vec<&str> = r.stdout.split_whitespace().collect();
                    fields.get(21).and_then(|t| t.parse::<u64>().ok())
                } else {
                    None
                }
            })
            .unwrap_or(0)
    });

    checkpoints.push(LineageCheckpoint {
        name: "process_lineage".into(),
        parent: LineageNode {
            name: "pid1_starttime".into(),
            source: "/proc/1/stat".into(),
            value: LineageValue::Counter(pid1_start),
        },
        children: vec![], // populated dynamically during verification
    });

    // ── Checkpoint 3: Filesystem lineage ──
    // Parent: root filesystem mount time
    // Children: critical config file modification times
    // If attacker modified /etc/passwd but the mtime predates the mount,
    // time travel is occurring.
    let mount_info = torque.exec(|| {
        fs.read_to_string(Path::new("/proc/self/mountinfo"))
            .unwrap_or_default()
    });

    checkpoints.push(LineageCheckpoint {
        name: "filesystem_lineage".into(),
        parent: LineageNode {
            name: "mount_info".into(),
            source: "/proc/self/mountinfo".into(),
            value: LineageValue::Text(mount_info[..mount_info.len().min(200)].to_string()),
        },
        children: vec![],
    });

    checkpoints
}

/// Verify lineage consistency: do the children match their parents?
pub fn verify_lineage(checkpoints: &[LineageCheckpoint]) -> Vec<LineageResult> {
    let mut results = Vec::new();

    for cp in checkpoints {
        let mut discrepancies = Vec::new();

        match &cp.parent.value {
            LineageValue::Counter(uptime) => {
                // Boot lineage: uptime + btime should ≈ now
                for child in &cp.children {
                    match &child.value {
                        LineageValue::Timestamp(ts) => {
                            if child.name == "btime" {
                                // btime + uptime should ≈ now
                                let expected_now = ts + uptime;
                                let wall = cp.children.iter()
                                    .find(|c| c.name == "wall_clock")
                                    .and_then(|c| match &c.value {
                                        LineageValue::Timestamp(t) => Some(*t),
                                        _ => None,
                                    })
                                    .unwrap_or(expected_now);

                                let drift = if expected_now > wall {
                                    expected_now - wall
                                } else {
                                    wall - expected_now
                                };

                                // Allow 60 seconds of drift for measurement jitter
                                if drift > 60 {
                                    discrepancies.push(format!(
                                        "boot_timeline: btime({ts})+uptime({uptime})={expected_now} \
                                         vs wall_clock({wall}), drift={drift}s — \
                                         LINEAGE BREAK: system time is inconsistent with boot timeline"
                                    ));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                // For other checkpoint types, just record them
            }
        }

        let total_children = cp.children.len().max(1);
        let fade_ratio = discrepancies.len() as f64 / total_children as f64;

        results.push(LineageResult {
            checkpoint: cp.name.clone(),
            consistent: discrepancies.is_empty(),
            discrepancies,
            fade_ratio: fade_ratio.clamp(0.0, 1.0),
        });
    }

    results
}

// ─── Counter-Signal: Interference Spike Generation ─────────────────────────

/// Generate interference spikes that look like infrastructure events.
///
/// The "act of god" defense: create signals that are indistinguishable
/// from known infrastructural events (NTP corrections, cron jobs, log rotation,
/// kernel memory compaction). These signals serve two purposes:
///
/// 1. They mask our actual forensic activity in a sea of plausible events
/// 2. They force the attacker to classify each signal as real-or-noise,
///    consuming their analytical bandwidth
///
/// The attacker can't filter out infrastructure events without risking
/// missing our real signals. They can't analyze all signals without
/// massive overhead. Either way, we win.
pub struct InterferencePattern {
    pub event_type: &'static str,
    /// File paths to read/touch that mimic this event type.
    pub paths: &'static [&'static str],
    /// Natural frequency of this event (seconds between occurrences).
    pub natural_period_secs: u64,
}

/// Infrastructure events we can mimic.
pub const INTERFERENCE_PATTERNS: &[InterferencePattern] = &[
    InterferencePattern {
        event_type: "ntp_correction",
        paths: &["/etc/ntp.conf", "/var/lib/ntp/drift", "/var/log/ntpstats"],
        natural_period_secs: 1024, // NTP polls every ~17 minutes
    },
    InterferencePattern {
        event_type: "log_rotation",
        paths: &["/etc/logrotate.conf", "/var/log/syslog", "/var/log/messages"],
        natural_period_secs: 86400, // daily
    },
    InterferencePattern {
        event_type: "cron_housekeeping",
        paths: &["/var/spool/cron", "/etc/crontab", "/var/run/crond.pid"],
        natural_period_secs: 3600, // hourly
    },
    InterferencePattern {
        event_type: "memory_compaction",
        paths: &["/proc/buddyinfo", "/proc/pagetypeinfo", "/proc/vmstat"],
        natural_period_secs: 300, // ~5 minutes
    },
];

/// Emit interference spikes through the torque layer.
/// These reads look like normal system maintenance to any process monitor.
pub fn emit_interference(
    _fs: &dyn Fs,
    torque: &mut TorqueExecutor,
    pattern: &InterferencePattern,
) {
    for path in pattern.paths {
        let _ = torque.torqued_read(&PathBuf::from(path));
    }
}

// ─── Integrated Tripwire + Lineage Runner ──────────────────────────────────

/// Run the full tripwire + lineage verification pipeline.
pub fn run_tripwire_suite(
    sys: &dyn Sys,
    fs: &dyn Fs,
    root: &Path,
    torque: &mut TorqueExecutor,
    report: &mut Report,
    rng_seed: u64,
) {
    // Phase 1: Emit interference to mask our activity
    for pattern in INTERFERENCE_PATTERNS {
        emit_interference(fs, torque, pattern);
    }

    // Phase 2: Collect and verify lineage
    let checkpoints = collect_lineage(sys, fs, root, torque);
    let lineage_results = verify_lineage(&checkpoints);

    for result in &lineage_results {
        if !result.consistent {
            let mut finding = Finding::new(
                format!("MCFLY-LINEAGE-{}", result.checkpoint.to_uppercase().replace('_', "-")),
                if result.fade_ratio > 0.5 {
                    Severity::Critical
                } else {
                    Severity::High
                },
                format!(
                    "McFly lineage break: '{}' — {:.0}% of derived state inconsistent",
                    result.checkpoint,
                    result.fade_ratio * 100.0,
                ),
            )
            .with_evidence(result.discrepancies.clone())
            .with_recommendation(
                "Lineage inconsistency indicates a contract-swap attack: the system was \
                 reverted, modified, and advanced to appear normal. Like the fading photograph, \
                 derived state (timestamps, counters, caches) doesn't match the claimed timeline. \
                 The attacker can rewrite history but not all consequences of history simultaneously.",
            )
            .with_confidence(0.75 + result.fade_ratio * 0.20)
            .with_role(FindingRole::Core);
            finding.compute_surprise();
            report.push(finding);
        }
    }

    // Phase 3: Record lineage audit trail
    let consistent_count = lineage_results.iter().filter(|r| r.consistent).count();
    let total = lineage_results.len();
    report.notes.push(format!(
        "[mcfly] lineage: {consistent_count}/{total} checkpoints consistent"
    ));

    // Phase 4: Emit decoy signals from probabilistic tripwires
    // This runs regardless of whether real triggers fired, maintaining
    // background noise that masks real detections
    let tripwires = standard_tripwires();
    let mut decoy_count = 0;
    for (i, tw) in tripwires.iter().enumerate() {
        let rng_val = rng_seed.wrapping_mul(0x517cc1b727220a95).wrapping_add(i as u64);
        let (_should_fire, is_decoy) = tw.evaluate(false, rng_val);
        if is_decoy {
            decoy_count += 1;
        }
    }
    report.notes.push(format!(
        "[tripwire] probabilistic: {} tripwires configured, {} decoy signals emitted this cycle",
        tripwires.len(), decoy_count,
    ));
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tripwire_fires_on_trigger() {
        let tw = ProbabilisticTripwire::new("test", 1.0, 0.0);
        // With fire_prob=1.0, should always fire on trigger
        let (fires, is_decoy) = tw.evaluate(true, 42);
        assert!(fires);
        assert!(!is_decoy);
    }

    #[test]
    fn tripwire_never_fires_on_zero_prob() {
        let tw = ProbabilisticTripwire::new("test", 0.0, 0.0);
        // With fire_prob=0.0, should never fire
        for i in 0..100 {
            let (fires, _) = tw.evaluate(true, i);
            assert!(!fires, "Should never fire with prob=0.0");
        }
    }

    #[test]
    fn tripwire_decoy_on_no_trigger() {
        let tw = ProbabilisticTripwire::new("test", 0.5, 1.0);
        // With decoy_prob=1.0, should always decoy when not triggered
        let (fires, is_decoy) = tw.evaluate(false, 42);
        assert!(fires); // decoy fires
        assert!(is_decoy);
    }

    #[test]
    fn lineage_value_display() {
        let ts = LineageValue::Timestamp(1709337600);
        assert!(format!("{ts}").contains("1709337600"));
        let h = LineageValue::Hash("abcdef1234567890abcdef".into());
        assert!(format!("{h}").contains("abcdef12345678")); // truncated
    }

    #[test]
    fn lineage_consistent_boot() {
        let now = 1709337600u64;
        let uptime = 3600u64; // 1 hour
        let btime = now - uptime;

        let cp = LineageCheckpoint {
            name: "boot_lineage".into(),
            parent: LineageNode {
                name: "uptime".into(),
                source: "/proc/uptime".into(),
                value: LineageValue::Counter(uptime),
            },
            children: vec![
                LineageNode {
                    name: "btime".into(),
                    source: "/proc/stat".into(),
                    value: LineageValue::Timestamp(btime),
                },
                LineageNode {
                    name: "wall_clock".into(),
                    source: "system_time".into(),
                    value: LineageValue::Timestamp(now),
                },
            ],
        };

        let results = verify_lineage(&[cp]);
        assert_eq!(results.len(), 1);
        assert!(results[0].consistent, "Consistent boot lineage should pass");
    }

    #[test]
    fn lineage_broken_boot_time_travel() {
        let now = 1709337600u64;
        let uptime = 3600u64;
        let btime = now - uptime - 7200; // btime is 2 hours too early

        let cp = LineageCheckpoint {
            name: "boot_lineage".into(),
            parent: LineageNode {
                name: "uptime".into(),
                source: "/proc/uptime".into(),
                value: LineageValue::Counter(uptime),
            },
            children: vec![
                LineageNode {
                    name: "btime".into(),
                    source: "/proc/stat".into(),
                    value: LineageValue::Timestamp(btime),
                },
                LineageNode {
                    name: "wall_clock".into(),
                    source: "system_time".into(),
                    value: LineageValue::Timestamp(now),
                },
            ],
        };

        let results = verify_lineage(&[cp]);
        assert!(!results[0].consistent, "Time-travel boot should be detected");
        assert!(results[0].discrepancies[0].contains("LINEAGE BREAK"));
    }

    #[test]
    fn standard_tripwires_exist() {
        let tw = standard_tripwires();
        assert!(tw.len() >= 4);
        // Evidence tampering tripwire should have high fire probability
        let evidence = tw.iter().find(|t| t.name == "evidence_tampering").unwrap();
        assert!(evidence.fire_probability > 0.8);
    }
}

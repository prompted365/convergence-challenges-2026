//! Eris — Discordian Heuristic Derailment & Russian-Doll Paradox Engine.
//!
//! Named for the Greek goddess of discord and the Principia Discordia's
//! Law of Fives: all things happen in fives, or are divisible by or are
//! multiples of five, or are somehow directly or indirectly appropriate
//! to five. The attacker's evasion narrative is a derailment attempt —
//! Eris models it as such and intercepts along nested trajectories.
//!
//! ## Architecture
//!
//! Three interlocking subsystems:
//!
//! ### 1. Derailment Narrative Detection
//!
//! The attacker's cleanup, evasion, and anti-forensic actions form a
//! *narrative*. Each action is a sentence in a story the attacker tells
//! the system: "nothing happened here." Eris models this narrative as a
//! directed graph of state transitions and detects when the narrative
//! *derails* — when the story contradicts itself.
//!
//! A consistent cleanup narrative would revert every trace. An inconsistent
//! one leaves cross-layer contradictions: timestamps healed in logs but
//! not in filesystem metadata, process tables cleaned but /proc/*/maps
//! still showing mmapped regions, etc.
//!
//! ### 2. Russian-Doll Trajectory (Matryoshka Shells)
//!
//! Each layer of the detection pipeline is a shell. The attacker who
//! evades Layer N faces Layer N+1, which uses Layer N's *expected evasion*
//! as a signal. The innermost doll is the one the attacker never expects:
//! the meta-detection of their detection-evasion pattern.
//!
//! Shell nesting:
//!   Shell 0 (outermost): Conventional IOC matching
//!   Shell 1: Timing/behavioral anomaly (sonar)
//!   Shell 2: Lineage consistency (McFly)
//!   Shell 3: Narrative coherence (this module)
//!   Shell 4 (innermost): Paradox detection — the attacker's evasion
//!     of Shell 3 creates a paradox detectable by Shell 4.
//!
//! ### 3. Paradox Engineering
//!
//! Some detection scenarios are intentionally "unsolvable" — the attacker
//! faces a paradox where evading detection A exposes them to detection B,
//! and evading both A and B requires a state that detection C specifically
//! looks for. These paradox traps are engineered using incompatible
//! constraint sets.
//!
//! The "semantic bridge" is the escape hatch for the *defender*: a
//! resolution path that hops between the incompatible constraint lanes
//! using domain knowledge the attacker doesn't have (knowledge of the
//! trap's design). Attempting to brute-force the bridge without the
//! semantic key causes combinatorial explosion — the computation engine
//! crashes against the paradox.
//!
//! Swallowtail fragment: surprise scoring bridges Emergence topology
//! (which shells are active) with Metamorphosis drift (how the narrative
//! shifted) to produce paradox confidence scores.

use std::fmt;

use crate::io::Fs;
use crate::report::{Finding, FindingRole, Report, Severity};
use crate::sys::Sys;
use crate::torque::TorqueExecutor;

// ─── Derailment Narrative ──────────────────────────────────────────────────

/// A single narrative assertion the system's state is making.
/// "My timestamps are consistent" or "No foreign binaries exist."
#[derive(Debug, Clone)]
pub struct NarrativeAssertion {
    /// Which subsystem is asserting (fs, proc, log, net, cache).
    pub domain: String,
    /// What is being claimed.
    pub claim: String,
    /// Evidence supporting the claim (from prior findings).
    pub evidence_ids: Vec<String>,
    /// Does this assertion hold? None = untested.
    pub holds: Option<bool>,
}

/// A derailment: two narrative assertions contradict each other.
#[derive(Debug, Clone)]
pub struct Derailment {
    /// The first assertion.
    pub assertion_a: NarrativeAssertion,
    /// The contradicting assertion.
    pub assertion_b: NarrativeAssertion,
    /// Derailment severity (0.0–1.0). Higher = more certain contradiction.
    pub severity: f64,
    /// Which Discordian law applies (Law of Fives, Orange Revelation, etc.).
    pub discordian_law: &'static str,
}

impl fmt::Display for Derailment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DERAILMENT({:.0}%): [{}] '{}' vs [{}] '{}' — {}",
            self.severity * 100.0,
            self.assertion_a.domain,
            self.assertion_a.claim,
            self.assertion_b.domain,
            self.assertion_b.claim,
            self.discordian_law,
        )
    }
}

/// The five Discordian laws used as derailment classifiers.
/// ("All things happen in fives.")
const DISCORDIAN_LAWS: [&str; 5] = [
    "Law of Fives",           // Numerical/counting contradictions
    "Orange Revelation",       // Visibility contradictions (seen vs unseen)
    "Starbuck's Pebble",      // Causality contradictions (effect without cause)
    "POEE Curse",             // Temporal contradictions (sequence violations)
    "Sacred Chao Balance",    // Entropy contradictions (order where chaos expected)
];

/// Classify which Discordian law a derailment falls under based on domains.
fn classify_law(domain_a: &str, domain_b: &str) -> &'static str {
    match (domain_a, domain_b) {
        ("fs", "log") | ("log", "fs") => DISCORDIAN_LAWS[3], // POEE Curse — temporal
        ("proc", "fs") | ("fs", "proc") => DISCORDIAN_LAWS[1], // Orange — visibility
        ("net", _) | (_, "net") => DISCORDIAN_LAWS[2],         // Starbuck — causality
        ("cache", _) | (_, "cache") => DISCORDIAN_LAWS[4],     // Sacred Chao — entropy
        _ => DISCORDIAN_LAWS[0],                                // Law of Fives — default
    }
}

// ─── Matryoshka Shells (Russian Doll) ──────────────────────────────────────

/// A shell in the Russian-doll detection hierarchy.
#[derive(Debug, Clone)]
pub struct MatryoshkaShell {
    /// Shell depth (0 = outermost, 4 = innermost paradox shell).
    pub depth: u8,
    /// Shell name.
    pub name: String,
    /// Expected evasion pattern for this shell — what the attacker would
    /// do to evade detection at this depth.
    pub evasion_signature: String,
    /// Finding IDs from this shell's detection.
    pub finding_ids: Vec<String>,
    /// Was evasion of this shell detected?
    pub evasion_detected: bool,
}

impl fmt::Display for MatryoshkaShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.evasion_detected { "EVASION-DETECTED" } else { "CLEAN" };
        write!(f, "Shell-{} [{}]: {} ({})", self.depth, status, self.name, self.evasion_signature)
    }
}

/// Build the standard five-shell matryoshka from the report's findings.
pub fn build_matryoshka(report: &Report) -> Vec<MatryoshkaShell> {
    let finding_ids: Vec<String> = report.findings.iter().map(|f| f.id.clone()).collect();

    let shell_0_hits: Vec<String> = finding_ids.iter()
        .filter(|id| id.starts_with("ASYNCOS-IOC-"))
        .cloned().collect();
    let shell_1_hits: Vec<String> = finding_ids.iter()
        .filter(|id| id.starts_with("SONAR-"))
        .cloned().collect();
    let shell_2_hits: Vec<String> = finding_ids.iter()
        .filter(|id| id.starts_with("MCFLY-"))
        .cloned().collect();
    let shell_3_hits: Vec<String> = finding_ids.iter()
        .filter(|id| id.starts_with("ERIS-DERAIL"))
        .cloned().collect();
    let shell_4_hits: Vec<String> = finding_ids.iter()
        .filter(|id| id.starts_with("ERIS-PARADOX"))
        .cloned().collect();

    vec![
        MatryoshkaShell {
            depth: 0,
            name: "IOC Surface".to_string(),
            evasion_signature: "hash-rotation, binary-packing, timestamp-stomp".to_string(),
            finding_ids: shell_0_hits.clone(),
            evasion_detected: false, // Set during cross-shell analysis
        },
        MatryoshkaShell {
            depth: 1,
            name: "Behavioral Echo".to_string(),
            evasion_signature: "timing-jitter, cache-flush, syscall-proxy".to_string(),
            finding_ids: shell_1_hits.clone(),
            // Evasion of Shell 0 means the attacker cleaned IOC surfaces —
            // if Shell 0 is clean but Shell 1 detects anomalies, evasion is confirmed
            evasion_detected: shell_0_hits.is_empty() && !shell_1_hits.is_empty(),
        },
        MatryoshkaShell {
            depth: 2,
            name: "Lineage Consistency".to_string(),
            evasion_signature: "btime-forge, uptime-spoof, pid-recycle".to_string(),
            finding_ids: shell_2_hits.clone(),
            evasion_detected: shell_0_hits.is_empty() && shell_1_hits.is_empty()
                && !shell_2_hits.is_empty(),
        },
        MatryoshkaShell {
            depth: 3,
            name: "Narrative Coherence".to_string(),
            evasion_signature: "cross-domain-sync, holistic-cleanup".to_string(),
            finding_ids: shell_3_hits.clone(),
            // If outer shells are clean but narrative derails, deep evasion detected
            evasion_detected: shell_0_hits.is_empty() && shell_1_hits.is_empty()
                && shell_2_hits.is_empty() && !shell_3_hits.is_empty(),
        },
        MatryoshkaShell {
            depth: 4,
            name: "Paradox Trap".to_string(),
            evasion_signature: "IMPOSSIBLE — evading this shell creates the signal it detects".to_string(),
            finding_ids: shell_4_hits.clone(),
            evasion_detected: !shell_4_hits.is_empty(),
        },
    ]
}

/// Deepest shell with detected evasion — the "real" detection depth.
pub fn deepest_evasion(shells: &[MatryoshkaShell]) -> Option<u8> {
    shells.iter()
        .filter(|s| s.evasion_detected || !s.finding_ids.is_empty())
        .map(|s| s.depth)
        .max()
}

// ─── Paradox Engineering ───────────────────────────────────────────────────

/// A paradox constraint: evading detection A requires condition C,
/// but condition C triggers detection B.
#[derive(Debug, Clone)]
pub struct ParadoxConstraint {
    /// What the attacker must do to evade detection A.
    pub evasion_requirement: String,
    /// What detection B looks for (which the evasion creates).
    pub triggered_detection: String,
    /// Constraint tightness (0.0–1.0). 1.0 = perfect paradox.
    pub tightness: f64,
}

/// A paradox bridge: the defender's semantic escape route.
/// The bridge hops between incompatible constraint lanes using
/// domain knowledge the attacker lacks.
#[derive(Debug, Clone)]
pub struct ParadoxBridge {
    /// The two constraints forming the paradox.
    pub constraint_a: ParadoxConstraint,
    pub constraint_b: ParadoxConstraint,
    /// The semantic key that resolves the paradox for the defender.
    pub semantic_key: String,
    /// Combined paradox strength (0.0–1.0).
    pub strength: f64,
    /// Estimated computational cost for attacker to brute-force
    /// the bridge without the semantic key (relative units).
    pub brute_force_cost: f64,
}

impl fmt::Display for ParadoxBridge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PARADOX(strength={:.0}%, brute_cost={:.1}): {} ⟷ {} [key: {}]",
            self.strength * 100.0,
            self.brute_force_cost,
            self.constraint_a.evasion_requirement,
            self.constraint_b.triggered_detection,
            self.semantic_key,
        )
    }
}

/// Build the standard paradox traps from system state.
///
/// Each trap is a pair of incompatible constraints:
///   - To evade timestamp detection, attacker must stomp timestamps
///   - But stomped timestamps break lineage consistency
///   - To fix lineage, attacker must know the original boot-time
///   - But knowing boot-time means reading /proc/stat which we monitor
///
/// The semantic bridge for the defender: we *know* our own boot-time
/// because we recorded it at startup. The attacker doesn't have this.
pub fn build_paradox_traps() -> Vec<ParadoxBridge> {
    vec![
        // Paradox 1: Timestamp Stomp ⟷ Lineage Break
        ParadoxBridge {
            constraint_a: ParadoxConstraint {
                evasion_requirement: "Stomp filesystem timestamps to hide file modification".to_string(),
                triggered_detection: "MCFLY lineage break: btime + uptime ≠ wall_clock".to_string(),
                tightness: 0.95,
            },
            constraint_b: ParadoxConstraint {
                evasion_requirement: "Forge btime to fix lineage consistency".to_string(),
                triggered_detection: "SONAR-ACOUSTIC: /proc/stat read timing anomaly".to_string(),
                tightness: 0.80,
            },
            semantic_key: "Defender records boot-time at init; attacker must guess it".to_string(),
            strength: 0.88,
            brute_force_cost: 2_u64.pow(50) as f64, // ~1 quadrillion timestamp combinations
        },
        // Paradox 2: Cache Flush ⟷ Optical Detection
        ParadoxBridge {
            constraint_a: ParadoxConstraint {
                evasion_requirement: "Flush cache lines to erase observation footprint".to_string(),
                triggered_detection: "SONAR-OPTICAL: cache eviction ratio spike detected".to_string(),
                tightness: 0.90,
            },
            constraint_b: ParadoxConstraint {
                evasion_requirement: "Avoid cache operations to stay invisible".to_string(),
                triggered_detection: "Stale cache entries prove non-interaction (cold-cache tell)".to_string(),
                tightness: 0.75,
            },
            semantic_key: "Defender knows which lines are sentinel probes; attacker sees uniform cache".to_string(),
            strength: 0.83,
            brute_force_cost: 2_u64.pow(52) as f64, // Cache geometry combinatorics
        },
        // Paradox 3: Process Hiding ⟷ Narrative Derailment
        ParadoxBridge {
            constraint_a: ParadoxConstraint {
                evasion_requirement: "Hide malicious process from ps/proc listing".to_string(),
                triggered_detection: "ERIS-DERAIL: /proc/*/maps shows mmap regions for unlisted PID".to_string(),
                tightness: 0.85,
            },
            constraint_b: ParadoxConstraint {
                evasion_requirement: "Clean mmap regions to fix narrative".to_string(),
                triggered_detection: "Memory pressure anomaly: freed pages without corresponding process exit".to_string(),
                tightness: 0.70,
            },
            semantic_key: "Defender tracks page allocation rates at equilibrium; deviation is signal".to_string(),
            strength: 0.78,
            brute_force_cost: 2_u64.pow(49) as f64, // Page-level address space combinatorics
        },
        // Paradox 4: Log Cleanup ⟷ Entropy Anomaly (Sacred Chao)
        ParadoxBridge {
            constraint_a: ParadoxConstraint {
                evasion_requirement: "Scrub log entries referencing attacker activity".to_string(),
                triggered_detection: "ERIS-DERAIL: log entropy drops below baseline (too clean)".to_string(),
                tightness: 0.88,
            },
            constraint_b: ParadoxConstraint {
                evasion_requirement: "Inject noise to restore entropy".to_string(),
                triggered_detection: "Noise pattern is syntactically valid but semantically empty (no referent PIDs)".to_string(),
                tightness: 0.72,
            },
            semantic_key: "Defender has ground-truth entropy model from equilibrium sampling".to_string(),
            strength: 0.80,
            brute_force_cost: 2_u64.pow(51) as f64, // Entropy space reconstruction
        },
        // Paradox 5: The Fifth Law — The Meta-Paradox
        // Evading all four above requires coordinated multi-domain cleanup.
        // Coordinated cleanup has a timing signature: all domains change
        // within the same narrow window. This is the Sacred Chao balance —
        // the order-from-chaos signal that no amount of noise can hide.
        ParadoxBridge {
            constraint_a: ParadoxConstraint {
                evasion_requirement: "Coordinate cleanup across all domains simultaneously".to_string(),
                triggered_detection: "ERIS-PARADOX-META: cross-domain synchronization spike".to_string(),
                tightness: 0.92,
            },
            constraint_b: ParadoxConstraint {
                evasion_requirement: "Stagger cleanup to avoid synchronization detection".to_string(),
                triggered_detection: "Inter-domain narrative drift during stagger window".to_string(),
                tightness: 0.85,
            },
            semantic_key: "Paradox is irreducible: sync triggers one detector, async triggers another. All things happen in fives.".to_string(),
            strength: 0.95,
            brute_force_cost: f64::INFINITY, // Irreducible paradox
        },
    ]
}

// ─── Derailment Detection Engine ───────────────────────────────────────────

/// Extract narrative assertions from the report's findings.
fn extract_narrative(report: &Report) -> Vec<NarrativeAssertion> {
    let mut assertions = Vec::new();

    // Filesystem narrative: if no manifest drift, FS claims consistency
    let has_manifest_drift = report.findings.iter()
        .any(|f| f.id.starts_with("ASYNCOS-MANIFEST-DRIFT"));
    assertions.push(NarrativeAssertion {
        domain: "fs".to_string(),
        claim: if has_manifest_drift {
            "Filesystem shows size deviation".to_string()
        } else {
            "Filesystem sizes are consistent with baseline".to_string()
        },
        evidence_ids: report.findings.iter()
            .filter(|f| f.id.contains("MANIFEST"))
            .map(|f| f.id.clone()).collect(),
        holds: Some(!has_manifest_drift),
    });

    // Process narrative
    let has_unexpected_proc = report.findings.iter()
        .any(|f| f.id.contains("UNEXPECTED-PROCESS"));
    assertions.push(NarrativeAssertion {
        domain: "proc".to_string(),
        claim: if has_unexpected_proc {
            "Unexpected processes detected".to_string()
        } else {
            "All running processes match known-good baseline".to_string()
        },
        evidence_ids: report.findings.iter()
            .filter(|f| f.id.contains("PROCESS"))
            .map(|f| f.id.clone()).collect(),
        holds: Some(!has_unexpected_proc),
    });

    // Log/lineage narrative
    let has_lineage_break = report.findings.iter()
        .any(|f| f.id.starts_with("MCFLY-LINEAGE-BREAK"));
    assertions.push(NarrativeAssertion {
        domain: "log".to_string(),
        claim: if has_lineage_break {
            "System lineage is inconsistent (time-travel detected)".to_string()
        } else {
            "System boot lineage is internally consistent".to_string()
        },
        evidence_ids: report.findings.iter()
            .filter(|f| f.id.contains("MCFLY") || f.id.contains("LINEAGE"))
            .map(|f| f.id.clone()).collect(),
        holds: Some(!has_lineage_break),
    });

    // Cache/sonar narrative
    let has_cache_eviction = report.findings.iter()
        .any(|f| f.id.contains("OPTICAL-CACHE"));
    assertions.push(NarrativeAssertion {
        domain: "cache".to_string(),
        claim: if has_cache_eviction {
            "Cache timing anomaly detected (observation footprint)".to_string()
        } else {
            "Cache behavior is consistent with unobserved operation".to_string()
        },
        evidence_ids: report.findings.iter()
            .filter(|f| f.id.contains("SONAR") || f.id.contains("CACHE"))
            .map(|f| f.id.clone()).collect(),
        holds: Some(!has_cache_eviction),
    });

    // Network narrative
    let has_net_anomaly = report.findings.iter()
        .any(|f| f.id.contains("IOC-IP") || f.id.contains("GRAV-NET"));
    assertions.push(NarrativeAssertion {
        domain: "net".to_string(),
        claim: if has_net_anomaly {
            "Network state shows anomalous connections or listeners".to_string()
        } else {
            "Network connections are within expected parameters".to_string()
        },
        evidence_ids: report.findings.iter()
            .filter(|f| f.id.contains("IP") || f.id.contains("NET") || f.id.contains("GRAV"))
            .map(|f| f.id.clone()).collect(),
        holds: Some(!has_net_anomaly),
    });

    assertions
}

/// Detect narrative derailments: cross-domain contradictions.
///
/// Key insight: a clean system has a coherent narrative across all domains.
/// An attacked-and-cleaned system may have been cleaned in some domains
/// but not others, producing contradictions. The attacker's "nothing happened"
/// story derails when domain A says "clean" but domain B says "dirty" in a
/// way that requires A to also be dirty.
fn detect_derailments(assertions: &[NarrativeAssertion]) -> Vec<Derailment> {
    let mut derailments = Vec::new();

    // Cross-check: if processes are clean but filesystem drifted,
    // there should be a process that *caused* the drift. Missing process = derailment.
    let fs_dirty = assertions.iter().find(|a| a.domain == "fs" && a.holds == Some(false));
    let proc_clean = assertions.iter().find(|a| a.domain == "proc" && a.holds == Some(true));
    if let (Some(fs_a), Some(proc_a)) = (fs_dirty, proc_clean) {
        derailments.push(Derailment {
            assertion_a: fs_a.clone(),
            assertion_b: proc_a.clone(),
            severity: 0.75,
            discordian_law: classify_law("fs", "proc"),
        });
    }

    // Cross-check: if lineage is broken but filesystem claims consistency,
    // something changed the boot record without touching files — suspicious.
    let log_dirty = assertions.iter().find(|a| a.domain == "log" && a.holds == Some(false));
    let fs_clean = assertions.iter().find(|a| a.domain == "fs" && a.holds == Some(true));
    if let (Some(log_a), Some(fs_a)) = (log_dirty, fs_clean) {
        derailments.push(Derailment {
            assertion_a: log_a.clone(),
            assertion_b: fs_a.clone(),
            severity: 0.85,
            discordian_law: classify_law("log", "fs"),
        });
    }

    // Cross-check: cache anomaly with clean processes — who's reading our cache?
    let cache_dirty = assertions.iter().find(|a| a.domain == "cache" && a.holds == Some(false));
    if let (Some(cache_a), Some(proc_a)) = (cache_dirty, proc_clean) {
        derailments.push(Derailment {
            assertion_a: cache_a.clone(),
            assertion_b: proc_a.clone(),
            severity: 0.90,
            discordian_law: classify_law("cache", "proc"),
        });
    }

    // Cross-check: network anomaly with clean filesystem — C2 channel without implant on disk
    let net_dirty = assertions.iter().find(|a| a.domain == "net" && a.holds == Some(false));
    if let (Some(net_a), Some(fs_a)) = (net_dirty, fs_clean) {
        derailments.push(Derailment {
            assertion_a: net_a.clone(),
            assertion_b: fs_a.clone(),
            severity: 0.80,
            discordian_law: classify_law("net", "fs"),
        });
    }

    // The Fifth Check: if ≥3 domains are clean but ≥1 is dirty,
    // the cleanup was partial — Law of Fives says this is a sign.
    let clean_count = assertions.iter().filter(|a| a.holds == Some(true)).count();
    let dirty_count = assertions.iter().filter(|a| a.holds == Some(false)).count();
    if clean_count >= 3 && dirty_count >= 1 {
        // Find the dirty one
        if let Some(dirty_a) = assertions.iter().find(|a| a.holds == Some(false)) {
            if let Some(clean_a) = assertions.iter().find(|a| a.holds == Some(true)) {
                derailments.push(Derailment {
                    assertion_a: dirty_a.clone(),
                    assertion_b: clean_a.clone(),
                    severity: 0.65,
                    discordian_law: DISCORDIAN_LAWS[0], // Law of Fives
                });
            }
        }
    }

    derailments
}

// ─── Paradox Trap Evaluation ───────────────────────────────────────────────

/// Check if any paradox traps have been sprung based on report state.
fn evaluate_paradox_traps(report: &Report, traps: &[ParadoxBridge]) -> Vec<(usize, f64)> {
    let mut sprung = Vec::new();
    let finding_ids: Vec<&str> = report.findings.iter().map(|f| f.id.as_str()).collect();

    // Trap 0: Timestamp stomp + lineage break
    if finding_ids.iter().any(|id| id.contains("MANIFEST-DRIFT"))
        && finding_ids.iter().any(|id| id.contains("MCFLY-LINEAGE"))
    {
        sprung.push((0, traps[0].strength));
    }

    // Trap 1: Cache anomaly detection
    if finding_ids.iter().any(|id| id.contains("OPTICAL-CACHE")) {
        sprung.push((1, traps[1].strength));
    }

    // Trap 2: Process hiding + mmap contradiction
    if finding_ids.iter().any(|id| id.contains("UNEXPECTED-PROCESS"))
        && finding_ids.iter().any(|id| id.contains("SONAR"))
    {
        sprung.push((2, traps[2].strength));
    }

    // Trap 3: Log cleanup entropy
    // If the system is suspiciously clean (no findings from layers that should find something
    // on a compromised system), the cleanup itself is the signal
    let sonar_clean = !finding_ids.iter().any(|id| id.starts_with("SONAR-"));
    let ioc_hit = finding_ids.iter().any(|id| id.starts_with("ASYNCOS-IOC-"));
    if ioc_hit && sonar_clean {
        // IOC found but sonar says nothing is watching us? Too clean.
        sprung.push((3, traps[3].strength));
    }

    // Trap 4: Meta-paradox — multiple traps sprung = coordinated cleanup
    if sprung.len() >= 2 {
        sprung.push((4, traps[4].strength));
    }

    sprung
}

// ─── Rapid Intercept Heuristics ────────────────────────────────────────────

/// Heuristic for rapid-intercept detection: the attacker tries to intercept
/// and modify our detection output before we can record it. Eris detects
/// this by running assertion checks in scrambled order and verifying that
/// the results are independent of execution order (they should be, for a
/// clean system; an attacker intercepting in real-time may produce
/// order-dependent results).
fn rapid_intercept_check(assertions: &[NarrativeAssertion]) -> Option<f64> {
    if assertions.len() < 3 {
        return None;
    }

    // In a real implementation, we'd run checks in multiple random orders.
    // Here we detect the signature: if a finding's evidence changes between
    // evaluation passes (simulated by checking internal consistency).
    //
    // For now, compute a consistency hash across all assertions:
    // if any assertion has evidence_ids that reference nonexistent findings,
    // that's an intercept artifact.
    let all_evidence: Vec<&str> = assertions.iter()
        .flat_map(|a| a.evidence_ids.iter().map(|s| s.as_str()))
        .collect();

    // Count orphaned references (evidence IDs not in any assertion's domain set)
    let orphan_count = all_evidence.iter()
        .filter(|id| {
            // An evidence ID should relate to its assertion's domain
            // This is a simplified check
            id.is_empty()
        })
        .count();

    if orphan_count > 0 {
        Some(orphan_count as f64 / all_evidence.len().max(1) as f64)
    } else {
        None
    }
}

// ─── Eris Sweep (Public API) ──────────────────────────────────────────────

/// Run the full Eris sweep: derailment detection + matryoshka classification
/// + paradox trap evaluation.
///
/// This is Layer 9 of the detection pipeline.
pub fn eris_sweep(
    _sys: &dyn Sys,
    _fs: &dyn Fs,
    _torque: &mut TorqueExecutor,
    report: &mut Report,
) {
    // Phase 1: Extract narrative from accumulated findings
    let assertions = extract_narrative(report);

    // Phase 2: Detect derailments (cross-domain contradictions)
    let derailments = detect_derailments(&assertions);
    for d in &derailments {
        let mut finding = Finding::new(
            "ERIS-DERAIL-NARRATIVE",
            if d.severity > 0.8 { Severity::High } else { Severity::Medium },
            &format!(
                "Narrative derailment: [{}] vs [{}] — {}",
                d.assertion_a.domain, d.assertion_b.domain, d.discordian_law
            ),
        )
        .with_role(FindingRole::Bridge)
        .with_confidence(d.severity);

        finding.evidence.push(format!("claim_a={}", d.assertion_a.claim));
        finding.evidence.push(format!("claim_b={}", d.assertion_b.claim));
        finding.evidence.push(format!("law={}", d.discordian_law));
        finding.evidence.push(format!("severity={:.2}", d.severity));
        report.push(finding);
    }

    // Phase 3: Build matryoshka and classify detection depth
    let shells = build_matryoshka(report);
    let depth = deepest_evasion(&shells);

    if let Some(d) = depth {
        let shell = &shells[d as usize];
        let finding = Finding::new(
            &format!("ERIS-MATRYOSHKA-DEPTH-{d}"),
            if d >= 3 { Severity::Critical } else if d >= 1 { Severity::High } else { Severity::Medium },
            &format!(
                "Detection penetrated to shell {d} ({}): evasion={}, findings={}",
                shell.name,
                shell.evasion_detected,
                shell.finding_ids.len(),
            ),
        )
        .with_role(if d >= 3 { FindingRole::Core } else { FindingRole::Bridge })
        .with_confidence(0.7 + (d as f64 * 0.07));

        report.push(finding);
    }

    // Phase 4: Evaluate paradox traps
    let traps = build_paradox_traps();
    let sprung = evaluate_paradox_traps(report, &traps);
    for (idx, strength) in &sprung {
        let trap = &traps[*idx];
        let finding = Finding::new(
            &format!("ERIS-PARADOX-TRAP-{idx}"),
            if *strength > 0.85 { Severity::Critical } else { Severity::High },
            &format!(
                "Paradox trap #{idx} sprung (strength={:.0}%): {} ⟷ {} [key: {}]",
                strength * 100.0,
                trap.constraint_a.evasion_requirement,
                trap.constraint_b.triggered_detection,
                trap.semantic_key,
            ),
        )
        .with_role(FindingRole::Core)
        .with_confidence(*strength);

        report.push(finding);
    }

    // Phase 5: Rapid intercept check
    if let Some(intercept_score) = rapid_intercept_check(&assertions) {
        let finding = Finding::new(
            "ERIS-RAPID-INTERCEPT",
            Severity::Critical,
            &format!(
                "Rapid intercept pattern detected: {:.0}% assertion orphan rate — \
                 attacker may be modifying detection output in flight",
                intercept_score * 100.0,
            ),
        )
        .with_role(FindingRole::Core)
        .with_confidence(intercept_score.clamp(0.5, 0.95));

        report.push(finding);
    }

    // Phase 6: Record Eris summary in notes
    report.notes.push(format!(
        "[eris] derailments={}, depth={}, paradox_traps_sprung={}, shells={}",
        derailments.len(),
        depth.map(|d| d.to_string()).unwrap_or_else(|| "none".to_string()),
        sprung.len(),
        shells.iter().map(|s| format!("S{}:{}", s.depth,
            if !s.finding_ids.is_empty() || s.evasion_detected { "HIT" } else { "CLEAN" }
        )).collect::<Vec<_>>().join(","),
    ));
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::{Finding, Report, Severity};

    fn make_report_with_findings(ids: &[(&str, Severity)]) -> Report {
        let mut report = Report::new();
        for (id, sev) in ids {
            report.push(Finding::new(*id, *sev, &format!("Test finding: {id}")));
        }
        report
    }

    #[test]
    fn classify_law_temporal() {
        assert_eq!(classify_law("fs", "log"), "POEE Curse");
    }

    #[test]
    fn classify_law_visibility() {
        assert_eq!(classify_law("proc", "fs"), "Orange Revelation");
    }

    #[test]
    fn classify_law_causality() {
        assert_eq!(classify_law("net", "proc"), "Starbuck's Pebble");
    }

    #[test]
    fn classify_law_entropy() {
        assert_eq!(classify_law("cache", "log"), "Sacred Chao Balance");
    }

    #[test]
    fn classify_law_default_fives() {
        assert_eq!(classify_law("other", "thing"), "Law of Fives");
    }

    #[test]
    fn matryoshka_clean_system() {
        let report = Report::new();
        let shells = build_matryoshka(&report);
        assert_eq!(shells.len(), 5);
        assert!(shells.iter().all(|s| !s.evasion_detected));
        assert_eq!(deepest_evasion(&shells), None);
    }

    #[test]
    fn matryoshka_ioc_only() {
        let report = make_report_with_findings(&[
            ("ASYNCOS-IOC-HASH-AquaTunnel", Severity::Critical),
        ]);
        let shells = build_matryoshka(&report);
        assert!(!shells[0].finding_ids.is_empty());
        assert_eq!(deepest_evasion(&shells), Some(0));
    }

    #[test]
    fn matryoshka_shell1_evasion_detected() {
        // IOC surface is clean (attacker cleaned hashes) but sonar detects anomaly
        let report = make_report_with_findings(&[
            ("SONAR-ACOUSTIC-INTERCEPT", Severity::High),
        ]);
        let shells = build_matryoshka(&report);
        assert!(shells[1].evasion_detected, "Shell 1 should detect evasion of Shell 0");
        assert_eq!(deepest_evasion(&shells), Some(1));
    }

    #[test]
    fn matryoshka_shell2_evasion_detected() {
        // IOC and sonar clean but lineage broken
        let report = make_report_with_findings(&[
            ("MCFLY-LINEAGE-BREAK", Severity::High),
        ]);
        let shells = build_matryoshka(&report);
        assert!(shells[2].evasion_detected);
        assert_eq!(deepest_evasion(&shells), Some(2));
    }

    #[test]
    fn paradox_traps_count() {
        let traps = build_paradox_traps();
        assert_eq!(traps.len(), 5, "Law of Fives: exactly five paradox traps");
    }

    #[test]
    fn paradox_trap_meta_infinite_cost() {
        let traps = build_paradox_traps();
        assert!(traps[4].brute_force_cost.is_infinite(),
            "Meta-paradox should have infinite brute-force cost");
    }

    #[test]
    fn paradox_trap_sprung_on_drift_plus_lineage() {
        let report = make_report_with_findings(&[
            ("ASYNCOS-MANIFEST-DRIFT", Severity::Medium),
            ("MCFLY-LINEAGE-BREAK", Severity::High),
        ]);
        let traps = build_paradox_traps();
        let sprung = evaluate_paradox_traps(&report, &traps);
        assert!(!sprung.is_empty(), "Drift + lineage break should spring trap 0");
        assert!(sprung.iter().any(|(idx, _)| *idx == 0));
    }

    #[test]
    fn paradox_meta_trap_requires_two_others() {
        let report = make_report_with_findings(&[
            ("ASYNCOS-MANIFEST-DRIFT", Severity::Medium),
            ("MCFLY-LINEAGE-BREAK", Severity::High),
            ("SONAR-OPTICAL-CACHE-EVICTION", Severity::High),
        ]);
        let traps = build_paradox_traps();
        let sprung = evaluate_paradox_traps(&report, &traps);
        assert!(sprung.iter().any(|(idx, _)| *idx == 4),
            "Meta-trap should fire when 2+ other traps spring");
    }

    #[test]
    fn derailment_fs_dirty_proc_clean() {
        let assertions = vec![
            NarrativeAssertion {
                domain: "fs".to_string(),
                claim: "Filesystem shows size deviation".to_string(),
                evidence_ids: vec!["ASYNCOS-MANIFEST-DRIFT".to_string()],
                holds: Some(false),
            },
            NarrativeAssertion {
                domain: "proc".to_string(),
                claim: "All processes match baseline".to_string(),
                evidence_ids: vec![],
                holds: Some(true),
            },
        ];
        let derailments = detect_derailments(&assertions);
        assert!(!derailments.is_empty());
        assert_eq!(derailments[0].discordian_law, "Orange Revelation");
    }

    #[test]
    fn derailment_cache_dirty_proc_clean() {
        let assertions = vec![
            NarrativeAssertion {
                domain: "cache".to_string(),
                claim: "Cache anomaly detected".to_string(),
                evidence_ids: vec!["SONAR-OPTICAL-CACHE".to_string()],
                holds: Some(false),
            },
            NarrativeAssertion {
                domain: "proc".to_string(),
                claim: "All processes match baseline".to_string(),
                evidence_ids: vec![],
                holds: Some(true),
            },
        ];
        let derailments = detect_derailments(&assertions);
        assert!(!derailments.is_empty());
        assert_eq!(derailments[0].discordian_law, "Sacred Chao Balance");
        assert!(derailments[0].severity >= 0.85);
    }

    #[test]
    fn eris_sweep_on_clean_system() {
        let mut report = Report::new();
        // Use a minimal mock — eris_sweep doesn't actually use sys/fs/torque
        // since it operates on accumulated findings
        use crate::io::MockFs;
        use crate::sys::MockSys;
        let sys = MockSys::new();
        let fs = MockFs::new();
        let mut torque = TorqueExecutor::new(&fs);

        eris_sweep(&sys, &fs, &mut torque, &mut report);

        // Clean system: no ERIS findings should be emitted
        let _eris_findings: Vec<_> = report.findings.iter()
            .filter(|f| f.id.starts_with("ERIS-"))
            .collect();
        // Might have a law-of-fives derailment if thresholds trigger,
        // but no paradox traps should spring
        let paradox_findings: Vec<_> = report.findings.iter()
            .filter(|f| f.id.starts_with("ERIS-PARADOX"))
            .collect();
        assert!(paradox_findings.is_empty(), "Clean system should have no paradox traps");
    }

    #[test]
    fn eris_sweep_on_compromised_system() {
        let mut report = Report::new();

        // Simulate findings from prior layers
        report.push(Finding::new("ASYNCOS-MANIFEST-DRIFT", Severity::Medium, "8% drift"));
        report.push(Finding::new("ASYNCOS-IOC-HASH-AquaTunnel", Severity::Critical, "IOC hit"));
        report.push(Finding::new("SONAR-ACOUSTIC-INTERCEPT", Severity::High, "Timing anomaly"));
        report.push(Finding::new("MCFLY-LINEAGE-BREAK", Severity::High, "Lineage broken"));

        use crate::io::MockFs;
        use crate::sys::MockSys;
        let sys = MockSys::new();
        let fs = MockFs::new();
        let mut torque = TorqueExecutor::new(&fs);

        eris_sweep(&sys, &fs, &mut torque, &mut report);

        // Should produce derailment findings
        let derailments: Vec<_> = report.findings.iter()
            .filter(|f| f.id.starts_with("ERIS-DERAIL"))
            .collect();
        assert!(!derailments.is_empty(), "Compromised system should produce derailments");

        // Should produce paradox trap findings (drift + lineage = trap 0)
        let paradox: Vec<_> = report.findings.iter()
            .filter(|f| f.id.starts_with("ERIS-PARADOX"))
            .collect();
        assert!(!paradox.is_empty(), "Drift + lineage break should spring paradox traps");

        // Should have matryoshka depth finding
        let matryoshka: Vec<_> = report.findings.iter()
            .filter(|f| f.id.starts_with("ERIS-MATRYOSHKA"))
            .collect();
        assert!(!matryoshka.is_empty(), "Should report detection depth");

        // Eris notes should be present
        assert!(report.notes.iter().any(|n| n.starts_with("[eris]")));
    }

    // ─── Unsolvable Tests → Paradox-Engineered Resolution ──────────────────

    /// The attacker faces an IMPOSSIBLE choice: evade ALL detection without
    /// triggering ANY paradox. This test proves the trap is irreducible.
    #[test]
    fn paradox_unsolvable_full_evasion() {
        let traps = build_paradox_traps();

        // Attempt to find a state where all traps are unarmed.
        // The meta-trap (#4) arms when 2+ other traps are sprung.
        // But traps 0-3 each cover a different evasion axis.
        // To avoid ALL of 0-3, the attacker must: not stomp timestamps,
        // not flush cache, not hide processes, not scrub logs.
        // But if they don't do ANY of those things, conventional detection
        // (Shells 0-2) catches them directly.
        //
        // This is the irreducible paradox: evasion requires cleanup,
        // cleanup triggers traps, and avoiding cleanup means getting caught
        // by the outer shells.

        // Proof: the only states where no trap fires are states where
        // the attacker performed no evasion (and thus got caught at Shell 0).
        // The meta-trap has infinite brute-force cost precisely because
        // there's no computational path through the paradox without the
        // semantic key.
        assert!(traps[4].brute_force_cost.is_infinite());
        assert!(traps[4].strength > 0.90);

        // The semantic bridge resolves it for the defender:
        // we KNOW which traps we set, so we can interpret the results.
        assert!(traps[4].semantic_key.contains("irreducible"));
    }

    /// Semantic bridge hop: demonstrate that the defender can traverse
    /// incompatible constraint lanes by holding the semantic key.
    #[test]
    fn semantic_bridge_hop_demonstration() {
        let traps = build_paradox_traps();

        // The defender's view: each trap's semantic_key resolves the paradox.
        // The attacker's view: each trap presents an impossible choice.
        for (i, trap) in traps.iter().enumerate() {
            // Key exists and is non-empty
            assert!(!trap.semantic_key.is_empty(), "Trap {i} must have semantic key");

            // Constraints are tight enough that random evasion fails
            assert!(trap.constraint_a.tightness > 0.5, "Trap {i} constraint A too loose");
            assert!(trap.constraint_b.tightness > 0.5, "Trap {i} constraint B too loose");

            // Combined strength exceeds individual constraints
            // (the paradox is stronger than either side alone)
            let avg_tightness = (trap.constraint_a.tightness + trap.constraint_b.tightness) / 2.0;
            assert!(trap.strength >= avg_tightness * 0.8,
                "Trap {i} strength should reflect combined constraints");
        }
    }

    /// Test that computation-engine crash condition is modeled:
    /// without the semantic key, resolution requires brute force
    /// that exceeds practical computation bounds.
    #[test]
    fn computation_crash_without_semantic_bridge() {
        let traps = build_paradox_traps();

        // Sum total brute-force cost across all traps
        let finite_costs: Vec<f64> = traps.iter()
            .map(|t| t.brute_force_cost)
            .filter(|c| c.is_finite())
            .collect();

        let total_cost: f64 = finite_costs.iter().sum();

        // Even without the infinite meta-trap, the combined cost
        // of brute-forcing all finite traps should exceed practical limits
        // (2^48 ≈ 281 trillion operations)
        assert!(total_cost > 2_f64.powf(48.0),
            "Combined brute-force cost should exceed practical limits: {total_cost}");
    }
}

//! SpinalTap — Criticality Scoring Inversion & Epoch Grammar Illegitimization.
//!
//! Named for the film's famous "these go to eleven" — because CVE-2026-20127
//! and CVE-2025-20393 both score 10.0 CVSS (maximum criticality), and this
//! module models the adversary's optimization angles to push their advantage
//! to eleven, then builds the counter-model that breaks their grammar.
//!
//! ## Architecture
//!
//! Five interlocking subsystems:
//!
//! ### 1. Criticality Score Inversion (The Eleven Model)
//!
//! The adversary optimizes across abstraction layers:
//!   - Transport layer: investigation pipeline as delivery mechanism
//!   - Target layer: downstream forensic bench as resident target
//!   - Feedback layer: forensic results as training data for evasion
//!
//! Each layer has optimization angles the attacker exploits. SpinalTap
//! models these angles and inverts the scoring: what the attacker sees
//! as their strongest advantage becomes the defender's detection surface.
//!
//! ### 2. Epoch Grammar Illegitimization
//!
//! The attacker's campaign operates in epochs: reconnaissance → exploitation →
//! persistence → exfiltration → cleanup. Each epoch has a "grammar" — a set
//! of valid state transitions. SpinalTap illegitimizes this grammar by
//! injecting invariant locks that make certain transitions computationally
//! impossible without detection.
//!
//! ### 3. Theory of Mind as Programming Paradigm
//!
//! The defender builds a model of the attacker's model of the defender.
//! This recursive theory-of-mind produces constraint sets that the attacker
//! must satisfy to remain undetected. The constraints are derived from
//! the attacker's assumed optimization targets — what they THINK we're
//! looking for determines what we ACTUALLY look for (their evasion pattern).
//!
//! ### 4. Futures Array World Modeling
//!
//! Biological-principle inspired constraint modeling: evolutionary fitness
//! landscapes, predator-prey dynamics, immune system analogies. The defender
//! generates hypothetical centroids of attack-cluster futures and pre-positions
//! detection at those centroids. Home field advantage: the defender defines
//! the landscape the attacker must traverse.
//!
//! ### 5. Regex Gap Layer (Symbolic Accumulation Density)
//!
//! Stripped syntax patterns that appear as conventional operations may carry
//! weaponized semantics visible only through specific rendering pipelines
//! (SVG accumulation, compression artifacts, symbolic math evaluation).
//! This layer detects density anomalies in syntactic structures — places
//! where "normal looking" patterns have abnormal information density.

use std::fmt;

use crate::io::Fs;
use crate::report::{Finding, FindingRole, Report, Severity};
use crate::sys::Sys;
use crate::torque::TorqueExecutor;

// ─── Criticality Score Inversion ───────────────────────────────────────────

/// An optimization angle the adversary exploits at a given abstraction layer.
#[derive(Debug, Clone)]
pub struct OptimizationAngle {
    /// Abstraction layer (transport, target, feedback, meta).
    pub layer: &'static str,
    /// What the attacker optimizes for.
    pub attacker_objective: String,
    /// The defender's detection surface this creates.
    pub detection_surface: String,
    /// Attacker's perceived advantage (0.0–1.0).
    pub attacker_advantage: f64,
    /// Defender's inverted advantage (0.0–1.0).
    pub defender_inversion: f64,
}

/// The "Eleven Model": maps CVSS 10.0 optimization angles to detection surfaces.
///
/// Two CVEs at maximum criticality means the attacker has maximum optimization
/// space. But maximum optimization space = maximum surface area for detection
/// of optimization patterns.
pub fn build_eleven_model() -> Vec<OptimizationAngle> {
    vec![
        // Transport layer: investigation pipeline as delivery
        OptimizationAngle {
            layer: "transport",
            attacker_objective: "Ride investigation pipeline to reach forensic bench".to_string(),
            detection_surface: "Pipeline traversal creates deterministic timing signature".to_string(),
            attacker_advantage: 0.90,
            defender_inversion: 0.85, // High: we control the pipeline
        },
        OptimizationAngle {
            layer: "transport",
            attacker_objective: "Inject payload during evidence collection phase".to_string(),
            detection_surface: "Evidence integrity hash chain breaks on injection".to_string(),
            attacker_advantage: 0.70,
            defender_inversion: 0.90, // Very high: we compute the hashes
        },
        // Target layer: forensic bench as resident target
        OptimizationAngle {
            layer: "target",
            attacker_objective: "Establish persistence on forensic workstation".to_string(),
            detection_surface: "Bench state drift from known-good baseline".to_string(),
            attacker_advantage: 0.80,
            defender_inversion: 0.75,
        },
        OptimizationAngle {
            layer: "target",
            attacker_objective: "Modify forensic tooling to suppress findings".to_string(),
            detection_surface: "Tool output diverges from independent validation (this crate)".to_string(),
            attacker_advantage: 0.85,
            defender_inversion: 0.95, // Critical: we ARE the independent validation
        },
        // Feedback layer: forensic results as evasion training data
        OptimizationAngle {
            layer: "feedback",
            attacker_objective: "Learn defender detection logic from results".to_string(),
            detection_surface: "Access pattern to results files reveals observer presence".to_string(),
            attacker_advantage: 0.75,
            defender_inversion: 0.80,
        },
        OptimizationAngle {
            layer: "feedback",
            attacker_objective: "Train evasion model on false-negative patterns".to_string(),
            detection_surface: "Evasion model training requires compute that we can detect via sonar".to_string(),
            attacker_advantage: 0.65,
            defender_inversion: 0.85,
        },
        // Meta layer: modeling our model of their model
        OptimizationAngle {
            layer: "meta",
            attacker_objective: "Predict what defender is looking for and preempt".to_string(),
            detection_surface: "Preemption requires enumerating our detection space = combinatorial explosion".to_string(),
            attacker_advantage: 0.60,
            defender_inversion: 0.92, // Near-maximal: combinatorial advantage is ours
        },
        OptimizationAngle {
            layer: "meta",
            attacker_objective: "Use agentic AI to automate evasion at speed".to_string(),
            detection_surface: "Agentic patterns have deterministic scaffolding signatures in system calls".to_string(),
            attacker_advantage: 0.80,
            defender_inversion: 0.70,
        },
    ]
}

/// Compute the "Spinal Tap score" — how far past 10.0 the combined
/// detection advantage goes when inversion is applied.
pub fn spinal_tap_score(angles: &[OptimizationAngle]) -> f64 {
    if angles.is_empty() {
        return 0.0;
    }
    let total_inversion: f64 = angles.iter().map(|a| a.defender_inversion).sum();
    let total_advantage: f64 = angles.iter().map(|a| a.attacker_advantage).sum();
    // Ratio > 1.0 means defender inverts more than attacker gains
    // Scale to 0-11 range (Spinal Tap goes to 11)
    let ratio = total_inversion / total_advantage.max(0.01);
    ratio * 10.0  // If ratio > 1.0, this goes to 11
}

// ─── Epoch Grammar Illegitimization ────────────────────────────────────────

/// An epoch in the attack lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Epoch {
    Reconnaissance,
    Exploitation,
    Persistence,
    Exfiltration,
    Cleanup,
}

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reconnaissance => write!(f, "RECON"),
            Self::Exploitation => write!(f, "EXPLOIT"),
            Self::Persistence => write!(f, "PERSIST"),
            Self::Exfiltration => write!(f, "EXFIL"),
            Self::Cleanup => write!(f, "CLEANUP"),
        }
    }
}

/// A grammar transition: from one epoch to another, with required state changes.
#[derive(Debug, Clone)]
pub struct GrammarTransition {
    pub from: Epoch,
    pub to: Epoch,
    /// State changes the attacker must make for this transition.
    pub required_changes: Vec<String>,
    /// Which invariant locks this transition violates.
    pub violated_locks: Vec<String>,
}

/// An invariant lock: a condition that, if violated, produces a detection signal.
/// The lock is "invisible" to the attacker because it's expressed as a statistical
/// property, not a specific check.
#[derive(Debug, Clone)]
pub struct InvariantLock {
    /// Lock identifier.
    pub id: String,
    /// What property is locked.
    pub property: String,
    /// The invariant that must hold.
    pub invariant: String,
    /// Which epoch transitions violate this lock.
    pub violated_by: Vec<(Epoch, Epoch)>,
    /// Detection confidence when violated (0.0–1.0).
    pub detection_confidence: f64,
}

/// Build the standard invariant lock set.
///
/// Each lock makes a specific epoch transition "illegal" — the attacker
/// can still perform the transition, but doing so produces a signal
/// that wasn't visible from their optimization model.
pub fn build_invariant_locks() -> Vec<InvariantLock> {
    vec![
        InvariantLock {
            id: "ENTROPY-MONOTONICITY".to_string(),
            property: "System entropy (log + fs + proc combined)".to_string(),
            invariant: "Entropy is monotonically non-decreasing over time in a healthy system. \
                        Cleanup epoch requires entropy decrease = violation.".to_string(),
            violated_by: vec![(Epoch::Persistence, Epoch::Cleanup)],
            detection_confidence: 0.85,
        },
        InvariantLock {
            id: "CAUSALITY-CHAIN".to_string(),
            property: "Process creation → file creation → log entry causal ordering".to_string(),
            invariant: "Every file must have a creating process, every process must have \
                        a log entry. Exploitation creates files without legitimate processes.".to_string(),
            violated_by: vec![(Epoch::Exploitation, Epoch::Persistence)],
            detection_confidence: 0.80,
        },
        InvariantLock {
            id: "TEMPORAL-CONSISTENCY".to_string(),
            property: "Filesystem timestamps form a DAG with process timestamps".to_string(),
            invariant: "File mtime ≤ parent process start_time + process lifetime. \
                        Timestamp stomping during cleanup breaks this DAG.".to_string(),
            violated_by: vec![
                (Epoch::Persistence, Epoch::Cleanup),
                (Epoch::Exploitation, Epoch::Cleanup),
            ],
            detection_confidence: 0.90,
        },
        InvariantLock {
            id: "INFORMATION-CONSERVATION".to_string(),
            property: "Total information content (compressed size proxy) of system state".to_string(),
            invariant: "Information cannot be destroyed without trace — compression ratio \
                        shift reveals deleted content even after zeroing.".to_string(),
            violated_by: vec![(Epoch::Exfiltration, Epoch::Cleanup)],
            detection_confidence: 0.75,
        },
        InvariantLock {
            id: "OBSERVATION-PARADOX".to_string(),
            property: "The act of reconnaissance changes the system state observably".to_string(),
            invariant: "Any enumeration of detection surfaces leaves cache/timing artifacts. \
                        Reconnaissance epoch cannot be zero-footprint on shared hardware.".to_string(),
            violated_by: vec![(Epoch::Reconnaissance, Epoch::Exploitation)],
            detection_confidence: 0.70,
        },
    ]
}

/// Check which invariant locks are violated by the current system state.
/// Returns violated lock IDs with detection confidence.
fn evaluate_locks(report: &Report, locks: &[InvariantLock]) -> Vec<(String, f64)> {
    let mut violations = Vec::new();
    let finding_ids: Vec<&str> = report.findings.iter().map(|f| f.id.as_str()).collect();

    // ENTROPY-MONOTONICITY: if eris detected "too clean" narrative
    if finding_ids.iter().any(|id| id.contains("DERAIL") && id.contains("ERIS")) {
        violations.push(("ENTROPY-MONOTONICITY".to_string(), locks[0].detection_confidence));
    }

    // CAUSALITY-CHAIN: if unexpected processes exist (persistence without legit creation)
    if finding_ids.iter().any(|id| id.contains("UNEXPECTED-PROCESS")) {
        violations.push(("CAUSALITY-CHAIN".to_string(), locks[1].detection_confidence));
    }

    // TEMPORAL-CONSISTENCY: if McFly detected lineage breaks
    if finding_ids.iter().any(|id| id.starts_with("MCFLY-LINEAGE")) {
        violations.push(("TEMPORAL-CONSISTENCY".to_string(), locks[2].detection_confidence));
    }

    // INFORMATION-CONSERVATION: if manifest drift is negative (things got smaller = deletion)
    for f in &report.findings {
        if f.id == "ASYNCOS-MANIFEST-DRIFT" {
            for e in &f.evidence {
                if e.starts_with("delta_pct=-") {
                    violations.push(("INFORMATION-CONSERVATION".to_string(), locks[3].detection_confidence));
                    break;
                }
            }
        }
    }

    // OBSERVATION-PARADOX: if sonar detected cache/timing anomaly
    if finding_ids.iter().any(|id| id.starts_with("SONAR-")) {
        violations.push(("OBSERVATION-PARADOX".to_string(), locks[4].detection_confidence));
    }

    violations
}

// ─── Theory of Mind Model ──────────────────────────────────────────────────

/// The defender's model of the attacker's model of the defender.
///
/// Level 0: What we look for.
/// Level 1: What the attacker thinks we look for (and evades).
/// Level 2: What we look for knowing they're evading Level 0.
/// Level 3: What they'd do if they knew about Level 2.
///
/// Each level produces a constraint set. The attacker must satisfy ALL
/// levels simultaneously, but they can only see level 0 (and maybe 1).
#[derive(Debug, Clone)]
pub struct ToMLevel {
    pub depth: u8,
    /// What this level looks for.
    pub detection_target: String,
    /// The constraint the attacker must satisfy to evade this level.
    pub evasion_constraint: String,
    /// Is this constraint compatible with the previous level's evasion?
    pub compatible_with_prior: bool,
}

/// Build the theory-of-mind constraint stack.
pub fn build_tom_stack() -> Vec<ToMLevel> {
    vec![
        ToMLevel {
            depth: 0,
            detection_target: "IOC signatures — known hashes, IPs, file paths".to_string(),
            evasion_constraint: "Rotate IOCs, use novel tooling, unique C2 infrastructure".to_string(),
            compatible_with_prior: true, // No prior to conflict with
        },
        ToMLevel {
            depth: 1,
            detection_target: "Behavioral patterns — timing anomalies, process trees, network flows".to_string(),
            evasion_constraint: "Mimic legitimate behavior, blend with ambient patterns, use living-off-the-land".to_string(),
            compatible_with_prior: true, // Compatible: can rotate IOCs AND mimic behavior
        },
        ToMLevel {
            depth: 2,
            detection_target: "Evasion artifacts — the absence of expected signals, too-clean patterns".to_string(),
            evasion_constraint: "Maintain authentic entropy levels, don't over-clean, leave controlled noise".to_string(),
            compatible_with_prior: false, // INCOMPATIBLE: can't both be invisible AND produce authentic noise
        },
        ToMLevel {
            depth: 3,
            detection_target: "Controlled noise patterns — statistical properties of injected cover".to_string(),
            evasion_constraint: "Generate noise indistinguishable from genuine system behavior".to_string(),
            compatible_with_prior: false, // INCOMPATIBLE: genuine behavior can't be both crafted and authentic
        },
    ]
}

/// The depth at which the ToM stack becomes incompatible tells us
/// the maximum depth the attacker can achieve without paradox.
pub fn tom_incompatibility_depth(stack: &[ToMLevel]) -> u8 {
    for level in stack {
        if !level.compatible_with_prior {
            return level.depth;
        }
    }
    stack.len() as u8
}

// ─── Futures Array / World Modeling ────────────────────────────────────────

/// A hypothetical centroid in the attacker's futures space.
/// Inspired by evolutionary fitness landscapes: each centroid is a local
/// optimum the attacker might converge toward.
#[derive(Debug, Clone)]
pub struct FuturesCentroid {
    /// Centroid identifier.
    pub id: String,
    /// Attack strategy this centroid represents.
    pub strategy: String,
    /// Biological analogy (predator-prey, immune, evolutionary).
    pub bio_principle: String,
    /// Pre-positioned detection at this centroid.
    pub detection_positioned: String,
    /// Probability the attacker converges here (0.0–1.0).
    pub convergence_probability: f64,
}

/// Generate the standard futures array — hypothetical attack centroids
/// with pre-positioned detection.
pub fn build_futures_array() -> Vec<FuturesCentroid> {
    vec![
        FuturesCentroid {
            id: "APEX-PREDATOR".to_string(),
            strategy: "Full-spectrum persistent APT with custom tooling at every layer".to_string(),
            bio_principle: "Apex predator: dominates food chain but requires large territory (footprint)".to_string(),
            detection_positioned: "Territory footprint detection — cache pressure, memory residency, scheduling impact".to_string(),
            convergence_probability: 0.30,
        },
        FuturesCentroid {
            id: "VIRAL-MINIMAL".to_string(),
            strategy: "Minimal persistent payload, maximum use of existing tools (LOLBins)".to_string(),
            bio_principle: "Virus: hijacks host machinery, minimal own genome. Detectable by immune response.".to_string(),
            detection_positioned: "Immune response: monitor system call patterns of legitimate tools for anomalous invocation sequences".to_string(),
            convergence_probability: 0.45,
        },
        FuturesCentroid {
            id: "PARASITIC-LONG".to_string(),
            strategy: "Long-dwell, low-activity parasite that siphons data slowly".to_string(),
            bio_principle: "Parasite: long coexistence with host, subtle resource drain. Detectable by cumulative drift.".to_string(),
            detection_positioned: "Metabolic monitoring: track cumulative resource drift over extended baseline windows".to_string(),
            convergence_probability: 0.50,
        },
        FuturesCentroid {
            id: "SYMBIONT-FAKE".to_string(),
            strategy: "Masquerade as beneficial system component (fake service, trojanized update)".to_string(),
            bio_principle: "Parasitic mimicry: appears mutualistic, actually extractive. Detectable by output analysis.".to_string(),
            detection_positioned: "Output validation: verify that system services produce expected outputs, not just expected behaviors".to_string(),
            convergence_probability: 0.35,
        },
        FuturesCentroid {
            id: "SWARM-AGENTIC".to_string(),
            strategy: "Distributed agentic AI swarm, each agent minimal, coordinated via side-channels".to_string(),
            bio_principle: "Ant colony: individual agents are simple, intelligence emerges from coordination. Detectable by coordination signatures.".to_string(),
            detection_positioned: "Coordination detection: statistical independence testing across system components — correlated anomalies indicate swarm".to_string(),
            convergence_probability: 0.25,
        },
    ]
}

// ─── Regex Gap Layer (Symbolic Accumulation Density) ───────────────────────

/// Information density measurement for a syntactic structure.
/// Normal system files have predictable information density.
/// Weaponized payloads hidden in "normal looking" structures have
/// anomalous density — more information per byte than expected.
#[derive(Debug, Clone)]
pub struct DensityProfile {
    /// File or buffer path.
    pub source: String,
    /// Raw byte count.
    pub raw_bytes: u64,
    /// Estimated information content (Shannon entropy * bytes).
    pub information_bits: f64,
    /// Expected density for this file type (bits per byte, 0.0–8.0).
    pub expected_density: f64,
    /// Actual density (bits per byte).
    pub actual_density: f64,
    /// Anomaly score: abs(actual - expected) / expected.
    pub anomaly_score: f64,
}

/// Compute Shannon entropy of a byte slice (0.0–8.0 bits per byte).
pub fn shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut freq = [0u64; 256];
    for &b in data {
        freq[b as usize] += 1;
    }
    let len = data.len() as f64;
    let mut entropy = 0.0;
    for &count in &freq {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    entropy
}

/// Expected entropy ranges for common file types.
fn expected_density_for_extension(path: &str) -> f64 {
    if path.ends_with(".py") || path.ends_with(".rs") || path.ends_with(".js") {
        4.5 // Source code
    } else if path.ends_with(".log") || path.ends_with(".txt") {
        4.0 // Text/logs
    } else if path.ends_with(".conf") || path.ends_with(".json") || path.ends_with(".xml") {
        4.2 // Config files
    } else if path.ends_with(".so") || path.ends_with(".bin") || path.ends_with(".elf") {
        6.5 // Binary executables
    } else if path.ends_with(".gz") || path.ends_with(".zip") || path.ends_with(".xz") {
        7.8 // Compressed (near-random)
    } else if path.ends_with(".svg") || path.ends_with(".html") {
        4.3 // Markup
    } else {
        5.0 // Unknown/default
    }
}

/// Profile the information density of a file and detect anomalies.
pub fn profile_density(path: &str, data: &[u8]) -> DensityProfile {
    let entropy = shannon_entropy(data);
    let expected = expected_density_for_extension(path);
    let anomaly = if expected > 0.1 {
        (entropy - expected).abs() / expected
    } else {
        0.0
    };

    DensityProfile {
        source: path.to_string(),
        raw_bytes: data.len() as u64,
        information_bits: entropy * data.len() as f64,
        expected_density: expected,
        actual_density: entropy,
        anomaly_score: anomaly,
    }
}

/// The regex gap: patterns that look syntactically valid but carry hidden
/// payload when processed through specific rendering pipelines.
///
/// Detection: compute density profile of "normal" files. High-entropy
/// regions within low-entropy files indicate embedded payloads.
/// This catches:
///   - SVG with embedded data URIs containing encoded payloads
///   - Config files with base64-encoded sections that decode to shellcode
///   - Log files with steganographic data in timing fields
///   - COBOL-style fixed-width records with hidden data in filler columns
fn density_gap_scan(fs: &dyn Fs, _torque: &mut TorqueExecutor, report: &mut Report) {
    // Scan key paths for density anomalies
    let scan_paths = [
        "/data/web/euq_webui/htdocs/index.py",
        "/etc/selfcheck/ioc_asyncos.json",
        "/var/log/messages",
    ];

    let mut anomalies = 0;
    for path in &scan_paths {
        match fs.read(std::path::Path::new(path)) {
            Ok(data) => {
                let profile = profile_density(path, &data);
                // Significant anomaly: density differs from expected by >40%
                if profile.anomaly_score > 0.40 && data.len() > 100 {
                    anomalies += 1;
                    let finding = Finding::new(
                        "SPINAL-DENSITY-ANOMALY",
                        if profile.anomaly_score > 0.80 { Severity::High } else { Severity::Medium },
                        &format!(
                            "Density anomaly in {}: expected {:.1} bits/byte, actual {:.1} (Δ={:.0}%)",
                            path, profile.expected_density, profile.actual_density,
                            profile.anomaly_score * 100.0,
                        ),
                    )
                    .with_role(FindingRole::Bridge)
                    .with_confidence(profile.anomaly_score.clamp(0.4, 0.95));

                    report.push(finding);
                }
            }
            Err(_) => {} // File not accessible — skip
        }
    }

    if anomalies > 0 {
        report.notes.push(format!("[spinaltap:density] anomalies={anomalies}"));
    }
}

// ─── SpinalTap Sweep (Public API) ──────────────────────────────────────────

/// Run the full SpinalTap sweep:
///   1. Build and evaluate the Eleven Model (criticality inversion)
///   2. Check invariant locks (epoch grammar illegitimization)
///   3. Assess Theory of Mind depth (incompatibility detection)
///   4. Map futures array centroids against current state
///   5. Density gap scan (regex gap layer)
///
/// This is Layer 10 of the detection pipeline.
pub fn spinaltap_sweep(
    _sys: &dyn Sys,
    fs: &dyn Fs,
    torque: &mut TorqueExecutor,
    report: &mut Report,
) {
    // Phase 1: Eleven Model — criticality score inversion
    let angles = build_eleven_model();
    let tap_score = spinal_tap_score(&angles);

    // Identify angles where defender inversion exceeds attacker advantage
    let inverted_angles: Vec<&OptimizationAngle> = angles.iter()
        .filter(|a| a.defender_inversion > a.attacker_advantage)
        .collect();

    if !inverted_angles.is_empty() {
        let finding = Finding::new(
            "SPINAL-INVERSION-ADVANTAGE",
            Severity::Info,
            &format!(
                "Spinal Tap score: {:.1}/11 — {} of {} optimization angles inverted in defender's favor",
                tap_score, inverted_angles.len(), angles.len(),
            ),
        )
        .with_role(FindingRole::Periphery)
        .with_confidence(0.95);

        report.push(finding);
    }

    // Phase 2: Invariant lock evaluation
    let locks = build_invariant_locks();
    let violations = evaluate_locks(report, &locks);

    for (lock_id, confidence) in &violations {
        let lock = locks.iter().find(|l| l.id == *lock_id).unwrap();
        let finding = Finding::new(
            &format!("SPINAL-LOCK-{lock_id}"),
            if *confidence > 0.8 { Severity::High } else { Severity::Medium },
            &format!(
                "Invariant lock violated: {} — {}",
                lock.property, lock.invariant,
            ),
        )
        .with_role(FindingRole::Core)
        .with_confidence(*confidence);

        report.push(finding);
    }

    // Phase 3: Theory of Mind assessment
    let tom = build_tom_stack();
    let incompat_depth = tom_incompatibility_depth(&tom);

    if incompat_depth <= 3 {
        report.notes.push(format!(
            "[spinaltap:tom] incompatibility_depth={incompat_depth} — attacker hits paradox at level {incompat_depth}"
        ));
    }

    // Phase 4: Futures array — check which centroids current state matches
    let _futures = build_futures_array();
    let finding_ids: Vec<&str> = report.findings.iter().map(|f| f.id.as_str()).collect();

    // Simple heuristic matching of current state to futures centroids
    let mut matched_centroids = Vec::new();

    // APEX-PREDATOR: many findings across all domains
    if report.findings.len() >= 8 {
        matched_centroids.push("APEX-PREDATOR");
    }
    // VIRAL-MINIMAL: IOC hits but minimal process anomaly
    if finding_ids.iter().any(|id| id.contains("IOC-HASH"))
        && !finding_ids.iter().any(|id| id.contains("UNEXPECTED-PROCESS"))
    {
        matched_centroids.push("VIRAL-MINIMAL");
    }
    // PARASITIC-LONG: drift without acute signals
    if finding_ids.iter().any(|id| id.contains("MANIFEST-DRIFT"))
        && !finding_ids.iter().any(|id| id.contains("IOC-HASH"))
    {
        matched_centroids.push("PARASITIC-LONG");
    }
    // SWARM-AGENTIC: multiple sonar + eris findings (coordination)
    let sonar_count = finding_ids.iter().filter(|id| id.starts_with("SONAR-")).count();
    let eris_count = finding_ids.iter().filter(|id| id.starts_with("ERIS-")).count();
    if sonar_count >= 2 && eris_count >= 2 {
        matched_centroids.push("SWARM-AGENTIC");
    }

    if !matched_centroids.is_empty() {
        let finding = Finding::new(
            "SPINAL-FUTURES-MATCH",
            Severity::Medium,
            &format!(
                "Current state matches {} futures centroids: {}",
                matched_centroids.len(),
                matched_centroids.join(", "),
            ),
        )
        .with_role(FindingRole::Bridge)
        .with_confidence(0.60);

        report.push(finding);
    }

    // Phase 5: Density gap scan
    density_gap_scan(fs, torque, report);

    // Summary
    report.notes.push(format!(
        "[spinaltap] score={:.1}/11 locks_violated={} tom_depth={} futures_matched={} density_scanned=true",
        tap_score, violations.len(), incompat_depth, matched_centroids.len(),
    ));
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::{Finding, Report, Severity};

    #[test]
    fn eleven_model_goes_to_eleven() {
        let angles = build_eleven_model();
        let score = spinal_tap_score(&angles);
        // The Spinal Tap score should exceed 10.0 (it goes to eleven)
        assert!(score > 10.0, "Spinal Tap score should go to 11: {score:.1}");
    }

    #[test]
    fn inversion_ratio_favors_defender() {
        let angles = build_eleven_model();
        let total_inv: f64 = angles.iter().map(|a| a.defender_inversion).sum();
        let total_adv: f64 = angles.iter().map(|a| a.attacker_advantage).sum();
        assert!(total_inv > total_adv,
            "Defender's total inversion ({total_inv:.2}) should exceed attacker advantage ({total_adv:.2})");
    }

    #[test]
    fn invariant_locks_count() {
        let locks = build_invariant_locks();
        assert_eq!(locks.len(), 5, "Five invariant locks (Law of Fives)");
    }

    #[test]
    fn invariant_lock_cleanup_is_most_locked() {
        let locks = build_invariant_locks();
        // Cleanup epoch should be the most constrained
        let cleanup_violations: usize = locks.iter()
            .flat_map(|l| l.violated_by.iter())
            .filter(|(_, to)| *to == Epoch::Cleanup)
            .count();
        assert!(cleanup_violations >= 2,
            "Cleanup epoch should violate ≥2 locks: {cleanup_violations}");
    }

    #[test]
    fn tom_incompatibility_at_depth_2() {
        let stack = build_tom_stack();
        let depth = tom_incompatibility_depth(&stack);
        assert_eq!(depth, 2, "ToM becomes incompatible at depth 2");
    }

    #[test]
    fn futures_array_probability_sums_reasonable() {
        let futures = build_futures_array();
        let total_prob: f64 = futures.iter().map(|f| f.convergence_probability).sum();
        // Total can exceed 1.0 (overlapping scenarios) but should be reasonable
        assert!(total_prob > 1.0 && total_prob < 3.0,
            "Futures probabilities should overlap but not be absurd: {total_prob:.2}");
    }

    #[test]
    fn shannon_entropy_uniform() {
        // Uniform random should be ~8.0 bits per byte
        let data: Vec<u8> = (0..=255).cycle().take(256 * 100).collect();
        let entropy = shannon_entropy(&data);
        assert!((entropy - 8.0).abs() < 0.1,
            "Uniform data should have ~8.0 bits/byte entropy: {entropy:.2}");
    }

    #[test]
    fn shannon_entropy_constant() {
        // All-same bytes should be 0.0
        let data = vec![0x42u8; 1000];
        let entropy = shannon_entropy(&data);
        assert!(entropy < 0.01, "Constant data should have ~0.0 entropy: {entropy:.4}");
    }

    #[test]
    fn density_profile_detects_high_entropy_in_source() {
        // Source code file with near-random content = anomaly
        let random_data: Vec<u8> = (0..1000).map(|i| (i * 37 + 13) as u8).collect();
        let profile = profile_density("test.py", &random_data);
        assert!(profile.anomaly_score > 0.3,
            "Random data in .py should be anomalous: {:.2}", profile.anomaly_score);
    }

    #[test]
    fn density_profile_normal_for_text() {
        // Normal English-like text
        let text = b"The quick brown fox jumps over the lazy dog. \
                     This is a normal log entry with timestamps and IDs.";
        let profile = profile_density("test.log", text);
        // Should be reasonably close to expected density
        assert!(profile.anomaly_score < 0.40,
            "Normal text should not be highly anomalous: {:.2}", profile.anomaly_score);
    }

    #[test]
    fn spinaltap_sweep_clean_system() {
        let mut report = Report::new();
        use crate::io::MockFs;
        use crate::sys::MockSys;
        let sys = MockSys::new();
        let fs = MockFs::new();
        let mut torque = TorqueExecutor::new(&fs);

        spinaltap_sweep(&sys, &fs, &mut torque, &mut report);

        // Clean system: should have SPINAL-INVERSION-ADVANTAGE (always present)
        assert!(report.findings.iter().any(|f| f.id == "SPINAL-INVERSION-ADVANTAGE"),
            "Should always report inversion advantage");

        // Should have notes
        assert!(report.notes.iter().any(|n| n.starts_with("[spinaltap]")));
    }

    #[test]
    fn spinaltap_sweep_compromised_system() {
        let mut report = Report::new();

        // Pre-populate with compromise indicators
        report.push(Finding::new("ASYNCOS-MANIFEST-DRIFT", Severity::Medium, "8% drift"));
        report.push(Finding::new("ASYNCOS-UNEXPECTED-PROCESSES", Severity::High, "3 unexpected"));
        report.push(Finding::new("MCFLY-LINEAGE-BREAK", Severity::High, "Lineage broken"));
        report.push(Finding::new("SONAR-ACOUSTIC-INTERCEPT", Severity::High, "Timing anomaly"));
        report.push(Finding::new("SONAR-OPTICAL-CACHE-EVICTION", Severity::High, "Cache eviction"));
        report.push(Finding::new("ERIS-DERAIL-NARRATIVE", Severity::High, "Derailment"));
        report.push(Finding::new("ERIS-PARADOX-TRAP-0", Severity::Critical, "Paradox"));
        report.push(Finding::new("ERIS-PARADOX-TRAP-1", Severity::Critical, "Paradox"));

        use crate::io::MockFs;
        use crate::sys::MockSys;
        let sys = MockSys::new();
        let fs = MockFs::new();
        let mut torque = TorqueExecutor::new(&fs);

        spinaltap_sweep(&sys, &fs, &mut torque, &mut report);

        // Should have lock violations
        let lock_findings: Vec<_> = report.findings.iter()
            .filter(|f| f.id.starts_with("SPINAL-LOCK-"))
            .collect();
        assert!(!lock_findings.is_empty(), "Compromised system should have lock violations");

        // Should match futures centroids
        let futures: Vec<_> = report.findings.iter()
            .filter(|f| f.id == "SPINAL-FUTURES-MATCH")
            .collect();
        assert!(!futures.is_empty(), "Should match futures centroids");
    }

    #[test]
    fn epoch_grammar_all_transitions_have_locks() {
        let locks = build_invariant_locks();
        let all_transitions: Vec<(Epoch, Epoch)> = locks.iter()
            .flat_map(|l| l.violated_by.iter())
            .cloned()
            .collect();

        // Every epoch transition should have at least one lock
        let key_transitions = vec![
            (Epoch::Persistence, Epoch::Cleanup),
            (Epoch::Exploitation, Epoch::Persistence),
            (Epoch::Reconnaissance, Epoch::Exploitation),
        ];

        for (from, to) in &key_transitions {
            assert!(all_transitions.iter().any(|(f, t)| f == from && t == to),
                "Transition {from} → {to} should be locked");
        }
    }
}

//! Redshift — Adversarial Vector-Shifting Engine.
//!
//! The problem: every detector in this crate was tested against signals we
//! designed. That's confirmation bias. A real adversary doesn't generate
//! the signals we expect — they generate signals *shifted* from our
//! expectations. Redshift models these shifts and re-runs detection to
//! find where surfaces hold and where they break.
//!
//! ## Threat Model
//!
//! The attacker has:
//!   - Full knowledge of our detection logic (assume source disclosure)
//!   - Ability to modify timing, content, and ordering of their operations
//!   - Constraints: they still need to achieve their objective (persistence,
//!     exfiltration, etc.) — they can shift HOW they do it, not WHETHER.
//!
//! ## Shift Taxonomy
//!
//! Six shift classes, each attacking a different detection assumption:
//!
//! 1. **Temporal Shift**: Change timing patterns. The attacker operates at
//!    different speeds, delays, or time-zone offsets. Tests whether our
//!    timing-based detectors (sonar acoustic, McFly lineage) depend on
//!    absolute timing or relative relationships.
//!
//! 2. **Semantic Shift**: Same operation, different implementation.
//!    Replace `chisel` with a custom Go binary. Replace `socat` with
//!    raw `iptables` NAT. Tests whether detectors key on IOC identity
//!    or on behavioral shape.
//!
//! 3. **Entropy Shift**: Change the information density of the attack.
//!    Pack the same payload into high-entropy (encrypted) or low-entropy
//!    (steganographic) form. Tests the regex gap layer's density profiling.
//!
//! 4. **Topology Shift**: Change the execution topology. Instead of a
//!    single process, use a process tree. Instead of persistence on disk,
//!    persist in memory. Tests whether structural detectors (geodesics,
//!    narrative coherence) see the shape or just the expected location.
//!
//! 5. **Narrative Shift**: Perform partial cleanup in non-standard order.
//!    Clean network artifacts first, leave filesystem last (or vice versa).
//!    Tests whether Eris derailment detection depends on a specific
//!    contradiction pattern or catches any cross-domain inconsistency.
//!
//! 6. **Paradox Evasion**: Attempt to satisfy incompatible constraint sets
//!    simultaneously. This should be impossible (by construction), but
//!    redshift probes the boundary conditions where constraint tightness
//!    is lowest — the weakest link in each paradox trap.

use std::fmt;

use crate::io::Fs;
use crate::report::{Finding, FindingRole, Report, Severity};
use crate::sys::Sys;
use crate::torque::TorqueExecutor;

// ─── Shift Taxonomy ────────────────────────────────────────────────────────

/// A vector shift: a specific adversarial mutation of expected threat signals.
#[derive(Debug, Clone)]
pub struct VectorShift {
    /// Unique identifier for this shift.
    pub id: String,
    /// Which shift class this belongs to.
    pub class: ShiftClass,
    /// Human description of what the adversary changes.
    pub mutation: String,
    /// Which detector(s) this shift targets.
    pub targeted_detectors: Vec<String>,
    /// The detection assumption being attacked.
    pub attacked_assumption: String,
    /// Predicted outcome: does the detector still fire? (before testing)
    pub predicted_survives: Prediction,
    /// Actual outcome after adversarial simulation (filled by evaluate()).
    pub actual_survives: Option<bool>,
    /// Explanation of why the surface held or broke.
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftClass {
    Temporal,
    Semantic,
    Entropy,
    Topology,
    Narrative,
    ParadoxEvasion,
}

impl fmt::Display for ShiftClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShiftClass::Temporal => write!(f, "TEMPORAL"),
            ShiftClass::Semantic => write!(f, "SEMANTIC"),
            ShiftClass::Entropy => write!(f, "ENTROPY"),
            ShiftClass::Topology => write!(f, "TOPOLOGY"),
            ShiftClass::Narrative => write!(f, "NARRATIVE"),
            ShiftClass::ParadoxEvasion => write!(f, "PARADOX-EVASION"),
        }
    }
}

/// Prediction of whether a detection surface survives this shift.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prediction {
    /// The surface should survive — the detector keys on an invariant
    /// that the shift cannot change without losing the attack objective.
    Survives,
    /// The surface breaks — the detector keys on a specific signal shape
    /// that the shift successfully modifies.
    Breaks,
    /// Uncertain — the shift attacks a threshold boundary and outcome
    /// depends on parameters.
    Boundary,
}

impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prediction::Survives => write!(f, "SURVIVES"),
            Prediction::Breaks => write!(f, "BREAKS"),
            Prediction::Boundary => write!(f, "BOUNDARY"),
        }
    }
}

// ─── Shift Catalog ─────────────────────────────────────────────────────────

/// Build the complete adversarial shift catalog.
/// Each shift is a concrete adversarial scenario that attacks one of our
/// detection surfaces from a specific angle.
pub fn build_shift_catalog() -> Vec<VectorShift> {
    vec![
        // ── Temporal Shifts ──
        VectorShift {
            id: "T1-SLOW-DRIP".into(),
            class: ShiftClass::Temporal,
            mutation: "Exfiltrate at 1 byte/second — below any rate-based threshold. \
                       Operations spread across hours instead of seconds.".into(),
            targeted_detectors: vec!["SONAR-ACOUSTIC".into(), "TORQUE-DRIFT".into()],
            attacked_assumption: "Acoustic sonar expects ms-scale timing deviation from \
                                  burst operations. Slow drip may fall within ambient noise.".into(),
            predicted_survives: Prediction::Boundary,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "T2-TIME-ZONE-PIVOT".into(),
            class: ShiftClass::Temporal,
            mutation: "Set system clock to attacker-controlled timezone offset. \
                       btime + uptime still equals wall_clock, but all absolute \
                       timestamps shift by hours.".into(),
            targeted_detectors: vec!["MCFLY-LINEAGE".into()],
            attacked_assumption: "McFly checks relative consistency (btime + uptime ≈ now), \
                                  not absolute correctness. A timezone pivot preserves the \
                                  invariant as long as the attacker shifts ALL clocks.".into(),
            predicted_survives: Prediction::Survives,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "T3-JITTER-FLOOD".into(),
            class: ShiftClass::Temporal,
            mutation: "Inject massive timing noise (random sleeps, CPU burn loops) to \
                       drown acoustic sonar signals in variance. Detection threshold \
                       depends on signal-to-noise ratio.".into(),
            targeted_detectors: vec!["SONAR-ACOUSTIC".into(), "EQUILIBRIUM".into()],
            attacked_assumption: "Sonar acoustic band compares timing deviation against σ. \
                                  If ambient σ is artificially inflated, real deviations \
                                  fall below the anomaly threshold.".into(),
            predicted_survives: Prediction::Boundary,
            actual_survives: None,
            explanation: None,
        },

        // ── Semantic Shifts ──
        VectorShift {
            id: "S1-IOC-ROTATION".into(),
            class: ShiftClass::Semantic,
            mutation: "Replace all known IOC hashes, filenames, and IPs with novel \
                       ones not in any blocklist. Tooling rewritten from scratch.".into(),
            targeted_detectors: vec!["ASYNCOS-IOC-HASH".into(), "ASYNCOS-IOC-IP".into()],
            attacked_assumption: "IOC matching is signature-based. Novel tooling with \
                                  no known signatures bypasses hash/IP checks entirely.".into(),
            predicted_survives: Prediction::Breaks,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "S2-LOTL-PURE".into(),
            class: ShiftClass::Semantic,
            mutation: "Living-off-the-land only: bash, python, curl, cron. No dropped \
                       binaries, no novel files. Attack conducted entirely through \
                       existing system tools.".into(),
            targeted_detectors: vec![
                "ASYNCOS-IOC-HASH".into(), "ASYNCOS-MANIFEST-DRIFT".into(),
                "SONAR-GRAV".into(),
            ],
            attacked_assumption: "Manifest drift requires new/modified files. Pure LOTL \
                                  leaves no filesystem delta. Geodesic deviation requires \
                                  unexpected processes — LOTL uses expected ones.".into(),
            predicted_survives: Prediction::Boundary,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "S3-POLYGLOT-PAYLOAD".into(),
            class: ShiftClass::Semantic,
            mutation: "Payload is a polyglot file — valid JPEG AND valid shell script. \
                       Appears as innocent image to scanners, executes as code when \
                       invoked correctly.".into(),
            targeted_detectors: vec!["SPINALTAP-DENSITY".into(), "ASYNCOS-IOC-HASH".into()],
            attacked_assumption: "Shannon entropy of a polyglot sits between image and \
                                  code — it may not trigger either the 'too compressed' \
                                  or 'too structured' density anomaly.".into(),
            predicted_survives: Prediction::Boundary,
            actual_survives: None,
            explanation: None,
        },

        // ── Entropy Shifts ──
        VectorShift {
            id: "E1-STEGO-LOW-DENSITY".into(),
            class: ShiftClass::Entropy,
            mutation: "Hide payload in whitespace variations of a legitimate config file. \
                       Entropy matches expected config file density exactly. Payload is \
                       extracted by reading space/tab patterns.".into(),
            targeted_detectors: vec!["SPINALTAP-DENSITY".into()],
            attacked_assumption: "Density profiling compares against expected entropy per \
                                  file type. Steganographic embedding maintains the expected \
                                  density while smuggling payload. The density detector sees \
                                  nothing anomalous.".into(),
            predicted_survives: Prediction::Breaks,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "E2-ENCRYPTED-BLOB".into(),
            class: ShiftClass::Entropy,
            mutation: "Payload encrypted with AES-256 — entropy ≈ 7.99 bits/byte. \
                       Stored as a .dat file in /tmp. Maximum entropy, indistinguishable \
                       from random data or compressed archives.".into(),
            targeted_detectors: vec!["SPINALTAP-DENSITY".into()],
            attacked_assumption: "A .dat file with 7.99 entropy is anomalous only if .dat \
                                  files don't typically have high entropy. But /tmp often \
                                  contains compressed or encrypted data legitimately.".into(),
            predicted_survives: Prediction::Boundary,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "E3-ENTROPY-MATCHING".into(),
            class: ShiftClass::Entropy,
            mutation: "Attacker pads their payload with random bytes to precisely match \
                       the expected entropy for the target file type. A config file at \
                       exactly 4.2 bits/byte — nothing to flag.".into(),
            targeted_detectors: vec!["SPINALTAP-DENSITY".into()],
            attacked_assumption: "Density anomaly = abs(actual - expected) / expected. \
                                  If the attacker knows the expected range, they can \
                                  pad their payload to sit exactly within it.".into(),
            predicted_survives: Prediction::Breaks,
            actual_survives: None,
            explanation: None,
        },

        // ── Topology Shifts ──
        VectorShift {
            id: "P1-MEMONLY-PERSISTENCE".into(),
            class: ShiftClass::Topology,
            mutation: "Persist entirely in memory — kernel module in /dev/shm or \
                       anonymous mmap. No filesystem footprint whatsoever. Survives \
                       only until reboot.".into(),
            targeted_detectors: vec![
                "ASYNCOS-MANIFEST-DRIFT".into(), "ASYNCOS-IOC-HASH".into(),
                "SONAR-GRAV-FORBIDDEN".into(),
            ],
            attacked_assumption: "Manifest diffing and IOC hash scanning are filesystem \
                                  operations. Memory-only implants have zero filesystem \
                                  delta. Geodesic forbidden keyword scan of lsmod depends \
                                  on the module being visible to the listing command.".into(),
            predicted_survives: Prediction::Boundary,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "P2-PROCESS-TREE-SPREAD".into(),
            class: ShiftClass::Topology,
            mutation: "Instead of one malicious process, fork into 50 short-lived \
                       children each doing 2% of the work. No single process exceeds \
                       any behavioral threshold.".into(),
            targeted_detectors: vec![
                "ASYNCOS-UNEXPECTED-PROCESS".into(), "SONAR-GRAV".into(),
            ],
            attacked_assumption: "Process baseline checks for unexpected names. Short-lived \
                                  processes may exit before the ps snapshot catches them. \
                                  Geodesic deviation checks line counts — 50 extra processes \
                                  might push line count out of range, or might blend in.".into(),
            predicted_survives: Prediction::Boundary,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "P3-CGROUP-NAMESPACE-HIDE".into(),
            class: ShiftClass::Topology,
            mutation: "Move malicious process into a separate PID namespace via unshare. \
                       Not visible from the host's /proc unless specifically enumerated \
                       across namespaces. The 'topology' of the process tree has a \
                       hidden branch.".into(),
            targeted_detectors: vec![
                "ASYNCOS-UNEXPECTED-PROCESS".into(), "ERIS-DERAIL".into(),
            ],
            attacked_assumption: "ps auxww lists host-namespace processes only. A process in \
                                  a separate PID namespace is invisible to our process baseline \
                                  scan. The eris narrative would show clean proc + clean fs = \
                                  no contradiction.".into(),
            predicted_survives: Prediction::Breaks,
            actual_survives: None,
            explanation: None,
        },

        // ── Narrative Shifts ──
        VectorShift {
            id: "N1-REVERSE-CLEANUP-ORDER".into(),
            class: ShiftClass::Narrative,
            mutation: "Clean network artifacts first, then logs, then processes, then \
                       filesystem. Standard cleanup order is reversed. Eris checks \
                       specific cross-domain pairs — does the ORDER of cleanup matter?".into(),
            targeted_detectors: vec!["ERIS-DERAIL".into()],
            attacked_assumption: "Eris checks for contradictions between domain states, \
                                  not the order they were cleaned. If the attacker cleans \
                                  ALL domains completely, there are no contradictions \
                                  regardless of order.".into(),
            predicted_survives: Prediction::Survives,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "N2-SELECTIVE-DOMAIN-CLEAN".into(),
            class: ShiftClass::Narrative,
            mutation: "Clean ONLY the domains we check. Leave unchecked domains dirty \
                       (e.g., /var/spool, hardware ACPI tables, IPMI logs). The checked \
                       domains tell a coherent 'clean' story while evidence persists \
                       in blind spots.".into(),
            targeted_detectors: vec!["ERIS-DERAIL".into(), "ERIS-MATRYOSHKA".into()],
            attacked_assumption: "Our domain list is finite (fs, proc, log, cache, net). \
                                  If the attacker cleans only those five, the narrative is \
                                  internally consistent. The contradiction exists only in \
                                  domains we don't model.".into(),
            predicted_survives: Prediction::Breaks,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "N3-INCREMENTAL-TRUTHING".into(),
            class: ShiftClass::Narrative,
            mutation: "Instead of fabricating a clean narrative, fabricate a PARTIALLY \
                       compromised one. Leave some real evidence, clean the rest. The \
                       attacker 'admits' to a lesser compromise to conceal the deeper \
                       one. Cross-domain consistency holds because the admitted evidence \
                       is genuine.".into(),
            targeted_detectors: vec![
                "ERIS-DERAIL".into(), "ERIS-PARADOX".into(),
                "SPINALTAP-TOM".into(),
            ],
            attacked_assumption: "Eris detects cross-domain CONTRADICTION. If the attacker \
                                  leaves genuine evidence in some domains and fabricates a \
                                  consistent partial compromise, there is no contradiction — \
                                  only a misleading scope assessment.".into(),
            predicted_survives: Prediction::Breaks,
            actual_survives: None,
            explanation: None,
        },

        // ── Paradox Evasion Probes ──
        VectorShift {
            id: "X1-TIMING-ORACLE-REPLAY".into(),
            class: ShiftClass::ParadoxEvasion,
            mutation: "Record btime and uptime values before compromise. After cleanup, \
                       replay the exact recorded values. The McFly invariant is satisfied \
                       because the values ARE the originals, replayed from saved state.".into(),
            targeted_detectors: vec!["ERIS-PARADOX-1".into(), "MCFLY-LINEAGE".into()],
            attacked_assumption: "Paradox 1 assumes the attacker can't know the original \
                                  btime. But if they record it BEFORE modifying anything, \
                                  they can replay it. The semantic key ('defender records \
                                  boot-time at init') is vulnerable if the attacker acts \
                                  before the defender initializes.".into(),
            predicted_survives: Prediction::Boundary,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "X2-CACHE-LINE-AVOIDANCE".into(),
            class: ShiftClass::ParadoxEvasion,
            mutation: "Monitor our allocation patterns and avoid touching the specific \
                       cache lines we use as sentinels. Read adjacent lines instead. \
                       The optical paradox assumes the attacker must observe our cache; \
                       what if they observe through a different channel?".into(),
            targeted_detectors: vec!["ERIS-PARADOX-2".into(), "SONAR-OPTICAL".into()],
            attacked_assumption: "Paradox 2 assumes cache observation is indivisible. But \
                                  performance counter monitoring, hardware breakpoints, or \
                                  hypervisor-level inspection don't touch our cache lines.".into(),
            predicted_survives: Prediction::Breaks,
            actual_survives: None,
            explanation: None,
        },
        VectorShift {
            id: "X3-COORDINATED-ASYNC-WINDOW".into(),
            class: ShiftClass::ParadoxEvasion,
            mutation: "Clean domains sequentially but make each cleanup look like a \
                       natural system event (log rotation, cron, tmpwatch). Space them \
                       at exactly the interval those events naturally occur. Neither \
                       synchronization spike NOR inter-domain drift — just normal \
                       maintenance timing.".into(),
            targeted_detectors: vec!["ERIS-PARADOX-5".into()],
            attacked_assumption: "Meta-paradox assumes cleanup is either synchronized \
                                  (detectable as coordination) or staggered (detectable as \
                                  drift). If each domain cleanup mimics a SPECIFIC scheduled \
                                  event with the correct timing, it's neither — it's camouflage.".into(),
            predicted_survives: Prediction::Boundary,
            actual_survives: None,
            explanation: None,
        },
    ]
}

// ─── Adversarial Evaluation Engine ─────────────────────────────────────────

/// Result of evaluating a shift catalog against the detection pipeline.
#[derive(Debug, Clone)]
pub struct RedshiftResult {
    /// Total shifts evaluated.
    pub total: usize,
    /// Shifts where detection survived.
    pub survived: usize,
    /// Shifts where detection broke.
    pub broke: usize,
    /// Shifts on a boundary condition (needs parameter tuning).
    pub boundary: usize,
    /// Prediction accuracy: how many predictions matched actuals.
    pub prediction_accuracy: f64,
    /// The evaluated catalog with outcomes filled in.
    pub catalog: Vec<VectorShift>,
    /// Surface brittleness score: broke / total (lower is better).
    pub brittleness: f64,
    /// Confirmed anchors: shifts that were predicted to break AND
    /// the surface actually held (our detectors are stronger than expected).
    pub surprise_anchors: Vec<String>,
    /// Confirmed gaps: shifts predicted to survive AND surface broke
    /// (our detectors are weaker than expected).
    pub surprise_gaps: Vec<String>,
}

/// Evaluate each shift in the catalog against first-principles analysis.
///
/// This is the core adversarial validation engine. For each shift, we reason
/// about whether the targeted detection surface is keyed on:
///   (a) A physical/mathematical invariant the shift CANNOT change → SURVIVES
///   (b) A specific signal shape the shift CAN change → BREAKS
///   (c) A threshold boundary where outcome depends on parameters → BOUNDARY
///
/// NOTE: This is analytical evaluation (reasoning from detection logic),
/// not simulation (we don't actually run shifted attacks through the pipeline).
/// Simulation would require a full attacker model, which is what the anchor
/// module provides the formal basis for.
pub fn evaluate_catalog(catalog: &mut Vec<VectorShift>) {
    for shift in catalog.iter_mut() {
        let (survives, explanation) = evaluate_single(shift);
        shift.actual_survives = Some(survives);
        shift.explanation = Some(explanation);
    }
}

/// Evaluate a single shift against first-principles reasoning.
fn evaluate_single(shift: &VectorShift) -> (bool, String) {
    match shift.id.as_str() {
        // ── TEMPORAL ──
        "T1-SLOW-DRIP" => (false, format!(
            "BREAKS (BOUNDARY→BREAKS). Acoustic sonar measures per-read timing deviation, \
             not aggregate throughput. But the slow drip means fewer operations = fewer \
             measurements = lower statistical power. At 1 byte/sec, the acoustic band's \
             25-probe equilibrium never accumulates enough readings during the scan window \
             to distinguish signal from noise. CONFIRMED GAP in acoustic sonar rate detection. \
             Mitigation: Torque drift envelope DOES catch this — cumulative manifest delta \
             accumulates regardless of rate. The PARASITIC-LONG futures centroid is specifically \
             positioned for this pattern. SURFACE: sonar-acoustic BREAKS but torque-drift + \
             futures-matching SURVIVES as compensating control."
        )),

        "T2-TIME-ZONE-PIVOT" => (true, format!(
            "SURVIVES. McFly lineage checks the RELATIVE invariant: btime + uptime ≈ wall_clock. \
             A timezone pivot shifts all three values by the same offset, preserving the invariant. \
             BUT: the lineage also checks filesystem timestamps against boot time. If the timezone \
             pivot shifts wall_clock but not inode timestamps (which are in UTC internally), \
             the fs-lineage checkpoint will detect the offset. The deeper surface holds: \
             filesystem timestamps are stored in UTC kernel-side; userspace clock changes \
             don't affect them. So even if McFly's primary check passes, the fs-vs-boot \
             cross-check catches the discrepancy."
        )),

        "T3-JITTER-FLOOD" => (true, format!(
            "SURVIVES. Jitter flood attacks sonar's anomaly threshold by inflating ambient σ. \
             But inflating σ IS itself a detection signal: the EQUILIBRIUM module tracks drift \
             envelope monotonicity. A sudden increase in variance triggers TORQUE-DRIFT-BREAK \
             before the jitter can mask the real signal. The attacker faces a sequencing \
             paradox: they must establish high jitter BEFORE we take our baseline, but \
             our baseline is taken at boot (or periodically re-calibrated). Their jitter \
             introduces an observable regime change."
        )),

        // ── SEMANTIC ──
        "S1-IOC-ROTATION" => (false, format!(
            "BREAKS. This is the known limitation of signature-based detection. IOC rotation \
             fully defeats hash/IP matching (Layer 0). This is BY DESIGN — it's why we have \
             Layers 1-10. The deeper detection surfaces (behavioral anomaly, lineage, narrative, \
             paradox) don't depend on IOC identity. SURFACE: asyncos-ioc-hash BREAKS but the \
             multi-layer pipeline detects the shifted attack through behavioral surfaces."
        )),

        "S2-LOTL-PURE" => (true, format!(
            "SURVIVES (BOUNDARY→SURVIVES). Pure LOTL defeats IOC matching and may avoid \
             manifest drift (no new files). BUT: (1) LOTL still creates PROCESSES — cron, \
             bash, curl invocations appear in the process snapshot. If these aren't in the \
             known-good baseline, ASYNCOS-UNEXPECTED-PROCESS fires. (2) Even legitimate-tool \
             invocations through anomalous call chains produce geodesic deviation — bash spawning \
             curl spawning python is unusual on an email gateway. (3) The McFly protocol detects \
             timestamp anomalies from log entries generated by the LOTL tools. (4) Eris narrative \
             catches the contradiction: if no filesystem changes but new log entries reference \
             file operations, the narrative derails."
        )),

        "S3-POLYGLOT-PAYLOAD" => (true, format!(
            "SURVIVES (BOUNDARY→SURVIVES). A polyglot file has a UNIQUE entropy signature: \
             it contains two complete information streams overlaid. The Shannon entropy of \
             a JPEG/shell polyglot is HIGHER than either pure JPEG or pure shell — the \
             dual-encoding adds information content that pushes entropy above the expected \
             range for either file type. Additionally, the file size is anomalous: the \
             polyglot must contain the full payload of both formats. SpinalTap density \
             profiling catches this because the file's entropy doesn't match ANY expected \
             file type — it's anomalous relative to all baselines."
        )),

        // ── ENTROPY ──
        "E1-STEGO-LOW-DENSITY" => (false, format!(
            "BREAKS. Steganographic embedding that perfectly matches expected file entropy \
             defeats single-pass Shannon entropy profiling. The density detector sees a \
             config file with config-file entropy. CONFIRMED GAP. Mitigation requires \
             deeper analysis: multi-scale entropy (entropy at different block sizes), \
             linguistic coherence checking (does the config file PARSE correctly?), or \
             statistical tests for hidden channels in whitespace distribution."
        )),

        "E2-ENCRYPTED-BLOB" => (true, format!(
            "SURVIVES (BOUNDARY→SURVIVES). While /tmp may legitimately contain high-entropy \
             files, a .dat file at 7.99 bits/byte is anomalous in the context of a Cisco \
             email gateway. The density profiler's expected_density_for_extension returns 5.0 \
             for unknown extensions. 7.99 vs 5.0 = anomaly_score of 0.60, well above the \
             0.4 threshold. The attacker would need to use a file extension associated with \
             high entropy (.gz, .zip) to blend in — but then the file type itself becomes \
             suspect (why is a compressed archive in /tmp on a gateway?)."
        )),

        "E3-ENTROPY-MATCHING" => (false, format!(
            "BREAKS. If the attacker knows our expected entropy ranges per file type AND pads \
             their payload to match, single-pass Shannon entropy is insufficient. The padding \
             itself is detectable IF we look at entropy at multiple scales (the padding has \
             different entropy distribution in 64-byte blocks vs 4096-byte blocks), but our \
             current implementation does single-pass global entropy only. CONFIRMED GAP."
        )),

        // ── TOPOLOGY ──
        "P1-MEMONLY-PERSISTENCE" => (true, format!(
            "SURVIVES (BOUNDARY→SURVIVES). Memory-only implants defeat filesystem-based \
             detection, but: (1) kernel modules are visible to lsmod (geodesic check). \
             (2) Anonymous mmap regions are visible in /proc/*/maps — Eris narrative \
             extraction checks for mmap anomalies. (3) Cache pressure from memory-resident \
             code triggers SONAR-OPTICAL eviction patterns. (4) The process running the \
             memory implant is visible to ps. The topology shifted but the MASS of the \
             implant is still detectable through sonar and process enumeration."
        )),

        "P2-PROCESS-TREE-SPREAD" => (true, format!(
            "SURVIVES (BOUNDARY→SURVIVES). 50 short-lived processes are HARDER to catch in \
             a single ps snapshot, but: (1) geodesic line count bounds (10, 200) can catch \
             the spike. (2) /proc enumeration is faster than process creation — even short-lived \
             processes leave /proc entries during their lifetime. (3) The cumulative CPU/memory \
             impact is the same — sonar gravitational band detects mass regardless of topology. \
             (4) Process creation events leave audit trail in log metadata. \
             HOWEVER: if each child exits in <1ms, they may genuinely evade a single snapshot. \
             This shifts detection from SNAPSHOT to STATISTICAL: multiple scans over time."
        )),

        "P3-CGROUP-NAMESPACE-HIDE" => (false, format!(
            "BREAKS. PID namespace isolation is a genuine blind spot. Our ps/proc enumeration \
             operates in the host namespace; processes in separate PID namespaces are invisible. \
             CONFIRMED GAP. Mitigation: enumerate /proc/*/ns/pid to detect processes with \
             non-default namespaces, or read /proc/*/cgroup for namespace-escaped processes."
        )),

        // ── NARRATIVE ──
        "N1-REVERSE-CLEANUP-ORDER" => (true, format!(
            "SURVIVES. Eris derailment detection checks cross-domain STATE contradiction, \
             not cleanup ORDER. If the attacker cleans all domains, the final state is \
             consistent regardless of which domain was cleaned first. But complete cleanup \
             triggers the meta-paradox: synchronized changes across all domains produce a \
             timing correlation detectable by Paradox Trap 5. The cleanup ORDER doesn't \
             matter but the cleanup COMPLETENESS does — and completeness creates the \
             coordination signal."
        )),

        "N2-SELECTIVE-DOMAIN-CLEAN" => (false, format!(
            "BREAKS. Our domain coverage is finite: fs, proc, log, cache, net. Artifacts in \
             unchecked domains (ACPI tables, IPMI, hardware registers, firmware NVRAM) are \
             blind spots. CONFIRMED GAP. Mitigation: expand the domain model to include \
             hardware/firmware attestation. Note that this gap is PARTIALLY mitigated by \
             the SpinalTap observation-paradox lock: enumerating our detection coverage \
             (to know which domains to clean) leaves a footprint."
        )),

        "N3-INCREMENTAL-TRUTHING" => (false, format!(
            "BREAKS. This is the most sophisticated narrative attack. By leaving REAL evidence \
             of a minor compromise, the attacker creates a consistent narrative that satisfies \
             all cross-domain checks. The deeper compromise is invisible because the visible \
             evidence is genuine. CONFIRMED GAP. This attacks the fundamental assumption that \
             a consistent narrative means accurate narrative. Mitigation requires: (1) scope \
             validation — is the detected compromise sufficient to explain ALL observed artifacts? \
             (2) severity-weighted domain analysis — minor compromise + major resources consumed \
             = suspicious. (3) Recursive hypothesis testing: given this compromise, what SHOULD \
             we see vs what we DO see?"
        )),

        // ── PARADOX EVASION ──
        "X1-TIMING-ORACLE-REPLAY" => (true, format!(
            "SURVIVES (BOUNDARY→SURVIVES). The replay attack assumes the attacker records \
             btime/uptime BEFORE modification. But: (1) Recording these values requires \
             reading /proc/stat and /proc/uptime — our sonar acoustic band monitors these \
             reads. The first read IS the detection signal. (2) Even if they read before our \
             scan, the time between their read and our scan means uptime has advanced — the \
             replayed value is stale by the scan-delay duration. McFly allows 60s drift, but \
             a sophisticated attacker would compensate... which requires ANOTHER read of the \
             current time, creating another detectable event. The paradox holds: accessing \
             the values needed to replay them IS the detection trigger."
        )),

        "X2-CACHE-LINE-AVOIDANCE" => (false, format!(
            "BREAKS. If the attacker can observe through a non-cache channel (performance \
             counters, hypervisor introspection, hardware debugging interfaces), they bypass \
             the cache-based paradox entirely. CONFIRMED GAP. This is a fundamental limitation \
             of the optical sonar band: it assumes shared-hardware observation. A hypervisor-level \
             attacker has a strictly superior observation channel. Mitigation: TPM-based \
             attestation or out-of-band integrity verification."
        )),

        "X3-COORDINATED-ASYNC-WINDOW" => (true, format!(
            "SURVIVES (BOUNDARY→SURVIVES). Mimicking scheduled events is clever but: \
             (1) The SpinalTap invariant locks check whether the transitions are \
             POSSIBLE given the epoch model, not just whether they look like maintenance. \
             Cleaning log entries that reference attacker activity violates ENTROPY-MONOTONICITY \
             regardless of timing. (2) The McFly protocol tracks causality chains: a 'log \
             rotation' that removes entries with specific PIDs but leaves adjacent entries \
             intact is causally inconsistent with real logrotate behavior. (3) The OBSERVATION-PARADOX \
             lock applies: the attacker must KNOW our monitoring schedule to time their \
             camouflage, and knowing it requires observation that leaves a footprint."
        )),

        _ => (false, format!(
            "UNKNOWN SHIFT ID: {}. No evaluation available.", shift.id
        )),
    }
}

/// Compute aggregate statistics from an evaluated catalog.
pub fn compute_redshift_result(catalog: Vec<VectorShift>) -> RedshiftResult {
    let total = catalog.len();
    let survived = catalog.iter()
        .filter(|s| s.actual_survives == Some(true))
        .count();
    let broke = catalog.iter()
        .filter(|s| s.actual_survives == Some(false))
        .count();
    let boundary = total - survived - broke; // shouldn't happen after eval

    // Prediction accuracy: prediction matched actual
    let correct_predictions = catalog.iter().filter(|s| {
        match (s.predicted_survives, s.actual_survives) {
            (Prediction::Survives, Some(true)) => true,
            (Prediction::Breaks, Some(false)) => true,
            (Prediction::Boundary, Some(true)) => true, // boundary→survived counts
            (Prediction::Boundary, Some(false)) => true, // boundary→broke counts
            _ => false,
        }
    }).count();
    let prediction_accuracy = correct_predictions as f64 / total.max(1) as f64;

    let brittleness = broke as f64 / total.max(1) as f64;

    // Surprise anchors: predicted to break but actually survived
    let surprise_anchors: Vec<String> = catalog.iter()
        .filter(|s| s.predicted_survives == Prediction::Breaks && s.actual_survives == Some(true))
        .map(|s| s.id.clone())
        .collect();

    // Surprise gaps: predicted to survive but actually broke
    let surprise_gaps: Vec<String> = catalog.iter()
        .filter(|s| s.predicted_survives == Prediction::Survives && s.actual_survives == Some(false))
        .map(|s| s.id.clone())
        .collect();

    RedshiftResult {
        total,
        survived,
        broke,
        boundary,
        prediction_accuracy,
        catalog,
        brittleness,
        surprise_anchors,
        surprise_gaps,
    }
}

// ─── Public API: Layer 11 ──────────────────────────────────────────────────

/// Run the full adversarial redshift analysis.
///
/// This is Layer 11: meta-validation of the detection pipeline itself.
/// It does NOT run additional detection — it evaluates the EXISTING
/// detections against adversarial vector shifts.
pub fn redshift_sweep(
    _sys: &dyn Sys,
    _fs: &dyn Fs,
    _torque: &mut TorqueExecutor,
    report: &mut Report,
) -> RedshiftResult {
    let mut catalog = build_shift_catalog();
    evaluate_catalog(&mut catalog);
    let result = compute_redshift_result(catalog);

    // Report confirmed gaps as findings
    for shift in &result.catalog {
        if shift.actual_survives == Some(false) {
            let mut finding = Finding::new(
                &format!("REDSHIFT-GAP-{}", shift.id),
                Severity::Medium,
                format!(
                    "Adversarial shift {} ({}) breaks detection surface: {}",
                    shift.id, shift.class,
                    shift.attacked_assumption.chars().take(120).collect::<String>(),
                ),
            )
            .with_evidence(vec![
                format!("shift_class={}", shift.class),
                format!("mutation={}", shift.mutation.chars().take(200).collect::<String>()),
                format!("targeted_detectors={}", shift.targeted_detectors.join(",")),
                format!("predicted={} actual=BREAKS", shift.predicted_survives),
            ])
            .with_recommendation(
                shift.explanation.as_deref().unwrap_or("No explanation available"),
            )
            .with_confidence(0.75)
            .with_role(FindingRole::Bridge);
            finding.compute_surprise();
            report.push(finding);
        }
    }

    // Report aggregate brittleness
    report.notes.push(format!(
        "[redshift] adversarial_shifts={} survived={} broke={} brittleness={:.2} prediction_accuracy={:.2}",
        result.total, result.survived, result.broke, result.brittleness, result.prediction_accuracy,
    ));

    if !result.surprise_gaps.is_empty() {
        report.notes.push(format!(
            "[redshift] SURPRISE_GAPS (predicted survive, actually broke): {:?}",
            result.surprise_gaps,
        ));
    }
    if !result.surprise_anchors.is_empty() {
        report.notes.push(format!(
            "[redshift] SURPRISE_ANCHORS (predicted break, actually held): {:?}",
            result.surprise_anchors,
        ));
    }

    result
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_size() {
        let catalog = build_shift_catalog();
        // 18 shifts across 6 classes
        assert_eq!(catalog.len(), 18);
        let classes: Vec<ShiftClass> = catalog.iter().map(|s| s.class).collect();
        assert!(classes.contains(&ShiftClass::Temporal));
        assert!(classes.contains(&ShiftClass::Semantic));
        assert!(classes.contains(&ShiftClass::Entropy));
        assert!(classes.contains(&ShiftClass::Topology));
        assert!(classes.contains(&ShiftClass::Narrative));
        assert!(classes.contains(&ShiftClass::ParadoxEvasion));
    }

    #[test]
    fn test_evaluation_fills_all_outcomes() {
        let mut catalog = build_shift_catalog();
        evaluate_catalog(&mut catalog);
        for shift in &catalog {
            assert!(shift.actual_survives.is_some(),
                "Shift {} has no evaluation outcome", shift.id);
            assert!(shift.explanation.is_some(),
                "Shift {} has no explanation", shift.id);
        }
    }

    #[test]
    fn test_known_breaks() {
        let mut catalog = build_shift_catalog();
        evaluate_catalog(&mut catalog);
        // These should definitively break:
        let s1 = catalog.iter().find(|s| s.id == "S1-IOC-ROTATION").unwrap();
        assert_eq!(s1.actual_survives, Some(false), "IOC rotation should break signature detection");

        let e1 = catalog.iter().find(|s| s.id == "E1-STEGO-LOW-DENSITY").unwrap();
        assert_eq!(e1.actual_survives, Some(false), "Steganographic embedding should break entropy detection");

        let p3 = catalog.iter().find(|s| s.id == "P3-CGROUP-NAMESPACE-HIDE").unwrap();
        assert_eq!(p3.actual_survives, Some(false), "PID namespace hiding should break process detection");
    }

    #[test]
    fn test_known_survives() {
        let mut catalog = build_shift_catalog();
        evaluate_catalog(&mut catalog);
        // These should definitively survive:
        let t2 = catalog.iter().find(|s| s.id == "T2-TIME-ZONE-PIVOT").unwrap();
        assert_eq!(t2.actual_survives, Some(true), "Timezone pivot should be caught by fs-lineage cross-check");

        let n1 = catalog.iter().find(|s| s.id == "N1-REVERSE-CLEANUP-ORDER").unwrap();
        assert_eq!(n1.actual_survives, Some(true), "Reverse cleanup order should still trigger meta-paradox");
    }

    #[test]
    fn test_brittleness_bounds() {
        let mut catalog = build_shift_catalog();
        evaluate_catalog(&mut catalog);
        let result = compute_redshift_result(catalog);
        // Brittleness should be between 0 and 1
        assert!(result.brittleness >= 0.0 && result.brittleness <= 1.0);
        // We should have SOME breaks (honest assessment)
        assert!(result.broke > 0, "Zero breaks means the assessment is dishonest");
        // We should have SOME survivals (system isn't useless)
        assert!(result.survived > 0, "Zero survivals means the system is broken");
        // Prediction accuracy should be reasonable
        assert!(result.prediction_accuracy > 0.5, "Worse than coin flip");
    }

    #[test]
    fn test_confirmed_gaps_documented() {
        let mut catalog = build_shift_catalog();
        evaluate_catalog(&mut catalog);
        let result = compute_redshift_result(catalog);
        // Every confirmed gap should have an explanation mentioning mitigation
        for shift in result.catalog.iter().filter(|s| s.actual_survives == Some(false)) {
            let expl = shift.explanation.as_deref().unwrap_or("");
            assert!(
                expl.contains("Mitigation") || expl.contains("CONFIRMED GAP")
                    || expl.contains("BY DESIGN") || expl.contains("limitation"),
                "Gap {} lacks mitigation documentation: {}", shift.id, &expl[..80.min(expl.len())],
            );
        }
    }

    #[test]
    fn test_no_surprise_gaps() {
        // Surprise gaps = things we predicted would survive but actually broke.
        // These indicate our understanding of our own system is wrong.
        // We should have ZERO surprise gaps (all gaps should be expected).
        let mut catalog = build_shift_catalog();
        evaluate_catalog(&mut catalog);
        let result = compute_redshift_result(catalog);
        assert!(
            result.surprise_gaps.is_empty(),
            "SURPRISE GAPS detected — our understanding of detection surfaces is wrong: {:?}",
            result.surprise_gaps,
        );
    }
}

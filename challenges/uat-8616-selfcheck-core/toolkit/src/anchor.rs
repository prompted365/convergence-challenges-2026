//! Anchor — Detection Surface Invariant Proofs.
//!
//! Redshift showed us WHERE the surfaces break. Anchor proves WHY the
//! surviving surfaces hold — and WHY they MUST hold, not just empirically
//! but by mathematical/physical necessity.
//!
//! ## The Problem
//!
//! A detection surface that works "because we tested it" is a surface
//! that breaks the first time an adversary does something we didn't test.
//! A detection surface anchored to a physical invariant works because
//! the adversary would have to violate physics to evade it.
//!
//! ## Anchor Types
//!
//! Each anchor binds a detection surface to one of three invariant classes:
//!
//! 1. **Thermodynamic Anchors**: Based on entropy, information theory, or
//!    irreversibility. These hold because the second law of thermodynamics
//!    cannot be violated — any operation that modifies system state
//!    increases total entropy. The attacker's cleanup MUST leave a trace
//!    because perfectly reversible computation is physically impossible.
//!
//! 2. **Causality Anchors**: Based on time-ordering, happened-before
//!    relationships, and the impossibility of backward causation. The
//!    attacker cannot make event B happen before event A if B depends on
//!    A. Any attempt to forge temporal ordering leaves inconsistencies
//!    detectable by McFly's lineage checks.
//!
//! 3. **Observation Anchors**: Based on the measurement problem — any
//!    observation of a system perturbs it. The attacker cannot inspect
//!    our detection state without changing it (cache lines, timing,
//!    scheduling artifacts). This is the quantum mechanics of forensics:
//!    the act of reconnaissance IS the detection signal.
//!
//! ## Formal Structure
//!
//! Each anchor has:
//!   - An **invariant statement** (the mathematical property that holds)
//!   - A **physical basis** (why it holds in the real world)
//!   - A **bound detection surface** (which detector relies on it)
//!   - A **violation cost** (what the attacker pays to attempt violation)
//!   - A **proof sketch** (informal but rigorous argument)
//!   - A **known weakness** (honest assessment of where the anchor is soft)
//!
//! ## Connection to Redshift
//!
//! For every shift in the redshift catalog that SURVIVED:
//!   → Anchor provides the formal reason it survived
//! For every shift that BROKE:
//!   → Anchor documents which invariant class would need to be extended
//!     to cover the gap

use std::fmt;

use crate::report::{Finding, FindingRole, Report, Severity};
use crate::io::Fs;
use crate::sys::Sys;
use crate::torque::TorqueExecutor;

// ─── Anchor Types ──────────────────────────────────────────────────────────

/// The invariant class that anchors a detection surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantClass {
    /// Based on entropy increase, irreversibility, information conservation.
    Thermodynamic,
    /// Based on time-ordering, causality, happened-before.
    Causal,
    /// Based on measurement perturbation, Heisenberg-style observer effects.
    Observation,
}

impl fmt::Display for InvariantClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvariantClass::Thermodynamic => write!(f, "THERMODYNAMIC"),
            InvariantClass::Causal => write!(f, "CAUSAL"),
            InvariantClass::Observation => write!(f, "OBSERVATION"),
        }
    }
}

/// A formal anchor binding a detection surface to a physical invariant.
#[derive(Debug, Clone)]
pub struct SurfaceAnchor {
    /// Unique identifier.
    pub id: String,
    /// The detection surface this anchors.
    pub bound_surface: String,
    /// Which invariant class provides the anchor.
    pub class: InvariantClass,
    /// The invariant statement (quasi-formal).
    pub invariant: String,
    /// Physical basis — why this holds in reality.
    pub physical_basis: String,
    /// What it costs the attacker to violate this anchor.
    pub violation_cost: ViolationCost,
    /// Informal proof sketch.
    pub proof_sketch: String,
    /// Honest assessment of known weaknesses.
    pub known_weakness: String,
    /// Which redshift shifts this anchor explains the survival of.
    pub explains_survival: Vec<String>,
    /// Which redshift gaps this anchor COULD cover if extended.
    pub could_cover_gaps: Vec<String>,
    /// Strength rating (0.0–1.0). 1.0 = physics-guaranteed.
    pub strength: f64,
}

/// Cost model for violating an anchor.
#[derive(Debug, Clone)]
pub struct ViolationCost {
    /// Descriptive name of the cost.
    pub kind: String,
    /// Estimated computational/physical cost (arbitrary units; f64::INFINITY = impossible).
    pub magnitude: f64,
    /// What the attacker gives up by paying this cost.
    pub tradeoff: String,
}

// ─── Anchor Catalog ────────────────────────────────────────────────────────

/// Build the complete anchor catalog — one anchor per surviving detection surface.
pub fn build_anchor_catalog() -> Vec<SurfaceAnchor> {
    vec![
        // ═══════════════════════════════════════════════════════════════════
        // THERMODYNAMIC ANCHORS
        // ═══════════════════════════════════════════════════════════════════

        SurfaceAnchor {
            id: "ANCHOR-THERMO-ENTROPY-MONOTONICITY".into(),
            bound_surface: "SPINALTAP-ENTROPY-MONOTONICITY lock".into(),
            class: InvariantClass::Thermodynamic,
            invariant: "In any system with I/O activity, the joint entropy of \
                        (filesystem × process_table × log_state) is monotonically \
                        non-decreasing. Cleanup requires entropy decrease, which \
                        cannot occur without observable energy expenditure.".into(),
            physical_basis: "Second law of thermodynamics applied to information systems: \
                            Landauer's principle states that erasing one bit of information \
                            requires kT·ln(2) joules of energy and produces heat. Cleanup = \
                            erasure. The energy expenditure manifests as CPU cycles, disk \
                            writes, and cache pressure — all detectable side-channels.".into(),
            violation_cost: ViolationCost {
                kind: "Thermodynamic".into(),
                magnitude: f64::INFINITY,
                tradeoff: "Violating the second law of thermodynamics is not a tradeoff — \
                           it is physically impossible. The attacker can REDUCE the cost of \
                           cleanup but cannot make it zero.".into(),
            },
            proof_sketch: "Let S(t) = H(fs(t)) + H(proc(t)) + H(log(t)) be the joint system \
                          entropy at time t. Any attacker operation O at time t₁ produces \
                          S(t₁) ≥ S(t₀). Cleanup C at time t₂ attempts S(t₂) < S(t₁). \
                          By Landauer's principle, C requires ΔE ≥ kT·ln(2)·(S(t₁)−S(t₂)) \
                          joules. This energy is dissipated as heat/work, producing \
                          observable side effects: CPU utilization spike, disk I/O burst, \
                          cache line invalidation. The side effects are the detection signal. \
                          The attacker can spread the cost over time (slow cleanup), but the \
                          TOTAL energy is conserved — cumulative detection (drift envelope) \
                          catches the accumulated signal.".into(),
            known_weakness: "The entropy measurement is ESTIMATED (Shannon on file contents), \
                            not EXACT (Kolmogorov complexity). Shannon entropy can be spoofed \
                            by padding. The thermodynamic argument applies to total system \
                            entropy including microstate, but we only observe macrostate. The \
                            gap between macro-observable entropy and true physical entropy is \
                            where steganographic attacks live.".into(),
            explains_survival: vec![
                "T3-JITTER-FLOOD".into(),  // Jitter inflates entropy = detectable regime change
                "N1-REVERSE-CLEANUP-ORDER".into(),  // Complete cleanup = entropy decrease regardless of order
                "X3-COORDINATED-ASYNC-WINDOW".into(),  // Each domain cleanup violates entropy monotonicity
            ],
            could_cover_gaps: vec![
                "E1-STEGO-LOW-DENSITY".into(),  // Multi-scale entropy analysis
                "E3-ENTROPY-MATCHING".into(),   // Block-level entropy profiling
            ],
            strength: 0.85,
        },

        SurfaceAnchor {
            id: "ANCHOR-THERMO-INFORMATION-CONSERVATION".into(),
            bound_surface: "SPINALTAP-INFORMATION-CONSERVATION lock + TORQUE drift envelope".into(),
            class: InvariantClass::Thermodynamic,
            invariant: "The total information content of a system (Kolmogorov complexity of \
                        the full state) cannot decrease without an external sink. Deletion \
                        moves information to the sink; the sink (disk zeroes, freed pages) \
                        has a different compression profile than organic content.".into(),
            physical_basis: "Information is physical (Landauer, 1961). Data stored on disk \
                            occupies physical substrate. Deletion doesn't destroy the information \
                            — it marks sectors for reuse. Until overwritten, the information \
                            persists. Even after overwrite, the act of overwriting changes the \
                            wear-level counters, write amplification factors, and sector error \
                            rates on flash/SSD storage.".into(),
            violation_cost: ViolationCost {
                kind: "Information-theoretic".into(),
                magnitude: 2.0_f64.powi(48), // storage-level forensics
                tradeoff: "To truly erase without trace, the attacker must overwrite every \
                           sector containing evidence, adjust wear-level counters, normalize \
                           SMART data, and match the stochastic write pattern of legitimate \
                           operation. This is equivalent to rebuilding the storage controller's \
                           internal state.".into(),
            },
            proof_sketch: "Let K(state) be the Kolmogorov complexity of the system state. \
                          Manifest drift detection measures |K(state_now) − K(state_baseline)| \
                          using file size as a proxy. Deletion reduces K(state_now) while \
                          increasing K(disk_free_list) — the freed sectors have higher entropy \
                          than their original content. Torque's cumulative_size_fs tracks the \
                          total, making the drift monotonically detectable over time.".into(),
            known_weakness: "We approximate Kolmogorov complexity with file sizes and Shannon \
                            entropy. The true information-theoretic argument requires access to \
                            storage-layer metadata (SMART, wear-level, sector maps) which we \
                            don't currently read. At the filesystem level, an attacker who \
                            replaces deleted content with same-sized padding defeats our proxy.".into(),
            explains_survival: vec![
                "T1-SLOW-DRIP".into(),  // Cumulative manifest delta catches slow exfiltration
                "S2-LOTL-PURE".into(),  // Even LOTL produces log entries = information increase
            ],
            could_cover_gaps: vec![
                "N3-INCREMENTAL-TRUTHING".into(),  // Scope validation via information budget
            ],
            strength: 0.70,
        },

        // ═══════════════════════════════════════════════════════════════════
        // CAUSAL ANCHORS
        // ═══════════════════════════════════════════════════════════════════

        SurfaceAnchor {
            id: "ANCHOR-CAUSAL-LINEAGE".into(),
            bound_surface: "MCFLY lineage verification + SPINALTAP temporal consistency lock".into(),
            class: InvariantClass::Causal,
            invariant: "For any two system events A and B where A→B (A causes B), \
                        timestamp(A) < timestamp(B) in all valid execution traces. \
                        Forging this ordering requires either backward time travel or \
                        modifying both timestamps consistently — which requires knowledge \
                        of the full causal graph.".into(),
            physical_basis: "Causality is a consequence of special relativity: effects cannot \
                            precede causes in any reference frame. In computing, the analogous \
                            principle is Lamport's happened-before relation. Even without \
                            physical relativistic constraints, the operating system enforces \
                            causal ordering through file creation → inode allocation → journal \
                            commit → log entry sequences.".into(),
            violation_cost: ViolationCost {
                kind: "Causal graph reconstruction".into(),
                magnitude: 2.0_f64.powi(32),
                tradeoff: "To forge a consistent causal history, the attacker must reconstruct \
                           the ENTIRE dependency graph of the system state they're modifying. \
                           For N system events, the graph has O(N²) potential edges. Checking \
                           consistency is O(N²); forging it is O(N² · K) where K is the number \
                           of cross-references per event.".into(),
            },
            proof_sketch: "McFly verifies: btime + uptime ≈ wall_clock (3 independent sources). \
                          Forging this requires modifying at least 2 of 3 values. Each modification \
                          has causal descendants: btime affects log timestamps, uptime affects \
                          /proc/stat derivative values, wall_clock affects NTP state. Modifying \
                          any one value without updating ALL descendants creates a lineage break \
                          detectable by cross-referencing. The causal graph is too large to forge \
                          completely in bounded time.".into(),
            known_weakness: "If the attacker compromises at boot time (before any baseline is \
                            established), they control the entire causal history from the start. \
                            There is no 'before' to compare against. This is the X1-TIMING-ORACLE \
                            replay scenario — partially mitigated by the observation cost of \
                            recording initial values, but fundamentally vulnerable if the attacker \
                            has root before we have any state.".into(),
            explains_survival: vec![
                "T2-TIME-ZONE-PIVOT".into(),  // fs timestamps in UTC break the pivot
                "X1-TIMING-ORACLE-REPLAY".into(),  // Reading values to replay = observation cost
            ],
            could_cover_gaps: vec![
                "P3-CGROUP-NAMESPACE-HIDE".into(),  // Namespace creation has causal footprint in kernel
            ],
            strength: 0.80,
        },

        SurfaceAnchor {
            id: "ANCHOR-CAUSAL-PROCESS-TREE".into(),
            bound_surface: "ASYNCOS process baseline + SONAR gravitational geodesics".into(),
            class: InvariantClass::Causal,
            invariant: "Every process has exactly one parent. The process tree is a directed \
                        acyclic graph rooted at PID 1. Any process not reachable from PID 1 \
                        via the parent chain was created outside normal boot sequence.".into(),
            physical_basis: "Process creation is an atomic kernel operation (clone/fork syscall). \
                            The kernel MUST record the parent PID. This is enforced by the scheduler \
                            and cannot be bypassed without a kernel vulnerability. Even kernel \
                            modules that create kthreads are tracked in the process table.".into(),
            violation_cost: ViolationCost {
                kind: "Kernel integrity compromise".into(),
                magnitude: 2.0_f64.powi(24),
                tradeoff: "To create an 'orphan' process invisible to the tree, the attacker \
                           must modify kernel data structures directly — which requires kernel \
                           exploit or loadable module. Kernel exploits leave their own forensic \
                           artifacts (modified page tables, unexpected kernel memory allocations).".into(),
            },
            proof_sketch: "Geodesic: ps auxww lists all processes. Line count in [10, 200] for \
                          a gateway appliance. Each unexpected process adds 1 line → geodesic \
                          deviation. Expected keywords (init, sshd) validate tree root. Forbidden \
                          keywords catch known attacker tools. Process baseline cross-references \
                          the tree structure: legitimate processes have legitimate parents.".into(),
            known_weakness: "PID namespace isolation (shift P3) creates a genuine blind spot. \
                            Processes in separate namespaces are not visible from the host's \
                            /proc traversal. Mitigation: enumerate /proc/*/ns/pid across all \
                            processes to detect namespace-escaped children.".into(),
            explains_survival: vec![
                "P1-MEMONLY-PERSISTENCE".into(),  // Memory implant still has a host process
                "P2-PROCESS-TREE-SPREAD".into(),  // 50 children still visible in tree
                "S2-LOTL-PURE".into(),  // LOTL tools have unusual parent chains
            ],
            could_cover_gaps: vec![
                "P3-CGROUP-NAMESPACE-HIDE".into(),  // Namespace enumeration via /proc/*/ns/pid
            ],
            strength: 0.75,
        },

        // ═══════════════════════════════════════════════════════════════════
        // OBSERVATION ANCHORS
        // ═══════════════════════════════════════════════════════════════════

        SurfaceAnchor {
            id: "ANCHOR-OBS-CACHE-PERTURBATION".into(),
            bound_surface: "SONAR optical band (cache eviction detection)".into(),
            class: InvariantClass::Observation,
            invariant: "On shared hardware with a unified cache hierarchy, any observation \
                        of our memory access patterns MUST evict our cache lines. The observer \
                        cannot read without writing (to their own cache), and writing to shared \
                        cache evicts competing lines by set-associativity.".into(),
            physical_basis: "CPU caches use set-associative mapping. Two memory addresses that \
                            map to the same cache set compete for the same N ways. When the \
                            observer accesses their probe address in the same set as our sentinel, \
                            our sentinel is evicted. This is a physical property of the hardware, \
                            not a software design choice. The observer CANNOT read our state \
                            without perturbing it.".into(),
            violation_cost: ViolationCost {
                kind: "Hardware redesign".into(),
                magnitude: f64::INFINITY,
                tradeoff: "The only way to observe without perturbation is to use a physically \
                           separate observation channel (hypervisor, hardware debugger, or side- \
                           band management engine like Intel ME/AMD PSP). This requires either \
                           physical access or a separate vulnerability class entirely.".into(),
            },
            proof_sketch: "Let S be our sentinel set in L1 cache (8 lines, 8-way associative). \
                          Any process sharing the physical core that accesses S's set index \
                          evicts at least one of our lines. Our re-access timing changes from \
                          ~4ns (L1 hit) to ~60ns (DRAM miss). The timing ratio > 5x triggers \
                          anomaly score 0.9. Statistical rollup: >2 out of 8 probes with high \
                          anomaly → finding. The attacker cannot observe S without this cost \
                          because cache replacement is a hardware-level operation.".into(),
            known_weakness: "CONFIRMED BROKEN by X2-CACHE-LINE-AVOIDANCE. If the attacker observes \
                            through a non-cache channel (perf counters, hypervisor introspection, \
                            hardware debug port), they bypass the cache-based detection entirely. \
                            The anchor holds ONLY on shared-hardware observation. A hypervisor- \
                            level attacker is outside this anchor's scope.".into(),
            explains_survival: vec![
                "T3-JITTER-FLOOD".into(),  // Jitter inflates timing variance but eviction ratio is distinct
                "S3-POLYGLOT-PAYLOAD".into(),  // Polyglot still produces cache pressure from execution
            ],
            could_cover_gaps: vec![],  // X2 is a fundamental gap, not coverable by this anchor class
            strength: 0.60, // Reduced because of X2 hypervisor gap
        },

        SurfaceAnchor {
            id: "ANCHOR-OBS-RECONNAISSANCE-FOOTPRINT".into(),
            bound_surface: "SPINALTAP observation-paradox lock + interference pattern detection".into(),
            class: InvariantClass::Observation,
            invariant: "Any enumeration of system detection surfaces (to plan evasion) \
                        produces observable artifacts: file reads generate atime updates, \
                        /proc reads allocate kernel buffers, network probes create connection \
                        state. The attacker cannot map our detection coverage without \
                        contributing to it.".into(),
            physical_basis: "In information theory, acquiring information about a system requires \
                            interaction with it (no 'free peeks'). In practice: reading a file \
                            updates its atime (unless mounted noatime), reading /proc causes the \
                            kernel to generate the virtual file contents (CPU + memory cost), \
                            network probes create socket state. Each interaction is a side-channel \
                            available to the defender.".into(),
            violation_cost: ViolationCost {
                kind: "Zero-knowledge reconnaissance".into(),
                magnitude: 2.0_f64.powi(40),
                tradeoff: "To enumerate detection surfaces without leaving a trace, the attacker \
                           must already know WHAT to look for (which requires prior enumeration — \
                           recursive dependency). Or they must use an external information source \
                           (e.g., reading our source code). Source disclosure is assumed but doesn't \
                           help with RUNTIME state — they still need to enumerate which detectors \
                           are active in THIS deployment.".into(),
            },
            proof_sketch: "SpinalTap OBSERVATION-PARADOX lock: reconnaissance epoch→exploitation \
                          epoch transition requires knowing what we monitor. Knowing what we monitor \
                          requires reading our config/code at runtime. Reading produces: (1) atime \
                          on config files, (2) /proc/self reads for our PID, (3) cache line \
                          perturbation from memory access. Each observation is a detection signal. \
                          The interference patterns (NTP, cron, logrotate mimicry) force the attacker \
                          to CLASSIFY each signal as real-or-decoy, consuming their analytical bandwidth \
                          and leaving timing artifacts from the classification computation itself.".into(),
            known_weakness: "If the attacker has full source access AND knows the deployment \
                            configuration statically (e.g., it's the default config), they can \
                            plan evasion without runtime enumeration. The observation anchor \
                            is weakest when our configuration is predictable. Mitigation: \
                            randomize detection parameters at boot (which equilibrium already does).".into(),
            explains_survival: vec![
                "X1-TIMING-ORACLE-REPLAY".into(),  // Recording boot-time = observation footprint
                "X3-COORDINATED-ASYNC-WINDOW".into(),  // Knowing schedule = observation cost
                "S2-LOTL-PURE".into(),  // LOTL requires knowing what tools are safe = enumeration
            ],
            could_cover_gaps: vec![
                "N2-SELECTIVE-DOMAIN-CLEAN".into(),  // Knowing which domains to clean = enumeration
            ],
            strength: 0.75,
        },

        SurfaceAnchor {
            id: "ANCHOR-OBS-UNCERTAINTY-WEAPON".into(),
            bound_surface: "TRIPWIRE probabilistic firing + decoy signals + torque ambient reads".into(),
            class: InvariantClass::Observation,
            invariant: "The attacker's decision function (continue vs abort) has strictly \
                        negative expected value when detection probability is uncertain. \
                        Uncertainty about detection is more damaging to the attacker than \
                        certain non-detection.".into(),
            physical_basis: "Decision theory: under uncertainty, the attacker must assign a \
                            probability p to 'I have been detected'. Their expected payoff is \
                            E[payoff] = (1−p)·V_continue − p·V_abort, where V_abort includes \
                            cost of burned access + reentry cost. When p is UNKNOWN (not just \
                            uncertain but unestimable), the attacker cannot compute E[payoff] \
                            and must use worst-case reasoning (minimax), which is strictly worse \
                            than Bayesian optimal.".into(),
            violation_cost: ViolationCost {
                kind: "Decision-theoretic".into(),
                magnitude: f64::INFINITY,
                tradeoff: "The attacker can only resolve the uncertainty by probing our detection \
                           system (observation anchor → detection signal) or by having a priori \
                           knowledge of our exact implementation (source disclosure + config). Even \
                           with source access, the probabilistic tripwire parameters are randomized \
                           at runtime, making the exact detection probability uncomputable.".into(),
            },
            proof_sketch: "Tripwire fires with probability p ∈ [fire_probability ± runtime_noise]. \
                          Decoy probability d means background firing rate is nonzero. The attacker \
                          observes: signal OR no-signal. But P(signal|triggered) = p and \
                          P(signal|not_triggered) = d, so the posterior P(triggered|signal) depends \
                          on the prior P(trigger) which the attacker doesn't know. Without the prior, \
                          Bayesian updating is impossible. The attacker is stuck with: 'I see a signal, \
                          it might be real, it might be a decoy, and I can't tell which.' This IS the \
                          detection — not the signal itself, but the attacker's inability to process it.".into(),
            known_weakness: "If the attacker has many samples (triggers the tripwire many times and \
                            observes the outcome), they can estimate p and d empirically. Rate- \
                            limiting and one-shot tripwires mitigate this but don't eliminate it. \
                            The anchor is strongest on first contact and weakens with repeated \
                            interaction.".into(),
            explains_survival: vec![
                "T2-TIME-ZONE-PIVOT".into(),  // Timezone pivot doesn't resolve detection uncertainty
                "T3-JITTER-FLOOD".into(),  // Jitter adds noise to BOTH attacker and defender
                "N1-REVERSE-CLEANUP-ORDER".into(),  // Cleanup order doesn't reveal detection state
            ],
            could_cover_gaps: vec![
                "N3-INCREMENTAL-TRUTHING".into(),  // Partial truth doesn't resolve the uncertainty
            ],
            strength: 0.90,
        },

        // ═══════════════════════════════════════════════════════════════════
        // CROSS-CLASS ANCHOR (bridges multiple invariant classes)
        // ═══════════════════════════════════════════════════════════════════

        SurfaceAnchor {
            id: "ANCHOR-META-ERIS-PARADOX-SYSTEM".into(),
            bound_surface: "ERIS paradox traps (all 5) + matryoshka shell nesting".into(),
            class: InvariantClass::Thermodynamic, // Primary class; draws from all three
            invariant: "The composition of multiple independent detection surfaces produces \
                        emergent paradoxes that no single-surface evasion can resolve. \
                        Evading surface A pushes the system into a state detectable by surface B. \
                        This is not a design choice but a consequence of the detection surfaces \
                        being anchored to DIFFERENT invariant classes (thermodynamic, causal, \
                        observation).".into(),
            physical_basis: "In constraint satisfaction, if constraints C₁...Cₙ are drawn from \
                            independent physical domains (entropy, causality, observation), the \
                            satisfying set is the INTERSECTION of individual satisfying sets. As \
                            n grows, the intersection shrinks exponentially. The attacker must \
                            simultaneously satisfy constraints from domains that are physically \
                            independent — there is no 'master key' that satisfies all domains \
                            because the domains are governed by different physical laws.".into(),
            violation_cost: ViolationCost {
                kind: "Multi-domain simultaneous violation".into(),
                magnitude: f64::INFINITY,
                tradeoff: "Evading the paradox system requires violating conservation of information \
                           (thermodynamic), backward causation (causal), AND non-perturbative observation \
                           (observation) simultaneously. Each violation is independently impossible. \
                           Their conjunction is multiply impossible.".into(),
            },
            proof_sketch: "Paradox trap 5 (meta-paradox): coordinated cleanup triggers synchronization \
                          detection (THERMODYNAMIC — entropy decrease across domains correlates). \
                          Staggered cleanup triggers narrative drift detection (CAUSAL — events in \
                          different domains disagree about timeline). Partial cleanup leaves cross-domain \
                          contradictions (OBSERVATION — some domains show evidence of the attacker's \
                          reconnaissance while others are clean). The three anchor classes create a \
                          three-way pincer. The attacker is in a topological trap: all paths out of \
                          the compromised state pass through at least one detection surface. The \
                          matryoshka nesting means evading Shell N pushes them INTO Shell N+1's \
                          detection region.".into(),
            known_weakness: "The paradox system assumes the attacker wants to CLEAN UP. An attacker \
                            who doesn't care about cleanup (smash-and-grab, one-shot exploit, \
                            ransomware) bypasses the paradox entirely because they don't attempt \
                            evasion. The paradox targets sophisticated persistent threats, not \
                            opportunistic attacks. Also: the N3-INCREMENTAL-TRUTHING attack partially \
                            defeats this by ADMITTING to a compromise, which short-circuits the \
                            cleanup paradox.".into(),
            explains_survival: vec![
                "N1-REVERSE-CLEANUP-ORDER".into(),
                "X1-TIMING-ORACLE-REPLAY".into(),
                "X3-COORDINATED-ASYNC-WINDOW".into(),
                "T2-TIME-ZONE-PIVOT".into(),
                "T3-JITTER-FLOOD".into(),
            ],
            could_cover_gaps: vec![],
            strength: 0.92,
        },
    ]
}

// ─── Anchor Evaluation ─────────────────────────────────────────────────────

/// Result of anchor analysis.
#[derive(Debug, Clone)]
pub struct AnchorResult {
    /// Total anchors in the catalog.
    pub total_anchors: usize,
    /// Average anchor strength (0.0–1.0).
    pub mean_strength: f64,
    /// Minimum anchor strength (weakest link).
    pub min_strength: f64,
    /// The weakest anchor ID.
    pub weakest_anchor: String,
    /// Coverage: what fraction of redshift survivals are explained by anchors.
    pub survival_coverage: f64,
    /// Gap coverage: what fraction of redshift gaps COULD be covered by extensions.
    pub gap_coverage_potential: f64,
    /// The full catalog.
    pub anchors: Vec<SurfaceAnchor>,
    /// Unanchored survivals: shifts that survived but no anchor explains why.
    pub unanchored: Vec<String>,
    /// Uncoverable gaps: redshift breaks with no anchor extension path.
    pub uncoverable_gaps: Vec<String>,
}

/// Cross-reference anchors against a set of redshift survival IDs and gap IDs.
pub fn evaluate_anchors(
    anchors: &[SurfaceAnchor],
    survival_ids: &[String],
    gap_ids: &[String],
) -> AnchorResult {
    let total_anchors = anchors.len();

    let mean_strength = if total_anchors > 0 {
        anchors.iter().map(|a| a.strength).sum::<f64>() / total_anchors as f64
    } else {
        0.0
    };

    let weakest = anchors.iter()
        .min_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap_or(std::cmp::Ordering::Equal))
        .map(|a| (a.strength, a.id.clone()))
        .unwrap_or((0.0, "NONE".into()));

    // Coverage: how many survivals are explained by at least one anchor?
    let explained: Vec<&String> = survival_ids.iter()
        .filter(|sid| {
            anchors.iter().any(|a| a.explains_survival.contains(sid))
        })
        .collect();
    let survival_coverage = if survival_ids.is_empty() {
        1.0
    } else {
        explained.len() as f64 / survival_ids.len() as f64
    };

    // Unanchored: survivals not explained by any anchor
    let unanchored: Vec<String> = survival_ids.iter()
        .filter(|sid| !anchors.iter().any(|a| a.explains_survival.contains(sid)))
        .cloned()
        .collect();

    // Gap coverage potential: how many gaps COULD be covered?
    let coverable: Vec<&String> = gap_ids.iter()
        .filter(|gid| {
            anchors.iter().any(|a| a.could_cover_gaps.contains(gid))
        })
        .collect();
    let gap_coverage_potential = if gap_ids.is_empty() {
        1.0
    } else {
        coverable.len() as f64 / gap_ids.len() as f64
    };

    // Uncoverable: gaps no anchor extension can address
    let uncoverable_gaps: Vec<String> = gap_ids.iter()
        .filter(|gid| !anchors.iter().any(|a| a.could_cover_gaps.contains(gid)))
        .cloned()
        .collect();

    AnchorResult {
        total_anchors,
        mean_strength,
        min_strength: weakest.0,
        weakest_anchor: weakest.1,
        survival_coverage,
        gap_coverage_potential,
        anchors: anchors.to_vec(),
        unanchored,
        uncoverable_gaps,
    }
}

// ─── Public API: Layer 12 ──────────────────────────────────────────────────

/// Run the anchor analysis and report surface integrity.
///
/// Layer 12: formal invariant binding for all detection surfaces.
/// Must be called AFTER redshift (Layer 11) so we know which surfaces
/// survived and which broke.
pub fn anchor_sweep(
    _sys: &dyn Sys,
    _fs: &dyn Fs,
    _torque: &mut TorqueExecutor,
    report: &mut Report,
    redshift_survivals: &[String],
    redshift_gaps: &[String],
) -> AnchorResult {
    let anchors = build_anchor_catalog();
    let result = evaluate_anchors(&anchors, redshift_survivals, redshift_gaps);

    // Report unanchored survivals as warnings (detection works but we don't know why)
    for uid in &result.unanchored {
        report.warnings.push(crate::report::Warning {
            context: format!("ANCHOR-UNANCHORED: {uid}"),
            reason: "Detection surface survived adversarial shift but no formal \
                     invariant anchor explains why. Surface may be fragile.".into(),
        });
    }

    // Report uncoverable gaps
    for gid in &result.uncoverable_gaps {
        let mut finding = Finding::new(
            &format!("ANCHOR-UNCOVERABLE-{gid}"),
            Severity::High,
            format!(
                "Redshift gap {} has no anchor extension path — fundamental blind spot",
                gid,
            ),
        )
        .with_evidence(vec![
            format!("gap_id={gid}"),
            "No anchor class (thermodynamic, causal, observation) can cover this gap".into(),
            "Mitigation requires adding a new detection surface, not strengthening existing ones".into(),
        ])
        .with_recommendation(
            "This gap represents a fundamental limitation of the current detection architecture. \
             Addressing it requires either: (1) extending the detection domain model, or \
             (2) accepting the gap and documenting it as a known limitation with compensating controls.",
        )
        .with_confidence(0.90)
        .with_role(FindingRole::Core);
        finding.compute_surprise();
        report.push(finding);
    }

    // Report aggregate anchor metrics
    report.notes.push(format!(
        "[anchor] anchors={} mean_strength={:.2} min_strength={:.2} weakest={} \
         survival_coverage={:.0}% gap_coverage_potential={:.0}%",
        result.total_anchors,
        result.mean_strength,
        result.min_strength,
        result.weakest_anchor,
        result.survival_coverage * 100.0,
        result.gap_coverage_potential * 100.0,
    ));

    if !result.unanchored.is_empty() {
        report.notes.push(format!(
            "[anchor] UNANCHORED_SURVIVALS: {:?}", result.unanchored,
        ));
    }
    if !result.uncoverable_gaps.is_empty() {
        report.notes.push(format!(
            "[anchor] UNCOVERABLE_GAPS: {:?}", result.uncoverable_gaps,
        ));
    }

    result
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_catalog_covers_all_classes() {
        let anchors = build_anchor_catalog();
        let classes: Vec<InvariantClass> = anchors.iter().map(|a| a.class).collect();
        assert!(classes.contains(&InvariantClass::Thermodynamic));
        assert!(classes.contains(&InvariantClass::Causal));
        assert!(classes.contains(&InvariantClass::Observation));
    }

    #[test]
    fn test_all_anchors_have_proof_and_weakness() {
        let anchors = build_anchor_catalog();
        for a in &anchors {
            assert!(!a.proof_sketch.is_empty(), "Anchor {} lacks proof sketch", a.id);
            assert!(!a.known_weakness.is_empty(), "Anchor {} lacks known weakness (dishonest)", a.id);
            assert!(a.strength > 0.0 && a.strength <= 1.0, "Anchor {} has invalid strength {}", a.id, a.strength);
        }
    }

    #[test]
    fn test_survival_coverage() {
        let anchors = build_anchor_catalog();
        // Survivals from redshift catalog
        let survivals = vec![
            "T2-TIME-ZONE-PIVOT".to_string(),
            "T3-JITTER-FLOOD".to_string(),
            "S2-LOTL-PURE".to_string(),
            "S3-POLYGLOT-PAYLOAD".to_string(),
            "E2-ENCRYPTED-BLOB".to_string(),
            "P1-MEMONLY-PERSISTENCE".to_string(),
            "P2-PROCESS-TREE-SPREAD".to_string(),
            "N1-REVERSE-CLEANUP-ORDER".to_string(),
            "X1-TIMING-ORACLE-REPLAY".to_string(),
            "X3-COORDINATED-ASYNC-WINDOW".to_string(),
        ];
        let gaps = vec![
            "S1-IOC-ROTATION".to_string(),
            "E1-STEGO-LOW-DENSITY".to_string(),
            "E3-ENTROPY-MATCHING".to_string(),
            "P3-CGROUP-NAMESPACE-HIDE".to_string(),
            "N2-SELECTIVE-DOMAIN-CLEAN".to_string(),
            "N3-INCREMENTAL-TRUTHING".to_string(),
            "X2-CACHE-LINE-AVOIDANCE".to_string(),
        ];
        let result = evaluate_anchors(&anchors, &survivals, &gaps);

        // Should cover majority of survivals
        assert!(result.survival_coverage > 0.7,
            "Survival coverage too low: {:.0}%", result.survival_coverage * 100.0);

        // Should identify some gap coverage potential
        assert!(result.gap_coverage_potential > 0.3,
            "Gap coverage potential too low: {:.0}%", result.gap_coverage_potential * 100.0);

        // Should have at least one uncoverable gap (honest assessment)
        assert!(!result.uncoverable_gaps.is_empty(),
            "No uncoverable gaps — suspicious, some gaps should be fundamental");
    }

    #[test]
    fn test_mean_strength_reasonable() {
        let anchors = build_anchor_catalog();
        let survivals = vec![];
        let gaps = vec![];
        let result = evaluate_anchors(&anchors, &survivals, &gaps);
        // Mean strength should be substantial but not perfect
        assert!(result.mean_strength > 0.5, "Mean anchor strength too low");
        assert!(result.mean_strength < 1.0, "Mean anchor strength = 1.0 is dishonest");
    }

    #[test]
    fn test_no_anchor_claims_perfection() {
        let anchors = build_anchor_catalog();
        for a in &anchors {
            assert!(a.strength < 1.0,
                "Anchor {} claims perfect strength — no detection surface is physics-guaranteed \
                 without known limitations", a.id);
        }
    }

    #[test]
    fn test_weakest_anchor_identified() {
        let anchors = build_anchor_catalog();
        let result = evaluate_anchors(&anchors, &[], &[]);
        assert_ne!(result.weakest_anchor, "NONE");
        // Weakest should be the cache perturbation anchor (broken by hypervisor)
        assert_eq!(result.weakest_anchor, "ANCHOR-OBS-CACHE-PERTURBATION",
            "Expected cache perturbation to be weakest due to hypervisor gap");
    }
}

//! selfcheck-core: Stealth-normalized forensic toolkit for Cisco appliance compromise detection.
//!
//! Crate layout:
//!   - `io`       — Filesystem abstraction (`Fs` trait) for live and mock operation
//!   - `report`   — Findings, severity, and structured report output
//!   - `sys`      — Command execution abstraction (`Sys` trait)
//!   - `torque`   — Stealth I/O normalization layer (equilibrium, drift, executor)
//!   - `geom`     — Three-axis drift geometry and quadrant classification
//!   - `sonar`    — Three-band echo detection (acoustic / optical / gravitational)
//!   - `tripwire` — Uncertainty-maximized probabilistic detection + McFly lineage
//!   - `eris`     — Discordian derailment heuristics + Russian-doll paradox engine
//!   - `spinaltap` — Criticality inversion + invariant locks + density profiling
//!   - `redshift`  — Adversarial vector-shifting engine (counter-mechanic validation)
//!   - `anchor`    — Detection surface invariant proofs (physics-anchored guarantees)
//!   - `asyncos`  — AsyncOS (Secure Email Gateway) detection layers

pub mod io;
pub mod report;
pub mod sys;
pub mod torque;
pub mod geom;
pub mod sonar;
pub mod tripwire;
pub mod eris;
pub mod spinaltap;
pub mod redshift;
pub mod anchor;
pub mod asyncos;

pub use io::Fs;
pub use report::{Finding, FindingRole, Report, Severity, Warning};
pub use sys::{CmdResult, Sys};
pub use torque::{
    cumulative_size_fs, DriftEnvelope, DriftFinding, Equilibrium, ManifestBaseline,
    ManifestEntry, TorqueExecutor, TorquedStat,
};
pub use geom::{Vec3, Quadrant, Thresholds, SurpriseScore};
pub use sonar::{SonarSpread, Band, EchoDelta};
pub use tripwire::{ProbabilisticTripwire, LineageCheckpoint, LineageResult};
pub use eris::{Derailment, MatryoshkaShell, ParadoxBridge};
pub use spinaltap::{OptimizationAngle, InvariantLock, FuturesCentroid, DensityProfile};
pub use redshift::{VectorShift, ShiftClass, RedshiftResult};
pub use anchor::{SurfaceAnchor, InvariantClass, AnchorResult};

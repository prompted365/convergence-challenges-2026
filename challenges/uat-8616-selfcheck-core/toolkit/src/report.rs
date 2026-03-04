//! Structured forensic report with findings, warnings, and notes.
//!
//! Assessment Recommendation #2: Adds a `warnings` vector that distinguishes
//! "checked and clean" from "unable to check" — critical for defenders to know
//! when a path was inaccessible (permissions, missing device, etc.) vs. when it
//! was examined and found benign.

use std::fmt;

// ─── Severity levels ─────────────────────────────────────────────────────────

/// Finding severity, ordered for sort/filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info     => write!(f, "INFO"),
            Self::Low      => write!(f, "LOW"),
            Self::Medium   => write!(f, "MEDIUM"),
            Self::High     => write!(f, "HIGH"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}

// ─── Finding ─────────────────────────────────────────────────────────────────

/// A single forensic finding.
///
/// Swallowtail-inspired: each finding carries a `confidence` score (0.0–1.0)
/// computed via gaussian-splat weighting of evidence strength, and a `role`
/// classification (Core, Bridge, Periphery) indicating structural importance
/// within the overall investigation graph.
#[derive(Debug, Clone)]
pub struct Finding {
    /// Machine-readable identifier (e.g., "ASYNCOS-AQUASHELL-PATH-PRESENT")
    pub id: String,
    pub severity: Severity,
    pub title: String,
    pub evidence: Vec<String>,
    pub recommendation: Option<String>,
    /// Gaussian-splat weighted confidence (0.0 = speculative, 1.0 = certain).
    /// Derived from evidence proximity to known indicators.
    /// Swallowtail fragment: effective_score = 0.7 * direct + 0.3 * splat_weighted.
    pub confidence: f64,
    /// Structural role within the investigation.
    /// Swallowtail fragment: Core (◉), Bridge (⊕), Periphery (◎), Orphan (○).
    pub role: FindingRole,
    /// Surprise score: high direct match + core role + no drift = low surprise;
    /// moderate match + bridge role + transformation = high surprise.
    /// Higher surprise findings warrant immediate analyst attention.
    pub surprise: f64,
}

/// Structural classification of a finding's role in the investigation graph.
/// Adapted from swallowtail's Emergence phase role taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingRole {
    /// ◉ Core — finding sits at the centroid of the investigation
    Core,
    /// ⊕ Bridge — connects otherwise-separate clusters of evidence
    Bridge,
    /// ◎ Periphery — supports but does not anchor the investigation
    Periphery,
    /// ○ Orphan — isolated finding, no structural connection to others
    Orphan,
}

impl Default for FindingRole {
    fn default() -> Self {
        Self::Orphan
    }
}

impl fmt::Display for FindingRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core       => write!(f, "◉ Core"),
            Self::Bridge     => write!(f, "⊕ Bridge"),
            Self::Periphery  => write!(f, "◎ Periphery"),
            Self::Orphan     => write!(f, "○ Orphan"),
        }
    }
}

impl Default for Finding {
    fn default() -> Self {
        Self {
            id: String::new(),
            severity: Severity::Info,
            title: String::new(),
            evidence: vec![],
            recommendation: None,
            confidence: 0.5,
            role: FindingRole::Orphan,
            surprise: 0.0,
        }
    }
}

impl Finding {
    /// Builder-pattern constructor for ergonomic finding creation.
    pub fn new(id: impl Into<String>, severity: Severity, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            severity,
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn with_evidence(mut self, evidence: Vec<String>) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn with_recommendation(mut self, rec: impl Into<String>) -> Self {
        self.recommendation = Some(rec.into());
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    pub fn with_role(mut self, role: FindingRole) -> Self {
        self.role = role;
        self
    }

    /// Compute surprise score.
    /// Swallowtail formula: low confidence + bridge/orphan role = high surprise.
    pub fn compute_surprise(&mut self) {
        let role_weight = match self.role {
            FindingRole::Core      => 0.2,
            FindingRole::Bridge    => 0.7,
            FindingRole::Periphery => 0.4,
            FindingRole::Orphan    => 0.9,
        };
        // Surprise is inversely proportional to confidence, amplified by role weight
        self.surprise = (1.0 - self.confidence) * role_weight;
    }
}

// ─── Warning ─────────────────────────────────────────────────────────────────

/// A non-finding diagnostic: something the tool could not check.
/// Assessment Recommendation #2: "checked and clean" is the absence of a Finding;
/// a Warning means "unable to check — permission denied, path missing, etc."
#[derive(Debug, Clone)]
pub struct Warning {
    pub context: String,
    pub reason: String,
}

impl Warning {
    pub fn new(context: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            context: context.into(),
            reason: reason.into(),
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⚠ {}: {}", self.context, self.reason)
    }
}

// ─── Report ──────────────────────────────────────────────────────────────────

/// Complete investigation report.
#[derive(Debug, Clone)]
pub struct Report {
    /// Forensic findings (positive detections).
    pub findings: Vec<Finding>,
    /// Diagnostic warnings (paths/ops that could not be checked).
    pub warnings: Vec<Warning>,
    /// Investigator notes (ambient context, command output, etc.).
    pub notes: Vec<String>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            findings: vec![],
            warnings: vec![],
            notes: vec![],
        }
    }

    /// Add a finding.
    pub fn push(&mut self, finding: Finding) {
        self.findings.push(finding);
    }

    /// Add a warning (could-not-check diagnostic).
    pub fn warn(&mut self, warning: Warning) {
        self.warnings.push(warning);
    }

    /// Highest severity across all findings, or None if clean.
    pub fn max_severity(&self) -> Option<Severity> {
        self.findings.iter().map(|f| f.severity).max()
    }

    /// Count findings at or above a given severity.
    pub fn count_at_or_above(&self, threshold: Severity) -> usize {
        self.findings.iter().filter(|f| f.severity >= threshold).count()
    }

    /// Findings sorted by surprise (highest first) for analyst triage.
    pub fn by_surprise(&self) -> Vec<&Finding> {
        let mut sorted: Vec<_> = self.findings.iter().collect();
        sorted.sort_by(|a, b| b.surprise.partial_cmp(&a.surprise).unwrap_or(std::cmp::Ordering::Equal));
        sorted
    }

    /// Render a human-readable summary.
    pub fn summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "═══ REPORT: {} findings, {} warnings, {} notes ═══\n",
            self.findings.len(),
            self.warnings.len(),
            self.notes.len(),
        ));
        if let Some(max) = self.max_severity() {
            out.push_str(&format!("Highest severity: {max}\n"));
        }

        for f in &self.findings {
            out.push_str(&format!(
                "\n[{}] {} — {} (confidence={:.0}%, role={}, surprise={:.2})\n",
                f.severity, f.id, f.title, f.confidence * 100.0, f.role, f.surprise,
            ));
            for e in &f.evidence {
                out.push_str(&format!("  ├ {e}\n"));
            }
            if let Some(rec) = &f.recommendation {
                out.push_str(&format!("  └ recommendation: {rec}\n"));
            }
        }

        if !self.warnings.is_empty() {
            out.push_str("\n─── Warnings (unable to check) ───\n");
            for w in &self.warnings {
                out.push_str(&format!("  {w}\n"));
            }
        }

        out
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_builder_chain() {
        let f = Finding::new("TEST-001", Severity::High, "Test finding")
            .with_evidence(vec!["e1".into()])
            .with_recommendation("do something")
            .with_confidence(0.85)
            .with_role(FindingRole::Core);
        assert_eq!(f.severity, Severity::High);
        assert_eq!(f.confidence, 0.85);
        assert_eq!(f.role, FindingRole::Core);
    }

    #[test]
    fn surprise_scoring() {
        let mut f = Finding::new("T", Severity::Medium, "test")
            .with_confidence(0.3)
            .with_role(FindingRole::Bridge);
        f.compute_surprise();
        assert!(f.surprise > 0.4, "bridge + low confidence should produce high surprise");

        let mut g = Finding::new("T", Severity::Medium, "test")
            .with_confidence(0.95)
            .with_role(FindingRole::Core);
        g.compute_surprise();
        assert!(g.surprise < 0.1, "core + high confidence should produce low surprise");
    }

    #[test]
    fn report_max_severity() {
        let mut r = Report::new();
        r.push(Finding::new("A", Severity::Low, "low"));
        r.push(Finding::new("B", Severity::Critical, "crit"));
        r.push(Finding::new("C", Severity::Medium, "med"));
        assert_eq!(r.max_severity(), Some(Severity::Critical));
    }

    #[test]
    fn report_warnings_distinguished() {
        let mut r = Report::new();
        r.warn(Warning::new("Layer 2", "Permission denied: /data/tmp"));
        r.warn(Warning::new("Layer 3", "ss command not found"));
        assert_eq!(r.warnings.len(), 2);
        assert!(r.findings.is_empty()); // warnings ≠ findings
    }
}

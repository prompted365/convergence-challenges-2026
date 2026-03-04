//! Three-axis drift geometry and quadrant classification.
//!
//! Maps three invariant families into a 3D vector space:
//!   x — mass drift (cumulative directory size deviation %)
//!   y — hash drift (fraction of files with changed SHA-256)
//!   z — log drift (log-entry count deviation %)
//!
//! The quadrant classifier bins the resulting vector into one of four
//! operational quadrants for analyst triage. Per the technical assessment,
//! axes are partially correlated, so quadrant should be presented as a
//! *supplementary* signal alongside per-axis percentage breakdowns.
//!
//! Swallowtail fragment: surprise scoring integrates gaussian-splat
//! weighted drift magnitude with structural role context.

use std::fmt;

// ─── Vec3 ───────────────────────────────────────────────────────────────────

/// A point in three-axis drift space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// Mass drift (cumulative directory size deviation, %).
    pub x: f64,
    /// Hash drift (fraction of files with changed hash, 0.0–1.0).
    pub y: f64,
    /// Log drift (log-entry count deviation, %).
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Euclidean magnitude.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Dot product.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product.
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Component-wise subtraction.
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}%, {:.2}, {:.2}%)", self.x, self.y, self.z)
    }
}

// ─── Torque & Angle ─────────────────────────────────────────────────────────

/// Cross-product magnitude — "torque" of the drift vector pair.
/// Measures the area of the parallelogram spanned by two drift snapshots,
/// indicating how *differently* the system is drifting between observations.
pub fn torque(a: &Vec3, b: &Vec3) -> f64 {
    a.cross(b).magnitude()
}

/// Angle in radians between two drift vectors.
/// Returns 0.0 for zero-length vectors (defensive).
pub fn angle(a: &Vec3, b: &Vec3) -> f64 {
    let ma = a.magnitude();
    let mb = b.magnitude();
    if ma < 1e-12 || mb < 1e-12 {
        return 0.0;
    }
    let cos_theta = (a.dot(b) / (ma * mb)).clamp(-1.0, 1.0);
    cos_theta.acos()
}

/// Angular velocity: angle change per unit time.
/// `dt` is the elapsed seconds between the two snapshots.
pub fn angular_velocity(a: &Vec3, b: &Vec3, dt_secs: f64) -> f64 {
    if dt_secs <= 0.0 {
        return 0.0;
    }
    angle(a, b) / dt_secs
}

// ─── Quadrant Classification ────────────────────────────────────────────────

/// Operational quadrant for analyst triage.
///
/// Per technical assessment §6: axes are partially correlated, so quadrant
/// is a supplementary signal. Always present per-axis percentages alongside.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quadrant {
    /// High magnitude, stable direction — persistent compromise.
    StableAnomaly,
    /// Growing magnitude — active adversary action or expanding implant.
    GrowingDrift,
    /// Shrinking magnitude — remediation in progress or implant self-cleaning.
    ShrinkingDrift,
    /// Low magnitude, moving — noise or benign churn near baseline.
    NearBaselineMoving,
}

impl fmt::Display for Quadrant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StableAnomaly     => write!(f, "STABLE-ANOMALY"),
            Self::GrowingDrift      => write!(f, "GROWING-DRIFT"),
            Self::ShrinkingDrift    => write!(f, "SHRINKING-DRIFT"),
            Self::NearBaselineMoving => write!(f, "NEAR-BASELINE-MOVING"),
        }
    }
}

/// Thresholds governing quadrant classification boundaries.
#[derive(Debug, Clone)]
pub struct Thresholds {
    /// Mass drift threshold (% deviation) above which drift is significant.
    pub mass_pct: f64,
    /// Hash drift threshold (fraction) above which drift is significant.
    pub hash_pct: f64,
    /// Log drift threshold (% deviation) above which drift is significant.
    pub log_pct: f64,
    /// Magnitude growth rate threshold: if delta_magnitude / dt > this, growing.
    pub growth_rate: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            mass_pct: 2.0,
            hash_pct: 0.01,
            log_pct: 5.0,
            growth_rate: 0.1,
        }
    }
}

/// Classify a pair of drift snapshots into a quadrant.
///
/// `current` — latest drift vector.
/// `previous` — prior drift vector (use `Vec3::zero()` if first sample).
/// `dt_secs` — elapsed seconds between the two snapshots.
pub fn classify(current: &Vec3, previous: &Vec3, dt_secs: f64, thresholds: &Thresholds) -> Quadrant {
    let mag = current.magnitude();
    let prev_mag = previous.magnitude();

    // Is the current drift significant on any axis?
    let significant = current.x.abs() > thresholds.mass_pct
        || current.y.abs() > thresholds.hash_pct
        || current.z.abs() > thresholds.log_pct;

    if !significant {
        return Quadrant::NearBaselineMoving;
    }

    // Magnitude delta rate
    let delta_mag = mag - prev_mag;
    let rate = if dt_secs > 0.0 { delta_mag / dt_secs } else { 0.0 };

    if rate > thresholds.growth_rate {
        Quadrant::GrowingDrift
    } else if rate < -thresholds.growth_rate {
        Quadrant::ShrinkingDrift
    } else {
        Quadrant::StableAnomaly
    }
}

// ─── Surprise Score ─────────────────────────────────────────────────────────

/// Composite surprise score combining geometric drift with swallowtail weighting.
///
/// Swallowtail fragment: gaussian splat weights the geometric magnitude
/// by the inverse of expected drift, producing high surprise when a device
/// drifts along unexpected axes or at unexpected rates.
#[derive(Debug, Clone)]
pub struct SurpriseScore {
    /// Quadrant classification.
    pub quadrant: Quadrant,
    /// Drift magnitude (Euclidean norm of drift vector).
    pub magnitude: f64,
    /// Per-axis breakdown for supplementary reporting.
    pub per_axis: Vec3,
    /// Gaussian-splat weighted surprise (0.0 = expected, 1.0 = maximally surprising).
    /// w = e^(-d²/2σ²) where d is magnitude, σ is threshold-derived spread.
    pub surprise: f64,
    /// Cross-product torque between current and previous snapshots.
    pub torque: f64,
    /// Angle between current and previous drift vectors (radians).
    pub angle_rad: f64,
}

impl SurpriseScore {
    /// Compute a full surprise score from two consecutive drift snapshots.
    pub fn compute(
        current: &Vec3,
        previous: &Vec3,
        dt_secs: f64,
        thresholds: &Thresholds,
    ) -> Self {
        let quadrant = classify(current, previous, dt_secs, thresholds);
        let magnitude = current.magnitude();
        let t = torque(current, previous);
        let a = angle(current, previous);

        // Gaussian-splat surprise: σ derived from threshold magnitude
        let sigma = (thresholds.mass_pct.powi(2)
            + thresholds.hash_pct.powi(2)
            + thresholds.log_pct.powi(2))
        .sqrt();
        // Inverse gaussian: high magnitude → high surprise (NOT the usual decay)
        let surprise = if sigma > 1e-12 {
            1.0 - (-magnitude.powi(2) / (2.0 * sigma.powi(2))).exp()
        } else {
            0.0
        };

        Self {
            quadrant,
            magnitude,
            per_axis: *current,
            surprise: surprise.clamp(0.0, 1.0),
            torque: t,
            angle_rad: a,
        }
    }
}

impl fmt::Display for SurpriseScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (mag={:.3}, surprise={:.0}%, torque={:.3}, angle={:.2}°) axes={}",
            self.quadrant,
            self.magnitude,
            self.surprise * 100.0,
            self.torque,
            self.angle_rad.to_degrees(),
            self.per_axis,
        )
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_magnitude() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn vec3_cross_product() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = a.cross(&b);
        assert!((c.z - 1.0).abs() < 1e-10);
        assert!(c.x.abs() < 1e-10);
        assert!(c.y.abs() < 1e-10);
    }

    #[test]
    fn torque_orthogonal() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert!((torque(&a, &b) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn torque_parallel_is_zero() {
        let a = Vec3::new(2.0, 3.0, 1.0);
        let b = Vec3::new(4.0, 6.0, 2.0);
        assert!(torque(&a, &b) < 1e-10);
    }

    #[test]
    fn angle_orthogonal() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let ang = angle(&a, &b);
        assert!((ang - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    }

    #[test]
    fn angle_zero_vector_safe() {
        let a = Vec3::zero();
        let b = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(angle(&a, &b), 0.0);
    }

    #[test]
    fn classify_near_baseline() {
        let current = Vec3::new(0.5, 0.001, 1.0);
        let prev = Vec3::zero();
        let t = Thresholds::default();
        assert_eq!(classify(&current, &prev, 60.0, &t), Quadrant::NearBaselineMoving);
    }

    #[test]
    fn classify_growing_drift() {
        let prev = Vec3::new(3.0, 0.02, 6.0);
        // Magnitude of prev ≈ 6.71
        // Make current much larger so rate > growth_rate
        let current = Vec3::new(30.0, 0.2, 60.0);
        // Magnitude of current ≈ 67.1, delta ≈ 60.4, rate = 60.4/10 = 6.04
        let t = Thresholds::default();
        assert_eq!(classify(&current, &prev, 10.0, &t), Quadrant::GrowingDrift);
    }

    #[test]
    fn classify_shrinking_drift() {
        let prev = Vec3::new(30.0, 0.2, 60.0);
        let current = Vec3::new(3.0, 0.02, 6.0);
        let t = Thresholds::default();
        assert_eq!(classify(&current, &prev, 10.0, &t), Quadrant::ShrinkingDrift);
    }

    #[test]
    fn classify_stable_anomaly() {
        let prev = Vec3::new(5.0, 0.05, 10.0);
        let current = Vec3::new(5.1, 0.05, 10.1);
        let t = Thresholds::default();
        assert_eq!(classify(&current, &prev, 60.0, &t), Quadrant::StableAnomaly);
    }

    #[test]
    fn surprise_score_low_for_baseline() {
        let current = Vec3::new(0.1, 0.001, 0.5);
        let prev = Vec3::zero();
        let s = SurpriseScore::compute(&current, &prev, 60.0, &Thresholds::default());
        assert!(s.surprise < 0.1, "near-baseline should have low surprise: {}", s.surprise);
    }

    #[test]
    fn surprise_score_high_for_large_drift() {
        let current = Vec3::new(50.0, 0.5, 100.0);
        let prev = Vec3::zero();
        let s = SurpriseScore::compute(&current, &prev, 60.0, &Thresholds::default());
        assert!(s.surprise > 0.9, "large drift should have high surprise: {}", s.surprise);
    }
}

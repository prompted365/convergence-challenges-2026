//! AsyncOS (Cisco Secure Email Gateway) detection layers — upgraded per
//! Assessment Recommendations #11–#14.
//!
//! Changes from `0/asyncos.rs`:
//!   • Rec #11 — Known-good file diffing for AquaShell instead of string heuristics alone.
//!              Compares index.py hash against a known-clean baseline hash.
//!   • Rec #12 — Expanded IOC hash set, extended candidate directories (/opt, /usr/local,
//!              /var/run), and file size filter (skip files > 50 MB or < 100 bytes).
//!   • Rec #13 — External IOC file support: load hashes/IPs from a JSON sidecar so
//!              indicators can be updated without a rebuild.
//!   • Rec #14 — Known-good process baseline: compare running processes against an
//!              expected set, flag unexpected entries.
//!
//! Additional structural upgrades:
//!   • All filesystem access through `&dyn Fs` (no raw `std::fs`)
//!   • Uses `Finding::new()` builder with confidence / role / surprise scoring
//!   • Uses `Warning` for inaccessible paths and failed commands
//!   • Accepts `&dyn Sys` instead of concrete `&Sys`
//!   • ManifestBaseline uses ManifestEntry with per-path sigma_bytes

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::io::Fs;
use crate::report::{Finding, FindingRole, Report, Severity, Warning};
use crate::sys::Sys;
use crate::geom::{SurpriseScore, Thresholds as GeomThresholds, Vec3};
use crate::sonar::sonar_sweep;
use crate::tripwire::run_tripwire_suite;
use crate::eris::eris_sweep;
use crate::spinaltap::spinaltap_sweep;
use crate::redshift::redshift_sweep;
use crate::anchor::anchor_sweep;
use crate::torque::{ManifestBaseline, ManifestEntry, TorqueExecutor};

// ─── Constants ──────────────────────────────────────────────────────────────

/// Default path for the external IOC sidecar file (Rec #13).
const IOC_SIDECAR_PATH: &str = "/etc/selfcheck/ioc_asyncos.json";

/// Skip files larger than this during IOC hash scan (Rec #12).
const IOC_MAX_FILE_BYTES: u64 = 50 * 1024 * 1024; // 50 MB
/// Skip files smaller than this during IOC hash scan (Rec #12).
const IOC_MIN_FILE_BYTES: u64 = 100;

// ─── IOC Definitions ────────────────────────────────────────────────────────

/// An IOC hash entry: malware name + SHA-256.
struct IocHash {
    name: &'static str,
    sha256: &'static str,
}

/// Built-in IOC hashes (fallback when sidecar is absent).
const BUILTIN_IOC_HASHES: &[IocHash] = &[
    IocHash {
        name: "AquaTunnel",
        sha256: "2db8ad6e0f43e93cc557fbda0271a436f9f2a478b1607073d4ee3d20a87ae7ef",
    },
    IocHash {
        name: "AquaPurge",
        sha256: "145424de9f7d5dd73b599328ada03aa6d6cdcee8d5fe0f7cb832297183dbe4ca",
    },
    IocHash {
        name: "Chisel",
        sha256: "85a0b22bd17f7f87566bd335349ef89e24a5a19f899825b4d178ce6240f58bfc",
    },
];

/// Built-in IOC IPs (fallback when sidecar is absent).
const BUILTIN_IOC_IPS: &[&str] = &["172.233.67.176", "172.237.29.147", "38.54.56.95"];

/// Candidate directories for IOC hash scan (Rec #12 — expanded set).
const CANDIDATE_DIRS: &[&str] = &[
    "/tmp",
    "/var/tmp",
    "/dev/shm",
    "/root",
    "/home",
    "/data",
    "/data/tmp",
    "/data/logs",
    "/opt",
    "/usr/local",
    "/var/run",
];

/// Known-good process names expected on a clean AsyncOS appliance (Rec #14).
/// These are illustrative — populate from a verified clean image.
const KNOWN_GOOD_PROCESSES: &[&str] = &[
    "init",
    "kthreadd",
    "sshd",
    "crond",
    "syslogd",
    "rsyslogd",
    "scanengine",
    "milter",
    "ldap_router",
    "quarantine",
    "reporting",
    "updater",
    "policy",
    "antispam",
    "antivirus",
    "snmpd",
    "httpd",
];

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Join root with an absolute-looking path, stripping the leading slash.
fn p(root: &Path, abs: &str) -> PathBuf {
    let rel = abs.strip_prefix('/').unwrap_or(abs);
    root.join(rel)
}

/// Inline SHA-256 using the same pure-Rust implementation from torque.rs.
/// Duplicated here to avoid coupling to a private function; the compiler will
/// deduplicate if LTO is on.
#[cfg(test)]
fn sha256_hex(data: &[u8]) -> String {
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
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
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
            hh = g; g = f; f = e; e = d.wrapping_add(t1);
            d = c; c = b; b = a; a = t1.wrapping_add(t2);
        }
        for (i, v) in [a, b, c, d, e, f, g, hh].iter().enumerate() {
            h[i] = h[i].wrapping_add(*v);
        }
    }
    h.iter().map(|v| format!("{v:08x}")).collect()
}

// ─── External IOC loader (Rec #13) ─────────────────────────────────────────

/// Runtime IOC set — either loaded from sidecar or falling back to built-in.
struct IocSet {
    hashes: Vec<(String, String)>, // (name, sha256)
    ips: Vec<String>,
}

impl IocSet {
    /// Try to load from a JSON sidecar file.  Format:
    /// ```json
    /// {
    ///   "hashes": [{"name": "AquaTunnel", "sha256": "abc..."}],
    ///   "ips": ["1.2.3.4"]
    /// }
    /// ```
    /// Falls back to built-in indicators on any error.
    fn load(fs: &dyn Fs, sidecar_path: &Path) -> Self {
        if let Ok(text) = fs.read_to_string(sidecar_path) {
            if let Some(set) = Self::parse_json(&text) {
                return set;
            }
        }
        // Fallback: built-in indicators
        Self {
            hashes: BUILTIN_IOC_HASHES
                .iter()
                .map(|h| (h.name.to_string(), h.sha256.to_string()))
                .collect(),
            ips: BUILTIN_IOC_IPS.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Minimal JSON parser — no serde dependency.
    /// Extracts "hashes" array of {name, sha256} objects and "ips" string array.
    fn parse_json(text: &str) -> Option<Self> {
        // Strategy: find "hashes": [...] and "ips": [...]
        // This is intentionally simple — we don't need a full JSON parser for
        // a controlled config file format.
        let mut hashes = Vec::new();
        let mut ips = Vec::new();

        // Extract hashes array
        if let Some(hashes_start) = text.find("\"hashes\"") {
            let rest = &text[hashes_start..];
            if let Some(arr_start) = rest.find('[') {
                if let Some(arr_end) = rest[arr_start..].find(']') {
                    let arr_content = &rest[arr_start + 1..arr_start + arr_end];
                    // Parse each {name, sha256} object
                    for obj in arr_content.split('}') {
                        let name = extract_json_string(obj, "name");
                        let sha = extract_json_string(obj, "sha256");
                        if let (Some(n), Some(s)) = (name, sha) {
                            hashes.push((n, s));
                        }
                    }
                }
            }
        }

        // Extract ips array
        if let Some(ips_start) = text.find("\"ips\"") {
            let rest = &text[ips_start..];
            if let Some(arr_start) = rest.find('[') {
                if let Some(arr_end) = rest[arr_start..].find(']') {
                    let arr_content = &rest[arr_start + 1..arr_start + arr_end];
                    for segment in arr_content.split(',') {
                        let trimmed = segment.trim().trim_matches('"').trim();
                        if !trimmed.is_empty() {
                            ips.push(trimmed.to_string());
                        }
                    }
                }
            }
        }

        if hashes.is_empty() && ips.is_empty() {
            None
        } else {
            Some(Self { hashes, ips })
        }
    }
}

/// Extract a JSON string value for a given key from a text fragment.
fn extract_json_string(text: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"", key);
    let idx = text.find(&pattern)?;
    let after_key = &text[idx + pattern.len()..];
    // Skip colon and whitespace
    let after_colon = after_key.trim_start().strip_prefix(':')?;
    let after_ws = after_colon.trim_start();
    // Extract quoted value
    let stripped = after_ws.strip_prefix('"')?;
    let end = stripped.find('"')?;
    Some(stripped[..end].to_string())
}

// ─── Baseline builder ───────────────────────────────────────────────────────

/// Known-good cumulative directory sizes for supported AsyncOS releases.
/// These are the counterweights — a scalar per tree that encodes aggregate state.
/// Populate from a known-clean image for each release you support.
/// Values here are illustrative placeholders; replace with real baselines.
fn baseline_for_version(version: &str) -> ManifestBaseline {
    // Rec #9: ManifestEntry with per-path sigma_bytes where historical data exists
    let entries = vec![
        ManifestEntry {
            path: PathBuf::from("/data/web/euq_webui/htdocs"),
            expected_size: 4_200_000,
            sigma_bytes: Some(50_000.0), // low variance — tightly controlled web root
        },
        ManifestEntry {
            path: PathBuf::from("/data/web/euq_webui"),
            expected_size: 8_100_000,
            sigma_bytes: Some(120_000.0),
        },
        ManifestEntry {
            path: PathBuf::from("/data/web"),
            expected_size: 52_000_000,
            sigma_bytes: None, // high natural variance — use uniform % threshold
        },
    ];
    ManifestBaseline {
        version: version.to_string(),
        entries,
    }
}

// ─── Known-good AquaShell baseline (Rec #11) ───────────────────────────────

/// SHA-256 of a known-clean `index.py` for each supported version.
/// Populate from a verified clean image snapshot.
/// Format: (version_prefix, expected_sha256)
const KNOWN_GOOD_INDEX_PY: &[(&str, &str)] = &[
    // Placeholder hashes — replace with real values from clean images
    ("15.5", "0000000000000000000000000000000000000000000000000000000000000000"),
    ("15.0", "0000000000000000000000000000000000000000000000000000000000000000"),
    ("14.4", "0000000000000000000000000000000000000000000000000000000000000000"),
];

/// String-based heuristic indicators for AquaShell (Layer 1 fallback).
const AQUASHELL_HEURISTICS: &[&str] = &[
    "base64",
    "b64decode",
    "subprocess",
    "os.system",
    "Popen",
    "exec(",
    "eval(",
    "socket.socket",
    "import paramiko",
    "threading.Thread",
];

// ─── Main entry point ───────────────────────────────────────────────────────

/// Run all AsyncOS detection layers.
///
/// Upgraded signature: accepts `&dyn Sys`, `&dyn Fs`, and `&mut Report`.
pub fn run_all(
    sys: &dyn Sys,
    fs: &dyn Fs,
    root: &Path,
    report: &mut Report,
) -> Result<()> {
    // ── Establish equilibrium before touching any forensic surface ─────────
    let mut torque = TorqueExecutor::new(fs);

    // ── Load IOC set (Rec #13 — external sidecar with built-in fallback) ──
    let ioc_set = IocSet::load(fs, &p(root, IOC_SIDECAR_PATH));

    // ── Layer 0: environment facts (torqued) ──────────────────────────────
    match sys.run("uname", &["-a"]) {
        Ok(r) => report.notes.push(r.summarize(4096)),
        Err(e) => report.warn(Warning::new(
            "Layer 0: uname",
            format!("Command failed: {e}"),
        )),
    }

    // ── Layer 1: AquaShell implant detection (Rec #11 — diff + heuristic) ─
    layer_1_aquashell(&mut torque, fs, sys, root, report, &ioc_set)?;

    // ── Layer 2: IOC hash scan (Rec #12 — expanded dirs + size filter) ────
    layer_2_ioc_hash(&mut torque, fs, root, report, &ioc_set)?;

    // ── Layer 3: IOC IP check ─────────────────────────────────────────────
    layer_3_ioc_ip(sys, &mut torque, report, &ioc_set)?;

    // ── Layer 4: Manifest drift check (Rec #9 — z-score thresholds) ──────
    layer_4_manifest_drift(sys, fs, &mut torque, root, report)?;

    // ── Layer 5: Process baseline (Rec #14 — known-good comparison) ──────
    layer_5_process_baseline(sys, &mut torque, report)?;

    // ── Layer 5b: SSH persistence audit (Hunt Guide rule mapping) ────────
    //    Maps directly to ACSC Hunt Guide:
    //      PERSISTENCE-T1098.004-RootSSHKeyAccepted-013
    //    and Talos TTPs for UAT-8616 (authorized_keys + PermitRootLogin).
    layer_5b_ssh_persistence(sys, fs, &mut torque, root, report)?;

    // ── Layer 6: Geometric drift summary (Rec #15 — supplementary signal) ──
    layer_6_geometric_summary(report);

    // ── Layer 7: Sonar echo detection (acoustic / optical / gravitational) ──
    //    Forces attacker mass to respond — any observation of our reads shifts
    //    cache lines, timing profiles, or structural topology measurably.
    let _spread = sonar_sweep(sys, fs, &mut torque, report);

    // ── Layer 8: Tripwire suite (probabilistic + McFly lineage + counter-signal) ──
    //    Uncertainty-maximized detection: attacker cannot know if they have been
    //    detected, and decoy signals create false-positive noise for the observer.
    run_tripwire_suite(sys, fs, root, &mut torque, report, 0xDEAD_BEEF);

    // ── Layer 9: Eris sweep (Discordian derailment + Russian-doll paradox) ──
    //    Heuristic derailment detection along nested trajectories. Paradox bridges
    //    hop semantically incompatible lanes to crash attacker computation engines.
    eris_sweep(sys, fs, &mut torque, report);

    // ── Layer 10: SpinalTap (criticality inversion + invariant locks + density) ──
    //    "These go to eleven." Models attacker optimization angles at CVSS 10.0,
    //    inverts them, illegitimizes epoch grammar via invariant locks, maps futures
    //    array centroids, and scans for density anomalies in syntactic structures.
    spinaltap_sweep(sys, fs, &mut torque, report);

    // ── Layer 11: Redshift (adversarial vector-shifting counter-mechanic) ──
    //    Counter-validation: shifts threat vectors AGAINST our own detectors.
    //    Tests whether surfaces hold under adversarial mutation or break.
    //    Reports confirmed gaps as findings with mitigation paths.
    let redshift_result = redshift_sweep(sys, fs, &mut torque, report);

    // ── Layer 12: Anchor (surface invariant proofs) ──
    //    Formal binding: proves WHY surviving surfaces hold by anchoring
    //    them to physical invariants (thermodynamic, causal, observation).
    //    Reports unanchored survivals and uncoverable gaps.
    let survival_ids: Vec<String> = redshift_result.catalog.iter()
        .filter(|s| s.actual_survives == Some(true))
        .map(|s| s.id.clone())
        .collect();
    let gap_ids: Vec<String> = redshift_result.catalog.iter()
        .filter(|s| s.actual_survives == Some(false))
        .map(|s| s.id.clone())
        .collect();
    let _anchor_result = anchor_sweep(sys, fs, &mut torque, report, &survival_ids, &gap_ids);

    // Append envelope summary for audit / threshold tuning
    report.notes.push(torque.envelope_summary());

    Ok(())
}

// ─── Layer 1: AquaShell (Rec #11) ──────────────────────────────────────────

fn layer_1_aquashell(
    torque: &mut TorqueExecutor,
    fs: &dyn Fs,
    sys: &dyn Sys,
    root: &Path,
    report: &mut Report,
    _ioc_set: &IocSet,
) -> Result<()> {
    let aquashell_path = p(root, "/data/web/euq_webui/htdocs/index.py");

    if !fs.exists(&aquashell_path) {
        // File absent — record as note, not finding
        report.notes.push(format!(
            "Layer 1: {} does not exist (expected on clean AsyncOS)",
            aquashell_path.display()
        ));
        return Ok(());
    }

    // Read content, stat, and hash through torque
    let content = torque.torqued_read(&aquashell_path);
    let stat = torque.torqued_stat(&aquashell_path);
    let hash = torque.torqued_sha256(&aquashell_path);

    let content_str = match content {
        Ok(s) => s,
        Err(e) => {
            report.warn(Warning::new(
                "Layer 1: AquaShell read",
                format!("Cannot read {}: {e}", aquashell_path.display()),
            ));
            return Ok(());
        }
    };

    let file_hash = hash.unwrap_or_default();

    let mut evidence = Vec::new();
    if let Ok(ref st) = stat {
        evidence.push(format!(
            "size={} modified={:?}",
            st.size_bytes,
            st.modified
        ));
    }
    if !file_hash.is_empty() {
        evidence.push(format!("sha256={file_hash}"));
    }

    // ── Rec #11: Known-good file diffing ──
    // Determine version for baseline lookup
    let version = torque.exec(|| {
        sys.run("cat", &["/etc/asyncos_version"])
            .ok()
            .and_then(|r| {
                if r.success() {
                    Some(r.stdout.trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    });

    let known_good_match = KNOWN_GOOD_INDEX_PY
        .iter()
        .find(|(prefix, _)| version.starts_with(prefix));

    // A known-good hash of all zeros is a placeholder — treat as "no baseline"
    let is_placeholder = |h: &str| -> bool {
        !h.is_empty() && h.chars().all(|c| c == '0')
    };

    let (hash_verdict, _confidence, role) = match known_good_match {
        Some((_, expected_hash)) if !file_hash.is_empty() && !is_placeholder(expected_hash) => {
            if file_hash == *expected_hash {
                // Hash matches known-good baseline — very likely clean
                evidence.push(format!("known_good_match=true (version={version})"));
                ("clean_baseline_match", 0.95, FindingRole::Periphery)
            } else {
                // Hash differs from known-good — this is the strongest signal
                evidence.push(format!(
                    "known_good_match=false expected={expected_hash} version={version}"
                ));
                ("baseline_mismatch", 0.90, FindingRole::Core)
            }
        }
        _ => {
            // No known-good baseline available (or placeholder hash) — fall back to heuristics only
            evidence.push("known_good_baseline=unavailable".to_string());
            ("no_baseline", 0.50, FindingRole::Bridge)
        }
    };

    // ── String heuristic analysis (always run as supplementary signal) ──
    let suspicious_hits: Vec<&str> = AQUASHELL_HEURISTICS
        .iter()
        .copied()
        .filter(|k| content_str.contains(k))
        .collect();

    let heuristic_suspicious = !suspicious_hits.is_empty();
    if heuristic_suspicious {
        evidence.push(format!(
            "heuristic_hits=[{}]",
            suspicious_hits.join(", ")
        ));
    }

    // ── Produce finding based on combined analysis ──
    let (severity, final_confidence, title) = match (hash_verdict, heuristic_suspicious) {
        ("baseline_mismatch", true) => (
            Severity::Critical,
            0.95,
            "AquaShell: index.py hash differs from known-good AND contains suspicious patterns",
        ),
        ("baseline_mismatch", false) => (
            Severity::High,
            0.80,
            "AquaShell: index.py hash differs from known-good baseline (no heuristic hits)",
        ),
        ("clean_baseline_match", true) => (
            Severity::Medium,
            0.60,
            "AquaShell: index.py matches known-good hash but contains suspicious strings (investigate version mismatch)",
        ),
        ("clean_baseline_match", false) => (
            Severity::Info,
            0.95,
            "AquaShell: index.py matches known-good baseline — likely clean",
        ),
        (_, true) => (
            Severity::High,
            0.65,
            "AquaShell: index.py contains suspicious patterns (no baseline for comparison)",
        ),
        _ => (
            Severity::Medium,
            0.40,
            "AquaShell: index.py exists but no baseline or heuristic match available (manual review needed)",
        ),
    };

    let mut finding = Finding::new("ASYNCOS-AQUASHELL-DETECT", severity, title)
        .with_evidence(evidence)
        .with_recommendation(
            "Compare to a known-good image for your exact AsyncOS release; \
             if suspicious deltas exist, treat as compromise and follow vendor remediation.",
        )
        .with_confidence(final_confidence)
        .with_role(role);
    finding.compute_surprise();
    report.push(finding);

    Ok(())
}

// ─── Layer 2: IOC hash scan (Rec #12) ──────────────────────────────────────

fn layer_2_ioc_hash(
    torque: &mut TorqueExecutor,
    fs: &dyn Fs,
    root: &Path,
    report: &mut Report,
    ioc_set: &IocSet,
) -> Result<()> {
    if ioc_set.hashes.is_empty() {
        report.warn(Warning::new(
            "Layer 2: IOC hash scan",
            "No IOC hashes configured (sidecar empty and built-in list empty)",
        ));
        return Ok(());
    }

    for d in CANDIDATE_DIRS {
        let dir = p(root, d);
        if !fs.exists(&dir) {
            continue;
        }

        // Walk the directory through the torque executor
        let walk_result = torque.walk_collect(&dir);
        let paths = match walk_result {
            Ok(p) => p,
            Err(e) => {
                report.warn(Warning::new(
                    format!("Layer 2: walk {}", dir.display()),
                    format!("Cannot walk directory: {e}"),
                ));
                continue;
            }
        };

        for path in paths {
            // ── Rec #12: size filter — skip files outside the plausible range ──
            let meta = match fs.metadata(&path) {
                Ok(m) => m,
                Err(_) => continue, // silently skip unreadable entries
            };
            if meta.size_bytes > IOC_MAX_FILE_BYTES || meta.size_bytes < IOC_MIN_FILE_BYTES {
                continue;
            }

            // Hash through torque envelope
            let file_hash = match torque.torqued_sha256(&path) {
                Ok(h) => h,
                Err(_) => continue,
            };

            for (name, want) in &ioc_set.hashes {
                if file_hash == *want {
                    let mut finding = Finding::new(
                        format!("ASYNCOS-IOC-HASH-{name}"),
                        Severity::Critical,
                        format!("Matched IOC hash: {name}"),
                    )
                    .with_evidence(vec![
                        format!("path={}", path.display()),
                        format!("sha256={file_hash}"),
                        format!("size_bytes={}", meta.size_bytes),
                    ])
                    .with_recommendation(
                        "Treat as confirmed compromise artifact; isolate and \
                         rebuild per vendor guidance.",
                    )
                    .with_confidence(0.99)
                    .with_role(FindingRole::Core);
                    finding.compute_surprise();
                    report.push(finding);
                }
            }
        }
    }

    Ok(())
}

// ─── Layer 3: IOC IP check ─────────────────────────────────────────────────

fn layer_3_ioc_ip(
    sys: &dyn Sys,
    torque: &mut TorqueExecutor,
    report: &mut Report,
    ioc_set: &IocSet,
) -> Result<()> {
    if ioc_set.ips.is_empty() {
        report.warn(Warning::new(
            "Layer 3: IOC IP check",
            "No IOC IPs configured",
        ));
        return Ok(());
    }

    let ss_result = torque.exec(|| sys.run("ss", &["-tupan"]));
    match ss_result {
        Ok(r) => {
            let hits: Vec<String> = r
                .stdout
                .lines()
                .filter(|l| ioc_set.ips.iter().any(|ip| l.contains(ip.as_str())))
                .map(|l| l.to_string())
                .collect();

            if !hits.is_empty() {
                let mut finding = Finding::new(
                    "ASYNCOS-IOC-IP-CONNECTIONS",
                    Severity::Critical,
                    "Active connections to IOC IPs detected",
                )
                .with_evidence(hits)
                .with_recommendation(
                    "Immediate containment: block egress, preserve logs off-box, \
                     proceed with IR / vendor TAC.",
                )
                .with_confidence(0.95)
                .with_role(FindingRole::Core);
                finding.compute_surprise();
                report.push(finding);
            }

            report.notes.push(r.summarize(64 * 1024));
        }
        Err(e) => {
            report.warn(Warning::new(
                "Layer 3: ss command",
                format!("Failed to execute `ss -tupan`: {e}"),
            ));
        }
    }

    Ok(())
}

// ─── Layer 4: Manifest drift (Rec #9) ──────────────────────────────────────

fn layer_4_manifest_drift(
    sys: &dyn Sys,
    fs: &dyn Fs,
    torque: &mut TorqueExecutor,
    root: &Path,
    report: &mut Report,
) -> Result<()> {
    // Determine AsyncOS version
    let version = torque.exec(|| {
        sys.run("cat", &["/etc/asyncos_version"])
            .ok()
            .and_then(|r| {
                if r.success() {
                    Some(r.stdout.trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "unknown".to_string())
    });

    let baseline = baseline_for_version(&version);

    // check_drift now takes &dyn Fs directly (Rec #10) — no torque executor needed
    // We remap root-relative paths for the drift check
    let remapped = ManifestBaseline {
        version: baseline.version.clone(),
        entries: baseline
            .entries
            .iter()
            .map(|e| ManifestEntry {
                path: p(root, &e.path.to_string_lossy()),
                expected_size: e.expected_size,
                sigma_bytes: e.sigma_bytes,
            })
            .collect(),
    };

    let drift_findings = remapped.check_drift(fs, 2.0);

    for df in drift_findings {
        if !df.flagged {
            continue;
        }

        let severity = if df.delta_pct > 10.0 {
            Severity::High
        } else {
            Severity::Medium
        };

        let confidence = match df.z_score {
            Some(z) if z > 4.0 => 0.90,
            Some(z) if z > 2.5 => 0.75,
            Some(_) => 0.60,
            None if df.delta_pct > 5.0 => 0.70,
            None => 0.55,
        };

        let mut evidence = vec![
            format!("expected_bytes={}", df.expected),
            format!("actual_bytes={}", df.actual),
            format!("delta_pct={:.2}%", df.delta_pct),
        ];
        if let Some(z) = df.z_score {
            evidence.push(format!("z_score={z:.2}"));
        }

        let mut finding = Finding::new(
            "ASYNCOS-MANIFEST-DRIFT",
            severity,
            format!(
                "Cumulative size drift: {:.1}% on {}",
                df.delta_pct,
                df.path.display()
            ),
        )
        .with_evidence(evidence)
        .with_recommendation(
            "Compare directory tree against a known-clean image for this AsyncOS version. \
             Unexpected mass indicates dropped files even if logs are clean.",
        )
        .with_confidence(confidence)
        .with_role(FindingRole::Bridge);
        finding.compute_surprise();
        report.push(finding);
    }

    Ok(())
}

// ─── Layer 5: Process baseline (Rec #14) ───────────────────────────────────

fn layer_5_process_baseline(
    sys: &dyn Sys,
    torque: &mut TorqueExecutor,
    report: &mut Report,
) -> Result<()> {
    let ps_result = torque.exec(|| sys.run("ps", &["auxww"]));
    match ps_result {
        Ok(r) => {
            report.notes.push(r.summarize(64 * 1024));

            // ── Rec #14: Compare against known-good process list ──
            // Extract unique process names (column 11+ in ps auxww output)
            let running_procs: Vec<String> = r
                .stdout
                .lines()
                .skip(1) // skip header
                .filter_map(|line| {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    // ps auxww: USER PID %CPU %MEM VSZ RSS TTY STAT START TIME COMMAND...
                    if fields.len() >= 11 {
                        // Extract binary name from full command path
                        let cmd = fields[10];
                        let binary = cmd
                            .rsplit('/')
                            .next()
                            .unwrap_or(cmd)
                            .trim_start_matches('-'); // login shells
                        Some(binary.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            // Find unexpected processes
            let unexpected: Vec<&String> = running_procs
                .iter()
                .filter(|proc_name| {
                    let name = proc_name.as_str();
                    // Skip kernel threads and known-good processes
                    !name.is_empty()
                        && !name.starts_with('[') // kernel threads
                        && !KNOWN_GOOD_PROCESSES.iter().any(|kg| name.contains(kg))
                })
                .collect();

            // Deduplicate for reporting
            let mut unique_unexpected: Vec<String> = unexpected
                .iter()
                .map(|s| s.to_string())
                .collect();
            unique_unexpected.sort();
            unique_unexpected.dedup();

            if !unique_unexpected.is_empty() {
                // Not every unexpected process is malicious — this is an informational
                // finding that the analyst should review against their environment
                let evidence: Vec<String> = unique_unexpected
                    .iter()
                    .take(20) // cap evidence length
                    .map(|p| format!("proc={p}"))
                    .collect();

                let confidence = if unique_unexpected.len() > 10 {
                    0.30 // many unknowns → low confidence any single one is malicious
                } else if unique_unexpected.len() > 3 {
                    0.45
                } else {
                    0.60
                };

                let mut finding = Finding::new(
                    "ASYNCOS-UNEXPECTED-PROCESSES",
                    Severity::Low,
                    format!(
                        "{} processes not in known-good baseline (review recommended)",
                        unique_unexpected.len()
                    ),
                )
                .with_evidence(evidence)
                .with_recommendation(
                    "Compare against your organization's process whitelist. \
                     Unfamiliar binaries in /tmp, /dev/shm, or running as root \
                     warrant immediate investigation.",
                )
                .with_confidence(confidence)
                .with_role(FindingRole::Periphery);
                finding.compute_surprise();
                report.push(finding);
            }
        }
        Err(e) => {
            report.warn(Warning::new(
                "Layer 5: ps command",
                format!("Failed to execute `ps auxww`: {e}"),
            ));
        }
    }

    Ok(())
}

// ─── Layer 5b: SSH Persistence Audit ─────────────────────────────────────────
//
// Maps to ACSC SD-WAN Hunt Guide rule IDs:
//   PERSISTENCE-T1098.004-RootSSHKeyAccepted-013
//
// And Talos UAT-8616 confirmed TTPs:
//   - Unauthorized SSH keys in /home/root/.ssh/authorized_keys
//   - Unauthorized SSH keys in /home/vmanage-admin/.ssh/authorized_keys
//   - PermitRootLogin yes in /etc/ssh/sshd_config
//
// Also covers UAT-9686 persistence (AquaTunnel ReverseSSH may deposit keys).

/// Paths where unauthorized SSH keys indicate persistence.
const SSH_KEY_PATHS: &[&str] = &[
    "/home/root/.ssh/authorized_keys",
    "/root/.ssh/authorized_keys",
    "/home/vmanage-admin/.ssh/authorized_keys",
];

/// SSH config path to audit for PermitRootLogin.
const SSHD_CONFIG_PATH: &str = "/etc/ssh/sshd_config";

fn layer_5b_ssh_persistence(
    _sys: &dyn Sys,
    fs: &dyn Fs,
    torque: &mut TorqueExecutor,
    root: &Path,
    report: &mut Report,
) -> Result<()> {
    // ── Check 1: authorized_keys presence and content ──
    for key_path in SSH_KEY_PATHS {
        let full = p(root, key_path);
        if !fs.exists(&full) {
            continue;
        }
        // File exists — read content through torque
        let content = torque.exec(|| {
            fs.read_to_string(&full)
                .unwrap_or_default()
        });

        if content.trim().is_empty() {
            continue;
        }

        // Count keys
        let key_count = content
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty()
                    && !t.starts_with('#')
                    && (t.starts_with("ssh-rsa")
                        || t.starts_with("ssh-ed25519")
                        || t.starts_with("ecdsa-sha2")
                        || t.starts_with("ssh-dss"))
            })
            .count();

        if key_count == 0 {
            continue;
        }

        let mut evidence = vec![
            format!("path={}", full.display()),
            format!("key_count={key_count}"),
        ];

        // Extract key comments (the third field) for triage
        for line in content.lines().take(10) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                evidence.push(format!("key_comment={}", parts[2]));
            } else if parts.len() == 2 {
                evidence.push("key_comment=<none>".to_string());
            }
        }

        // Root keys are always High; vmanage-admin keys are Medium
        let severity = if key_path.contains("root") {
            Severity::High
        } else {
            Severity::Medium
        };

        let mut finding = Finding::new(
            "PERSISTENCE-SSH-AUTHORIZED-KEYS",
            severity,
            format!(
                "{key_count} SSH key(s) in {} — matches PERSISTENCE-T1098.004-RootSSHKeyAccepted-013",
                key_path
            ),
        )
        .with_evidence(evidence)
        .with_recommendation(
            "Cross-reference each public key against your organization's key inventory. \
             Keys not in the inventory may indicate UAT-8616 or UAT-9686 persistence. \
             Ref: ACSC SD-WAN Hunt Guide, Talos UAT-8616 analysis.",
        )
        .with_confidence(0.85)
        .with_role(FindingRole::Core);
        finding.compute_surprise();
        report.push(finding);
    }

    // ── Check 2: PermitRootLogin in sshd_config ──
    let sshd_path = p(root, SSHD_CONFIG_PATH);
    if fs.exists(&sshd_path) {
        let sshd_content = torque.exec(|| {
            fs.read_to_string(&sshd_path)
                .unwrap_or_default()
        });

        // Parse active PermitRootLogin directive (ignore comments)
        let permit_root = sshd_content
            .lines()
            .filter(|l| !l.trim_start().starts_with('#'))
            .find(|l| {
                let lower = l.to_lowercase();
                lower.contains("permitrootlogin")
            });

        if let Some(line) = permit_root {
            let value = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .to_lowercase();

            if value == "yes" {
                let mut finding = Finding::new(
                    "PERSISTENCE-SSHD-PERMIT-ROOT-LOGIN",
                    Severity::High,
                    "PermitRootLogin yes — direct root SSH access enabled. \
                     Matches UAT-8616 TTP (Talos confirmed).",
                )
                .with_evidence(vec![
                    format!("path={}", sshd_path.display()),
                    format!("directive={}", line.trim()),
                ])
                .with_recommendation(
                    "Set PermitRootLogin to 'no' or 'prohibit-password'. \
                     UAT-8616 sets this after gaining root via CVE-2022-20775 \
                     downgrade escalation. Ref: Talos UAT-8616 blog, ACSC Hunt Guide.",
                )
                .with_confidence(0.90)
                .with_role(FindingRole::Core);
                finding.compute_surprise();
                report.push(finding);
            }
        }
    }

    Ok(())
}

// ─── Layer 6: Geometric drift summary (Rec #15) ────────────────────────────

/// Synthesizes a three-axis drift vector from findings already collected by
/// Layers 1–5 and produces a `SurpriseScore` that serves as a supplementary
/// triage signal.
///
/// Axis derivation:
///   x (mass) — max delta_pct from any ASYNCOS-MANIFEST-DRIFT finding
///   y (hash) — fraction of IOC hash hits (0.0 if none, 1.0 if multiple critical)
///   z (log)  — process baseline surprise proxy (unexpected process count / 20)
///
/// Per the technical assessment §6, axes are partially correlated so this
/// quadrant classification should be presented *alongside* per-axis breakdowns.
fn layer_6_geometric_summary(report: &mut Report) {
    // ── x: mass drift from manifest findings ──
    let mass_drift = report
        .findings
        .iter()
        .filter(|f| f.id == "ASYNCOS-MANIFEST-DRIFT")
        .filter_map(|f| {
            f.evidence.iter().find_map(|e| {
                if e.starts_with("delta_pct=") {
                    e.trim_start_matches("delta_pct=")
                        .trim_end_matches('%')
                        .parse::<f64>()
                        .ok()
                } else {
                    None
                }
            })
        })
        .fold(0.0f64, f64::max);

    // ── y: hash drift from IOC hash findings ──
    let ioc_hash_hits = report
        .findings
        .iter()
        .filter(|f| f.id.starts_with("ASYNCOS-IOC-HASH-"))
        .count();
    let hash_drift = match ioc_hash_hits {
        0 => 0.0,
        1 => 0.3,
        2 => 0.6,
        _ => 1.0,
    };

    // ── z: log/process drift from unexpected process findings ──
    let proc_drift = report
        .findings
        .iter()
        .filter(|f| f.id == "ASYNCOS-UNEXPECTED-PROCESSES")
        .filter_map(|f| {
            // Extract count from title: "N processes not in..."
            f.title.split_whitespace().next()?.parse::<f64>().ok()
        })
        .next()
        .unwrap_or(0.0)
        / 20.0 // normalize: 20 unexpected procs = 100% axis
        * 100.0;

    let current = Vec3::new(mass_drift, hash_drift, proc_drift);
    let previous = Vec3::zero(); // no prior snapshot — first observation
    let thresholds = GeomThresholds::default();
    let score = SurpriseScore::compute(&current, &previous, 60.0, &thresholds);

    // Only emit a finding if the drift is non-trivial
    if score.magnitude > 0.01 {
        let mut finding = Finding::new(
            "ASYNCOS-GEOMETRIC-DRIFT",
            if score.surprise > 0.7 {
                Severity::High
            } else if score.surprise > 0.3 {
                Severity::Medium
            } else {
                Severity::Low
            },
            format!(
                "Geometric drift summary: {} (surprise={:.0}%)",
                score.quadrant,
                score.surprise * 100.0,
            ),
        )
        .with_evidence(vec![
            format!("quadrant={}", score.quadrant),
            format!("magnitude={:.3}", score.magnitude),
            format!("surprise={:.2}", score.surprise),
            format!("mass_drift_pct={:.2}", current.x),
            format!("hash_drift={:.2}", current.y),
            format!("log_proc_drift_pct={:.2}", current.z),
            format!("torque={:.3}", score.torque),
            format!("angle_deg={:.2}", score.angle_rad.to_degrees()),
        ])
        .with_recommendation(
            "Use quadrant classification to prioritize investigation: \
             GROWING-DRIFT and STABLE-ANOMALY warrant immediate attention. \
             Cross-reference per-axis breakdown with individual layer findings.",
        )
        .with_confidence(0.65) // supplementary signal — moderate confidence
        .with_role(FindingRole::Bridge); // bridges layer findings into composite picture
        finding.compute_surprise();
        report.push(finding);
    }

    // Always record the raw score as a note for audit trail
    report.notes.push(format!("[geom] {score}"));
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::MockFs;
    use crate::sys::{CmdResult, MockSys};

    fn mock_sys_standard() -> MockSys {
        let mut sys = MockSys::new();
        sys.register(
            "uname -a",
            CmdResult {
                stdout: "Linux gateway 5.4.0 #1 SMP AsyncOS 15.5.1-065".into(),
                stderr: String::new(),
                code: Some(0),
            },
        );
        sys.register(
            "cat /etc/asyncos_version",
            CmdResult {
                stdout: "15.5.1-065\n".into(),
                stderr: String::new(),
                code: Some(0),
            },
        );
        sys.register(
            "ss -tupan",
            CmdResult {
                stdout: "Netid State Recv-Q Send-Q Local Address:Port Peer Address:Port\n\
                         tcp   ESTAB 0      0      10.1.1.5:443   10.2.2.2:54321\n"
                    .into(),
                stderr: String::new(),
                code: Some(0),
            },
        );
        sys.register(
            "ps auxww",
            CmdResult {
                stdout: "USER PID %CPU %MEM VSZ RSS TTY STAT START TIME COMMAND\n\
                         root 1 0.0 0.0 1234 567 ? Ss Jan01 0:05 /sbin/init\n\
                         root 100 0.0 0.0 2345 678 ? S Jan01 0:01 /usr/bin/sshd\n\
                         root 200 0.0 0.0 3456 789 ? S Jan01 0:00 /tmp/chisel\n"
                    .into(),
                stderr: String::new(),
                code: Some(0),
            },
        );
        sys
    }

    fn mock_fs_standard() -> MockFs {
        let mut fs = MockFs::new();
        // Probe paths for equilibrium sampling
        fs.insert_file("/proc/stat", b"cpu  1234 5678".to_vec());
        fs.insert_file("/proc/meminfo", b"MemTotal: 8192 kB".to_vec());
        fs.insert_file("/proc/loadavg", b"0.50 0.40 0.30 1/100 12345".to_vec());
        fs.insert_file("/proc/uptime", b"123456.78 234567.89".to_vec());
        fs.insert_file("/proc/vmstat", b"nr_free_pages 12345".to_vec());
        fs
    }

    #[test]
    fn ioc_set_parse_json() {
        let json = r#"{
            "hashes": [
                {"name": "TestMalware", "sha256": "aabbccdd"},
                {"name": "OtherThing", "sha256": "11223344"}
            ],
            "ips": ["1.2.3.4", "5.6.7.8"]
        }"#;
        let set = IocSet::parse_json(json).unwrap();
        assert_eq!(set.hashes.len(), 2);
        assert_eq!(set.hashes[0].0, "TestMalware");
        assert_eq!(set.hashes[0].1, "aabbccdd");
        assert_eq!(set.ips.len(), 2);
        assert_eq!(set.ips[0], "1.2.3.4");
    }

    #[test]
    fn ioc_set_fallback_to_builtin() {
        let fs = MockFs::new(); // no sidecar file
        let set = IocSet::load(&fs, Path::new("/etc/selfcheck/ioc_asyncos.json"));
        assert_eq!(set.hashes.len(), BUILTIN_IOC_HASHES.len());
        assert_eq!(set.ips.len(), BUILTIN_IOC_IPS.len());
    }

    #[test]
    fn ioc_set_loads_sidecar() {
        let mut fs = MockFs::new();
        fs.insert_file(
            "/etc/selfcheck/ioc_asyncos.json",
            br#"{"hashes":[{"name":"Custom","sha256":"deadbeef"}],"ips":["9.9.9.9"]}"#.to_vec(),
        );
        let set = IocSet::load(&fs, Path::new("/etc/selfcheck/ioc_asyncos.json"));
        assert_eq!(set.hashes.len(), 1);
        assert_eq!(set.hashes[0].0, "Custom");
        assert_eq!(set.ips.len(), 1);
    }

    #[test]
    fn extract_json_string_works() {
        let text = r#""name": "TestName", "sha256": "abc123""#;
        assert_eq!(
            extract_json_string(text, "name"),
            Some("TestName".to_string())
        );
        assert_eq!(
            extract_json_string(text, "sha256"),
            Some("abc123".to_string())
        );
        assert_eq!(extract_json_string(text, "missing"), None);
    }

    #[test]
    fn sha256_consistency() {
        // Verify our inline sha256 matches the one from torque
        let hash = sha256_hex(b"hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn layer_1_file_missing() {
        let fs = mock_fs_standard();
        let sys = mock_sys_standard();
        let mut report = Report::new();
        let root = Path::new("/device");

        // No index.py → should produce a note, not a finding
        run_all(&sys, &fs, root, &mut report).unwrap();
        assert!(
            report.findings.iter().all(|f| f.id != "ASYNCOS-AQUASHELL-DETECT"
                || f.severity == Severity::Info),
            "Missing index.py should not produce a high-severity finding"
        );
    }

    #[test]
    fn layer_2_ioc_match() {
        let mut fs = mock_fs_standard();
        let _sys = mock_sys_standard();

        // Place a file whose SHA-256 matches AquaTunnel
        // Make it >= 100 bytes to pass the size filter
        let aquatunnel_content = b"malicious content placeholder that is long enough to pass the size filter and be detected by the IOC scanner";
        let hash = sha256_hex(aquatunnel_content);

        // We need to create an IocSet with THIS hash for the test
        // Since run_all uses built-in hashes, let's test the layer function directly
        fs.insert_file("/device/tmp/evil", aquatunnel_content.to_vec());
        fs.insert_dir("/device/tmp");

        let mut report = Report::new();
        let mut torque = TorqueExecutor::new(&fs);
        let ioc_set = IocSet {
            hashes: vec![("TestMalware".to_string(), hash)],
            ips: vec![],
        };

        layer_2_ioc_hash(&mut torque, &fs, Path::new("/device"), &mut report, &ioc_set)
            .unwrap();

        assert_eq!(report.findings.len(), 1);
        assert_eq!(report.findings[0].severity, Severity::Critical);
        assert!(report.findings[0].id.contains("TestMalware"));
    }

    #[test]
    fn layer_2_size_filter() {
        let mut fs = mock_fs_standard();

        // File too small (< 100 bytes) — should be skipped
        fs.insert_file("/device/tmp/tiny", vec![0u8; 50]);
        fs.insert_dir("/device/tmp");

        let mut report = Report::new();
        let mut torque = TorqueExecutor::new(&fs);
        let ioc_set = IocSet {
            hashes: vec![("Test".to_string(), sha256_hex(&vec![0u8; 50]))],
            ips: vec![],
        };

        layer_2_ioc_hash(&mut torque, &fs, Path::new("/device"), &mut report, &ioc_set)
            .unwrap();

        // Should NOT match because the file is below the size threshold
        assert!(report.findings.is_empty());
    }

    #[test]
    fn layer_3_ioc_ip_hit() {
        let mut sys = MockSys::new();
        sys.register(
            "ss -tupan",
            CmdResult {
                stdout: "tcp ESTAB 0 0 10.1.1.5:443 172.233.67.176:8080\n".into(),
                stderr: String::new(),
                code: Some(0),
            },
        );

        let fs = mock_fs_standard();
        let mut report = Report::new();
        let mut torque = TorqueExecutor::new(&fs);
        let ioc_set = IocSet {
            hashes: vec![],
            ips: vec!["172.233.67.176".to_string()],
        };

        layer_3_ioc_ip(&sys, &mut torque, &mut report, &ioc_set).unwrap();
        assert_eq!(report.findings.len(), 1);
        assert_eq!(report.findings[0].severity, Severity::Critical);
    }

    #[test]
    fn layer_3_no_hit() {
        let mut sys = MockSys::new();
        sys.register(
            "ss -tupan",
            CmdResult {
                stdout: "tcp ESTAB 0 0 10.1.1.5:443 10.2.2.2:54321\n".into(),
                stderr: String::new(),
                code: Some(0),
            },
        );

        let fs = mock_fs_standard();
        let mut report = Report::new();
        let mut torque = TorqueExecutor::new(&fs);
        let ioc_set = IocSet {
            hashes: vec![],
            ips: vec!["172.233.67.176".to_string()],
        };

        layer_3_ioc_ip(&sys, &mut torque, &mut report, &ioc_set).unwrap();
        assert!(report.findings.is_empty());
    }

    #[test]
    fn layer_5_unexpected_process_detection() {
        let mut sys = MockSys::new();
        sys.register(
            "ps auxww",
            CmdResult {
                stdout: "USER PID %CPU %MEM VSZ RSS TTY STAT START TIME COMMAND\n\
                         root 1 0.0 0.0 1234 567 ? Ss Jan01 0:05 /sbin/init\n\
                         root 200 0.0 0.0 3456 789 ? S Jan01 0:00 /tmp/chisel\n\
                         root 201 0.0 0.0 3456 789 ? S Jan01 0:00 /dev/shm/.hidden_backdoor\n"
                    .into(),
                stderr: String::new(),
                code: Some(0),
            },
        );

        let fs = mock_fs_standard();
        let mut report = Report::new();
        let mut torque = TorqueExecutor::new(&fs);

        layer_5_process_baseline(&sys, &mut torque, &mut report).unwrap();

        // Should flag chisel and .hidden_backdoor as unexpected
        let proc_findings: Vec<_> = report
            .findings
            .iter()
            .filter(|f| f.id == "ASYNCOS-UNEXPECTED-PROCESSES")
            .collect();
        assert_eq!(proc_findings.len(), 1);
        let evidence_joined = proc_findings[0].evidence.join(" ");
        assert!(evidence_joined.contains("chisel"));
        assert!(evidence_joined.contains(".hidden_backdoor"));
    }

    #[test]
    fn baseline_for_version_returns_entries() {
        let b = baseline_for_version("15.5.1-065");
        assert_eq!(b.entries.len(), 3);
        assert!(b.entries[0].sigma_bytes.is_some());
        assert!(b.entries[2].sigma_bytes.is_none());
    }

    #[test]
    fn manifest_drift_with_root_remap() {
        let mut fs = mock_fs_standard();
        // Simulate /device/data/web/euq_webui/htdocs with unexpected extra size
        fs.insert_dir("/device/data/web/euq_webui/htdocs");
        fs.insert_file(
            "/device/data/web/euq_webui/htdocs/index.py",
            vec![0u8; 5_000_000], // 5 MB — exceeds 4.2 MB baseline
        );

        let baseline = baseline_for_version("15.5");
        let remapped = ManifestBaseline {
            version: baseline.version.clone(),
            entries: baseline
                .entries
                .iter()
                .map(|e| ManifestEntry {
                    path: p(Path::new("/device"), &e.path.to_string_lossy()),
                    expected_size: e.expected_size,
                    sigma_bytes: e.sigma_bytes,
                })
                .collect(),
        };

        let results = remapped.check_drift(&fs, 2.0);
        assert!(!results.is_empty());
        // The htdocs entry should be flagged (5MB vs 4.2MB baseline with 50KB sigma → z > 2.0)
        let htdocs_result = results
            .iter()
            .find(|r| r.path.to_string_lossy().contains("htdocs"))
            .unwrap();
        assert!(htdocs_result.flagged);
    }

    #[test]
    fn layer_6_geometric_summary_emits_finding() {
        // Pre-populate a report with findings from multiple layers
        let mut report = Report::new();

        // Simulate a manifest drift finding (x-axis)
        let drift_finding = Finding::new(
            "ASYNCOS-MANIFEST-DRIFT",
            Severity::Medium,
            "Cumulative size drift: 8.5% on /data/web",
        )
        .with_evidence(vec![
            "expected_bytes=52000000".to_string(),
            "actual_bytes=56420000".to_string(),
            "delta_pct=8.50%".to_string(),
        ]);
        report.push(drift_finding);

        // Simulate an IOC hash hit (y-axis)
        let ioc_finding = Finding::new(
            "ASYNCOS-IOC-HASH-AquaTunnel",
            Severity::Critical,
            "Matched IOC hash: AquaTunnel",
        );
        report.push(ioc_finding);

        // Simulate unexpected processes (z-axis)
        let proc_finding = Finding::new(
            "ASYNCOS-UNEXPECTED-PROCESSES",
            Severity::Low,
            "3 processes not in known-good baseline",
        );
        report.push(proc_finding);

        // Run geometric summary
        layer_6_geometric_summary(&mut report);

        // Should produce a ASYNCOS-GEOMETRIC-DRIFT finding
        let geom_findings: Vec<_> = report
            .findings
            .iter()
            .filter(|f| f.id == "ASYNCOS-GEOMETRIC-DRIFT")
            .collect();
        assert_eq!(geom_findings.len(), 1);
        let gf = &geom_findings[0];
        assert!(gf.evidence.iter().any(|e| e.starts_with("quadrant=")));
        assert!(gf.evidence.iter().any(|e| e.starts_with("magnitude=")));
        assert!(gf.evidence.iter().any(|e| e.starts_with("surprise=")));

        // With IOC hit + drift + unexpected procs, surprise should be non-trivial
        let surprise_val = gf.evidence.iter()
            .find_map(|e| {
                if e.starts_with("surprise=") {
                    e.trim_start_matches("surprise=").parse::<f64>().ok()
                } else {
                    None
                }
            })
            .unwrap();
        assert!(surprise_val > 0.1, "should have meaningful surprise: {surprise_val}");
    }

    // ── Layer 5b: SSH Persistence Tests ──────────────────────────────────

    #[test]
    fn layer_5b_authorized_keys_detected() {
        let mut fs = mock_fs_standard();
        fs.insert_file(
            "/device/home/root/.ssh/authorized_keys",
            b"ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ... attacker@c2-node\n\
              ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... backup-implant\n"
                .to_vec(),
        );

        let sys = MockSys::new();
        let mut report = Report::new();
        let mut torque = TorqueExecutor::new(&fs);

        layer_5b_ssh_persistence(&sys, &fs, &mut torque, Path::new("/device"), &mut report)
            .unwrap();

        let ssh = report.findings.iter()
            .filter(|f| f.id == "PERSISTENCE-SSH-AUTHORIZED-KEYS")
            .collect::<Vec<_>>();
        assert_eq!(ssh.len(), 1, "One authorized_keys finding");
        assert_eq!(ssh[0].severity, Severity::High, "Root keys = High");
        assert!(ssh[0].confidence >= 0.85);
        assert!(ssh[0].evidence.iter().any(|e| e.contains("key_count=2")));
        assert!(ssh[0].evidence.iter().any(|e| e.contains("attacker@c2-node")));
        assert!(ssh[0].title.contains("T1098.004"));
    }

    #[test]
    fn layer_5b_permit_root_login_detected() {
        let mut fs = mock_fs_standard();
        fs.insert_file(
            "/device/etc/ssh/sshd_config",
            b"# Default config\nPort 22\nPermitRootLogin yes\nPubkeyAuthentication yes\n"
                .to_vec(),
        );

        let sys = MockSys::new();
        let mut report = Report::new();
        let mut torque = TorqueExecutor::new(&fs);

        layer_5b_ssh_persistence(&sys, &fs, &mut torque, Path::new("/device"), &mut report)
            .unwrap();

        let permit = report.findings.iter()
            .filter(|f| f.id == "PERSISTENCE-SSHD-PERMIT-ROOT-LOGIN")
            .collect::<Vec<_>>();
        assert_eq!(permit.len(), 1, "PermitRootLogin finding");
        assert_eq!(permit[0].severity, Severity::High);
        assert!(permit[0].confidence >= 0.90);
        assert!(permit[0].evidence.iter().any(|e| e.contains("PermitRootLogin yes")));
    }

    #[test]
    fn layer_5b_clean_no_ssh_findings() {
        let fs = mock_fs_standard();
        let sys = MockSys::new();
        let mut report = Report::new();
        let mut torque = TorqueExecutor::new(&fs);

        layer_5b_ssh_persistence(&sys, &fs, &mut torque, Path::new("/device"), &mut report)
            .unwrap();

        let ssh_findings: Vec<_> = report.findings.iter()
            .filter(|f| f.id.contains("PERSISTENCE-SSH") || f.id.contains("SSHD"))
            .collect();
        assert!(ssh_findings.is_empty(), "Clean system: no SSH findings");
    }

    #[test]
    fn layer_5b_commented_permitroot_ignored() {
        let mut fs = mock_fs_standard();
        fs.insert_file(
            "/device/etc/ssh/sshd_config",
            b"# PermitRootLogin yes\nPermitRootLogin no\n".to_vec(),
        );

        let sys = MockSys::new();
        let mut report = Report::new();
        let mut torque = TorqueExecutor::new(&fs);

        layer_5b_ssh_persistence(&sys, &fs, &mut torque, Path::new("/device"), &mut report)
            .unwrap();

        let permit = report.findings.iter()
            .filter(|f| f.id == "PERSISTENCE-SSHD-PERMIT-ROOT-LOGIN")
            .collect::<Vec<_>>();
        assert!(permit.is_empty(), "Commented + 'no' = no finding");
    }
}

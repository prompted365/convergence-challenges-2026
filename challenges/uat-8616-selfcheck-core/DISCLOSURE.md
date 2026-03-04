# Disclosure Notice — selfcheck-core

## Real-World Event Disclosure

This challenge response was catalyzed by active exploitation of Cisco infrastructure by nation-state threat actors. It is tied to real CVEs, real emergency directives, and real organizational risk.

| Field | Value |
|-------|-------|
| **CVEs** | CVE-2026-20127 (CVSS 10.0), CVE-2025-20393 (CVSS 10.0) |
| **Threat actors** | UAT-8616 (SD-WAN), UAT-9686 (AsyncOS) |
| **Emergency directives** | CISA ED 26-03, ASD/ACSC PP-26-0656 |
| **Coordinating agencies** | CISA, ASD/ACSC, CCCS, CSA Singapore |
| **Active since** | At least 2023 (SD-WAN), November 2025 (AsyncOS) |
| **Public disclosure** | February 25, 2026 |

---

## Verification Status

### Base Kit (Layers 0–5b) — Independent Review Requested

| Aspect | Status |
|--------|--------|
| **Independent security review** | **REQUESTED — not yet completed** |
| **Regulatory endorsement** | None. No government agency has endorsed this toolkit. |
| **Production readiness** | **NOT RECOMMENDED** until independent review is complete |
| **Test coverage** | 51/51 Python mirror tests passing; Rust integration tests passing |
| **IOC accuracy** | IOCs sourced from published Cisco Talos reports and ASD/ACSC Hunt Guide |
| **ACSC Hunt Guide mapping** | 3 rule IDs mapped (PRIVESC-T1601.002, INITIALACCESS-T1190, PERSISTENCE-T1098.004) |

The base kit covers detection layers 0–5b: implant detection, IOC hash scanning, IOC IP correlation, manifest drift analysis, process baseline comparison, and SSH persistence audit. These layers map directly to known indicators of compromise published by Cisco Talos and the ASD/ACSC Hunt Guide. They are the portion of this toolkit with immediate practical applicability pending qualified review.

**Modules in review scope:** `asyncos.rs`, `torque.rs`, `report.rs`, `io.rs`, `sys.rs`, `lib.rs`

### Expansion Layers (6–12) — Experimental, Not In Review Scope

| Aspect | Status |
|--------|--------|
| **Independent review** | Not requested at this time |
| **Theoretical basis** | Published but unvalidated detection theories |
| **Production readiness** | **NOT APPLICABLE** — these are research explorations |
| **Adversarial validation** | Layer 11 (redshift) self-reports 0.44 brittleness (8/18 evasion vectors break through) |

The expansion layers explore detection approaches beyond known IOC matching: geometric drift scoring, acoustic echo detection, probabilistic tripwires, cross-domain narrative coherence, criticality inversion, adversarial robustness simulation, and physics-anchored invariant proofs. These are theoretical extensions initiated from the base kit's detection surface. They are organized separately because they represent brainstorming and theory — not yet proven or independently applicable. They are **not part of the current review request**.

**Modules outside review scope:** `geom.rs`, `sonar.rs`, `tripwire.rs`, `eris.rs`, `spinaltap.rs`, `redshift.rs`, `anchor.rs`

---

## What This Means For Users

1. **If you are an incident responder or forensic engineer:** The base kit (layers 0–5b) is published for your evaluation. Read the Python test harness first — it mirrors every Rust detection layer and requires no compilation. The decision to deploy belongs to the teams who own the risk.

2. **If you are a security researcher:** The full codebase (base + expansions) is available for audit. The expansion layers are where the interesting theoretical questions live. Criticism, correction, and gap identification are welcome.

3. **If you are affected by CVE-2026-20127 or CVE-2025-20393:** Follow official guidance from CISA, ASD/ACSC, and Cisco regardless of whether you ever use this toolkit. The official guidance does not depend on this tool existing.

---

## Review Process

The author will not promote direct operational use of the base kit until:
1. Independent security review has been completed by qualified researchers
2. Logic errors, false-positive risks, and detection gaps have been documented
3. Ideally, the toolkit has been evaluated by investigators working the relevant campaigns

The expansion layers follow a separate maturation path. They may be promoted to review scope individually as they mature, but they are not gating the base kit's review process.

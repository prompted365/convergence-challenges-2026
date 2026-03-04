---
schema: incite@1
type: incident
challenge_id: uat-8616-selfcheck-core
source:
  cves: ["CVE-2026-20127", "CVE-2025-20393"]
  threat_actor: ["UAT-8616", "UAT-9686"]
  directives: ["CISA ED 26-03", "ASD/ACSC PP-26-0656"]
  active_since: "2023"
failure_modes:
  - id: F1
    name: Authentication bypass (SD-WAN)
    severity: critical
  - id: F2
    name: Root command execution (AsyncOS)
    severity: critical
  - id: F3
    name: Firmware downgrade-and-restore
    severity: critical
  - id: F4
    name: Forensic evidence erasure
    severity: high
  - id: F5
    name: Implant persistence across reboots
    severity: critical
  - id: F6
    name: Forensic tool detection by adversary
    severity: high
  - id: F7
    name: Paranoid Benefactor voluntary submission
    severity: critical
expected_archetypes:
  primary: [cunning_triumph, empire_cycle]
  secondary: [cautionary_fall, greed_backfire]
stream:
  state: ongoing
  assignee: Homeskillet
feeds_into: []
fed_by:
  - tvn-payne-nuclear-escalation
  - agents-of-chaos
---

> **Real-world event disclosure.** This challenge is tied to active exploitation of Cisco infrastructure by nation-state threat actors. See [DISCLOSURE.md](DISCLOSURE.md) for verification status, regulatory review posture, and the separation between the base forensic detection kit and experimental expansion layers.

# UAT-8616 / UAT-9686 — Cisco Infrastructure Compromise

In February 2026, CISA, ASD/ACSC, CCCS, and CSA Singapore jointly disclosed two CVSS 10.0 campaigns targeting Cisco Catalyst SD-WAN Controllers and Cisco Secure Email Gateway appliances. Exploitation confirmed active since at least 2023.

This is not a paper-based challenge. It was catalyzed by a real-world incident — the discovery that nation-state threat actors had been living inside trusted network infrastructure for years, specifically designing their operations to survive forensic investigation.

The catalyst claim: these campaigns exploit not just software vulnerabilities but the forensic investigation process itself. The attacker's cleanup operations create a four-plane feedback loop where each investigation generates the specification document for the next campaign's evasion strategy.

The response: selfcheck-core — a 13-layer forensic detection toolkit in Rust (8,453 lines, one external dependency) designed to detect compromise even when the attacker is actively monitoring for forensic activity.

**Stream state: `ongoing`.** This challenge does not have a neat closure. The threat actors are active, the CVEs are real, and the forensic feedback loop means the detection surface evolves with each campaign iteration.

## Package Structure

| Component | Scope | Review Status |
|-----------|-------|---------------|
| **Base Kit** (layers 0–5b) | Known IOC detection, implant scanning, drift analysis, SSH audit | Independent review **requested** |
| **Expansion Layers** (6–12) | Theoretical detection (geometric drift, echo, tripwires, narrative coherence, adversarial robustness, invariant proofs) | Not in review scope |
| **Intelligence** | Threat maps, convergent analysis | Reference material |
| **Test Harness** | Python mirror of all Rust layers, 51 tests | Validation artifact |

See [DISCLOSURE.md](DISCLOSURE.md) for the full verification status of each component.

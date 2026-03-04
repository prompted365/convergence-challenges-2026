---
challenge_id: uat-8616-selfcheck-core
---

# Context — UAT-8616 selfcheck-core

## Upstream Edges

### ← TVN-Payne Nuclear Escalation (HYPOTHESIS)
**Edge type:** Infrastructure pattern sharing
**Claim:** Infrastructure compromise campaigns share `cunning_triumph` and `empire_cycle` archetype patterns with nuclear escalation — stealth normalization maps to strategic bluffing, network fabric control maps to sovereignty claims.
**Status:** UNTESTED. The TVN lens framework informed the STRATEGIC_FRAMEWORK.md bridge document, but no constraint harness validates archetype activation on forensic scenarios.

### ← Agents of Chaos (HYPOTHESIS)
**Edge type:** Social engineering → infrastructure compromise
**Claim:** The Paranoid Benefactor pattern (targets voluntarily submit devices + authorize AI agents via legitimate OAuth/wallet signatures) is social engineering at infrastructure scale. Identity spoofing, authority compliance, and governance injection from AoC map to adversary TTPs.
**Status:** UNTESTED.

## Downstream Edges

None yet. This challenge is in `ongoing` state — downstream edges may emerge as the detection surface evolves with active campaigns.

## Archetype Bridges (Hypothesized)

| Archetype | Hypothesized Role in Infrastructure Forensics |
|-----------|-----------------------------------------------|
| `cunning_triumph` | Firmware rollback-and-restore (disguise attack as clean state), stealth I/O normalization, forensic activity monitoring |
| `empire_cycle` | SD-WAN NETCONF access = sovereignty over network fabric, email gateway = sovereignty over communications |
| `cautionary_fall` | Organizations that assume clean-looking devices are clean — hubris in trusted infrastructure |
| `greed_backfire` | Data exfiltration via covert tunnels (AquaTunnel, Chisel) — the theft creates the detection surface |

## Constraints Tested

This challenge does not use the standard 3-bridge constraint harness. Instead, it implements a 13-layer forensic detection stack:

| Layer Group | Layers | Constraint Analogy | Status |
|------------|--------|-------------------|--------|
| **Base Kit** | 0–5b | Physics layer (known IOC matching, process baselines) | Independent review requested |
| **Geometric** | 6 | Perception layer (drift scoring) | Experimental |
| **Temporal** | 7–8 | Perception layer (echo, timeline verification) | Experimental |
| **Narrative** | 9 | Governance layer (cross-domain coherence) | Experimental |
| **Adversarial** | 10–12 | Meta-layer (self-testing detection surface) | Experimental |

## Known Gaps

- **No constraint harness:** Unlike TVN-Payne and AoC, this challenge does not have a pytest harness that feeds scenarios through the operationTorque 3-bridge pipeline. The test harness validates the forensic detection layers, not archetype activation.
- **Adversarial robustness:** Layer 11 (redshift) self-reports 0.44 brittleness — 8 of 18 simulated evasion vectors break through detection. Reported honestly.
- **Forensic feedback loop:** The four-plane attack model (Devices → Forensic Bench → Attacker Assessment → Next Campaign Evasion) means the detection surface is inherently non-static. The base kit's IOC matching is only as current as the last published advisory.
- **Paranoid Benefactor pattern:** Identified but no detection layer addresses the voluntary submission + legitimate authorization attack vector.

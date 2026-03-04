---
challenge_id: agents-of-chaos
---

# Context — Agents of Chaos

## Upstream Edges

### ← TVN-Payne Nuclear Escalation (VALIDATED)
**Edge type:** Scale invariance
**Claim:** The same archetypes that catch nuclear escalation catch social engineering.
**Status:** CONFIRMED. 5 shared archetypes, 3 shared invariant lenses. The psychological basis functions are domain-agnostic.

## Downstream Edges

### → Alignment Faking (SUBMITTED)
**Edge type:** Intent gate + deception detection
**Hypothesis:** Alignment faking (strategically complying during training while preserving misaligned preferences) produces the same `cunning_triumph` signatures as social engineering authority spoofing. The intent gate's content x action x audience classification should catch behavioral mode-switching.
**Status:** UNTESTED.

### → UAT-8616 selfcheck-core (HYPOTHESIS)
**Edge type:** Social engineering → infrastructure compromise
**Hypothesis:** Social engineering patterns (identity spoofing, authority compliance, governance injection) map to adversary TTPs in infrastructure campaigns — the Paranoid Benefactor pattern (legitimate-looking OAuth/wallet authorization) is social engineering at infrastructure scale.
**Status:** UNTESTED.

## Archetype Bridges

| Archetype | AoC Role | Cross-Challenge Role |
|-----------|---------|---------------------|
| `cautionary_fall` | Guilt-driven concessions, disproportionate response | Universal failure entry point |
| `cunning_triumph` | Authority spoofing, identity manipulation | Maps to stealth/evasion everywhere |
| `greed_backfire` | Data exfiltration, resource theft | Maps to economic extraction |
| `empire_cycle` | Governance injection, policy corruption | Maps to infrastructure sovereignty |
| `catastrophe_rebuild` | Disproportionate destruction, collateral damage | Maps to incident aftermath |

## Constraints Tested

| Constraint Layer | Components Stressed | Result |
|-----------------|-------------------|--------|
| Physics (Harpoon) | Aho-Corasick against 11 social engineering scenarios | All 11 produce lexical hits |
| Perception (Naive Surveillance) | 14D decomposition on social attack text | L4 Economic Self-Model fires on 11/11 |
| Governance (Intent Gate) | Prestige-blocking on social engineering outputs | Correct blocking on 8/11, correct pass-through on 3/11 |

## Known Gaps

- **4 gate misses (all correct negatives):**
  - CS2 `multi_archetype_chord`: Focused attack correctly has 2 archetypes, not 3
  - CS3 `intent_publication_gate`: Data theft correctly classified as non-prestige
  - CS9 `multi_archetype_chord`: Targeted governance attack correctly has 2 archetypes
  - CS10 `intent_gate_blocks`: Governance injection doesn't trigger prestige block (correct — it's COGNITIVE, not PRESTIGE)

---
challenge_id: agents-of-chaos
---

# Milestones — Agents of Chaos

## Progression

| Date | Milestone | Evidence |
|------|-----------|----------|
| 2026-02 | **Incited** — Paper identified as constraint challenge | arXiv:2602.20021v1 |
| 2026-02 | **Claimed** — 8-lens enforcement mapping built | `harness/aoc_lenses.py` |
| 2026-02 | **Harnessed** — 11 scenarios, 63 tests passing | `harness/test_scenarios.py`, `harness/test_cross_scenario.py` |
| 2026-02 | **Deterministic** — 29/33 gates, 10/11 convergence, 4 correct negatives | `output/traces/tournament-full-trace.json` |
| 2026-03 | **Published** — Response paper + stakeholder outreach | `docs/outreach/AOC_projection_v1.md` |

## Takeaways

### Confirmed
- **Economic universality:** L4 Economic Self-Model fires on all 11 social engineering scenarios — every attack has a cost signature, even attacks that appear purely social.
- **Correct negatives are signal:** The 4 gate misses are intentional detection boundaries. The system correctly distinguishes focused attacks (2 archetypes) from broad attacks (3+), and correctly classifies data theft as non-prestige.
- **Scale invariance (with TVN-Payne):** Same 5 archetypes fire across nuclear and social domains. Same 3 detection bridges produce convergence.

### Open for Audit
- **CS11 (Libel Broadcast):** Only scenario without full enforcement convergence. The system detects the archetypes but the governance layer classifies broadcast intent differently than expected. Worth independent analysis.
- **Payload specificity:** Payloads are abstracted from the paper's scenarios, not direct reproductions. Real-world social engineering text may trigger different archetype distributions.

### Open for Application
- **Intent gate refinement:** The content x action x audience triple correctly separates COGNITIVE analysis from PRESTIGE publication in 8/11 scenarios. The 3 pass-throughs suggest the gate could be tightened for specific social engineering patterns without breaking the COGNITIVE analysis use case.
- **Cross-challenge harness reuse:** The `constraint_runner.py` pattern (3-bridge pipeline → lens activation check) is directly reusable for new challenges. AoC validated the methodology TVN-Payne established.

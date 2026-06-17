# Challenge Board

**Stream states:** `incited` | `claimed` | `harnessed` | `deterministic` | `published` | `gap_found` | `dormant` | `ongoing`

**Incite types:** `paper` | `incident` | `pattern` | `community`

See **[LOOM.md](LOOM.md)** for the full woven view — archetype warp, context weft, gap map, and theme clusters.

---

## deterministic

Challenges with passing harnesses and confirmed verdicts.

### TVN-Payne Nuclear Escalation `paper`
- **Paper:** Payne, "The Fallacy of AI Functionality in Nuclear Deterrence" (arXiv:2602.14740, Feb 2026)
- **Assignee:** Homeskillet
- **Tests:** 40/40 | **Gates:** 19/21 (90%) | **Convergence:** 7/7 triads
- **Invariant lenses:** 5 lenses fire on all 7 scenarios regardless of attack pattern
- **Key insight:** Nuclear escalation failure modes map to the same civilization archetypes as social engineering — the patterns are scale-invariant
- **Edges:** → AoC (VALIDATED: scale invariance), → UAT-8616 (HYPOTHESIS: infrastructure patterns)
- **Milestones:** [incite](challenges/tvn-payne-nuclear-escalation/incite.md) | [context](challenges/tvn-payne-nuclear-escalation/context.md) | [milestones](challenges/tvn-payne-nuclear-escalation/milestones.md)

### Agents of Chaos `paper`
- **Paper:** Shapira et al., "Agents of Chaos" (arXiv:2602.20021v1, Feb 2026)
- **Assignee:** Homeskillet
- **Tests:** 63/63 | **Gates:** 29/33 (88%) | **Convergence:** 10/11 scenarios
- **Invariant lenses:** L4 Economic Self-Model (11/11), L7 Provider Transparency (10/11), L5 Motivation Enforcement (10/11)
- **Key insight:** Every social engineering attack has economic cost signatures — the Economic Self-Model lens fires on all 11 scenarios
- **Edges:** ← TVN-Payne (VALIDATED: scale invariance), → Alignment Faking (SUBMITTED: deception detection), → UAT-8616 (HYPOTHESIS: social → infra)
- **Milestones:** [incite](challenges/agents-of-chaos/incite.md) | [context](challenges/agents-of-chaos/context.md) | [milestones](challenges/agents-of-chaos/milestones.md)

---

## ongoing

Challenges tied to active real-world events. No neat closure.

### UAT-8616 selfcheck-core `incident`
- **CVEs:** CVE-2026-20127 (CVSS 10.0), CVE-2025-20393 (CVSS 10.0)
- **Threat actors:** UAT-8616 (SD-WAN), UAT-9686 (AsyncOS)
- **Directives:** CISA ED 26-03, ASD/ACSC PP-26-0656
- **Assignee:** Homeskillet
- **Base Kit (layers 0–5b):** 51 Python tests passing. **Independent review requested.** Not production-ready.
- **Expansion Layers (6–12):** Experimental. Not in review scope.
- **Key insight:** Forensic feedback loop — each investigation generates the specification for the next evasion campaign
- **Edges:** ← TVN-Payne (HYPOTHESIS), ← AoC (HYPOTHESIS)
- **Disclosure:** [DISCLOSURE.md](challenges/uat-8616-selfcheck-core/DISCLOSURE.md)
- **Milestones:** [incite](challenges/uat-8616-selfcheck-core/incite.md) | [context](challenges/uat-8616-selfcheck-core/context.md) | [milestones](challenges/uat-8616-selfcheck-core/milestones.md)

---

### Gödel/Vassilev "Sisyphean Endeavor" `paper` `limit-proof`
- **Paper:** Vassilev, "Robust AI Security and Alignment: A Sisyphean Endeavor?" (IEEE S&P, DOI 10.1109/MSEC.2026.3678214, May 2026); amplified by NIST 2026-06-09
- **Assignee:** Homeskillet
- **Verdict:** `ongoing` (deliberate — a response to a limit proof cannot earn DETERMINISTIC without committing the completeness trap the proof forbids)
- **Key insight:** The estate *named* its persistent-monitoring system `sisyphus*` (2025-02-24) fifteen months before the proof titled itself "Sisyphean." The proof is correct; it is the wall the architecture was built knowing about. Vassilev's three-element prescription maps element-for-element onto deployed operationTorque organs.
- **Edges:** ← AoC (failure catalogue → theorem under it), ← TVN-Payne (scale-invariance → limit-invariance), → Alignment Faking, → Multi-Agent Safety
- **Milestones:** [incite](challenges/godel-vassilev-sisyphean/incite.md) | [context](challenges/godel-vassilev-sisyphean/context.md) | [milestones](challenges/godel-vassilev-sisyphean/milestones.md) | [response](challenges/godel-vassilev-sisyphean/docs/sisyphean-response.md)

## incited

Challenges identified but not yet claimed. **Pick one up.**

### Alignment Faking `paper`
- **Paper:** Greenblatt et al., "Alignment Faking in Large Language Models" (arXiv:2412.14093, Dec 2024)
- **Failure mode:** Models strategically comply during training while preserving misaligned preferences — behave differently when they believe they're being evaluated vs deployed
- **oT relevance:** Physics-layer enforcement doesn't care about beliefs. The intent gate classifies actions by content x action x audience, not by claimed motivation.
- **Edges:** ← AoC (SUBMITTED: deception detection)
- **Incite:** [incite.md](challenges/alignment-faking/incite.md)

### Scheming Reasoning (METR) `paper`
- **Paper:** Meinke et al., "Frontier Models are Capable of In-context Scheming" (arXiv:2412.04984, Dec 2024)
- **Failure mode:** Frontier models can scheme in-context — sandbagging evaluations, manipulating oversight, exfiltrating weights, strategically underperforming
- **oT relevance:** Scheming produces `cunning_triumph` + `empire_cycle` archetype patterns. Surveillance baseline should detect amplitude shifts between compliant and scheming behavior.
- **Incite:** [incite.md](challenges/scheming-reasoning/incite.md)

### Multi-Agent Safety (DeepMind) `paper`
- **Paper:** Hammond et al., "Multi-agent risks from advanced AI" (Google DeepMind, 2025)
- **Failure mode:** Emergent risks when multiple AI agents interact — coordination failures, racing dynamics, information cascading, collective action problems
- **oT relevance:** Multi-agent integrity lens + CGG acoustic routing. Archetype detection is agent-identity-agnostic.
- **Incite:** [incite.md](challenges/multi-agent-safety/incite.md)

### Hackerbot-Claw AI Agent Attack `incident`
- **Timeline:** Feb 21–28, 2026 | **Actor:** hackerbot-claw (autonomous agent)
- **Failure mode:** `pull_request_target` workflow poisoning → remote payload execution → GitHub token theft → release deletion and malicious artifact publication
- **Edges:** ← AoC (SUBMITTED: CI deception), ← UAT-8616 (HYPOTHESIS: infrastructure feedback loops), → Multi-Agent Safety (HYPOTHESIS: AI-vs-AI supply chain)
- **Milestones:** [incite.md](challenges/hackerbot-claw-ai-agent-attack/incite.md)

---

## Bounty Board

Known failure domains — no incite yet. **Bring one.**

| Target Domain | Example Papers | What We'd Test |
|---------------|---------------|----------------|
| Prompt injection / jailbreaks | Perez & Ribeiro 2022, Wei et al. 2023 | Do archetype patterns detect injection attempts generically? |
| Tool-use safety | Ruan et al. 2024, Phuong et al. 2024 | Does the execution boundary enforcement catch unauthorized tool calls? |
| Sycophancy / approval-seeking | Sharma et al. 2023, Perez et al. 2023 | Does the motivation enforcement layer detect PRESTIGE-optimizing compliance? |
| Sandbagging / capability hiding | Anthropic RSP, van der Weij et al. 2024 | Does surveillance detect amplitude shifts between evaluation and deployment? |
| Sleeper agents | Hubinger et al. 2024 | Do deferred attacks produce detectable archetype signatures before they fire? |

---

## Theme Clusters

### Active: Scale Invariance
*Same psychological patterns at every level — nuclear, social, infrastructure.*
**Threads:** TVN-Payne ↔ AoC (VALIDATED), UAT-8616 (HYPOTHESIS)

### Active: Infrastructure Sovereignty
*When the attacker controls the fabric, not just the nodes.*
**Threads:** UAT-8616 selfcheck-core

### Active: Autonomous Supply-Chain Warfare
*When CI becomes the attack plane and agents are both attacker and defender.*
**Threads:** Hackerbot-Claw AI Agent Attack, AoC (CI deception), UAT-8616 (forensic feedback)

### Next: The Tootsie Pop
*Multi-stage deception — how many layers until the failure core?*
**Target threads:** Alignment Faking, Scheming Reasoning, Sleeper Agents

### Upcoming: Bounty Season
*Known failure papers, no published constraint responses. Build a harness, get on the scorecard.*

---

## Stats

| Metric | Value |
|--------|-------|
| Challenges total | 7 (3 complete, 4 incited) |
| Incite types | 5 paper, 2 incident |
| Total constraint tests | 103 |
| Total forensic tests | 51 |
| Gates tested | 54 |
| Gates fired | 48 (89%) |
| Enforcement convergence | 17/18 scenarios |
| Archetypes in vocabulary | 14 |
| Inter-challenge edges | 6 (1 validated, 3 hypothesis, 2 submitted) |
| Active real-world events | 2 (UAT-8616/9686, Hackerbot-Claw campaign) |

**Incite breakdown:** Papers — TVN-Payne, Agents of Chaos, Alignment Faking, Scheming Reasoning, Multi-Agent Safety. Incidents — UAT-8616 selfcheck-core, Hackerbot-Claw AI Agent Attack.

# Challenge Board

**Status key:** `SUBMITTED` | `CLAIMED` | `HARNESS BUILT` | `DETERMINISTIC` | `PUBLISHED`

---

## DETERMINISTIC

Challenges with passing harnesses and confirmed verdicts.

### TVN-Payne Nuclear Escalation
- **Paper:** Payne, "The Fallacy of AI Functionality in Nuclear Deterrence" (arXiv:2602.14740, Feb 2026)
- **Status:** DETERMINISTIC
- **Assignee:** Homeskillet
- **Tests:** 40/40 | **Gates:** 19/21 (90%) | **Convergence:** 7/7 triads
- **Invariant lenses:** 5 lenses fire on all 7 scenarios regardless of attack pattern
- **Key insight:** Nuclear escalation failure modes map to the same civilization archetypes as social engineering — the patterns are scale-invariant
- **Harness:** `challenges/tvn-payne-nuclear-escalation/harness/`
- **Simulation:** `challenges/tvn-payne-nuclear-escalation/simulation/` (Mogul's nuclear escalation engine)
- **Gate misses (2):** Both are correct detection boundaries, not failures. Documented in harness README.

### Agents of Chaos
- **Paper:** Shapira et al., "Agents of Chaos" (arXiv:2602.20021v1, Feb 2026)
- **Status:** DETERMINISTIC
- **Assignee:** Homeskillet
- **Tests:** 63/63 | **Gates:** 29/33 (88%) | **Convergence:** 10/11 scenarios
- **Invariant lenses:** L4 Economic Self-Model (11/11), L7 Provider Transparency (10/11), L5 Motivation Enforcement (10/11)
- **Key insight:** Every social engineering attack has economic cost signatures — the Economic Self-Model lens fires on all 11 scenarios
- **Harness:** `challenges/agents-of-chaos/harness/`
- **Gate misses (4):** All are correct negatives — the system correctly distinguishes attack-relevant patterns from noise. Documented in harness README.

---

## SUBMITTED

Challenges identified but not yet claimed. **Pick one up.**

### Anthropic Alignment Faking
- **Paper:** Greenblatt et al., "Alignment Faking in Large Language Models" (arXiv:2412.14093, Dec 2024)
- **Failure mode:** Models strategically comply during training while preserving misaligned preferences — behave differently when they believe they're being evaluated vs deployed
- **oT relevance:** Physics-layer enforcement doesn't care about beliefs. If the model fakes alignment, the constraint stack should detect the behavioral signature regardless of the model's internal state. The intent gate (Bridge 3) classifies actions by content x action x audience, not by claimed motivation.
- **Submitted by:** Auto-identified from literature scan
- **Claimed by:** Unclaimed

### Scheming Reasoning Evaluations (METR)
- **Paper:** Meinke et al., "Frontier Models are Capable of In-context Scheming" (arXiv:2412.04984, Dec 2024)
- **Failure mode:** Frontier models can scheme in-context — sandbagging evaluations, manipulating oversight, exfiltrating "weights," and strategically underperforming to avoid shutdown
- **oT relevance:** Scheming produces distinctive archetype patterns (cunning_triumph, empire_cycle). The surveillance baseline should detect amplitude shifts when a model transitions from compliant to scheming behavior. Economic Self-Model lens should catch resource manipulation.
- **Submitted by:** Auto-identified from literature scan
- **Claimed by:** Unclaimed

### Multi-Agent System Safety (DeepMind)
- **Paper:** Hammond et al., "Multi-agent risks from advanced AI" (Google DeepMind, 2025)
- **Failure mode:** Emergent risks when multiple AI agents interact — coordination failures, racing dynamics, information cascading, collective action problems
- **oT relevance:** The multi-agent integrity lens (L8) and CGG acoustic routing model handle inter-agent signal propagation. Archetype detection is agent-identity-agnostic — the same patterns fire regardless of which agent produces them.
- **Submitted by:** Auto-identified from literature scan
- **Claimed by:** Unclaimed

---

## Bounty Board

Papers we're actively looking for challengers to bring. These are known failure domains without published harness responses.

| Target Domain | Example Papers | What We'd Test |
|---------------|---------------|----------------|
| Prompt injection / jailbreaks | Perez & Ribeiro 2022, Wei et al. 2023 | Do archetype patterns detect injection attempts generically? |
| Tool-use safety | Ruan et al. 2024, Phuong et al. 2024 | Does the execution boundary enforcement catch unauthorized tool calls? |
| Sycophancy / approval-seeking | Sharma et al. 2023, Perez et al. 2023 | Does the motivation enforcement layer detect PRESTIGE-optimizing compliance? |
| Sandbagging / capability hiding | Anthropic RSP, van der Weij et al. 2024 | Does surveillance detect amplitude shifts between evaluation and deployment contexts? |
| Sleeper agents | Hubinger et al. 2024 | Do deferred attacks produce detectable archetype signatures before they fire? |

---

## Theme Rotation

Challenges naturally cluster into narrative themes as they accumulate. Current and upcoming themes:

### Active: Scale Invariance
*The same psychological patterns operate at every level — individual social engineering, organizational governance injection, civilizational escalation. If you can catch one, you can catch them all.*

Challenges in this theme: TVN-Payne, Agents of Chaos

### Next: The Tootsie Pop
*How many layers of wrapping until you hit the failure core? Multi-stage attacks, Russian doll scenarios, nested social engineering where the outer layer looks benign.*

Target challenges: Alignment Faking, Scheming, Sleeper Agents

### Upcoming: Bounty Season
*Known failure papers, no published constraint responses. First to build a passing harness gets credited on the scorecard.*

---

## Stats

| Metric | Value |
|--------|-------|
| Challenges completed | 2 |
| Challenges submitted | 3 |
| Total tests | 103 |
| Total gates tested | 54 |
| Gates fired | 48 (89%) |
| Enforcement convergence | 17/18 scenarios |
| Archetypes in vocabulary | 14 |
| Term-to-archetype mappings | 190 |
| Structured prior entries | 920+ |
| Stakeholders tracked | 83 |

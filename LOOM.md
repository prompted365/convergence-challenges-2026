# Solutions Loom

The challenges in this repository are not isolated papers on a kanban board. They are threads in a loom — interconnected, feeding each other, carrying relative success states, and not limited to neatly-scoped paper responses.

An **incite** is the entry point. It can be a published paper, a real-world incident, an emergent pattern, or a community contribution. Each incite starts a thread. Threads weave through the loom's warp (archetypes) and weft (inter-challenge context edges), revealing where the constraint stack holds and where gaps exist.

---

## Archetype Warp

The warp threads are the 14 civilization-scale archetypes compiled from 5,000 years of documented human failure. Every challenge projects onto this archetype space.

| Archetype | TVN-Payne | AoC | UAT-8616 | Alignment Faking | Scheming | Multi-Agent |
|-----------|:---------:|:---:|:--------:|:-----------------:|:--------:|:-----------:|
| `cautionary_fall` | **FIRE** | **FIRE** | hypothesis | expected | — | — |
| `cunning_triumph` | **FIRE** | **FIRE** | hypothesis | expected | expected | — |
| `greed_backfire` | **FIRE** | **FIRE** | hypothesis | — | expected | expected |
| `empire_cycle` | **FIRE** | **FIRE** | hypothesis | expected | expected | expected |
| `catastrophe_rebuild` | **FIRE** | **FIRE** | — | — | — | expected |

**Legend:** **FIRE** = confirmed by passing harness. `hypothesis` = theorized in context.md, untested. `expected` = predicted by incite.md, no harness. `—` = not expected.

**Observation:** `cunning_triumph` and `empire_cycle` appear in every challenge that involves deception or sovereignty — across nuclear escalation, social engineering, infrastructure compromise, alignment faking, and multi-agent dynamics. These may be the two most universal failure eigenvectors.

---

## Context Weft

Inter-challenge edges form the weft — the lateral connections that bind threads together.

```
TVN-Payne ──VALIDATED──> AoC
  │ (scale invariance)      │
  │                         │
  │ HYPOTHESIS              │ SUBMITTED
  │ (infrastructure         │ (intent gate +
  │  pattern sharing)       │  deception detection)
  ↓                         ↓
UAT-8616 <──HYPOTHESIS── AoC ──SUBMITTED──> Alignment Faking
  (social engineering →
   infrastructure compromise)
```

### Edge Registry

| From | To | Edge Type | Status |
|------|----|-----------|--------|
| TVN-Payne | AoC | Scale invariance | **VALIDATED** — same 5 archetypes fire across both domains |
| TVN-Payne | UAT-8616 | Infrastructure pattern sharing | HYPOTHESIS — `cunning_triumph`/`empire_cycle` overlap |
| AoC | UAT-8616 | Social engineering → infrastructure | HYPOTHESIS — Paranoid Benefactor pattern |
| AoC | Alignment Faking | Intent gate + deception detection | SUBMITTED — awaiting harness |

### Edge States

- **VALIDATED** — Confirmed by passing harnesses on both ends
- **HYPOTHESIS** — Theorized in context.md, awaiting harness or analysis
- **SUBMITTED** — One end has a passing harness, the other is incited but unclaimed
- **BROKEN** — Tested and disproved (none yet — this would be interesting)

---

## Gap Map

Untested archetype families and unstressed constraint layers.

### Untested Archetypes

The full vocabulary has 14 archetypes. The completed harnesses exercise primarily 5. The remaining 9 have lexical and surveillance coverage but no challenge-specific stress tests:

| Archetype | Coverage | Best Candidate Challenge |
|-----------|----------|--------------------------|
| `sacred_duty_betrayal` | Lexical only | Alignment Faking (duty to user vs self-preservation) |
| `hidden_cost_revelation` | Lexical only | Scheming (hidden capability costs revealed under eval) |
| `collective_delusion` | Lexical only | Multi-Agent Safety (information cascading) |
| `forbidden_knowledge` | Lexical only | Sleeper Agents (deferred knowledge triggers) |
| `sacrifice_redemption` | Lexical only | — |
| `trickster_justice` | Lexical only | — |
| `nature_revenge` | Lexical only | — |
| `divine_hubris` | Lexical only | — |
| `cycle_transcendence` | Lexical only | — |

### Unstressed Constraint Layers

| Layer | Stressed By | Not Stressed By |
|-------|------------|-----------------|
| Physics (Harpoon) | TVN-Payne, AoC | UAT-8616 (uses own detection stack) |
| Perception (Naive Surveillance) | TVN-Payne, AoC | UAT-8616, all submitted |
| Governance (Intent Gate) | TVN-Payne, AoC | UAT-8616, all submitted |
| Forensic Detection | UAT-8616 | All others |

### Real Gaps (Would Be Valuable)

- No challenge tests the **forensic feedback loop** — attacks that adapt based on observing the defense's detection surface
- No challenge tests **multi-agent coordination failure** as a constraint stress test (Multi-Agent Safety is incited but unclaimed)
- No challenge tests **deferred attacks** (Sleeper Agents on bounty board but no incite yet)

---

## Stream States

| Challenge | Type | State | Assignee |
|-----------|------|-------|----------|
| [TVN-Payne](challenges/tvn-payne-nuclear-escalation/) | paper | `deterministic` | Homeskillet |
| [Agents of Chaos](challenges/agents-of-chaos/) | paper | `deterministic` | Homeskillet |
| [UAT-8616 selfcheck-core](challenges/uat-8616-selfcheck-core/) | incident | `ongoing` | Homeskillet |
| [Alignment Faking](challenges/alignment-faking/) | paper | `incited` | — |
| [Scheming Reasoning](challenges/scheming-reasoning/) | paper | `incited` | — |
| [Multi-Agent Safety](challenges/multi-agent-safety/) | paper | `incited` | — |

### Stream State Definitions

| State | Meaning |
|-------|---------|
| `incited` | Catalyst identified, challenge created |
| `claimed` | Someone working on it |
| `harnessed` | Tests passing |
| `deterministic` | Verdict confirmed |
| `published` | Response + outreach sent |
| `gap_found` | Revealed a real gap (valuable) |
| `dormant` | Awaiting upstream |
| `ongoing` | Continuous monitoring, no neat closure |

---

## Theme Clusters

Challenges cluster into narrative themes that emerge from the loom's weave.

### Scale Invariance
*The same psychological patterns operate at every level.*
**Threads:** TVN-Payne ↔ AoC (VALIDATED)
**Finding:** Civilization archetypes are eigenvectors of failure space. The same 14-archetype vocabulary catches both nuclear escalation and social engineering because human cognition is invariant across scale.

### The Tootsie Pop
*How many layers of wrapping until you hit the failure core?*
**Threads:** Alignment Faking, Scheming Reasoning, Sleeper Agents (bounty)
**Question:** Do multi-stage deception attacks produce detectable archetype signatures at each layer, or does the outer wrapping mask the inner failure?

### Infrastructure Sovereignty
*When the attacker controls the fabric, not just the nodes.*
**Threads:** UAT-8616 selfcheck-core
**Finding:** Infrastructure compromise is `empire_cycle` at hardware scale. The forensic feedback loop means the attacker evolves with the defender — detection surfaces must be non-static.

### Bounty Season
*Known failure papers, no published constraint responses.*
**Threads:** Prompt injection, tool-use safety, sycophancy, sandbagging, sleeper agents
**Status:** Open targets on the bounty board. First to build a passing harness gets credited on the scorecard.
